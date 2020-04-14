use hyper::{header, Body, Method, Request, Response};

use super::routes;
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
        // Serve hard-coded images
        (&Method::GET, ["images", name]) => routes::image_serve::handle_get(name),

        (method, frags) => handle_pages(method, frags),
    }
}

fn handle_pages(method: &Method, frags: &[&str]) -> Result<Response<Body>, hyper::Error> {
    match (method, frags) {
        // Serve some instructions at /
        (&Method::GET, []) => routes::index::handle_get(),

        (&Method::GET, ["maps"]) => routes::map_list::handle_get(),

        (&Method::GET, ["maps", map_id]) => routes::map_single::handle_get(map_id),

        (&Method::GET, ["games"]) => routes::game_list::handle_get(),
        (&Method::POST, ["games"]) => routes::game_list::handle_post(),
        (&Method::GET, ["games", game_id]) => routes::game_single::handle_get(game_id),
        (&Method::GET, ["games", game_id, "edit"]) => routes::game_edit::handle_get(game_id),
        (&Method::POST, ["games", game_id, "edit", "character", character_str]) => {
            routes::game_edit::handle_post_set_value(
                game_id,
                util::TerrainOrCharacter::Character,
                character_str,
            )
        }
        (&Method::POST, ["games", game_id, "edit", "terrain", terrain_str]) => {
            routes::game_edit::handle_post_set_value(
                game_id,
                util::TerrainOrCharacter::Terrain,
                terrain_str,
            )
        }

        (&Method::POST, ["games", game_id, "edit", "unset", "character"]) => {
            routes::game_edit::handle_post_unset_value(game_id, util::TerrainOrCharacter::Character)
        }
        (&Method::POST, ["games", game_id, "edit", "unset", "terrain"]) => {
            routes::game_edit::handle_post_unset_value(game_id, util::TerrainOrCharacter::Terrain)
        }

        (&Method::POST, ["games", game_id, "edit", "cursor", direction]) => {
            routes::cursor_move::handle_post(game_id, direction, true)
        }

        (&Method::POST, ["games", game_id, "cursor", direction]) => {
            routes::cursor_move::handle_post(game_id, direction, false)
        }

        // Return the 404 Not Found for other routes.
        _ => util::not_found_response(),
    }
    .map(|mut resp| {
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("text/html"),
        );
        resp
    })
}
