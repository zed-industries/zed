use channel::ChannelStore;
use client::{ChannelId, DevServerId};
use editor::Editor;
use gpui::{
    AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model, View, ViewContext,
};
use ui::prelude::*;
use workspace::{notifications::DetachAndPromptErr, ModalView};

pub struct DevServerModal {
    focus_handle: FocusHandle,
    channel_store: Model<ChannelStore>,
    channel_id: ChannelId,
    name_editor: View<Editor>,
    path_editor: View<Editor>,
}

impl DevServerModal {
    pub fn new(
        channel_store: Model<ChannelStore>,
        channel_id: ChannelId,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let name_editor = cx.new_view(|cx| Editor::single_line(cx));
        let path_editor = cx.new_view(|cx| Editor::single_line(cx));

        Self {
            focus_handle: cx.focus_handle(),
            channel_store,
            channel_id,
            name_editor,
            path_editor,
        }
    }

    pub fn on_create(&self, cx: &mut ViewContext<Self>) {
        let channel_id = self.channel_id;
        let name = self.name_editor.read(cx).text(cx).trim().to_string();
        let path = self.path_editor.read(cx).text(cx).trim().to_string();

        if name == "" {
            return;
        }
        if path == "" {
            return;
        }

        let task = self.channel_store.update(cx, |store, cx| {
            store.create_remote_project(channel_id, DevServerId(1), name, path, cx)
        });

        task.detach_and_prompt_err("Failed to create remote project", cx, |_, _| None);
    }
}
impl ModalView for DevServerModal {}

impl FocusableView for DevServerModal {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for DevServerModal {}

impl Render for DevServerModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let channel_store = self.channel_store.read(cx);
        let dev_servers = channel_store.dev_servers_for_id(self.channel_id);
        div()
            .track_focus(&self.focus_handle)
            .elevation_2(cx)
            .key_context("DevServerModal")
            // .on_action(cx.listener(Self::cancel))
            // .on_action(cx.listener(Self::confirm))
            .w_96()
            .child(
                v_flex()
                    .px_1()
                    .pt_0p5()
                    .gap_px()
                    .child(
                        v_flex()
                            .py_0p5()
                            .px_1()
                            .child(div().px_1().py_0p5().child("Add Remote Project:")),
                    )
                    .child(h_flex().child("Name:").child(self.name_editor.clone()))
                    .child("Dev Server:")
                    .children(dev_servers.iter().map(|dev_server| dev_server.name.clone()))
                    .child(h_flex().child("Path:").child(self.path_editor.clone()))
                    .child(
                        Button::new("create-button", "Create")
                            .on_click(cx.listener(|this, _event, cx| this.on_create(cx))),
                    ),
            )
    }
}
