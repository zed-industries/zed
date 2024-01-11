use crate::{Action, KeyContext, Keymap, KeymapVersion, Keystroke};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct KeystrokeMatcher {
    pending_keystrokes: Vec<Keystroke>,
    keymap: Arc<Mutex<Keymap>>,
    keymap_version: KeymapVersion,
}

impl KeystrokeMatcher {
    pub fn new(keymap: Arc<Mutex<Keymap>>) -> Self {
        let keymap_version = keymap.lock().version();
        Self {
            pending_keystrokes: Vec::new(),
            keymap_version,
            keymap,
        }
    }

    pub fn clear_pending(&mut self) {
        self.pending_keystrokes.clear();
    }

    pub fn has_pending_keystrokes(&self) -> bool {
        !self.pending_keystrokes.is_empty()
    }

    /// Pushes a keystroke onto the matcher.
    /// The result of the new keystroke is returned:
    /// - KeyMatch::None =>
    ///         No match is valid for this key given any pending keystrokes.
    /// - KeyMatch::Pending =>
    ///         There exist bindings which are still waiting for more keys.
    /// - KeyMatch::Complete(matches) =>
    ///         One or more bindings have received the necessary key presses.
    ///         Bindings added later will take precedence over earlier bindings.
    pub fn match_keystroke(
        &mut self,
        keystroke: &Keystroke,
        context_stack: &[KeyContext],
    ) -> KeyMatch {
        let keymap = self.keymap.lock();
        // Clear pending keystrokes if the keymap has changed since the last matched keystroke.
        if keymap.version() != self.keymap_version {
            self.keymap_version = keymap.version();
            self.pending_keystrokes.clear();
        }

        let mut pending_key = None;
        let mut found_actions = Vec::new();

        for binding in keymap.bindings().rev() {
            if !keymap.binding_enabled(binding, context_stack) {
                continue;
            }

            for candidate in keystroke.match_candidates() {
                self.pending_keystrokes.push(candidate.clone());
                match binding.match_keystrokes(&self.pending_keystrokes) {
                    KeyMatch::Some(mut actions) => {
                        found_actions.append(&mut actions);
                    }
                    KeyMatch::Pending => {
                        pending_key.get_or_insert(candidate);
                    }
                    KeyMatch::None => {}
                }
                self.pending_keystrokes.pop();
            }
        }

        if !found_actions.is_empty() {
            self.pending_keystrokes.clear();
            return KeyMatch::Some(found_actions);
        }

        if let Some(pending_key) = pending_key {
            self.pending_keystrokes.push(pending_key);
            KeyMatch::Pending
        } else {
            self.pending_keystrokes.clear();
            KeyMatch::None
        }
    }
}

#[derive(Debug)]
pub enum KeyMatch {
    None,
    Pending,
    Some(Vec<Box<dyn Action>>),
}

impl KeyMatch {
    pub fn is_some(&self) -> bool {
        matches!(self, KeyMatch::Some(_))
    }

    pub fn matches(self) -> Option<Vec<Box<dyn Action>>> {
        match self {
            KeyMatch::Some(matches) => Some(matches),
            _ => None,
        }
    }
}

impl PartialEq for KeyMatch {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (KeyMatch::None, KeyMatch::None) => true,
            (KeyMatch::Pending, KeyMatch::Pending) => true,
            (KeyMatch::Some(a), KeyMatch::Some(b)) => {
                if a.len() != b.len() {
                    return false;
                }

                for (a, b) in a.iter().zip(b.iter()) {
                    if !a.partial_eq(b.as_ref()) {
                        return false;
                    }
                }

                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use serde_derive::Deserialize;

    use super::*;
    use crate::{self as gpui, KeyBindingContextPredicate, Modifiers};
    use crate::{actions, KeyBinding};

    #[test]
    fn test_keymap_and_view_ordering() {
        actions!(test, [EditorAction, ProjectPanelAction]);

        let mut editor = KeyContext::default();
        editor.add("Editor");

        let mut project_panel = KeyContext::default();
        project_panel.add("ProjectPanel");

        // Editor 'deeper' in than project panel
        let dispatch_path = vec![project_panel, editor];

        // But editor actions 'higher' up in keymap
        let keymap = Keymap::new(vec![
            KeyBinding::new("left", EditorAction, Some("Editor")),
            KeyBinding::new("left", ProjectPanelAction, Some("ProjectPanel")),
        ]);

        let mut matcher = KeystrokeMatcher::new(Arc::new(Mutex::new(keymap)));

        let matches = matcher
            .match_keystroke(&Keystroke::parse("left").unwrap(), &dispatch_path)
            .matches()
            .unwrap();

        assert!(matches[0].partial_eq(&EditorAction));
        assert!(matches.get(1).is_none());
    }

    #[test]
    fn test_multi_keystroke_match() {
        actions!(test, [B, AB, C, D, DA, E, EF]);

        let mut context1 = KeyContext::default();
        context1.add("1");

        let mut context2 = KeyContext::default();
        context2.add("2");

        let dispatch_path = vec![context2, context1];

        let keymap = Keymap::new(vec![
            KeyBinding::new("a b", AB, Some("1")),
            KeyBinding::new("b", B, Some("2")),
            KeyBinding::new("c", C, Some("2")),
            KeyBinding::new("d", D, Some("1")),
            KeyBinding::new("d", D, Some("2")),
            KeyBinding::new("d a", DA, Some("2")),
        ]);

        let mut matcher = KeystrokeMatcher::new(Arc::new(Mutex::new(keymap)));

        // Binding with pending prefix always takes precedence
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("a").unwrap(), &dispatch_path),
            KeyMatch::Pending,
        );
        // B alone doesn't match because a was pending, so AB is returned instead
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("b").unwrap(), &dispatch_path),
            KeyMatch::Some(vec![Box::new(AB)]),
        );
        assert!(!matcher.has_pending_keystrokes());

        // Without an a prefix, B is dispatched like expected
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("b").unwrap(), &dispatch_path[0..1]),
            KeyMatch::Some(vec![Box::new(B)]),
        );
        assert!(!matcher.has_pending_keystrokes());

        eprintln!("PROBLEM AREA");
        // If a is prefixed, C will not be dispatched because there
        // was a pending binding for it
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("a").unwrap(), &dispatch_path),
            KeyMatch::Pending,
        );
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("c").unwrap(), &dispatch_path),
            KeyMatch::None,
        );
        assert!(!matcher.has_pending_keystrokes());

        // If a single keystroke matches multiple bindings in the tree
        // only one of them is returned.
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("d").unwrap(), &dispatch_path),
            KeyMatch::Some(vec![Box::new(D)]),
        );
    }

    #[test]
    fn test_keystroke_parsing() {
        assert_eq!(
            Keystroke::parse("ctrl-p").unwrap(),
            Keystroke {
                key: "p".into(),
                modifiers: Modifiers {
                    control: true,
                    alt: false,
                    shift: false,
                    command: false,
                    function: false,
                },
                ime_key: None,
            }
        );

        assert_eq!(
            Keystroke::parse("alt-shift-down").unwrap(),
            Keystroke {
                key: "down".into(),
                modifiers: Modifiers {
                    control: false,
                    alt: true,
                    shift: true,
                    command: false,
                    function: false,
                },
                ime_key: None,
            }
        );

        assert_eq!(
            Keystroke::parse("shift-cmd--").unwrap(),
            Keystroke {
                key: "-".into(),
                modifiers: Modifiers {
                    control: false,
                    alt: false,
                    shift: true,
                    command: true,
                    function: false,
                },
                ime_key: None,
            }
        );
    }

    #[test]
    fn test_context_predicate_parsing() {
        use KeyBindingContextPredicate::*;

        assert_eq!(
            KeyBindingContextPredicate::parse("a && (b == c || d != e)").unwrap(),
            And(
                Box::new(Identifier("a".into())),
                Box::new(Or(
                    Box::new(Equal("b".into(), "c".into())),
                    Box::new(NotEqual("d".into(), "e".into())),
                ))
            )
        );

        assert_eq!(
            KeyBindingContextPredicate::parse("!a").unwrap(),
            Not(Box::new(Identifier("a".into())),)
        );
    }

    #[test]
    fn test_context_predicate_eval() {
        let predicate = KeyBindingContextPredicate::parse("a && b || c == d").unwrap();

        let mut context = KeyContext::default();
        context.add("a");
        assert!(!predicate.eval(&[context]));

        let mut context = KeyContext::default();
        context.add("a");
        context.add("b");
        assert!(predicate.eval(&[context]));

        let mut context = KeyContext::default();
        context.add("a");
        context.set("c", "x");
        assert!(!predicate.eval(&[context]));

        let mut context = KeyContext::default();
        context.add("a");
        context.set("c", "d");
        assert!(predicate.eval(&[context]));

        let predicate = KeyBindingContextPredicate::parse("!a").unwrap();
        assert!(predicate.eval(&[KeyContext::default()]));
    }

    #[test]
    fn test_context_child_predicate_eval() {
        let predicate = KeyBindingContextPredicate::parse("a && b > c").unwrap();
        let contexts = [
            context_set(&["a", "b"]),
            context_set(&["c", "d"]), // match this context
            context_set(&["e", "f"]),
        ];

        assert!(!predicate.eval(&contexts[..=0]));
        assert!(predicate.eval(&contexts[..=1]));
        assert!(!predicate.eval(&contexts[..=2]));

        let predicate = KeyBindingContextPredicate::parse("a && b > c && !d > e").unwrap();
        let contexts = [
            context_set(&["a", "b"]),
            context_set(&["c", "d"]),
            context_set(&["e"]),
            context_set(&["a", "b"]),
            context_set(&["c"]),
            context_set(&["e"]), // only match this context
            context_set(&["f"]),
        ];

        assert!(!predicate.eval(&contexts[..=0]));
        assert!(!predicate.eval(&contexts[..=1]));
        assert!(!predicate.eval(&contexts[..=2]));
        assert!(!predicate.eval(&contexts[..=3]));
        assert!(!predicate.eval(&contexts[..=4]));
        assert!(predicate.eval(&contexts[..=5]));
        assert!(!predicate.eval(&contexts[..=6]));

        fn context_set(names: &[&str]) -> KeyContext {
            let mut keymap = KeyContext::default();
            names.iter().for_each(|name| keymap.add(name.to_string()));
            keymap
        }
    }

    #[test]
    fn test_matcher() {
        #[derive(Clone, Deserialize, PartialEq, Eq, Debug)]
        pub struct A(pub String);
        impl_actions!(test, [A]);
        actions!(test, [B, Ab, Dollar, Quote, Ess, Backtick]);

        #[derive(Clone, Debug, Eq, PartialEq)]
        struct ActionArg {
            a: &'static str,
        }

        let keymap = Keymap::new(vec![
            KeyBinding::new("a", A("x".to_string()), Some("a")),
            KeyBinding::new("b", B, Some("a")),
            KeyBinding::new("a b", Ab, Some("a || b")),
            KeyBinding::new("$", Dollar, Some("a")),
            KeyBinding::new("\"", Quote, Some("a")),
            KeyBinding::new("alt-s", Ess, Some("a")),
            KeyBinding::new("ctrl-`", Backtick, Some("a")),
        ]);

        let mut context_a = KeyContext::default();
        context_a.add("a");

        let mut context_b = KeyContext::default();
        context_b.add("b");

        let mut matcher = KeystrokeMatcher::new(Arc::new(Mutex::new(keymap)));

        // Basic match
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("a").unwrap(), &[context_a.clone()]),
            KeyMatch::Some(vec![Box::new(A("x".to_string()))])
        );
        matcher.clear_pending();

        // Multi-keystroke match
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("a").unwrap(), &[context_b.clone()]),
            KeyMatch::Pending
        );
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("b").unwrap(), &[context_b.clone()]),
            KeyMatch::Some(vec![Box::new(Ab)])
        );
        matcher.clear_pending();

        // Failed matches don't interfere with matching subsequent keys
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("x").unwrap(), &[context_a.clone()]),
            KeyMatch::None
        );
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("a").unwrap(), &[context_a.clone()]),
            KeyMatch::Some(vec![Box::new(A("x".to_string()))])
        );
        matcher.clear_pending();

        let mut context_c = KeyContext::default();
        context_c.add("c");

        assert_eq!(
            matcher.match_keystroke(
                &Keystroke::parse("a").unwrap(),
                &[context_c.clone(), context_b.clone()]
            ),
            KeyMatch::Pending
        );
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("b").unwrap(), &[context_b.clone()]),
            KeyMatch::Some(vec![Box::new(Ab)])
        );

        // handle Czech $ (option + 4 key)
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("alt-ç->$").unwrap(), &[context_a.clone()]),
            KeyMatch::Some(vec![Box::new(Dollar)])
        );

        // handle Brazillian quote (quote key then space key)
        assert_eq!(
            matcher.match_keystroke(
                &Keystroke::parse("space->\"").unwrap(),
                &[context_a.clone()]
            ),
            KeyMatch::Some(vec![Box::new(Quote)])
        );

        // handle ctrl+` on a brazillian keyboard
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("ctrl-->`").unwrap(), &[context_a.clone()]),
            KeyMatch::Some(vec![Box::new(Backtick)])
        );

        // handle alt-s on a US keyboard
        assert_eq!(
            matcher.match_keystroke(&Keystroke::parse("alt-s->ß").unwrap(), &[context_a.clone()]),
            KeyMatch::Some(vec![Box::new(Ess)])
        );
    }
}
