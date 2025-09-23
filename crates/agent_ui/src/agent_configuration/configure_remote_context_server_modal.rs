use crate::agent_configuration::AddContextServer;
use gpui::{
    actions, App, AppContext, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle,
    Focusable, Global, Subscription, Task, WeakEntity, Window,
};
use language::LanguageRegistry;
use project::{
    context_server_store::ContextServerStore,
    project_settings::{ContextServerSettings, ProjectSettings},
    Project,
};
use settings::{Settings, SettingsStore, update_settings_file};
use std::sync::Arc;
use ui::{
    prelude::*, Button, ButtonStyle, Checkbox, Divider, Editor, EditorWithAction, Icon,
    IconButton, IconName, Label, Select, Switch, SwitchColor, TextField,
};
use workspace::Workspace;

pub struct ConfigureRemoteContextServerModal {
    fs: Arc<fs::Fs>,
    server_id: Option<ContextServerId>,
    server_name_editor: Entity<Editor>,
    server_url_editor: Entity<Editor>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
}

actions!(
    remote_context_server_modal,
    [Submit, Dismiss, AddContextServer]
);

impl ConfigureRemoteContextServerModal {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        server_id: Option<ContextServerId>,
        cx: &mut Context<Self>,
    ) -> Self {
        let fs = workspace.read(cx).app_state().fs.clone();
        let server_name_editor = cx.new(|cx| Editor::single_line(Some(cx.text_style()), cx));
        let server_url_editor = cx.new(|cx| Editor::single_line(Some(cx.text_style()), cx));
        let focus_handle = cx.focus_handle();

        if let Some(server_id) = &server_id {
            server_name_editor.update(cx, |editor, cx| {
                editor.set_text(server_id.0.to_string(), cx);
            });

            if let Some(project) = workspace.read(cx).project().as_ref() {
                let settings = ProjectSettings::get_global(cx);
                if let Some(ContextServerSettings::Remote { url, .. }) =
                    settings.context_servers.get(&server_id.0)
                {
                    server_url_editor.update(cx, |editor, cx| {
                        editor.set_text(url.clone(), cx);
                    });
                }
            }
        }

        Self {
            fs,
            server_id,
            server_name_editor,
            server_url_editor,
            workspace,
            focus_handle,
        }
    }

    pub fn toggle(
        workspace: &mut Workspace,
        _: &super::AddRemoteContextServer,
        window: &mut Window,
        cx: &mut AppContext,
    ) {
        window.toggle_modal(cx, |cx| {
            Self::new(workspace.weak_handle(), None, cx)
        });
    }

    pub fn show_modal_for_existing_server(
        server_id: ContextServerId,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> Task<()> {
        let task = window
            .new_modal(cx, |cx| Self::new(workspace, Some(server_id), cx))
            .log_err();
        cx.spawn(|_, _| async move {
            task.await;
        })
    }

    fn submit(&mut self, _: &Submit, window: &mut Window, cx: &mut AppContext) {
        let server_name = self.server_name_editor.read(cx).text(cx);
        let server_url = self.server_url_editor.read(cx).text(cx);

        if server_name.is_empty() || server_url.is_empty() {
            return;
        }

        let settings_path = SettingsStore::global(cx).read(cx).user_settings_file_path();
        let fs = self.fs.clone();
        let server_id = self.server_id.clone();
        cx.spawn(|_cx| async move {
            update_settings_file::<ProjectSettings>(fs, settings_path, |settings| {
                if let Some(server_id) = server_id {
                    if server_id.0.as_ref() != server_name {
                        settings.context_servers.remove(&server_id.0);
                    }
                }

                settings
                    .context_servers
                    .insert(server_name.into(), ContextServerSettings::Remote {
                        enabled: true,
                        url: server_url,
                    });
            })
            .await
        })
        .detach_and_log_err(cx);

        self.dismiss(&Dismiss, window, cx);
    }
}

impl Focusable for ConfigureRemoteContextServerModal {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ConfigureRemoteContextServerModal {
    fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("ConfigureRemoteContextServerModal")
            .on_action(cx.listener(Self::submit))
            .on_action(cx.listener(Self::dismiss))
            .w_96()
            .gap_4()
            .child(
                v_flex()
                    .gap_2()
                    .child(Label::new("Server Name"))
                    .child(TextField::new(self.server_name_editor.clone())),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(Label::new("Server URL"))
                    .child(TextField::new(self.server_url_editor.clone())),
            )
            .child(
                h_flex()
                    .justify_end()
                    .gap_2()
                    .child(Button::new("cancel", "Cancel").on_click(cx.listener(Self::dismiss)))
                    .child(Button::new("submit", "Add Server").on_click(cx.listener(Self::submit))),
            )
    }
}

impl Modal for ConfigureRemoteContextServerModal {
    fn on_before_dismiss(&mut self, _cx: &mut AppContext) -> bool {
        false
    }

    fn on_after_dismiss(&mut self, _action: &Self::Action, _cx: &mut AppContext) {}
}
