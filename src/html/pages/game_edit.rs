use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::units;
use htmldsl::TagRenderableIntoElement;

use crate::models;

use crate::html::shared;
use crate::html::util;

pub fn page<'a>(game: models::Game) -> elements::Body<'a> {
    elements::Body::style_less(vec![
        shared::index_link(),
        shared::games_link(),
        elements::H3::style_less(vec![
            elements::A::style_less(
                attributes::Href {
                    value: units::SourceValue::new(format!("/games/{}", game.id)),
                },
                vec![htmldsl::text("view map")],
            )
            .into_element(),
            elements::A::style_less(
                attributes::Href {
                    value: units::SourceValue::new(format!("/games/{}/edit", game.id)),
                },
                vec![htmldsl::text("editing")],
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
