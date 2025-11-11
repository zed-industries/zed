use std::str::FromStr;

use editor::Editor;
use git::RemoteUrl;
use gpui::{AppContext, DismissEvent, Entity, EventEmitter, Focusable, Styled};
use project::git_store::Repository;
use ui::{
    ActiveTheme, App, Context, InteractiveElement, IntoElement, Label, LabelCommon, ParentElement,
    Render, StyledExt, StyledTypography, Window, div, v_flex,
};
use util::maybe;
use workspace::{ModalView, Workspace, notifications::DetachAndPromptErr};
use zeroize::Zeroize;

use crate::git_panel::GitPanel;

pub(crate) struct RemoteModal {
    repo: Entity<Repository>,
    editor_remote_name: Entity<Editor>,
    editor_remote_url: Entity<Editor>,
}

impl EventEmitter<DismissEvent> for RemoteModal {}
impl ModalView for RemoteModal {}
impl Focusable for RemoteModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.editor_remote_name.focus_handle(cx)
    }
}

impl RemoteModal {
    pub fn new(repo: Entity<Repository>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let editor_remote_name = cx.new(|cx| {
            let editor = Editor::single_line(window, cx);
            editor
        });
        let editor_remote_url = cx.new(|cx| {
            let editor = Editor::single_line(window, cx);
            editor
        });
        Self {
            repo,
            editor_remote_name,
            editor_remote_url,
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        maybe!({
            let mut remote_name = self.editor_remote_name.update(cx, |this, cx| {
                let text = this.text(cx);
                this.clear(window, cx);
                text
            });
            let mut remote_url = self.editor_remote_url.update(cx, |this, cx| {
                let text = this.text(cx);
                this.clear(window, cx);
                text
            });
            let url = RemoteUrl::from_str(&remote_url).ok()?;
            let repo = self.repo.clone();
            let new_remote_name = remote_name.clone();
            cx.spawn(async move |_, cx| {
                repo.update(cx, |repo, _| {
                    repo.create_remote(new_remote_name.clone(), url.into())
                })?
                .await??;
                repo.update(cx, |repo, _| repo.change_remote(new_remote_name))?
                    .await??;

                Ok(())
            })
            .detach_and_prompt_err(
                "Failed to create remote",
                window,
                cx,
                |e, _, _| Some(e.to_string()),
            );
            remote_url.zeroize();
            remote_name.zeroize();
            Some(())
        });

        cx.emit(DismissEvent);
    }

    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let Some(git_panel) = workspace.panel::<GitPanel>(cx) else {
            return;
        };
        let Some(active_repository) = git_panel.read(cx).active_repository.clone() else {
            return;
        };

        workspace.open_panel::<GitPanel>(window, cx);
        workspace.toggle_modal(window, cx, move |window, cx| {
            RemoteModal::new(active_repository, window, cx)
        });
    }
}

impl Render for RemoteModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RemoteModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .size_full()
            .child(
                div()
                    .font_buffer(cx)
                    .text_buffer(cx)
                    .py_2()
                    .px_3()
                    .bg(cx.theme().colors().editor_background)
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .size_full()
                    .overflow_hidden()
                    .child(Label::new("Remote name: ").mr_2())
                    .child(self.editor_remote_name.clone()),
            )
            .child(
                div()
                    .font_buffer(cx)
                    .text_buffer(cx)
                    .py_2()
                    .px_3()
                    .bg(cx.theme().colors().editor_background)
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .size_full()
                    .overflow_hidden()
                    .child(Label::new("Remote URL: ").mr_2())
                    .child(self.editor_remote_url.clone()),
            )
    }
}
