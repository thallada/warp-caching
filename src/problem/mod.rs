use http::StatusCode;
use http_api_problem::HttpApiProblem;
use tracing::error;
use warp::{reject, Rejection, Reply};

pub fn from_anyhow(error: anyhow::Error) -> HttpApiProblem {
    let error = match error.downcast::<HttpApiProblem>() {
        Ok(problem) => return problem,
        Err(error) => error,
    };

    error!("Recovering unhandled error: {:?}", error);
    HttpApiProblem::new("Internal Server Error: 500").set_status(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn unpack_problem(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(problem) = rejection.find::<HttpApiProblem>() {
        let code = problem.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let reply = warp::reply::json(problem);
        let reply = warp::reply::with_status(reply, code);
        let reply = warp::reply::with_header(
            reply,
            warp::http::header::CONTENT_TYPE,
            http_api_problem::PROBLEM_JSON_MEDIA_TYPE,
        );

        return Ok(reply);
    }

    Err(rejection)
}

pub fn reject_anyhow(error: anyhow::Error) -> Rejection {
    reject::custom(from_anyhow(error))
}
