use crate::{Action, ActionContext, ActionContextPredicate, KeyMatch, Keystroke};
use anyhow::Result;
use smallvec::SmallVec;

pub struct KeyBinding {
    action: Box<dyn Action>,
    pub(super) keystrokes: SmallVec<[Keystroke; 2]>,
    pub(super) context_predicate: Option<ActionContextPredicate>,
}

impl KeyBinding {
    pub fn new<A: Action>(keystrokes: &str, action: A, context_predicate: Option<&str>) -> Self {
        Self::load(keystrokes, Box::new(action), context_predicate).unwrap()
    }

    pub fn load(keystrokes: &str, action: Box<dyn Action>, context: Option<&str>) -> Result<Self> {
        let context = if let Some(context) = context {
            Some(ActionContextPredicate::parse(context)?)
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

    pub fn matches_context(&self, contexts: &[ActionContext]) -> bool {
        self.context_predicate
            .as_ref()
            .map(|predicate| predicate.eval(contexts))
            .unwrap_or(true)
    }

    pub fn match_keystrokes(
        &self,
        pending_keystrokes: &[Keystroke],
        contexts: &[ActionContext],
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
        contexts: &[ActionContext],
    ) -> Option<SmallVec<[Keystroke; 2]>> {
        if self.action.eq(action) && self.matches_context(contexts) {
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
