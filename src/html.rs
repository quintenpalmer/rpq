use std::collections::BTreeMap;

use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::styles;
use htmldsl::units;
use htmldsl::TagRenderableStyleSetter;

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
        htmldsl::tag(Box::new(
            elements::Img::style_less_with_src(format!("/images/{}.png", self.image_name()))
                .add_style(vec![&styles::Display::Block]),
        ))
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
        let (max_x, max_y) = self.maxes();
        let mut empty_rendered_map: Vec<Vec<models::Terrain>> = (0..max_x)
            .into_iter()
            .map(|_| {
                (0..max_y)
                    .into_iter()
                    .map(|_| self.default_terrain.clone())
                    .collect()
            })
            .collect();
        for ((x, y), terrain) in self.specified_terrain.iter() {
            empty_rendered_map[*x as usize][*y as usize] = terrain.clone();
        }

        htmldsl::tag(Box::new(elements::Table::style_less(
            None,
            elements::Tbody::style_less(
                empty_rendered_map
                    .into_iter()
                    .map(|row| {
                        elements::Tr::style_less(
                            row.into_iter()
                                .map(|data| elements::Td::style_less(vec![data.into_html()]))
                                .collect(),
                        )
                    })
                    .collect(),
            ),
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
            specified_terrain: vec![
                ((3, 3), models::Terrain::Dirt),
                ((3, 4), models::Terrain::Rock),
            ]
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
            hint_max_x: 17,
            hint_max_y: 17,
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
