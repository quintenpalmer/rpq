use hyper::{Body, Response};

use crate::html;

pub fn handle_get() -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from(html::common::render_page(
        html::pages::index::page(),
    ))))
}
