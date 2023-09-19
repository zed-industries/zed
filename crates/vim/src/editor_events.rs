use crate::{Vim, VimEvent};
use editor::{EditorBlurred, EditorFocused, EditorReleased};
use gpui::AppContext;

pub fn init(cx: &mut AppContext) {
    cx.subscribe_global(focused).detach();
    cx.subscribe_global(blurred).detach();
    cx.subscribe_global(released).detach();
}

fn focused(EditorFocused(editor): &EditorFocused, cx: &mut AppContext) {
    if let Some(previously_active_editor) = Vim::read(cx).active_editor.clone() {
        previously_active_editor.window().update(cx, |cx| {
            Vim::update(cx, |vim, cx| {
                vim.update_active_editor(cx, |previously_active_editor, cx| {
                    vim.unhook_vim_settings(previously_active_editor, cx)
                });
            });
        });
    }

    editor.window().update(cx, |cx| {
        Vim::update(cx, |vim, cx| {
            vim.set_active_editor(editor.clone(), cx);
            if vim.enabled {
                cx.emit_global(VimEvent::ModeChanged {
                    mode: vim.state().mode,
                });
            }
        });
    });
}

fn blurred(EditorBlurred(editor): &EditorBlurred, cx: &mut AppContext) {
    editor.window().update(cx, |cx| {
        Vim::update(cx, |vim, cx| {
            vim.workspace_state.recording = false;
            vim.workspace_state.recorded_actions.clear();
            if let Some(previous_editor) = vim.active_editor.clone() {
                if previous_editor == editor.clone() {
                    vim.clear_operator(cx);
                    vim.active_editor = None;
                }
            }

            editor.update(cx, |editor, cx| vim.unhook_vim_settings(editor, cx))
        });
    });
}

fn released(EditorReleased(editor): &EditorReleased, cx: &mut AppContext) {
    editor.window().update(cx, |cx| {
        cx.update_default_global(|vim: &mut Vim, _| {
            if let Some(previous_editor) = vim.active_editor.clone() {
                if previous_editor == editor.clone() {
                    vim.active_editor = None;
                }
            }
            vim.editor_states.remove(&editor.id())
        });
    });
}

#[cfg(test)]
mod test {
    use crate::{test::VimTestContext, Vim};
    use editor::Editor;
    use gpui::View;
    use language::Buffer;

    // regression test for blur called with a different active editor
    #[gpui::test]
    async fn test_blur_focus(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        let buffer = cx.add_model(|_| Buffer::new(0, 0, "a = 1\nb = 2\n"));
        let window2 = cx.add_window(|cx| Editor::for_buffer(buffer, None, cx));
        let editor2 = cx.read(|cx| window2.root(cx)).unwrap();

        cx.update(|cx| {
            let vim = Vim::read(cx);
            assert_eq!(vim.active_editor.unwrap().id(), editor2.id())
        });

        // no panic when blurring an editor in a different window.
        cx.update_editor(|editor1, cx| {
            editor1.focus_out(cx.handle().into_any(), cx);
        });
    }
}
