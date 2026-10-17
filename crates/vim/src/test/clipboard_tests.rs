use crate::{
    PushRegister, UseSystemClipboard, VimAddon, motion,
    normal::{YankToEndOfLine, paste::Paste, repeat::ReplayLastRecording},
    state::{Mode, Register, ReplayableAction, VimGlobals},
    test::{VimTestContext, add_background_editor, dispatch_keys},
};
use editor::{ClipboardAccess, Editor, SelectionEffects, test::NativeClipboardCapture};
use futures::FutureExt;
use gpui::{
    Action, App, ClipboardItem, ClipboardReadError, ClipboardSnapshot, ClipboardSnapshotRead,
    Context, Entity, Focusable, KeyBinding, TestAppContext, Window,
};
use language::Point;
use settings::SettingsStore;
use std::{cell::Cell, rc::Rc, sync::Arc};

#[gpui::test]
async fn test_chord_mismatch_pastes_read_distinct_clipboard_snapshots(cx: &mut TestAppContext) {
    for queued in [false, true] {
        let mut cx = VimTestContext::new(cx, true).await;
        set_clipboard_setting(&mut cx, UseSystemClipboard::Always);
        cx.set_state("«Vˇ»!", Mode::Visual);
        cx.update_editor(|editor, _, cx| {
            editor.set_soft_wrap_mode(settings::SoftWrap::None, cx);
        });
        cx.update(|window, cx| {
            cx.bind_keys([
                KeyBinding::new("ctrl-k", Paste::default(), Some("Editor")),
                KeyBinding::new("ctrl-k ctrl-k", Paste::default(), Some("Editor")),
                KeyBinding::new("ctrl-j", Paste::default(), Some("Editor")),
            ]);
            cx.write_to_clipboard(ClipboardItem::new_string(String::from("A")));
            window.draw(cx).clear(cx);
        });
        if queued {
            cx.update_editor(|editor, window, cx| {
                editor.schedule_input_continuation(window, cx, |_, _| {});
            });
        }
        dispatch_keys(&mut cx, "ctrl-k ctrl-j");
        let completion = cx.update_editor(|editor, _, cx| editor.input_completion(cx));
        completion.await.expect("both paste commands completed");
        cx.run_until_parked();
        assert_eq!(cx.buffer_text(), "AV!", "queued={queued}");
        cx.update(|_, cx| {
            assert_eq!(
                cx.read_from_clipboard().and_then(|item| item.text()),
                Some(String::from("V")),
            );
        });
    }
}

#[gpui::test]
async fn test_black_hole_repeat_does_not_capture_native_input(cx: &mut TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    set_clipboard_setting(&mut cx, UseSystemClipboard::Always);
    cx.set_state("ˇabcdef", Mode::Normal);
    let captures = Rc::new(Cell::new(0));
    cx.update(|_, cx| {
        let captures = captures.clone();
        cx.set_global(NativeClipboardCapture(Rc::new(move |_| {
            captures.set(captures.get() + 1);
            ClipboardSnapshotRead::failed(ClipboardReadError::Unavailable)
        })));
    });
    cx.simulate_keystrokes("\" _ x .");
    cx.run_until_parked();
    cx.assert_state("ˇcdef", Mode::Normal);
    assert_eq!(captures.get(), 0);
    cx.update(|_, cx| {
        cx.remove_global::<NativeClipboardCapture>();
        assert!(cx.global::<VimGlobals>().replayer.is_none());
        assert!(!cx.global::<VimGlobals>().dot_replaying);
    });
}

#[gpui::test]
async fn test_native_admission_rejection_clears_command_before_next_key(cx: &mut TestAppContext) {
    for delayed in [false, true] {
        for temporary in [false, true] {
            let mut cx = VimTestContext::new(cx, true).await;
            set_clipboard_setting(&mut cx, UseSystemClipboard::Never);
            cx.set_state("ˇseed", Mode::Normal);
            cx.simulate_keystrokes("\" _ x");
            cx.set_state(
                "ˇabcdef",
                if temporary {
                    Mode::Insert
                } else {
                    Mode::Normal
                },
            );
            if temporary {
                cx.simulate_keystrokes("ctrl-o");
            }
            let recorded =
                cx.update(|_, cx| format!("{:?}", cx.global::<VimGlobals>().recorded_actions));
            let (read, resolver) = ClipboardSnapshotRead::pending();
            let resolver = if delayed {
                Some(resolver)
            } else {
                resolver.resolve(Err(ClipboardReadError::Unavailable));
                None
            };
            cx.update(|_, cx| {
                cx.set_global(NativeClipboardCapture(Rc::new(move |_| read.clone())));
            });
            dispatch_keys(&mut cx, "\" + 2 g shift-r 3 w");
            if delayed {
                cx.run_until_parked();
                cx.assert_state("ˇabcdef", Mode::Normal);
            }
            dispatch_keys(&mut cx, "l");
            if let Some(resolver) = resolver {
                resolver.resolve(Err(ClipboardReadError::Unavailable));
            }
            cx.run_until_parked();
            cx.assert_state(
                if temporary { "lˇabcdef" } else { "aˇbcdef" },
                if temporary {
                    Mode::Insert
                } else {
                    Mode::Normal
                },
            );
            assert_rejected_command_cleared(&mut cx);
            cx.update(|_, cx| {
                cx.remove_global::<NativeClipboardCapture>();
                let globals = cx.global::<VimGlobals>();
                assert_eq!(format!("{:?}", globals.recorded_actions), recorded);
                assert_eq!(globals.recorded_register_for_dot, Some('_'));
            });
            if !temporary {
                cx.simulate_keystrokes(".");
                cx.assert_state("aˇcdef", Mode::Normal);
            }
        }
        for (keys, mode, paste_keys, expected) in [
            ("i shift-a", Mode::Insert, Some("ctrl-r +"), "AAˇBBbcdef"),
            (
                "i shift-a",
                Mode::Insert,
                Some("ctrl-r f6 f7 +"),
                "AAˇBBbcdef",
            ),
            (
                "2 i shift-a",
                Mode::Insert,
                Some("ctrl-r +"),
                "ABAABAˇBBbcdef",
            ),
            ("i shift-a", Mode::Insert, None, "AAˇBBbcdef"),
            ("2 i shift-a", Mode::Insert, None, "ABAABAˇBBbcdef"),
            ("shift-r shift-a", Mode::Replace, None, "AAˇBef"),
            ("2 shift-r shift-a", Mode::Replace, None, "ABAABAˇB"),
            ("i shift-a", Mode::Insert, Some("ctrl-v"), "AAˇBBbcdef"),
            ("shift-r shift-a", Mode::Replace, Some("ctrl-v"), "AAˇBef"),
        ] {
            let mut cx = VimTestContext::new(cx, true).await;
            set_clipboard_setting(&mut cx, UseSystemClipboard::Never);
            cx.set_state("ˇabcdef", Mode::Normal);
            cx.simulate_keystrokes("\" _ x q a");
            cx.simulate_keystrokes(keys);
            let (read, resolver) = ClipboardSnapshotRead::pending();
            let resolver = if delayed {
                Some(resolver)
            } else {
                resolver.resolve(Err(ClipboardReadError::Unavailable));
                None
            };
            cx.update(|_, cx| {
                cx.bind_keys([
                    KeyBinding::new("ctrl-v", editor::actions::Paste, Some("Editor")),
                    KeyBinding::new("f6", editor::actions::MoveLeft, Some("Editor")),
                    KeyBinding::new("f7", editor::actions::MoveRight, Some("Editor")),
                ]);
                cx.set_global(NativeClipboardCapture(Rc::new(move |_| read.clone())));
            });
            if let Some(paste_keys) = paste_keys {
                dispatch_keys(&mut cx, paste_keys);
            } else {
                cx.update(|window, cx| {
                    window.focused(cx).expect("editor focus").dispatch_action(
                        &editor::actions::Paste,
                        window,
                        cx,
                    );
                });
            }
            dispatch_keys(&mut cx, "shift-b");
            if let Some(resolver) = resolver {
                resolver.resolve(Err(ClipboardReadError::Unavailable));
            }
            cx.run_until_parked();
            cx.assert_state(
                if mode == Mode::Insert {
                    "ABˇbcdef"
                } else {
                    "ABˇdef"
                },
                mode,
            );
            cx.simulate_keystrokes("escape q");
            cx.update(|_, cx| {
                let globals = cx.global::<VimGlobals>();
                assert_eq!(
                    globals.recording_register, None,
                    "{delayed} {keys} {paste_keys:?}"
                );
            });
            cx.simulate_keystrokes(".");
            cx.assert_state(expected, Mode::Normal);
            cx.set_state("ˇbcdef", Mode::Normal);
            cx.simulate_keystrokes("@ a");
            cx.assert_state(
                match (mode, keys.starts_with('2')) {
                    (Mode::Insert, false) => "AˇBbcdef",
                    (Mode::Insert, true) => "ABAˇBbcdef",
                    (Mode::Replace, false) => "AˇBdef",
                    (Mode::Replace, true) => "ABAˇBf",
                    _ => unreachable!(),
                },
                Mode::Normal,
            );
            cx.update(|_, cx| {
                cx.remove_global::<NativeClipboardCapture>();
            });
        }
    }
}

#[gpui::test]
async fn test_native_admission_rejection_stops_dot_replay(cx: &mut TestAppContext) {
    for delayed in [false, true] {
        let mut cx = VimTestContext::new(cx, true).await;
        set_clipboard_setting(&mut cx, UseSystemClipboard::Never);
        cx.set_state("ˇseed", Mode::Normal);
        cx.update(|_, cx| {
            cx.write_to_clipboard(ClipboardItem::new_string(String::from("Q")));
        });
        cx.simulate_keystrokes("\" + g shift-r w");
        cx.assert_state("ˇQ", Mode::Normal);
        cx.set_state("ˇabcdef", Mode::Normal);
        let (read, resolver) = ClipboardSnapshotRead::pending();
        let resolver = if delayed {
            Some(resolver)
        } else {
            resolver.resolve(Err(ClipboardReadError::Unavailable));
            None
        };
        cx.update(|_, cx| {
            cx.set_global(NativeClipboardCapture(Rc::new(move |_| read.clone())));
        });
        dispatch_keys(&mut cx, "2 .");
        cx.run_until_parked();
        dispatch_keys(&mut cx, "l");
        if let Some(resolver) = resolver {
            resolver.resolve(Err(ClipboardReadError::Unavailable));
        }
        cx.run_until_parked();
        cx.assert_state("aˇbcdef", Mode::Normal);
        assert_rejected_command_cleared(&mut cx);
        cx.update(|_, cx| {
            cx.remove_global::<NativeClipboardCapture>();
            assert_eq!(
                cx.global::<VimGlobals>().recorded_register_for_dot,
                Some('+')
            );
        });
    }
}

#[gpui::test]
async fn test_on_yank_delete_waits_for_captured_clipboard(cx: &mut TestAppContext) {
    for setting in [UseSystemClipboard::Never, UseSystemClipboard::OnYank] {
        let mut cx = VimTestContext::new(cx, true).await;
        set_clipboard_setting(&mut cx, setting);
        cx.update_editor(|editor, _, cx| {
            editor.set_soft_wrap_mode(settings::SoftWrap::None, cx);
        });
        cx.set_state("ˇstart\nshort", Mode::Normal);
        let suffix = "x".repeat(1_998);
        let text = format!("start\nAB{suffix}");
        let (read, resolver) = ClipboardSnapshotRead::pending();
        cx.update(|_, cx| {
            cx.global_mut::<VimGlobals>().last_yank = None;
            cx.write_to_clipboard(ClipboardItem::new_string(String::from("EXT")));
            cx.set_global(NativeClipboardCapture(Rc::new(move |_| read.clone())));
        });
        cx.background_executor.set_block_on_ticks(0..=0);
        cx.update_editor(|editor, window, cx| {
            editor.set_text(text.clone(), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 0)..Point::new(0, 0)]);
            });
        });
        dispatch_keys(&mut cx, "j x p");
        let mut completion = cx.update_editor(|editor, _, cx| editor.input_completion(cx));
        assert!((&mut completion).now_or_never().is_none());
        assert_eq!(cx.buffer_text(), text);
        cx.background_executor.set_block_on_ticks(0..=50);
        cx.run_until_parked();
        cx.assert_state(
            &if setting == UseSystemClipboard::OnYank {
                format!("start\nˇAB{suffix}")
            } else {
                format!("start\nBˇA{suffix}")
            },
            Mode::Normal,
        );
        resolver.resolve(Ok(Arc::new(ClipboardSnapshot {
            item: Some(ClipboardItem::new_string(String::from("EXT"))),
            revision: None,
        })));
        completion.await.expect("delete and paste completed");
        cx.run_until_parked();
        cx.assert_state(&format!("start\nBˇA{suffix}"), Mode::Normal);
        cx.update(|_, cx| {
            cx.remove_global::<NativeClipboardCapture>();
            assert_eq!(
                cx.global::<VimGlobals>().last_yank.as_deref(),
                if setting == UseSystemClipboard::OnYank {
                    Some("EXT")
                } else {
                    None
                },
            );
            assert_eq!(
                cx.read_from_clipboard()
                    .and_then(|item| item.text())
                    .as_deref(),
                Some("EXT"),
            );
        });
    }
}

#[gpui::test]
async fn test_on_yank_observation_failure_preserves_internal_registers(cx: &mut TestAppContext) {
    for delayed in [false, true] {
        for error in [
            ClipboardReadError::Unavailable,
            ClipboardReadError::UnsupportedContent,
        ] {
            let mut cx = VimTestContext::new(cx, true).await;
            set_clipboard_setting(&mut cx, UseSystemClipboard::OnYank);
            cx.set_state("ˇone\ntwo", Mode::Normal);
            let (read, resolver) = ClipboardSnapshotRead::pending();
            let resolver = if delayed {
                Some(resolver)
            } else {
                resolver.resolve(Err(error.clone()));
                None
            };
            cx.update(|_, cx| {
                cx.set_global(NativeClipboardCapture(Rc::new(move |_| read.clone())));
            });
            dispatch_keys(&mut cx, "d d");
            let completion = cx.update_editor(|editor, _, cx| editor.input_completion(cx));
            if let Some(resolver) = resolver {
                cx.run_until_parked();
                cx.assert_state("ˇone\ntwo", Mode::Normal);
                resolver.resolve(Err(error));
            }
            completion
                .await
                .expect("clipboard observation must not reject deletion");
            cx.run_until_parked();
            cx.assert_state("ˇtwo", Mode::Normal);
            cx.update(|_, cx| {
                assert_eq!(
                    cx.global::<VimGlobals>()
                        .registers
                        .get(&'"')
                        .map(|register| register.text.as_ref()),
                    Some("one\n")
                );
            });
            dispatch_keys(&mut cx, "shift-p");
            cx.update_editor(|editor, _, cx| editor.input_completion(cx))
                .await
                .expect("unnamed paste must fall back to the internal register");
            cx.run_until_parked();
            cx.assert_state("ˇone\ntwo", Mode::Normal);
            cx.update(|_, cx| {
                cx.remove_global::<NativeClipboardCapture>();
            });
        }
    }
}

#[gpui::test]
async fn test_shared_fifo_native_register_reads(cx: &mut TestAppContext) {
    for cold in [false, true] {
        for (keys, setting, expected) in [
            ("p", UseSystemClipboard::Always, "VA!"),
            ("p", UseSystemClipboard::OnYank, "VA!"),
            ("\" + p", UseSystemClipboard::Never, "VA!"),
            ("i ctrl-r + escape", UseSystemClipboard::Never, "AV!"),
            ("\" + g shift-r l", UseSystemClipboard::Never, "A!"),
            ("\" + g shift-r i w", UseSystemClipboard::Never, "A!"),
        ] {
            let mut cx = VimTestContext::new(cx, true).await;
            set_clipboard_setting(&mut cx, setting);
            cx.set_state("ˇV!", Mode::Normal);
            let source = prepare_source(&mut cx, cold).await;
            cx.update(|window, cx| {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(String::from("Z")));
                copy_source(&source, window, cx);
            });
            dispatch_keys(&mut cx, keys);
            if cold {
                assert_eq!(cx.buffer_text(), "V!", "{keys}");
            }
            cx.background_executor.set_block_on_ticks(0..=50);
            cx.run_until_parked();
            assert_eq!(cx.buffer_text(), expected, "cold={cold}, {keys}");
            assert_eq!(cx.mode(), Mode::Normal);
        }
    }
}

#[gpui::test]
async fn test_shared_fifo_native_register_writes(cx: &mut TestAppContext) {
    for cold in [false, true] {
        for (keys, setting, shared, last_yank) in [
            ("x", UseSystemClipboard::Never, true, None),
            ("x", UseSystemClipboard::OnYank, true, Some("A")),
            ("x", UseSystemClipboard::Always, true, Some("V")),
            ("\" + x", UseSystemClipboard::Never, true, None),
            ("\" a x", UseSystemClipboard::Always, true, None),
            ("\" \" x", UseSystemClipboard::Always, true, None),
            ("\" _ x", UseSystemClipboard::Always, false, None),
            ("d i w", UseSystemClipboard::Never, true, None),
            ("d t !", UseSystemClipboard::Never, true, None),
        ] {
            let mut cx = VimTestContext::new(cx, true).await;
            set_clipboard_setting(&mut cx, setting);
            cx.set_state("ˇV!", Mode::Normal);
            let source = prepare_source(&mut cx, cold).await;
            cx.update(|window, cx| {
                cx.global_mut::<VimGlobals>().last_yank = None;
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(String::from("Z")));
                copy_source(&source, window, cx);
            });
            dispatch_keys(&mut cx, keys);
            assert_eq!(
                cx.buffer_text(),
                if cold && shared { "V!" } else { "!" },
                "{keys}"
            );
            cx.background_executor.set_block_on_ticks(0..=50);
            cx.run_until_parked();
            assert_eq!(cx.buffer_text(), "!", "cold={cold}, {keys}");
            cx.update(|_, cx| {
                assert_eq!(
                    cx.global::<VimGlobals>().last_yank.as_deref(),
                    last_yank,
                    "cold={cold}, {keys}"
                );
                assert_eq!(
                    cx.read_from_clipboard()
                        .and_then(|item| item.text())
                        .as_deref(),
                    Some(
                        if setting == UseSystemClipboard::Always && keys == "x" || keys == "\" + x"
                        {
                            "V"
                        } else {
                            "A"
                        }
                    ),
                    "cold={cold}, {keys}",
                );
            });
        }
    }
}

#[gpui::test]
async fn test_shared_fifo_visual_paste_register_union(cx: &mut TestAppContext) {
    for cold in [false, true] {
        let mut cx = VimTestContext::new(cx, true).await;
        set_clipboard_setting(&mut cx, UseSystemClipboard::Never);
        cx.set_state("«Vˇ»!", Mode::Visual);
        let source = prepare_source(&mut cx, cold).await;
        cx.update(|window, cx| {
            cx.write_to_clipboard(gpui::ClipboardItem::new_string(String::from("Z")));
            copy_source(&source, window, cx);
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            source.update(cx, |editor, cx| {
                editor.schedule_input_continuation(window, cx, |_, cx| {
                    cx.write_to_primary(ClipboardItem::new_string(String::from("P")));
                });
            });
        });
        dispatch_keys(&mut cx, "\" * p");
        if cold {
            assert_eq!(cx.buffer_text(), "V!");
            assert_eq!(cx.mode(), Mode::Visual);
        }
        cx.background_executor.set_block_on_ticks(0..=50);
        cx.run_until_parked();
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        assert_eq!(cx.buffer_text(), "P!");
        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
        assert_eq!(cx.buffer_text(), "A!");
        cx.update(|_, cx| {
            assert_eq!(
                cx.global::<VimGlobals>()
                    .registers
                    .get(&'"')
                    .map(|register| register.text.as_ref()),
                Some("V")
            );
            assert_eq!(cx.global::<VimGlobals>().last_yank.as_deref(), None);
        });
    }
}

#[gpui::test]
async fn test_shared_fifo_macro_cross_editor_clipboard(cx: &mut TestAppContext) {
    for cold in [false, true] {
        for external in [false, true] {
            let mut cx = VimTestContext::new(cx, true).await;
            set_clipboard_setting(&mut cx, UseSystemClipboard::Never);
            cx.set_state("ˇV!", Mode::Normal);
            let source = prepare_source(&mut cx, cold).await;
            cx.update(|window, cx| {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(String::from("Z")));
                let mut actions = Vec::new();
                for action in [
                    PushRegister.boxed_clone(),
                    Paste::default().boxed_clone(),
                    motion::StartOfLine {
                        display_lines: false,
                    }
                    .boxed_clone(),
                    PushRegister.boxed_clone(),
                    YankToEndOfLine.boxed_clone(),
                    workspace::pane::ActivateItem(1).boxed_clone(),
                    PushRegister.boxed_clone(),
                    Paste::default().boxed_clone(),
                    workspace::pane::ActivateItem(0).boxed_clone(),
                    PushRegister.boxed_clone(),
                    Paste::default().boxed_clone(),
                ] {
                    let selects_register = action.as_any().is::<PushRegister>();
                    actions.push(ReplayableAction::Action(action));
                    if selects_register {
                        actions.push(ReplayableAction::Insertion {
                            text: "+".into(),
                            utf16_range_to_replace: None,
                        });
                    }
                }
                cx.global_mut::<VimGlobals>()
                    .recordings
                    .insert('a', actions);
                cx.global_mut::<VimGlobals>().last_recorded_register = Some('a');
                copy_source(&source, window, cx);
                window
                    .focused(cx)
                    .unwrap()
                    .dispatch_action(&ReplayLastRecording, window, cx);
            });
            cx.update(|_, cx| {
                source.update(cx, |editor, cx| {
                    editor
                        .write_to_clipboard(gpui::ClipboardItem::new_string(String::from("C")), cx)
                        .expect("later native publication");
                });
                if external {
                    cx.write_to_clipboard(gpui::ClipboardItem::new_string(String::from("E")));
                }
            });
            cx.background_executor.set_block_on_ticks(0..=50);
            cx.run_until_parked();
            let pasted = if cold {
                "A"
            } else if external {
                "E"
            } else {
                "C"
            };
            assert_eq!(
                cx.buffer_text(),
                format!("VV{pasted}!{pasted}!"),
                "cold={cold}, external={external}"
            );
            cx.update(|_, cx| {
                assert_eq!(
                    source.read(cx).text(cx),
                    format!("AV{pasted}!\n{}", "x".repeat(2_000)),
                    "cold={cold}, external={external}"
                );
                assert_eq!(
                    cx.read_from_clipboard()
                        .and_then(|item| item.text())
                        .as_deref(),
                    Some(format!("V{pasted}!").as_str())
                );
                assert!(cx.global::<VimGlobals>().replayer.is_none());
                assert!(!cx.global::<VimGlobals>().dot_replaying);
                assert_eq!(
                    source
                        .read(cx)
                        .addon::<VimAddon>()
                        .unwrap()
                        .entity
                        .read(cx)
                        .mode,
                    Mode::Normal
                );
            });
        }
    }
}

#[gpui::test]
async fn test_shared_registers_across_editors(cx: &mut TestAppContext) {
    let line = "x".repeat(2_000);
    let yanked = format!("ab\n{line}\n");
    for cold in [false, true] {
        for (source_keys, destination_keys, register, expected_register, expected_text) in [
            (
                "\" a y j",
                "\" a p",
                'a',
                yanked.clone(),
                format!("KEEP\nab\n{line}"),
            ),
            (
                "\" a y j",
                "p",
                '"',
                yanked.clone(),
                format!("KEEP\nab\n{line}"),
            ),
            (
                "y j",
                "\" 0 p",
                '0',
                yanked.clone(),
                format!("KEEP\nab\n{line}"),
            ),
            (
                "d j",
                "\" 1 p",
                '1',
                yanked.clone(),
                format!("KEEP\nab\n{line}"),
            ),
            (
                "d j",
                "\" 9 p",
                '9',
                String::from("OLD"),
                String::from("KOLDEEP"),
            ),
            (
                "\" A y j",
                "\" a p",
                'a',
                format!("OLD{yanked}"),
                format!("KEEP\nOLDab\n{line}"),
            ),
            (
                "\" a y j",
                "\" A y y \" a p",
                'a',
                format!("{yanked}KEEP\n"),
                format!("KEEP\nab\n{line}\nKEEP"),
            ),
        ] {
            let mut cx = VimTestContext::new(cx, true).await;
            set_clipboard_setting(&mut cx, UseSystemClipboard::Never);
            cx.set_state("ˇKEEP", Mode::Normal);
            let destination = cx.editor.clone();
            let (workspace, source) = add_background_editor("ab\nshort", &mut cx);
            cx.update(|window, cx| {
                source.update(cx, |editor, cx| {
                    editor.set_soft_wrap_mode(settings::SoftWrap::None, cx);
                });
                let registers = &mut cx.global_mut::<VimGlobals>().registers;
                for register in ['a', '"', '0'].into_iter().chain('1'..='8') {
                    registers.insert(register, Register::from(String::from("OLD")));
                }
                registers.insert('9', Register::from(String::from("STALE")));
                workspace.update(cx, |workspace, cx| {
                    assert!(workspace.activate_item(&source, true, true, window, cx));
                });
                window.draw(cx).clear(cx);
            });
            cx.run_until_parked();
            if cold {
                cx.background_executor.set_block_on_ticks(0..=0);
            }
            cx.update(|window, cx| {
                source.update(cx, |editor, cx| {
                    editor.set_text(format!("ab\n{line}"), window, cx);
                    editor.change_selections(
                        SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
                        },
                    );
                });
            });
            if !cold {
                dispatch_keys(&mut cx, "j k");
                cx.run_until_parked();
            }
            dispatch_keys(&mut cx, source_keys);
            let mut source_completion =
                cx.update(|_, cx| source.update(cx, |editor, cx| editor.input_completion(cx)));
            if cold {
                assert!((&mut source_completion).now_or_never().is_none());
                cx.update(|_, cx| {
                    assert_eq!(
                        cx.global::<VimGlobals>()
                            .registers
                            .get(&register)
                            .map(|register| register.text.as_ref()),
                        Some(if register == '9' { "STALE" } else { "OLD" }),
                    );
                });
            }
            cx.update(|window, cx| {
                workspace.update(cx, |workspace, cx| {
                    assert!(workspace.activate_item(&destination, true, true, window, cx));
                });
                window.draw(cx).clear(cx);
            });
            if cold {
                assert!((&mut source_completion).now_or_never().is_none());
            }
            dispatch_keys(&mut cx, destination_keys);
            let destination_completion =
                cx.update_editor(|editor, _, cx| editor.input_completion(cx));
            if cold {
                assert!((&mut source_completion).now_or_never().is_none());
                assert_eq!(cx.buffer_text(), "KEEP");
            }
            cx.background_executor.set_block_on_ticks(0..=50);
            source_completion
                .await
                .expect("source register write completed");
            destination_completion
                .await
                .expect("destination register access completed");
            cx.run_until_parked();
            cx.update(|_, cx| {
                assert_eq!(
                    cx.global::<VimGlobals>()
                        .registers
                        .get(&register)
                        .map(|register| register.text.as_ref()),
                    Some(expected_register.as_str()),
                    "cold={cold}, {source_keys}, {destination_keys}",
                );
            });
            assert_eq!(
                cx.buffer_text(),
                expected_text,
                "cold={cold}, {source_keys}, {destination_keys}"
            );
        }
    }
}

#[gpui::test]
async fn test_register_native_requirements(cx: &mut TestAppContext) {
    let mut cx = VimTestContext::new(cx, true).await;
    let no_native = Some((None, None));
    let contents = Some((Some(ClipboardAccess::Read), None));
    let write = Some((Some(ClipboardAccess::Write), None));
    let observation = Some((Some(ClipboardAccess::Observe), None));
    let (primary_contents, primary_write) = if cfg!(any(target_os = "linux", target_os = "freebsd"))
    {
        (
            Some((None, Some(ClipboardAccess::Read))),
            Some((None, Some(ClipboardAccess::Write))),
        )
    } else {
        (contents, write)
    };
    for (setting, default_read, default_delete, default_yank) in [
        (UseSystemClipboard::Never, no_native, no_native, no_native),
        (UseSystemClipboard::OnYank, observation, observation, write),
        (UseSystemClipboard::Always, contents, write, write),
    ] {
        set_clipboard_setting(&mut cx, setting);
        cx.update(|_, cx| {
            for (register, read, delete, yank) in [
                (None, default_read, default_delete, default_yank),
                (Some('"'), default_read, no_native, no_native),
                (Some('a'), no_native, no_native, no_native),
                (Some('A'), no_native, no_native, no_native),
                (Some('0'), no_native, no_native, no_native),
                (Some('1'), no_native, no_native, no_native),
                (Some('9'), no_native, no_native, no_native),
                (Some('-'), no_native, no_native, no_native),
                (Some('/'), no_native, None, None),
                (Some('+'), contents, write, write),
                (Some('*'), primary_contents, primary_write, primary_write),
                (Some('_'), None, None, None),
                (Some('%'), None, None, None),
            ] {
                assert_eq!(
                    VimGlobals::register_read_access(register, cx)
                        .map(|required| (required.clipboard, required.primary)),
                    read,
                    "{setting:?}, {register:?}",
                );
                for (is_yank, expected) in [(false, delete), (true, yank)] {
                    assert_eq!(
                        VimGlobals::register_write_access(register, is_yank, cx)
                            .map(|required| (required.clipboard, required.primary)),
                        expected,
                        "{setting:?}, {register:?}, is_yank={is_yank}",
                    );
                }
            }
        });
    }
    cx.set_state("«Vˇ»!", Mode::Visual);
    cx.update_editor(|editor, _, cx| {
        let vim = editor.addon::<VimAddon>().unwrap().entity.clone();
        vim.update(cx, |vim, cx| {
            vim.selected_register = Some('*');
            assert_eq!(
                vim.action_register_access(&Paste::default(), cx)
                    .map(|required| (required.clipboard, required.primary)),
                if cfg!(any(target_os = "linux", target_os = "freebsd")) {
                    Some((Some(ClipboardAccess::Write), Some(ClipboardAccess::Read)))
                } else {
                    contents
                },
            );
        });
    });
}

fn set_clipboard_setting(cx: &mut VimTestContext, setting: UseSystemClipboard) {
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings(cx, |settings| {
            settings.vim.get_or_insert_default().use_system_clipboard = Some(setting);
        });
    });
    cx.update(|_, cx| {
        cx.global_mut::<VimGlobals>()
            .registers
            .insert('"', Register::from(String::from("internal")));
    });
}

fn assert_rejected_command_cleared(cx: &mut VimTestContext) {
    cx.update_editor(|editor, _, cx| {
        let vim = editor
            .addon::<VimAddon>()
            .expect("Vim addon")
            .entity
            .read(cx);
        assert!(vim.operator_stack.is_empty());
        assert_eq!(vim.selected_register, None);
        assert!(!vim.temp_mode);
        assert!(!vim.exit_temporary_mode);
        let globals = cx.global::<VimGlobals>();
        assert_eq!(globals.pre_count, None);
        assert_eq!(globals.post_count, None);
        assert!(!globals.forced_motion);
        assert!(!globals.dot_recording);
        assert!(!globals.stop_recording_after_next_action);
        assert!(globals.recording_actions.is_empty());
        assert_eq!(globals.recording_count, None);
        assert_eq!(globals.recording_register_for_dot, None);
        assert!(!globals.dot_replaying);
        assert!(globals.replayer.is_none());
    });
}

async fn prepare_source(cx: &mut VimTestContext, cold: bool) -> Entity<Editor> {
    let (_, source) = add_background_editor("A\nshort", cx);
    cx.run_until_parked();
    cx.background_executor.set_block_on_ticks(0..=0);
    let mut completion = cx.update(|window, cx| {
        source.update(cx, |editor, cx| {
            editor.set_soft_wrap_mode(settings::SoftWrap::None, cx);
            editor.set_text(format!("A\n{}", "x".repeat(2_000)), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 0)..Point::new(0, 0)])
            });
            editor.move_down(&zed_actions::editor::MoveDown, window, cx);
            editor.input_completion(cx)
        })
    });
    assert!((&mut completion).now_or_never().is_none());
    if !cold {
        cx.background_executor.set_block_on_ticks(0..=50);
        completion.await.expect("source geometry completed");
        cx.run_until_parked();
    }
    source
}

fn copy_source(source: &Entity<Editor>, window: &mut Window, cx: &mut App) {
    let workspace = workspace::Workspace::for_window(window, cx).unwrap();
    let target = workspace
        .read(cx)
        .active_item(cx)
        .unwrap()
        .act_as::<Editor>(cx)
        .unwrap();
    workspace.update(cx, |workspace, cx| {
        assert!(workspace.activate_item(source, true, true, window, cx));
    });
    window.draw(cx).clear(cx);
    source.update(cx, |editor, cx| {
        let select = |editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 0)..Point::new(0, 1)])
            });
        };
        if !editor.defer_input(select, window, cx) {
            select(editor, window, cx);
        }
    });
    source
        .focus_handle(cx)
        .dispatch_action(&editor::actions::Copy, window, cx);
    source.update(cx, |editor, cx| {
        let collapse = |editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 0)..Point::new(0, 0)])
            });
        };
        if !editor.defer_input(collapse, window, cx) {
            collapse(editor, window, cx);
        }
    });
    workspace.update(cx, |workspace, cx| {
        assert!(workspace.activate_item(&target, true, true, window, cx));
    });
    window.draw(cx).clear(cx);
}
