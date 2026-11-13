//! In automatic table layout, a cell's length width is its column's preferred width, not a
//! minimum: the column grows to fit the cell's min-content width, but a table too narrow for
//! its columns' widths shrinks them, down to their contents (CSS Tables 3: the width raises
//! only the outer max-content width of a cell in a constrained column).
//!
//! Regression test: HTML mail with a `width="600"` cell in a `width="100%"` table, laid out
//! narrower than 600px, overflowed the page instead of fitting it.

use blitz_dom::DocumentConfig;
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;

/// The widths of the elements with ids `a`, `b`, ... in the laid-out `html`.
fn widths(html: &str, ids: &[&str]) -> Vec<f32> {
    let mut doc = HtmlDocument::from_html(
        html,
        DocumentConfig {
            viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
            html_parser_provider: Some(Arc::new(HtmlProvider) as _),
            ..Default::default()
        },
    );
    doc.resolve(0.0);
    ids.iter()
        .map(|id| {
            let node = doc.query_selector(&format!("#{id}")).unwrap().expect(id);
            doc.get_node(node).unwrap().final_layout().size.width
        })
        .collect()
}

/// A `width`×4 block, for cell contents of a known size.
fn block(width: u32) -> String {
    format!(r#"<div style="width:{width}px;height:4px"></div>"#)
}

#[test]
fn widths_wider_than_the_table_shrink_to_fit_it() {
    let w = widths(
        &format!(
            r#"<body style="margin:0"><table width="300" style="border-spacing:0"><tr>
            <td id="a" width="200" style="padding:0">{0}</td>
            <td id="b" width="200" style="padding:0">{0}</td>
            </tr></table></body>"#,
            block(10)
        ),
        &["a", "b"],
    );
    assert_eq!(w, [150.0, 150.0]);
}

#[test]
fn a_width_wider_than_the_table_leaves_the_others_their_content() {
    let w = widths(
        &format!(
            r#"<body style="margin:0"><table width="300" style="border-spacing:0"><tr>
            <td id="a" width="900" style="padding:0">{}</td>
            <td id="b" style="padding:0">{}</td>
            </tr></table></body>"#,
            block(50),
            block(20)
        ),
        &["a", "b"],
    );
    assert_eq!(w, [280.0, 20.0]);
}

#[test]
fn a_width_does_not_raise_the_tables_min_content_width() {
    let w = widths(
        r#"<body style="margin:0"><div style="width:200px"><table id="t" style="border-spacing:0"><tr>
        <td id="a" style="width:600px;padding:0"><div style="display:inline-block;width:50px;height:4px"></div>
        <div style="display:inline-block;width:50px;height:4px"></div></td>
        </tr></table></div></body>"#,
        &["t", "a"],
    );
    assert_eq!(w, [200.0, 200.0]);
}

#[test]
fn a_column_still_grows_to_its_contents() {
    let w = widths(
        &format!(
            r#"<body style="margin:0"><table width="300" style="border-spacing:0"><tr>
            <td id="a" style="width:24px;padding:0">{}</td>
            <td id="b" style="padding:0">{}</td>
            </tr></table></body>"#,
            block(47),
            block(20)
        ),
        &["a", "b"],
    );
    assert_eq!(w, [47.0, 253.0]);
}

#[test]
fn widths_narrower_than_the_table_share_out_the_rest_when_nothing_is_auto() {
    let w = widths(
        &format!(
            r#"<body style="margin:0"><table width="600" style="border-spacing:0"><tr>
            <td id="a" width="100" style="padding:0">{0}</td>
            <td id="b" width="200" style="padding:0">{0}</td>
            </tr></table></body>"#,
            block(10)
        ),
        &["a", "b"],
    );
    assert_eq!(w, [200.0, 400.0]);
}
