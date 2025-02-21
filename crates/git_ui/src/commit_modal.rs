#![allow(unused, dead_code)]

use crate::git_panel::commit_message_editor;
use crate::repository_selector::RepositorySelector;
use anyhow::Result;
use git::Commit;
use language::Buffer;
use panel::{panel_editor_container, panel_editor_style, panel_filled_button, panel_icon_button};
use settings::Settings;
use theme::ThemeSettings;
use ui::{prelude::*, Tooltip};

use editor::{Editor, EditorElement, EditorMode, MultiBuffer};
use gpui::*;
use project::git::Repository;
use project::{Fs, Project};
use std::sync::Arc;
use workspace::{ModalView, Workspace};

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        CommitModal::register(workspace, window, cx)
    })
    .detach();
}

pub struct CommitModal {
    git_panel: Entity<GitPanel>,
    commit_editor: Entity<Editor>,
}

impl Focusable for CommitModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.commit_editor.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for CommitModal {}
impl ModalView for CommitModal {}

impl CommitModal {
    pub fn register(workspace: &mut Workspace, _: &mut Window, cx: &mut Context<Workspace>) {
        workspace.register_action(|workspace, _: &Commit, window, cx| {
            let Some(git_panel) = workspace.panel::<GitPanel>(cx) else {
                return;
            };
            workspace.toggle_modal(window, cx, move |window, cx| {
                CommitModal::new(git_panel, window, cx)
            })
        });
    }

    pub fn new(
        git_panel: Entity<GitPanel>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let buffer = git_panel.read(cx).commit_message_buffer().clone();
        let commit_editor = cx.new(|cx| {
            commit_message_editor(buffer, project.clone(), false, window, cx)
        });

        Self {
            fs,
            project,
            commit_editor,
            should_commit_all: false,
        }
    }

    pub fn render_commit_editor(
        &self,
        name_and_email: Option<(SharedString, SharedString)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let editor = self.commit_editor.clone();
        let can_commit = !self.commit_pending && self.can_commit && !editor.read(cx).is_empty(cx);
        let editor_focus_handle = editor.read(cx).focus_handle(cx).clone();

        let panel_editor_style = panel_editor_style(true, window, cx);

        let settings = ThemeSettings::get_global(cx);
        let line_height = relative(settings.buffer_line_height.value())
            .to_pixels(settings.buffer_font_size.into(), window.rem_size());

        panel_editor_container(window, cx)
            .id("commit-editor-container")
            .relative()
            .w_full()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .h(20. * line_height)
            .bg(cx.theme().colors().editor_background)
            .on_click(cx.listener(move |_, _: &ClickEvent, window, _cx| {
                window.focus(&editor_focus_handle);
            }))
            .child(EditorElement::new(&self.commit_editor, panel_editor_style))
    }

    pub fn render_footer(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .child(h_flex().child("cmd+esc clear message"))
            .child(h_flex().child(panel_filled_button("Commit")))
    }

    fn dismiss(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl Render for CommitModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        v_flex()
            .id("commit-modal")
            .key_context("GitCommit")
            .on_action(cx.listener(Self::dismiss))
            .relative()
            .bg(cx.theme().colors().elevated_surface_background)
            .rounded(px(16.))
            .border_1()
            .border_color(cx.theme().colors().border)
            .py_2()
            .px_4()
            .w(self.width.unwrap_or(px(640.)))
            .h(px(450.))
            .flex_1()
            .overflow_hidden()
            .child(
                v_flex()
                    .flex_1()
                    // TODO: pass name_and_email
                    .child(self.render_commit_editor(None, window, cx)),
            )
            .child(self.render_footer(window, cx))
    }
}
