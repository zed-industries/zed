mod binding;
mod keymap;
mod keymap_context;
mod keystroke;

use std::{any::TypeId, fmt::Debug};

use collections::HashMap;
use smallvec::SmallVec;

use crate::Action;

pub use binding::{Binding, BindingMatchResult};
pub use keymap::Keymap;
pub use keymap_context::{KeymapContext, KeymapContextPredicate};
pub use keystroke::Keystroke;

pub struct KeymapMatcher {
    pub contexts: Vec<KeymapContext>,
    pending_views: HashMap<usize, KeymapContext>,
    pending_keystrokes: Vec<Keystroke>,
    keymap: Keymap,
}

impl KeymapMatcher {
    pub fn new(keymap: Keymap) -> Self {
        Self {
            contexts: Vec::new(),
            pending_views: Default::default(),
            pending_keystrokes: Vec::new(),
            keymap,
        }
    }

    pub fn set_keymap(&mut self, keymap: Keymap) {
        self.clear_pending();
        self.keymap = keymap;
    }

    pub fn add_bindings<T: IntoIterator<Item = Binding>>(&mut self, bindings: T) {
        self.clear_pending();
        self.keymap.add_bindings(bindings);
    }

    pub fn clear_bindings(&mut self) {
        self.clear_pending();
        self.keymap.clear();
    }

    pub fn bindings_for_action_type(&self, action_type: TypeId) -> impl Iterator<Item = &Binding> {
        self.keymap.bindings_for_action_type(action_type)
    }

    pub fn clear_pending(&mut self) {
        self.pending_keystrokes.clear();
        self.pending_views.clear();
    }

    pub fn has_pending_keystrokes(&self) -> bool {
        !self.pending_keystrokes.is_empty()
    }

    /// Pushes a keystroke onto the matcher.
    /// The result of the new keystroke is returned:
    ///     MatchResult::None =>
    ///         No match is valid for this key given any pending keystrokes.
    ///     MatchResult::Pending =>
    ///         There exist bindings which are still waiting for more keys.
    ///     MatchResult::Complete(matches) =>
    ///         1 or more bindings have recieved the necessary key presses.
    ///         The order of the matched actions is by position of the matching first,
    //          and order in the keymap second.
    pub fn push_keystroke(
        &mut self,
        keystroke: Keystroke,
        mut dispatch_path: Vec<(usize, KeymapContext)>,
    ) -> MatchResult {
        let mut any_pending = false;
        // Collect matched bindings into an ordered list using the position in the matching binding first,
        // and then the order the binding matched in the view tree second.
        // The key is the reverse position of the binding in the bindings list so that later bindings
        // match before earlier ones in the user's config
        let mut matched_bindings: Vec<(usize, Box<dyn Action>)> = Default::default();

        let first_keystroke = self.pending_keystrokes.is_empty();
        self.pending_keystrokes.push(keystroke.clone());

        self.contexts.clear();
        self.contexts
            .extend(dispatch_path.iter_mut().map(|e| std::mem::take(&mut e.1)));

        // Find the bindings which map the pending keystrokes and current context
        for (i, (view_id, _)) in dispatch_path.iter().enumerate() {
            // Don't require pending view entry if there are no pending keystrokes
            if !first_keystroke && !self.pending_views.contains_key(view_id) {
                continue;
            }

            // If there is a previous view context, invalidate that view if it
            // has changed
            if let Some(previous_view_context) = self.pending_views.remove(view_id) {
                if previous_view_context != self.contexts[i] {
                    continue;
                }
            }

            for binding in self.keymap.bindings().iter().rev() {
                match binding.match_keys_and_context(&self.pending_keystrokes, &self.contexts[i..])
                {
                    BindingMatchResult::Complete(action) => {
                        matched_bindings.push((*view_id, action));
                    }
                    BindingMatchResult::Partial => {
                        self.pending_views
                            .insert(*view_id, self.contexts[i].clone());
                        any_pending = true;
                    }
                    _ => {}
                }
            }
        }

        if !any_pending {
            self.clear_pending();
        }

        if !matched_bindings.is_empty() {
            // Collect the sorted matched bindings into the final vec for ease of use
            // Matched bindings are in order by precedence
            MatchResult::Matches(matched_bindings)
        } else if any_pending {
            MatchResult::Pending
        } else {
            MatchResult::None
        }
    }

    pub fn keystrokes_for_action(
        &self,
        action: &dyn Action,
        contexts: &[KeymapContext],
    ) -> Option<SmallVec<[Keystroke; 2]>> {
        self.keymap
            .bindings()
            .iter()
            .rev()
            .find_map(|binding| binding.keystrokes_for_action(action, contexts))
    }
}

impl Default for KeymapMatcher {
    fn default() -> Self {
        Self::new(Keymap::default())
    }
}

pub enum MatchResult {
    None,
    Pending,
    Matches(Vec<(usize, Box<dyn Action>)>),
}

impl Debug for MatchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchResult::None => f.debug_struct("MatchResult::None").finish(),
            MatchResult::Pending => f.debug_struct("MatchResult::Pending").finish(),
            MatchResult::Matches(matches) => f
                .debug_list()
                .entries(
                    matches
                        .iter()
                        .map(|(view_id, action)| format!("{view_id}, {}", action.name())),
                )
                .finish(),
        }
    }
}

impl PartialEq for MatchResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MatchResult::None, MatchResult::None) => true,
            (MatchResult::Pending, MatchResult::Pending) => true,
            (MatchResult::Matches(matches), MatchResult::Matches(other_matches)) => {
                matches.len() == other_matches.len()
                    && matches.iter().zip(other_matches.iter()).all(
                        |((view_id, action), (other_view_id, other_action))| {
                            view_id == other_view_id && action.eq(other_action.as_ref())
                        },
                    )
            }
            _ => false,
        }
    }
}

impl Eq for MatchResult {}

impl Clone for MatchResult {
    fn clone(&self) -> Self {
        match self {
            MatchResult::None => MatchResult::None,
            MatchResult::Pending => MatchResult::Pending,
            MatchResult::Matches(matches) => MatchResult::Matches(
                matches
                    .iter()
                    .map(|(view_id, action)| (*view_id, Action::boxed_clone(action.as_ref())))
                    .collect(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use serde::Deserialize;

    use crate::{actions, impl_actions, keymap_matcher::KeymapContext};

    use super::*;

    #[test]
    fn test_keymap_and_view_ordering() -> Result<()> {
        actions!(test, [EditorAction, ProjectPanelAction]);

        let mut editor = KeymapContext::default();
        editor.add_identifier("Editor");

        let mut project_panel = KeymapContext::default();
        project_panel.add_identifier("ProjectPanel");

        // Editor 'deeper' in than project panel
        let dispatch_path = vec![(2, editor), (1, project_panel)];

        // But editor actions 'higher' up in keymap
        let keymap = Keymap::new(vec![
            Binding::new("left", EditorAction, Some("Editor")),
            Binding::new("left", ProjectPanelAction, Some("ProjectPanel")),
        ]);

        let mut matcher = KeymapMatcher::new(keymap);

        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("left")?, dispatch_path.clone()),
            MatchResult::Matches(vec![
                (2, Box::new(EditorAction)),
                (1, Box::new(ProjectPanelAction)),
            ]),
        );

        Ok(())
    }

    #[test]
    fn test_push_keystroke() -> Result<()> {
        actions!(test, [B, AB, C, D, DA, E, EF]);

        let mut context1 = KeymapContext::default();
        context1.add_identifier("1");

        let mut context2 = KeymapContext::default();
        context2.add_identifier("2");

        let dispatch_path = vec![(2, context2), (1, context1)];

        let keymap = Keymap::new(vec![
            Binding::new("a b", AB, Some("1")),
            Binding::new("b", B, Some("2")),
            Binding::new("c", C, Some("2")),
            Binding::new("d", D, Some("1")),
            Binding::new("d", D, Some("2")),
            Binding::new("d a", DA, Some("2")),
        ]);

        let mut matcher = KeymapMatcher::new(keymap);

        // Binding with pending prefix always takes precedence
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("a")?, dispatch_path.clone()),
            MatchResult::Pending,
        );
        // B alone doesn't match because a was pending, so AB is returned instead
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("b")?, dispatch_path.clone()),
            MatchResult::Matches(vec![(1, Box::new(AB))]),
        );
        assert!(!matcher.has_pending_keystrokes());

        // Without an a prefix, B is dispatched like expected
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("b")?, dispatch_path.clone()),
            MatchResult::Matches(vec![(2, Box::new(B))]),
        );
        assert!(!matcher.has_pending_keystrokes());

        // If a is prefixed, C will not be dispatched because there
        // was a pending binding for it
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("a")?, dispatch_path.clone()),
            MatchResult::Pending,
        );
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("c")?, dispatch_path.clone()),
            MatchResult::None,
        );
        assert!(!matcher.has_pending_keystrokes());

        // If a single keystroke matches multiple bindings in the tree
        // all of them are returned so that we can fallback if the action
        // handler decides to propagate the action
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("d")?, dispatch_path.clone()),
            MatchResult::Matches(vec![(2, Box::new(D)), (1, Box::new(D))]),
        );

        // If none of the d action handlers consume the binding, a pending
        // binding may then be used
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("a")?, dispatch_path.clone()),
            MatchResult::Matches(vec![(2, Box::new(DA))]),
        );
        assert!(!matcher.has_pending_keystrokes());

        Ok(())
    }

    #[test]
    fn test_keystroke_parsing() -> Result<()> {
        assert_eq!(
            Keystroke::parse("ctrl-p")?,
            Keystroke {
                key: "p".into(),
                ctrl: true,
                alt: false,
                shift: false,
                cmd: false,
                function: false,
            }
        );

        assert_eq!(
            Keystroke::parse("alt-shift-down")?,
            Keystroke {
                key: "down".into(),
                ctrl: false,
                alt: true,
                shift: true,
                cmd: false,
                function: false,
            }
        );

        assert_eq!(
            Keystroke::parse("shift-cmd--")?,
            Keystroke {
                key: "-".into(),
                ctrl: false,
                alt: false,
                shift: true,
                cmd: true,
                function: false,
            }
        );

        Ok(())
    }

    #[test]
    fn test_context_predicate_parsing() -> Result<()> {
        use KeymapContextPredicate::*;

        assert_eq!(
            KeymapContextPredicate::parse("a && (b == c || d != e)")?,
            And(
                Box::new(Identifier("a".into())),
                Box::new(Or(
                    Box::new(Equal("b".into(), "c".into())),
                    Box::new(NotEqual("d".into(), "e".into())),
                ))
            )
        );

        assert_eq!(
            KeymapContextPredicate::parse("!a")?,
            Not(Box::new(Identifier("a".into())),)
        );

        Ok(())
    }

    #[test]
    fn test_context_predicate_eval() {
        let predicate = KeymapContextPredicate::parse("a && b || c == d").unwrap();

        let mut context = KeymapContext::default();
        context.add_identifier("a");
        assert!(!predicate.eval(&[context]));

        let mut context = KeymapContext::default();
        context.add_identifier("a");
        context.add_identifier("b");
        assert!(predicate.eval(&[context]));

        let mut context = KeymapContext::default();
        context.add_identifier("a");
        context.add_key("c", "x");
        assert!(!predicate.eval(&[context]));

        let mut context = KeymapContext::default();
        context.add_identifier("a");
        context.add_key("c", "d");
        assert!(predicate.eval(&[context]));

        let predicate = KeymapContextPredicate::parse("!a").unwrap();
        assert!(predicate.eval(&[KeymapContext::default()]));
    }

    #[test]
    fn test_context_child_predicate_eval() {
        let predicate = KeymapContextPredicate::parse("a && b > c").unwrap();
        let contexts = [
            context_set(&["e", "f"]),
            context_set(&["c", "d"]), // match this context
            context_set(&["a", "b"]),
        ];

        assert!(!predicate.eval(&contexts[0..]));
        assert!(predicate.eval(&contexts[1..]));
        assert!(!predicate.eval(&contexts[2..]));

        let predicate = KeymapContextPredicate::parse("a && b > c && !d > e").unwrap();
        let contexts = [
            context_set(&["f"]),
            context_set(&["e"]), // only match this context
            context_set(&["c"]),
            context_set(&["a", "b"]),
            context_set(&["e"]),
            context_set(&["c", "d"]),
            context_set(&["a", "b"]),
        ];

        assert!(!predicate.eval(&contexts[0..]));
        assert!(predicate.eval(&contexts[1..]));
        assert!(!predicate.eval(&contexts[2..]));
        assert!(!predicate.eval(&contexts[3..]));
        assert!(!predicate.eval(&contexts[4..]));
        assert!(!predicate.eval(&contexts[5..]));
        assert!(!predicate.eval(&contexts[6..]));

        fn context_set(names: &[&str]) -> KeymapContext {
            let mut keymap = KeymapContext::new();
            names
                .iter()
                .for_each(|name| keymap.add_identifier(name.to_string()));
            keymap
        }
    }

    #[test]
    fn test_matcher() -> Result<()> {
        #[derive(Clone, Deserialize, PartialEq, Eq, Debug)]
        pub struct A(pub String);
        impl_actions!(test, [A]);
        actions!(test, [B, Ab]);

        #[derive(Clone, Debug, Eq, PartialEq)]
        struct ActionArg {
            a: &'static str,
        }

        let keymap = Keymap::new(vec![
            Binding::new("a", A("x".to_string()), Some("a")),
            Binding::new("b", B, Some("a")),
            Binding::new("a b", Ab, Some("a || b")),
        ]);

        let mut context_a = KeymapContext::default();
        context_a.add_identifier("a");

        let mut context_b = KeymapContext::default();
        context_b.add_identifier("b");

        let mut matcher = KeymapMatcher::new(keymap);

        // Basic match
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("a")?, vec![(1, context_a.clone())]),
            MatchResult::Matches(vec![(1, Box::new(A("x".to_string())))])
        );
        matcher.clear_pending();

        // Multi-keystroke match
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("a")?, vec![(1, context_b.clone())]),
            MatchResult::Pending
        );
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("b")?, vec![(1, context_b.clone())]),
            MatchResult::Matches(vec![(1, Box::new(Ab))])
        );
        matcher.clear_pending();

        // Failed matches don't interfere with matching subsequent keys
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("x")?, vec![(1, context_a.clone())]),
            MatchResult::None
        );
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("a")?, vec![(1, context_a.clone())]),
            MatchResult::Matches(vec![(1, Box::new(A("x".to_string())))])
        );
        matcher.clear_pending();

        // Pending keystrokes are cleared when the context changes
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("a")?, vec![(1, context_b.clone())]),
            MatchResult::Pending
        );
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("b")?, vec![(1, context_a.clone())]),
            MatchResult::None
        );
        matcher.clear_pending();

        let mut context_c = KeymapContext::default();
        context_c.add_identifier("c");

        // Pending keystrokes are maintained per-view
        assert_eq!(
            matcher.push_keystroke(
                Keystroke::parse("a")?,
                vec![(1, context_b.clone()), (2, context_c.clone())]
            ),
            MatchResult::Pending
        );
        assert_eq!(
            matcher.push_keystroke(Keystroke::parse("b")?, vec![(1, context_b.clone())]),
            MatchResult::Matches(vec![(1, Box::new(Ab))])
        );

        Ok(())
    }
}
