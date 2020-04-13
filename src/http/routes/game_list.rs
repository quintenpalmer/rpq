use hyper::{Body, Response};

use crate::db;
use crate::html;

use crate::http::util;

pub fn handle_get() -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();
    let games = match db.get_games() {
        Ok(d) => d,
        Err(e) => return util::db_error_page(e),
    };

    Ok(Response::new(Body::from(html::common::render_page(
        html::pages::game_list::page(games),
    ))))
}

pub fn handle_post() -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();
    match db.add_game() {
        Ok(()) => (),
        Err(e) => return util::db_error_page(e),
    };

    handle_get()
}
