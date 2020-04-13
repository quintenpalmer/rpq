use std::iter::Iterator;

use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::styles;
use htmldsl::units;
use htmldsl::TagRenderableIntoElement;

use super::models;

pub mod common;
pub mod index;
pub mod pages;
mod shared;
mod util;

pub fn maps<'a>(maps: Vec<models::Map>) -> elements::Body<'a> {
    elements::Body::style_less(vec![
        shared::index_link(),
        shared::maps_link(),
        elements::Div::style_less(
            maps.into_iter()
                .map(|map| {
                    elements::A::style_less(
                        attributes::Href {
                            value: units::SourceValue::new(format!("/maps/{}", map.id)),
                        },
                        vec![htmldsl::text(format!("map: {}", map.id))],
                    )
                    .into_element()
                })
                .chain(
                    vec![elements::Form {
                        formmethod: attributes::Formmethod {
                            inner: units::FormmethodValue::Post,
                        },
                        action: Some(attributes::Action {
                            value: units::SourceValue::new("/maps".into()),
                        }),
                        inputs: Vec::new(),
                        button: elements::Button::style_less(htmldsl::text("add map".into())),
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

pub fn map<'a>(map: models::Map) -> elements::Body<'a> {
    elements::Body::style_less(vec![
        shared::index_link(),
        shared::maps_link(),
        elements::H3::style_less(vec![elements::A::style_less(
            attributes::Href {
                value: units::SourceValue::new(format!("/maps/{}", map.id)),
            },
            vec![htmldsl::text("this map".into())],
        )
        .into_element()])
        .into_element(),
        map.into_html(Vec::new().into_iter(), None),
    ])
}

pub fn game<'a>(game: models::Game) -> elements::Body<'a> {
    elements::Body::style_less(vec![
        shared::index_link(),
        shared::games_link(),
        elements::H3::style_less(vec![
            elements::A::style_less(
                attributes::Href {
                    value: units::SourceValue::new(format!("/games/{}", game.id)),
                },
                vec![htmldsl::text("this game".into())],
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
        util::cursor_form_button(game.id, models::Direction::Left, false),
        util::cursor_form_button(game.id, models::Direction::Up, false),
        util::cursor_form_button(game.id, models::Direction::Down, false),
        util::cursor_form_button(game.id, models::Direction::Right, false),
    ])
}

pub fn edit_game<'a>(game: models::Game) -> elements::Body<'a> {
    elements::Body::style_less(vec![
        shared::index_link(),
        shared::games_link(),
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
        util::cursor_form_button(game.id, models::Direction::Left, true),
        util::cursor_form_button(game.id, models::Direction::Up, true),
        util::cursor_form_button(game.id, models::Direction::Down, true),
        util::cursor_form_button(game.id, models::Direction::Right, true),
    ])
}
