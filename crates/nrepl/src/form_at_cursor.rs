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
    let layer = snapshot.syntax_layer_at(offset)?;
    let root = layer.node();

    // Drop straight down to the leaf containing `offset`. Using a cursor
    // (rather than `descendant_for_byte_range(offset, offset)`) lets us
    // recover gracefully when the cursor sits at the boundary between
    // two siblings: by default tree-sitter prefers the *left* sibling
    // (its end_byte == offset), and for eval-at-point we want the form
    // the user is *inside*, which is the right one.
    let mut cursor = root.walk();
    while cursor.goto_first_child_for_byte(offset).is_some() {
        if cursor.node().end_byte() == offset && !cursor.goto_next_sibling() {
            // No right sibling either; the offset is at the very end of
            // the parent's children. Re-anchor on the left sibling we just
            // landed on by walking back to the parent.
            cursor.goto_parent();
            break;
        }
    }

    // Walk back up to the highest ancestor whose parent *is* the layer
    // root. If we never reach such an ancestor we're already at the
    // root — i.e. the offset isn't inside any form at all.
    let mut node = cursor.node();
    if node.id() == root.id() {
        return None;
    }
    loop {
        let parent = node.parent()?;
        if parent.id() == root.id() {
            break;
        }
        node = parent;
    }

    let range = node.byte_range();
    let text = snapshot.text_for_range(range.clone()).collect::<String>();
    if text.trim().is_empty() {
        // Pathological case: a synthetic node with no actual textual
        // content. Don't send empty `:code` to the server.
        return None;
    }
    Some(TopLevelForm { text, range })
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
