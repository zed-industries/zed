use std::sync::Arc;

use call::{room::Event, ActiveCall};
use gpui::{
    actions, App, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle, Focusable,
    Render, Subscription, Task, WeakEntity,
};
use ui::{prelude::*, IconName, Label, List, ListItem};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

actions!(agent_debug_panel, [ToggleFocus]);

pub struct AgentDebugPanel {
    width: Option<gpui::Pixels>,
    workspace: WeakEntity<Workspace>,
    active_call: Option<Entity<ActiveCall>>,
    _subscriptions: Vec<Subscription>,
}

impl AgentDebugPanel {
    pub fn load(
        workspace: WeakEntity<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<anyhow::Result<Entity<Self>>> {
        cx.spawn(async move |cx| {
            let active_call = cx.update(|_window, cx| ActiveCall::global(cx))?;

            workspace.update_in(cx, |workspace, _window, cx| {
                cx.new(|cx| Self::new(workspace, active_call, cx))
            })
        })
    }

    fn new(
        workspace: &Workspace,
        active_call: Option<Entity<ActiveCall>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut subscriptions = Vec::new();

        if let Some(active_call) = &active_call {
            subscriptions.push(cx.subscribe(active_call, Self::handle_call_event));
        }

        Self {
            width: Some(400.0),
            workspace: workspace.weak_handle(),
            active_call,
            _subscriptions: subscriptions,
        }
    }

    fn handle_call_event(
        &mut self,
        _call: &Entity<ActiveCall>,
        event: &call::room::Event,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, Event::ParticipantAgentActivityChanged { .. }) {
            cx.notify();
        }
    }
}

impl Focusable for AgentDebugPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).focus_handle(cx))
            .unwrap_or_else(|| cx.focus_handle())
    }
}

impl EventEmitter<PanelEvent> for AgentDebugPanel {}

impl Panel for AgentDebugPanel {
    fn persistent_name() -> &'static str {
        "Agent Debug Panel"
    }

    fn panel_key() -> &'static str {
        "AgentDebugPanel"
    }

    fn position(&self, _window: &gpui::Window, _cx: &App) -> DockPosition {
        DockPosition::Right
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(
            position,
            DockPosition::Left | DockPosition::Right | DockPosition::Bottom
        )
    }

    fn set_position(
        &mut self,
        _position: DockPosition,
        _window: &mut gpui::Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn size(&self, _window: &gpui::Window, _cx: &App) -> gpui::Pixels {
        self.width.unwrap_or(400.0)
    }

    fn set_size(
        &mut self,
        size: Option<gpui::Pixels>,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        self.width = size;
        cx.notify();
    }

    fn icon(&self, _window: &gpui::Window, _cx: &App) -> Option<IconName> {
        Some(IconName::Bug)
    }

    fn icon_tooltip(&self, _window: &gpui::Window, _cx: &App) -> Option<&'static str> {
        Some("Agent Debug Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }

    fn starts_open(&self, _window: &gpui::Window, _cx: &App) -> bool {
        false
    }

    fn activation_priority(&self) -> u32 {
        100
    }
}

impl Render for AgentDebugPanel {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let participants = self
            .active_call
            .as_ref()
            .and_then(|call| {
                call.read(cx)
                    .room()
                    .map(|room| room.read(cx).remote_participants().collect::<Vec<_>>())
            })
            .unwrap_or_default();

        let local_participant = self.active_call.as_ref().and_then(|call| {
            call.read(cx)
                .room()
                .map(|room| room.read(cx).local_participant())
        });

        v_flex()
            .key_context("AgentDebugPanel")
            .size_full()
            .child(
                h_flex()
                    .p_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(Label::new("Agent Activity Monitor").size(ui::LabelSize::Large)),
            )
            .child(
                div()
                    .flex_1()
                    .overflow_y_scroll()
                    .p_2()
                    .child(
                        v_flex()
                            .gap_2()
                            .when(local_participant.is_none() && participants.is_empty(), |this| {
                                this.child(
                                    Label::new("No active call")
                                        .color(Color::Muted)
                                        .size(ui::LabelSize::Small),
                                )
                            })
                            .children(local_participant.map(|participant| {
                                render_participant_activity(&participant, true, cx)
                            }))
                            .children(participants.into_iter().map(|participant| {
                                render_participant_activity(participant, false, cx)
                            })),
                    ),
            )
    }
}

fn render_participant_activity(
    participant: &call::participant::Participant,
    is_local: bool,
    cx: &App,
) -> impl IntoElement {
    let agent_activity = match participant {
        call::participant::Participant::Local(local) => &local.agent_activity,
        call::participant::Participant::Remote(remote) => &remote.agent_activity,
    };

    let user_name = match participant {
        call::participant::Participant::Local(_) => "You (Local)".to_string(),
        call::participant::Participant::Remote(remote) => {
            format!("{} (Remote)", remote.user.github_login)
        }
    };

    let (status_text, status_color, bg_color) = if let Some(activity) = agent_activity {
        match activity.status {
            call::participant::AgentActivityStatus::Active => (
                "Active",
                Color::Success,
                Some(cx.theme().status().success_background.opacity(0.1)),
            ),
            call::participant::AgentActivityStatus::Idle => ("Idle", Color::Muted, None),
        }
    } else {
        ("No Activity", Color::Muted, None)
    };

    div()
        .id(format!("participant-{}", user_name))
        .child(
            v_flex()
                .gap_2()
                .p_3()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .when_some(bg_color, |this, color| this.bg(color))
                .child(
                    h_flex()
                        .justify_between()
                        .child(
                            Label::new(user_name)
                                .size(ui::LabelSize::Default)
                                .weight(ui::FontWeight::BOLD),
                        )
                        .child(
                            Label::new(status_text)
                                .size(ui::LabelSize::Small)
                                .color(status_color),
                        ),
                )
                .children(agent_activity.as_ref().map(|activity| {
                    v_flex()
                        .gap_1()
                        .pt_1()
                        .child(
                            Label::new(format!("Agent: {}", activity.agent_type))
                                .size(ui::LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .children(activity.prompt_summary.as_ref().map(|summary| {
                            v_flex().gap_1().child(
                                Label::new("Prompt Summary:")
                                    .size(ui::LabelSize::Small)
                                    .color(Color::Muted),
                            ).child(
                                div()
                                    .p_2()
                                    .rounded_sm()
                                    .bg(cx.theme().colors().surface_background)
                                    .child(
                                        Label::new(summary.as_ref())
                                            .size(ui::LabelSize::Small)
                                            .color(Color::Default),
                                    ),
                            )
                        }))
                })),
        )
}

