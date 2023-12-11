use crate::Vim;
use editor::{Editor, EditorEvent};
use gpui::{AppContext, Entity, EntityId, View, ViewContext, WindowContext};

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|_, cx: &mut ViewContext<Editor>| {
        let editor = cx.view().clone();
        cx.subscribe(&editor, |_, editor, event: &EditorEvent, cx| match event {
            EditorEvent::Focused => cx.window_context().defer(|cx| focused(editor, cx)),
            EditorEvent::Blurred => cx.window_context().defer(|cx| blurred(editor, cx)),
            _ => {}
        })
        .detach();

        let id = cx.view().entity_id();
        cx.on_release(move |_, cx| released(id, cx)).detach();
    })
    .detach();
}

fn focused(editor: View<Editor>, cx: &mut WindowContext) {
    if Vim::read(cx).active_editor.clone().is_some() {
        Vim::update(cx, |vim, cx| {
            vim.update_active_editor(cx, |previously_active_editor, cx| {
                vim.unhook_vim_settings(previously_active_editor, cx)
            });
        });
    }

    Vim::update(cx, |vim, cx| {
        vim.set_active_editor(editor.clone(), cx);
    });
}

fn blurred(editor: View<Editor>, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.workspace_state.recording = false;
        vim.workspace_state.recorded_actions.clear();
        if let Some(previous_editor) = vim.active_editor.clone() {
            if previous_editor
                .upgrade()
                .is_some_and(|previous| previous == editor.clone())
            {
                vim.clear_operator(cx);
                vim.active_editor = None;
                vim.editor_subscription = None;
            }
        }

        editor.update(cx, |editor, cx| vim.unhook_vim_settings(editor, cx))
    });
}

fn released(entity_id: EntityId, cx: &mut WindowContext) {
    Vim::update(cx, |vim, _| {
        if vim
            .active_editor
            .as_ref()
            .is_some_and(|previous| previous.entity_id() == entity_id)
        {
            vim.active_editor = None;
            vim.editor_subscription = None;
        }
        vim.editor_states.remove(&entity_id)
    });
}

// #[cfg(test)]
// mod test {
//     use crate::{test::VimTestContext, Vim};
//     use editor::Editor;
//     use gpui::{Context, Entity};
//     use language::Buffer;

//     // regression test for blur called with a different active editor
//     #[gpui::test]
//     async fn test_blur_focus(cx: &mut gpui::TestAppContext) {
//         let mut cx = VimTestContext::new(cx, true).await;

//         let buffer = cx.build_model(|_| Buffer::new(0, 0, "a = 1\nb = 2\n"));
//         let window2 = cx.add_window(|cx| Editor::for_buffer(buffer, None, cx));
//         let editor2 = cx
//             .update(|cx| window2.update(cx, |editor, cx| cx.view()))
//             .unwrap();

//         cx.update(|cx| {
//             let vim = Vim::read(cx);
//             assert_eq!(
//                 vim.active_editor.unwrap().entity_id().unwrap(),
//                 editor2.entity_id()
//             )
//         });

//         // no panic when blurring an editor in a different window.
//         cx.update_editor(|editor1, cx| {
//             editor1.focus_out(cx.handle().into_any(), cx);
//         });
//     }
// }
