use hyper::{Body, Response, StatusCode};

use crate::db;
use crate::html;

pub enum TerrainOrCharacter {
    Terrain,
    Character,
}

pub fn db_error_page(e: db::DBError) -> Result<Response<Body>, hyper::Error> {
    match e {
        db::DBError::FindingTable(_) => internal_server_error(e),
        db::DBError::ParsingRecord(_) => internal_server_error(e),
        db::DBError::FindingRecord(r) => not_found_response(r),
        db::DBError::Internal(_) => internal_server_error(e),
    }
}

pub fn internal_server_error<T: std::fmt::Debug>(
    log_message: T,
) -> Result<Response<Body>, hyper::Error> {
    println!("internal server error: {:?}", log_message);
    let mut not_found = Response::new(Body::from(html::common::render_page(
        html::common::internal_server_error(),
    )));
    *not_found.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    Ok(not_found)
}

pub fn not_found_response<T: std::fmt::Debug>(
    log_message: T,
) -> Result<Response<Body>, hyper::Error> {
    println!("not found: {:?}", log_message);
    let mut not_found = Response::new(Body::from(html::common::render_page(
        html::common::not_found(),
    )));
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

pub fn bad_request_response<T: Into<String>>(message: T) -> Result<Response<Body>, hyper::Error> {
    let mut not_found = Response::new(Body::from(html::common::render_page(
        html::common::bad_request(message),
    )));
    *not_found.status_mut() = StatusCode::BAD_REQUEST;
    Ok(not_found)
}
