use std::fmt::Debug;

use crate::models::ListOptions;

mod cache;
mod cached_response;

pub use cache::Cache;
pub use cached_response::CachedResponse;

#[derive(Debug, Clone)]
pub struct Caches {
    pub todos: Cache<u64, CachedResponse>,
    pub list_todos: Cache<ListOptions, CachedResponse>,
}

impl Caches {
    pub fn initialize() -> Self {
        Caches {
            todos: Cache::new("todos", 100),
            list_todos: Cache::new("list_todos", 100),
        }
    }
}
