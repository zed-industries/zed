use call::ActiveCall;
use channel::ChannelStore;
use gpui::{
    App, Context, Entity, EventEmitter, Render, SharedString, Subscription, WeakEntity, Window,
};
use std::rc::Rc;
use title_bar::collab::{toggle_deafen, toggle_mute, toggle_screen_sharing};
use ui::{
    ButtonStyle, IconButton, IconName, IconSize, Label, LabelSize, TintColor, Tooltip, prelude::*,
};
use workspace::{StatusItemView, Workspace, item::ItemHandle};

pub struct CollabOverlayStatusItem {
    workspace: WeakEntity<Workspace>,
    _subscriptions: Vec<Subscription>,
}

pub enum Event {
    CallStateChanged,
}

impl EventEmitter<Event> for CollabOverlayStatusItem {}

impl CollabOverlayStatusItem {
    pub fn new(_workspace: &Workspace, cx: &mut Context<Workspace>) -> Entity<Self> {
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
            self._subscriptions
                .push(cx.subscribe(&room, |_: &mut Self, _, _, cx| {
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

            if dock
                .panel_index_for_persistent_name("CollabPanel", cx)
                .is_some()
            {
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

    fn channel_name(&self, cx: &App) -> SharedString {
        let Some(room) = ActiveCall::global(cx).read(cx).room() else {
            return "Call".into();
        };

        let channel_id = room.read(cx).channel_id();

        if let Some(channel_id) = channel_id {
            let channel_store = ChannelStore::global(cx);
            if let Some(channel) = channel_store.read(cx).channel_for_id(channel_id) {
                return channel.name.clone();
            }
        }

        "Call".into()
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
        let is_deafened = room_read.is_deafened().unwrap_or(false);
        let is_screen_sharing = room_read.is_sharing_screen();
        let can_use_microphone = room_read.can_use_microphone();
        let screen_sharing_supported = cx.is_screen_capture_supported();

        let channel_name = self.channel_name(cx);

        h_flex()
            .id("collab-status-item")
            .gap_1()
            .items_center()
            .child(
                h_flex()
                    .id("collab-status-call-info")
                    .gap_1()
                    .items_center()
                    .cursor_pointer()
                    .child(
                        Icon::new(IconName::AudioOn)
                            .size(IconSize::Small)
                            .color(Color::Success),
                    )
                    .child(
                        Label::new(channel_name)
                            .size(LabelSize::Small)
                            .color(Color::Default),
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.open_collab_panel(window, cx);
                    }))
                    .tooltip(Tooltip::text("Open collaboration panel")),
            )
            .when(can_use_microphone, |this| {
                this.child(
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
            })
            .child(
                IconButton::new("status-deafen", IconName::AudioOn)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text(if is_deafened {
                        "Unmute Audio"
                    } else {
                        "Mute Audio"
                    }))
                    .selected_icon(IconName::AudioOff)
                    .selected_icon_color(Color::Error)
                    .toggle_state(is_deafened)
                    .on_click(|_, _, cx| {
                        toggle_deafen(cx);
                    }),
            )
            .when(can_use_microphone && screen_sharing_supported, |this| {
                this.child(
                    IconButton::new("status-screen-share", IconName::Screen)
                        .icon_size(IconSize::Small)
                        .tooltip(Tooltip::text(if is_screen_sharing {
                            "Stop Sharing Screen"
                        } else {
                            "Share Screen"
                        }))
                        .toggle_state(is_screen_sharing)
                        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                        .on_click(move |_, window, cx| {
                            let should_share = ActiveCall::global(cx)
                                .read(cx)
                                .room()
                                .is_some_and(|room| !room.read(cx).is_sharing_screen());

                            window
                                .spawn(cx, async move |cx| {
                                    let screen = if should_share {
                                        cx.update(|_, cx| pick_default_screen(cx))?.await
                                    } else {
                                        Ok(None)
                                    };
                                    cx.update(|window, cx| {
                                        toggle_screen_sharing(screen, window, cx)
                                    })?;
                                    Result::<_, anyhow::Error>::Ok(())
                                })
                                .detach();
                        }),
                )
            })
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

fn pick_default_screen(
    cx: &App,
) -> gpui::Task<anyhow::Result<Option<Rc<dyn gpui::ScreenCaptureSource>>>> {
    let source = cx.screen_capture_sources();
    cx.spawn(async move |_| {
        let available_sources = source.await??;
        Ok(available_sources
            .iter()
            .find(|it| {
                it.as_ref()
                    .metadata()
                    .is_ok_and(|meta| meta.is_main.unwrap_or_default())
            })
            .or_else(|| available_sources.first())
            .cloned())
    })
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
