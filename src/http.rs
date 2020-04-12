use hyper::{Body, Method, Request, Response, StatusCode};
use std::fs::File;
use std::io::{ErrorKind, Read};

use super::db;
use super::html;
use super::models;

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
        (&Method::GET, []) => index_response(),

        (&Method::GET, ["maps"]) => maps_response(),
        (&Method::GET, ["maps", map_id]) => map_response(map_id),

        (&Method::GET, ["games"]) => games_response(),
        (&Method::POST, ["games"]) => games_create_response(),
        (&Method::GET, ["games", game_id]) => game_response(game_id),
        (&Method::GET, ["games", game_id, "edit"]) => edit_game_response(game_id),
        (&Method::POST, ["games", game_id, "edit", "character", character_str]) => {
            edit_game_set_value(game_id, TerrainOrCharacter::Character, character_str)
        }
        (&Method::POST, ["games", game_id, "edit", "terrain", terrain_str]) => {
            edit_game_set_value(game_id, TerrainOrCharacter::Terrain, terrain_str)
        }

        (&Method::POST, ["games", game_id, "edit", "unset", "character"]) => {
            edit_game_unset_value(game_id, TerrainOrCharacter::Character)
        }
        (&Method::POST, ["games", game_id, "edit", "unset", "terrain"]) => {
            edit_game_unset_value(game_id, TerrainOrCharacter::Terrain)
        }

        (&Method::POST, ["games", game_id, "edit", "cursor", direction]) => {
            move_cursor(game_id, direction, true)
        }

        (&Method::POST, ["games", game_id, "cursor", direction]) => {
            move_cursor(game_id, direction, false)
        }

        // Serve hard-coded images
        (&Method::GET, ["images", name]) => serve_image(name),

        // Return the 404 Not Found for other routes.
        _ => not_found_response(),
    }
}

enum TerrainOrCharacter {
    Terrain,
    Character,
}

fn index_response() -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from(html::render_page(html::index()))))
}

fn maps_response() -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();
    let games = match db.get_maps() {
        Ok(d) => d,
        Err(e) => return internal_server_error(e),
    };

    Ok(Response::new(Body::from(html::render_page(html::maps(
        games,
    )))))
}

fn map_response(map_id_str: &str) -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();

    let map_id = match map_id_str.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => return bad_request_response("must supply map id as u32"),
    };

    let map = match db.get_map(map_id) {
        Ok(d) => d,
        Err(e) => return internal_server_error(e),
    };

    Ok(Response::new(Body::from(html::render_page(html::map(map)))))
}

fn games_response() -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();
    let games = match db.get_games() {
        Ok(d) => d,
        Err(e) => return internal_server_error(e),
    };

    Ok(Response::new(Body::from(html::render_page(html::games(
        games,
    )))))
}

fn games_create_response() -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();
    match db.add_game() {
        Ok(()) => (),
        Err(e) => return internal_server_error(e),
    };

    games_response()
}

fn game_response(game_id_str: &str) -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();

    let game_id = match game_id_str.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => return bad_request_response("must supply map id as u32"),
    };

    let game = match db.get_game(game_id) {
        Ok(d) => d,
        Err(e) => return internal_server_error(e),
    };

    Ok(Response::new(Body::from(html::render_page(html::game(
        game,
    )))))
}

fn edit_game_response(game_id_str: &str) -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();

    let game_id = match game_id_str.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => return bad_request_response("must supply map id as u32"),
    };

    let game = match db.get_game(game_id) {
        Ok(d) => d,
        Err(e) => return internal_server_error(e),
    };

    Ok(Response::new(Body::from(html::render_page(
        html::edit_game(game),
    ))))
}

fn edit_game_set_value(
    game_id_str: &str,
    value_type: TerrainOrCharacter,
    value_value: &str,
) -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();

    let game_id = match game_id_str.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => return bad_request_response("must supply map id as u32"),
    };

    match match value_type {
        TerrainOrCharacter::Terrain => db.update_game_terrain(
            game_id,
            match models::Terrain::parse_str(value_value) {
                Some(v) => v,
                None => return bad_request_response("terrain in path invalid"),
            },
        ),
        TerrainOrCharacter::Character => db.update_game_character(
            game_id,
            match models::Character::parse_str(value_value) {
                Some(v) => v,
                None => return bad_request_response("character in path invalid"),
            },
        ),
    } {
        Ok(()) => (),
        Err(e) => {
            return internal_server_error(format!("could not update terrain or character: {:?}", e))
        }
    };

    edit_game_response(game_id_str)
}

fn edit_game_unset_value(
    game_id_str: &str,
    value_type: TerrainOrCharacter,
) -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();

    let game_id = match game_id_str.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => return bad_request_response("must supply map id as u32"),
    };

    match match value_type {
        TerrainOrCharacter::Terrain => db.unset_game_terrain(game_id),
        TerrainOrCharacter::Character => db.unset_game_character(game_id),
    } {
        Ok(()) => (),
        Err(e) => {
            return internal_server_error(format!("could not unset terrain or character: {:?}", e))
        }
    };

    edit_game_response(game_id_str)
}

fn move_cursor(
    game_id_str: &str,
    direction_str: &str,
    edit: bool,
) -> Result<Response<Body>, hyper::Error> {
    let db = db::DB::new();

    let game_id = match game_id_str.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => return bad_request_response("must supply map id as u32"),
    };

    let mut game = match db.get_game(game_id) {
        Ok(d) => d,
        Err(e) => return internal_server_error(e),
    };

    let direction = match models::Direction::parse(direction_str) {
        Some(d) => d,
        None => return bad_request_response("direction must be one of right, up, left, down"),
    };

    game.move_cursor(direction);

    match db.update_game_cursor(game.id, game.current_selection) {
        Ok(()) => (),
        Err(e) => return internal_server_error(e),
    };

    Ok(Response::new(Body::from(html::render_page(if edit {
        html::edit_game(game)
    } else {
        html::game(game)
    }))))
}

fn not_found_response() -> Result<Response<Body>, hyper::Error> {
    let mut not_found = Response::new(Body::from(html::render_page(html::not_found())));
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

fn internal_server_error<T: Into<String>>(log_message: T) -> Result<Response<Body>, hyper::Error> {
    println!("internal server error: {}", log_message.into());
    let mut not_found = Response::new(Body::from(html::render_page(html::internal_server_error())));
    *not_found.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    Ok(not_found)
}

fn bad_request_response<T: Into<String>>(message: T) -> Result<Response<Body>, hyper::Error> {
    let mut not_found = Response::new(Body::from(html::render_page(html::bad_request(message))));
    *not_found.status_mut() = StatusCode::BAD_REQUEST;
    Ok(not_found)
}

pub enum ImageFileType {
    PNG,
}

impl ImageFileType {
    pub fn extension(&self) -> &'static str {
        match self {
            ImageFileType::PNG => "png",
        }
    }
}

fn serve_image(file_name: &str) -> Result<Response<Body>, hyper::Error> {
    let (name, suffix) = match file_name.split('.').collect::<Vec<&str>>().as_slice() {
        &[name, suffix] => (name, suffix),
        _ => return bad_request_response("images must be 'file.ext'"),
    };

    let ext = match suffix {
        "png" => ImageFileType::PNG,
        _ => return bad_request_response("only .png image file type is supported"),
    };

    match validate_file_name(name) {
        Ok(()) => (),
        Err(e) => return bad_request_response(format!("image file invalid: {}", e)),
    };
    serve_file(format!("images/{}.{}", name, ext.extension()))
}

fn validate_file_name(name: &str) -> Result<(), &'static str> {
    for c in name.chars() {
        if !is_alpha_numeric_underscore(c) {
            return Err("must contain only ascii alphanumeric and '_' characters");
        }
    }
    return Ok(());
}

fn is_alpha_numeric_underscore(c: char) -> bool {
    return c.is_ascii_alphanumeric() || c == '_';
}

pub fn serve_file(path: String) -> Result<Response<Body>, hyper::Error> {
    let mut f = match File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return not_found_response(),
            _ => return internal_server_error(format!("file open failed: {:?}", e)),
        },
    };

    let mut source = Vec::new();

    match f.read_to_end(&mut source) {
        Ok(_) => (),
        Err(e) => return internal_server_error(format!("file read to end failed: {:?}", e)),
    };

    Ok(Response::new(Body::from(source)))
}
