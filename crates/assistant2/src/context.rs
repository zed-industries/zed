use gpui::SharedString;
use serde::{Deserialize, Serialize};
use util::post_inc;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct ContextId(pub(crate) usize);

impl ContextId {
    pub fn post_inc(&mut self) -> Self {
        Self(post_inc(&mut self.0))
    }
}

/// Some context attached to a message in a thread.
#[derive(Debug, Clone)]
pub struct Context {
    pub id: ContextId,
    pub name: SharedString,
    pub kind: ContextKind,
    pub text: SharedString,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextKind {
    File,
    FetchedUrl,
    Thread,
}
