use hyper::{Body, Response};

use crate::db;
use crate::html;

use super::util;

pub fn handle_get(game_id_str: &str) -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();

    let game_id = match game_id_str.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => return util::bad_request_response("must supply game id as u32"),
    };

    let game = match db.get_game(game_id) {
        Ok(d) => d,
        Err(e) => return util::db_error_page(e),
    };

    Ok(Response::new(Body::from(html::render_page(html::game(
        game,
    )))))
}
