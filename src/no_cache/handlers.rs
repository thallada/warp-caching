/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.
use std::convert::Infallible;
use tracing::debug;
use warp::http::StatusCode;

use crate::models::{Environment, ListOptions, Todo};

pub async fn list_todos(
    opts: ListOptions,
    env: Environment,
) -> Result<impl warp::Reply, Infallible> {
    // Just return a JSON array of todos, applying the limit and offset.
    let todos = env.db.lock().await;
    let todos: Vec<Todo> = todos
        .clone()
        .into_iter()
        .skip(opts.offset.unwrap_or(0))
        .take(opts.limit.unwrap_or(std::usize::MAX))
        .collect();
    Ok(warp::reply::json(&todos))
}

pub async fn get_todo(id: u64, env: Environment) -> Result<impl warp::Reply, warp::Rejection> {
    debug!("get_todo: id={}", id);
    let mut vec = env.db.lock().await;

    // Look for the specified Todo...
    for todo in vec.iter_mut() {
        if todo.id == id {
            return Ok(warp::reply::json(&todo));
        }
    }

    debug!("    -> todo id not found!");

    // If the for loop didn't return OK, then the ID doesn't exist...
    Err(warp::reject::not_found())
}

pub async fn create_todo(create: Todo, env: Environment) -> Result<impl warp::Reply, Infallible> {
    debug!("create_todo: {:?}", create);

    let mut vec = env.db.lock().await;

    for todo in vec.iter() {
        if todo.id == create.id {
            debug!("    -> id already exists: {}", create.id);
            // Todo with id already exists, return `400 BadRequest`.
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    // No existing Todo with id, so insert and return `201 Created`.
    vec.push(create);

    Ok(StatusCode::CREATED)
}

pub async fn update_todo(
    id: u64,
    update: Todo,
    env: Environment,
) -> Result<impl warp::Reply, warp::Rejection> {
    debug!("update_todo: id={}, todo={:?}", id, update);
    let mut vec = env.db.lock().await;

    // Look for the specified Todo...
    for todo in vec.iter_mut() {
        if todo.id == id {
            *todo = update;
            return Ok(StatusCode::OK);
        }
    }

    debug!("    -> todo id not found!");

    // If the for loop didn't return OK, then the ID doesn't exist...
    Ok(StatusCode::NOT_FOUND)
}

pub async fn delete_todo(id: u64, env: Environment) -> Result<impl warp::Reply, warp::Rejection> {
    debug!("delete_todo: id={}", id);

    let mut vec = env.db.lock().await;

    let len = vec.len();
    vec.retain(|todo| {
        // Retain all Todos that aren't this id...
        // In other words, remove all that *are* this id...
        todo.id != id
    });

    // If the vec is smaller, we found and deleted a Todo!
    let deleted = vec.len() != len;

    if deleted {
        // respond with a `204 No Content`, which means successful,
        // yet no body expected...
        Ok(StatusCode::NO_CONTENT)
    } else {
        debug!("    -> todo id not found!");
        Ok(StatusCode::NOT_FOUND)
    }
}
