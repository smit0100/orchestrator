use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::orchestrator::v1::TaskProgress;

#[derive(Clone)]
pub struct TaskStore {
    // task_id -> Vec<TaskProgress>
    task_progress: Arc<DashMap<String, Vec<TaskProgress>>>,
}

impl TaskStore {
    pub fn new() -> Self {
        Self {
            task_progress: Arc::new(DashMap::new()),
        }
    }

    pub fn create_task_id() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn add_progress(&self, task_id: String, progress: TaskProgress) {
        self.task_progress
            .entry(task_id)
            .or_insert_with(Vec::new)
            .push(progress);
    }

    pub fn get_progress(&self, task_id: &str) -> Option<Vec<TaskProgress>> {
        self.task_progress.get(task_id).map(|v| v.clone())
    }

    pub fn task_exists(&self, task_id: &str) -> bool {
        self.task_progress.contains_key(task_id)
    }

    pub fn initialize_task(&self, task_id: String) {
        self.task_progress.insert(task_id, Vec::new());
    }
}

impl Default for TaskStore {
    fn default() -> Self {
        Self::new()
    }
}
