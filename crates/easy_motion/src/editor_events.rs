use crate::EasyMotion;
use editor::{Editor, EditorEvent};
use gpui::{AppContext, Entity, EntityId, View, ViewContext, WindowContext};

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|_, cx: &mut ViewContext<Editor>| {
        let editor = cx.view().clone();
        cx.subscribe(&editor, |_, editor, event: &EditorEvent, cx| match event {
            EditorEvent::Focused => cx.window_context().defer(|cx| focused(editor, cx)),
            _ => {}
        })
        .detach();

        let id = cx.view().entity_id();
        cx.on_release(move |_, _, cx| released(id, cx)).detach();
    })
    .detach();
}
fn focused(editor: View<Editor>, cx: &mut WindowContext) {
    EasyMotion::update(cx, |easy, _cx| {
        if !easy.enabled {
            return;
        }
        easy.activate_editor(editor.clone());
    });
}

fn released(entity_id: EntityId, cx: &mut AppContext) {
    EasyMotion::update(cx, |easy, _cx| {
        if easy
            .active_editor
            .as_ref()
            .is_some_and(|previous| previous.entity_id() == entity_id)
        {
            easy.active_editor = None;
        }
        easy.editor_states.remove(&entity_id)
    });
}
