use hyper::{Body, Response};

use crate::db;
use crate::html;
use crate::models;

use crate::http::util;

pub fn handle_get(game_id_str: &str) -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();

    let game_id = match game_id_str.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => return util::bad_request_response("must supply map id as u32"),
    };

    let game = match db.get_game(game_id) {
        Ok(d) => d,
        Err(e) => return util::db_error_page(e),
    };

    Ok(Response::new(Body::from(html::common::render_page(
        html::pages::game_edit::page(game),
    ))))
}

pub fn handle_post_set_value(
    game_id_str: &str,
    value_type: util::TerrainOrCharacter,
    value_value: &str,
) -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();

    let game_id = match game_id_str.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => return util::bad_request_response("must supply map id as u32"),
    };

    match match value_type {
        util::TerrainOrCharacter::Terrain => db.update_game_terrain(
            game_id,
            match models::Terrain::parse_str(value_value) {
                Some(v) => v,
                None => return util::bad_request_response("terrain in path invalid"),
            },
        ),
        util::TerrainOrCharacter::Character => db.update_game_character(
            game_id,
            match models::Character::parse_str(value_value) {
                Some(v) => v,
                None => return util::bad_request_response("character in path invalid"),
            },
        ),
    } {
        Ok(()) => (),
        Err(e) => return util::db_error_page(e),
    };

    handle_get(game_id_str)
}

pub fn handle_post_unset_value(
    game_id_str: &str,
    value_type: util::TerrainOrCharacter,
) -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();

    let game_id = match game_id_str.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => return util::bad_request_response("must supply map id as u32"),
    };

    match match value_type {
        util::TerrainOrCharacter::Terrain => db.unset_game_terrain(game_id),
        util::TerrainOrCharacter::Character => db.unset_game_character(game_id),
    } {
        Ok(()) => (),
        Err(e) => return util::db_error_page(e),
    };

    handle_get(game_id_str)
}
