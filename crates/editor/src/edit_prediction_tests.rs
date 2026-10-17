use edit_prediction_types::{
    EditPredictionDelegate, EditPredictionIconSet, EditPredictionRequestTrigger,
    PredictedCursorPosition,
};
use futures::StreamExt;
use gpui::{
    Capslock, ClipboardItem, Entity, Focusable, KeyBinding, KeybindingKeystroke, Keystroke,
    Modifiers, ModifiersChangedEvent, NoAction, Pixels, PlatformInput, Task, prelude::*, size,
};
use indoc::indoc;
use language::EditPredictionsMode;
use language::language_settings::SoftWrap;
use language::{Buffer, CodeLabel};
use multi_buffer::{Anchor, MultiBufferSnapshot, ToPoint};
use project::{Completion, CompletionResponse, CompletionSource};
use std::{
    cell::RefCell,
    ops::Range,
    path::PathBuf,
    rc::Rc,
    sync::{
        Arc,
        atomic::{self, AtomicUsize},
    },
    time::Duration,
};
use text::{Bias, Point, ToOffset};
use ui::prelude::*;

use crate::{
    AcceptEditPrediction, CodeContextMenu, CompletionContext, CompletionProvider, DisplayPoint,
    EditPrediction, EditPredictionKeybindAction, EditPredictionKeybindSurface,
    MenuEditPredictionsPolicy, MultiBuffer, ShowCompletions,
    deferred_input::CapturedInput,
    editor_tests::{init_test, update_test_language_settings},
    inlays::Inlay,
    test::{
        build_editor, editor_lsp_test_context::EditorLspTestContext,
        editor_test_context::EditorTestContext,
    },
};
use rpc::proto::PeerId;
use workspace::CollaboratorId;

struct EditorWithRightOccluders {
    editor: Entity<crate::Editor>,
    editor_width: Pixels,
    right_dock_width: Option<Pixels>,
    right_sidebar_width: Option<Pixels>,
}

impl Render for EditorWithRightOccluders {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .child(
                div()
                    .h_full()
                    .w(self.editor_width)
                    .overflow_hidden()
                    .child(self.editor.clone()),
            )
            .when_some(self.right_dock_width, |this, width| {
                this.child(
                    div()
                        .h_full()
                        .w(width)
                        .flex_shrink_0()
                        .occlude()
                        .debug_selector(|| "right_dock".into()),
                )
            })
            .when_some(self.right_sidebar_width, |this, width| {
                this.child(
                    div()
                        .h_full()
                        .w(width)
                        .flex_shrink_0()
                        .occlude()
                        .debug_selector(|| "right_sidebar".into()),
                )
            })
    }
}

async fn assert_edit_prediction_diff_popover_avoids_right_occluders(
    cx: &mut gpui::TestAppContext,
    right_dock_width: Option<Pixels>,
    right_sidebar_width: Option<Pixels>,
) {
    init_test(cx, |_| {});

    let editor_width = px(700.);
    let window_width = editor_width
        + right_dock_width.unwrap_or_default()
        + right_sidebar_width.unwrap_or_default();
    let buffer = cx.update(|cx| MultiBuffer::build_simple("", cx));
    let window = cx.add_window(|window, cx| {
        let editor = cx.new(|cx| build_editor(buffer, window, cx));
        window.focus(&editor.focus_handle(cx), cx);
        EditorWithRightOccluders {
            editor,
            editor_width,
            right_dock_width,
            right_sidebar_width,
        }
    });
    let editor = window
        .read_with(cx, |root, _| root.editor.clone())
        .expect("test window should contain editor");
    let mut cx = gpui::VisualTestContext::from_window(*window, cx);
    cx.simulate_resize(size(window_width, px(500.)));
    cx.run_until_parked();

    let mut cx = EditorTestContext::for_editor_in(editor, &mut cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.update_editor(|editor, _, _| {
        editor.set_menu_edit_predictions_policy(MenuEditPredictionsPolicy::Never);
    });
    cx.set_state("abcdefghijklmnopqrstuvwxyzabcdefghijklmnˇopqrstuvwxyzabcdef");

    propose_edits(&provider, vec![(40..41, "REPLACEMENT")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
    cx.editor(|editor, _, _| {
        assert!(!editor.edit_prediction_visible_in_cursor_popover(true));
        assert!(matches!(
            editor
                .active_edit_prediction
                .as_ref()
                .map(|state| &state.completion),
            Some(EditPrediction::Edit {
                display_mode: crate::EditDisplayMode::DiffPopover,
                ..
            })
        ));
    });
    cx.cx.update(|window, cx| {
        window.refresh();
        let _ = window.draw(cx);
    });

    cx.editor(|editor, _, _| {
        assert!(
            editor.last_position_map.is_some(),
            "editor should have rendered a position map"
        );
    });

    let popover_bounds = cx
        .cx
        .debug_bounds("edit_prediction_diff_popover")
        .expect("diff popover should render");

    for selector in ["right_dock", "right_sidebar"] {
        if let Some(occluder_bounds) = cx.cx.debug_bounds(selector) {
            assert!(
                !popover_bounds.intersects(&occluder_bounds),
                "diff popover {popover_bounds:?} should not overlap {selector} {occluder_bounds:?}"
            );
        }
    }
}

#[gpui::test]
async fn test_edit_prediction_diff_popover_avoids_right_sidebar(cx: &mut gpui::TestAppContext) {
    assert_edit_prediction_diff_popover_avoids_right_occluders(cx, None, Some(px(300.))).await;
}

#[gpui::test]
async fn test_edit_prediction_diff_popover_avoids_right_dock(cx: &mut gpui::TestAppContext) {
    assert_edit_prediction_diff_popover_avoids_right_occluders(cx, Some(px(300.)), None).await;
}

#[gpui::test]
async fn test_edit_prediction_diff_popover_avoids_right_dock_and_sidebar(
    cx: &mut gpui::TestAppContext,
) {
    assert_edit_prediction_diff_popover_avoids_right_occluders(cx, Some(px(300.)), Some(px(300.)))
        .await;
}

#[gpui::test]
async fn test_captured_input_completion_identity(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    for (item_ix, replace_menu) in [(Some(0), false), (None, true)] {
        let mut cx = EditorTestContext::new(cx).await;
        cx.set_state("aˇ");
        assign_editor_completion_menu_provider(&mut cx);
        cx.update_editor(|editor, window, cx| {
            editor.show_completions(&ShowCompletions, window, cx);
        });
        cx.run_until_parked();
        let (completion, input) = cx.update_editor(|editor, window, cx| {
            defer_input_for_identity_test(editor, window, cx);
            let completion = editor
                .confirm_completion(&crate::ConfirmCompletion { item_ix }, window, cx)
                .expect("queued completion");
            {
                let mut context_menu = editor.context_menu.borrow_mut();
                let Some(CodeContextMenu::Completions(menu)) = context_menu.as_mut() else {
                    panic!("completion menu");
                };
                if replace_menu {
                    menu.id += 1;
                } else {
                    Rc::make_mut(&mut menu.entries).swap(0, 1);
                }
            }
            (completion, editor.input_completion(cx))
        });
        input.await.expect("input completed");
        completion.await.expect("completion completed");
        if replace_menu {
            cx.assert_editor_state("aˇ");
            cx.editor(|editor, _, _| assert!(editor.context_menu.borrow().is_some()));
        } else {
            cx.assert_editor_state("fake_completionˇ");
        }
    }
}

#[gpui::test]
async fn test_captured_input_completion_navigation(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    for deferred in [false, true] {
        for item_ix in [None, Some(0)] {
            let mut cx = EditorTestContext::new(cx).await;
            cx.set_state("aˇ");
            assign_editor_completion_menu_provider(&mut cx);
            cx.update_editor(|editor, window, cx| {
                editor.show_completions(&ShowCompletions, window, cx);
            });
            cx.run_until_parked();
            cx.update(|window, cx| window.draw(cx).clear(cx));
            let editor = cx.editor.clone();
            let input = cx.update(|window, cx| {
                let focus = editor.update(cx, |editor, cx| {
                    if deferred {
                        defer_input_for_identity_test(editor, window, cx);
                    }
                    editor.focus_handle.clone()
                });
                focus.dispatch_action(&crate::ContextMenuNext, window, cx);
                focus.dispatch_action(&crate::ConfirmCompletion { item_ix }, window, cx);
                editor.update(cx, |editor, cx| editor.input_completion(cx))
            });
            input.await.expect("completion input completed");
            cx.run_until_parked();
            cx.assert_editor_state(if item_ix.is_some() {
                "fake_completionˇ"
            } else {
                "fake_completion_2ˇ"
            });
        }
    }
}

#[gpui::test]
async fn test_captured_input_completion_filter_reset(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state("aˇ");
    assign_editor_completion_menu_provider(&mut cx);
    cx.update_editor(|editor, window, cx| editor.show_completions(&ShowCompletions, window, cx));
    cx.run_until_parked();
    let matches = cx
        .update_editor(|editor, _, cx| {
            let context_menu = editor.context_menu.borrow();
            let Some(CodeContextMenu::Completions(menu)) = context_menu.as_ref() else {
                panic!("completion menu");
            };
            menu.do_async_filtering(
                Arc::new(String::new()),
                menu.buffer.read(cx).anchor_before(1),
                &menu.buffer,
                cx,
            )
        })
        .await;
    cx.update(|window, cx| window.draw(cx).clear(cx));
    let editor = cx.editor.clone();
    let input = cx.update(|window, cx| {
        let focus = editor.read(cx).focus_handle.clone();
        focus.dispatch_action(&crate::ContextMenuNext, window, cx);
        editor.update(cx, |editor, cx| {
            defer_input_for_identity_test(editor, window, cx);
        });
        focus.dispatch_action(&crate::ConfirmCompletion { item_ix: None }, window, cx);
        editor.update(cx, |editor, cx| {
            {
                let mut context_menu = editor.context_menu.borrow_mut();
                let Some(CodeContextMenu::Completions(menu)) = context_menu.as_mut() else {
                    panic!("completion menu");
                };
                assert_eq!(menu.selected_item, 1);
                menu.set_filter_results(matches, None, window, cx);
                assert_eq!(menu.selected_item, 0);
                assert_eq!(
                    menu.entries
                        .as_ref()
                        .iter()
                        .filter_map(|entry| entry.as_match())
                        .map(|candidate| candidate.candidate_id)
                        .collect::<Vec<_>>(),
                    [0, 1]
                );
            }
            editor.input_completion(cx)
        })
    });
    input.await.expect("completion input completed");
    cx.run_until_parked();
    cx.assert_editor_state("aˇ");
    let completion = cx.update_editor(|editor, window, cx| {
        editor
            .confirm_completion(&crate::ConfirmCompletion { item_ix: None }, window, cx)
            .expect("ready completion after filter reset")
    });
    completion.await.expect("completion completed");
    cx.assert_editor_state("fake_completionˇ");
}

#[gpui::test]
async fn test_captured_input_completion_scope_runs_handler(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state("aˇ");
    assign_editor_completion_menu_provider(&mut cx);
    cx.update_editor(|editor, window, cx| editor.show_completions(&ShowCompletions, window, cx));
    cx.run_until_parked();
    let completion = cx.update_editor(|editor, window, cx| {
        let action = crate::ConfirmCompletion { item_ix: Some(0) };
        let captured = editor
            .capture_completion_action(&action, cx)
            .expect("completion action");
        {
            let mut context_menu = editor.context_menu.borrow_mut();
            let Some(CodeContextMenu::Completions(menu)) = context_menu.as_mut() else {
                panic!("completion menu");
            };
            let completions = menu.completions.borrow().clone();
            menu.completions = Rc::new(RefCell::new(completions));
        }
        let mut handler_ran = false;
        CapturedInput::completion(captured).run(cx, |cx| {
            handler_ran = true;
            assert!(editor.confirm_completion(&action, window, cx).is_none());
        });
        assert!(handler_ran);
        assert_eq!(editor.text(cx), "a");
        editor
            .confirm_completion(&action, window, cx)
            .expect("ready completion after scope")
    });
    completion.await.expect("completion completed");
    cx.assert_editor_state("fake_completionˇ");
}

#[gpui::test]
async fn test_captured_input_raw_completion_identity(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    for (target_present, item_ix, replace_menu) in [
        (false, None, true),
        (true, None, true),
        (true, Some(1), false),
    ] {
        let mut cx = EditorTestContext::new(cx).await;
        cx.set_state("aˇ");
        assign_editor_completion_menu_provider(&mut cx);
        cx.update_editor(|editor, window, cx| {
            editor.show_completions(&ShowCompletions, window, cx)
        });
        cx.run_until_parked();
        let input = cx.update_editor(|editor, window, cx| {
            let hidden_menu = if target_present {
                None
            } else {
                editor.context_menu.borrow_mut().take()
            };
            cx.write_to_clipboard(ClipboardItem::new_string(String::from("before")));
            defer_input_for_identity_test(editor, window, cx);
            let origin = cx.weak_entity();
            assert!(editor.defer_keystroke_input(
                move |_, _| {
                    Box::new(move |window, cx| {
                        origin
                            .update(cx, |editor, cx| {
                                let action = crate::ConfirmCompletion { item_ix };
                                let captured = editor
                                    .capture_completion_action(&action, cx)
                                    .expect("completion capture");
                                CapturedInput::completion(captured).run(cx, |cx| {
                                    assert_eq!(
                                        editor
                                            .read_from_clipboard(cx)
                                            .expect("captured clipboard")
                                            .and_then(|item| item.text()),
                                        Some(String::from("before"))
                                    );
                                    let completion = editor.confirm_completion(&action, window, cx);
                                    if replace_menu {
                                        assert!(completion.is_none());
                                    } else {
                                        editor.detach_and_notify_err(
                                            completion.expect("captured explicit completion"),
                                            window,
                                            cx,
                                        );
                                    }
                                });
                            })
                            .expect("editor is alive");
                    })
                },
                window,
                cx,
            ));
            {
                let mut context_menu = editor.context_menu.borrow_mut();
                if let Some(menu) = hidden_menu {
                    *context_menu = Some(menu);
                }
                let Some(CodeContextMenu::Completions(menu)) = context_menu.as_mut() else {
                    panic!("completion menu");
                };
                if replace_menu {
                    menu.id += 1;
                }
            }
            cx.write_to_clipboard(ClipboardItem::new_string(String::from("after")));
            editor.input_completion(cx)
        });
        input.await.expect("raw completion input completed");
        cx.run_until_parked();
        if !replace_menu {
            cx.assert_editor_state("fake_completion_2ˇ");
            continue;
        }
        cx.assert_editor_state("aˇ");
        let completion = cx.update_editor(|editor, window, cx| {
            editor
                .confirm_completion(&crate::ConfirmCompletion { item_ix: None }, window, cx)
                .expect("ready completion after raw input")
        });
        completion.await.expect("completion completed");
        cx.assert_editor_state("fake_completionˇ");
    }
}

#[gpui::test]
async fn test_captured_input_raw_prediction_identity(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    for target_present in [false, true] {
        let mut cx = EditorTestContext::new(cx).await;
        cx.set_state("aˇ");
        let provider = cx.new(|_| FakeEditPredictionDelegate::default());
        assign_editor_completion_provider(provider.clone(), &mut cx);
        if target_present {
            propose_edits(&provider, vec![(1..1, "A")], &mut cx);
            cx.update_editor(|editor, window, cx| {
                editor.update_visible_edit_prediction(window, cx)
            });
        }
        let input = cx.update_editor(|editor, window, cx| {
            assert_eq!(editor.has_active_edit_prediction(), target_present);
            cx.write_to_clipboard(ClipboardItem::new_string(String::from("before")));
            defer_input_for_identity_test(editor, window, cx);
            let origin = cx.weak_entity();
            assert!(editor.defer_keystroke_input(
                move |_, _| {
                    Box::new(move |window, cx| {
                        origin
                            .update(cx, |editor, cx| {
                                let captured = editor.capture_edit_prediction_target(cx);
                                CapturedInput::edit_prediction(captured).run(cx, |cx| {
                                    assert_eq!(
                                        editor
                                            .read_from_clipboard(cx)
                                            .expect("captured clipboard")
                                            .and_then(|item| item.text()),
                                        Some(String::from("before"))
                                    );
                                    editor.accept_edit_prediction(
                                        &AcceptEditPrediction,
                                        window,
                                        cx,
                                    );
                                });
                            })
                            .expect("editor is alive");
                    })
                },
                window,
                cx,
            ));
            let snapshot = editor.buffer.read(cx).snapshot(cx);
            let position = snapshot
                .anchor_to_buffer_anchor(snapshot.anchor_after(Point::new(0, 1)))
                .expect("prediction position")
                .0;
            provider.update(cx, |provider, _| {
                provider.set_edit_prediction(Some(edit_prediction_types::EditPrediction::Local {
                    id: None,
                    edits: vec![(position..position, Arc::from("B"))],
                    cursor_position: None,
                    edit_preview: None,
                }));
            });
            editor.update_visible_edit_prediction(window, cx);
            cx.write_to_clipboard(ClipboardItem::new_string(String::from("after")));
            editor.input_completion(cx)
        });
        input.await.expect("raw prediction input completed");
        cx.assert_editor_state("aˇ");
        cx.update_editor(|editor, window, cx| {
            editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
        });
        cx.assert_editor_state("aBˇ");
    }
}

#[gpui::test]
async fn test_captured_input_prediction_presentation_refresh(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    update_test_language_settings(cx, &|settings| {
        settings.edit_predictions.get_or_insert_default().mode = Some(EditPredictionsMode::Subtle);
    });
    cx.update(|cx| cx.bind_keys([KeyBinding::new("ctrl-enter", AcceptEditPrediction, None)]));
    for (id, notify) in [
        (None, false),
        (None, true),
        (Some("prediction"), false),
        (Some("prediction"), true),
    ] {
        let mut cx = EditorTestContext::new(cx).await;
        cx.set_state("aˇ");
        let provider = cx.new(|_| FakeEditPredictionDelegate::default());
        assign_editor_completion_provider(provider.clone(), &mut cx);
        let position = cx.buffer_snapshot().anchor_after(1);
        cx.update(|_, cx| {
            provider.update(cx, |provider, _| {
                provider.set_edit_prediction(Some(edit_prediction_types::EditPrediction::Local {
                    id: id.map(SharedString::from),
                    edits: vec![(position..position, Arc::from("A"))],
                    cursor_position: None,
                    edit_preview: None,
                }));
            });
        });
        let completion = cx.update_editor(|editor, window, cx| {
            editor.update_visible_edit_prediction(window, cx);
            defer_input_for_identity_test(editor, window, cx);
            editor.update_edit_prediction_preview(&Modifiers::control(), window, cx);
            editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
            if notify {
                provider.update(cx, |_, cx| cx.notify());
            }
            assert_eq!(editor.text(cx), "a");
            editor.input_completion(cx)
        });
        completion.await.expect("prediction input completed");
        cx.assert_editor_state("aAˇ");
    }
}

#[gpui::test]
async fn test_captured_input_prediction_republication(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    for move_within in [false, true] {
        for (id, replacement_id, replacement_text, cursor_offset, replace_provider) in [
            (Some("first"), Some("second"), "A", None, false),
            (Some("first"), Some("first"), "B", None, false),
            (Some("first"), Some("first"), "A", Some(0), false),
            (None, None, "B", None, false),
            (Some("first"), Some("first"), "A", None, true),
        ] {
            let mut cx = EditorTestContext::new(cx).await;
            let initial_state = if move_within {
                "aˇ\nb\nc\nd\ne\nf"
            } else {
                "aˇ"
            };
            cx.set_state(initial_state);
            let provider = cx.new(|_| FakeEditPredictionDelegate::default());
            assign_editor_completion_provider(provider.clone(), &mut cx);
            let snapshot = cx.buffer_snapshot();
            let position = snapshot.anchor_after(snapshot.len());
            let completion = cx.update_editor(|editor, window, cx| {
                provider.update(cx, |provider, _| {
                    provider.set_edit_prediction(Some(
                        edit_prediction_types::EditPrediction::Local {
                            id: id.map(SharedString::from),
                            edits: vec![(position..position, Arc::from("A"))],
                            cursor_position: None,
                            edit_preview: None,
                        },
                    ));
                });
                editor.update_visible_edit_prediction(window, cx);
                assert!(editor.has_active_edit_prediction());
                defer_input_for_identity_test(editor, window, cx);
                if move_within {
                    editor.accept_next_word_edit_prediction(
                        &crate::AcceptNextWordEditPrediction,
                        window,
                        cx,
                    );
                } else {
                    editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
                }
                let replacement = edit_prediction_types::EditPrediction::Local {
                    id: replacement_id.map(SharedString::from),
                    edits: vec![(position..position, Arc::from(replacement_text))],
                    cursor_position: cursor_offset
                        .map(|offset| PredictedCursorPosition::new(position, offset)),
                    edit_preview: None,
                };
                if replace_provider {
                    let provider = cx.new(|_| FakeEditPredictionDelegate {
                        completion: Some(replacement),
                        ..FakeEditPredictionDelegate::default()
                    });
                    editor.set_edit_prediction_provider(
                        Some(provider),
                        EditPredictionRequestTrigger::EditorCreated,
                        window,
                        cx,
                    );
                } else {
                    provider.update(cx, |provider, _| {
                        provider.set_edit_prediction(Some(replacement));
                    });
                }
                editor.update_visible_edit_prediction(window, cx);
                editor.input_completion(cx)
            });
            completion.await.expect("prediction input completed");
            cx.assert_editor_state(initial_state);
            cx.editor(|editor, _, _| assert!(editor.has_active_edit_prediction()));
            if move_within {
                cx.update_editor(|editor, window, cx| {
                    editor.accept_next_word_edit_prediction(
                        &crate::AcceptNextWordEditPrediction,
                        window,
                        cx,
                    );
                });
                cx.assert_editor_state("a\nb\nc\nd\ne\nfˇ");
            }
        }
    }
}

#[gpui::test]
async fn test_captured_input_prediction_identity(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    for geometry_retry in [false, true] {
        let mut cx = EditorTestContext::new(cx).await;
        let provider = cx.new(|_| FakeEditPredictionDelegate::default());
        assign_editor_completion_provider(provider.clone(), &mut cx);
        cx.update_editor(|editor, _, cx| editor.set_soft_wrap_mode(SoftWrap::None, cx));
        let text = format!("x\nx\nx\nx\nx\nx\n{}", "x".repeat(2_000));
        cx.set_state(&format!("xˇ\nx\nx\nx\nx\nx\n{}", "x".repeat(2_000)));
        let target = if geometry_retry {
            Point::new(6, 1_000)
        } else {
            Point::new(0, 1)
        };
        propose_edits(&provider, vec![(target..target, "A")], &mut cx);
        let completion = cx.update_editor(|editor, window, cx| {
            editor.update_visible_edit_prediction(window, cx);
            if !geometry_retry {
                defer_input_for_identity_test(editor, window, cx);
            }
            editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
            assert!(editor.input_is_deferred());
            let snapshot = editor.buffer.read(cx).snapshot(cx);
            let position = snapshot
                .anchor_to_buffer_anchor(snapshot.anchor_after(Point::new(0, 1)))
                .expect("prediction position")
                .0;
            provider.update(cx, |provider, _| {
                provider.set_edit_prediction(Some(edit_prediction_types::EditPrediction::Local {
                    id: None,
                    edits: vec![(position..position, Arc::from("B"))],
                    cursor_position: None,
                    edit_preview: None,
                }));
            });
            editor.update_visible_edit_prediction(window, cx);
            editor.input_completion(cx)
        });
        completion.await.expect("prediction input completed");
        cx.editor(|editor, _, cx| {
            assert_eq!(editor.text(cx).get(..3), Some("x\nx"));
            assert_eq!(editor.text(cx), text);
            assert!(editor.has_active_edit_prediction());
        });
        cx.update_editor(|editor, window, cx| {
            editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
        });
        cx.editor(|editor, _, cx| assert_eq!(editor.text(cx).get(..3), Some("xB\n")));
    }
}

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
async fn test_edit_prediction_preview_jump_to_horizontally_offscreen_target(
    cx: &mut gpui::TestAppContext,
) {
    let (mut cx, provider) = modifier_preview_test_context(cx).await;
    let long_row = "x".repeat(100_000);
    let trailing_rows = "\n".repeat(40);
    cx.set_state(&format!("xˇ\nx\nx\nx\nx\nx\n{long_row}{trailing_rows}"));
    draw_editor(&mut cx);

    let right_target = Point::new(6, 90_000);
    propose_edits(&provider, vec![(right_target..right_target, "y")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
    assert_editor_active_move_completion(&mut cx, |snapshot, target| {
        assert_eq!(target.to_point(&snapshot), right_target);
    });

    cx.simulate_modifiers_change(Modifiers::control());
    draw_editor(&mut cx);
    assert_line_popover_within_text_bounds(&mut cx);

    cx.update_editor(|editor, window, cx| {
        editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
        editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
    });
    draw_editor(&mut cx);
    assert_eq!(newest_cursor(&mut cx), Point::new(0, 1));
    assert_line_popover_within_text_bounds(&mut cx);
    assert!(editor_scroll_position(&mut cx).x > 0.);

    cx.simulate_modifiers_change(Modifiers::none());
    draw_editor(&mut cx);
    assert_eq!(editor_scroll_position(&mut cx), gpui::Point::new(0., 0.));

    cx.simulate_modifiers_change(Modifiers::control());
    draw_editor(&mut cx);
    accept_completion(&mut cx);
    draw_editor(&mut cx);
    accept_completion(&mut cx);
    assert_eq!(newest_cursor(&mut cx), right_target);

    cx.simulate_modifiers_change(Modifiers::none());
    draw_editor(&mut cx);
    let scrolled_right_position = editor_scroll_position(&mut cx);
    assert!(scrolled_right_position.x > 0.);

    let left_target = Point::new(2, 1);
    propose_edits(&provider, vec![(left_target..left_target, "y")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
    cx.simulate_modifiers_change(Modifiers::control());
    draw_editor(&mut cx);
    assert_line_popover_within_text_bounds(&mut cx);

    cx.update_editor(|editor, window, cx| {
        editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
        editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
    });
    draw_editor(&mut cx);
    assert_eq!(newest_cursor(&mut cx), right_target);
    assert_eq!(editor_scroll_position(&mut cx).x, 0.);
    assert_line_popover_within_text_bounds(&mut cx);

    cx.simulate_modifiers_change(Modifiers::none());
    draw_editor(&mut cx);
    assert_eq!(editor_scroll_position(&mut cx), scrolled_right_position);
}

#[gpui::test]
async fn test_edit_prediction_cold_preview_acceptance_is_fifo(cx: &mut gpui::TestAppContext) {
    let (mut cx, provider) = modifier_preview_test_context(cx).await;
    let long_row = "x".repeat(100_000);
    let trailing_rows = "\n".repeat(40);
    let text = format!("x\nx\nx\nx\nx\nx\n{long_row}{trailing_rows}");
    cx.set_state(&format!("xˇ\nx\nx\nx\nx\nx\n{long_row}{trailing_rows}"));

    let target = Point::new(6, 90_000);
    propose_edits(&provider, vec![(target..target, "y")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
    cx.cx.update(|window, cx| {
        window.refresh();
        window.draw(cx).clear(cx);
        window.dispatch_event(
            PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                modifiers: Modifiers::control(),
                capslock: Capslock { on: false },
            }),
            cx,
        );
    });

    let completion = cx.update_editor(|editor, window, cx| {
        assert!(editor.last_position_map.is_none());
        assert!(editor.last_input_viewport().is_some());
        let initial_scroll_position = editor.scroll_position(cx);
        editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
        editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
        assert!(editor.input_is_deferred());
        assert_eq!(editor.scroll_position(cx), initial_scroll_position);
        assert_eq!(editor.buffer.read(cx).snapshot(cx).text(), text);
        assert_eq!(
            editor
                .highlighted_rows::<crate::EditPredictionPreview>(cx)
                .count(),
            0,
        );
        assert!(matches!(
            editor.edit_prediction_preview,
            crate::EditPredictionPreview::Active {
                previous_scroll_position: None,
                ..
            }
        ));
        assert!(editor.defer_input(
            |editor, _, cx| {
                let snapshot = editor.display_snapshot(cx);
                assert_eq!(
                    editor.selections.newest_anchor().head().to_point(&snapshot),
                    Point::new(0, 1),
                );
                assert_eq!(
                    editor.highlighted_display_row_for_autoscroll(&snapshot),
                    Some(crate::DisplayRow(6)),
                );
                assert!(editor.scroll_position(cx).x > 0.);
                assert!(matches!(
                    editor.edit_prediction_preview,
                    crate::EditPredictionPreview::Active {
                        previous_scroll_position: Some(_),
                        ..
                    }
                ));
            },
            window,
            cx,
        ));
        editor.input_completion(cx)
    });
    cx.cx.update(|window, cx| {
        window.dispatch_event(
            PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                modifiers: Modifiers::none(),
                capslock: Capslock { on: false },
            }),
            cx,
        );
    });
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("!", window, cx);
        assert_eq!(editor.buffer.read(cx).snapshot(cx).text(), text);
        assert!(editor.edit_prediction_preview_is_active());
    });
    completion.await.expect("prediction input completed");

    cx.assert_editor_state(&format!("x!ˇ\nx\nx\nx\nx\nx\n{long_row}{trailing_rows}"));
    assert_eq!(editor_scroll_position(&mut cx), gpui::Point::new(0., 0.));
    cx.editor(|editor, _, cx| {
        assert!(!editor.edit_prediction_preview_is_active());
        assert_eq!(
            editor
                .highlighted_rows::<crate::EditPredictionPreview>(cx)
                .count(),
            0,
        );
    });
}

#[gpui::test]
async fn test_edit_prediction_preview_resolves_target_after_inlay_change(
    cx: &mut gpui::TestAppContext,
) {
    let (mut cx, provider) = modifier_preview_test_context(cx).await;
    let long_row = "x".repeat(100_000);
    let trailing_rows = "\n".repeat(40);
    cx.set_state(&format!("xˇ\nx\nx\nx\nx\nx\n{long_row}{trailing_rows}"));
    draw_editor(&mut cx);

    let target_point = Point::new(6, 1);
    propose_edits(&provider, vec![(target_point..target_point, "y")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
    cx.simulate_modifiers_change(Modifiers::control());
    draw_editor(&mut cx);

    let completion = cx.update_editor(|editor, window, cx| {
        let initial_scroll_position = editor.scroll_position(cx);
        let position = editor
            .buffer()
            .read(cx)
            .snapshot(cx)
            .anchor_after(Point::new(6, 0));
        editor.splice_inlays(
            &[],
            vec![Inlay::mock_hint(0, position, "x".repeat(200))],
            cx,
        );
        editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
        assert!(editor.input_is_deferred());
        assert_eq!(editor.scroll_position(cx), initial_scroll_position);
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        assert_eq!(
            editor.selections.newest_anchor().head().to_point(&snapshot),
            Point::new(0, 1),
        );
        editor.input_completion(cx)
    });
    completion.await.expect("prediction input completed");
    draw_editor(&mut cx);

    assert_eq!(newest_cursor(&mut cx), Point::new(0, 1));
    cx.editor(|editor, _, cx| {
        let position_map = editor
            .last_position_map
            .as_ref()
            .expect("editor should have been drawn");
        let target_x = 201. * f64::from(position_map.em_layout_width);
        let visible_x = editor
            .visible_x_range(&position_map.snapshot, cx)
            .expect("editor should have a viewport");
        assert!(target_x >= *visible_x.start());
        assert!(target_x + f64::from(position_map.em_layout_width) <= *visible_x.end());
    });
}

#[gpui::test]
async fn test_edit_prediction_preview_jump_to_vertically_offscreen_target(
    cx: &mut gpui::TestAppContext,
) {
    let (mut cx, provider) = modifier_preview_test_context(cx).await;
    let rows = "\nx".repeat(80);
    let half_row = "x".repeat(50_000);
    cx.set_state(&format!("{half_row}ˇ{half_row}{rows}"));
    draw_editor(&mut cx);

    let target = Point::new(70, 0);
    propose_edits(&provider, vec![(target..target, "y")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
    cx.simulate_modifiers_change(Modifiers::control());
    for scroll_x in [0., 60_000.] {
        cx.update_editor(|editor, window, cx| {
            editor.set_scroll_position(gpui::Point::new(scroll_x, 0.), window, cx);
        });
        draw_editor(&mut cx);
        assert_line_popover_within_text_bounds(&mut cx);
    }
    let previous_scroll_position = editor_scroll_position(&mut cx);
    let original_left_edge = left_edge_point(&mut cx);

    cx.update_editor(|editor, window, cx| {
        editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
        editor.accept_edit_prediction(&AcceptEditPrediction, window, cx);
    });
    draw_editor(&mut cx);
    assert_eq!(newest_cursor(&mut cx), Point::new(0, 50_000));
    let preview_scroll_position = editor_scroll_position(&mut cx);
    assert_eq!(preview_scroll_position.x, 0.);
    assert!(preview_scroll_position.y > 0.);

    cx.simulate_modifiers_change(Modifiers::none());
    draw_editor(&mut cx);
    assert_eq!(editor_scroll_position(&mut cx), previous_scroll_position);

    cx.simulate_modifiers_change(Modifiers::control());
    draw_editor(&mut cx);
    accept_completion(&mut cx);
    draw_editor(&mut cx);

    cx.update_editor(|editor, _, cx| {
        let position = editor
            .buffer()
            .read(cx)
            .snapshot(cx)
            .anchor_after(Point::new(0, 10));
        editor.splice_inlays(
            &[],
            vec![Inlay::mock_hint(0, position, "h".repeat(500))],
            cx,
        );
    });
    cx.simulate_modifiers_change(Modifiers::none());
    draw_editor(&mut cx);
    cx.run_until_parked();
    draw_editor(&mut cx);
    assert_eq!(left_edge_point(&mut cx), original_left_edge);
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
        editor.refresh_edit_prediction(
            false,
            false,
            EditPredictionRequestTrigger::Other,
            window,
            cx,
        );
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
        editor.refresh_edit_prediction(
            false,
            false,
            EditPredictionRequestTrigger::Other,
            window,
            cx,
        );
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
        editor.refresh_edit_prediction(
            false,
            false,
            EditPredictionRequestTrigger::Other,
            window,
            cx,
        );
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
async fn test_hidden_edit_prediction_does_not_open_snippet_menu_on_word_input(
    cx: &mut gpui::TestAppContext,
) {
    init_test(cx, |_| {});

    let mut cx = hidden_edit_prediction_snippet_test_context(cx).await;
    cx.simulate_input("t");
    cx.run_until_parked();

    cx.update_editor(|editor, _, _| {
        assert!(editor.has_active_edit_prediction());
        assert!(editor.context_menu.borrow().is_none());
    });
}

#[gpui::test]
async fn test_hidden_edit_prediction_opens_snippet_menu_for_strong_prefix_match(
    cx: &mut gpui::TestAppContext,
) {
    init_test(cx, |_| {});

    let mut cx = hidden_edit_prediction_snippet_test_context(cx).await;
    cx.simulate_input("t");
    cx.run_until_parked();
    cx.simulate_input("h");
    cx.run_until_parked();

    cx.update_editor(|editor, _, _| {
        let Some(CodeContextMenu::Completions(menu)) = &*editor.context_menu.borrow() else {
            panic!("expected completions menu");
        };
        let entries = menu.entries.as_ref();
        assert!(
            entries
                .iter()
                .any(|entry| { entry.as_match().is_some_and(|m| m.string == "Theta") })
        );
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

#[gpui::test]
async fn test_edit_prediction_preview_does_not_hide_code_actions_on_modifier_press(
    cx: &mut gpui::TestAppContext,
) {
    init_test(cx, |_| {});
    update_test_language_settings(cx, &|settings| {
        settings.edit_predictions.get_or_insert_default().mode = Some(EditPredictionsMode::Subtle);
    });
    cx.update(|cx| {
        cx.bind_keys([KeyBinding::new(
            "ctrl-enter",
            AcceptEditPrediction,
            Some("Editor && edit_prediction && !showing_completions"),
        )]);
    });

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            code_action_provider: Some(lsp::CodeActionProviderCapability::Simple(true)),
            ..Default::default()
        },
        cx,
    )
    .await;
    cx.set_state(indoc! {"
        fn main() {
            let valueˇ = 1;
        }
    "});

    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    cx.update_editor(|editor, window, cx| {
        editor.set_edit_prediction_provider(
            Some(provider.clone()),
            EditPredictionRequestTrigger::EditorCreated,
            window,
            cx,
        );
    });

    let snapshot = cx.buffer_snapshot();
    let edit_position = snapshot.anchor_after(Point::new(1, 13));
    cx.update(|_, cx| {
        provider.update(cx, |provider, _| {
            provider.set_edit_prediction(Some(edit_prediction_types::EditPrediction::Local {
                id: None,
                edits: vec![(edit_position..edit_position, " + 1".into())],
                cursor_position: None,
                edit_preview: None,
            }))
        })
    });
    cx.update_editor(|editor, window, cx| {
        editor.set_menu_edit_predictions_policy(MenuEditPredictionsPolicy::ByProvider);
        editor.update_visible_edit_prediction(window, cx);
    });
    cx.update_editor(|editor, _, _| {
        assert!(editor.has_active_edit_prediction());
        assert!(editor.stale_edit_prediction_in_menu.is_none());
    });

    let mut code_action_requests = cx.set_request_handler::<lsp::request::CodeActionRequest, _, _>(
        move |_, _, _| async move {
            Ok(Some(vec![lsp::CodeActionOrCommand::CodeAction(
                lsp::CodeAction {
                    title: "Inline value".to_string(),
                    kind: Some(lsp::CodeActionKind::QUICKFIX),
                    ..Default::default()
                },
            )]))
        },
    );

    cx.update_editor(|editor, window, cx| {
        editor.toggle_code_actions(
            &crate::actions::ToggleCodeActions {
                deployed_from: None,
                quick_launch: false,
            },
            window,
            cx,
        );
    });
    code_action_requests.next().await;
    cx.run_until_parked();
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;

    cx.update_editor(|editor, _, _| {
        assert!(!editor.has_active_edit_prediction());
        assert!(editor.stale_edit_prediction_in_menu.is_some());
        assert!(editor.context_menu_visible());
        assert!(matches!(
            editor.context_menu.borrow().as_ref(),
            Some(crate::code_context_menus::CodeContextMenu::CodeActions(_))
        ));
        assert!(!editor.edit_prediction_preview_is_active());
    });

    cx.simulate_modifiers_change(Modifiers::control());
    cx.run_until_parked();

    cx.update_editor(|editor, _, _| {
        assert!(
            !editor.edit_prediction_preview_is_active(),
            "modifier-only press should not activate edit prediction preview while code actions are open"
        );
        assert!(
            editor.context_menu_visible(),
            "modifier-only press should not hide the code actions menu"
        );
        assert!(matches!(
            editor.context_menu.borrow().as_ref(),
            Some(crate::code_context_menus::CodeContextMenu::CodeActions(_))
        ));
    });
}

#[gpui::test]
async fn test_edit_prediction_preview_supersedes_completions_menu(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    update_test_language_settings(cx, &|settings| {
        settings.edit_predictions.get_or_insert_default().mode = Some(EditPredictionsMode::Subtle);
    });
    cx.update(|cx| {
        cx.bind_keys([KeyBinding::new(
            "ctrl-enter",
            AcceptEditPrediction,
            Some("Editor && edit_prediction && showing_completions"),
        )]);
    });

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    assign_editor_completion_menu_provider(&mut cx);
    cx.set_state("let x = ˇ;");

    propose_edits(&provider, vec![(8..8, "42")], &mut cx);
    cx.update_editor(|editor, window, cx| {
        editor.set_menu_edit_predictions_policy(MenuEditPredictionsPolicy::ByProvider);
        editor.update_visible_edit_prediction(window, cx);
    });
    cx.update_editor(|editor, window, cx| {
        editor.show_completions(&ShowCompletions, window, cx);
    });
    cx.run_until_parked();

    cx.editor(|editor, _, _| {
        assert!(editor.has_active_edit_prediction());
        assert!(editor.context_menu_visible());
        assert!(matches!(
            editor.context_menu.borrow().as_ref(),
            Some(crate::code_context_menus::CodeContextMenu::Completions(_))
        ));
        assert!(!editor.edit_prediction_preview_is_active());
    });

    cx.simulate_modifiers_change(Modifiers::control());
    cx.run_until_parked();

    cx.editor(|editor, _, _| {
        assert!(editor.edit_prediction_preview_is_active());
        assert!(!editor.context_menu_visible());
        assert!(matches!(
            editor.context_menu.borrow().as_ref(),
            Some(crate::code_context_menus::CodeContextMenu::Completions(_))
        ));
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
                    editor.take_active_edit_prediction(true, cx);
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

#[gpui::test]
async fn test_cancel_clears_stale_edit_prediction_in_menu(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    load_default_keymap(cx);

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let x = ˇ;");

    propose_edits(&provider, vec![(8..8, "42")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    cx.update_editor(|editor, _window, _cx| {
        assert!(editor.active_edit_prediction.is_some());
        assert!(editor.stale_edit_prediction_in_menu.is_none());
    });

    cx.simulate_keystroke("escape");
    cx.run_until_parked();

    cx.update_editor(|editor, _window, _cx| {
        assert!(editor.active_edit_prediction.is_none());
        assert!(editor.stale_edit_prediction_in_menu.is_none());
    });
}

#[gpui::test]
async fn test_discard_clears_delegate_completion(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    load_default_keymap(cx);

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let x = ˇ;");

    propose_edits(&provider, vec![(8..8, "42")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    cx.update_editor(|editor, _window, _cx| {
        assert!(editor.active_edit_prediction.is_some());
    });

    // Dismiss the prediction — this must call discard() on the delegate,
    // which should clear self.completion.
    cx.simulate_keystroke("escape");
    cx.run_until_parked();

    cx.update_editor(|editor, _window, _cx| {
        assert!(editor.active_edit_prediction.is_none());
    });

    // update_visible_edit_prediction must NOT bring the prediction back,
    // because discard() cleared self.completion in the delegate.
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    cx.update_editor(|editor, _window, _cx| {
        assert!(
            editor.active_edit_prediction.is_none(),
            "prediction must not resurface after discard()"
        );
    });
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

async fn hidden_edit_prediction_snippet_test_context(
    cx: &mut gpui::TestAppContext,
) -> EditorTestContext {
    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.update_editor(|editor, _, cx| {
        editor.set_menu_edit_predictions_policy(MenuEditPredictionsPolicy::Never);
        editor.project().unwrap().update(cx, |project, cx| {
            project.snippets().update(cx, |snippets, _cx| {
                let snippet = project::snippet_provider::Snippet {
                    prefix: vec!["Theta".to_string(), "turnstile".to_string()],
                    body: "⊢".to_string(),
                    description: Some("unicode symbol".to_string()),
                    name: "unicode snippets".to_string(),
                };
                snippets.add_snippet_for_test(
                    None,
                    PathBuf::from("test_snippets.json"),
                    vec![Arc::new(snippet)],
                );
            });
        })
    });
    cx.set_state("ˇ");

    propose_edits(&provider, vec![(0..0, "x")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
    cx
}

fn assign_editor_completion_provider(
    provider: Entity<FakeEditPredictionDelegate>,
    cx: &mut EditorTestContext,
) {
    cx.update_editor(|editor, window, cx| {
        editor.set_edit_prediction_provider(
            Some(provider),
            EditPredictionRequestTrigger::EditorCreated,
            window,
            cx,
        );
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
        editor.set_edit_prediction_provider(
            Some(provider),
            EditPredictionRequestTrigger::EditorCreated,
            window,
            cx,
        );
    })
}

struct FakeCompletionMenuProvider;

impl CompletionProvider for FakeCompletionMenuProvider {
    fn completions(
        &self,
        buffer: &Entity<Buffer>,
        _buffer_position: text::Anchor,
        _trigger: CompletionContext,
        _window: &mut Window,
        cx: &mut Context<crate::Editor>,
    ) -> Task<anyhow::Result<Vec<CompletionResponse>>> {
        let replace_range = text::Anchor::min_max_range_for_buffer(buffer.read(cx).remote_id());
        let completions = ["fake_completion", "fake_completion_2"]
            .into_iter()
            .map(|label| Completion {
                replace_range: replace_range.clone(),
                new_text: label.to_string(),
                label: CodeLabel::plain(label.to_string(), None),
                documentation: None,
                source: CompletionSource::Custom,
                icon_path: None,
                icon_color: None,
                match_start: None,
                snippet_deduplication_key: None,
                insert_text_mode: None,
                confirm: None,
                group: None,
            })
            .collect();

        Task::ready(Ok(vec![CompletionResponse {
            completions,
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
        _debounce_duration: Duration,
        _trigger: edit_prediction_types::EditPredictionRequestTrigger,
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
        self.completion.take();
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
        _debounce_duration: Duration,
        _trigger: edit_prediction_types::EditPredictionRequestTrigger,
        _cx: &mut gpui::Context<Self>,
    ) {
    }

    fn accept(&mut self, _cx: &mut gpui::Context<Self>) {}

    fn discard(
        &mut self,
        _reason: edit_prediction_types::EditPredictionDiscardReason,
        _cx: &mut gpui::Context<Self>,
    ) {
        self.completion.take();
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

async fn modifier_preview_test_context(
    cx: &mut gpui::TestAppContext,
) -> (EditorTestContext, Entity<FakeEditPredictionDelegate>) {
    init_test(cx, |_| {});
    update_test_language_settings(cx, &|settings| {
        settings.defaults.soft_wrap = Some(SoftWrap::None);
        settings.edit_predictions.get_or_insert_default().mode = Some(EditPredictionsMode::Subtle);
    });
    cx.update(|cx| cx.bind_keys([KeyBinding::new("ctrl-enter", AcceptEditPrediction, None)]));

    let mut cx = EditorTestContext::new(cx).await;
    cx.simulate_resize(size(px(800.), px(600.)));
    let provider = cx.new(|_| FakeEditPredictionDelegate::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    (cx, provider)
}

fn draw_editor(cx: &mut EditorTestContext) {
    cx.cx.update(|window, cx| {
        window.refresh();
        window.draw(cx).clear(cx);
    });
}

fn newest_cursor(cx: &mut EditorTestContext) -> Point {
    cx.editor(|editor, _, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        editor.selections.newest_anchor().head().to_point(&snapshot)
    })
}

fn editor_scroll_position(cx: &mut EditorTestContext) -> gpui::Point<f64> {
    cx.update_editor(|editor, _, cx| editor.scroll_position(cx))
}

fn left_edge_point(cx: &mut EditorTestContext) -> Point {
    cx.editor(|editor, _, _| {
        let position_map = editor
            .last_position_map
            .as_ref()
            .expect("editor should have been drawn");
        let column = position_map.line_layouts[0]
            .index_for_x(position_map.scroll_pixel_position.x)
            .expect("a visible left edge");
        position_map.snapshot.display_point_to_point(
            DisplayPoint::new(position_map.visible_row_range.start, column as u32),
            Bias::Left,
        )
    })
}

fn defer_input_for_identity_test(
    editor: &mut crate::Editor,
    window: &mut Window,
    cx: &mut Context<crate::Editor>,
) {
    editor.set_placeholder_text(&"x".repeat(2_000), window, cx);
    let pending = editor
        .placeholder_display_map
        .as_ref()
        .expect("placeholder map")
        .update(cx, |map, cx| {
            map.snapshot(cx)
                .long_row(crate::DisplayRow(0))
                .err()
                .expect("cold geometry")
        });
    editor.defer_geometry(pending, |_, _, _| {}, window, cx);
    assert!(editor.input_is_deferred());
}

fn assert_line_popover_within_text_bounds(cx: &mut EditorTestContext) {
    let text_bounds = cx.editor(|editor, window, _| {
        window.pixel_snap_bounds(
            editor
                .last_position_map
                .as_ref()
                .expect("editor should have been drawn")
                .text_hitbox
                .bounds,
        )
    });
    let popover_bounds = cx
        .cx
        .debug_bounds("edit_prediction_line_popover")
        .expect("edit prediction indicator should be drawn");
    assert_eq!(
        popover_bounds.intersect(&text_bounds),
        popover_bounds,
        "edit prediction indicator should be within the text bounds {text_bounds:?}"
    );
}
