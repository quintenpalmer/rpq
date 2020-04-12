use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::style_sheet;
use htmldsl::styles;
use htmldsl::units;
use htmldsl::{TagRenderableIntoElement, TagRenderableStyleSetter};

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

fn current_selection_marker<'a>() -> elements::Img<'a> {
    elements::Img::style_less_with_src("/images/marker.png".to_string())
}

fn absolute_hover<'a, T: TagRenderableStyleSetter<'a>>(element: T) -> T {
    element.add_style(vec![
        &styles::Display::Block,
        &styles::Position::Absolute,
        &styles::Top {
            value: units::Number::Length(0, units::Length::Pixel),
        },
        &styles::Left {
            value: units::Number::Length(0, units::Length::Pixel),
        },
    ])
}

impl models::Terrain {
    fn into_html<'a>(self) -> elements::Img<'a> {
        elements::Img::style_less_with_src(format!("/images/{}.png", self.image_name()))
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
    fn into_html<'a>(self) -> elements::Img<'a> {
        elements::Img::style_less_with_src(format!("/images/{}.png", self.image_name()))
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
    fn into_html(&self, current_selection: Option<(u32, u32)>) -> htmldsl::Element {
        let (max_x, max_y) = self.maxes();
        let mut empty_rendered_map: Vec<Vec<(models::Terrain, Option<models::Character>, bool)>> =
            (0..max_y)
                .into_iter()
                .map(|_| {
                    (0..max_x)
                        .into_iter()
                        .map(|_| (self.default_terrain.clone(), None, false))
                        .collect()
                })
                .collect();
        for ((x, y), terrain) in self.specified_terrain.iter() {
            empty_rendered_map[(max_y - *y - 1) as usize][*x as usize].0 = terrain.clone();
        }
        for ((x, y), character) in self.characters.iter() {
            empty_rendered_map[(max_y - *y - 1) as usize][*x as usize].1 = Some(character.clone());
        }
        match current_selection {
            Some((x, y)) => empty_rendered_map[(max_y - y - 1) as usize][x as usize].2 = true,
            None => (),
        };

        elements::Table::style_less(
            None,
            elements::Tbody::style_less(
                empty_rendered_map
                    .into_iter()
                    .map(|row| {
                        elements::Tr::style_less(
                            row.into_iter()
                                .map(|data| {
                                    elements::Td::style_less(vec![elements::Div::style_less(
                                        maybe_append(
                                            maybe_append(
                                                vec![data
                                                    .0
                                                    .into_html()
                                                    .add_style(vec![&styles::Display::Block])
                                                    .into_element()],
                                                data.1.map(|x| {
                                                    absolute_hover(x.into_html()).into_element()
                                                }),
                                            ),
                                            if data.2 {
                                                Some(
                                                    absolute_hover(current_selection_marker())
                                                        .into_element(),
                                                )
                                            } else {
                                                None
                                            },
                                        ),
                                    )
                                    .add_style(vec![&styles::Position::Relative])
                                    .into_element()])
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            ),
        )
        .into_element()
    }
}

impl models::Game {
    fn into_html(&self, edit: bool) -> htmldsl::Element {
        let (terrain, o_character) = self.map.at(&self.current_selection);

        let hover_info = elements::Div::style_less(vec![
            elements::P::style_less(maybe_append(
                vec![
                    terrain.clone().into_html().into_element(),
                    htmldsl::text("Terrain: ".into()),
                    htmldsl::text(terrain.display_string()),
                ],
                if edit {
                    Some(build_terrain_adding_buttons(self.id).into_element())
                } else {
                    None
                },
            ))
            .into_element(),
            elements::P::style_less(maybe_append(
                vec![
                    o_character
                        .clone()
                        .map_or(current_selection_marker().into_element(), |x| {
                            x.into_html().into_element()
                        }),
                    htmldsl::text("Character: ".into()),
                    htmldsl::text(match o_character {
                        Some(v) => v.display_string(),
                        None => "--".into(),
                    }),
                ],
                if edit {
                    Some(build_character_adding_buttons(self.id).into_element())
                } else {
                    None
                },
            ))
            .into_element(),
        ])
        .add_style(vec![
            &styles::Display::InlineBlock,
            &styles::Width {
                value: units::NumberOrAuto::Number(units::Number::Length(
                    200,
                    units::Length::Pixel,
                )),
            },
            &styles::Height {
                value: units::NumberOrAuto::Number(units::Number::Length(
                    300,
                    units::Length::Pixel,
                )),
            },
            &styles::Border {
                style: units::BorderStyle::Solid,
            },
        ])
        .into_element();

        elements::Table::style_less(
            None,
            elements::Tbody::style_less(vec![elements::Tr::style_less(vec![
                elements::Td::style_less(vec![self.map.into_html(Some(self.current_selection))]),
                elements::Td::style_less(vec![hover_info]),
            ])]),
        )
        .into_element()
    }
}

fn build_terrain_adding_buttons<'a>(game_id: u32) -> elements::Div<'a> {
    elements::Div::style_less(
        models::Terrain::all_values()
            .into_iter()
            .map(|x| {
                elements::Form {
                    formmethod: attributes::Formmethod {
                        inner: units::FormmethodValue::Post,
                    },
                    action: Some(attributes::Action {
                        value: units::SourceValue::new(format!(
                            "/games/{}/edit/terrain/{}",
                            game_id,
                            x.url_frag_string()
                        )),
                    }),
                    inputs: Vec::new(),
                    button: elements::Button::style_less(x.into_html().into_element()),
                    styles: attributes::StyleAttr::new(vec![&styles::Display::Inline]),
                }
                .into_element()
            })
            .chain(
                vec![elements::Form {
                    formmethod: attributes::Formmethod {
                        inner: units::FormmethodValue::Post,
                    },
                    action: Some(attributes::Action {
                        value: units::SourceValue::new(format!(
                            "/games/{}/edit/unset/terrain",
                            game_id,
                        )),
                    }),
                    inputs: Vec::new(),
                    button: elements::Button::style_less(htmldsl::text("delete".into())),
                    styles: attributes::StyleAttr::new(vec![&styles::Display::Inline]),
                }
                .into_element()]
                .into_iter(),
            )
            .collect(),
    )
}

fn build_character_adding_buttons<'a>(game_id: u32) -> elements::Div<'a> {
    elements::Div::style_less(
        models::Character::all_values()
            .into_iter()
            .map(|x| {
                elements::Form {
                    formmethod: attributes::Formmethod {
                        inner: units::FormmethodValue::Post,
                    },
                    action: Some(attributes::Action {
                        value: units::SourceValue::new(format!(
                            "/games/{}/edit/character/{}",
                            game_id,
                            x.url_frag_string()
                        )),
                    }),
                    inputs: Vec::new(),
                    button: elements::Button::style_less(x.into_html().into_element()),
                    styles: attributes::StyleAttr::new(vec![&styles::Display::Inline]),
                }
                .into_element()
            })
            .chain(
                vec![elements::Form {
                    formmethod: attributes::Formmethod {
                        inner: units::FormmethodValue::Post,
                    },
                    action: Some(attributes::Action {
                        value: units::SourceValue::new(format!(
                            "/games/{}/edit/unset/character",
                            game_id,
                        )),
                    }),
                    inputs: Vec::new(),
                    button: elements::Button::style_less(htmldsl::text("delete".into())),
                    styles: attributes::StyleAttr::new(vec![&styles::Display::Inline]),
                }
                .into_element()]
                .into_iter(),
            )
            .collect(),
    )
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
    elements::Body::style_less(vec![index_link(), games_link()])
}

pub fn games<'a>(games: Vec<models::Game>) -> elements::Body<'a> {
    elements::Body::style_less(vec![
        index_link(),
        games_link(),
        elements::Div::style_less(
            games
                .into_iter()
                .map(|game| {
                    elements::A::style_less(
                        attributes::Href {
                            value: units::SourceValue::new(format!("/games/{}", game.id)),
                        },
                        vec![htmldsl::text(format!("game: {}", game.id))],
                    )
                    .into_element()
                })
                .chain(
                    vec![elements::Form {
                        formmethod: attributes::Formmethod {
                            inner: units::FormmethodValue::Post,
                        },
                        action: Some(attributes::Action {
                            value: units::SourceValue::new("/games".into()),
                        }),
                        inputs: Vec::new(),
                        button: elements::Button::style_less(htmldsl::text("add map game".into())),
                        styles: attributes::StyleAttr::new(vec![&styles::Display::Inline]),
                    }
                    .into_element()]
                    .into_iter(),
                )
                .collect(),
        )
        .into_element(),
    ])
}

pub fn game<'a>(game: models::Game) -> elements::Body<'a> {
    elements::Body::style_less(vec![
        index_link(),
        games_link(),
        elements::H3::style_less(vec![
            elements::A::style_less(
                attributes::Href {
                    value: units::SourceValue::new(format!("/games/{}", game.id)),
                },
                vec![htmldsl::text("this map".into())],
            )
            .into_element(),
            elements::A::style_less(
                attributes::Href {
                    value: units::SourceValue::new(format!("/games/{}/edit", game.id)),
                },
                vec![htmldsl::text("edit".into())],
            )
            .into_element(),
        ])
        .into_element(),
        game.into_html(false),
        cursor_form_button(game.id, models::Direction::Left, false),
        cursor_form_button(game.id, models::Direction::Up, false),
        cursor_form_button(game.id, models::Direction::Down, false),
        cursor_form_button(game.id, models::Direction::Right, false),
    ])
}

pub fn edit_display<'a>(game: models::Game) -> elements::Body<'a> {
    elements::Body::style_less(vec![
        index_link(),
        games_link(),
        elements::H3::style_less(vec![
            elements::A::style_less(
                attributes::Href {
                    value: units::SourceValue::new(format!("/games/{}", game.id)),
                },
                vec![htmldsl::text("view map".into())],
            )
            .into_element(),
            elements::A::style_less(
                attributes::Href {
                    value: units::SourceValue::new(format!("/games/{}/edit", game.id)),
                },
                vec![htmldsl::text("editing".into())],
            )
            .into_element(),
        ])
        .into_element(),
        game.into_html(true),
        cursor_form_button(game.id, models::Direction::Left, true),
        cursor_form_button(game.id, models::Direction::Up, true),
        cursor_form_button(game.id, models::Direction::Down, true),
        cursor_form_button(game.id, models::Direction::Right, true),
    ])
}

fn cursor_form_button(game_id: u32, dir: models::Direction, edit: bool) -> htmldsl::Element {
    let (url_frag, symbol) = dir.form_strings();
    elements::Form {
        formmethod: attributes::Formmethod {
            inner: units::FormmethodValue::Post,
        },
        action: Some(attributes::Action {
            value: units::SourceValue::new(if edit {
                format!("/games/{}/edit/cursor/{}", game_id, url_frag)
            } else {
                format!("/games/{}/cursor/{}", game_id, url_frag)
            }),
        }),
        inputs: Vec::new(),
        button: elements::Button::style_less(htmldsl::text(symbol.into())),
        styles: attributes::StyleAttr::new(vec![&styles::Display::Inline]),
    }
    .into_element()
}

fn index_link() -> htmldsl::Element {
    elements::H1::style_less(vec![elements::A::style_less(
        attributes::Href {
            value: units::SourceValue::new("/".into()),
        },
        vec![htmldsl::text("home".into())],
    )
    .into_element()])
    .into_element()
}

fn games_link() -> htmldsl::Element {
    elements::H2::style_less(vec![elements::A::style_less(
        attributes::Href {
            value: units::SourceValue::new("/games".into()),
        },
        vec![htmldsl::text("games".into())],
    )
    .into_element()])
    .into_element()
}

pub fn not_found<'a>() -> elements::Body<'a> {
    elements::Body::style_less(vec![elements::H1::style_less(vec![htmldsl::text(
        "not found".into(),
    )])
    .into_element()])
}

pub fn internal_server_error<'a>() -> elements::Body<'a> {
    elements::Body::style_less(vec![elements::H1::style_less(vec![htmldsl::text(
        "internal server error".into(),
    )])
    .into_element()])
}

pub fn bad_request<'a, T: Into<String>>(message: T) -> elements::Body<'a> {
    elements::Body::style_less(vec![
        elements::H1::style_less(vec![htmldsl::text("bad request".into())]).into_element(),
        elements::P::style_less(vec![htmldsl::text(message.into())]).into_element(),
    ])
}
