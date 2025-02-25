#![allow(unused, dead_code)]

use crate::git_panel::{commit_message_editor, GitPanel};
use crate::repository_selector::RepositorySelector;
use anyhow::Result;
use git::Commit;
use language::language_settings::LanguageSettings;
use language::Buffer;
use panel::{
    panel_button, panel_editor_container, panel_editor_style, panel_filled_button,
    panel_icon_button,
};
use settings::Settings;
use theme::ThemeSettings;
use ui::{prelude::*, KeybindingHint, Tooltip};

use editor::{Direction, Editor, EditorElement, EditorMode, EditorSettings, MultiBuffer};
use gpui::*;
use project::git::Repository;
use project::{Fs, Project};
use std::sync::Arc;
use workspace::dock::{Dock, DockPosition, PanelHandle};
use workspace::{ModalView, Workspace};

// actions!(commit_modal, [NextSuggestion, PrevSuggestion]);

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
    current_suggestion: Option<usize>,
    suggested_messages: Vec<SharedString>,
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
        self.restore_dock.dock.update(cx, |dock, cx| {
            if let Some(active_index) = self.restore_dock.active_index {
                dock.activate_panel(active_index, window, cx)
            }
            dock.set_open(self.restore_dock.is_open, window, cx)
        });
        workspace::DismissDecision::Dismiss(true)
    }
}

struct RestoreDock {
    dock: WeakEntity<Dock>,
    is_open: bool,
    active_index: Option<usize>,
}

impl CommitModal {
    pub fn register(workspace: &mut Workspace, _: &mut Window, cx: &mut Context<Workspace>) {
        workspace.register_action(|workspace, _: &Commit, window, cx| {
            let Some(git_panel) = workspace.panel::<GitPanel>(cx) else {
                return;
            };

            let (can_commit, conflict) = git_panel.update(cx, |git_panel, cx| {
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

        let focus_handle = commit_editor.focus_handle(cx);

        cx.on_focus_out(&focus_handle, window, |this, _, window, cx| {
            cx.emit(DismissEvent);
        })
        .detach();

        Self {
            git_panel,
            commit_editor,
            restore_dock,
            current_suggestion: None,
            suggested_messages: vec![],
        }
    }

    /// Returns container `(width, x padding, border radius)`
    fn container_properties(&self, window: &mut Window, cx: &mut Context<Self>) -> (f32, f32, f32) {
        // TODO: Let's set the width based on your set wrap guide if possible

        // let settings = EditorSettings::get_global(cx);

        // let first_wrap_guide = self
        //     .commit_editor
        //     .read(cx)
        //     .wrap_guides(cx)
        //     .iter()
        //     .next()
        //     .map(|(guide, active)| if *active { Some(*guide) } else { None })
        //     .flatten();

        // let preferred_width = if let Some(guide) = first_wrap_guide {
        //     guide
        // } else {
        //     80
        // };

        let border_radius = 16.0;

        let preferred_width = 50; // (chars wide)

        let mut width = 460.0;
        let padding_x = 16.0;

        let mut snapshot = self
            .commit_editor
            .update(cx, |editor, cx| editor.snapshot(window, cx));
        let style = window.text_style().clone();

        let font_id = window.text_system().resolve_font(&style.font());
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line_height = style.line_height_in_pixels(window.rem_size());
        if let Ok(em_width) = window.text_system().em_width(font_id, font_size) {
            width = preferred_width as f32 * em_width.0 + (padding_x * 2.0);
            cx.notify();
        }

        // cx.notify();

        (width, padding_x, border_radius)
    }

    // fn cycle_suggested_messages(&mut self, direction: Direction, cx: &mut Context<Self>) {
    //     let new_index = match direction {
    //         Direction::Next => {
    //             (self.current_suggestion.unwrap_or(0) + 1).rem_euclid(self.suggested_messages.len())
    //         }
    //         Direction::Prev => {
    //             (self.current_suggestion.unwrap_or(0) + self.suggested_messages.len() - 1)
    //                 .rem_euclid(self.suggested_messages.len())
    //         }
    //     };
    //     self.current_suggestion = Some(new_index);

    //     cx.notify();
    // }

    // fn next_suggestion(&mut self, _: &NextSuggestion, window: &mut Window, cx: &mut Context<Self>) {
    //     self.current_suggestion = Some(1);
    //     self.apply_suggestion(window, cx);
    // }

    // fn prev_suggestion(&mut self, _: &PrevSuggestion, window: &mut Window, cx: &mut Context<Self>) {
    //     self.current_suggestion = Some(0);
    //     self.apply_suggestion(window, cx);
    // }

    // fn set_commit_message(&mut self, message: &str, window: &mut Window, cx: &mut Context<Self>) {
    //     self.commit_editor.update(cx, |editor, cx| {
    //         editor.set_text(message.to_string(), window, cx)
    //     });
    //     self.current_suggestion = Some(0);
    //     cx.notify();
    // }

    // fn apply_suggestion(&mut self, window: &mut Window, cx: &mut Context<Self>) {
    //     let suggested_messages = self.suggested_messages.clone();

    //     if let Some(suggestion) = self.current_suggestion {
    //         let suggested_message = &suggested_messages[suggestion];

    //         self.set_commit_message(suggested_message, window, cx);
    //     }

    //     cx.notify();
    // }

    fn commit_editor_element(&self, window: &mut Window, cx: &mut Context<Self>) -> EditorElement {
        let mut editor = self.commit_editor.clone();

        let editor_style = panel_editor_style(true, window, cx);

        EditorElement::new(&self.commit_editor, editor_style)
    }

    pub fn render_commit_editor(
        &self,
        name_and_email: Option<(SharedString, SharedString)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let (width, padding_x, modal_border_radius) = self.container_properties(window, cx);

        let border_radius = modal_border_radius - padding_x / 2.0;

        let editor = self.commit_editor.clone();
        let editor_focus_handle = editor.focus_handle(cx);

        let settings = ThemeSettings::get_global(cx);
        let line_height = relative(settings.buffer_line_height.value())
            .to_pixels(settings.buffer_font_size(cx).into(), window.rem_size());

        let mut snapshot = self
            .commit_editor
            .update(cx, |editor, cx| editor.snapshot(window, cx));
        let style = window.text_style().clone();

        let font_id = window.text_system().resolve_font(&style.font());
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line_height = style.line_height_in_pixels(window.rem_size());
        let em_width = window.text_system().em_width(font_id, font_size);

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

        let changes_count = self.git_panel.read(cx).total_staged_count();

        let close_kb_hint =
            if let Some(close_kb) = ui::KeyBinding::for_action(&menu::Cancel, window, cx) {
                Some(
                    KeybindingHint::new(close_kb, cx.theme().colors().editor_background)
                        .suffix("Cancel"),
                )
            } else {
                None
            };

        let fake_commit_kb =
            ui::KeyBinding::new(gpui::KeyBinding::new("cmd-enter", gpui::NoAction, None), cx);

        let commit_hint =
            KeybindingHint::new(fake_commit_kb, cx.theme().colors().editor_background)
                .suffix(commit_label);

        let focus_handle = self.focus_handle(cx);

        // let next_suggestion_kb =
        //     ui::KeyBinding::for_action_in(&NextSuggestion, &focus_handle.clone(), window, cx);
        // let next_suggestion_hint = next_suggestion_kb.map(|kb| {
        //     KeybindingHint::new(kb, cx.theme().colors().editor_background).suffix("Next Suggestion")
        // });

        // let prev_suggestion_kb =
        //     ui::KeyBinding::for_action_in(&PrevSuggestion, &focus_handle.clone(), window, cx);
        // let prev_suggestion_hint = prev_suggestion_kb.map(|kb| {
        //     KeybindingHint::new(kb, cx.theme().colors().editor_background)
        //         .suffix("Previous Suggestion")
        // });

        v_flex()
            .id("editor-container")
            .bg(cx.theme().colors().editor_background)
            .flex_1()
            .size_full()
            .rounded(px(border_radius))
            .overflow_hidden()
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .py_2()
            .px_3()
            .on_click(cx.listener(move |_, _: &ClickEvent, window, _cx| {
                window.focus(&editor_focus_handle);
            }))
            .child(
                div()
                    .size_full()
                    .flex_1()
                    .child(self.commit_editor_element(window, cx)),
            )
            .child(
                h_flex()
                    .group("commit_editor_footer")
                    .flex_none()
                    .w_full()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .pt_2()
                    .pb_0p5()
                    .gap_1()
                    .child(h_flex().gap_1().child(branch_selector).children(co_authors))
                    .child(div().flex_1())
                    .child(
                        h_flex()
                            .opacity(0.7)
                            .group_hover("commit_editor_footer", |this| this.opacity(1.0))
                            .items_center()
                            .justify_end()
                            .flex_none()
                            .px_1()
                            .gap_4()
                            .children(close_kb_hint)
                            // .children(next_suggestion_hint)
                            .child(commit_hint),
                    ),
            )
    }

    pub fn render_footer(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (branch, tooltip, title, co_authors) = self.git_panel.update(cx, |git_panel, cx| {
            let branch = git_panel
                .active_repository
                .as_ref()
                .and_then(|repo| {
                    repo.read(cx)
                        .repository_entry
                        .branch()
                        .map(|b| b.name.clone())
                })
                .unwrap_or_else(|| "<no branch>".into());
            let tooltip = if git_panel.has_staged_changes() {
                "Commit staged changes"
            } else {
                "Commit changes to tracked files"
            };
            let title = if git_panel.has_staged_changes() {
                "Commit"
            } else {
                "Commit All"
            };
            let co_authors = git_panel.render_co_authors(cx);
            (branch, tooltip, title, co_authors)
        });

        let branch_selector = panel_button(branch)
            .icon(IconName::GitBranch)
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted)
            .icon_position(IconPosition::Start)
            .tooltip(Tooltip::for_action_title(
                "Switch Branch",
                &zed_actions::git::Branch,
            ))
            .on_click(cx.listener(|_, _, window, cx| {
                window.dispatch_action(zed_actions::git::Branch.boxed_clone(), cx);
            }))
            .style(ButtonStyle::Transparent);

        let changes_count = self.git_panel.read(cx).total_staged_count();

        let close_kb_hint =
            if let Some(close_kb) = ui::KeyBinding::for_action(&menu::Cancel, window, cx) {
                Some(
                    KeybindingHint::new(close_kb, cx.theme().colors().editor_background)
                        .suffix("Cancel"),
                )
            } else {
                None
            };

        h_flex()
            .items_center()
            .h(px(36.0))
            .w_full()
            .justify_between()
            .px_3()
            .child(h_flex().child(branch_selector))
            .child(
                h_flex().gap_1p5().children(co_authors).child(
                    Button::new("stage-button", title)
                        .tooltip(Tooltip::for_action_title(tooltip, &git::Commit))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.commit(&Default::default(), window, cx);
                        })),
                ),
            )
    }

    fn border_radius(&self) -> f32 {
        8.0
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
        let (width, _, border_radius) = self.container_properties(window, cx);

        v_flex()
            .id("commit-modal")
            .key_context("GitCommit")
            .elevation_3(cx)
            .overflow_hidden()
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::commit))
            // .on_action(cx.listener(Self::next_suggestion))
            // .on_action(cx.listener(Self::prev_suggestion))
            .relative()
            .justify_between()
            .bg(cx.theme().colors().elevated_surface_background)
            .rounded(px(border_radius))
            .border_1()
            .border_color(cx.theme().colors().border)
            .w(px(width))
            .h(px(360.))
            .flex_1()
            .overflow_hidden()
            .child(
                v_flex()
                    .flex_1()
                    .p_2()
                    .child(self.render_commit_editor(None, window, cx)),
            )
        // .child(self.render_footer(window, cx))
    }
}
