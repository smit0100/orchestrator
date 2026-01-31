use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Worker {
    pub id: String,
    pub address: String,
    pub available_slots: i32,
}

#[derive(Clone)]
pub struct WorkerRegistry {
    workers: Arc<DashMap<String, Worker>>,
}

impl WorkerRegistry {
    pub fn new() -> Self {
        Self {
            workers: Arc::new(DashMap::new()),
        }
    }

    pub fn register_worker(&self, worker: Worker) {
        self.workers.insert(worker.id.clone(), worker);
    }

    pub fn update_worker_slots(&self, worker_id: &str, available_slots: i32) {
        if let Some(mut worker) = self.workers.get_mut(worker_id) {
            worker.available_slots = available_slots;
        }
    }

    pub fn get_available_worker(&self) -> Option<Worker> {
        self.workers
            .iter()
            .find(|entry| entry.available_slots > 0)
            .map(|entry| entry.value().clone())
    }

    pub fn get_worker(&self, worker_id: &str) -> Option<Worker> {
        self.workers.get(worker_id).map(|w| w.clone())
    }

    pub fn list_workers(&self) -> Vec<Worker> {
        self.workers.iter().map(|entry| entry.value().clone()).collect()
    }

    pub fn worker_exists(&self, worker_id: &str) -> bool {
        self.workers.contains_key(worker_id)
    }
}

impl Default for WorkerRegistry {
    fn default() -> Self {
        Self::new()
    }
}
