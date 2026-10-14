//! Find the top-level Clojure form at a cursor position.
//!
//! Eval-at-point in CIDER (and `evalDefun` in other editors) means: find
//! the form whose parent is the document root and send *that* to the
//! REPL — not the innermost expression under the cursor. This module is
//! the one helper everything in `editor.rs` builds on top of.
//!
//! We deliberately don't require a Clojure-specific tree-sitter
//! dependency: the algorithm walks the layer's root node generically. The
//! grammar that ships with the Clojure extension uses a `source` root
//! whose direct children are top-level forms, which is the only structural
//! property we rely on.
//!
//! ## Namespace parsing
//!
//! [`parse_namespace`] does a one-pass scan over the layer's root looking
//! for the first `(ns <name> ...)` list. It's used on connect and on save
//! to keep the per-editor `:ns` cached for eval requests. Tracking
//! `(in-ns ...)` mid-buffer is out of scope for v1 (see the design doc).

use std::ops::Range;

use language::{BufferSnapshot, Node};

/// A top-level form located at a cursor position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopLevelForm {
    /// The form's text, suitable for sending as `:code` to nREPL.
    pub text: String,
    /// Byte range in the buffer that produced the form. Used to anchor
    /// the result block (and gutter highlight) at the form's end.
    pub range: Range<usize>,
}

/// Returns the top-level form containing `offset`, if any.
///
/// "Top-level" means: the highest ancestor of the node at `offset` whose
/// parent is the layer's root (the `source` node, in the Clojure
/// grammar). This matches `cider-eval-defun-at-point`.
///
/// Returns `None` in either of these cases:
///
/// - The buffer doesn't have a syntax layer at this offset (the language
///   hasn't loaded yet, or the offset is in a non-Clojure injection).
/// - The offset sits in pure whitespace / between top-level forms with
///   nothing immediately before or after to attach to.
///
/// Callers should treat `None` as "no-op" (with a status-bar toast in the
/// UI layer if appropriate); it isn't an error condition.
pub fn top_level_form_at_offset(snapshot: &BufferSnapshot, offset: usize) -> Option<TopLevelForm> {
    let had_syntax_layer = snapshot.syntax_layer_at(offset).is_some();
    let tree_sitter_form = (|| {
        let layer = snapshot.syntax_layer_at(offset)?;
        let root = layer.node();

        // Calva semantics: the top-level form containing the cursor wins,
        // including when the cursor sits right after its closing bracket
        // (`(defn f [] 1)|`). Children are in order, so at a boundary
        // between two adjacent forms the one ending at the cursor is found
        // first. Failing that, a form followed only by spaces up to the
        // cursor on the same line still counts as "the form before the
        // cursor".
        let mut walker = root.walk();
        let children: Vec<Node> = root
            .named_children(&mut walker)
            .filter(|child| child.kind() != "comment")
            .collect();
        let node = children
            .iter()
            .find(|child| child.start_byte() <= offset && offset <= child.end_byte())
            .or_else(|| {
                children.iter().rev().find(|child| {
                    child.end_byte() <= offset
                        && snapshot
                            .text_for_range(child.end_byte()..offset)
                            .all(|chunk| chunk.chars().all(|ch| ch == ' ' || ch == '\t'))
                })
            })?;

        let range = node.byte_range();
        let text = snapshot.text_for_range(range.clone()).collect::<String>();
        if text.trim().is_empty() {
            return None;
        }
        Some(TopLevelForm { text, range })
    })();

    if let Some(form) = tree_sitter_form {
        return Some(form);
    }

    log::debug!(
        "top_level_form_at_offset: falling back to text scan at offset {offset} (had_syntax_layer: {had_syntax_layer})"
    );
    let text = snapshot.text();
    let range = text_top_level_form(&text, offset)?;
    let form_text = snapshot.text_for_range(range.clone()).collect::<String>();
    if form_text.trim().is_empty() {
        return None;
    }
    Some(TopLevelForm {
        text: form_text,
        range,
    })
}

/// Parse the first `(ns <name> ...)` form in the buffer and return the
/// namespace symbol's text.
///
/// Returns `None` when no syntactically well-formed `(ns ...)` is present,
/// in which case callers should default to `"user"` (the same default the
/// nREPL server uses when no `:ns` is attached to an eval request).
///
/// We only look at *top-level* forms: an `(ns foo)` nested inside another
/// list isn't real Clojure and we'd rather miss it than pick up a stray
/// macro that happens to be spelled the same.
pub fn parse_namespace(snapshot: &BufferSnapshot) -> Option<String> {
    let layer = snapshot.syntax_layer_at(0)?;
    let root = layer.node();
    let mut walker = root.walk();
    for child in root.named_children(&mut walker) {
        if let Some(name) = ns_name_of_list(&child, snapshot) {
            return Some(name);
        }
    }
    None
}

/// Returns the full text of the first top-level `(ns ...)` form in the buffer.
///
/// Uses the buffer's syntax layer if available (matching [`parse_namespace`]).
/// When no syntax layer is present, falls back to textual scanning via
/// `parse_forms`, returning the first depth-0 form whose text starts with `(ns`
/// followed by whitespace.
pub fn ns_form_text(snapshot: &BufferSnapshot) -> Option<String> {
    if let Some(layer) = snapshot.syntax_layer_at(0) {
        let root = layer.node();
        let mut walker = root.walk();
        for child in root.named_children(&mut walker) {
            if ns_name_of_list(&child, snapshot).is_some() {
                let text = snapshot
                    .text_for_range(child.byte_range())
                    .collect::<String>();
                if !text.trim().is_empty() {
                    return Some(text);
                }
            }
        }
    }

    let text = snapshot.text();
    let forms = parse_forms(&text);
    let mut top_level_ranges: Vec<Range<usize>> = forms
        .into_iter()
        .filter(|form| form.depth == 0)
        .map(|form| form.range)
        .collect();
    top_level_ranges.sort_by_key(|range| range.start);

    for range in top_level_ranges {
        let form_text = &text[range];
        let trimmed = form_text.trim_start();
        if trimmed.starts_with("(ns ")
            || trimmed
                .strip_prefix("(ns")
                .is_some_and(|rest| rest.starts_with(char::is_whitespace))
        {
            return Some(form_text.to_string());
        }
    }

    None
}

/// If `node` is a `(ns <name> ...)` list, return `<name>` as a string.
///
/// We inspect the first two *named* descendants of the list (skipping
/// metadata). Tree-sitter's Clojure grammar wraps reader-macro forms in
/// extra nodes, so we recurse one level into anything that isn't a plain
/// symbol — that handles the `^{:doc ...} (ns ...)` case the Clojure
/// reader accepts.
fn ns_name_of_list(node: &Node, snapshot: &BufferSnapshot) -> Option<String> {
    // Cheap kind check: only `list_lit` (and the bare list) can be `(ns ...)`.
    if !matches!(node.kind(), "list_lit" | "_bare_list_lit") {
        return None;
    }

    let mut walker = node.walk();
    let mut named = node.named_children(&mut walker);

    let head = named.next()?;
    if !is_symbol_named(&head, "ns", snapshot) {
        return None;
    }

    let name_node = named.next()?;
    let raw = snapshot
        .text_for_range(name_node.byte_range())
        .collect::<String>();
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_string())
}

/// Returns true if `node` is a symbol literal whose textual content equals
/// `expected`. Avoids allocating when the byte length doesn't match.
fn is_symbol_named(node: &Node, expected: &str, snapshot: &BufferSnapshot) -> bool {
    if !matches!(node.kind(), "sym_lit" | "sym_name") {
        return false;
    }
    let range = node.byte_range();
    if range.len() != expected.len() {
        return false;
    }
    let actual = snapshot.text_for_range(range).collect::<String>();
    actual == expected
}

/// Returns the "current form" at `offset`, Calva style:
///
/// - If the cursor sits right after a form (`(foo)|` or `foo|`), that form.
/// - Otherwise the innermost form containing the cursor — a symbol or
///   literal under it, or the enclosing list / vector / map / set.
///
/// Returns `None` between top-level forms.
pub fn current_form_at_offset(snapshot: &BufferSnapshot, offset: usize) -> Option<TopLevelForm> {
    let had_syntax_layer = snapshot.syntax_layer_at(offset).is_some();
    let tree_sitter_form = (|| {
        let layer = snapshot.syntax_layer_at(offset)?;
        let root = layer.node();

        let form_ending_at_cursor = offset
            .checked_sub(1)
            .and_then(|before| root.descendant_for_byte_range(before, before))
            .and_then(|node| enclosing_form(node, root.id()))
            .filter(|node| node.end_byte() == offset);
        let node = match form_ending_at_cursor {
            Some(node) => node,
            None => enclosing_form(root.descendant_for_byte_range(offset, offset)?, root.id())?,
        };

        let range = node.byte_range();
        let text = snapshot.text_for_range(range.clone()).collect::<String>();
        if text.trim().is_empty() {
            return None;
        }
        Some(TopLevelForm { text, range })
    })();

    if let Some(form) = tree_sitter_form {
        return Some(form);
    }

    log::debug!(
        "current_form_at_offset: falling back to text scan at offset {offset} (had_syntax_layer: {had_syntax_layer})"
    );
    let text = snapshot.text();
    let range = text_current_form(&text, offset)?;
    let form_text = snapshot.text_for_range(range.clone()).collect::<String>();
    if form_text.trim().is_empty() {
        return None;
    }
    Some(TopLevelForm {
        text: form_text,
        range,
    })
}

/// Returns the symbol under (or immediately before) the cursor, e.g. for
/// documentation lookups.
pub fn symbol_at_offset(snapshot: &BufferSnapshot, offset: usize) -> Option<String> {
    let layer = snapshot.syntax_layer_at(offset)?;
    let root = layer.node();
    let candidates = [Some(offset), offset.checked_sub(1)];
    for candidate in candidates.into_iter().flatten() {
        let Some(mut node) = root.descendant_for_byte_range(candidate, candidate) else {
            continue;
        };
        loop {
            if node.kind() == "sym_lit" {
                let text = snapshot
                    .text_for_range(node.byte_range())
                    .collect::<String>();
                // Strip any metadata prefix: `^:private foo` -> `foo`.
                let symbol = text.split_whitespace().last().unwrap_or_default();
                if !symbol.is_empty() {
                    return Some(symbol.to_string());
                }
            }
            match node.parent() {
                Some(parent) if parent.id() != root.id() => node = parent,
                _ => break,
            }
        }
    }
    None
}

/// Returns the code from the start of the enclosing top-level form up to
/// `offset`, with any still-open brackets closed. This is Calva's
/// "evaluate from start of top-level form to cursor".
pub fn top_level_form_to_cursor(snapshot: &BufferSnapshot, offset: usize) -> Option<TopLevelForm> {
    let form = top_level_form_at_offset(snapshot, offset)?;
    let end = offset.clamp(form.range.start, form.range.end);
    let mut text = snapshot
        .text_for_range(form.range.start..end)
        .collect::<String>();
    if text.trim().is_empty() {
        return None;
    }
    text.push_str(&closing_brackets(&text));
    Some(TopLevelForm {
        text,
        range: form.range.start..end,
    })
}

/// Climbs from `node` to the nearest ancestor that is a Clojure form
/// (`*_lit` in the tree-sitter-clojure grammar). Stops before the root.
fn enclosing_form<'a>(mut node: Node<'a>, root_id: usize) -> Option<Node<'a>> {
    loop {
        if node.id() == root_id {
            return None;
        }
        if node.kind().ends_with("_lit") && node.kind() != "comment" {
            return Some(node);
        }
        node = node.parent()?;
    }
}

/// Returns the top-level form containing `offset`, using purely textual
/// Clojure scanning (no tree-sitter).
pub fn text_top_level_form(text: &str, offset: usize) -> Option<Range<usize>> {
    if offset > text.len() {
        return None;
    }
    let forms = parse_forms(text);
    let mut top_level_ranges: Vec<Range<usize>> = forms
        .into_iter()
        .filter(|form| form.depth == 0)
        .map(|form| form.range)
        .collect();
    top_level_ranges.sort_by_key(|range| range.start);

    for range in &top_level_ranges {
        if offset == range.end {
            return Some(range.clone());
        }
    }

    for range in &top_level_ranges {
        if range.contains(&offset) {
            return Some(range.clone());
        }
    }

    None
}

/// Returns the current form at `offset`, using purely textual Clojure scanning
/// (no tree-sitter).
pub fn text_current_form(text: &str, offset: usize) -> Option<Range<usize>> {
    if offset > text.len() {
        return None;
    }
    let forms = parse_forms(text);

    let form_ending_before_offset = forms
        .iter()
        .filter(|form| form.range.end == offset)
        .min_by_key(|form| form.range.len());
    if let Some(form) = form_ending_before_offset {
        return Some(form.range.clone());
    }

    let atom_under_cursor = forms
        .iter()
        .find(|form| !form.is_collection && form.range.contains(&offset));
    if let Some(atom) = atom_under_cursor {
        return Some(atom.range.clone());
    }

    let innermost_collection = forms
        .iter()
        .filter(|form| form.is_collection && form.range.start <= offset && offset <= form.range.end)
        .min_by_key(|form| form.range.len());
    if let Some(collection) = innermost_collection {
        return Some(collection.range.clone());
    }

    None
}

#[derive(Clone, Debug)]
struct FormNode {
    range: Range<usize>,
    is_collection: bool,
    depth: usize,
}

struct OpenDelimiter {
    start: usize,
    open_char: u8,
    depth: usize,
}

fn is_whitespace_byte(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\r' | b'\n' | b',')
}

fn is_delimiter_byte(byte: u8) -> bool {
    matches!(byte, b'(' | b')' | b'[' | b']' | b'{' | b'}' | b'"' | b';')
}

fn is_delimiter_or_whitespace_byte(byte: u8) -> bool {
    is_whitespace_byte(byte) || is_delimiter_byte(byte)
}

fn parse_forms(text: &str) -> Vec<FormNode> {
    let bytes = text.as_bytes();
    let mut open_stack: Vec<OpenDelimiter> = Vec::new();
    let mut forms: Vec<FormNode> = Vec::new();
    let mut index = 0;

    while index < bytes.len() {
        let byte = bytes[index];
        match byte {
            b' ' | b'\t' | b'\r' | b'\n' | b',' => {
                index += 1;
            }
            b';' => {
                index += 1;
                while index < bytes.len() && bytes[index] != b'\n' {
                    index += 1;
                }
            }
            b'"' => {
                let start_index = index;
                index += 1;
                while index < bytes.len() {
                    if bytes[index] == b'\\' {
                        index += 1;
                        if index < bytes.len() {
                            index += 1;
                        }
                    } else if bytes[index] == b'"' {
                        index += 1;
                        break;
                    } else {
                        index += 1;
                    }
                }
                forms.push(FormNode {
                    range: start_index..index,
                    is_collection: false,
                    depth: open_stack.len(),
                });
            }
            b'\\' => {
                let start_index = index;
                index += 1;
                if index < bytes.len() {
                    let next_byte = bytes[index];
                    if matches!(
                        next_byte,
                        b'(' | b')' | b'[' | b']' | b'{' | b'}' | b'"' | b';' | b'\\'
                    ) {
                        index += 1;
                    } else {
                        while index < bytes.len() && !is_delimiter_or_whitespace_byte(bytes[index])
                        {
                            index += 1;
                        }
                    }
                }
                forms.push(FormNode {
                    range: start_index..index,
                    is_collection: false,
                    depth: open_stack.len(),
                });
            }
            b')' | b']' | b'}' => {
                let close_char = byte;
                let close_end = index + 1;
                index += 1;
                let matching_open = match close_char {
                    b')' => b'(',
                    b']' => b'[',
                    b'}' => b'{',
                    _ => unreachable!(),
                };
                if let Some(position) = open_stack
                    .iter()
                    .rposition(|open| open.open_char == matching_open)
                {
                    while open_stack.len() > position + 1 {
                        if let Some(unclosed) = open_stack.pop() {
                            forms.push(FormNode {
                                range: unclosed.start..index.saturating_sub(1),
                                is_collection: true,
                                depth: unclosed.depth,
                            });
                        }
                    }
                    if let Some(open) = open_stack.pop() {
                        forms.push(FormNode {
                            range: open.start..close_end,
                            is_collection: true,
                            depth: open.depth,
                        });
                    }
                } else if let Some(open) = open_stack.pop() {
                    forms.push(FormNode {
                        range: open.start..close_end,
                        is_collection: true,
                        depth: open.depth,
                    });
                } else {
                    forms.push(FormNode {
                        range: index.saturating_sub(1)..index,
                        is_collection: false,
                        depth: 0,
                    });
                }
            }
            _ => {
                let start_index = index;
                if byte == b'#' && index + 1 < bytes.len() && bytes[index + 1] == b'"' {
                    index += 2;
                    while index < bytes.len() {
                        if bytes[index] == b'\\' {
                            index += 1;
                            if index < bytes.len() {
                                index += 1;
                            }
                        } else if bytes[index] == b'"' {
                            index += 1;
                            break;
                        } else {
                            index += 1;
                        }
                    }
                    forms.push(FormNode {
                        range: start_index..index,
                        is_collection: false,
                        depth: open_stack.len(),
                    });
                    continue;
                }

                let mut lookahead_index = index;
                while lookahead_index < bytes.len() {
                    match bytes[lookahead_index] {
                        b'#' => {
                            lookahead_index += 1;
                            if lookahead_index < bytes.len() && bytes[lookahead_index] == b'?' {
                                lookahead_index += 1;
                                if lookahead_index < bytes.len() && bytes[lookahead_index] == b'@' {
                                    lookahead_index += 1;
                                }
                            }
                        }
                        b'\'' | b'`' | b'@' | b'^' => {
                            lookahead_index += 1;
                        }
                        b'~' => {
                            lookahead_index += 1;
                            if lookahead_index < bytes.len() && bytes[lookahead_index] == b'@' {
                                lookahead_index += 1;
                            }
                        }
                        _ => break,
                    }
                }

                if lookahead_index < bytes.len()
                    && matches!(bytes[lookahead_index], b'(' | b'[' | b'{')
                {
                    let open_char = bytes[lookahead_index];
                    let depth = open_stack.len();
                    open_stack.push(OpenDelimiter {
                        start: start_index,
                        open_char,
                        depth,
                    });
                    index = lookahead_index + 1;
                } else {
                    while index < bytes.len() && !is_delimiter_or_whitespace_byte(bytes[index]) {
                        index += 1;
                    }
                    forms.push(FormNode {
                        range: start_index..index,
                        is_collection: false,
                        depth: open_stack.len(),
                    });
                }
            }
        }
    }

    while let Some(open) = open_stack.pop() {
        forms.push(FormNode {
            range: open.start..bytes.len(),
            is_collection: true,
            depth: open.depth,
        });
    }

    forms
}

/// Computes the closing brackets needed to balance `code`, skipping
/// brackets inside strings, character literals and line comments.
pub fn closing_brackets(code: &str) -> String {
    let mut stack = Vec::new();
    let mut chars = code.chars();
    let mut in_string = false;
    while let Some(ch) = chars.next() {
        if in_string {
            match ch {
                '\\' => {
                    chars.next();
                }
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '\\' => {
                chars.next();
            }
            ';' => {
                for next in chars.by_ref() {
                    if next == '\n' {
                        break;
                    }
                }
            }
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                stack.pop();
            }
            _ => {}
        }
    }
    let mut closing: String = stack.into_iter().rev().collect();
    if in_string {
        closing.insert(0, '"');
    }
    closing
}

#[cfg(test)]
mod tests {
    use super::{closing_brackets, text_current_form, text_top_level_form};
    use gpui::AppContext as _;

    #[test]
    fn closes_open_brackets() {
        assert_eq!(closing_brackets("(defn f [x] (let [y"), "]))");
        assert_eq!(closing_brackets("(str \"(\" {:a"), "})");
        assert_eq!(closing_brackets("(foo \\( ; (\n [1"), "])");
        assert_eq!(closing_brackets("(println \"abc"), "\")");
        assert_eq!(closing_brackets("(+ 1 2)"), "");
    }

    #[test]
    fn text_top_level_form_ns_and_defn() {
        let text = "(ns my.app)\n\n(defn f [x] (/ x 0))";
        let defn_start = text.find("(defn").expect("defn start not found");
        let defn_end = text.len();
        let expected_defn_range = defn_start..defn_end;

        // Cursor inside `(defn f [x] (/ x 0))`
        let cursor_inside = text.find("(/ x 0)").expect("inner form not found") + 2;
        assert_eq!(
            text_top_level_form(text, cursor_inside),
            Some(expected_defn_range.clone())
        );

        // Cursor right after the final `)`
        assert_eq!(
            text_top_level_form(text, defn_end),
            Some(expected_defn_range)
        );

        // Cursor in blank line between forms returns None
        let blank_line_offset = text.find("\n\n").expect("blank line not found") + 1;
        assert_eq!(text_top_level_form(text, blank_line_offset), None);
    }

    #[test]
    fn text_top_level_form_strings_chars_and_comments() {
        // Strings with parenthesis "("
        let text_string = "(foo \"(\" bar)";
        let inside_string_offset = text_string.find("bar").expect("bar not found");
        assert_eq!(
            text_top_level_form(text_string, inside_string_offset),
            Some(0..text_string.len())
        );

        // Character literal \(
        let text_char = "(foo \\( bar)";
        let inside_char_offset = text_char.find("bar").expect("bar not found");
        assert_eq!(
            text_top_level_form(text_char, inside_char_offset),
            Some(0..text_char.len())
        );

        // Comment with parenthesis
        let text_comment = "(foo\n ; (inner paren)\n bar)";
        let inside_comment_offset = text_comment.find("bar").expect("bar not found");
        assert_eq!(
            text_top_level_form(text_comment, inside_comment_offset),
            Some(0..text_comment.len())
        );
    }

    #[test]
    fn text_top_level_form_includes_attached_prefixes() {
        let text_fn = "#(+ % 1)";
        let fn_offset = text_fn.find('%').expect("% not found");
        assert_eq!(
            text_top_level_form(text_fn, fn_offset),
            Some(0..text_fn.len())
        );
        assert_eq!(
            &text_fn[text_top_level_form(text_fn, fn_offset).expect("form not found")],
            "#(+ % 1)"
        );

        let text_quote = "'(1 2)";
        let quote_offset = text_quote.find('2').expect("2 not found");
        assert_eq!(
            text_top_level_form(text_quote, quote_offset),
            Some(0..text_quote.len())
        );
        assert_eq!(
            &text_quote[text_top_level_form(text_quote, quote_offset).expect("form not found")],
            "'(1 2)"
        );
    }

    #[test]
    fn text_current_form_delimiters_and_symbols() {
        // text_current_form in `(+ 1 (* 2 3)|)` returns `(* 2 3)`
        let text_nested = "(+ 1 (* 2 3))";
        let inner_start = text_nested.find("(*").expect("(* not found");
        let inner_close_index = text_nested.find(')').expect(") not found");
        let offset_right_after_inner_close = inner_close_index + 1;
        assert_eq!(
            text_current_form(text_nested, offset_right_after_inner_close),
            Some(inner_start..offset_right_after_inner_close)
        );
        assert_eq!(
            &text_nested[text_current_form(text_nested, offset_right_after_inner_close)
                .expect("current form not found")],
            "(* 2 3)"
        );

        // Cursor over symbol `foo` inside `(bar foo)` returns `foo`
        let text_symbol = "(bar foo)";
        let foo_start = text_symbol.find("foo").expect("foo not found");
        let foo_end = foo_start + "foo".len();
        let cursor_on_foo = foo_start + 1;
        assert_eq!(
            text_current_form(text_symbol, cursor_on_foo),
            Some(foo_start..foo_end)
        );
        assert_eq!(
            &text_symbol
                [text_current_form(text_symbol, cursor_on_foo).expect("current form not found")],
            "foo"
        );
    }

    #[gpui::test]
    fn ns_form_text_fallback(cx: &mut gpui::App) {
        let buffer = cx.new(|cx| {
            language::Buffer::local(
                "(ns my.app\n  (:require [clojure.string :as str]))\n\n(defn f [x] x)",
                cx,
            )
        });
        let snapshot = buffer.read(cx).snapshot();
        assert_eq!(
            super::ns_form_text(&snapshot),
            Some("(ns my.app\n  (:require [clojure.string :as str]))".to_string())
        );

        let buffer_with_comment = cx.new(|cx| {
            language::Buffer::local(
                ";; File comment\n(ns my.app.with-comment)\n\n(defn f [x] x)",
                cx,
            )
        });
        let snapshot_with_comment = buffer_with_comment.read(cx).snapshot();
        assert_eq!(
            super::ns_form_text(&snapshot_with_comment),
            Some("(ns my.app.with-comment)".to_string())
        );

        let buffer_without_ns = cx.new(|cx| language::Buffer::local("(defn f [x] x)", cx));
        let snapshot_without_ns = buffer_without_ns.read(cx).snapshot();
        assert_eq!(super::ns_form_text(&snapshot_without_ns), None);
    }

    #[test]
    fn text_top_level_form_regression_defn_after_ns() {
        let text = "(ns my.app)\n\n(defn f [x]\n  (/ x 0))\n\n(println (f 2))\n";
        let defn_start = text.find("(defn").unwrap();
        let defn_end = text.find("0))").unwrap() + 3;
        for offset in [defn_start, defn_start + 3, text.find("[x]").unwrap(), defn_end - 1, defn_end] {
            assert_eq!(
                super::text_top_level_form(text, offset),
                Some(defn_start..defn_end),
                "offset {offset}"
            );
        }
    }

}
