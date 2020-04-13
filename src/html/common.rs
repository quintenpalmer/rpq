use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::units;
use htmldsl::TagRenderableIntoElement;

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

pub fn index_link() -> htmldsl::Element {
    elements::H1::style_less(vec![elements::A::style_less(
        attributes::Href {
            value: units::SourceValue::new("/".into()),
        },
        vec![htmldsl::text("home".into())],
    )
    .into_element()])
    .into_element()
}

pub fn maps_link() -> htmldsl::Element {
    elements::H2::style_less(vec![elements::A::style_less(
        attributes::Href {
            value: units::SourceValue::new("/maps".into()),
        },
        vec![htmldsl::text("maps".into())],
    )
    .into_element()])
    .into_element()
}

pub fn games_link() -> htmldsl::Element {
    elements::H2::style_less(vec![elements::A::style_less(
        attributes::Href {
            value: units::SourceValue::new("/games".into()),
        },
        vec![htmldsl::text("games".into())],
    )
    .into_element()])
    .into_element()
}
