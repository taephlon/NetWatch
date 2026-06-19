use std::sync::{Arc, Mutex};

use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<rusqlite::Connection>>,

    pub tx: broadcast::Sender<String>,
}
