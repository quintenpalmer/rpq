use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::style_sheet;
use htmldsl::styles;
use htmldsl::units;
use htmldsl::TagRenderableIntoElement;

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

pub fn not_found<'a>() -> elements::Body<'a> {
    elements::Body::style_less(vec![elements::H1::style_less(vec![htmldsl::text(
        "not found",
    )])
    .into_element()])
}

pub fn internal_server_error<'a>() -> elements::Body<'a> {
    elements::Body::style_less(vec![elements::H1::style_less(vec![htmldsl::text(
        "internal server error",
    )])
    .into_element()])
}

pub fn bad_request<'a, T: Into<String>>(message: T) -> elements::Body<'a> {
    elements::Body::style_less(vec![
        elements::H1::style_less(vec![htmldsl::text("bad request")]).into_element(),
        elements::P::style_less(vec![htmldsl::text(message)]).into_element(),
    ])
}
