use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::units;

use super::models;

pub fn render_page<'a>(body: elements::Body<'a>) -> String {
    let html = elements::Html {
        lang: attributes::Lang {
            tag: units::LanguageTag::En,
            sub_tag: units::LanguageSubTag::Us,
        },
        styles: attributes::StyleAttr::empty(),
        head: Some(elements::Head {
            metas: vec![elements::Meta {
                charset: Option::Some(attributes::Charset {
                    value: units::CharsetValue::Utf8,
                }),
                styles: attributes::StyleAttr::empty(),
            }],
            styles: Vec::new(),
        }),
        body: Some(body),
    };

    htmldsl::render_simple_html_page(true, html)
}

impl models::Terrain {
    fn into_html(&self) -> htmldsl::Element {
        htmldsl::tag(Box::new(elements::Img::style_less(attributes::Src {
            value: units::SourceValue::new(format!("/images/{}.png", self.image_name())),
        })))
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
    fn into_html(&self) -> htmldsl::Element {
        htmldsl::tag(Box::new(elements::Table::style_less_from_vecs(
            None,
            vec![vec![self.default_terrain.into_html()]],
        )))
    }
}

pub fn index<'a>() -> elements::Body<'a> {
    elements::Body::style_less(vec![
        htmldsl::tag(Box::new(elements::H1::style_less(vec![htmldsl::text(
            "hi there".into(),
        )]))),
        models::Map {
            default_terrain: models::Terrain::Grass,
        }
        .into_html(),
    ])
}

pub fn not_found<'a>() -> elements::Body<'a> {
    elements::Body::style_less(vec![htmldsl::tag(Box::new(elements::H1::style_less(
        vec![htmldsl::text("not found".into())],
    )))])
}

pub fn internal_server_error<'a>() -> elements::Body<'a> {
    elements::Body::style_less(vec![htmldsl::tag(Box::new(elements::H1::style_less(
        vec![htmldsl::text("internal server error".into())],
    )))])
}

pub fn bad_request<'a, T: Into<String>>(message: T) -> elements::Body<'a> {
    elements::Body::style_less(vec![
        htmldsl::tag(Box::new(elements::H1::style_less(vec![htmldsl::text(
            "bad request".into(),
        )]))),
        htmldsl::tag(Box::new(elements::P::style_less(vec![htmldsl::text(
            message.into(),
        )]))),
    ])
}
