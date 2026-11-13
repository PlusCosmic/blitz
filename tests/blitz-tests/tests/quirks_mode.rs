//! The document's mode (no-quirks, limited-quirks, quirks), as the HTML parser chooses it from
//! the doctype, and what it changes: how Stylo parses and matches CSS, the line height
//! calculation quirk (quirks and limited-quirks mode), and the quirks-mode table rules.
//!
//! Text is measured with the system fonts; the assertions compare heights that differ by much
//! more than any font's metrics would.

use blitz_dom::{DocumentConfig, FontContext, QuirksMode};
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;

const STANDARDS: &str = "<!DOCTYPE html>";
/// HTML 4.01 Transitional with a system identifier: limited-quirks mode.
const LIMITED: &str = r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">"#;
/// No doctype: quirks mode.
const QUIRKS: &str = "";

fn layout_doc(html: &str) -> HtmlDocument {
    let mut doc = HtmlDocument::from_html(
        html,
        DocumentConfig {
            viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
            html_parser_provider: Some(Arc::new(HtmlProvider) as _),
            font_ctx: Some(FontContext::new()),
            ..Default::default()
        },
    );
    doc.resolve(0.0);
    doc
}

/// The border-box height of `#t` in `body` under `doctype`.
fn height(doctype: &str, body: &str) -> f32 {
    let doc = layout_doc(&format!(r#"{doctype}<body style="margin:0">{body}</body>"#));
    let node = doc.query_selector("#t").unwrap().expect("#t");
    doc.get_node(node).unwrap().final_layout().size.height
}

#[test]
fn the_doctype_sets_the_mode() {
    for (doctype, mode) in [
        (STANDARDS, QuirksMode::NoQuirks),
        (LIMITED, QuirksMode::LimitedQuirks),
        (QUIRKS, QuirksMode::Quirks),
        (
            r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN">"#,
            QuirksMode::Quirks,
        ),
    ] {
        let doc = layout_doc(&format!("{doctype}<p>x</p>"));
        assert_eq!(doc.quirks_mode(), mode, "{doctype}");
    }
}

#[test]
fn style_attributes_take_unitless_lengths_in_quirks_mode() {
    let div = r#"<div id="t" style="height:30"></div>"#;
    assert_eq!(height(QUIRKS, div), 30.0);
    assert_eq!(height(LIMITED, div), 0.0);
    assert_eq!(height(STANDARDS, div), 0.0);
}

#[test]
fn stylesheets_take_unitless_lengths_and_any_case_classes_in_quirks_mode() {
    let body = r#"<style>.Box { height: 30 }</style><div id="t" class="box"></div>"#;
    assert_eq!(height(QUIRKS, body), 30.0);
    assert_eq!(height(STANDARDS, body), 0.0);
}

#[test]
fn tables_do_not_inherit_font_size_in_quirks_mode() {
    let table = r#"<div style="font-size:40px;line-height:40px"><table id="t" style="border-spacing:0"><tr><td style="padding:0">a</td></tr></table></div>"#;
    assert!(height(QUIRKS, table) < 30.0);
    assert_eq!(height(LIMITED, table), 40.0);
    assert_eq!(height(STANDARDS, table), 40.0);
    // Author styles still apply.
    let styled = r#"<style>table { font-size: 40px; line-height: 40px }</style><table id="t" style="border-spacing:0"><tr><td style="padding:0">a</td></tr></table>"#;
    assert_eq!(height(QUIRKS, styled), 40.0);
}
