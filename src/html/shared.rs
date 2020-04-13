use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::units;
use htmldsl::TagRenderableIntoElement;

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
