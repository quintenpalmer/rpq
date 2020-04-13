use hyper::{Body, Response};

use crate::db;
use crate::html;

use crate::http::util;

pub fn handle_get() -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();
    let games = match db.get_maps() {
        Ok(d) => d,
        Err(e) => return util::db_error_page(e),
    };

    Ok(Response::new(Body::from(html::common::render_page(
        html::pages::map_list::page(games),
    ))))
}
