use crate::{
    PushRegister, UseSystemClipboard, VimAddon, motion,
    normal::{YankToEndOfLine, paste::Paste, repeat::ReplayLastRecording},
    state::{Mode, Register, ReplayableAction, VimGlobals},
    test::{VimTestContext, add_background_editor, dispatch_keys},
};
use editor::{Editor, SelectionEffects};
use futures::FutureExt;
use gpui::{Action, App, Context, Entity, Focusable, TestAppContext, Window};
use language::Point;
use settings::SettingsStore;

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
        for (keys, setting, native, last_yank) in [
            ("x", UseSystemClipboard::Never, true, Some("A")),
            ("x", UseSystemClipboard::OnYank, true, Some("A")),
            ("x", UseSystemClipboard::Always, true, Some("V")),
            ("\" + x", UseSystemClipboard::Never, true, None),
            ("\" a x", UseSystemClipboard::Always, false, None),
            ("\" \" x", UseSystemClipboard::Always, false, None),
            ("\" _ x", UseSystemClipboard::Always, false, None),
            ("d i w", UseSystemClipboard::Never, true, Some("A")),
            ("d t !", UseSystemClipboard::Never, true, Some("A")),
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
                if cold && native { "V!" } else { "!" },
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
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            cx.write_to_primary(gpui::ClipboardItem::new_string(String::from("P")));
            copy_source(&source, window, cx);
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
            assert_eq!(cx.global::<VimGlobals>().last_yank.as_deref(), Some("A"));
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
            assert_eq!(
                cx.buffer_text(),
                "VVA!A!",
                "cold={cold}, external={external}"
            );
            cx.update(|_, cx| {
                assert_eq!(
                    source.read(cx).text(cx),
                    format!("AVA!\n{}", "x".repeat(2_000)),
                    "cold={cold}, external={external}"
                );
                assert_eq!(
                    cx.read_from_clipboard()
                        .and_then(|item| item.text())
                        .as_deref(),
                    Some(if external { "E" } else { "C" })
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
