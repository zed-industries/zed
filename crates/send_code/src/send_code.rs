mod code_getter;
mod eval;
mod senders;
mod settings;

use ::settings::Settings;
use editor::{Editor, SelectionEffects};
use gpui::{App, actions, prelude::*};
use settings::SendCodeSettings;

pub use settings::SendCodeSettingsContent;

actions!(
    send_code,
    [
        /// Send the current selection to the active terminal.
        SendSelectionToTerminal,
        /// Send the smallest evaluable block at the cursor to the active terminal.
        SendEvalAtCursorToTerminal,
    ]
);

pub fn init(cx: &mut App) {
    SendCodeSettings::register(cx);

    cx.observe_new(
        move |editor: &mut Editor, window, cx: &mut Context<Editor>| {
            if window.is_none() {
                return;
            }
            if !editor.buffer().read(cx).is_singleton() {
                return;
            }

            let editor_handle = cx.entity().downgrade();

            editor
                .register_action({
                    let editor_handle = editor_handle.clone();
                    move |_: &SendSelectionToTerminal, window, cx| {
                        if !SendCodeSettings::enabled(cx) {
                            return;
                        }
                        send_selection_action(editor_handle.clone(), window, cx);
                    }
                })
                .detach();

            editor
                .register_action(move |_: &SendEvalAtCursorToTerminal, window, cx| {
                    if !SendCodeSettings::enabled(cx) {
                        return;
                    }
                    send_eval_action(editor_handle.clone(), window, cx);
                })
                .detach();
        },
    )
    .detach();
}

fn send_selection_action(
    editor: gpui::WeakEntity<Editor>,
    _window: &mut gpui::Window,
    cx: &mut App,
) {
    let Some(editor_entity) = editor.upgrade() else {
        return;
    };

    let payload = editor_entity.update(cx, |editor, cx| code_getter::get_selection(editor, cx));

    if let Some(payload) = payload {
        send_payload(&payload, &editor_entity, cx);
    }
}

fn send_eval_action(editor: gpui::WeakEntity<Editor>, window: &mut gpui::Window, cx: &mut App) {
    let Some(editor_entity) = editor.upgrade() else {
        return;
    };

    let payload =
        editor_entity.update(cx, |editor, cx| code_getter::get_eval_at_cursor(editor, cx));

    if let Some(payload) = payload {
        send_payload(&payload, &editor_entity, cx);

        if let Some(advance_to) = payload.advance_to {
            editor_entity.update(cx, |editor, cx| {
                editor.change_selections(SelectionEffects::default(), window, cx, |s| {
                    s.select_ranges([advance_to..advance_to]);
                });
            });
        }
    }
}

fn send_payload(payload: &code_getter::CodePayload, editor: &gpui::Entity<Editor>, cx: &mut App) {
    let settings = SendCodeSettings::get_global(cx).clone();
    let Some(workspace) = editor.read(cx).workspace().map(|ws| ws.downgrade()) else {
        return;
    };

    senders::send_to_terminal(&payload.text, settings.bracketed_paste, &workspace, cx);
}
