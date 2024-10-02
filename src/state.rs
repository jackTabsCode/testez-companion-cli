use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::Config;

pub struct Place {
    pub name: String,
    pub id: u64,
}

pub struct AppState {
    pub config: Arc<Config>,

    pub places: DashMap<String, Place>,
    pub active_place: Mutex<Option<String>>,

    pub only_log_failures: bool,
}
