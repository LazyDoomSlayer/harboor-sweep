use std::sync::{Arc, Mutex};

pub struct AppState {
    pub interval: Arc<Mutex<u64>>,
    pub is_monitoring: Arc<Mutex<bool>>,
}

impl AppState {
    pub fn new(default_interval: u64) -> Self {
        Self {
            interval: Arc::new(Mutex::new(default_interval)),
            is_monitoring: Arc::new(Mutex::new(false)),
        }
    }

    pub fn set_interval(&self, interval: u64) {
        *self.interval.lock().unwrap() = interval;
    }

    pub fn is_monitoring(&self) -> bool {
        *self.is_monitoring.lock().unwrap()
    }

    pub fn set_monitoring(&self, state: bool) {
        *self.is_monitoring.lock().unwrap() = state;
    }
}
