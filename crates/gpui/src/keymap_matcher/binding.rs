use anyhow::Result;
use smallvec::SmallVec;

use crate::Action;

use super::{KeymapContext, KeymapContextPredicate, Keystroke};

pub struct Binding {
    action: Box<dyn Action>,
    keystrokes: SmallVec<[Keystroke; 2]>,
    context_predicate: Option<KeymapContextPredicate>,
}

impl std::fmt::Debug for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Binding {{ keystrokes: {:?}, action: {}::{}, context_predicate: {:?} }}",
            self.keystrokes,
            self.action.namespace(),
            self.action.name(),
            self.context_predicate
        )
    }
}

impl Clone for Binding {
    fn clone(&self) -> Self {
        Self {
            action: self.action.boxed_clone(),
            keystrokes: self.keystrokes.clone(),
            context_predicate: self.context_predicate.clone(),
        }
    }
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
        if self.keystrokes.as_ref().starts_with(&pending_keystrokes) && self.match_context(contexts)
        {
            // If the binding is completed, push it onto the matches list
            if self.keystrokes.as_ref().len() == pending_keystrokes.len() {
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

pub enum BindingMatchResult {
    Complete(Box<dyn Action>),
    Partial,
    Fail,
}
