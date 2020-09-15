use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::caches::Caches;

/// So we don't have to tackle how different database work, we'll just use
/// a simple in-memory DB, a vector synchronized by a mutex.
pub type Db = Arc<Mutex<Vec<Todo>>>;

pub fn blank_db() -> Db {
    Arc::new(Mutex::new(Vec::new()))
}

pub fn blank_caches() -> Caches {
    Caches::initialize()
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub db: Db,
    pub caches: Caches,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: u64,
    pub text: String,
    pub completed: bool,
}

// The query parameters for list_todos.
#[derive(Debug, Deserialize, Clone, Hash, Eq, PartialEq)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}
