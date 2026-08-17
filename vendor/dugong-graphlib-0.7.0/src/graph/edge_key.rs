//! Edge key types.
//!
//! `graphlib` models edges with `v`, `w`, and an optional `name` (for multigraph support).

use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Hash)]
pub(in crate::graph) struct EdgeKeyView<'a> {
    pub(in crate::graph) v: &'a str,
    pub(in crate::graph) w: &'a str,
    pub(in crate::graph) name: Option<&'a str>,
}

impl<'a> hashbrown::Equivalent<EdgeKey> for EdgeKeyView<'a> {
    fn equivalent(&self, key: &EdgeKey) -> bool {
        key.v == self.v && key.w == self.w && key.name.as_deref() == self.name
    }
}

#[derive(Debug, Clone)]
pub struct EdgeKey {
    pub v: String,
    pub w: String,
    pub name: Option<String>,
}

impl EdgeKey {
    pub fn new(
        v: impl Into<String>,
        w: impl Into<String>,
        name: Option<impl Into<String>>,
    ) -> Self {
        Self {
            v: v.into(),
            w: w.into(),
            name: name.map(Into::into),
        }
    }
}

impl PartialEq for EdgeKey {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v && self.w == other.w && self.name == other.name
    }
}

impl Eq for EdgeKey {}

impl Hash for EdgeKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.v.hash(state);
        self.w.hash(state);
        self.name.hash(state);
    }
}
