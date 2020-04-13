use hyper::{Body, Response};

use crate::db;
use crate::html;

use super::util;

pub fn handle_get(map_id_str: &str) -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();

    let map_id = match map_id_str.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => return util::bad_request_response("must supply map id as u32"),
    };

    let map = match db.get_map(map_id) {
        Ok(d) => d,
        Err(e) => return util::db_error_page(e),
    };

    Ok(Response::new(Body::from(html::render_page(html::map(map)))))
}
