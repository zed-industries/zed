use futures::Future;
use gpui::Task;
use smallvec::{smallvec, SmallVec};
use text::Anchor;

use crate::Editor;

struct Completions {
    trigger_characters: SmallVec<[char; 1]>,
    language: Option<String>,
    provider: Box<dyn Fn(&mut Editor, &Anchor, &str) -> Option<Task<String>>>,
}

impl Completions {
    fn new(f: impl Fn(&mut Editor, &Anchor, &str) -> Option<Task<String>> + 'static) -> Self {
        Self {
            trigger_characters: smallvec![],
            language: None,
            provider: Box::new(f),
        }
    }
}

impl Editor {
    /// Provide completions to the editor when the given character is typed
    ///
    fn provide_completions(config: Completions) {}
}
