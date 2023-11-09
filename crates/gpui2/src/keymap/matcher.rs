use crate::{Action, DispatchContext, Keymap, KeymapVersion, Keystroke};
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::sync::Arc;

pub struct KeyMatcher {
    pending_keystrokes: Vec<Keystroke>,
    keymap: Arc<Mutex<Keymap>>,
    keymap_version: KeymapVersion,
}

impl KeyMatcher {
    pub fn new(keymap: Arc<Mutex<Keymap>>) -> Self {
        let keymap_version = keymap.lock().version();
        Self {
            pending_keystrokes: Vec::new(),
            keymap_version,
            keymap,
        }
    }

    // todo!("replace with a function that calls an FnMut for every binding matching the action")
    // pub fn bindings_for_action(&self, action_id: TypeId) -> impl Iterator<Item = &Binding> {
    //     self.keymap.lock().bindings_for_action(action_id)
    // }

    pub fn clear_pending(&mut self) {
        self.pending_keystrokes.clear();
    }

    pub fn has_pending_keystrokes(&self) -> bool {
        !self.pending_keystrokes.is_empty()
    }

    /// Pushes a keystroke onto the matcher.
    /// The result of the new keystroke is returned:
    ///     KeyMatch::None =>
    ///         No match is valid for this key given any pending keystrokes.
    ///     KeyMatch::Pending =>
    ///         There exist bindings which are still waiting for more keys.
    ///     KeyMatch::Complete(matches) =>
    ///         One or more bindings have received the necessary key presses.
    ///         Bindings added later will take precedence over earlier bindings.
    pub fn match_keystroke(
        &mut self,
        keystroke: &Keystroke,
        context_stack: &[&DispatchContext],
    ) -> KeyMatch {
        dbg!(keystroke, &context_stack);
        let keymap = self.keymap.lock();
        // Clear pending keystrokes if the keymap has changed since the last matched keystroke.
        if keymap.version() != self.keymap_version {
            self.keymap_version = keymap.version();
            self.pending_keystrokes.clear();
        }

        let mut pending_key = None;

        for binding in keymap.bindings().iter().rev() {
            for candidate in keystroke.match_candidates() {
                self.pending_keystrokes.push(candidate.clone());
                match binding.match_keystrokes(&self.pending_keystrokes, context_stack) {
                    KeyMatch::Some(action) => {
                        self.pending_keystrokes.clear();
                        return KeyMatch::Some(action);
                    }
                    KeyMatch::Pending => {
                        pending_key.get_or_insert(candidate);
                    }
                    KeyMatch::None => {}
                }
                self.pending_keystrokes.pop();
            }
        }

        if let Some(pending_key) = pending_key {
            self.pending_keystrokes.push(pending_key);
        }

        if self.pending_keystrokes.is_empty() {
            KeyMatch::None
        } else {
            KeyMatch::Pending
        }
    }

    pub fn keystrokes_for_action(
        &self,
        action: &dyn Action,
        contexts: &[&DispatchContext],
    ) -> Option<SmallVec<[Keystroke; 2]>> {
        self.keymap
            .lock()
            .bindings()
            .iter()
            .rev()
            .find_map(|binding| binding.keystrokes_for_action(action, contexts))
    }
}

pub enum KeyMatch {
    None,
    Pending,
    Some(Box<dyn Action>),
}

impl KeyMatch {
    pub fn is_some(&self) -> bool {
        matches!(self, KeyMatch::Some(_))
    }
}

// #[cfg(test)]
// mod tests {
//     use anyhow::Result;
//     use serde::Deserialize;

//     use crate::{actions, impl_actions, keymap_matcher::ActionContext};

//     use super::*;

//     #[test]
//     fn test_keymap_and_view_ordering() -> Result<()> {
//         actions!(test, [EditorAction, ProjectPanelAction]);

//         let mut editor = ActionContext::default();
//         editor.add_identifier("Editor");

//         let mut project_panel = ActionContext::default();
//         project_panel.add_identifier("ProjectPanel");

//         // Editor 'deeper' in than project panel
//         let dispatch_path = vec![(2, editor), (1, project_panel)];

//         // But editor actions 'higher' up in keymap
//         let keymap = Keymap::new(vec![
//             Binding::new("left", EditorAction, Some("Editor")),
//             Binding::new("left", ProjectPanelAction, Some("ProjectPanel")),
//         ]);

//         let mut matcher = KeymapMatcher::new(keymap);

//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("left")?, dispatch_path.clone()),
//             KeyMatch::Matches(vec![
//                 (2, Box::new(EditorAction)),
//                 (1, Box::new(ProjectPanelAction)),
//             ]),
//         );

//         Ok(())
//     }

//     #[test]
//     fn test_push_keystroke() -> Result<()> {
//         actions!(test, [B, AB, C, D, DA, E, EF]);

//         let mut context1 = ActionContext::default();
//         context1.add_identifier("1");

//         let mut context2 = ActionContext::default();
//         context2.add_identifier("2");

//         let dispatch_path = vec![(2, context2), (1, context1)];

//         let keymap = Keymap::new(vec![
//             Binding::new("a b", AB, Some("1")),
//             Binding::new("b", B, Some("2")),
//             Binding::new("c", C, Some("2")),
//             Binding::new("d", D, Some("1")),
//             Binding::new("d", D, Some("2")),
//             Binding::new("d a", DA, Some("2")),
//         ]);

//         let mut matcher = KeymapMatcher::new(keymap);

//         // Binding with pending prefix always takes precedence
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("a")?, dispatch_path.clone()),
//             KeyMatch::Pending,
//         );
//         // B alone doesn't match because a was pending, so AB is returned instead
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("b")?, dispatch_path.clone()),
//             KeyMatch::Matches(vec![(1, Box::new(AB))]),
//         );
//         assert!(!matcher.has_pending_keystrokes());

//         // Without an a prefix, B is dispatched like expected
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("b")?, dispatch_path.clone()),
//             KeyMatch::Matches(vec![(2, Box::new(B))]),
//         );
//         assert!(!matcher.has_pending_keystrokes());

//         // If a is prefixed, C will not be dispatched because there
//         // was a pending binding for it
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("a")?, dispatch_path.clone()),
//             KeyMatch::Pending,
//         );
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("c")?, dispatch_path.clone()),
//             KeyMatch::None,
//         );
//         assert!(!matcher.has_pending_keystrokes());

//         // If a single keystroke matches multiple bindings in the tree
//         // all of them are returned so that we can fallback if the action
//         // handler decides to propagate the action
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("d")?, dispatch_path.clone()),
//             KeyMatch::Matches(vec![(2, Box::new(D)), (1, Box::new(D))]),
//         );

//         // If none of the d action handlers consume the binding, a pending
//         // binding may then be used
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("a")?, dispatch_path.clone()),
//             KeyMatch::Matches(vec![(2, Box::new(DA))]),
//         );
//         assert!(!matcher.has_pending_keystrokes());

//         Ok(())
//     }

//     #[test]
//     fn test_keystroke_parsing() -> Result<()> {
//         assert_eq!(
//             Keystroke::parse("ctrl-p")?,
//             Keystroke {
//                 key: "p".into(),
//                 ctrl: true,
//                 alt: false,
//                 shift: false,
//                 cmd: false,
//                 function: false,
//                 ime_key: None,
//             }
//         );

//         assert_eq!(
//             Keystroke::parse("alt-shift-down")?,
//             Keystroke {
//                 key: "down".into(),
//                 ctrl: false,
//                 alt: true,
//                 shift: true,
//                 cmd: false,
//                 function: false,
//                 ime_key: None,
//             }
//         );

//         assert_eq!(
//             Keystroke::parse("shift-cmd--")?,
//             Keystroke {
//                 key: "-".into(),
//                 ctrl: false,
//                 alt: false,
//                 shift: true,
//                 cmd: true,
//                 function: false,
//                 ime_key: None,
//             }
//         );

//         Ok(())
//     }

//     #[test]
//     fn test_context_predicate_parsing() -> Result<()> {
//         use KeymapContextPredicate::*;

//         assert_eq!(
//             KeymapContextPredicate::parse("a && (b == c || d != e)")?,
//             And(
//                 Box::new(Identifier("a".into())),
//                 Box::new(Or(
//                     Box::new(Equal("b".into(), "c".into())),
//                     Box::new(NotEqual("d".into(), "e".into())),
//                 ))
//             )
//         );

//         assert_eq!(
//             KeymapContextPredicate::parse("!a")?,
//             Not(Box::new(Identifier("a".into())),)
//         );

//         Ok(())
//     }

//     #[test]
//     fn test_context_predicate_eval() {
//         let predicate = KeymapContextPredicate::parse("a && b || c == d").unwrap();

//         let mut context = ActionContext::default();
//         context.add_identifier("a");
//         assert!(!predicate.eval(&[context]));

//         let mut context = ActionContext::default();
//         context.add_identifier("a");
//         context.add_identifier("b");
//         assert!(predicate.eval(&[context]));

//         let mut context = ActionContext::default();
//         context.add_identifier("a");
//         context.add_key("c", "x");
//         assert!(!predicate.eval(&[context]));

//         let mut context = ActionContext::default();
//         context.add_identifier("a");
//         context.add_key("c", "d");
//         assert!(predicate.eval(&[context]));

//         let predicate = KeymapContextPredicate::parse("!a").unwrap();
//         assert!(predicate.eval(&[ActionContext::default()]));
//     }

//     #[test]
//     fn test_context_child_predicate_eval() {
//         let predicate = KeymapContextPredicate::parse("a && b > c").unwrap();
//         let contexts = [
//             context_set(&["e", "f"]),
//             context_set(&["c", "d"]), // match this context
//             context_set(&["a", "b"]),
//         ];

//         assert!(!predicate.eval(&contexts[0..]));
//         assert!(predicate.eval(&contexts[1..]));
//         assert!(!predicate.eval(&contexts[2..]));

//         let predicate = KeymapContextPredicate::parse("a && b > c && !d > e").unwrap();
//         let contexts = [
//             context_set(&["f"]),
//             context_set(&["e"]), // only match this context
//             context_set(&["c"]),
//             context_set(&["a", "b"]),
//             context_set(&["e"]),
//             context_set(&["c", "d"]),
//             context_set(&["a", "b"]),
//         ];

//         assert!(!predicate.eval(&contexts[0..]));
//         assert!(predicate.eval(&contexts[1..]));
//         assert!(!predicate.eval(&contexts[2..]));
//         assert!(!predicate.eval(&contexts[3..]));
//         assert!(!predicate.eval(&contexts[4..]));
//         assert!(!predicate.eval(&contexts[5..]));
//         assert!(!predicate.eval(&contexts[6..]));

//         fn context_set(names: &[&str]) -> ActionContext {
//             let mut keymap = ActionContext::new();
//             names
//                 .iter()
//                 .for_each(|name| keymap.add_identifier(name.to_string()));
//             keymap
//         }
//     }

//     #[test]
//     fn test_matcher() -> Result<()> {
//         #[derive(Clone, Deserialize, PartialEq, Eq, Debug)]
//         pub struct A(pub String);
//         impl_actions!(test, [A]);
//         actions!(test, [B, Ab, Dollar, Quote, Ess, Backtick]);

//         #[derive(Clone, Debug, Eq, PartialEq)]
//         struct ActionArg {
//             a: &'static str,
//         }

//         let keymap = Keymap::new(vec![
//             Binding::new("a", A("x".to_string()), Some("a")),
//             Binding::new("b", B, Some("a")),
//             Binding::new("a b", Ab, Some("a || b")),
//             Binding::new("$", Dollar, Some("a")),
//             Binding::new("\"", Quote, Some("a")),
//             Binding::new("alt-s", Ess, Some("a")),
//             Binding::new("ctrl-`", Backtick, Some("a")),
//         ]);

//         let mut context_a = ActionContext::default();
//         context_a.add_identifier("a");

//         let mut context_b = ActionContext::default();
//         context_b.add_identifier("b");

//         let mut matcher = KeymapMatcher::new(keymap);

//         // Basic match
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("a")?, vec![(1, context_a.clone())]),
//             KeyMatch::Matches(vec![(1, Box::new(A("x".to_string())))])
//         );
//         matcher.clear_pending();

//         // Multi-keystroke match
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("a")?, vec![(1, context_b.clone())]),
//             KeyMatch::Pending
//         );
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("b")?, vec![(1, context_b.clone())]),
//             KeyMatch::Matches(vec![(1, Box::new(Ab))])
//         );
//         matcher.clear_pending();

//         // Failed matches don't interfere with matching subsequent keys
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("x")?, vec![(1, context_a.clone())]),
//             KeyMatch::None
//         );
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("a")?, vec![(1, context_a.clone())]),
//             KeyMatch::Matches(vec![(1, Box::new(A("x".to_string())))])
//         );
//         matcher.clear_pending();

//         // Pending keystrokes are cleared when the context changes
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("a")?, vec![(1, context_b.clone())]),
//             KeyMatch::Pending
//         );
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("b")?, vec![(1, context_a.clone())]),
//             KeyMatch::None
//         );
//         matcher.clear_pending();

//         let mut context_c = ActionContext::default();
//         context_c.add_identifier("c");

//         // Pending keystrokes are maintained per-view
//         assert_eq!(
//             matcher.match_keystroke(
//                 Keystroke::parse("a")?,
//                 vec![(1, context_b.clone()), (2, context_c.clone())]
//             ),
//             KeyMatch::Pending
//         );
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("b")?, vec![(1, context_b.clone())]),
//             KeyMatch::Matches(vec![(1, Box::new(Ab))])
//         );

//         // handle Czech $ (option + 4 key)
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("alt-ç->$")?, vec![(1, context_a.clone())]),
//             KeyMatch::Matches(vec![(1, Box::new(Dollar))])
//         );

//         // handle Brazillian quote (quote key then space key)
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("space->\"")?, vec![(1, context_a.clone())]),
//             KeyMatch::Matches(vec![(1, Box::new(Quote))])
//         );

//         // handle ctrl+` on a brazillian keyboard
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("ctrl-->`")?, vec![(1, context_a.clone())]),
//             KeyMatch::Matches(vec![(1, Box::new(Backtick))])
//         );

//         // handle alt-s on a US keyboard
//         assert_eq!(
//             matcher.match_keystroke(Keystroke::parse("alt-s->ß")?, vec![(1, context_a.clone())]),
//             KeyMatch::Matches(vec![(1, Box::new(Ess))])
//         );

//         Ok(())
//     }
// }
