use anyhow::Result;
use smallvec::SmallVec;

use crate::Action;

use super::{KeymapContext, KeymapContextPredicate, Keystroke};

pub struct Binding {
    action: Box<dyn Action>,
    keystrokes: Option<SmallVec<[Keystroke; 2]>>,
    context_predicate: Option<KeymapContextPredicate>,
}

impl Binding {
    pub fn new<A: Action>(keystrokes: &str, action: A, context: Option<&str>) -> Self {
        Self::load(keystrokes, Box::new(action), context).unwrap()
    }

    pub fn load(keystrokes: &str, action: Box<dyn Action>, context: Option<&str>) -> Result<Self> {
        let context = if let Some(context) = context {
            Some(KeymapContextPredicate::parse(context)?)
        } else {
            None
        };

        let keystrokes = if keystrokes == "*" {
            None // Catch all context
        } else {
            Some(
                keystrokes
                    .split_whitespace()
                    .map(Keystroke::parse)
                    .collect::<Result<_>>()?,
            )
        };

        Ok(Self {
            keystrokes,
            action,
            context_predicate: context,
        })
    }

    pub fn match_context(&self, contexts: &[KeymapContext]) -> bool {
        self.context_predicate
            .as_ref()
            .map(|predicate| predicate.eval(contexts))
            .unwrap_or(true)
    }

    pub fn match_keys_and_context(
        &self,
        pending_keystrokes: &Vec<Keystroke>,
        contexts: &[KeymapContext],
    ) -> BindingMatchResult {
        if self
            .keystrokes
            .as_ref()
            .map(|keystrokes| keystrokes.starts_with(&pending_keystrokes))
            .unwrap_or(true)
            && self.match_context(contexts)
        {
            // If the binding is completed, push it onto the matches list
            if self
                .keystrokes
                .as_ref()
                .map(|keystrokes| keystrokes.len() == pending_keystrokes.len())
                .unwrap_or(true)
            {
                BindingMatchResult::Complete(self.action.boxed_clone())
            } else {
                BindingMatchResult::Partial
            }
        } else {
            BindingMatchResult::Fail
        }
    }

    pub fn keystrokes_for_action(
        &self,
        action: &dyn Action,
        contexts: &[KeymapContext],
    ) -> Option<SmallVec<[Keystroke; 2]>> {
        if self.action.eq(action) && self.match_context(contexts) {
            self.keystrokes.clone()
        } else {
            None
        }
    }

    pub fn keystrokes(&self) -> Option<&[Keystroke]> {
        self.keystrokes.as_deref()
    }

    pub fn action(&self) -> &dyn Action {
        self.action.as_ref()
    }
}

pub enum BindingMatchResult {
    Complete(Box<dyn Action>),
    Partial,
    Fail,
}
