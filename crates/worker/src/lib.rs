pub mod orchestrator {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/orchestrator.v1.rs"));
    }
}

pub mod error;
pub mod service;