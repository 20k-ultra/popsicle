use regex::Regex;
use std::convert::Infallible;
use std::io::Write;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

mod log;
mod profiler;

async fn handler(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    let pattern = Regex::new(r"^/profile/([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})$").unwrap();
    match (request.method(), pattern.captures(request.uri().path())) {
        (&Method::POST, Some(matches)) => {
            let domain = matches.get(1).unwrap().as_str();
            let stats = profiler::profile(domain).unwrap();
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(stats.to_string()))
                .unwrap();
            info!("GET /profile - {}", response.status());
            Ok(response)
        }
        (method, _) => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            warn!("{method} {} - {}", request.uri().path(), response.status());
            Ok(response)
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handler)) });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    debug!("Listening on: {}", addr);

    server.await?;

    Ok(())
}
