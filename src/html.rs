use super::models;

impl models::Terrain {
    fn into_html(&self) -> String {
        format!("<img src=\"/images/{}.png\"/>", self.image_name())
    }

    fn image_name(&self) -> String {
        match self {
            models::Terrain::Grass => "grass",
            models::Terrain::Dirt => "dirt",
            models::Terrain::Rock => "rock",
        }
        .into()
    }
}

impl models::Map {
    fn into_html(&self) -> String {
        format!(
            "<table><tbody><tr><td>{}</td></tr></tbody></table>",
            self.default_terrain.into_html()
        )
    }
}

pub fn index() -> String {
    format!(
        "<html><body><h1>hi there</h1><p>{}</p></body></html>",
        models::Map {
            default_terrain: models::Terrain::Grass,
        }
        .into_html()
    )
}

pub fn not_found() -> String {
    "<html><body><h1>not found</h1></body></html>".into()
}

pub fn internal_server_error() -> String {
    "<html><body><h1>internal server error</h1></body></html>".into()
}

pub fn bad_request<T: Into<String>>(message: T) -> String {
    format!(
        "<html><body><h1>bad request</h1><p>{}</p></body></html>",
        message.into()
    )
}
