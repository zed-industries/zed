//! Canonical identities for syntax capture names.
//!
//! A [`SyntaxTokenId`] names a syntactic role — `keyword`, `string`,
//! `punctuation.bracket` — independently of any theme. Grammars tag spans with
//! these ids, and themes are asked to style them at paint time. Because the id
//! denotes a role rather than a position in some theme's style list, it stays
//! valid when the active theme changes and means the same thing in every window.

use std::{
    num::NonZeroU32,
    sync::{Arc, LazyLock},
};

use collections::HashMap;
use parking_lot::RwLock;

/// A canonical, process-wide identity for a syntax capture name.
///
/// Ids are interned on first use and remain valid for the life of the process,
/// so they may be stored in long-lived structures and sent between threads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SyntaxTokenId(NonZeroU32);

impl SyntaxTokenId {
    /// Returns a zero-based index suitable for indexing a per-token side table.
    ///
    /// Indices are dense and monotonically assigned, so a table sized by
    /// [`interned_count`] covers every id issued so far.
    #[inline]
    pub fn index(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Default)]
struct Interner {
    ids_by_name: HashMap<Arc<str>, SyntaxTokenId>,
    names_by_id: Vec<Arc<str>>,
}

static INTERNER: RwLock<Option<Interner>> = RwLock::new(None);

/// Returns the canonical id for `name`, assigning one if this is its first use.
pub fn intern(name: &str) -> SyntaxTokenId {
    if let Some(interner) = INTERNER.read().as_ref()
        && let Some(id) = interner.ids_by_name.get(name)
    {
        return *id;
    }

    let mut guard = INTERNER.write();
    let interner = guard.get_or_insert_with(Interner::default);
    // Another writer may have interned this name while the read lock was released.
    if let Some(id) = interner.ids_by_name.get(name) {
        return *id;
    }

    let name: Arc<str> = Arc::from(name);
    interner.names_by_id.push(name.clone());
    // Ids are one-based so that `Option<SyntaxTokenId>` is niche-packed into four
    // bytes; chunk iteration carries one per span. Saturating rather than
    // panicking is unreachable in practice: names come from grammar queries and
    // theme files, and exhausting `u32` would take billions of distinct names.
    let id = SyntaxTokenId(
        u32::try_from(interner.names_by_id.len())
            .ok()
            .and_then(NonZeroU32::new)
            .unwrap_or(NonZeroU32::MIN),
    );
    interner.ids_by_name.insert(name, id);
    id
}

/// Returns the capture name `id` was interned from.
pub fn name_for(id: SyntaxTokenId) -> Option<Arc<str>> {
    INTERNER
        .read()
        .as_ref()?
        .names_by_id
        .get(id.index())
        .cloned()
}

/// Identity for the caret position in a completion's snippet expansion.
///
/// Reserved names begin with `$`, which tree-sitter capture names and theme
/// syntax keys never do, so these cannot collide with a real capture.
pub fn tabstop_insert() -> SyntaxTokenId {
    static ID: LazyLock<SyntaxTokenId> = LazyLock::new(|| intern("$tabstop.insert"));
    *ID
}

/// Identity for a placeholder region in a completion's snippet expansion.
pub fn tabstop_replace() -> SyntaxTokenId {
    static ID: LazyLock<SyntaxTokenId> = LazyLock::new(|| intern("$tabstop.replace"));
    *ID
}

/// Returns how many distinct names have been interned so far.
///
/// Callers sizing a per-token table should treat this as a lower bound: a
/// grammar loaded later can intern additional names.
pub fn interned_count() -> usize {
    INTERNER
        .read()
        .as_ref()
        .map_or(0, |interner| interner.names_by_id.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interning_is_stable_and_round_trips() {
        let keyword = intern("keyword");
        let string = intern("string");

        assert_eq!(intern("keyword"), keyword);
        assert_ne!(keyword, string);

        assert_eq!(name_for(keyword).as_deref(), Some("keyword"));
        assert_eq!(name_for(string).as_deref(), Some("string"));
    }

    // Ids come from a process-wide interner shared with every other test in this
    // binary, so assertions here must not assume which ids were handed out first.
    #[test]
    fn indices_are_distinct_and_address_the_interned_table() {
        let bracket = intern("punctuation.bracket");
        let delimiter = intern("punctuation.delimiter");

        assert_ne!(bracket.index(), delimiter.index());
        assert!(bracket.index() < interned_count());
        assert!(delimiter.index() < interned_count());
        assert_eq!(
            name_for(delimiter).as_deref(),
            Some("punctuation.delimiter")
        );
    }
}
