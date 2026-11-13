//! `min-height` and `max-height` do not apply to table cells, as in browsers (CSS 2.1 §17.5.3
//! leaves their effect undefined): a cell's `height` is its minimum, and it grows to its content.
//!
//! Regression test: receipt mail spaces its rows with `<td height="32" style="max-height:1px">`
//! cells, which collapsed to 1px.

use blitz_dom::DocumentConfig;
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;

/// The height of the cell `#c` in a one-cell table, with the given attributes and content.
fn cell_height(attrs: &str, content: &str) -> f32 {
    let mut doc = HtmlDocument::from_html(
        &format!(
            r#"<body style="margin:0"><table style="border-spacing:0"><tr>
            <td id="c" {attrs}>{content}</td></tr></table></body>"#
        ),
        DocumentConfig {
            viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
            html_parser_provider: Some(Arc::new(HtmlProvider) as _),
            ..Default::default()
        },
    );
    doc.resolve(0.0);
    let node = doc.query_selector("#c").unwrap().expect("#c");
    doc.get_node(node).unwrap().final_layout().size.height
}

const BLOCK: &str = r#"<div style="width:10px;height:50px"></div>"#;

#[test]
fn max_height_does_not_shrink_a_cell_below_its_height() {
    assert_eq!(
        cell_height(r#"height="32" style="padding:0;max-height:1px""#, ""),
        32.0
    );
}

#[test]
fn max_height_does_not_shrink_a_cell_below_its_content() {
    assert_eq!(
        cell_height(r#"style="padding:0;max-height:10px""#, BLOCK),
        50.0
    );
}

#[test]
fn min_height_does_not_grow_a_cell() {
    assert_eq!(
        cell_height(r#"style="padding:0;min-height:80px""#, BLOCK),
        50.0
    );
}
