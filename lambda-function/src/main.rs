use lambda_http::{
    http::Method, http::StatusCode, run, service_fn, Body, Error, IntoResponse, Request,
    RequestExt, Response,
};
use profiler;
use serde_json;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

pub async fn function_handler(request: Request) -> Result<impl IntoResponse, Error> {
    match (
        request.method(),
        request.uri().path(),
        request.query_string_parameters().first("domain"),
        request.query_string_parameters().first("concurrency"),
    ) {
        (&Method::POST, "/", Some(d), Some(c)) => {
            info!("Profiling {d} webserver {c} time(s)");
            let num_of_tasks: usize = c.parse().unwrap();
            let stats = profiler::benchmarks(&d, num_of_tasks).await.unwrap();
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&stats).unwrap()))
                .unwrap();
            info!("POST / - {}", response.status());
            Ok(response)
        }
        (method, path, _, _) => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::Empty)
                .unwrap();
            warn!("{method} {path} - {}", response.status());
            Ok(response)
        }
    }
}
