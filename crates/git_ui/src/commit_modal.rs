// #![allow(unused, dead_code)]

use crate::branch_picker::{self, BranchList};
use crate::git_panel::{commit_message_editor, GitPanel};
use git::{Commit, ShowCommitEditor};
use panel::{panel_button, panel_editor_style, panel_filled_button};
use ui::{prelude::*, KeybindingHint, PopoverMenu, Tooltip};

use editor::{Editor, EditorElement};
use gpui::*;
use util::ResultExt;
use workspace::{
    dock::{Dock, PanelHandle},
    ModalView, Workspace,
};

// nate: It is a pain to get editors to size correctly and not overflow.
//
// this can get replaced with a simple flex layout with more time/a more thoughtful approach.
#[derive(Debug, Clone, Copy)]
pub struct ModalContainerProperties {
    pub modal_width: f32,
    pub editor_height: f32,
    pub footer_height: f32,
    pub container_padding: f32,
    pub modal_border_radius: f32,
}

impl ModalContainerProperties {
    pub fn new(window: &Window, preferred_char_width: usize) -> Self {
        let container_padding = 5.0;

        // Calculate width based on character width
        let mut modal_width = 460.0;
        let style = window.text_style().clone();
        let font_id = window.text_system().resolve_font(&style.font());
        let font_size = style.font_size.to_pixels(window.rem_size());

        if let Ok(em_width) = window.text_system().em_width(font_id, font_size) {
            modal_width = preferred_char_width as f32 * em_width.0 + (container_padding * 2.0);
        }

        Self {
            modal_width,
            editor_height: 300.0,
            footer_height: 24.0,
            container_padding,
            modal_border_radius: 12.0,
        }
    }

    pub fn editor_border_radius(&self) -> Pixels {
        px(self.modal_border_radius - self.container_padding / 2.0)
    }
}

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
    branch_list: Entity<BranchList>,
    git_panel: Entity<GitPanel>,
    commit_editor: Entity<Editor>,
    restore_dock: RestoreDock,
    properties: ModalContainerProperties,
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
        workspace.register_action(|workspace, _: &ShowCommitEditor, window, cx| {
            let Some(git_panel) = workspace.panel::<GitPanel>(cx) else {
                return;
            };

            git_panel.update(cx, |git_panel, cx| {
                git_panel.set_modal_open(true, cx);
            });

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
        let active_repository = panel.active_repository.clone();
        let suggested_commit_message = panel.suggest_commit_message();

        let commit_editor = git_panel.update(cx, |git_panel, cx| {
            git_panel.set_modal_open(true, cx);
            let buffer = git_panel.commit_message_buffer(cx).clone();
            let panel_editor = git_panel.commit_editor.clone();
            let project = git_panel.project.clone();

            cx.new(|cx| {
                let mut editor =
                    commit_message_editor(buffer, None, project.clone(), false, window, cx);
                editor.sync_selections(panel_editor, cx).detach();

                editor
            })
        });

        let commit_message = commit_editor.read(cx).text(cx);

        if let Some(suggested_commit_message) = suggested_commit_message {
            if commit_message.is_empty() {
                commit_editor.update(cx, |editor, cx| {
                    editor.set_placeholder_text(suggested_commit_message, cx);
                });
            }
        }

        let focus_handle = commit_editor.focus_handle(cx);

        cx.on_focus_out(&focus_handle, window, |this, _, window, cx| {
            if !this
                .branch_list
                .focus_handle(cx)
                .contains_focused(window, cx)
            {
                cx.emit(DismissEvent);
            }
        })
        .detach();

        let properties = ModalContainerProperties::new(window, 50);

        Self {
            branch_list: branch_picker::popover(active_repository.clone(), window, cx),
            git_panel,
            commit_editor,
            restore_dock,
            properties,
        }
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
        let properties = self.properties;
        let padding_t = 3.0;
        let padding_b = 6.0;
        // magic number for editor not to overflow the container??
        let extra_space_hack = 1.5 * window.line_height();

        v_flex()
            .h(px(properties.editor_height + padding_b + padding_t) + extra_space_hack)
            .w_full()
            .flex_none()
            .rounded(properties.editor_border_radius())
            .overflow_hidden()
            .px_1p5()
            .pt(px(padding_t))
            .pb(px(padding_b))
            .child(
                div()
                    .h(px(properties.editor_height))
                    .w_full()
                    .child(self.commit_editor_element(window, cx)),
            )
    }

    pub fn render_footer(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let git_panel = self.git_panel.clone();

        let (branch, can_commit, tooltip, commit_label, co_authors) =
            self.git_panel.update(cx, |git_panel, cx| {
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
                let (can_commit, tooltip) = git_panel.configure_commit_button(cx);
                let title = git_panel.commit_button_title();
                let co_authors = git_panel.render_co_authors(cx);
                (branch, can_commit, tooltip, title, co_authors)
            });

        let branch_picker_button = panel_button(branch)
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

        let branch_picker = PopoverMenu::new("popover-button")
            .menu({
                let branch_list = self.branch_list.clone();
                move |_window, _cx| Some(branch_list.clone())
            })
            .trigger_with_tooltip(
                branch_picker_button,
                Tooltip::for_action_title("Switch Branch", &zed_actions::git::Branch),
            )
            .anchor(Corner::BottomLeft)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            });

        let close_kb_hint =
            if let Some(close_kb) = ui::KeyBinding::for_action(&menu::Cancel, window, cx) {
                Some(
                    KeybindingHint::new(close_kb, cx.theme().colors().editor_background)
                        .suffix("Cancel"),
                )
            } else {
                None
            };

        let panel_editor_focus_handle =
            git_panel.update(cx, |git_panel, cx| git_panel.editor_focus_handle(cx));

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
            .h(px(self.properties.footer_height))
            .gap_1()
            .child(h_flex().gap_1().child(branch_picker).children(co_authors))
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
        let properties = self.properties;
        let width = px(properties.modal_width);
        let container_padding = px(properties.container_padding);
        let border_radius = properties.modal_border_radius;
        let editor_focus_handle = self.commit_editor.focus_handle(cx);

        v_flex()
            .id("commit-modal")
            .key_context("GitCommit")
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::commit))
            .on_action(
                cx.listener(|this, _: &zed_actions::git::Branch, window, cx| {
                    this.branch_list.update(cx, |branch_list, cx| {
                        branch_list.popover_handle.toggle(window, cx);
                    })
                }),
            )
            .elevation_3(cx)
            .overflow_hidden()
            .flex_none()
            .relative()
            .bg(cx.theme().colors().elevated_surface_background)
            .rounded(px(border_radius))
            .border_1()
            .border_color(cx.theme().colors().border)
            .w(width)
            .p(container_padding)
            .child(
                v_flex()
                    .id("editor-container")
                    .justify_between()
                    .p_2()
                    .size_full()
                    .gap_2()
                    .rounded(properties.editor_border_radius())
                    .overflow_hidden()
                    .cursor_text()
                    .bg(cx.theme().colors().editor_background)
                    .border_1()
                    .border_color(cx.theme().colors().border_variant)
                    .on_click(cx.listener(move |_, _: &ClickEvent, window, _cx| {
                        window.focus(&editor_focus_handle);
                    }))
                    .child(
                        div()
                            .flex_1()
                            .size_full()
                            .child(self.render_commit_editor(window, cx)),
                    )
                    .child(self.render_footer(window, cx)),
            )
    }
}
