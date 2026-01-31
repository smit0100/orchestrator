# Distributed Task Orchestrator - Rust + gRPC

A production-grade distributed task orchestrator built with **Rust**, **Tokio**, and **gRPC** (tonic).

## 📋 Project Overview

This project implements a **three-service architecture** for distributed task execution:

### Services

1. **Gateway** (Port 50051)
   - Client-facing API
   - Submit tasks via `SubmitTask` (unary RPC)
   - Subscribe to progress via `SubscribeProgress` (server streaming RPC)
   - Health checks

2. **Scheduler** (Port 50052)
   - Manages worker registrations
   - Bidirectional streaming with workers
   - Task assignment logic
   - Health checks

3. **Worker** (Port 50053)
   - Executes tasks assigned by scheduler
   - Sends progress updates via server streaming
   - Heartbeat mechanism
   - Health checks

## 🏗️ Architecture

```
┌─────────────┐
│   Client    │
└──────┬──────┘
       │ gRPC Unary/Streaming
       ▼
┌─────────────────┐
│    Gateway      │ (50051)
└────────┬────────┘
         │ Internal gRPC
         ▼
┌──────────────────┐
│   Scheduler      │ (50052)
└────────┬─────────┘
         │ Bidirectional Streaming
         ▼
   ┌─────────────┐
   │   Worker N  │ (50053+)
   └─────────────┘
```

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+
- Cargo
- protobuf-compiler (for proto compilation)

### Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Run Services

**Terminal 1 - Scheduler:**
```bash
./target/release/scheduler
# Output: Scheduler service listening on 127.0.0.1:50052
```

**Terminal 2 - Worker:**
```bash
./target/release/worker
# Output: Worker service listening on 127.0.0.1:50053
```

**Terminal 3 - Gateway:**
```bash
./target/release/gateway
# Output: Gateway service listening on 127.0.0.1:50051
```

## 📦 Project Structure

```
orchestrator/
├── proto/
│   └── orchestrator/v1/
│       └── orchestrator.proto      # gRPC service definitions
├── crates/
│   ├── gateway/
│   │   ├── src/
│   │   │   ├── main.rs            # Gateway entry point
│   │   │   ├── lib.rs             # Library exports
│   │   │   ├── error.rs           # Error types
│   │   │   ├── service.rs         # Gateway service impl
│   │   │   └── store.rs           # Task storage
│   │   ├── build.rs               # Build script for proto
│   │   └── Cargo.toml
│   ├── scheduler/
│   │   ├── src/
│   │   │   ├── main.rs            # Scheduler entry point
│   │   │   ├── lib.rs             # Library exports
│   │   │   ├── error.rs           # Error types
│   │   │   ├── service.rs         # Scheduler service impl
│   │   │   ├── worker_registry.rs # Worker management
│   │   │   └── task_queue.rs      # Task queueing
│   │   ├── build.rs               # Build script for proto
│   │   └── Cargo.toml
│   └── worker/
│       ├── src/
│       │   ├── main.rs            # Worker entry point
│       │   ├── lib.rs             # Library exports
│       │   ├── error.rs           # Error types
│       │   └── service.rs         # Worker service impl
│       ├── build.rs               # Build script for proto
│       └── Cargo.toml
├── Cargo.toml                      # Workspace root
├── build.rs                        # Workspace build script
└── plan.md                         # Project requirements
```

## 🔌 gRPC API

### Gateway Service

#### SubmitTask (Unary RPC)
```protobuf
rpc SubmitTask(TaskSpec) returns (TaskAccepted);
```
- **Request:** Task specification with name, description, payload
- **Response:** Assigned task ID and timestamp
- **Use Case:** Client submits a task

#### SubscribeProgress (Server Streaming RPC)
```protobuf
rpc SubscribeProgress(ProgressRequest) returns (stream TaskProgress);
```
- **Request:** Task ID to monitor
- **Response:** Stream of progress updates
- **Use Case:** Real-time progress monitoring

### Scheduler Service

#### RegisterWorker (Bidirectional Streaming RPC)
```protobuf
rpc RegisterWorker(stream WorkerResponse) returns (stream TaskAssignment);
```
- **Request Stream:** Worker heartbeats, progress updates, results
- **Response Stream:** Task assignments
- **Use Case:** Long-lived worker connection to scheduler

## 📊 Message Types

### Core Messages

- **TaskSpec** - Task definition with opaque payload
- **TaskAccepted** - Response when task is submitted (contains task ID)
- **TaskProgress** - Progress updates during execution (percent, message)
- **TaskResult** - Final result with output or error
- **WorkerHeartbeat** - Worker status with available slots
- **TaskAssignment** - Task to be executed by a worker

### Status Enum

```
TASK_STATUS_UNSPECIFIED = 0
TASK_STATUS_PENDING     = 1
TASK_STATUS_RUNNING     = 2
TASK_STATUS_COMPLETED   = 3
TASK_STATUS_FAILED      = 4
TASK_STATUS_CANCELLED   = 5
```

## 🛠️ Key Design Decisions

### 1. RPC Types
- **Unary** for bounded operations (task submission)
- **Server Streaming** for progress updates
- **Bidirectional Streaming** for worker-scheduler long-lived connections

### 2. Error Handling
- Custom error types per crate
- gRPC Status codes for proper error propagation
- Tracing for observability

### 3. Concurrency
- **Tokio** multi-threaded runtime for async execution
- **DashMap** for thread-safe, lock-free concurrent data structures
- **mpsc channels** for message passing between tasks

### 4. Versioning
- Proto package: `orchestrator.v1`
- Backward compatibility required
- Version in health check responses

## 📝 Building Custom Features

### Adding a New Service

1. Create new crate:
```bash
cargo new --lib crates/myservice
```

2. Add to workspace in [Cargo.toml](Cargo.toml#L2):
```toml
members = ["crates/myservice"]
```

3. Create [build.rs](crates/myservice/build.rs) for proto compilation

4. Implement service in [src/service.rs](crates/myservice/src/service.rs)

### Extending Proto Definitions

Edit [orchestrator.proto](proto/orchestrator/v1/orchestrator.proto) and add:
- New message types
- New service methods
- Ensure backward compatibility with `oneof` patterns

## 🧪 Testing

Run tests for all crates:
```bash
cargo test
```

Test specific crate:
```bash
cargo test -p gateway
cargo test -p scheduler
cargo test -p worker
```

## 🔍 Health Checks

Each service exposes a health check endpoint:

```bash
# Via gRPC CLI (grpcurl)
grpcurl -plaintext localhost:50051 orchestrator.v1.Gateway/HealthCheck
grpcurl -plaintext localhost:50052 orchestrator.v1.Scheduler/HealthCheck
grpcurl -plaintext localhost:50053 orchestrator.v1.Worker/HealthCheck
```

## 📚 Dependencies

### Core
- **tokio**: Async runtime with full features
- **tonic**: gRPC framework and transport
- **prost**: Protocol Buffers serialization

### Data & Utilities
- **dashmap**: Concurrent hashmap
- **uuid**: Unique ID generation
- **chrono**: Timestamps
- **serde**: Serialization framework

### Observability
- **tracing**: Structured logging
- **tracing-subscriber**: Log formatting and filtering

### Error Handling
- **thiserror**: Error type derivations
- **futures**: Async utilities

## 🎯 Learning Objectives

This project teaches:

1. **Async Rust** - Tokio patterns, spawning tasks, channels
2. **gRPC** - Service definition, unary/streaming RPC types
3. **Protocol Buffers** - Message design, code generation
4. **Distributed Systems** - Service communication, failure handling
5. **Concurrency** - Thread-safe data structures, channels, synchronization

## 📖 References

- [Tokio Documentation](https://tokio.rs)
- [Tonic gRPC Framework](https://github.com/hyperium/tonic)
- [Protocol Buffers Guide](https://developers.google.com/protocol-buffers)
- [gRPC Best Practices](https://grpc.io/docs/guides/performance-best-practices/)

## ⚖️ License

MIT

---

**Project Status:** ✅ Complete and buildable

All three services compile without errors and are ready for:
- Feature implementation
- Integration testing
- Production deployment
