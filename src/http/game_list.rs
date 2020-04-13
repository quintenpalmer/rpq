use hyper::{Body, Response};

use crate::db;
use crate::html;

use super::util;

pub fn handle_get() -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();
    let games = match db.get_games() {
        Ok(d) => d,
        Err(e) => return util::db_error_page(e),
    };

    Ok(Response::new(Body::from(html::render_page(html::games(
        games,
    )))))
}

pub fn handle_post() -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();
    match db.add_game() {
        Ok(()) => (),
        Err(e) => return util::db_error_page(e),
    };

    handle_get()
}