# `cosmic-mail/0.3.0-beta.4`

An interim branch of Blitz for [Cosmic Mail](https://github.com/PlusCosmic/cosmic-mail), which
lays HTML mail out with `blitz-dom`/`blitz-html` 0.3.0-beta.4. It is the
[`v0.3.0-beta.4`](https://github.com/DioxusLabs/blitz/releases/tag/v0.3.0-beta.4) tag plus
upstream's open table column-width patches, ported to the tag, and a few small fixes on top.
Nothing else. Cosmic Mail depends on it through `[patch.crates-io]` by commit rev.

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
- **Ignore `min-height`/`max-height` on table cells**, as browsers do.
- **Fixed layout: a spanning first-row cell's width is divided over its columns** (CSS 2.1
  §17.5.2.1), rather than all going to its first column.

Each has a test in `tests/blitz-tests/tests/table_*.rs`.

## Moving it

- **New Blitz release without #1370**: rebase this branch onto the new tag (`git rebase --onto
  <new-tag> v0.3.0-beta.4`), drop any patch upstream has merged, push it as a new
  `cosmic-mail/<version>` branch, and point Cosmic Mail's `[patch.crates-io]` revs at it.
- **A patch merged upstream**: drop its commit when moving to the release that has it.
- **A release with #1370**: drop the branch and Cosmic Mail's `[patch.crates-io]` entries.
