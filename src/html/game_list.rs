use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::styles;
use htmldsl::units;
use htmldsl::TagRenderableIntoElement;

use crate::models;

use super::shared;

pub fn page<'a>(games: Vec<models::Game>) -> elements::Body<'a> {
    elements::Body::style_less(vec![
        shared::index_link(),
        shared::games_link(),
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
