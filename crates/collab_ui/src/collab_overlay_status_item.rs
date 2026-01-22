use call::ActiveCall;
use gpui::{App, Context, Entity, EventEmitter, Render, Subscription, WeakEntity, Window};
use title_bar::collab::toggle_mute;
use ui::{px, Avatar, IconButton, IconName, IconSize, Tooltip, prelude::*};
use workspace::{StatusItemView, Workspace, item::ItemHandle};

const MAX_VISIBLE_AVATARS: usize = 3;

pub struct CollabOverlayStatusItem {
    workspace: WeakEntity<Workspace>,
    _subscriptions: Vec<Subscription>,
}

pub enum Event {
    CallStateChanged,
}

impl EventEmitter<Event> for CollabOverlayStatusItem {}

impl CollabOverlayStatusItem {
    pub fn new(
        _workspace: &Workspace,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let workspace_weak = cx.weak_entity();
        cx.new(|cx: &mut Context<Self>| {
            let active_call = ActiveCall::global(cx);

            let mut subscriptions = vec![cx.observe(&active_call, |this: &mut Self, _, cx| {
                cx.emit(Event::CallStateChanged);
                this.subscribe_to_room(cx);
                cx.notify();
            })];

            let room = active_call.read(cx).room().cloned();
            if let Some(room) = room {
                subscriptions.push(cx.subscribe(&room, |_: &mut Self, _, _, cx| {
                    cx.notify();
                }));
            }

            Self {
                workspace: workspace_weak,
                _subscriptions: subscriptions,
            }
        })
    }

    fn subscribe_to_room(&mut self, cx: &mut Context<Self>) {
        let active_call = ActiveCall::global(cx);
        let room = active_call.read(cx).room().cloned();
        if let Some(room) = room {
            self._subscriptions.push(cx.subscribe(&room, |_: &mut Self, _, _, cx| {
                cx.notify();
            }));
        }
    }

    fn is_in_call(&self, cx: &App) -> bool {
        ActiveCall::global(cx).read(cx).room().is_some()
    }

    fn is_collab_panel_dock_open(&self, cx: &App) -> bool {
        let Some(workspace) = self.workspace.upgrade() else {
            return false;
        };

        let workspace = workspace.read(cx);

        for dock in workspace.all_docks() {
            let dock = dock.read(cx);

            // Check if this dock contains the CollabPanel
            if dock.panel_index_for_persistent_name("CollabPanel", cx).is_some() {
                return dock.is_open();
            }
        }

        false
    }

    fn should_show(&self, cx: &App) -> bool {
        self.is_in_call(cx) && !self.is_collab_panel_dock_open(cx)
    }

    fn open_collab_panel(&self, window: &mut Window, cx: &mut App) {
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_panel_focus::<crate::collab_panel::CollabPanel>(window, cx);
            });
        }
    }

    fn render_avatars(&self, cx: &App) -> Vec<impl IntoElement> {
        let Some(room) = ActiveCall::global(cx).read(cx).room() else {
            return Vec::new();
        };

        let room = room.read(cx);
        let mut avatars: Vec<Div> = Vec::new();
        let mut total_participants: usize = 0;

        if let Some(user) = room.local_participant_user(cx) {
            total_participants += 1;
            if avatars.len() < MAX_VISIBLE_AVATARS {
                avatars.push(
                    div().child(
                        Avatar::new(user.avatar_uri.to_string()).size(px(16.)),
                    ),
                );
            }
        }

        for (_, participant) in room.remote_participants() {
            total_participants += 1;
            if avatars.len() < MAX_VISIBLE_AVATARS {
                avatars.push(
                    div().child(
                        Avatar::new(participant.user.avatar_uri.to_string()).size(px(16.)),
                    ),
                );
            }
        }

        let overflow = total_participants.saturating_sub(MAX_VISIBLE_AVATARS);
        if overflow > 0 {
            avatars.push(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size_4()
                    .rounded_full()
                    .bg(cx.theme().colors().element_background)
                    .child(
                        Label::new(format!("+{}", overflow))
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    ),
            );
        }

        avatars
    }
}

impl Render for CollabOverlayStatusItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.should_show(cx) {
            return div().into_any_element();
        }

        let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() else {
            return div().into_any_element();
        };

        let room_read = room.read(cx);
        let is_muted = room_read.is_muted();

        let avatars = self.render_avatars(cx);

        h_flex()
            .id("collab-status-item")
            .gap_1()
            .items_center()
            .child(
                h_flex()
                    .id("collab-status-avatars")
                    .gap_0p5()
                    .cursor_pointer()
                    .children(avatars)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.open_collab_panel(window, cx);
                    }))
                    .tooltip(Tooltip::text("Open collaboration panel")),
            )
            .child(
                IconButton::new("status-mute", IconName::Mic)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text(if is_muted { "Unmute" } else { "Mute" }))
                    .selected_icon(IconName::MicMute)
                    .selected_icon_color(Color::Error)
                    .toggle_state(is_muted)
                    .on_click(|_, _, cx| {
                        toggle_mute(cx);
                    }),
            )
            .child(
                IconButton::new("status-leave", IconName::Exit)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Error)
                    .tooltip(Tooltip::text("Leave call"))
                    .on_click(|_, _, cx| {
                        ActiveCall::global(cx)
                            .update(cx, |call, cx| call.hang_up(cx))
                            .detach_and_log_err(cx);
                    }),
            )
            .into_any_element()
    }
}

impl StatusItemView for CollabOverlayStatusItem {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}
