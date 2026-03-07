use acp_thread::AgentSessionInfo;
use agent::{ClaudeCodeSessionIndex, ClaudeCodeSessionList};
use chrono::{Local, Utc};
use gpui::{
    DismissEvent, EventEmitter, FocusHandle, Focusable, Render, ScrollHandle, WeakEntity,
};
use std::path::PathBuf;
use ui::{prelude::*, ListItem, ListItemSpacing};
use workspace::{ModalView, Workspace};

use crate::agent_panel::AgentPanel;
use crate::ExternalAgent;

pub struct ClaudeCodeSessionPicker {
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    sessions: Vec<AgentSessionInfo>,
    selected_index: usize,
    scroll_handle: ScrollHandle,
    project_path: PathBuf,
}

impl ClaudeCodeSessionPicker {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        project_path: PathBuf,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        // Load sessions from Claude Code CLI storage
        let sessions = ClaudeCodeSessionIndex::for_project(&project_path)
            .map(|index| {
                let list = ClaudeCodeSessionList::new(index);
                list.list_sessions_sync()
            })
            .unwrap_or_default();

        cx.on_focus_in(&cx.focus_handle(), window, |this, _window, cx| {
            cx.notify();
        })
        .detach();

        Self {
            focus_handle: cx.focus_handle(),
            workspace,
            sessions,
            selected_index: 0,
            scroll_handle: ScrollHandle::new(),
            project_path,
        }
    }

    pub fn toggle(
        workspace: &mut Workspace,
        project_path: PathBuf,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        // Check if there are sessions first
        let has_sessions = ClaudeCodeSessionIndex::for_project(&project_path)
            .map(|index| {
                index
                    .list_sessions()
                    .map(|s| !s.is_empty())
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        if has_sessions {
            let workspace_weak = cx.entity().downgrade();
            workspace.toggle_modal(window, cx, move |window, cx| {
                Self::new(workspace_weak.clone(), project_path.clone(), window, cx)
            });
        } else {
            // No sessions, start new directly
            Self::start_new_session(workspace, window, cx);
        }
    }

    fn start_new_session(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        workspace.focus_panel::<AgentPanel>(window, cx);
        if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
            panel.update(cx, |panel, cx| {
                panel.external_thread(
                    Some(ExternalAgent::ClaudeCode),
                    None,
                    None,
                    window,
                    cx,
                );
            });
        }
    }

    fn resume_session(&mut self, session: AgentSessionInfo, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.focus_panel::<AgentPanel>(window, cx);
                if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                    panel.update(cx, |panel, cx| {
                        panel.external_thread(
                            Some(ExternalAgent::ClaudeCode),
                            Some(session),
                            None,
                            window,
                            cx,
                        );
                    });
                }
            });
        }
        cx.emit(DismissEvent);
    }

    fn start_new(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                Self::start_new_session(workspace, window, cx);
            });
        }
        cx.emit(DismissEvent);
    }

    fn select_previous(&mut self, _: &menu::SelectPrevious, _window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            cx.notify();
        }
    }

    fn select_next(&mut self, _: &menu::SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        // +1 for "New Conversation" option
        if self.selected_index < self.sessions.len() {
            self.selected_index += 1;
            cx.notify();
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_index == 0 {
            self.start_new(window, cx);
        } else if let Some(session) = self.sessions.get(self.selected_index - 1).cloned() {
            self.resume_session(session, window, cx);
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn format_time(updated_at: Option<chrono::DateTime<Utc>>) -> String {
        match updated_at {
            Some(time) => {
                let local = time.with_timezone(&Local);
                let now = Local::now();
                let duration = now.signed_duration_since(local);

                if duration.num_days() > 0 {
                    format!("{}d ago", duration.num_days())
                } else if duration.num_hours() > 0 {
                    format!("{}h ago", duration.num_hours())
                } else if duration.num_minutes() > 0 {
                    format!("{}m ago", duration.num_minutes())
                } else {
                    "Just now".to_string()
                }
            }
            None => "Unknown".to_string(),
        }
    }
}

impl EventEmitter<DismissEvent> for ClaudeCodeSessionPicker {}

impl Focusable for ClaudeCodeSessionPicker {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for ClaudeCodeSessionPicker {}

impl Render for ClaudeCodeSessionPicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sessions = self.sessions.clone();

        v_flex()
            .key_context("ClaudeCodeSessionPicker")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .elevation_3(cx)
            .w(rems(24.))
            .max_h(rems(32.))
            .child(
                v_flex()
                    .p_2()
                    .gap_1()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        Label::new("Claude Code Sessions")
                            .size(LabelSize::Large)
                            ,
                    )
                    .child(
                        Label::new("Resume a past session or start new")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(
                v_flex()
                    .id("session-list")
                    .p_1()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .child(
                        // "New Conversation" option
                        ListItem::new("new-conversation")
                            .rounded()
                            .toggle_state(self.selected_index == 0)
                            .spacing(ListItemSpacing::Sparse)
                            .start_slot(
                                h_flex()
                                    .gap_2()
                                    .child(Icon::new(IconName::Plus).size(IconSize::Small))
                                    .child(
                                        Label::new("New Conversation")
                                            .size(LabelSize::Default)
                                            ,
                                    ),
                            )
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.start_new(window, cx);
                            })),
                    )
                    .child(
                        div()
                            .my_1()
                            .h_px()
                            .bg(cx.theme().colors().border),
                    )
                    .children(sessions.into_iter().enumerate().map(|(ix, session)| {
                        let title = session
                            .title
                            .clone()
                            .unwrap_or_else(|| "Untitled".into());
                        let time = Self::format_time(session.updated_at);
                        let selected = self.selected_index == ix + 1;
                        let session_for_click = session.clone();

                        ListItem::new(session.session_id.0.clone())
                            .rounded()
                            .toggle_state(selected)
                            .spacing(ListItemSpacing::Sparse)
                            .start_slot(
                                v_flex()
                                    .gap_0p5()
                                    .child(
                                        Label::new(title)
                                            .size(LabelSize::Default)
                                            .truncate(),
                                    )
                                    .child(
                                        Label::new(time)
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted),
                                    ),
                            )
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.resume_session(session_for_click.clone(), window, cx);
                            }))
                    })),
            )
    }
}
