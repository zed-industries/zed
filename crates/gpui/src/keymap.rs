mod binding;
mod context;
mod matcher;

pub use binding::*;
pub use context::*;
pub(crate) use matcher::*;

use crate::{Action, Keystroke, NoAction};
use collections::{HashMap, HashSet};
use smallvec::SmallVec;
use std::any::{Any, TypeId};

/// An opaque identifier of which version of the keymap is currently active.
/// The keymap's version is changed whenever bindings are added or removed.
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct KeymapVersion(usize);

/// A collection of key bindings for the user's application.
#[derive(Default)]
pub struct Keymap {
    bindings: Vec<KeyBinding>,
    binding_indices_by_action_id: HashMap<TypeId, SmallVec<[usize; 3]>>,
    disabled_keystrokes:
        HashMap<SmallVec<[Keystroke; 2]>, HashSet<Option<KeyBindingContextPredicate>>>,
    version: KeymapVersion,
}

impl Keymap {
    /// Create a new keymap with the given bindings.
    pub fn new(bindings: Vec<KeyBinding>) -> Self {
        let mut this = Self::default();
        this.add_bindings(bindings);
        this
    }

    /// Get the current version of the keymap.
    pub fn version(&self) -> KeymapVersion {
        self.version
    }

    /// Add more bindings to the keymap.
    pub fn add_bindings<T: IntoIterator<Item = KeyBinding>>(&mut self, bindings: T) {
        let no_action_id = (NoAction {}).type_id();

        for binding in bindings {
            let action_id = binding.action().as_any().type_id();
            if action_id == no_action_id {
                self.disabled_keystrokes
                    .entry(binding.keystrokes)
                    .or_default()
                    .insert(binding.context_predicate);
            } else {
                self.binding_indices_by_action_id
                    .entry(action_id)
                    .or_default()
                    .push(self.bindings.len());
                self.bindings.push(binding);
            }
        }

        self.version.0 += 1;
    }

    /// Reset this keymap to its initial state.
    pub fn clear(&mut self) {
        self.bindings.clear();
        self.binding_indices_by_action_id.clear();
        self.disabled_keystrokes.clear();
        self.version.0 += 1;
    }

    /// Iterate over all bindings, in the order they were added.
    pub fn bindings(&self) -> impl DoubleEndedIterator<Item = &KeyBinding> {
        self.bindings.iter()
    }

    /// Iterate over all bindings for the given action, in the order they were added.
    pub fn bindings_for_action<'a>(
        &'a self,
        action: &'a dyn Action,
    ) -> impl 'a + DoubleEndedIterator<Item = &'a KeyBinding> {
        let action_id = action.type_id();
        self.binding_indices_by_action_id
            .get(&action_id)
            .map_or(&[] as _, SmallVec::as_slice)
            .iter()
            .map(|ix| &self.bindings[*ix])
            .filter(move |binding| binding.action().partial_eq(action))
    }

    /// Check if the given binding is enabled, given a certain key context.
    pub fn binding_enabled(&self, binding: &KeyBinding, context: &[KeyContext]) -> bool {
        // If binding has a context predicate, it must match the current context,
        if let Some(predicate) = &binding.context_predicate {
            if !predicate.eval(context) {
                return false;
            }
        }

        if let Some(disabled_predicates) = self.disabled_keystrokes.get(&binding.keystrokes) {
            for disabled_predicate in disabled_predicates {
                match disabled_predicate {
                    // The binding must not be globally disabled.
                    None => return false,

                    // The binding must not be disabled in the current context.
                    Some(predicate) => {
                        if predicate.eval(context) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as gpui;
    use gpui::actions;

    actions!(
        keymap_test,
        [ActionAlpha, ActionBeta, ActionGamma, ActionDelta,]
    );

    #[test]
    fn test_keymap() {
        let bindings = [
            KeyBinding::new("ctrl-a", ActionAlpha {}, None),
            KeyBinding::new("ctrl-a", ActionBeta {}, Some("pane")),
            KeyBinding::new("ctrl-a", ActionGamma {}, Some("editor && mode==full")),
        ];

        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings.clone());

        // global bindings are enabled in all contexts
        assert!(keymap.binding_enabled(&bindings[0], &[]));
        assert!(keymap.binding_enabled(&bindings[0], &[KeyContext::parse("terminal").unwrap()]));

        // contextual bindings are enabled in contexts that match their predicate
        assert!(!keymap.binding_enabled(&bindings[1], &[KeyContext::parse("barf x=y").unwrap()]));
        assert!(keymap.binding_enabled(&bindings[1], &[KeyContext::parse("pane x=y").unwrap()]));

        assert!(!keymap.binding_enabled(&bindings[2], &[KeyContext::parse("editor").unwrap()]));
        assert!(keymap.binding_enabled(
            &bindings[2],
            &[KeyContext::parse("editor mode=full").unwrap()]
        ));
    }

    #[test]
    fn test_keymap_disabled() {
        let bindings = [
            KeyBinding::new("ctrl-a", ActionAlpha {}, Some("editor")),
            KeyBinding::new("ctrl-b", ActionAlpha {}, Some("editor")),
            KeyBinding::new("ctrl-a", NoAction {}, Some("editor && mode==full")),
            KeyBinding::new("ctrl-b", NoAction {}, None),
        ];

        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings.clone());

        // binding is only enabled in a specific context
        assert!(!keymap.binding_enabled(&bindings[0], &[KeyContext::parse("barf").unwrap()]));
        assert!(keymap.binding_enabled(&bindings[0], &[KeyContext::parse("editor").unwrap()]));

        // binding is disabled in a more specific context
        assert!(!keymap.binding_enabled(
            &bindings[0],
            &[KeyContext::parse("editor mode=full").unwrap()]
        ));

        // binding is globally disabled
        assert!(!keymap.binding_enabled(&bindings[1], &[KeyContext::parse("barf").unwrap()]));
    }
}
