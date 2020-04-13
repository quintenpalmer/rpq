use htmldsl::elements;

use crate::html::shared;

pub fn page<'a>() -> elements::Body<'a> {
    elements::Body::style_less(vec![
        shared::index_link(),
        shared::games_link(),
        shared::maps_link(),
    ])
}
