use hyper::{Body, Method, Request, Response};

use crate::db;
use crate::html;
use crate::models;

use super::cursor_move;
use super::game_list;
use super::game_single;
use super::image_serve;
use super::index;
use super::map_list;
use super::map_single;
use super::util;

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
        // Serve some instructions at /
        (&Method::GET, []) => index::handle_get(),

        (&Method::GET, ["maps"]) => map_list::handle_get(),
        (&Method::GET, ["maps", map_id]) => map_single::handle_get(map_id),

        (&Method::GET, ["games"]) => game_list::handle_get(),
        (&Method::POST, ["games"]) => game_list::handle_post(),
        (&Method::GET, ["games", game_id]) => game_single::handle_get(game_id),
        (&Method::GET, ["games", game_id, "edit"]) => edit_game_response(game_id),
        (&Method::POST, ["games", game_id, "edit", "character", character_str]) => {
            edit_game_set_value(game_id, util::TerrainOrCharacter::Character, character_str)
        }
        (&Method::POST, ["games", game_id, "edit", "terrain", terrain_str]) => {
            edit_game_set_value(game_id, util::TerrainOrCharacter::Terrain, terrain_str)
        }

        (&Method::POST, ["games", game_id, "edit", "unset", "character"]) => {
            edit_game_unset_value(game_id, util::TerrainOrCharacter::Character)
        }
        (&Method::POST, ["games", game_id, "edit", "unset", "terrain"]) => {
            edit_game_unset_value(game_id, util::TerrainOrCharacter::Terrain)
        }

        (&Method::POST, ["games", game_id, "edit", "cursor", direction]) => {
            cursor_move::handle_post(game_id, direction, true)
        }

        (&Method::POST, ["games", game_id, "cursor", direction]) => {
            cursor_move::handle_post(game_id, direction, false)
        }

        // Serve hard-coded images
        (&Method::GET, ["images", name]) => image_serve::handle_get(name),

        // Return the 404 Not Found for other routes.
        _ => util::not_found_response(),
    }
}

fn edit_game_response(game_id_str: &str) -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();

    let game_id = match game_id_str.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => return util::bad_request_response("must supply map id as u32"),
    };

    let game = match db.get_game(game_id) {
        Ok(d) => d,
        Err(e) => return util::db_error_page(e),
    };

    Ok(Response::new(Body::from(html::render_page(
        html::edit_game(game),
    ))))
}

fn edit_game_set_value(
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

    edit_game_response(game_id_str)
}

fn edit_game_unset_value(
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

    edit_game_response(game_id_str)
}
