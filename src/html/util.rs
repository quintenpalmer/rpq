use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::styles;
use htmldsl::units;
use htmldsl::TagRenderableIntoElement;

use super::models;

pub fn maybe_append<T>(mut vec: Vec<T>, maybe: Option<T>) -> Vec<T> {
    match maybe {
        Some(v) => {
            vec.push(v);
            vec
        }
        None => vec,
    }
}

pub fn cursor_form_button(game_id: u32, dir: models::Direction, edit: bool) -> htmldsl::Element {
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
