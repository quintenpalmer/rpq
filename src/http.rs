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
