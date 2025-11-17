use std::{cell::RefCell, rc::Rc};

use crate::{
    Vim,
    insert::NormalBefore,
    motion::Motion,
    normal::InsertBefore,
    state::{Mode, Operator, RecordedSelection, ReplayableAction, VimGlobals},
};
use editor::Editor;
use gpui::{Action, App, Context, Window, actions};
use workspace::Workspace;

actions!(
    vim,
    [
        /// Repeats the last change.
        Repeat,
        /// Ends the repeat recording.
        EndRepeat,
        /// Toggles macro recording.
        ToggleRecord,
        /// Replays the last recorded macro.
        ReplayLastRecording
    ]
);

fn should_replay(action: &dyn Action) -> bool {
    // skip so that we don't leave the character palette open
    if editor::actions::ShowCharacterPalette.partial_eq(action) {
        return false;
    }
    true
}

fn repeatable_insert(action: &ReplayableAction) -> Option<Box<dyn Action>> {
    match action {
        ReplayableAction::Action(action) => {
            if super::InsertBefore.partial_eq(&**action)
                || super::InsertAfter.partial_eq(&**action)
                || super::InsertFirstNonWhitespace.partial_eq(&**action)
                || super::InsertEndOfLine.partial_eq(&**action)
            {
                Some(super::InsertBefore.boxed_clone())
            } else if super::InsertLineAbove.partial_eq(&**action)
                || super::InsertLineBelow.partial_eq(&**action)
            {
                Some(super::InsertLineBelow.boxed_clone())
            } else if crate::replace::ToggleReplace.partial_eq(&**action) {
                Some(crate::replace::ToggleReplace.boxed_clone())
            } else {
                None
            }
        }
        ReplayableAction::Insertion { .. } => None,
    }
}

pub(crate) fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, |vim, _: &EndRepeat, window, cx| {
        Vim::globals(cx).dot_replaying = false;
        vim.switch_mode(Mode::Normal, false, window, cx)
    });

    Vim::action(editor, cx, |vim, _: &Repeat, window, cx| {
        vim.repeat(false, window, cx)
    });

    Vim::action(editor, cx, |vim, _: &ToggleRecord, window, cx| {
        let globals = Vim::globals(cx);
        if let Some(char) = globals.recording_register.take() {
            globals.last_recorded_register = Some(char)
        } else {
            vim.push_operator(Operator::RecordRegister, window, cx);
        }
    });

    Vim::action(editor, cx, |vim, _: &ReplayLastRecording, window, cx| {
        let Some(register) = Vim::globals(cx).last_recorded_register else {
            return;
        };
        vim.replay_register(register, window, cx)
    });
}

pub struct ReplayerState {
    actions: Vec<ReplayableAction>,
    running: bool,
    ix: usize,
}

#[derive(Clone)]
pub struct Replayer(Rc<RefCell<ReplayerState>>);

impl Replayer {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(ReplayerState {
            actions: vec![],
            running: false,
            ix: 0,
        })))
    }

    pub fn replay(&mut self, actions: Vec<ReplayableAction>, window: &mut Window, cx: &mut App) {
        let mut lock = self.0.borrow_mut();
        let range = lock.ix..lock.ix;
        lock.actions.splice(range, actions);
        if lock.running {
            return;
        }
        lock.running = true;
        let this = self.clone();
        window.defer(cx, move |window, cx| {
            this.next(window, cx);
            let Some(Some(workspace)) = window.root::<Workspace>() else {
                return;
            };
            let Some(editor) = workspace
                .read(cx)
                .active_item(cx)
                .and_then(|item| item.act_as::<Editor>(cx))
            else {
                return;
            };
            editor.update(cx, |editor, cx| {
                editor
                    .buffer()
                    .update(cx, |multi, cx| multi.finalize_last_transaction(cx))
            });
        })
    }

    pub fn stop(self) {
        self.0.borrow_mut().actions.clear()
    }

    pub fn next(self, window: &mut Window, cx: &mut App) {
        let mut lock = self.0.borrow_mut();
        let action = if lock.ix < 10000 {
            lock.actions.get(lock.ix).cloned()
        } else {
            log::error!("Aborting replay after 10000 actions");
            None
        };
        lock.ix += 1;
        drop(lock);
        let Some(action) = action else {
            Vim::globals(cx).replayer.take();
            return;
        };
        match action {
            ReplayableAction::Action(action) => {
                if should_replay(&*action) {
                    window.dispatch_action(action.boxed_clone(), cx);
                    cx.defer(move |cx| Vim::globals(cx).observe_action(action.boxed_clone()));
                }
            }
            ReplayableAction::Insertion {
                text,
                utf16_range_to_replace,
            } => {
                let Some(Some(workspace)) = window.root::<Workspace>() else {
                    return;
                };
                let Some(editor) = workspace
                    .read(cx)
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
                else {
                    return;
                };
                editor.update(cx, |editor, cx| {
                    editor.replay_insert_event(&text, utf16_range_to_replace.clone(), window, cx)
                })
            }
        }
        window.defer(cx, move |window, cx| self.next(window, cx));
    }
}

impl Vim {
    pub(crate) fn record_register(
        &mut self,
        register: char,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let globals = Vim::globals(cx);
        globals.recording_register = Some(register);
        globals.recordings.remove(&register);
        globals.ignore_current_insertion = true;
        self.clear_operator(window, cx)
    }

    pub(crate) fn replay_register(
        &mut self,
        mut register: char,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut count = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);
        self.clear_operator(window, cx);

        let globals = Vim::globals(cx);
        if register == '@' {
            let Some(last) = globals.last_replayed_register else {
                return;
            };
            register = last;
        }
        let Some(actions) = globals.recordings.get(&register) else {
            return;
        };

        let mut repeated_actions = vec![];
        while count > 0 {
            repeated_actions.extend(actions.iter().cloned());
            count -= 1
        }

        globals.last_replayed_register = Some(register);
        let mut replayer = globals.replayer.get_or_insert_with(Replayer::new).clone();
        replayer.replay(repeated_actions, window, cx);
    }

    pub(crate) fn repeat(
        &mut self,
        from_insert_mode: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = Vim::take_count(cx);
        Vim::take_forced_motion(cx);

        let Some((mut actions, selection, mode)) = Vim::update_globals(cx, |globals, _| {
            let actions = globals.recorded_actions.clone();
            if actions.is_empty() {
                return None;
            }
            if globals.replayer.is_none()
                && let Some(recording_register) = globals.recording_register
            {
                globals
                    .recordings
                    .entry(recording_register)
                    .or_default()
                    .push(ReplayableAction::Action(Repeat.boxed_clone()));
            }

            let mut mode = None;
            let selection = globals.recorded_selection.clone();
            match selection {
                RecordedSelection::SingleLine { .. } | RecordedSelection::Visual { .. } => {
                    globals.recorded_count = None;
                    mode = Some(Mode::Visual);
                }
                RecordedSelection::VisualLine { .. } => {
                    globals.recorded_count = None;
                    mode = Some(Mode::VisualLine)
                }
                RecordedSelection::VisualBlock { .. } => {
                    globals.recorded_count = None;
                    mode = Some(Mode::VisualBlock)
                }
                RecordedSelection::None => {
                    if let Some(count) = count {
                        globals.recorded_count = Some(count);
                    }
                }
            }

            Some((actions, selection, mode))
        }) else {
            return;
        };
        if mode != Some(self.mode) {
            if let Some(mode) = mode {
                self.switch_mode(mode, false, window, cx)
            }

            match selection {
                RecordedSelection::SingleLine { cols } => {
                    if cols > 1 {
                        self.visual_motion(Motion::Right, Some(cols as usize - 1), window, cx)
                    }
                }
                RecordedSelection::Visual { rows, cols } => {
                    self.visual_motion(
                        Motion::Down {
                            display_lines: false,
                        },
                        Some(rows as usize),
                        window,
                        cx,
                    );
                    self.visual_motion(
                        Motion::StartOfLine {
                            display_lines: false,
                        },
                        None,
                        window,
                        cx,
                    );
                    if cols > 1 {
                        self.visual_motion(Motion::Right, Some(cols as usize - 1), window, cx)
                    }
                }
                RecordedSelection::VisualBlock { rows, cols } => {
                    self.visual_motion(
                        Motion::Down {
                            display_lines: false,
                        },
                        Some(rows as usize),
                        window,
                        cx,
                    );
                    if cols > 1 {
                        self.visual_motion(Motion::Right, Some(cols as usize - 1), window, cx);
                    }
                }
                RecordedSelection::VisualLine { rows } => {
                    self.visual_motion(
                        Motion::Down {
                            display_lines: false,
                        },
                        Some(rows as usize),
                        window,
                        cx,
                    );
                }
                RecordedSelection::None => {}
            }
        }

        // insert internally uses repeat to handle counts
        // vim doesn't treat 3a1 as though you literally repeated a1
        // 3 times, instead it inserts the content thrice at the insert position.
        if let Some(to_repeat) = repeatable_insert(&actions[0]) {
            if let Some(ReplayableAction::Action(action)) = actions.last()
                && NormalBefore.partial_eq(&**action)
            {
                actions.pop();
            }

            let mut new_actions = actions.clone();
            actions[0] = ReplayableAction::Action(to_repeat.boxed_clone());

            let mut count = cx.global::<VimGlobals>().recorded_count.unwrap_or(1);

            // if we came from insert mode we're just doing repetitions 2 onwards.
            if from_insert_mode {
                count -= 1;
                new_actions[0] = actions[0].clone();
            }

            for _ in 1..count {
                new_actions.append(actions.clone().as_mut());
            }
            new_actions.push(ReplayableAction::Action(NormalBefore.boxed_clone()));
            actions = new_actions;
        }

        actions.push(ReplayableAction::Action(EndRepeat.boxed_clone()));

        if self.temp_mode {
            self.temp_mode = false;
            actions.push(ReplayableAction::Action(InsertBefore.boxed_clone()));
        }

        let globals = Vim::globals(cx);
        globals.dot_replaying = true;
        let mut replayer = globals.replayer.get_or_insert_with(Replayer::new).clone();

        replayer.replay(actions, window, cx);
    }
}

#[cfg(test)]
mod test {
    use editor::test::editor_lsp_test_context::EditorLspTestContext;
    use futures::StreamExt;
    use indoc::indoc;

    use gpui::EntityInputHandler;

    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };

    #[gpui::test]
    async fn test_dot_repeat(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        // "o"
        cx.set_shared_state("ˇhello").await;
        cx.simulate_shared_keystrokes("o w o r l d escape").await;
        cx.shared_state().await.assert_eq("hello\nworlˇd");
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("hello\nworld\nworlˇd");

        // "d"
        cx.simulate_shared_keystrokes("^ d f o").await;
        cx.simulate_shared_keystrokes("g g .").await;
        cx.shared_state().await.assert_eq("ˇ\nworld\nrld");

        // "p" (note that it pastes the current clipboard)
        cx.simulate_shared_keystrokes("j y y p").await;
        cx.simulate_shared_keystrokes("shift-g y y .").await;
        cx.shared_state()
            .await
            .assert_eq("\nworld\nworld\nrld\nˇrld");

        // "~" (note that counts apply to the action taken, not . itself)
        cx.set_shared_state("ˇthe quick brown fox").await;
        cx.simulate_shared_keystrokes("2 ~ .").await;
        cx.set_shared_state("THE ˇquick brown fox").await;
        cx.simulate_shared_keystrokes("3 .").await;
        cx.set_shared_state("THE QUIˇck brown fox").await;
        cx.run_until_parked();
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("THE QUICK ˇbrown fox");
    }

    #[gpui::test]
    async fn test_repeat_ime(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("hˇllo", Mode::Normal);
        cx.simulate_keystrokes("i");

        // simulate brazilian input for ä.
        cx.update_editor(|editor, window, cx| {
            editor.replace_and_mark_text_in_range(None, "\"", Some(1..1), window, cx);
            editor.replace_text_in_range(None, "ä", window, cx);
        });
        cx.simulate_keystrokes("escape");
        cx.assert_state("hˇällo", Mode::Normal);
        cx.simulate_keystrokes(".");
        cx.assert_state("hˇäällo", Mode::Normal);
    }

    #[gpui::test]
    async fn test_repeat_completion(cx: &mut gpui::TestAppContext) {
        VimTestContext::init(cx);
        let cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    resolve_provider: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            },
            cx,
        )
        .await;
        let mut cx = VimTestContext::new_with_lsp(cx, true);

        cx.set_state(
            indoc! {"
            onˇe
            two
            three
        "},
            Mode::Normal,
        );

        let mut request = cx.set_request_handler::<lsp::request::Completion, _, _>(
            move |_, params, _| async move {
                let position = params.text_document_position.position;
                Ok(Some(lsp::CompletionResponse::Array(vec![
                    lsp::CompletionItem {
                        label: "first".to_string(),
                        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            range: lsp::Range::new(position, position),
                            new_text: "first".to_string(),
                        })),
                        ..Default::default()
                    },
                    lsp::CompletionItem {
                        label: "second".to_string(),
                        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            range: lsp::Range::new(position, position),
                            new_text: "second".to_string(),
                        })),
                        ..Default::default()
                    },
                ])))
            },
        );
        cx.simulate_keystrokes("a .");
        request.next().await;
        cx.condition(|editor, _| editor.context_menu_visible())
            .await;
        cx.simulate_keystrokes("down enter ! escape");

        cx.assert_state(
            indoc! {"
                one.secondˇ!
                two
                three
            "},
            Mode::Normal,
        );
        cx.simulate_keystrokes("j .");
        cx.assert_state(
            indoc! {"
                one.second!
                two.secondˇ!
                three
            "},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_repeat_completion_unicode_bug(cx: &mut gpui::TestAppContext) {
        VimTestContext::init(cx);
        let cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    resolve_provider: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            },
            cx,
        )
        .await;
        let mut cx = VimTestContext::new_with_lsp(cx, true);

        cx.set_state(
            indoc! {"
                ĩлˇк
                ĩлк
            "},
            Mode::Normal,
        );

        let mut request = cx.set_request_handler::<lsp::request::Completion, _, _>(
            move |_, params, _| async move {
                let position = params.text_document_position.position;
                let mut to_the_left = position;
                to_the_left.character -= 2;
                Ok(Some(lsp::CompletionResponse::Array(vec![
                    lsp::CompletionItem {
                        label: "oops".to_string(),
                        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            range: lsp::Range::new(to_the_left, position),
                            new_text: "к!".to_string(),
                        })),
                        ..Default::default()
                    },
                ])))
            },
        );
        cx.simulate_keystrokes("i .");
        request.next().await;
        cx.condition(|editor, _| editor.context_menu_visible())
            .await;
        cx.simulate_keystrokes("enter escape");
        cx.assert_state(
            indoc! {"
                ĩкˇ!к
                ĩлк
            "},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_repeat_visual(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        // single-line (3 columns)
        cx.set_shared_state(indoc! {
            "ˇthe quick brown
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes("v i w s o escape").await;
        cx.shared_state().await.assert_eq(indoc! {
            "ˇo quick brown
            fox jumps over
            the lazy dog"
        });
        cx.simulate_shared_keystrokes("j w .").await;
        cx.shared_state().await.assert_eq(indoc! {
            "o quick brown
            fox ˇops over
            the lazy dog"
        });
        cx.simulate_shared_keystrokes("f r .").await;
        cx.shared_state().await.assert_eq(indoc! {
            "o quick brown
            fox ops oveˇothe lazy dog"
        });

        // visual
        cx.set_shared_state(indoc! {
            "the ˇquick brown
            fox jumps over
            fox jumps over
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes("v j x").await;
        cx.shared_state().await.assert_eq(indoc! {
            "the ˇumps over
            fox jumps over
            fox jumps over
            the lazy dog"
        });
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq(indoc! {
            "the ˇumps over
            fox jumps over
            the lazy dog"
        });
        cx.simulate_shared_keystrokes("w .").await;
        cx.shared_state().await.assert_eq(indoc! {
            "the umps ˇumps over
            the lazy dog"
        });
        cx.simulate_shared_keystrokes("j .").await;
        cx.shared_state().await.assert_eq(indoc! {
            "the umps umps over
            the ˇog"
        });

        // block mode (3 rows)
        cx.set_shared_state(indoc! {
            "ˇthe quick brown
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes("ctrl-v j j shift-i o escape")
            .await;
        cx.shared_state().await.assert_eq(indoc! {
            "ˇothe quick brown
            ofox jumps over
            othe lazy dog"
        });
        cx.simulate_shared_keystrokes("j 4 l .").await;
        cx.shared_state().await.assert_eq(indoc! {
            "othe quick brown
            ofoxˇo jumps over
            otheo lazy dog"
        });

        // line mode
        cx.set_shared_state(indoc! {
            "ˇthe quick brown
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes("shift-v shift-r o escape")
            .await;
        cx.shared_state().await.assert_eq(indoc! {
            "ˇo
            fox jumps over
            the lazy dog"
        });
        cx.simulate_shared_keystrokes("j .").await;
        cx.shared_state().await.assert_eq(indoc! {
            "o
            ˇo
            the lazy dog"
        });
    }

    #[gpui::test]
    async fn test_repeat_motion_counts(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {
            "ˇthe quick brown
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes("3 d 3 l").await;
        cx.shared_state().await.assert_eq(indoc! {
            "ˇ brown
            fox jumps over
            the lazy dog"
        });
        cx.simulate_shared_keystrokes("j .").await;
        cx.shared_state().await.assert_eq(indoc! {
            " brown
            ˇ over
            the lazy dog"
        });
        cx.simulate_shared_keystrokes("j 2 .").await;
        cx.shared_state().await.assert_eq(indoc! {
            " brown
             over
            ˇe lazy dog"
        });
    }

    #[gpui::test]
    async fn test_record_interrupted(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("ˇhello\n", Mode::Normal);
        cx.simulate_keystrokes("4 i j cmd-shift-p escape");
        cx.simulate_keystrokes("escape");
        cx.assert_state("ˇjhello\n", Mode::Normal);
    }

    #[gpui::test]
    async fn test_repeat_over_blur(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello hello hello\n").await;
        cx.simulate_shared_keystrokes("c f o x escape").await;
        cx.shared_state().await.assert_eq("ˇx hello hello\n");
        cx.simulate_shared_keystrokes(": escape").await;
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("ˇx hello\n");
    }

    #[gpui::test]
    async fn test_undo_repeated_insert(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("hellˇo").await;
        cx.simulate_shared_keystrokes("3 a . escape").await;
        cx.shared_state().await.assert_eq("hello..ˇ.");
        cx.simulate_shared_keystrokes("u").await;
        cx.shared_state().await.assert_eq("hellˇo");
    }

    #[gpui::test]
    async fn test_record_replay(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello world").await;
        cx.simulate_shared_keystrokes("q w c w j escape q").await;
        cx.shared_state().await.assert_eq("ˇj world");
        cx.simulate_shared_keystrokes("2 l @ w").await;
        cx.shared_state().await.assert_eq("j ˇj");
    }

    #[gpui::test]
    async fn test_record_replay_count(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello world!!").await;
        cx.simulate_shared_keystrokes("q a v 3 l s 0 escape l q")
            .await;
        cx.shared_state().await.assert_eq("0ˇo world!!");
        cx.simulate_shared_keystrokes("2 @ a").await;
        cx.shared_state().await.assert_eq("000ˇ!");
    }

    #[gpui::test]
    async fn test_record_replay_dot(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello world").await;
        cx.simulate_shared_keystrokes("q a r a l r b l q").await;
        cx.shared_state().await.assert_eq("abˇllo world");
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("abˇblo world");
        cx.simulate_shared_keystrokes("shift-q").await;
        cx.shared_state().await.assert_eq("ababˇo world");
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("ababˇb world");
    }

    #[gpui::test]
    async fn test_record_replay_of_dot(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello world").await;
        cx.simulate_shared_keystrokes("r o q w . q").await;
        cx.shared_state().await.assert_eq("ˇoello world");
        cx.simulate_shared_keystrokes("d l").await;
        cx.shared_state().await.assert_eq("ˇello world");
        cx.simulate_shared_keystrokes("@ w").await;
        cx.shared_state().await.assert_eq("ˇllo world");
    }

    #[gpui::test]
    async fn test_record_replay_interleaved(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello world").await;
        cx.simulate_shared_keystrokes("q z r a l q").await;
        cx.shared_state().await.assert_eq("aˇello world");
        cx.simulate_shared_keystrokes("q b @ z @ z q").await;
        cx.shared_state().await.assert_eq("aaaˇlo world");
        cx.simulate_shared_keystrokes("@ @").await;
        cx.shared_state().await.assert_eq("aaaaˇo world");
        cx.simulate_shared_keystrokes("@ b").await;
        cx.shared_state().await.assert_eq("aaaaaaˇworld");
        cx.simulate_shared_keystrokes("@ @").await;
        cx.shared_state().await.assert_eq("aaaaaaaˇorld");
        cx.simulate_shared_keystrokes("q z r b l q").await;
        cx.shared_state().await.assert_eq("aaaaaaabˇrld");
        cx.simulate_shared_keystrokes("@ b").await;
        cx.shared_state().await.assert_eq("aaaaaaabbbˇd");
    }
}
