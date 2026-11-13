//! In the fixed table layout algorithm, a first-row cell spanning several columns divides its
//! width over them (CSS 2.1 §17.5.2.1), rather than giving all of it to its first column.

use blitz_dom::DocumentConfig;
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;

/// The widths of the cells `#a`, `#b` and `#c` on the second row of a fixed 600px wide
/// table whose first row is `first`.
fn second_row(first: &str) -> Vec<f32> {
    let mut doc = HtmlDocument::from_html(
        &format!(
            r#"<body style="margin:0"><table style="table-layout:fixed;width:600px;border-spacing:0">
            <tr>{first}</tr>
            <tr><td id="a" style="padding:0"></td><td id="b" style="padding:0"></td><td id="c" style="padding:0"></td></tr>
            </table></body>"#
        ),
        DocumentConfig {
            viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
            html_parser_provider: Some(Arc::new(HtmlProvider) as _),
            ..Default::default()
        },
    );
    doc.resolve(0.0);
    ["a", "b", "c"]
        .iter()
        .map(|id| {
            let node = doc.query_selector(&format!("#{id}")).unwrap().expect(id);
            doc.get_node(node).unwrap().final_layout().size.width
        })
        .collect()
}

#[test]
fn a_spanning_length_width_is_divided_over_its_columns() {
    let w = second_row(
        r#"<td colspan="2" width="200" style="padding:0"></td><td style="padding:0"></td>"#,
    );
    assert_eq!(w, [100.0, 100.0, 400.0]);
}

#[test]
fn a_spanning_percentage_width_is_divided_over_its_columns() {
    let w = second_row(
        r#"<td colspan="2" style="width:50%;padding:0"></td><td style="padding:0"></td>"#,
    );
    assert_eq!(w, [150.0, 150.0, 300.0]);
}
