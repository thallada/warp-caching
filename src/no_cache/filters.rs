use warp::Filter;

use super::handlers;
use crate::models::{Environment, ListOptions, Todo};

/// The 4 TODOs filters combined.
pub fn todos(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("no_cache").and(
        todos_list(env.clone())
            .or(todos_create(env.clone()))
            .or(todos_update(env.clone()))
            .or(todos_get(env.clone()))
            .or(todos_delete(env)),
    )
}

/// GET /todos?offset=3&limit=5
pub fn todos_list(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_env(env))
        .and_then(handlers::list_todos)
}

/// GET /todos/:id
pub fn todos_get(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos" / u64)
        .and(warp::get())
        .and(with_env(env))
        .and_then(handlers::get_todo)
}

/// POST /todos with JSON body
pub fn todos_create(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos")
        .and(warp::post())
        .and(json_body())
        .and(with_env(env))
        .and_then(handlers::create_todo)
}

/// PUT /todos/:id with JSON body
pub fn todos_update(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos" / u64)
        .and(warp::put())
        .and(json_body())
        .and(with_env(env))
        .and_then(handlers::update_todo)
}

/// DELETE /todos/:id
pub fn todos_delete(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // We'll make one of our endpoints admin-only to show how authentication filters are used
    let admin_only = warp::header::exact("authorization", "Bearer admin");

    warp::path!("todos" / u64)
        // It is important to put the auth check _after_ the path filters.
        // If we put the auth check before, the request `PUT /todos/invalid-string`
        // would try this filter and reject because the authorization header doesn't match,
        // rather because the param is wrong for that other path.
        .and(admin_only)
        .and(warp::delete())
        .and(with_env(env))
        .and_then(handlers::delete_todo)
}

fn with_env(
    env: Environment,
) -> impl Filter<Extract = (Environment,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || env.clone())
}

fn json_body() -> impl Filter<Extract = (Todo,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
