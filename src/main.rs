use std::env;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

mod caches;
mod lru_cache;
mod models;
mod no_cache;
mod problem;

/// Provides a RESTful web server managing some Todos.
///
/// API will be:
///
/// - `GET /todos`: return a JSON list of Todos.
/// - `POST /todos`: create a new Todo.
/// - `PUT /todos/:id`: update a specific Todo.
/// - `DELETE /todos/:id`: delete a specific Todo.
#[tokio::main]
async fn main() {
    let env_log_filter =
        env::var("RUST_LOG").unwrap_or_else(|_| "warp=info,warp_caching=debug".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(env_log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let env = models::Environment {
        db: models::blank_db(),
        caches: models::blank_caches(),
    };

    let api = no_cache::filters::todos(env.clone()).or(lru_cache::filters::todos(env));

    // View access logs by setting `RUST_LOG=warp-caching`.
    let routes = api.with(warp::log("warp_caching"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::models::{self, Todo};
    use crate::no_cache::filters;

    #[tokio::test]
    async fn test_post() {
        let env = models::Environment {
            db: models::blank_db(),
            caches: models::blank_caches(),
        };
        let api = filters::todos(env);

        let resp = request()
            .method("POST")
            .path("/todos")
            .json(&Todo {
                id: 1,
                text: "test 1".into(),
                completed: false,
            })
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_post_conflict() {
        let env = models::Environment {
            db: models::blank_db(),
            caches: models::blank_caches(),
        };
        env.db.lock().await.push(todo1());
        let api = filters::todos(env);

        let resp = request()
            .method("POST")
            .path("/todos")
            .json(&todo1())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_put_unknown() {
        let env = models::Environment {
            db: models::blank_db(),
            caches: models::blank_caches(),
        };
        let api = filters::todos(env);

        let resp = request()
            .method("PUT")
            .path("/todos/1")
            .header("authorization", "Bearer admin")
            .json(&todo1())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    fn todo1() -> Todo {
        Todo {
            id: 1,
            text: "test 1".into(),
            completed: false,
        }
    }
}
