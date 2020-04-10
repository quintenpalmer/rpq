use hyper::{Body, Request, Response, StatusCode};

use super::html;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
pub async fn service_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path_frags = req
        .uri()
        .path()
        .split('/')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    println!(
        "responding to: {} ({:?}) ({})",
        req.uri().path(),
        path_frags,
        req.method()
    );
    match (req.method(), path_frags.as_slice()) {
        // Return the 404 Not Found for other routes.
        _ => not_found_response(),
    }
}

fn not_found_response() -> Result<Response<Body>, hyper::Error> {
    let mut not_found = Response::new(Body::from(html::not_found()));
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

fn internal_server_error<T: Into<String>>(log_message: T) -> Result<Response<Body>, hyper::Error> {
    println!("internal server error: {}", log_message.into());
    let mut not_found = Response::new(Body::from(html::internal_server_error()));
    *not_found.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    Ok(not_found)
}

fn bad_request_response<T: Into<String>>(message: T) -> Result<Response<Body>, hyper::Error> {
    let mut not_found = Response::new(Body::from(html::bad_request(message)));
    *not_found.status_mut() = StatusCode::BAD_REQUEST;
    Ok(not_found)
}
