use crate::{Action, KeyBindingContextPredicate, KeyContext, KeyMatch, Keystroke};
use anyhow::Result;
use smallvec::SmallVec;

pub struct KeyBinding {
    pub(crate) action: Box<dyn Action>,
    pub(crate) keystrokes: SmallVec<[Keystroke; 2]>,
    pub(crate) context_predicate: Option<KeyBindingContextPredicate>,
}

impl Clone for KeyBinding {
    fn clone(&self) -> Self {
        KeyBinding {
            action: self.action.boxed_clone(),
            keystrokes: self.keystrokes.clone(),
            context_predicate: self.context_predicate.clone(),
        }
    }
}

impl KeyBinding {
    pub fn new<A: Action>(keystrokes: &str, action: A, context_predicate: Option<&str>) -> Self {
        Self::load(keystrokes, Box::new(action), context_predicate).unwrap()
    }

    pub fn load(keystrokes: &str, action: Box<dyn Action>, context: Option<&str>) -> Result<Self> {
        let context = if let Some(context) = context {
            Some(KeyBindingContextPredicate::parse(context)?)
        } else {
            None
        };

        let keystrokes = keystrokes
            .split_whitespace()
            .map(Keystroke::parse)
            .collect::<Result<_>>()?;

        Ok(Self {
            keystrokes,
            action,
            context_predicate: context,
        })
    }

    pub fn matches_context(&self, contexts: &[KeyContext]) -> bool {
        self.context_predicate
            .as_ref()
            .map(|predicate| predicate.eval(contexts))
            .unwrap_or(true)
    }

    pub fn match_keystrokes(
        &self,
        pending_keystrokes: &[Keystroke],
        contexts: &[KeyContext],
    ) -> KeyMatch {
        if self.keystrokes.as_ref().starts_with(&pending_keystrokes)
            && self.matches_context(contexts)
        {
            // If the binding is completed, push it onto the matches list
            if self.keystrokes.as_ref().len() == pending_keystrokes.len() {
                KeyMatch::Some(self.action.boxed_clone())
            } else {
                KeyMatch::Pending
            }
        } else {
            KeyMatch::None
        }
    }

    pub fn keystrokes_for_action(
        &self,
        action: &dyn Action,
        contexts: &[KeyContext],
    ) -> Option<SmallVec<[Keystroke; 2]>> {
        if self.action.partial_eq(action) && self.matches_context(contexts) {
            Some(self.keystrokes.clone())
        } else {
            None
        }
    }

    pub fn keystrokes(&self) -> &[Keystroke] {
        self.keystrokes.as_slice()
    }

    pub fn action(&self) -> &dyn Action {
        self.action.as_ref()
    }
}
