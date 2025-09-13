mod binding;
mod context;

pub use binding::*;
pub use context::*;

use crate::{Action, AsKeystroke, Keystroke, is_no_action};
use collections::{HashMap, HashSet};
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

/// Index of a binding within a keymap.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BindingIndex(usize);

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
    pub fn bindings(&self) -> impl DoubleEndedIterator<Item = &KeyBinding> + ExactSizeIterator {
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
    /// order (higher precedence first, reverse of the order they were added to the keymap).
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
        input: &[impl AsKeystroke],
        context_stack: &[KeyContext],
    ) -> (SmallVec<[KeyBinding; 1]>, bool) {
        let mut matched_bindings = SmallVec::<[(usize, BindingIndex, &KeyBinding); 1]>::new();
        let mut pending_bindings = SmallVec::<[(BindingIndex, &KeyBinding); 1]>::new();

        for (ix, binding) in self.bindings().enumerate().rev() {
            let Some(depth) = self.binding_enabled(binding, context_stack) else {
                continue;
            };
            let Some(pending) = binding.match_keystrokes(input) else {
                continue;
            };

            if !pending {
                matched_bindings.push((depth, BindingIndex(ix), binding));
            } else {
                pending_bindings.push((BindingIndex(ix), binding));
            }
        }

        matched_bindings.sort_by(|(depth_a, ix_a, _), (depth_b, ix_b, _)| {
            depth_b.cmp(depth_a).then(ix_b.cmp(ix_a))
        });

        let mut bindings: SmallVec<[_; 1]> = SmallVec::new();
        let mut first_binding_index = None;

        for (_, ix, binding) in matched_bindings {
            if is_no_action(&*binding.action) {
                // Only break if this is a user-defined NoAction binding
                // This allows user keymaps to override base keymap NoAction bindings
                if let Some(meta) = binding.meta {
                    if meta.0 == 0 {
                        break;
                    }
                } else {
                    // If no meta is set, assume it's a user binding for safety
                    break;
                }
                // For non-user NoAction bindings, continue searching for user overrides
                continue;
            }
            bindings.push(binding.clone());
            first_binding_index.get_or_insert(ix);
        }

        let mut pending = HashSet::default();
        for (ix, binding) in pending_bindings.into_iter().rev() {
            if let Some(binding_ix) = first_binding_index
                && binding_ix > ix
            {
                continue;
            }
            if is_no_action(&*binding.action) {
                pending.remove(&&binding.keystrokes);
                continue;
            }
            pending.insert(&binding.keystrokes);
        }

        (bindings, !pending.is_empty())
    }
    /// Check if the given binding is enabled, given a certain key context.
    /// Returns the deepest depth at which the binding matches, or None if it doesn't match.
    fn binding_enabled(&self, binding: &KeyBinding, contexts: &[KeyContext]) -> Option<usize> {
        if let Some(predicate) = &binding.context_predicate {
            predicate.depth_of(contexts)
        } else {
            Some(contexts.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as gpui;
    use gpui::NoAction;

    actions!(
        test_only,
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
        assert_eq!(keymap.binding_enabled(&bindings[0], &[]), Some(0));
        assert_eq!(
            keymap.binding_enabled(&bindings[0], &[KeyContext::parse("terminal").unwrap()]),
            Some(1)
        );

        // contextual bindings are enabled in contexts that match their predicate
        assert_eq!(
            keymap.binding_enabled(&bindings[1], &[KeyContext::parse("barf x=y").unwrap()]),
            None
        );
        assert_eq!(
            keymap.binding_enabled(&bindings[1], &[KeyContext::parse("pane x=y").unwrap()]),
            Some(1)
        );

        assert_eq!(
            keymap.binding_enabled(&bindings[2], &[KeyContext::parse("editor").unwrap()]),
            None
        );
        assert_eq!(
            keymap.binding_enabled(
                &bindings[2],
                &[KeyContext::parse("editor mode=full").unwrap()]
            ),
            Some(1)
        );
    }

    #[test]
    fn test_depth_precedence() {
        let bindings = [
            KeyBinding::new("ctrl-a", ActionBeta {}, Some("pane")),
            KeyBinding::new("ctrl-a", ActionGamma {}, Some("editor")),
        ];

        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        let (result, pending) = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-a").unwrap()],
            &[
                KeyContext::parse("pane").unwrap(),
                KeyContext::parse("editor").unwrap(),
            ],
        );

        assert!(!pending);
        assert_eq!(result.len(), 2);
        assert!(result[0].action.partial_eq(&ActionGamma {}));
        assert!(result[1].action.partial_eq(&ActionBeta {}));
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
        keymap.add_bindings(bindings);

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
        keymap.add_bindings(bindings);

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
        assert!(space_workspace.1);

        let space_editor = keymap.bindings_for_input(&[space()], &editor_workspace_context());
        assert!(space_editor.0.is_empty());
        assert!(!space_editor.1);

        // Ensure `space w` results in pending input on the workspace, but not editor
        let space_w_workspace = keymap.bindings_for_input(&space_w, &workspace_context());
        assert!(space_w_workspace.0.is_empty());
        assert!(space_w_workspace.1);

        let space_w_editor = keymap.bindings_for_input(&space_w, &editor_workspace_context());
        assert!(space_w_editor.0.is_empty());
        assert!(!space_w_editor.1);

        // Ensure `space w w` results in the binding in the workspace, but not in the editor
        let space_w_w_workspace = keymap.bindings_for_input(&space_w_w, &workspace_context());
        assert!(!space_w_w_workspace.0.is_empty());
        assert!(!space_w_w_workspace.1);

        let space_w_w_editor = keymap.bindings_for_input(&space_w_w, &editor_workspace_context());
        assert!(space_w_w_editor.0.is_empty());
        assert!(!space_w_w_editor.1);

        // Now test what happens if we have another binding defined AFTER the NoAction
        // that should result in pending
        let bindings = [
            KeyBinding::new("space w w", ActionAlpha {}, Some("workspace")),
            KeyBinding::new("space w w", NoAction {}, Some("editor")),
            KeyBinding::new("space w x", ActionAlpha {}, Some("editor")),
        ];
        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        let space_editor = keymap.bindings_for_input(&[space()], &editor_workspace_context());
        assert!(space_editor.0.is_empty());
        assert!(space_editor.1);

        // Now test what happens if we have another binding defined BEFORE the NoAction
        // that should result in pending
        let bindings = [
            KeyBinding::new("space w w", ActionAlpha {}, Some("workspace")),
            KeyBinding::new("space w x", ActionAlpha {}, Some("editor")),
            KeyBinding::new("space w w", NoAction {}, Some("editor")),
        ];
        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        let space_editor = keymap.bindings_for_input(&[space()], &editor_workspace_context());
        assert!(space_editor.0.is_empty());
        assert!(space_editor.1);

        // Now test what happens if we have another binding defined at a higher context
        // that should result in pending
        let bindings = [
            KeyBinding::new("space w w", ActionAlpha {}, Some("workspace")),
            KeyBinding::new("space w x", ActionAlpha {}, Some("workspace")),
            KeyBinding::new("space w w", NoAction {}, Some("editor")),
        ];
        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        let space_editor = keymap.bindings_for_input(&[space()], &editor_workspace_context());
        assert!(space_editor.0.is_empty());
        assert!(space_editor.1);
    }

    #[test]
    fn test_override_multikey() {
        let bindings = [
            KeyBinding::new("ctrl-w left", ActionAlpha {}, Some("editor")),
            KeyBinding::new("ctrl-w", NoAction {}, Some("editor")),
        ];

        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        // Ensure `space` results in pending input on the workspace, but not editor
        let (result, pending) = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-w").unwrap()],
            &[KeyContext::parse("editor").unwrap()],
        );
        assert!(result.is_empty());
        assert!(pending);

        let bindings = [
            KeyBinding::new("ctrl-w left", ActionAlpha {}, Some("editor")),
            KeyBinding::new("ctrl-w", ActionBeta {}, Some("editor")),
        ];

        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        // Ensure `space` results in pending input on the workspace, but not editor
        let (result, pending) = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-w").unwrap()],
            &[KeyContext::parse("editor").unwrap()],
        );
        assert_eq!(result.len(), 1);
        assert!(!pending);
    }

    #[test]
    fn test_simple_disable() {
        let bindings = [
            KeyBinding::new("ctrl-x", ActionAlpha {}, Some("editor")),
            KeyBinding::new("ctrl-x", NoAction {}, Some("editor")),
        ];

        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        // Ensure `space` results in pending input on the workspace, but not editor
        let (result, pending) = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-x").unwrap()],
            &[KeyContext::parse("editor").unwrap()],
        );
        assert!(result.is_empty());
        assert!(!pending);
    }

    #[test]
    fn test_fail_to_disable() {
        // disabled at the wrong level
        let bindings = [
            KeyBinding::new("ctrl-x", ActionAlpha {}, Some("editor")),
            KeyBinding::new("ctrl-x", NoAction {}, Some("workspace")),
        ];

        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        // Ensure `space` results in pending input on the workspace, but not editor
        let (result, pending) = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-x").unwrap()],
            &[
                KeyContext::parse("workspace").unwrap(),
                KeyContext::parse("editor").unwrap(),
            ],
        );
        assert_eq!(result.len(), 1);
        assert!(!pending);
    }

    #[test]
    fn test_disable_deeper() {
        let bindings = [
            KeyBinding::new("ctrl-x", ActionAlpha {}, Some("workspace")),
            KeyBinding::new("ctrl-x", NoAction {}, Some("editor")),
        ];

        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        // Ensure `space` results in pending input on the workspace, but not editor
        let (result, pending) = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-x").unwrap()],
            &[
                KeyContext::parse("workspace").unwrap(),
                KeyContext::parse("editor").unwrap(),
            ],
        );
        assert_eq!(result.len(), 0);
        assert!(!pending);
    }

    #[test]
    fn test_pending_match_enabled() {
        let bindings = [
            KeyBinding::new("ctrl-x", ActionBeta, Some("vim_mode == normal")),
            KeyBinding::new("ctrl-x 0", ActionAlpha, Some("Workspace")),
        ];
        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        let matched = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-x")].map(Result::unwrap),
            &[
                KeyContext::parse("Workspace"),
                KeyContext::parse("Pane"),
                KeyContext::parse("Editor vim_mode=normal"),
            ]
            .map(Result::unwrap),
        );
        assert_eq!(matched.0.len(), 1);
        assert!(matched.0[0].action.partial_eq(&ActionBeta));
        assert!(matched.1);
    }

    #[test]
    fn test_pending_match_enabled_extended() {
        let bindings = [
            KeyBinding::new("ctrl-x", ActionBeta, Some("vim_mode == normal")),
            KeyBinding::new("ctrl-x 0", NoAction, Some("Workspace")),
        ];
        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        let matched = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-x")].map(Result::unwrap),
            &[
                KeyContext::parse("Workspace"),
                KeyContext::parse("Pane"),
                KeyContext::parse("Editor vim_mode=normal"),
            ]
            .map(Result::unwrap),
        );
        assert_eq!(matched.0.len(), 1);
        assert!(matched.0[0].action.partial_eq(&ActionBeta));
        assert!(!matched.1);
        let bindings = [
            KeyBinding::new("ctrl-x", ActionBeta, Some("Workspace")),
            KeyBinding::new("ctrl-x 0", NoAction, Some("vim_mode == normal")),
        ];
        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        let matched = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-x")].map(Result::unwrap),
            &[
                KeyContext::parse("Workspace"),
                KeyContext::parse("Pane"),
                KeyContext::parse("Editor vim_mode=normal"),
            ]
            .map(Result::unwrap),
        );
        assert_eq!(matched.0.len(), 1);
        assert!(matched.0[0].action.partial_eq(&ActionBeta));
        assert!(!matched.1);
    }

    #[test]
    fn test_overriding_prefix() {
        let bindings = [
            KeyBinding::new("ctrl-x 0", ActionAlpha, Some("Workspace")),
            KeyBinding::new("ctrl-x", ActionBeta, Some("vim_mode == normal")),
        ];
        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        let matched = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-x")].map(Result::unwrap),
            &[
                KeyContext::parse("Workspace"),
                KeyContext::parse("Pane"),
                KeyContext::parse("Editor vim_mode=normal"),
            ]
            .map(Result::unwrap),
        );
        assert_eq!(matched.0.len(), 1);
        assert!(matched.0[0].action.partial_eq(&ActionBeta));
        assert!(!matched.1);
    }

    #[test]
    fn test_context_precedence_with_same_source() {
        // Test case: User has both Workspace and Editor bindings for the same key
        // Editor binding should take precedence over Workspace binding
        let bindings = [
            KeyBinding::new("cmd-r", ActionAlpha {}, Some("Workspace")),
            KeyBinding::new("cmd-r", ActionBeta {}, Some("Editor")),
        ];

        let mut keymap = Keymap::default();
        keymap.add_bindings(bindings);

        // Test with context stack: [Workspace, Editor] (Editor is deeper)
        let (result, _) = keymap.bindings_for_input(
            &[Keystroke::parse("cmd-r").unwrap()],
            &[
                KeyContext::parse("Workspace").unwrap(),
                KeyContext::parse("Editor").unwrap(),
            ],
        );

        // Both bindings should be returned, but Editor binding should be first (highest precedence)
        assert_eq!(result.len(), 2);
        assert!(result[0].action.partial_eq(&ActionBeta {})); // Editor binding first
        assert!(result[1].action.partial_eq(&ActionAlpha {})); // Workspace binding second
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
        keymap.add_bindings(bindings);

        assert_bindings(&keymap, &ActionAlpha {}, &["ctrl-a"]);
        assert_bindings(&keymap, &ActionBeta {}, &[]);
        assert_bindings(&keymap, &ActionGamma {}, &["ctrl-c"]);

        #[track_caller]
        fn assert_bindings(keymap: &Keymap, action: &dyn Action, expected: &[&str]) {
            let actual = keymap
                .bindings_for_action(action)
                .map(|binding| binding.keystrokes[0].inner().unparse())
                .collect::<Vec<_>>();
            assert_eq!(actual, expected, "{:?}", action);
        }
    }

    #[test]
    fn test_source_precedence_sorting() {
        // KeybindSource precedence: User (0) > Vim (1) > Base (2) > Default (3)
        // Test that user keymaps take precedence over default keymaps at the same context depth
        let mut keymap = Keymap::default();

        // Add a default keymap binding first
        let mut default_binding = KeyBinding::new("cmd-r", ActionAlpha {}, Some("Editor"));
        default_binding.set_meta(KeyBindingMetaIndex(3)); // Default source
        keymap.add_bindings([default_binding]);

        // Add a user keymap binding
        let mut user_binding = KeyBinding::new("cmd-r", ActionBeta {}, Some("Editor"));
        user_binding.set_meta(KeyBindingMetaIndex(0)); // User source
        keymap.add_bindings([user_binding]);

        // Test with Editor context stack
        let (result, _) = keymap.bindings_for_input(
            &[Keystroke::parse("cmd-r").unwrap()],
            &[KeyContext::parse("Editor").unwrap()],
        );

        // User binding should take precedence over default binding
        assert_eq!(result.len(), 2);
        assert!(result[0].action.partial_eq(&ActionBeta {}));
        assert!(result[1].action.partial_eq(&ActionAlpha {}));
    }
}
