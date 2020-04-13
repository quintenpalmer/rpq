use hyper::{Body, Response};

use crate::db;
use crate::html;
use crate::models;

use super::util;

pub fn handle_post(
    game_id_str: &str,
    direction_str: &str,
    edit: bool,
) -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();

    let game_id = match game_id_str.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => return util::bad_request_response("must supply map id as u32"),
    };

    let mut game = match db.get_game(game_id) {
        Ok(d) => d,
        Err(e) => return util::internal_server_error(e),
    };

    let direction = match models::Direction::parse(direction_str) {
        Some(d) => d,
        None => {
            return util::bad_request_response("direction must be one of right, up, left, down")
        }
    };

    game.move_cursor(direction);

    match db.update_game_cursor(game.id, game.current_selection) {
        Ok(()) => (),
        Err(e) => return util::internal_server_error(e),
    };

    Ok(Response::new(Body::from(html::common::render_page(
        if edit {
            html::pages::game_edit::page(game)
        } else {
            html::pages::game_single::page(game)
        },
    ))))
}
