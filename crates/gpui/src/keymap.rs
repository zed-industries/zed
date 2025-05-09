mod binding;
mod context;

pub use binding::*;
pub use context::*;

use crate::{Action, Keystroke, is_no_action};
use collections::HashMap;
use smallvec::SmallVec;
use std::any::TypeId;

/// An opaque identifier of which version of the keymap is currently active.
/// The keymap's version is changed whenever bindings are added or removed.
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct KeymapVersion(usize);

/// A collection of key bindings for the user's application.
#[derive(Default)]
pub struct Keymap {
    bindings: Vec<KeyBinding>,
    binding_indices_by_action_id: HashMap<TypeId, SmallVec<[usize; 3]>>,
    no_action_binding_indices: Vec<usize>,
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
            if is_no_action(&*binding.action) {
                self.no_action_binding_indices.push(self.bindings.len());
            } else {
                self.binding_indices_by_action_id
                    .entry(action_id)
                    .or_default()
                    .push(self.bindings.len());
            }
            self.bindings.push(binding);
        }

        self.version.0 += 1;
    }

    /// Reset this keymap to its initial state.
    pub fn clear(&mut self) {
        self.bindings.clear();
        self.binding_indices_by_action_id.clear();
        self.no_action_binding_indices.clear();
        self.version.0 += 1;
    }

    /// Iterate over all bindings, in the order they were added.
    pub fn bindings(&self) -> impl DoubleEndedIterator<Item = &KeyBinding> {
        self.bindings.iter()
    }

    /// Iterate over all bindings for the given action, in the order they were added. For display,
    /// the last binding should take precedence.
    pub fn bindings_for_action<'a>(
        &'a self,
        action: &'a dyn Action,
    ) -> impl 'a + DoubleEndedIterator<Item = &'a KeyBinding> {
        let action_id = action.type_id();
        let binding_indices = self
            .binding_indices_by_action_id
            .get(&action_id)
            .map_or(&[] as _, SmallVec::as_slice)
            .iter();

        binding_indices.filter_map(|ix| {
            let binding = &self.bindings[*ix];
            if !binding.action().partial_eq(action) {
                return None;
            }

            for null_ix in &self.no_action_binding_indices {
                if null_ix > ix {
                    let null_binding = &self.bindings[*null_ix];
                    if null_binding.keystrokes == binding.keystrokes {
                        let null_binding_matches =
                            match (&null_binding.context_predicate, &binding.context_predicate) {
                                (None, _) => true,
                                (Some(_), None) => false,
                                (Some(null_predicate), Some(predicate)) => {
                                    null_predicate.is_superset(predicate)
                                }
                            };
                        if null_binding_matches {
                            return None;
                        }
                    }
                }
            }

            Some(binding)
        })
    }

    /// Returns all bindings that might match the input without checking context. The bindings
    /// returned in precedence order (reverse of the order they were added to the keymap).
    pub fn all_bindings_for_input(&self, input: &[Keystroke]) -> Vec<KeyBinding> {
        self.bindings()
            .rev()
            .filter_map(|binding| {
                binding.match_keystrokes(input).filter(|pending| !pending)?;
                Some(binding.clone())
            })
            .collect()
    }

    /// Returns a list of bindings that match the given input, and a boolean indicating whether or
    /// not more bindings might match if the input was longer. Bindings are returned in precedence
    /// order.
    ///
    /// Precedence is defined by the depth in the tree (matches on the Editor take precedence over
    /// matches on the Pane, then the Workspace, etc.). Bindings with no context are treated as the
    /// same as the deepest context.
    ///
    /// In the case of multiple bindings at the same depth, the ones added to the keymap later take
    /// precedence. User bindings are added after built-in bindings so that they take precedence.
    ///
    /// If a user has disabled a binding with `"x": null` it will not be returned. Disabled bindings
    /// are evaluated with the same precedence rules so you can disable a rule in a given context
    /// only.
    pub fn bindings_for_input(
        &self,
        input: &[Keystroke],
        context_stack: &[KeyContext],
    ) -> (SmallVec<[KeyBinding; 1]>, bool) {
        let possibilities = self.bindings().rev().filter_map(|binding| {
            binding
                .match_keystrokes(input)
                .map(|pending| (binding, pending))
        });

        let mut bindings: SmallVec<[(KeyBinding, usize); 1]> = SmallVec::new();

        // (pending, is_no_action, depth)
        let mut pending_info_opt: Option<(bool, bool, usize)> = None;

        'outer: for (binding, pending) in possibilities {
            for depth in (0..=context_stack.len()).rev() {
                if self.binding_enabled(binding, &context_stack[0..depth]) {
                    let is_no_action = is_no_action(&*binding.action);
                    // We only want to consider a binding pending if it has an action
                    // This, however, means that if we have both a NoAction binding and a binding
                    // with an action at the same depth, we should still set is_pending to true.
                    if let Some(pending_info) = pending_info_opt.as_mut() {
                        let (already_pending, pending_is_no_action, pending_depth) = *pending_info;

                        // We only want to change the pending status if it's not already pending AND if
                        // the existing pending status was set by a NoAction binding. This avoids a NoAction
                        // binding erroneously setting the pending status to true when a binding with an action
                        // already set it to false
                        if pending
                            && !already_pending
                            && pending_is_no_action
                            && pending_depth == depth
                        {
                            pending_info.0 = !is_no_action;
                        }
                    } else {
                        pending_info_opt = Some((pending && !is_no_action, is_no_action, depth));
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
                if is_no_action(&*binding.action) {
                    None
                } else {
                    Some(binding)
                }
            })
            .collect();

        (bindings, pending_info_opt.unwrap_or_default().0)
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

    /// WARN: Assumes the bindings are in the order they were added to the keymap
    /// returns the last binding for the given bindings, which
    /// should be the user's binding in their keymap.json if they've set one,
    /// otherwise, the last declared binding for this action in the base keymaps
    /// (with Vim mode bindings being considered as declared later if Vim mode
    /// is enabled)
    ///
    /// If you are considering changing the behavior of this function
    /// (especially to fix a user reported issue) see issues #23621, #24931,
    /// and possibly others as evidence that it has swapped back and forth a
    /// couple times. The decision as of now is to pick a side and leave it
    /// as is, until we have a better way to decide which binding to display
    /// that is consistent and not confusing.
    pub fn binding_to_display_from_bindings(mut bindings: Vec<KeyBinding>) -> Option<KeyBinding> {
        bindings.pop()
    }

    /// Like `bindings_to_display_from_bindings` but takes a `DoubleEndedIterator` and returns a
    /// reference.
    pub fn binding_to_display_from_bindings_iterator<'a>(
        mut bindings: impl DoubleEndedIterator<Item = &'a KeyBinding>,
    ) -> Option<&'a KeyBinding> {
        bindings.next_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as gpui;
    use gpui::{NoAction, actions};

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
        assert!(
            keymap
                .bindings_for_input(
                    &[Keystroke::parse("ctrl-a").unwrap()],
                    &[KeyContext::parse("barf").unwrap()],
                )
                .0
                .is_empty()
        );
        assert!(
            !keymap
                .bindings_for_input(
                    &[Keystroke::parse("ctrl-a").unwrap()],
                    &[KeyContext::parse("editor").unwrap()],
                )
                .0
                .is_empty()
        );

        // binding is disabled in a more specific context
        assert!(
            keymap
                .bindings_for_input(
                    &[Keystroke::parse("ctrl-a").unwrap()],
                    &[KeyContext::parse("editor mode=full").unwrap()],
                )
                .0
                .is_empty()
        );

        // binding is globally disabled
        assert!(
            keymap
                .bindings_for_input(
                    &[Keystroke::parse("ctrl-b").unwrap()],
                    &[KeyContext::parse("barf").unwrap()],
                )
                .0
                .is_empty()
        );
    }

    #[test]
    /// Tests for https://github.com/zed-industries/zed/issues/30259
    fn test_multiple_keystroke_binding_disabled() {
        let bindings = [
            KeyBinding::new("space w w", ActionAlpha {}, Some("workspace")),
            KeyBinding::new("space w w", NoAction {}, Some("editor")),
        ];

        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings.clone());

        let space = || Keystroke::parse("space").unwrap();
        let w = || Keystroke::parse("w").unwrap();

        let space_w = [space(), w()];
        let space_w_w = [space(), w(), w()];

        let workspace_context = || [KeyContext::parse("workspace").unwrap()];

        let editor_workspace_context = || {
            [
                KeyContext::parse("workspace").unwrap(),
                KeyContext::parse("editor").unwrap(),
            ]
        };

        // Ensure `space` results in pending input on the workspace, but not editor
        let space_workspace = keymap.bindings_for_input(&[space()], &workspace_context());
        assert!(space_workspace.0.is_empty());
        assert_eq!(space_workspace.1, true);

        let space_editor = keymap.bindings_for_input(&[space()], &editor_workspace_context());
        assert!(space_editor.0.is_empty());
        assert_eq!(space_editor.1, false);

        // Ensure `space w` results in pending input on the workspace, but not editor
        let space_w_workspace = keymap.bindings_for_input(&space_w, &workspace_context());
        assert!(space_w_workspace.0.is_empty());
        assert_eq!(space_w_workspace.1, true);

        let space_w_editor = keymap.bindings_for_input(&space_w, &editor_workspace_context());
        assert!(space_w_editor.0.is_empty());
        assert_eq!(space_w_editor.1, false);

        // Ensure `space w w` results in the binding in the workspace, but not in the editor
        let space_w_w_workspace = keymap.bindings_for_input(&space_w_w, &workspace_context());
        assert!(!space_w_w_workspace.0.is_empty());
        assert_eq!(space_w_w_workspace.1, false);

        let space_w_w_editor = keymap.bindings_for_input(&space_w_w, &editor_workspace_context());
        assert!(space_w_w_editor.0.is_empty());
        assert_eq!(space_w_w_editor.1, false);

        // Now test what happens if we have another binding defined AFTER the NoAction
        // that should result in pending
        let bindings = [
            KeyBinding::new("space w w", ActionAlpha {}, Some("workspace")),
            KeyBinding::new("space w w", NoAction {}, Some("editor")),
            KeyBinding::new("space w x", ActionAlpha {}, Some("editor")),
        ];
        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings.clone());

        let space_editor = keymap.bindings_for_input(&[space()], &editor_workspace_context());
        assert!(space_editor.0.is_empty());
        assert_eq!(space_editor.1, true);

        // Now test what happens if we have another binding defined BEFORE the NoAction
        // that should result in pending
        let bindings = [
            KeyBinding::new("space w w", ActionAlpha {}, Some("workspace")),
            KeyBinding::new("space w x", ActionAlpha {}, Some("editor")),
            KeyBinding::new("space w w", NoAction {}, Some("editor")),
        ];
        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings.clone());

        let space_editor = keymap.bindings_for_input(&[space()], &editor_workspace_context());
        assert!(space_editor.0.is_empty());
        assert_eq!(space_editor.1, true);
    }

    #[test]
    fn test_bindings_for_action() {
        let bindings = [
            KeyBinding::new("ctrl-a", ActionAlpha {}, Some("pane")),
            KeyBinding::new("ctrl-b", ActionBeta {}, Some("editor && mode == full")),
            KeyBinding::new("ctrl-c", ActionGamma {}, Some("workspace")),
            KeyBinding::new("ctrl-a", NoAction {}, Some("pane && active")),
            KeyBinding::new("ctrl-b", NoAction {}, Some("editor")),
        ];

        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings.clone());

        assert_bindings(&keymap, &ActionAlpha {}, &["ctrl-a"]);
        assert_bindings(&keymap, &ActionBeta {}, &[]);
        assert_bindings(&keymap, &ActionGamma {}, &["ctrl-c"]);

        #[track_caller]
        fn assert_bindings(keymap: &Keymap, action: &dyn Action, expected: &[&str]) {
            let actual = keymap
                .bindings_for_action(action)
                .map(|binding| binding.keystrokes[0].unparse())
                .collect::<Vec<_>>();
            assert_eq!(actual, expected, "{:?}", action);
        }
    }
}
