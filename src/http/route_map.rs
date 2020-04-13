use hyper::{Body, Method, Request, Response};

use super::cursor_move;
use super::game_edit;
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
        (&Method::GET, ["games", game_id, "edit"]) => game_edit::handle_get(game_id),
        (&Method::POST, ["games", game_id, "edit", "character", character_str]) => {
            game_edit::handle_post_set_value(
                game_id,
                util::TerrainOrCharacter::Character,
                character_str,
            )
        }
        (&Method::POST, ["games", game_id, "edit", "terrain", terrain_str]) => {
            game_edit::handle_post_set_value(
                game_id,
                util::TerrainOrCharacter::Terrain,
                terrain_str,
            )
        }

        (&Method::POST, ["games", game_id, "edit", "unset", "character"]) => {
            game_edit::handle_post_unset_value(game_id, util::TerrainOrCharacter::Character)
        }
        (&Method::POST, ["games", game_id, "edit", "unset", "terrain"]) => {
            game_edit::handle_post_unset_value(game_id, util::TerrainOrCharacter::Terrain)
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
