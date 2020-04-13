use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::units;
use htmldsl::TagRenderableIntoElement;

use crate::models;

use crate::html::shared;

pub fn page<'a>(map: models::Map) -> elements::Body<'a> {
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
