use hyper::{Body, Response};
use std::fs::File;
use std::io::{ErrorKind, Read};

use super::util;

pub fn handle_get(file_name: &str) -> Result<Response<Body>, hyper::Error> {
    let (name, suffix) = match file_name.split('.').collect::<Vec<&str>>().as_slice() {
        &[name, suffix] => (name, suffix),
        _ => return util::bad_request_response("images must be 'file.ext'"),
    };

    let ext = match suffix {
        "png" => ImageFileType::PNG,
        _ => return util::bad_request_response("only .png image file type is supported"),
    };

    match validate_file_name(name) {
        Ok(()) => (),
        Err(e) => return util::bad_request_response(format!("image file invalid: {}", e)),
    };
    serve_file(format!("images/{}.{}", name, ext.extension()))
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
            ErrorKind::NotFound => return util::not_found_response(),
            _ => return util::internal_server_error(format!("file open failed: {:?}", e)),
        },
    };

    let mut source = Vec::new();

    match f.read_to_end(&mut source) {
        Ok(_) => (),
        Err(e) => return util::internal_server_error(format!("file read to end failed: {:?}", e)),
    };

    Ok(Response::new(Body::from(source)))
}
