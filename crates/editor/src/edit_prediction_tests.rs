use edit_prediction_types::{
    EditPredictionDelegate, EditPredictionIconSet, PredictedCursorPosition,
};
use gpui::{
    Entity, KeyBinding, KeybindingKeystroke, Keystroke, Modifiers, NoAction, Task, prelude::*,
};
use indoc::indoc;
use language::EditPredictionsMode;
use language::{Buffer, CodeLabel};
use multi_buffer::{Anchor, ExcerptId, MultiBufferSnapshot, ToPoint};
use project::{Completion, CompletionResponse, CompletionSource};
use std::{
    ops::Range,
    rc::Rc,
    sync::{
        Arc,
        atomic::{self, AtomicUsize},
    },
};
use text::{Point, ToOffset};
use ui::prelude::*;

use crate::{
    AcceptEditPrediction, CompletionContext, CompletionProvider, EditPrediction,
    EditPredictionKeybindAction, EditPredictionKeybindSurface, MenuEditPredictionsPolicy,
    ShowCompletions,
    editor_tests::{init_test, update_test_language_settings},
    test::editor_test_context::EditorTestContext,
};
use rpc::proto::PeerId;
use workspace::CollaboratorId;

#[gpui::test]
async fn test_edit_prediction_insert(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let absolute_zero_celsius = ˇ;");

    propose_edits(&provider, vec![(28..28, "-273.15")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    assert_editor_active_edit_completion(&mut cx, |_, edits| {
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].1.as_ref(), "-273.15");
    });

    accept_completion(&mut cx);

    cx.assert_editor_state("let absolute_zero_celsius = -273.15ˇ;")
}

#[gpui::test]
async fn test_edit_prediction_cursor_position_inside_insertion(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {
        eprintln!("");
    });

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());

    assign_editor_completion_provider(provider.clone(), &mut cx);
    // Buffer: "fn foo() {}" - we'll insert text and position cursor inside the insertion
    cx.set_state("fn foo() ˇ{}");

    // Insert "bar()" at offset 9, with cursor at offset 2 within the insertion (after "ba")
    // This tests the case where cursor is inside newly inserted text
    propose_edits_with_cursor_position_in_insertion(
        &provider,
        vec![(9..9, "bar()")],
        9, // anchor at the insertion point
        2, // offset 2 within "bar()" puts cursor after "ba"
        &mut cx,
    );
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    assert_editor_active_edit_completion(&mut cx, |_, edits| {
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].1.as_ref(), "bar()");
    });

    accept_completion(&mut cx);

    // Cursor should be inside the inserted text at "baˇr()"
    cx.assert_editor_state("fn foo() baˇr(){}");
}

#[gpui::test]
async fn test_edit_prediction_cursor_position_outside_edit(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    // Buffer: "let x = ;" with cursor before semicolon - we'll insert "42" and position cursor elsewhere
    cx.set_state("let x = ˇ;");

    // Insert "42" at offset 8, but set cursor_position to offset 4 (the 'x')
    // This tests that cursor moves to the predicted position, not the end of the edit
    propose_edits_with_cursor_position(
        &provider,
        vec![(8..8, "42")],
        Some(4), // cursor at offset 4 (the 'x'), NOT at the edit location
        &mut cx,
    );
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    assert_editor_active_edit_completion(&mut cx, |_, edits| {
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].1.as_ref(), "42");
    });

    accept_completion(&mut cx);

    // Cursor should be at offset 4 (the 'x'), not at the end of the inserted "42"
    cx.assert_editor_state("let ˇx = 42;");
}

#[gpui::test]
async fn test_edit_prediction_cursor_position_fallback(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let x = ˇ;");

    // Propose an edit without a cursor position - should fall back to end of edit
    propose_edits(&provider, vec![(8..8, "42")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    accept_completion(&mut cx);

    // Cursor should be at the end of the inserted text (default behavior)
    cx.assert_editor_state("let x = 42ˇ;")
}

#[gpui::test]
async fn test_edit_prediction_modification(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let pi = ˇ\"foo\";");

    propose_edits(&provider, vec![(9..14, "3.14159")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    assert_editor_active_edit_completion(&mut cx, |_, edits| {
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].1.as_ref(), "3.14159");
    });

    accept_completion(&mut cx);

    cx.assert_editor_state("let pi = 3.14159ˇ;")
}

#[gpui::test]
async fn test_edit_prediction_jump_button(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);

    // Cursor is 2+ lines above the proposed edit
    cx.set_state(indoc! {"
        line 0
        line ˇ1
        line 2
        line 3
        line
    "});

    propose_edits(
        &provider,
        vec![(Point::new(4, 3)..Point::new(4, 3), " 4")],
        &mut cx,
    );

    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
    assert_editor_active_move_completion(&mut cx, |snapshot, move_target| {
        assert_eq!(move_target.to_point(&snapshot), Point::new(4, 3));
    });

    // When accepting, cursor is moved to the proposed location
    accept_completion(&mut cx);
    cx.assert_editor_state(indoc! {"
        line 0
        line 1
        line 2
        line 3
        linˇe
    "});

    // Cursor is 2+ lines below the proposed edit
    cx.set_state(indoc! {"
        line 0
        line
        line 2
        line 3
        line ˇ4
    "});

    propose_edits(
        &provider,
        vec![(Point::new(1, 3)..Point::new(1, 3), " 1")],
        &mut cx,
    );

    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
    assert_editor_active_move_completion(&mut cx, |snapshot, move_target| {
        assert_eq!(move_target.to_point(&snapshot), Point::new(1, 3));
    });

    // When accepting, cursor is moved to the proposed location
    accept_completion(&mut cx);
    cx.assert_editor_state(indoc! {"
        line 0
        linˇe
        line 2
        line 3
        line 4
    "});
}

#[gpui::test]
async fn test_edit_prediction_invalidation_range(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);

    // Cursor is 3+ lines above the proposed edit
    cx.set_state(indoc! {"
        line 0
        line ˇ1
        line 2
        line 3
        line 4
        line
    "});
    let edit_location = Point::new(5, 3);

    propose_edits(
        &provider,
        vec![(edit_location..edit_location, " 5")],
        &mut cx,
    );

    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
    assert_editor_active_move_completion(&mut cx, |snapshot, move_target| {
        assert_eq!(move_target.to_point(&snapshot), edit_location);
    });

    // If we move *towards* the completion, it stays active
    cx.set_selections_state(indoc! {"
        line 0
        line 1
        line ˇ2
        line 3
        line 4
        line
    "});
    assert_editor_active_move_completion(&mut cx, |snapshot, move_target| {
        assert_eq!(move_target.to_point(&snapshot), edit_location);
    });

    // If we move *away* from the completion, it is discarded
    cx.set_selections_state(indoc! {"
        line ˇ0
        line 1
        line 2
        line 3
        line 4
        line
    "});
    cx.editor(|editor, _, _| {
        assert!(editor.active_edit_prediction.is_none());
    });

    // Cursor is 3+ lines below the proposed edit
    cx.set_state(indoc! {"
        line
        line 1
        line 2
        line 3
        line ˇ4
        line 5
    "});
    let edit_location = Point::new(0, 3);

    propose_edits(
        &provider,
        vec![(edit_location..edit_location, " 0")],
        &mut cx,
    );

    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
    assert_editor_active_move_completion(&mut cx, |snapshot, move_target| {
        assert_eq!(move_target.to_point(&snapshot), edit_location);
    });

    // If we move *towards* the completion, it stays active
    cx.set_selections_state(indoc! {"
        line
        line 1
        line 2
        line ˇ3
        line 4
        line 5
    "});
    assert_editor_active_move_completion(&mut cx, |snapshot, move_target| {
        assert_eq!(move_target.to_point(&snapshot), edit_location);
    });

    // If we move *away* from the completion, it is discarded
    cx.set_selections_state(indoc! {"
        line
        line 1
        line 2
        line 3
        line 4
        line ˇ5
    "});
    cx.editor(|editor, _, _| {
        assert!(editor.active_edit_prediction.is_none());
    });
}

#[gpui::test]
async fn test_edit_prediction_jump_disabled_for_non_zed_providers(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeNonZedEditPredictionDelegate::default());
    assign_editor_completion_provider_non_zed(provider.clone(), &mut cx);

    // Cursor is 2+ lines above the proposed edit
    cx.set_state(indoc! {"
        line 0
        line ˇ1
        line 2
        line 3
        line
    "});

    propose_edits_non_zed(
        &provider,
        vec![(Point::new(4, 3)..Point::new(4, 3), " 4")],
        &mut cx,
    );

    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    // For non-Zed providers, there should be no move completion (jump functionality disabled)
    cx.editor(|editor, _, _| {
        if let Some(completion_state) = &editor.active_edit_prediction {
            // Should be an Edit prediction, not a Move prediction
            match &completion_state.completion {
                EditPrediction::Edit { .. } => {
                    // This is expected for non-Zed providers
                }
                EditPrediction::MoveWithin { .. } | EditPrediction::MoveOutside { .. } => {
                    panic!(
                        "Non-Zed providers should not show Move predictions (jump functionality)"
                    );
                }
            }
        }
    });
}

#[gpui::test]
async fn test_edit_prediction_refresh_suppressed_while_following(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let x = ˇ;");

    propose_edits(&provider, vec![(8..8, "42")], &mut cx);

    cx.update_editor(|editor, window, cx| {
        editor.refresh_edit_prediction(false, false, window, cx);
        editor.update_visible_edit_prediction(window, cx);
    });

    assert_eq!(
        provider.read_with(&cx.cx, |provider, _| {
            provider.refresh_count.load(atomic::Ordering::SeqCst)
        }),
        1
    );
    cx.editor(|editor, _, _| {
        assert!(editor.active_edit_prediction.is_some());
    });

    cx.update_editor(|editor, window, cx| {
        editor.leader_id = Some(CollaboratorId::PeerId(PeerId::default()));
        editor.refresh_edit_prediction(false, false, window, cx);
    });

    assert_eq!(
        provider.read_with(&cx.cx, |provider, _| {
            provider.refresh_count.load(atomic::Ordering::SeqCst)
        }),
        1
    );
    cx.editor(|editor, _, _| {
        assert!(editor.active_edit_prediction.is_none());
    });

    cx.update_editor(|editor, window, cx| {
        editor.leader_id = None;
        editor.refresh_edit_prediction(false, false, window, cx);
    });

    assert_eq!(
        provider.read_with(&cx.cx, |provider, _| {
            provider.refresh_count.load(atomic::Ordering::SeqCst)
        }),
        2
    );
}

#[gpui::test]
async fn test_edit_prediction_preview_cleanup_on_toggle_off(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    // Bind `ctrl-shift-a` to accept the provided edit prediction. The actual key
    // binding here doesn't matter, we simply need to confirm that holding the
    // binding's modifiers triggers the edit prediction preview.
    cx.update(|cx| cx.bind_keys([KeyBinding::new("ctrl-shift-a", AcceptEditPrediction, None)]));

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let x = ˇ;");

    propose_edits(&provider, vec![(8..8, "42")], &mut cx);
    cx.update_editor(|editor, window, cx| {
        editor.set_menu_edit_predictions_policy(MenuEditPredictionsPolicy::ByProvider);
        editor.update_visible_edit_prediction(window, cx)
    });

    cx.editor(|editor, _, _| {
        assert!(editor.has_active_edit_prediction());
    });

    // Simulate pressing the modifiers for `AcceptEditPrediction`, namely
    // `ctrl-shift`, so that we can confirm that the edit prediction preview is
    // activated.
    let modifiers = Modifiers::control_shift();
    cx.simulate_modifiers_change(modifiers);
    cx.run_until_parked();

    cx.editor(|editor, _, _| {
        assert!(editor.edit_prediction_preview_is_active());
    });

    // Disable showing edit predictions without issuing a new modifiers changed
    // event, to confirm that the edit prediction preview is still active.
    cx.update_editor(|editor, window, cx| {
        editor.set_show_edit_predictions(Some(false), window, cx);
    });

    cx.editor(|editor, _, _| {
        assert!(!editor.has_active_edit_prediction());
        assert!(editor.edit_prediction_preview_is_active());
    });

    // Now release the modifiers
    // Simulate releasing all modifiers, ensuring that even with edit prediction
    // disabled, the edit prediction preview is cleaned up.
    cx.simulate_modifiers_change(Modifiers::none());
    cx.run_until_parked();

    cx.editor(|editor, _, _| {
        assert!(!editor.edit_prediction_preview_is_active());
    });
}

#[gpui::test]
async fn test_edit_prediction_preview_activates_when_prediction_arrives_with_modifier_held(
    cx: &mut gpui::TestAppContext,
) {
    init_test(cx, |_| {});
    load_default_keymap(cx);
    update_test_language_settings(cx, &|settings| {
        settings.edit_predictions.get_or_insert_default().mode = Some(EditPredictionsMode::Subtle);
    });

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let x = ˇ;");

    cx.editor(|editor, _, _| {
        assert!(!editor.has_active_edit_prediction());
        assert!(!editor.edit_prediction_preview_is_active());
    });

    let preview_modifiers = cx.update_editor(|editor, window, cx| {
        *editor
            .preview_edit_prediction_keystroke(window, cx)
            .unwrap()
            .modifiers()
    });

    cx.simulate_modifiers_change(preview_modifiers);
    cx.run_until_parked();

    cx.editor(|editor, _, _| {
        assert!(!editor.has_active_edit_prediction());
        assert!(editor.edit_prediction_preview_is_active());
    });

    propose_edits(&provider, vec![(8..8, "42")], &mut cx);
    cx.update_editor(|editor, window, cx| {
        editor.set_menu_edit_predictions_policy(MenuEditPredictionsPolicy::ByProvider);
        editor.update_visible_edit_prediction(window, cx)
    });

    cx.editor(|editor, _, _| {
        assert!(editor.has_active_edit_prediction());
        assert!(
            editor.edit_prediction_preview_is_active(),
            "prediction preview should activate immediately when the prediction arrives while the preview modifier is still held",
        );
    });
}

fn load_default_keymap(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        cx.bind_keys(
            settings::KeymapFile::load_asset_allow_partial_failure(
                settings::DEFAULT_KEYMAP_PATH,
                cx,
            )
            .expect("failed to load default keymap"),
        );
    });
}

#[gpui::test]
async fn test_inline_edit_prediction_keybind_selection_cases(cx: &mut gpui::TestAppContext) {
    enum InlineKeybindState {
        Normal,
        ShowingCompletions,
        InLeadingWhitespace,
        ShowingCompletionsAndLeadingWhitespace,
    }

    enum ExpectedKeystroke {
        DefaultAccept,
        DefaultPreview,
        Literal(&'static str),
    }

    struct InlineKeybindCase {
        name: &'static str,
        use_default_keymap: bool,
        mode: EditPredictionsMode,
        extra_bindings: Vec<KeyBinding>,
        state: InlineKeybindState,
        expected_accept_keystroke: ExpectedKeystroke,
        expected_preview_keystroke: ExpectedKeystroke,
        expected_displayed_keystroke: ExpectedKeystroke,
    }

    init_test(cx, |_| {});
    load_default_keymap(cx);
    let mut default_cx = EditorTestContext::new(cx).await;
    let provider = default_cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut default_cx);
    default_cx.set_state("let x = ˇ;");
    propose_edits(&provider, vec![(8..8, "42")], &mut default_cx);
    default_cx
        .update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    let (default_accept_keystroke, default_preview_keystroke) =
        default_cx.update_editor(|editor, window, cx| {
            let keybind_display = editor.edit_prediction_keybind_display(
                EditPredictionKeybindSurface::Inline,
                window,
                cx,
            );
            let accept_keystroke = keybind_display
                .accept_keystroke
                .as_ref()
                .expect("default inline edit prediction should have an accept binding")
                .clone();
            let preview_keystroke = keybind_display
                .preview_keystroke
                .as_ref()
                .expect("default inline edit prediction should have a preview binding")
                .clone();
            (accept_keystroke, preview_keystroke)
        });

    let cases = [
        InlineKeybindCase {
            name: "default setup prefers tab over alt-tab for accept",
            use_default_keymap: true,
            mode: EditPredictionsMode::Eager,
            extra_bindings: Vec::new(),
            state: InlineKeybindState::Normal,
            expected_accept_keystroke: ExpectedKeystroke::DefaultAccept,
            expected_preview_keystroke: ExpectedKeystroke::DefaultPreview,
            expected_displayed_keystroke: ExpectedKeystroke::DefaultAccept,
        },
        InlineKeybindCase {
            name: "subtle mode displays preview binding inline",
            use_default_keymap: true,
            mode: EditPredictionsMode::Subtle,
            extra_bindings: Vec::new(),
            state: InlineKeybindState::Normal,
            expected_accept_keystroke: ExpectedKeystroke::DefaultPreview,
            expected_preview_keystroke: ExpectedKeystroke::DefaultPreview,
            expected_displayed_keystroke: ExpectedKeystroke::DefaultPreview,
        },
        InlineKeybindCase {
            name: "removing default tab binding still displays tab",
            use_default_keymap: true,
            mode: EditPredictionsMode::Eager,
            extra_bindings: vec![KeyBinding::new(
                "tab",
                NoAction,
                Some("Editor && edit_prediction && edit_prediction_mode == eager"),
            )],
            state: InlineKeybindState::Normal,
            expected_accept_keystroke: ExpectedKeystroke::DefaultPreview,
            expected_preview_keystroke: ExpectedKeystroke::DefaultPreview,
            expected_displayed_keystroke: ExpectedKeystroke::DefaultPreview,
        },
        InlineKeybindCase {
            name: "custom-only rebound accept key uses replacement key",
            use_default_keymap: true,
            mode: EditPredictionsMode::Eager,
            extra_bindings: vec![KeyBinding::new(
                "ctrl-enter",
                AcceptEditPrediction,
                Some("Editor && edit_prediction"),
            )],
            state: InlineKeybindState::Normal,
            expected_accept_keystroke: ExpectedKeystroke::Literal("ctrl-enter"),
            expected_preview_keystroke: ExpectedKeystroke::Literal("ctrl-enter"),
            expected_displayed_keystroke: ExpectedKeystroke::Literal("ctrl-enter"),
        },
        InlineKeybindCase {
            name: "showing completions restores conflict-context binding",
            use_default_keymap: true,
            mode: EditPredictionsMode::Eager,
            extra_bindings: vec![KeyBinding::new(
                "ctrl-enter",
                AcceptEditPrediction,
                Some("Editor && edit_prediction && showing_completions"),
            )],
            state: InlineKeybindState::ShowingCompletions,
            expected_accept_keystroke: ExpectedKeystroke::Literal("ctrl-enter"),
            expected_preview_keystroke: ExpectedKeystroke::Literal("ctrl-enter"),
            expected_displayed_keystroke: ExpectedKeystroke::Literal("ctrl-enter"),
        },
        InlineKeybindCase {
            name: "leading whitespace restores conflict-context binding",
            use_default_keymap: false,
            mode: EditPredictionsMode::Eager,
            extra_bindings: vec![KeyBinding::new(
                "ctrl-enter",
                AcceptEditPrediction,
                Some("Editor && edit_prediction && in_leading_whitespace"),
            )],
            state: InlineKeybindState::InLeadingWhitespace,
            expected_accept_keystroke: ExpectedKeystroke::Literal("ctrl-enter"),
            expected_preview_keystroke: ExpectedKeystroke::Literal("ctrl-enter"),
            expected_displayed_keystroke: ExpectedKeystroke::Literal("ctrl-enter"),
        },
        InlineKeybindCase {
            name: "showing completions and leading whitespace restore combined conflict binding",
            use_default_keymap: false,
            mode: EditPredictionsMode::Eager,
            extra_bindings: vec![KeyBinding::new(
                "ctrl-enter",
                AcceptEditPrediction,
                Some("Editor && edit_prediction && showing_completions && in_leading_whitespace"),
            )],
            state: InlineKeybindState::ShowingCompletionsAndLeadingWhitespace,
            expected_accept_keystroke: ExpectedKeystroke::Literal("ctrl-enter"),
            expected_preview_keystroke: ExpectedKeystroke::Literal("ctrl-enter"),
            expected_displayed_keystroke: ExpectedKeystroke::Literal("ctrl-enter"),
        },
    ];

    for case in cases {
        init_test(cx, |_| {});
        if case.use_default_keymap {
            load_default_keymap(cx);
        }
        update_test_language_settings(cx, &|settings| {
            settings.edit_predictions.get_or_insert_default().mode = Some(case.mode);
        });

        if !case.extra_bindings.is_empty() {
            cx.update(|cx| cx.bind_keys(case.extra_bindings.clone()));
        }

        let mut cx = EditorTestContext::new(cx).await;
        let provider = cx.new(|_| FakeEditPredictionDelegate::default());
        assign_editor_completion_provider(provider.clone(), &mut cx);

        match case.state {
            InlineKeybindState::Normal | InlineKeybindState::ShowingCompletions => {
                cx.set_state("let x = ˇ;");
            }
            InlineKeybindState::InLeadingWhitespace
            | InlineKeybindState::ShowingCompletionsAndLeadingWhitespace => {
                cx.set_state(indoc! {"
                    fn main() {
                        ˇ
                    }
                "});
            }
        }

        propose_edits(&provider, vec![(8..8, "42")], &mut cx);
        cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

        if matches!(
            case.state,
            InlineKeybindState::ShowingCompletions
                | InlineKeybindState::ShowingCompletionsAndLeadingWhitespace
        ) {
            assign_editor_completion_menu_provider(&mut cx);
            cx.update_editor(|editor, window, cx| {
                editor.show_completions(&ShowCompletions, window, cx);
            });
            cx.run_until_parked();
        }

        cx.update_editor(|editor, window, cx| {
            assert!(
                editor.has_active_edit_prediction(),
                "case '{}' should have an active edit prediction",
                case.name
            );

            let keybind_display = editor.edit_prediction_keybind_display(
                EditPredictionKeybindSurface::Inline,
                window,
                cx,
            );
            let accept_keystroke = keybind_display
                .accept_keystroke
                .as_ref()
                .unwrap_or_else(|| panic!("case '{}' should have an accept binding", case.name));
            let preview_keystroke = keybind_display
                .preview_keystroke
                .as_ref()
                .unwrap_or_else(|| panic!("case '{}' should have a preview binding", case.name));
            let displayed_keystroke = keybind_display
                .displayed_keystroke
                .as_ref()
                .unwrap_or_else(|| panic!("case '{}' should have a displayed binding", case.name));

            let expected_accept_keystroke = match case.expected_accept_keystroke {
                ExpectedKeystroke::DefaultAccept => default_accept_keystroke.clone(),
                ExpectedKeystroke::DefaultPreview => default_preview_keystroke.clone(),
                ExpectedKeystroke::Literal(keystroke) => KeybindingKeystroke::from_keystroke(
                    Keystroke::parse(keystroke).expect("expected test keystroke to parse"),
                ),
            };
            let expected_preview_keystroke = match case.expected_preview_keystroke {
                ExpectedKeystroke::DefaultAccept => default_accept_keystroke.clone(),
                ExpectedKeystroke::DefaultPreview => default_preview_keystroke.clone(),
                ExpectedKeystroke::Literal(keystroke) => KeybindingKeystroke::from_keystroke(
                    Keystroke::parse(keystroke).expect("expected test keystroke to parse"),
                ),
            };
            let expected_displayed_keystroke = match case.expected_displayed_keystroke {
                ExpectedKeystroke::DefaultAccept => default_accept_keystroke.clone(),
                ExpectedKeystroke::DefaultPreview => default_preview_keystroke.clone(),
                ExpectedKeystroke::Literal(keystroke) => KeybindingKeystroke::from_keystroke(
                    Keystroke::parse(keystroke).expect("expected test keystroke to parse"),
                ),
            };

            assert_eq!(
                accept_keystroke, &expected_accept_keystroke,
                "case '{}' selected the wrong accept binding",
                case.name
            );
            assert_eq!(
                preview_keystroke, &expected_preview_keystroke,
                "case '{}' selected the wrong preview binding",
                case.name
            );
            assert_eq!(
                displayed_keystroke, &expected_displayed_keystroke,
                "case '{}' selected the wrong displayed binding",
                case.name
            );

            if matches!(case.mode, EditPredictionsMode::Subtle) {
                assert!(
                    editor.edit_prediction_requires_modifier(),
                    "case '{}' should require a modifier",
                    case.name
                );
            }
        });
    }
}

#[gpui::test]
async fn test_tab_accepts_edit_prediction_over_completion(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    load_default_keymap(cx);

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let x = ˇ;");

    propose_edits(&provider, vec![(8..8, "42")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    assert_editor_active_edit_completion(&mut cx, |_, edits| {
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].1.as_ref(), "42");
    });

    cx.simulate_keystroke("tab");
    cx.run_until_parked();

    cx.assert_editor_state("let x = 42ˇ;");
}

#[gpui::test]
async fn test_cursor_popover_edit_prediction_keybind_cases(cx: &mut gpui::TestAppContext) {
    enum CursorPopoverPredictionKind {
        SingleLine,
        MultiLine,
        SingleLineWithPreview,
        MultiLineWithPreview,
        DeleteSingleNewline,
        StaleSingleLineAfterMultiLine,
    }

    struct CursorPopoverCase {
        name: &'static str,
        prediction_kind: CursorPopoverPredictionKind,
        expected_action: EditPredictionKeybindAction,
    }

    let cases = [
        CursorPopoverCase {
            name: "single line prediction uses accept action",
            prediction_kind: CursorPopoverPredictionKind::SingleLine,
            expected_action: EditPredictionKeybindAction::Accept,
        },
        CursorPopoverCase {
            name: "multi line prediction uses preview action",
            prediction_kind: CursorPopoverPredictionKind::MultiLine,
            expected_action: EditPredictionKeybindAction::Preview,
        },
        CursorPopoverCase {
            name: "single line prediction with preview still uses accept action",
            prediction_kind: CursorPopoverPredictionKind::SingleLineWithPreview,
            expected_action: EditPredictionKeybindAction::Accept,
        },
        CursorPopoverCase {
            name: "multi line prediction with preview uses preview action",
            prediction_kind: CursorPopoverPredictionKind::MultiLineWithPreview,
            expected_action: EditPredictionKeybindAction::Preview,
        },
        CursorPopoverCase {
            name: "single line newline deletion uses accept action",
            prediction_kind: CursorPopoverPredictionKind::DeleteSingleNewline,
            expected_action: EditPredictionKeybindAction::Accept,
        },
        CursorPopoverCase {
            name: "stale multi line prediction does not force preview action",
            prediction_kind: CursorPopoverPredictionKind::StaleSingleLineAfterMultiLine,
            expected_action: EditPredictionKeybindAction::Accept,
        },
    ];

    for case in cases {
        init_test(cx, |_| {});
        load_default_keymap(cx);

        let mut cx = EditorTestContext::new(cx).await;
        let provider = cx.new(|_| FakeEditPredictionDelegate::default());
        assign_editor_completion_provider(provider.clone(), &mut cx);

        match case.prediction_kind {
            CursorPopoverPredictionKind::SingleLine => {
                cx.set_state("let x = ˇ;");
                propose_edits(&provider, vec![(8..8, "42")], &mut cx);
                cx.update_editor(|editor, window, cx| {
                    editor.update_visible_edit_prediction(window, cx)
                });
            }
            CursorPopoverPredictionKind::MultiLine => {
                cx.set_state("let x = ˇ;");
                propose_edits(&provider, vec![(8..8, "42\n43")], &mut cx);
                cx.update_editor(|editor, window, cx| {
                    editor.update_visible_edit_prediction(window, cx)
                });
            }
            CursorPopoverPredictionKind::SingleLineWithPreview => {
                cx.set_state("let x = ˇ;");
                propose_edits_with_preview(&provider, vec![(8..8, "42")], &mut cx).await;
                cx.update_editor(|editor, window, cx| {
                    editor.update_visible_edit_prediction(window, cx)
                });
            }
            CursorPopoverPredictionKind::MultiLineWithPreview => {
                cx.set_state("let x = ˇ;");
                propose_edits_with_preview(&provider, vec![(8..8, "42\n43")], &mut cx).await;
                cx.update_editor(|editor, window, cx| {
                    editor.update_visible_edit_prediction(window, cx)
                });
            }
            CursorPopoverPredictionKind::DeleteSingleNewline => {
                cx.set_state(indoc! {"
                    fn main() {
                        let value = 1;
                        ˇprintln!(\"done\");
                    }
                "});
                propose_edits(
                    &provider,
                    vec![(Point::new(1, 18)..Point::new(2, 17), "")],
                    &mut cx,
                );
                cx.update_editor(|editor, window, cx| {
                    editor.update_visible_edit_prediction(window, cx)
                });
            }
            CursorPopoverPredictionKind::StaleSingleLineAfterMultiLine => {
                cx.set_state("let x = ˇ;");
                propose_edits(&provider, vec![(8..8, "42\n43")], &mut cx);
                cx.update_editor(|editor, window, cx| {
                    editor.update_visible_edit_prediction(window, cx)
                });
                cx.update_editor(|editor, _window, cx| {
                    assert!(editor.active_edit_prediction.is_some());
                    assert!(editor.stale_edit_prediction_in_menu.is_none());
                    editor.take_active_edit_prediction(cx);
                    assert!(editor.active_edit_prediction.is_none());
                    assert!(editor.stale_edit_prediction_in_menu.is_some());
                });

                propose_edits(&provider, vec![(8..8, "42")], &mut cx);
                cx.update_editor(|editor, window, cx| {
                    editor.update_visible_edit_prediction(window, cx)
                });
            }
        }

        cx.update_editor(|editor, window, cx| {
            assert!(
                editor.has_active_edit_prediction(),
                "case '{}' should have an active edit prediction",
                case.name
            );

            let keybind_display = editor.edit_prediction_keybind_display(
                EditPredictionKeybindSurface::CursorPopoverExpanded,
                window,
                cx,
            );
            let accept_keystroke = keybind_display
                .accept_keystroke
                .as_ref()
                .unwrap_or_else(|| panic!("case '{}' should have an accept binding", case.name));
            let preview_keystroke = keybind_display
                .preview_keystroke
                .as_ref()
                .unwrap_or_else(|| panic!("case '{}' should have a preview binding", case.name));

            assert_eq!(
                keybind_display.action, case.expected_action,
                "case '{}' selected the wrong cursor popover action",
                case.name
            );
            assert_eq!(
                accept_keystroke.key(),
                "tab",
                "case '{}' selected the wrong accept binding",
                case.name
            );
            assert!(
                preview_keystroke.modifiers().modified(),
                "case '{}' should use a modified preview binding",
                case.name
            );

            if matches!(
                case.prediction_kind,
                CursorPopoverPredictionKind::StaleSingleLineAfterMultiLine
            ) {
                assert!(
                    editor.stale_edit_prediction_in_menu.is_none(),
                    "case '{}' should clear stale menu state",
                    case.name
                );
            }
        });
    }
}

fn assert_editor_active_edit_completion(
    cx: &mut EditorTestContext,
    assert: impl FnOnce(MultiBufferSnapshot, &Vec<(Range<Anchor>, Arc<str>)>),
) {
    cx.editor(|editor, _, cx| {
        let completion_state = editor
            .active_edit_prediction
            .as_ref()
            .expect("editor has no active completion");

        if let EditPrediction::Edit { edits, .. } = &completion_state.completion {
            assert(editor.buffer().read(cx).snapshot(cx), edits);
        } else {
            panic!("expected edit completion");
        }
    })
}

fn assert_editor_active_move_completion(
    cx: &mut EditorTestContext,
    assert: impl FnOnce(MultiBufferSnapshot, Anchor),
) {
    cx.editor(|editor, _, cx| {
        let completion_state = editor
            .active_edit_prediction
            .as_ref()
            .expect("editor has no active completion");

        if let EditPrediction::MoveWithin { target, .. } = &completion_state.completion {
            assert(editor.buffer().read(cx).snapshot(cx), *target);
        } else {
            panic!("expected move completion");
        }
    })
}

fn accept_completion(cx: &mut EditorTestContext) {
    cx.update_editor(|editor, window, cx| {
        editor.accept_edit_prediction(&crate::AcceptEditPrediction, window, cx)
    })
}

fn propose_edits<T: ToOffset>(
    provider: &Entity<FakeEditPredictionDelegate>,
    edits: Vec<(Range<T>, &str)>,
    cx: &mut EditorTestContext,
) {
    propose_edits_with_cursor_position(provider, edits, None, cx);
}

async fn propose_edits_with_preview<T: ToOffset + Clone>(
    provider: &Entity<FakeEditPredictionDelegate>,
    edits: Vec<(Range<T>, &str)>,
    cx: &mut EditorTestContext,
) {
    let snapshot = cx.buffer_snapshot();
    let edits = edits
        .into_iter()
        .map(|(range, text)| {
            let anchor_range =
                snapshot.anchor_after(range.start.clone())..snapshot.anchor_before(range.end);
            (anchor_range, Arc::<str>::from(text))
        })
        .collect::<Vec<_>>();

    let preview_edits = edits
        .iter()
        .map(|(range, text)| (range.clone(), text.clone()))
        .collect::<Arc<[_]>>();

    let edit_preview = cx
        .buffer(|buffer: &Buffer, app| buffer.preview_edits(preview_edits, app))
        .await;

    let provider_edits = edits.into_iter().collect();

    cx.update(|_, cx| {
        provider.update(cx, |provider, _| {
            provider.set_edit_prediction(Some(edit_prediction_types::EditPrediction::Local {
                id: None,
                edits: provider_edits,
                cursor_position: None,
                edit_preview: Some(edit_preview),
            }))
        })
    });
}

fn propose_edits_with_cursor_position<T: ToOffset>(
    provider: &Entity<FakeEditPredictionDelegate>,
    edits: Vec<(Range<T>, &str)>,
    cursor_offset: Option<usize>,
    cx: &mut EditorTestContext,
) {
    let snapshot = cx.buffer_snapshot();
    let cursor_position = cursor_offset
        .map(|offset| PredictedCursorPosition::at_anchor(snapshot.anchor_after(offset)));
    let edits = edits.into_iter().map(|(range, text)| {
        let range = snapshot.anchor_after(range.start)..snapshot.anchor_before(range.end);
        (range, text.into())
    });

    cx.update(|_, cx| {
        provider.update(cx, |provider, _| {
            provider.set_edit_prediction(Some(edit_prediction_types::EditPrediction::Local {
                id: None,
                edits: edits.collect(),
                cursor_position,
                edit_preview: None,
            }))
        })
    });
}

fn propose_edits_with_cursor_position_in_insertion<T: ToOffset>(
    provider: &Entity<FakeEditPredictionDelegate>,
    edits: Vec<(Range<T>, &str)>,
    anchor_offset: usize,
    offset_within_insertion: usize,
    cx: &mut EditorTestContext,
) {
    let snapshot = cx.buffer_snapshot();
    // Use anchor_before (left bias) so the anchor stays at the insertion point
    // rather than moving past the inserted text
    let cursor_position = Some(PredictedCursorPosition::new(
        snapshot.anchor_before(anchor_offset),
        offset_within_insertion,
    ));
    let edits = edits.into_iter().map(|(range, text)| {
        let range = snapshot.anchor_after(range.start)..snapshot.anchor_before(range.end);
        (range, text.into())
    });

    cx.update(|_, cx| {
        provider.update(cx, |provider, _| {
            provider.set_edit_prediction(Some(edit_prediction_types::EditPrediction::Local {
                id: None,
                edits: edits.collect(),
                cursor_position,
                edit_preview: None,
            }))
        })
    });
}

fn assign_editor_completion_provider(
    provider: Entity<FakeEditPredictionDelegate>,
    cx: &mut EditorTestContext,
) {
    cx.update_editor(|editor, window, cx| {
        editor.set_edit_prediction_provider(Some(provider), window, cx);
    })
}

fn assign_editor_completion_menu_provider(cx: &mut EditorTestContext) {
    cx.update_editor(|editor, _, _| {
        editor.set_completion_provider(Some(Rc::new(FakeCompletionMenuProvider)));
    });
}

fn propose_edits_non_zed<T: ToOffset>(
    provider: &Entity<FakeNonZedEditPredictionDelegate>,
    edits: Vec<(Range<T>, &str)>,
    cx: &mut EditorTestContext,
) {
    let snapshot = cx.buffer_snapshot();
    let edits = edits.into_iter().map(|(range, text)| {
        let range = snapshot.anchor_after(range.start)..snapshot.anchor_before(range.end);
        (range, text.into())
    });

    cx.update(|_, cx| {
        provider.update(cx, |provider, _| {
            provider.set_edit_prediction(Some(edit_prediction_types::EditPrediction::Local {
                id: None,
                edits: edits.collect(),
                cursor_position: None,
                edit_preview: None,
            }))
        })
    });
}

fn assign_editor_completion_provider_non_zed(
    provider: Entity<FakeNonZedEditPredictionDelegate>,
    cx: &mut EditorTestContext,
) {
    cx.update_editor(|editor, window, cx| {
        editor.set_edit_prediction_provider(Some(provider), window, cx);
    })
}

struct FakeCompletionMenuProvider;

impl CompletionProvider for FakeCompletionMenuProvider {
    fn completions(
        &self,
        _excerpt_id: ExcerptId,
        _buffer: &Entity<Buffer>,
        _buffer_position: text::Anchor,
        _trigger: CompletionContext,
        _window: &mut Window,
        _cx: &mut Context<crate::Editor>,
    ) -> Task<anyhow::Result<Vec<CompletionResponse>>> {
        let completion = Completion {
            replace_range: text::Anchor::MIN..text::Anchor::MAX,
            new_text: "fake_completion".to_string(),
            label: CodeLabel::plain("fake_completion".to_string(), None),
            documentation: None,
            source: CompletionSource::Custom,
            icon_path: None,
            match_start: None,
            snippet_deduplication_key: None,
            insert_text_mode: None,
            confirm: None,
        };

        Task::ready(Ok(vec![CompletionResponse {
            completions: vec![completion],
            display_options: Default::default(),
            is_incomplete: false,
        }]))
    }

    fn is_completion_trigger(
        &self,
        _buffer: &Entity<Buffer>,
        _position: language::Anchor,
        _text: &str,
        _trigger_in_words: bool,
        _cx: &mut Context<crate::Editor>,
    ) -> bool {
        false
    }

    fn filter_completions(&self) -> bool {
        false
    }
}

#[derive(Default, Clone)]
pub struct FakeEditPredictionDelegate {
    pub completion: Option<edit_prediction_types::EditPrediction>,
    pub refresh_count: Arc<AtomicUsize>,
}

impl FakeEditPredictionDelegate {
    pub fn set_edit_prediction(
        &mut self,
        completion: Option<edit_prediction_types::EditPrediction>,
    ) {
        self.completion = completion;
    }
}

impl EditPredictionDelegate for FakeEditPredictionDelegate {
    fn name() -> &'static str {
        "fake-completion-provider"
    }

    fn display_name() -> &'static str {
        "Fake Completion Provider"
    }

    fn show_predictions_in_menu() -> bool {
        true
    }

    fn supports_jump_to_edit() -> bool {
        true
    }

    fn icons(&self, _cx: &gpui::App) -> EditPredictionIconSet {
        EditPredictionIconSet::new(IconName::ZedPredict)
    }

    fn is_enabled(
        &self,
        _buffer: &gpui::Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &gpui::App,
    ) -> bool {
        true
    }

    fn is_refreshing(&self, _cx: &gpui::App) -> bool {
        false
    }

    fn refresh(
        &mut self,
        _buffer: gpui::Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _debounce: bool,
        _cx: &mut gpui::Context<Self>,
    ) {
        self.refresh_count.fetch_add(1, atomic::Ordering::SeqCst);
    }

    fn accept(&mut self, _cx: &mut gpui::Context<Self>) {}

    fn discard(
        &mut self,
        _reason: edit_prediction_types::EditPredictionDiscardReason,
        _cx: &mut gpui::Context<Self>,
    ) {
    }

    fn suggest<'a>(
        &mut self,
        _buffer: &gpui::Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &mut gpui::Context<Self>,
    ) -> Option<edit_prediction_types::EditPrediction> {
        self.completion.clone()
    }
}

#[derive(Default, Clone)]
pub struct FakeNonZedEditPredictionDelegate {
    pub completion: Option<edit_prediction_types::EditPrediction>,
}

impl FakeNonZedEditPredictionDelegate {
    pub fn set_edit_prediction(
        &mut self,
        completion: Option<edit_prediction_types::EditPrediction>,
    ) {
        self.completion = completion;
    }
}

impl EditPredictionDelegate for FakeNonZedEditPredictionDelegate {
    fn name() -> &'static str {
        "fake-non-zed-provider"
    }

    fn display_name() -> &'static str {
        "Fake Non-Zed Provider"
    }

    fn show_predictions_in_menu() -> bool {
        false
    }

    fn supports_jump_to_edit() -> bool {
        false
    }

    fn icons(&self, _cx: &gpui::App) -> EditPredictionIconSet {
        EditPredictionIconSet::new(IconName::ZedPredict)
    }

    fn is_enabled(
        &self,
        _buffer: &gpui::Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &gpui::App,
    ) -> bool {
        true
    }

    fn is_refreshing(&self, _cx: &gpui::App) -> bool {
        false
    }

    fn refresh(
        &mut self,
        _buffer: gpui::Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _debounce: bool,
        _cx: &mut gpui::Context<Self>,
    ) {
    }

    fn accept(&mut self, _cx: &mut gpui::Context<Self>) {}

    fn discard(
        &mut self,
        _reason: edit_prediction_types::EditPredictionDiscardReason,
        _cx: &mut gpui::Context<Self>,
    ) {
    }

    fn suggest<'a>(
        &mut self,
        _buffer: &gpui::Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &mut gpui::Context<Self>,
    ) -> Option<edit_prediction_types::EditPrediction> {
        self.completion.clone()
    }
}
