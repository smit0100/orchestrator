# Quick Start Guide - Distributed Task Orchestrator

## 📚 Table of Contents
1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Build](#build)
4. [Running Services](#running-services)
5. [Testing Services](#testing-services)
6. [Making First Request](#making-first-request)
7. [Stopping Services](#stopping-services)

---

## Prerequisites

### System Requirements
- **OS**: Linux, macOS, or WSL2 (Windows)
- **RAM**: Minimum 2GB (4GB recommended)
- **Disk**: 500MB free space

### Software Requirements
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
rustc --version    # Should show 1.70+
cargo --version    # Should show 1.70+

# Install protobuf compiler
# On Ubuntu/Debian:
sudo apt-get install protobuf-compiler

# On macOS:
brew install protobuf
```

---

## Installation

### Clone/Navigate to Project
```bash
cd /home/smit-dankhra/Desktop/rust_projects/orchestrator
```

### Verify Project Structure
```bash
ls -la
# Should see:
# - proto/
# - crates/
# - Cargo.toml
# - README.md
```

---

## Build

### Build Debug (Development)
```bash
cargo build
# Takes ~2-5 minutes on first build
# Output: ./target/debug/{gateway,scheduler,worker}
```

### Build Release (Production)
```bash
cargo build --release
# Takes ~40 seconds
# Output: ./target/release/{gateway,scheduler,worker}
# Binaries: ~3.5-3.6 MB each
```

### Verify Build
```bash
bash verify_build.sh
# Shows: ✅ BUILD VERIFICATION PASSED
```

---

## Running Services

### Prerequisites
- Open **3 terminal windows/tabs**
- Navigate to project directory in each

### Terminal 1: Start Scheduler (Port 50052)
```bash
./target/release/scheduler

# Expected output:
# Scheduler service listening on 127.0.0.1:50052
```

### Terminal 2: Start Worker (Port 50053)
```bash
./target/release/worker

# Expected output:
# Worker service listening on 127.0.0.1:50053
```

### Terminal 3: Start Gateway (Port 50051)
```bash
./target/release/gateway

# Expected output:
# Gateway service listening on 127.0.0.1:50051
```

### All services running? ✅
You should see 3 services listening:
- Gateway: `127.0.0.1:50051`
- Scheduler: `127.0.0.1:50052`
- Worker: `127.0.0.1:50053`

---

## Testing Services

### Test 1: Health Check (Gateway)
```bash
# Install grpcurl (if not installed)
go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest

# Check Gateway health
grpcurl -plaintext localhost:50051 \
  orchestrator.v1.Gateway/HealthCheck

# Expected Response:
# {
#   "healthy": true,
#   "version": "0.1.0"
# }
```

### Test 2: Health Check (Scheduler)
```bash
grpcurl -plaintext localhost:50052 \
  orchestrator.v1.Scheduler/HealthCheck

# Expected Response:
# {
#   "healthy": true,
#   "version": "0.1.0"
# }
```

### Test 3: Health Check (Worker)
```bash
grpcurl -plaintext localhost:50053 \
  orchestrator.v1.Worker/HealthCheck

# Expected Response:
# {
#   "healthy": true,
#   "version": "0.1.0"
# }
```

---

## Making First Request

### Submit a Task (Unary RPC)

Create request file: `task_request.json`
```json
{
  "name": "data-processing-job",
  "description": "Process and analyze customer data",
  "payload": "SGVsbG8gV29ybGQ=",
  "timeout_secs": 300,
  "max_retries": 3
}
```

Submit task:
```bash
grpcurl -plaintext -d @ localhost:50051 \
  orchestrator.v1.Gateway/SubmitTask < task_request.json

# Expected Response:
# {
#   "task_id": "550e8400-e29b-41d4-a716-446655440000",
#   "timestamp": "1674993600"
# }
```

### Subscribe to Progress (Server Streaming)

Create request file: `progress_request.json`
```json
{
  "task_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

Subscribe:
```bash
grpcurl -plaintext -d @ localhost:50051 \
  orchestrator.v1.Gateway/SubscribeProgress < progress_request.json

# Expected Response:
# (Streams progress updates - may be empty if no progress recorded)
```

---

## Understanding Service Communication

```
┌──────────────────────────────────────────┐
│ Your Client (grpcurl or code)            │
└──────────────────────────────────────────┘
              │ gRPC
              ▼
┌──────────────────────────────────────────┐
│ Gateway Service (50051)                  │
│ - Accepts tasks (SubmitTask)             │
│ - Streams progress (SubscribeProgress)   │
└──────────────────────────────────────────┘
              │ Internal gRPC
              ▼
┌──────────────────────────────────────────┐
│ Scheduler Service (50052)                │
│ - Manages workers                        │
│ - Distributes tasks                      │
└──────────────────────────────────────────┘
              │ Bidirectional gRPC
              ▼
┌──────────────────────────────────────────┐
│ Worker Service (50053)                   │
│ - Executes tasks                         │
│ - Reports progress                       │
└──────────────────────────────────────────┘
```

---

## Stopping Services

### Graceful Shutdown
In each terminal where a service is running:
```bash
# Press Ctrl+C
^C

# You'll see:
# Scheduler service stopped
```

### Verify All Stopped
```bash
# Check if ports are free
lsof -i :50051  # Should return nothing
lsof -i :50052  # Should return nothing
lsof -i :50053  # Should return nothing
```

---

## Common Issues & Solutions

### Issue 1: "Address already in use"
```
Error: address 127.0.0.1:50051 already in use
```

**Solution:**
```bash
# Kill process on that port
lsof -i :50051
kill -9 <PID>

# Or change port in code (for development)
```

### Issue 2: "protobuf-compiler not found"
```
Error: Could not find `protoc`
```

**Solution:**
```bash
# Install protobuf compiler
sudo apt-get install protobuf-compiler  # Ubuntu/Debian
brew install protobuf                   # macOS
```

### Issue 3: Build fails with "duplicate key"
```
Error: duplicate key in Cargo.toml
```

**Solution:**
```bash
# This is fixed in the provided code
# If you still get it, check for duplicate [build-dependencies] sections
vim crates/*/Cargo.toml
```

### Issue 4: "Connection refused"
```
Error: failed to connect to 127.0.0.1:50051
```

**Solution:**
- Verify all 3 services are running
- Check with: `lsof -i :50051`
- Restart the service

---

## Next Steps

### 1. Read Full Documentation
```bash
cat README.md           # Architecture and design
cat IMPLEMENTATION_SUMMARY.md  # What was built
cat ARCHITECTURE.md     # Deep dive
```

### 2. Explore Code
```bash
# Gateway implementation
cat crates/gateway/src/service.rs

# Scheduler implementation
cat crates/scheduler/src/service.rs

# Worker implementation
cat crates/worker/src/service.rs
```

### 3. Write Custom Client
See `INTERVIEW_GUIDE.md` for Python/Rust client examples

### 4. Extend Features
- Add database persistence
- Implement retry logic
- Add metrics/monitoring
- Deploy with Docker

---

## Development Tips

### Run Specific Service Only
```bash
cargo run -p gateway --release
cargo run -p scheduler --release
cargo run -p worker --release
```

### Run Tests
```bash
cargo test
cargo test -p gateway
cargo test -p scheduler
cargo test -p worker
```

### View Logs with Tracing
All services output structured logs:
```bash
# Look for INFO level logs in terminal output
2026-01-29T15:23:45.123Z INFO Task submitted task_id=...
```

### Rebuild Proto
Proto files are automatically recompiled. If you modify `orchestrator.proto`:
```bash
cargo clean
cargo build
```

---

## Performance Benchmarks

### Build Times
- **Debug**: 2-5 minutes (first build)
- **Release**: 40 seconds (first), <1 second (incremental)

### Binary Sizes
- Gateway: 3.6 MB
- Scheduler: 3.6 MB
- Worker: 3.5 MB

### Memory Usage
- Per service: ~50 MB at rest
- Minimal overhead (async-first design)

---

## Useful Commands Cheatsheet

```bash
# Build
cargo build                    # Debug build
cargo build --release        # Release build
cargo build -p gateway       # Single crate

# Run
./target/release/gateway     # Run binary
cargo run -p gateway         # Build and run
cargo run --release -p scheduler

# Test
cargo test                   # All tests
cargo test -p gateway       # Single crate tests
cargo test -- --test-threads=1  # Serial tests

# Clean
cargo clean                  # Remove build artifacts

# Check
cargo check                  # Fast compile check
cargo clippy                 # Linting

# Format
cargo fmt                    # Format code
cargo fmt --check           # Check formatting
```

---

## Quick Troubleshooting

| Problem | Solution |
|---------|----------|
| Services won't start | Check ports with `lsof -i :5005x` |
| gRPC requests fail | Verify all 3 services are running |
| Proto changes not applied | Run `cargo clean && cargo build` |
| High memory usage | This is normal; services use Tokio runtime |
| Compilation errors | Check Rust version: `rustc --version` |

---

## Getting Help

1. **Error Messages**: Read the full error output carefully
2. **Logs**: Services output to console
3. **Check**: Port availability, Rust version, protobuf installed
4. **Documentation**: See README.md for detailed info
5. **Code**: Read service.rs files for implementation details

---

**You're ready to go! 🚀**

Follow the steps above and you'll have a fully functional distributed task orchestrator running locally.
