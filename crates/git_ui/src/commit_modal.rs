// #![allow(unused, dead_code)]

use crate::git_panel::{commit_message_editor, GitPanel};
use git::Commit;
use panel::{panel_button, panel_editor_style, panel_filled_button};
use ui::{prelude::*, KeybindingHint, Tooltip};

use editor::{Editor, EditorElement};
use gpui::*;
use util::ResultExt;
use workspace::{
    dock::{Dock, PanelHandle},
    ModalView, Workspace,
};

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
    restore_dock: RestoreDock,
}

impl Focusable for CommitModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.commit_editor.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for CommitModal {}
impl ModalView for CommitModal {
    fn on_before_dismiss(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        self.git_panel.update(cx, |git_panel, cx| {
            git_panel.set_modal_open(false, cx);
        });
        self.restore_dock
            .dock
            .update(cx, |dock, cx| {
                if let Some(active_index) = self.restore_dock.active_index {
                    dock.activate_panel(active_index, window, cx)
                }
                dock.set_open(self.restore_dock.is_open, window, cx)
            })
            .log_err();
        workspace::DismissDecision::Dismiss(true)
    }
}

struct RestoreDock {
    dock: WeakEntity<Dock>,
    is_open: bool,
    active_index: Option<usize>,
}

impl CommitModal {
    pub fn register(workspace: &mut Workspace, _: &mut Window, _cx: &mut Context<Workspace>) {
        workspace.register_action(|workspace, _: &Commit, window, cx| {
            let Some(git_panel) = workspace.panel::<GitPanel>(cx) else {
                return;
            };

            let (can_commit, conflict) = git_panel.update(cx, |git_panel, _cx| {
                let can_commit = git_panel.can_commit();
                let conflict = git_panel.has_unstaged_conflicts();
                (can_commit, conflict)
            });
            if !can_commit {
                let message = if conflict {
                    "There are still conflicts. You must stage these before committing."
                } else {
                    "No changes to commit."
                };
                let prompt = window.prompt(PromptLevel::Warning, message, None, &["Ok"], cx);
                cx.spawn(|_, _| async move {
                    prompt.await.ok();
                })
                .detach();
            }

            let dock = workspace.dock_at_position(git_panel.position(window, cx));
            let is_open = dock.read(cx).is_open();
            let active_index = dock.read(cx).active_panel_index();
            let dock = dock.downgrade();
            let restore_dock_position = RestoreDock {
                dock,
                is_open,
                active_index,
            };
            workspace.open_panel::<GitPanel>(window, cx);
            workspace.toggle_modal(window, cx, move |window, cx| {
                CommitModal::new(git_panel, restore_dock_position, window, cx)
            })
        });
    }

    fn new(
        git_panel: Entity<GitPanel>,
        restore_dock: RestoreDock,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let panel = git_panel.read(cx);
        let suggested_message = panel.suggest_commit_message();

        let commit_editor = git_panel.update(cx, |git_panel, cx| {
            git_panel.set_modal_open(true, cx);
            let buffer = git_panel.commit_message_buffer(cx).clone();
            let project = git_panel.project.clone();
            cx.new(|cx| commit_message_editor(buffer, project.clone(), false, window, cx))
        });

        let commit_message = commit_editor.read(cx).text(cx);

        if let Some(suggested_message) = suggested_message {
            if commit_message.is_empty() {
                commit_editor.update(cx, |editor, cx| {
                    editor.set_text(suggested_message, window, cx);
                    editor.select_all(&Default::default(), window, cx);
                });
            } else {
                if commit_message.as_str().trim() == suggested_message.trim() {
                    commit_editor.update(cx, |editor, cx| {
                        // select the message to make it easy to delete
                        editor.select_all(&Default::default(), window, cx);
                    });
                }
            }
        }

        Self {
            git_panel,
            commit_editor,
            restore_dock,
        }
    }
    fn container_width(&self, window: &mut Window, cx: &mut Context<Self>) -> f32 {
        let preferred_width = 50; // (chars wide)
        let padding_x = self.container_padding();

        let mut width = 460.0;

        let style = window.text_style().clone();
        let font_id = window.text_system().resolve_font(&style.font());
        let font_size = style.font_size.to_pixels(window.rem_size());

        if let Ok(em_width) = window.text_system().em_width(font_id, font_size) {
            width = preferred_width as f32 * em_width.0 + (padding_x * 2.0);
        }

        cx.notify();

        width
    }

    fn height(&self) -> f32 {
        360.0
    }

    fn footer_height(&self) -> f32 {
        32.0
    }

    fn container_padding(&self) -> f32 {
        16.0
    }

    /// x, y
    fn editor_padding(&self) -> (f32, f32) {
        (8.0, 12.0)
    }

    fn border_radius(&self) -> f32 {
        8.0
    }

    fn commit_editor_element(&self, window: &mut Window, cx: &mut Context<Self>) -> EditorElement {
        let editor_style = panel_editor_style(true, window, cx);

        EditorElement::new(&self.commit_editor, editor_style)
    }

    pub fn render_commit_editor(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let height = self.height();
        let footer_height = self.footer_height();
        let (padding_x, padding_y) = self.editor_padding();
        let modal_border_radius = self.border_radius();

        let container_height = height - padding_y * 2.0;
        let editor_height = container_height - footer_height;
        let border_radius = modal_border_radius - padding_x / 2.0;

        let editor = self.commit_editor.clone();
        let editor_focus_handle = editor.focus_handle(cx);

        v_flex()
            .debug_below()
            .id("editor-container")
            .h(px(container_height))
            .cursor_text()
            .bg(cx.theme().colors().editor_background)
            .flex_1()
            .size_full()
            .rounded(px(border_radius))
            .overflow_hidden()
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            // .py_2()
            .px_3()
            .on_click(cx.listener(move |_, _: &ClickEvent, window, _cx| {
                window.focus(&editor_focus_handle);
            }))
            .child(
                div()
                    .h(px(editor_height))
                    .w_full()
                    .child(self.commit_editor_element(window, cx)),
            )
    }

    fn render_footer(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let git_panel = self.git_panel.clone();

        let (branch, tooltip, commit_label, co_authors) =
            self.git_panel.update(cx, |git_panel, cx| {
                let branch = git_panel
                    .active_repository
                    .as_ref()
                    .and_then(|repo| repo.read(cx).current_branch().map(|b| b.name.clone()))
                    .unwrap_or_else(|| "<no branch>".into());
                let tooltip = if git_panel.has_staged_changes() {
                    "Commit staged changes"
                } else {
                    "Commit changes to tracked files"
                };
                let title = if git_panel.has_staged_changes() {
                    "Commit"
                } else {
                    "Commit Tracked"
                };
                let co_authors = git_panel.render_co_authors(cx);
                (branch, tooltip, title, co_authors)
            });

        let branch_selector = panel_button(branch)
            .icon(IconName::GitBranch)
            .icon_size(IconSize::Small)
            .icon_color(Color::Placeholder)
            .color(Color::Muted)
            .icon_position(IconPosition::Start)
            .tooltip(Tooltip::for_action_title(
                "Switch Branch",
                &zed_actions::git::Branch,
            ))
            .on_click(cx.listener(|_, _, window, cx| {
                window.dispatch_action(zed_actions::git::Branch.boxed_clone(), cx);
            }))
            .style(ButtonStyle::Transparent);

        let close_kb_hint =
            if let Some(close_kb) = ui::KeyBinding::for_action(&menu::Cancel, window, cx) {
                Some(
                    KeybindingHint::new(close_kb, cx.theme().colors().editor_background)
                        .suffix("Cancel"),
                )
            } else {
                None
            };

        let (panel_editor_focus_handle, can_commit) = git_panel.update(cx, |git_panel, cx| {
            (git_panel.editor_focus_handle(cx), git_panel.can_commit())
        });

        let commit_button = panel_filled_button(commit_label)
            .tooltip(move |window, cx| {
                Tooltip::for_action_in(tooltip, &Commit, &panel_editor_focus_handle, window, cx)
            })
            .disabled(!can_commit)
            .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                this.git_panel
                    .update(cx, |git_panel, cx| git_panel.commit_changes(window, cx));
                cx.emit(DismissEvent);
            }));

        h_flex()
            .group("commit_editor_footer")
            .flex_none()
            .w_full()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(self.footer_height()))
            .pb_0p5()
            .gap_1()
            .child(h_flex().gap_1().child(branch_selector).children(co_authors))
            .child(div().flex_1())
            .child(
                h_flex()
                    .items_center()
                    .justify_end()
                    .flex_none()
                    .px_1()
                    .gap_4()
                    .children(close_kb_hint)
                    .child(commit_button),
            )
    }

    fn dismiss(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
    fn commit(&mut self, _: &git::Commit, window: &mut Window, cx: &mut Context<Self>) {
        self.git_panel
            .update(cx, |git_panel, cx| git_panel.commit_changes(window, cx));
        cx.emit(DismissEvent);
    }
}

impl Render for CommitModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let width = self.container_width(window, cx);
        let height = self.height();
        let container_padding = self.container_padding();
        let border_radius = self.border_radius();
        let footer_height = 32.0;

        v_flex()
            .id("commit-modal")
            .key_context("GitCommit")
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::commit))
            .elevation_3(cx)
            .overflow_hidden()
            .flex_none()
            .relative()
            .bg(cx.theme().colors().elevated_surface_background)
            .rounded(px(border_radius))
            .border_1()
            .border_color(cx.theme().colors().border)
            .w(px(width))
            .h(px(height))
            .p(px(container_padding))
            .child(
                div()
                    .h(px(height - footer_height))
                    .overflow_hidden()
                    .child(self.render_commit_editor(window, cx)),
            )
            .child(
                v_flex()
                    .flex_1()
                    .p_2()
                    .overflow_hidden()
                    .child(self.render_footer(window, cx)),
            )
        // .child(self.render_footer(window, cx))
    }
}
