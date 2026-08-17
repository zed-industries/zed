# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

The minimum supported rustc version is now `1.67.0` (was `1.40.0`).
This is because some dependencies now require this Rust version.

### Added

-   Add the option to display a configurable amount of lines in front of and after any label.
-   The `Severity` enum now implements full `Ord`. (#335)

### Changed

-   Broken lines are now rendered properly with multiline spans.

    We used to render the wrong lines in the gutter when there were multiline spans
    and there were breaks in the file.

    <details>
    <summary>Example</summary>

    Before:

    ```text
    error[0001]: oh noes, a cupcake has occurred!
       ┌─ test:4:1
       │
     4 │   ╭ Cupcake ipsum dolor. Sit amet marshmallow topping cheesecake
     5 │   │ muffin. Halvah croissant candy canes bonbon candy. Apple pie jelly
       │ ╭─│─────────'
       ·   │
    10 │ │ │ Muffin danish chocolate soufflé pastry icing bonbon oat cake.
    11 │ │ │ Powder cake jujubes oat cake. Lemon drops tootsie roll marshmallow
       │ │ ╰─────────────────────────────' blah blah
       · │ │
    19 │ │   soufflé marzipan. Chocolate bar oat cake jujubes lollipop pastry
    20 │ │   cupcake. Candy canes cupcake toffee gingerbread candy canes muffin
       │ ╰──────────' blah blah
    ```

    After:

    ```text
    error[0001]: oh noes, a cupcake has occurred!
       ┌─ test:4:1
       │
     4 │   ╭ Cupcake ipsum dolor. Sit amet marshmallow topping cheesecake
     5 │   │ muffin. Halvah croissant candy canes bonbon candy. Apple pie jelly
       │ ╭─│─────────'
       · │ │
    10 │ │ │ Muffin danish chocolate soufflé pastry icing bonbon oat cake.
    11 │ │ │ Powder cake jujubes oat cake. Lemon drops tootsie roll marshmallow
       │ │ ╰─────────────────────────────' blah blah
       · │
    19 │ │   soufflé marzipan. Chocolate bar oat cake jujubes lollipop pastry
    20 │ │   cupcake. Candy canes cupcake toffee gingerbread candy canes muffin
       │ ╰──────────' blah blah
    ```

    </details>

## [0.11.1] - 2021-01-18

### Added

-   Add `Chars::{box_drawing, ascii}` functions, the latter supporting a rustc-style of
    output that only uses ASCII characters (not above U+007F) for use cases that do not allow
    for box drawing characters, e.g. terminals that do not support them.

### Changed

-   `Diagnostic::with_labels` and `Diagnostic::with_notes` now append additional
    labels rather tan overwriting them, meaning that the documentation and behaviour match
    more closely. The behaviour will only differ if you call the same builder methods
    multiple times. If you call every builder method once only, nothing should change.
-   `config::Chars::snippet_start` is now a String instead of a single `char`.

## [0.11.0] - 2020-11-30

There is now a [code of conduct](https://github.com/brendanzab/codespan/blob/master/CODE_OF_CONDUCT.md)
and a [contributing guide](https://github.com/brendanzab/codespan/blob/master/CONTRIBUTING.md).

Some versions were skipped to sync up with the `codespan-lsp` crate. The release
process has been changed so this should not happen again.

### Added

-   If a label spans over multiple lines, not all lines are rendered.
    The number of lines rendered at beginning and end is configurable separately.
-   There is now a custom error type.
-   There now is a medium rendering mode that is like the short rendering mode
    but also shows notes from the diagnostic.
-   `PartialEq` and `Eq` implementations for the `diagnostic::{Diagnostic, Label, Severity}` types.

### Changed

-   All errors now use the error type `codespan_reporting::file::Error`.
    This type also replaces the custom error type for `codespan-lsp`.

### Fixed

-   Empty error codes are not rendered.
-   The locus ("location of the diagnostic") is now computed so it is always at the first
    primary label, or at the first secondary label if no primary labels are available.
-   All `unwrap`s outside of tests and examples have been removed.
-   Some internal improvements, including various code style improvements by using Clippy.
-   Improved documentation, also mentioning how the ordering of labels is handled.

## [0.9.5] - 2020-06-24

### Changed

-   Sections of source code that are marked with primary labels are now rendered
    using the primary highlight color.
-   Tab stops are now rendered properly.

    We used to just render `\t` characters in source snippets with the same
    number of spaces.

    <details>
    <summary>Example</summary>

    For example, when rendering with a tab width of `3` we
    would print:

    ```text
    warning: tab test
      ┌─ tab_columns:1:2
      │
    1 │    hello
      │    ^^^^^
    2 │ ∙   hello
      │     ^^^^^
    3 │ ∙∙   hello
      │      ^^^^^
    4 │ ∙∙∙   hello
      │       ^^^^^
    5 │ ∙∙∙∙   hello
      │        ^^^^^
    6 │ ∙∙∙∙∙   hello
      │         ^^^^^
    7 │ ∙∙∙∙∙∙   hello
      │          ^^^^^
    ```

    Now we properly take into account the column of the tab character:

    ```text
    warning: tab test
      ┌─ tab_columns:1:2
      │
    1 │    hello
      │    ^^^^^
    2 │ ∙  hello
      │    ^^^^^
    3 │ ∙∙ hello
      │    ^^^^^
    4 │ ∙∙∙   hello
      │       ^^^^^
    5 │ ∙∙∙∙  hello
      │       ^^^^^
    6 │ ∙∙∙∙∙ hello
      │       ^^^^^
    7 │ ∙∙∙∙∙∙   hello
      │          ^^^^^
    ```

    </details>

## [0.9.4] - 2020-05-18

### Changed

-   We have made the caret rendering easier to read when there are multiple
    labels on the same line. We also avoid printing trailing borders on the
    final source source snippet if no notes are present.

    <details>
    <summary>Example</summary>

    Instead of this:

    ```text
       ┌─ one_line.rs:3:5
       │
     3 │     v.push(v.pop().unwrap());
       │     - first borrow later used by call
       │       ---- first mutable borrow occurs here
       │            ^ second mutable borrow occurs here
       │
    ```

    …we now render the following:

    ```text
       ┌─ one_line.rs:3:5
       │
     3 │     v.push(v.pop().unwrap());
       │     - ---- ^ second mutable borrow occurs here
       │     │ │
       │     │ first mutable borrow occurs here
       │     first borrow later used by call
    ```

    </details>

### Fixed

-   Diagnostic rendering no longer panics if label ranges are between UTF-8
    character boundaries.

## [0.9.3] - 2020-04-29

### Changed

-   Some panics were fixed when invalid unicode boundaries are supplied.
-   Labels that marked the same span were originally rendered in reverse order.
    This was a mistake! We've now fixed this.

    <details>
    <summary>Example</summary>

    For example, this diagnostic:

    ```text
       ┌─ same_range:1:7
       │
     1 │ ::S { }
       │     - Expected '('
       │     ^ Unexpected '{'
       │
    ```

    …will now be rendered as:

    ```text
       ┌─ same_range:1:7
       │
     1 │ ::S { }
       │     ^ Unexpected '{'
       │     - Expected '('
       │
    ```

    </details>

-   We've reduced the prominence of the 'locus' on source snippets by
    simplifying the border and reducing the spacing around it. This is to help
    focus attention on the underlined source snippet and error messages, rather
    than the location, which should be a secondary focus.

    <details>
    <summary>Example</summary>

    For example we originally rendered this:

    ```text
    error: unknown builtin: `NATRAL`

       ┌── Data/Nat.fun:7:13 ───
       │
     7 │ {-# BUILTIN NATRAL Nat #-}
       │             ^^^^^^ unknown builtin
       │
       = there is a builtin with a similar name: `NATURAL`

    ```

    …and now we render this:

    ```text
    error: unknown builtin: `NATRAL`
      ┌─ Data/Nat.fun:7:13
      │
    7 │ {-# BUILTIN NATRAL Nat #-}
      │             ^^^^^^ unknown builtin
      │
      = there is a builtin with a similar name: `NATURAL`

    ```

    </details>

## [0.9.2] - 2020-03-29

### Changed

-   Render overlapping multiline marks on the same lines of source code.

    <details>
    <summary>Example</summary>

    For example:

    ```text
    error[E0308]: match arms have incompatible types

       ┌── codespan/src/file.rs:1:9 ───
       │
     1 │ ╭         match line_index.compare(self.last_line_index()) {
     2 │ │             Ordering::Less => Ok(self.line_starts()[line_index.to_usize()]),
     3 │ │             Ordering::Equal => Ok(self.source_span().end()),
     4 │ │             Ordering::Greater => LineIndexOutOfBoundsError {
     5 │ │                 given: line_index,
     6 │ │                 max: self.last_line_index(),
     7 │ │             },
     8 │ │         }
       │ ╰─────────' `match` arms have incompatible types
       ·
     2 │               Ordering::Less => Ok(self.line_starts()[line_index.to_usize()]),
       │                                 --------------------------------------------- this is found to be of type `Result<ByteIndex, LineIndexOutOfBoundsError>`
     3 │               Ordering::Equal => Ok(self.source_span().end()),
       │                                  ---------------------------- this is found to be of type `Result<ByteIndex, LineIndexOutOfBoundsError>`
     4 │               Ordering::Greater => LineIndexOutOfBoundsError {
       │ ╭──────────────────────────────────^
     5 │ │                 given: line_index,
     6 │ │                 max: self.last_line_index(),
     7 │ │             },
       │ ╰─────────────^ expected enum `Result`, found struct `LineIndexOutOfBoundsError`
       │
       = expected type `Result<ByteIndex, LineIndexOutOfBoundsError>`
            found type `LineIndexOutOfBoundsError`
    ```

    …is now rendered as:

    ```text
    error[E0308]: match arms have incompatible types

       ┌── codespan/src/file.rs:1:9 ───
       │
     1 │   ╭         match line_index.compare(self.last_line_index()) {
     2 │   │             Ordering::Less => Ok(self.line_starts()[line_index.to_usize()]),
       │   │                               --------------------------------------------- this is found to be of type `Result<ByteIndex, LineIndexOutOfBoundsError>`
     3 │   │             Ordering::Equal => Ok(self.source_span().end()),
       │   │                                ---------------------------- this is found to be of type `Result<ByteIndex, LineIndexOutOfBoundsError>`
     4 │   │             Ordering::Greater => LineIndexOutOfBoundsError {
       │ ╭─│──────────────────────────────────^
     5 │ │ │                 given: line_index,
     6 │ │ │                 max: self.last_line_index(),
     7 │ │ │             },
       │ ╰─│─────────────^ expected enum `Result`, found struct `LineIndexOutOfBoundsError`
     8 │   │         }
       │   ╰─────────' `match` arms have incompatible types
       │
       = expected type `Result<ByteIndex, LineIndexOutOfBoundsError>`
            found type `LineIndexOutOfBoundsError`
    ```

    </details>

## [0.9.1] - 2020-03-23

### Added

-   `codespan_reporting::diagnostic::Diagnostic` now implements `Debug`.

### Changed

-   Single-line labels are now rendered together, under the same source line.

    <details>
    <summary>Example</summary>

    For example:

    ```text
       ┌── one_line.rs:3:5 ───
       │
     3 │     v.push(v.pop().unwrap());
       │     - first borrow later used by call
       ·
     3 │     v.push(v.pop().unwrap());
       │       ---- first mutable borrow occurs here
       ·
     3 │     v.push(v.pop().unwrap());
       │            ^ second mutable borrow occurs here
       │
    ```

    …is now rendered as:

    ```text
       ┌── one_line.rs:3:5 ───
       │
     3 │     v.push(v.pop().unwrap());
       │     - first borrow later used by call
       │       ---- first mutable borrow occurs here
       │            ^ second mutable borrow occurs here
       │
    ```

    </details>

## [0.9.0] - 2020-03-11

### Added

-   The `codespan_reporting::files` module was added as a way to decouple
    `codespan_reporting` from `codespan`.
    -   `codespan_reporting::files::Files` allows users to implement custom file
        databases that work with `codespan_reporting`. This should make it
        easier to integrate with libraries like Salsa, and also makes it less
        invasive to use `codespan_reporting` on existing projects.
    -   `codespan_reporting::files::SimpleFile` is a simple implementation of
        `codespan_reporting::files::Files` where only a single file is needed.
    -   `codespan_reporting::files::SimpleFiles` is a simple implementation of
        `codespan_reporting::files::Files` where multiple files are needed.

### Changed

-   The `codespan_reporting::diagnostic` module has been greatly revamped,
    making the builder API format more nicely with rustfmt, and allowing for
    multiple primary labels.
-   The output of `codespan_reporting::term::emit` was improved,
    with the following changes:
    -   labels on consecutive lines no longer render breaks between them
    -   source lines are rendered when there is only one line between labels
    -   the inner gutter of code snippets is now aligned consistently
    -   the outer gutter of consecutive code snippets are now aligned consistently
-   `codespan_reporting::term::emit` now takes writers as a trait object (rather
    than using static dispatch) in order to reduce coda bloat and improve
    compile times.
-   The field names in `codespan_reporting::term::Chars` were tweaked for
    consistency.

### Removed

-   `codespan_reporting` no longer depends on `codespan`.
    Note that `codespan` can _still_ be used with `codespan_reporting`,
    as `codespan::Files` now implements `codespan_reporting::files::Files`.

## [0.8.0] - 2020-02-24
## [0.7.0] - 2020-01-06
## [0.6.0] - 2019-12-18
## [0.5.0] - 2019-10-02
## [0.4.1] - 2019-08-25
## [0.4.0] - 2019-08-22
## [0.3.0] - 2019-05-01
## [0.2.1] - 2019-02-26
## [0.2.0] - 2018-10-11

[Unreleased]: https://github.com/brendanzab/codespan/compare/v0.11.1...HEAD
[0.11.1]: https://github.com/brendanzab/codespan/compare/v0.11.0..v0.11.1
[0.11.0]: https://github.com/brendanzab/codespan/compare/v0.9.5...v0.11.0
[0.9.5]: https://github.com/brendanzab/codespan/compare/v0.9.4...v0.9.5
[0.9.4]: https://github.com/brendanzab/codespan/compare/v0.9.3...v0.9.4
[0.9.3]: https://github.com/brendanzab/codespan/compare/v0.9.2...v0.9.3
[0.9.2]: https://github.com/brendanzab/codespan/compare/v0.9.1...v0.9.2
[0.9.1]: https://github.com/brendanzab/codespan/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/brendanzab/codespan/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/brendanzab/codespan/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/brendanzab/codespan/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/brendanzab/codespan/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/brendanzab/codespan/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/brendanzab/codespan/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/brendanzab/codespan/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/brendanzab/codespan/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/brendanzab/codespan/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/brendanzab/codespan/releases/tag/v0.2.0
