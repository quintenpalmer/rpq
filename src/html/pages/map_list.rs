use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::styles;
use htmldsl::units;
use htmldsl::TagRenderableIntoElement;

use crate::models;

use crate::html::shared;

pub fn page<'a>(maps: Vec<models::Map>) -> elements::Body<'a> {
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
                        button: elements::Button::style_less(htmldsl::text("add map")),
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
