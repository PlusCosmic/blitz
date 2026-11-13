# `cosmic-mail/0.3.0-beta.4`

An interim branch of Blitz for [Cosmic Mail](https://github.com/PlusCosmic/cosmic-mail), which
lays HTML mail out with `blitz-dom`/`blitz-html` 0.3.0-beta.4. It is the
[`v0.3.0-beta.4`](https://github.com/DioxusLabs/blitz/releases/tag/v0.3.0-beta.4) tag plus
upstream's open table column-width patches, ported to the tag, a few small table fixes on top,
and quirks-mode support. Nothing else. Cosmic Mail depends on it through `[patch.crates-io]` by
commit rev.

Blitz lays tables out as a CSS grid. Upstream is replacing that with a real table algorithm
([DioxusLabs/blitz#1370](https://github.com/DioxusLabs/blitz/pull/1370), on
[DioxusLabs/taffy#1380](https://github.com/DioxusLabs/taffy/pull/1380)). **This branch is dropped
once a Blitz release includes #1370.**

## Patches

Ported from upstream pull requests (cherry-picked with `-x`, authors kept):

| upstream | what |
|---|---|
| [#1387](https://github.com/DioxusLabs/blitz/pull/1387) | auto layout takes column widths from any row, not only the first |
| [#1345](https://github.com/DioxusLabs/blitz/pull/1345) | auto layout: a cell's length width no longer fixes its column; the column grows to its content |
| [#1482](https://github.com/DioxusLabs/blitz/pull/1482) | auto layout: percentage columns get their share of a table with a width |
| [#1486](https://github.com/DioxusLabs/blitz/pull/1486) | fixed layout: with no `auto` column, columns with a width share out the extra width |

Fixes on top (not upstream):

- **Fix-up for #1345**: a length width raises only the cell's max-content width, not its
  min-content width, so a table too narrow for its columns' widths shrinks them (CSS Tables 3)
  instead of overflowing.
- **Fix-up for #1482 with #1387**: when several rows give a column a percentage, the largest wins.
- **Fix-up for #1482**: with no `auto` column, the columns with a width share out the rest of the
  table (CSS 2.1 §17.5.2.2; #1486 does the same in the fixed layout): a lone `width="50%"` cell
  fills its table again, and 100px + 100px in a 600px table gives 300/300.
- **Ignore `min-height`/`max-height` on table cells**, as browsers do.
- **Fixed layout: a spanning first-row cell's width is divided over its columns** (CSS 2.1
  §17.5.2.1), rather than all going to its first column.

- **A cell's length `height` is its minimum**: the cell grows to its content, as in browsers
  (it overflowed into the rows below).

Each has a test in `tests/blitz-tests/tests/table_*.rs`.

## Quirks mode (not upstream)

Blitz recorded the mode the HTML parser chose from the doctype and used it nowhere. About half
of HTML mail is in quirks or limited-quirks mode (no doctype; HTML 4.01 / XHTML 1.0
Transitional), where browsers lay lines out differently. Written to be offered upstream: keyed
on the document's mode, nothing mail-specific.

- **The document keeps its mode** (`BaseDocument::quirks_mode`, set by the HTML sink on the
  doctype): Stylo parses stylesheets and `style` attributes and matches selectors in it
  (unitless lengths, hashless colours, case-insensitive classes), and in quirks mode the
  document adds `assets/quirks.css` (tables do not inherit font properties).
- **The line height calculation quirk** (quirks and limited-quirks): an inline box, the strut
  included, only counts towards the lines on which it has text of its own. Done in
  `layout/construct.rs` with Parley's span boxes (zero line-height on the span, its own text in
  a span with the real one). Not covered: a padded span counts on all its lines (Chrome: only
  those with its start or end edge); an outside list marker's strut.
- **The table cell height box sizing quirk** (quirks): a cell's `height` includes its padding
  and border.
- Also: a `<br>`'s newline sits on its parent's baseline (it copied a middle-aligned cell's
  `vertical-align` and made its line taller).

Tests: `tests/blitz-tests/tests/quirks_mode.rs`, `br_trailing_line.rs`.

## Moving it

- **New Blitz release without #1370**: rebase this branch onto the new tag (`git rebase --onto
  <new-tag> v0.3.0-beta.4`), drop any patch upstream has merged, push it as a new
  `cosmic-mail/<version>` branch, and point Cosmic Mail's `[patch.crates-io]` revs at it.
- **A patch merged upstream**: drop its commit when moving to the release that has it.
- **A release with #1370**: drop the table commits. The quirks-mode commits stay until upstream
  has an equivalent; until then the branch moves with each release.
