use dashmap::DashMap;
use std::sync::Arc;
use std::collections::VecDeque;

use crate::orchestrator::v1::TaskSpec;

#[derive(Clone)]
pub struct TaskQueue {
    queue: Arc<DashMap<String, VecDeque<TaskSpec>>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(DashMap::new()),
        }
    }

    pub fn enqueue(&self, worker_id: String, task: TaskSpec) {
        self.queue
            .entry(worker_id)
            .or_insert_with(VecDeque::new)
            .push_back(task);
    }

    pub fn dequeue(&self, worker_id: &str) -> Option<TaskSpec> {
        if let Some(mut queue) = self.queue.get_mut(worker_id) {
            queue.pop_front()
        } else {
            None
        }
    }

    pub fn queue_size(&self, worker_id: &str) -> usize {
        self.queue
            .get(worker_id)
            .map(|q| q.len())
            .unwrap_or(0)
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}
