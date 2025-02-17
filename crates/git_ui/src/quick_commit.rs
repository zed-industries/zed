#![allow(unused, dead_code)]

use crate::repository_selector::RepositorySelector;
use anyhow::Result;
use git::{CommitAllChanges, CommitChanges};
use language::Buffer;
use panel::{panel_editor_container, panel_editor_style, panel_filled_button, panel_icon_button};
use ui::{prelude::*, Tooltip};

use editor::{Editor, EditorElement, EditorMode, MultiBuffer};
use gpui::*;
use project::git::Repository;
use project::{Fs, Project};
use std::sync::Arc;
use workspace::{ModalView, Workspace};

actions!(
    git,
    [QuickCommitWithMessage, QuickCommitStaged, QuickCommitAll]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        QuickCommitModal::register(workspace, window, cx)
    })
    .detach();
}

fn commit_message_editor(
    commit_message_buffer: Option<Entity<Buffer>>,
    window: &mut Window,
    cx: &mut Context<'_, Editor>,
) -> Editor {
    let mut commit_editor = if let Some(commit_message_buffer) = commit_message_buffer {
        let buffer = cx.new(|cx| MultiBuffer::singleton(commit_message_buffer, cx));
        Editor::new(
            EditorMode::AutoHeight { max_lines: 10 },
            buffer,
            None,
            false,
            window,
            cx,
        )
    } else {
        Editor::auto_height(10, window, cx)
    };
    commit_editor.set_use_autoclose(false);
    commit_editor.set_show_gutter(false, cx);
    commit_editor.set_show_wrap_guides(false, cx);
    commit_editor.set_show_indent_guides(false, cx);
    commit_editor.set_placeholder_text("Enter commit message", cx);
    commit_editor
}

pub struct QuickCommitModal {
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    project: Entity<Project>,
    active_repository: Option<Entity<Repository>>,
    repository_selector: Entity<RepositorySelector>,
    commit_editor: Entity<Editor>,
    width: Option<Pixels>,
    commit_task: Task<Result<()>>,
    commit_pending: bool,
    can_commit: bool,
    can_commit_all: bool,
    enable_auto_coauthors: bool,
}

impl Focusable for QuickCommitModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for QuickCommitModal {}
impl ModalView for QuickCommitModal {}

impl QuickCommitModal {
    pub fn register(workspace: &mut Workspace, _: &mut Window, cx: &mut Context<Workspace>) {
        workspace.register_action(|workspace, _: &QuickCommitWithMessage, window, cx| {
            let project = workspace.project().clone();
            let fs = workspace.app_state().fs.clone();

            workspace.toggle_modal(window, cx, move |window, cx| {
                QuickCommitModal::new(project, fs, window, None, cx)
            });
        });
    }

    pub fn new(
        project: Entity<Project>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        commit_message_buffer: Option<Entity<Buffer>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let git_store = project.read(cx).git_store().clone();
        let active_repository = project.read(cx).active_repository(cx);

        let focus_handle = cx.focus_handle();

        let commit_editor = cx.new(|cx| commit_message_editor(commit_message_buffer, window, cx));
        commit_editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
        });

        let repository_selector = cx.new(|cx| RepositorySelector::new(project.clone(), window, cx));

        Self {
            focus_handle,
            fs,
            project,
            active_repository,
            repository_selector,
            commit_editor,
            width: None,
            commit_task: Task::ready(Ok(())),
            commit_pending: false,
            can_commit: false,
            can_commit_all: false,
            enable_auto_coauthors: true,
        }
    }

    pub fn render_header(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let all_repositories = self
            .project
            .read(cx)
            .git_store()
            .read(cx)
            .all_repositories();
        let entry_count = self
            .active_repository
            .as_ref()
            .map_or(0, |repo| repo.read(cx).entry_count());

        let changes_string = match entry_count {
            0 => "No changes".to_string(),
            1 => "1 change".to_string(),
            n => format!("{} changes", n),
        };

        div().absolute().top_0().right_0().child(
            panel_icon_button("open_change_list", IconName::PanelRight)
                .disabled(true)
                .tooltip(Tooltip::text("Changes list coming soon!")),
        )
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

        let focus_handle_1 = self.focus_handle(cx).clone();
        let focus_handle_2 = self.focus_handle(cx).clone();

        let panel_editor_style = panel_editor_style(true, window, cx);

        let commit_staged_button = panel_filled_button("Commit")
            .tooltip(move |window, cx| {
                let focus_handle = focus_handle_1.clone();
                Tooltip::for_action_in(
                    "Commit all staged changes",
                    &CommitChanges,
                    &focus_handle,
                    window,
                    cx,
                )
            })
            .when(!can_commit, |this| {
                this.disabled(true).style(ButtonStyle::Transparent)
            });
        // .on_click({
        //     let name_and_email = name_and_email.clone();
        //     cx.listener(move |this, _: &ClickEvent, window, cx| {
        //         this.commit_changes(&CommitChanges, name_and_email.clone(), window, cx)
        //     })
        // });

        let commit_all_button = panel_filled_button("Commit All")
            .tooltip(move |window, cx| {
                let focus_handle = focus_handle_2.clone();
                Tooltip::for_action_in(
                    "Commit all changes, including unstaged changes",
                    &CommitAllChanges,
                    &focus_handle,
                    window,
                    cx,
                )
            })
            .when(!can_commit, |this| {
                this.disabled(true).style(ButtonStyle::Transparent)
            });
        // .on_click({
        //     let name_and_email = name_and_email.clone();
        //     cx.listener(move |this, _: &ClickEvent, window, cx| {
        //         this.commit_tracked_changes(
        //             &CommitAllChanges,
        //             name_and_email.clone(),
        //             window,
        //             cx,
        //         )
        //     })
        // });

        let co_author_button = panel_icon_button("add-co-author", IconName::UserGroup)
            .icon_color(if self.enable_auto_coauthors {
                Color::Muted
            } else {
                Color::Accent
            })
            .icon_size(IconSize::Small)
            .toggle_state(self.enable_auto_coauthors)
            // .on_click({
            //     cx.listener(move |this, _: &ClickEvent, _, cx| {
            //         this.toggle_auto_coauthors(cx);
            //     })
            // })
            .tooltip(move |window, cx| {
                Tooltip::with_meta(
                    "Toggle automatic co-authors",
                    None,
                    "Automatically adds current collaborators",
                    window,
                    cx,
                )
            });

        panel_editor_container(window, cx)
            .id("commit-editor-container")
            .relative()
            .w_full()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .h(px(140.))
            .bg(cx.theme().colors().editor_background)
            .on_click(cx.listener(move |_, _: &ClickEvent, window, _cx| {
                window.focus(&editor_focus_handle);
            }))
            .child(EditorElement::new(&self.commit_editor, panel_editor_style))
            .child(div().flex_1())
            .child(
                h_flex()
                    .items_center()
                    .h_8()
                    .justify_between()
                    .gap_1()
                    .child(co_author_button)
                    .child(commit_all_button)
                    .child(commit_staged_button),
            )
    }

    pub fn render_footer(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .child(h_flex().child("cmd+esc clear message"))
            .child(
                h_flex()
                    .child(panel_filled_button("Commit"))
                    .child(panel_filled_button("Commit All")),
            )
    }

    fn dismiss(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl Render for QuickCommitModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        v_flex()
            .id("quick-commit-modal")
            .key_context("QuickCommit")
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
            .child(self.render_header(window, cx))
            .child(
                v_flex()
                    .flex_1()
                    // TODO: pass name_and_email
                    .child(self.render_commit_editor(None, window, cx)),
            )
            .child(self.render_footer(window, cx))
    }
}
