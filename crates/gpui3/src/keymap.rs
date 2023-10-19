use crate::{AnyBox, Keystroke};

#[derive(Default)]
pub struct KeyMatcher;

impl KeyMatcher {
    pub fn push_keystroke(&mut self, keystroke: Keystroke) -> KeyMatch {
        todo!()
    }
}

pub enum KeyMatch {
    None,
    Pending,
    Some(AnyBox),
}
