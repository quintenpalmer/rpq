use std::collections::BTreeMap;

use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::style_sheet;
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
            styles: vec![elements::Style {
                style_sheet: style_sheet::StyleSheet {
                    assignments: vec![
                        style_sheet::StyleAssignment {
                            names: vec![
                                "html", "body", "span", "div", "h1", "h2", "h3", "h4", "h5", "h6",
                                "p", "pre", "a", "code", "img", "b", "u", "i", "ul", "ol", "li",
                                "table", "tbody", "thead", "tfoot", "tr", "th", "td",
                            ]
                            .into_iter()
                            .map(|x| x.to_string())
                            .collect(),
                            styles: vec![
                                &styles::Border {
                                    style: units::BorderStyle::None,
                                },
                                &styles::Margin::AllFour(units::NumberOrAuto::Number(
                                    units::Number::Length(0, units::Length::Pixel),
                                )),
                                &styles::Padding::AllFour(units::Number::Length(
                                    0,
                                    units::Length::Pixel,
                                )),
                                &styles::VerticalAlign {
                                    value: units::VerticalAlignValue::Baseline,
                                },
                            ],
                        },
                        style_sheet::StyleAssignment {
                            names: vec!["table".into()],
                            styles: vec![
                                &styles::BorderCollapse {
                                    value: units::BorderCollapseStyle::Collapse,
                                },
                                &styles::BorderSpacing {
                                    value: units::Number::Length(0, units::Length::Pixel),
                                },
                            ],
                        },
                    ],
                },
            }],
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

impl models::Character {
    fn into_html(&self) -> htmldsl::Element {
        htmldsl::tag(Box::new(
            elements::Img::style_less_with_src(format!("/images/{}.png", self.image_name()))
                .add_style(vec![&styles::Display::Block, &styles::Position::Absolute]),
        ))
    }

    fn image_name(&self) -> String {
        match self {
            models::Character::Knight => "knight",
            models::Character::Mage => "mage",
            models::Character::Thief => "thief",
        }
        .into()
    }
}

impl models::Map {
    fn into_html(&self) -> htmldsl::Element {
        let (max_x, max_y) = self.maxes();
        let mut empty_rendered_map: Vec<Vec<(models::Terrain, Option<models::Character>)>> = (0
            ..max_x)
            .into_iter()
            .map(|_| {
                (0..max_y)
                    .into_iter()
                    .map(|_| (self.default_terrain.clone(), None))
                    .collect()
            })
            .collect();
        for ((x, y), terrain) in self.specified_terrain.iter() {
            empty_rendered_map[*x as usize][*y as usize].0 = terrain.clone();
        }
        for ((x, y), character) in self.characters.iter() {
            empty_rendered_map[*x as usize][*y as usize].1 = Some(character.clone());
        }

        htmldsl::tag(Box::new(elements::Table::style_less(
            None,
            elements::Tbody::style_less(
                empty_rendered_map
                    .into_iter()
                    .map(|row| {
                        elements::Tr::style_less(
                            row.into_iter()
                                .map(|data| {
                                    elements::Td::style_less(maybe_append(
                                        vec![data.0.into_html()],
                                        data.1.map(|x| x.into_html()),
                                    ))
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            ),
        )))
    }
}

fn maybe_append<T>(mut vec: Vec<T>, maybe: Option<T>) -> Vec<T> {
    match maybe {
        Some(v) => {
            vec.push(v);
            vec
        }
        None => vec,
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
            characters: vec![((4, 3), models::Character::Knight)]
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
