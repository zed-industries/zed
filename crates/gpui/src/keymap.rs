mod binding;
mod context;

pub use binding::*;
pub use context::*;

use crate::{Action, Keystroke, NoAction};
use collections::HashMap;
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
        for binding in bindings {
            let action_id = binding.action().as_any().type_id();
            self.binding_indices_by_action_id
                .entry(action_id)
                .or_default()
                .push(self.bindings.len());
            self.bindings.push(binding);
        }

        self.version.0 += 1;
    }

    /// Reset this keymap to its initial state.
    pub fn clear(&mut self) {
        self.bindings.clear();
        self.binding_indices_by_action_id.clear();
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

    /// bindings_for_input returns a list of bindings that match the given input,
    /// and a boolean indicating whether or not more bindings might match if
    /// the input was longer.
    ///
    /// Precedence is defined by the depth in the tree (matches on the Editor take
    /// precedence over matches on the Pane, then the Workspace, etc.). Bindings with
    /// no context are treated as the same as the deepest context.
    ///
    /// In the case of multiple bindings at the same depth, the ones defined later in the
    /// keymap take precedence (so user bindings take precedence over built-in bindings).
    ///
    /// If a user has disabled a binding with `"x": null` it will not be returned. Disabled
    /// bindings are evaluated with the same precedence rules so you can disable a rule in
    /// a given context only.
    ///
    /// In the case of multi-key bindings, the
    pub fn bindings_for_input(
        &self,
        input: &[Keystroke],
        context_path: &[KeyContext],
    ) -> (SmallVec<[KeyBinding; 1]>, bool) {
        let possibilities = self.bindings().rev().filter_map(|binding| {
            binding
                .match_keystrokes(input)
                .map(|pending| (binding, pending))
        });

        let mut bindings: SmallVec<[(KeyBinding, usize); 1]> = SmallVec::new();
        let mut is_pending = None;

        'outer: for (binding, pending) in possibilities {
            for depth in (0..=context_path.len()).rev() {
                if self.binding_enabled(binding, &context_path[0..depth]) {
                    if is_pending.is_none() {
                        is_pending = Some(pending);
                    }
                    if !pending {
                        bindings.push((binding.clone(), depth));
                        continue 'outer;
                    }
                }
            }
        }
        bindings.sort_by(|a, b| a.1.cmp(&b.1).reverse());
        let bindings = bindings
            .into_iter()
            .map_while(|(binding, _)| {
                if binding.action.as_any().type_id() == (NoAction {}).type_id() {
                    None
                } else {
                    Some(binding)
                }
            })
            .collect();

        (bindings, is_pending.unwrap_or_default())
    }

    /// Check if the given binding is enabled, given a certain key context.
    fn binding_enabled(&self, binding: &KeyBinding, context: &[KeyContext]) -> bool {
        // If binding has a context predicate, it must match the current context,
        if let Some(predicate) = &binding.context_predicate {
            if !predicate.eval(context) {
                return false;
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
        assert!(keymap
            .bindings_for_input(
                &[Keystroke::parse("ctrl-a").unwrap()],
                &[KeyContext::parse("barf").unwrap()],
            )
            .0
            .is_empty());
        assert!(!keymap
            .bindings_for_input(
                &[Keystroke::parse("ctrl-a").unwrap()],
                &[KeyContext::parse("editor").unwrap()],
            )
            .0
            .is_empty());

        // binding is disabled in a more specific context
        assert!(keymap
            .bindings_for_input(
                &[Keystroke::parse("ctrl-a").unwrap()],
                &[KeyContext::parse("editor mode=full").unwrap()],
            )
            .0
            .is_empty());

        // binding is globally disabled
        assert!(keymap
            .bindings_for_input(
                &[Keystroke::parse("ctrl-b").unwrap()],
                &[KeyContext::parse("barf").unwrap()],
            )
            .0
            .is_empty());
    }
}
