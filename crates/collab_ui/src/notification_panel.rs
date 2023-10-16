use crate::{
    format_timestamp, is_channels_feature_enabled,
    notifications::contact_notification::ContactNotification, render_avatar,
    NotificationPanelSettings,
};
use anyhow::Result;
use channel::ChannelStore;
use client::{Client, Notification, UserStore};
use db::kvp::KEY_VALUE_STORE;
use futures::StreamExt;
use gpui::{
    actions,
    elements::*,
    platform::{CursorStyle, MouseButton},
    serde_json, AnyViewHandle, AppContext, AsyncAppContext, Entity, ModelHandle, Task, View,
    ViewContext, ViewHandle, WeakViewHandle, WindowContext,
};
use notifications::{NotificationEntry, NotificationEvent, NotificationStore};
use project::Fs;
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use std::sync::Arc;
use theme::{IconButton, Theme};
use time::{OffsetDateTime, UtcOffset};
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel},
    Workspace,
};

const NOTIFICATION_PANEL_KEY: &'static str = "NotificationPanel";

pub struct NotificationPanel {
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
    channel_store: ModelHandle<ChannelStore>,
    notification_store: ModelHandle<NotificationStore>,
    fs: Arc<dyn Fs>,
    width: Option<f32>,
    active: bool,
    notification_list: ListState<Self>,
    pending_serialization: Task<Option<()>>,
    subscriptions: Vec<gpui::Subscription>,
    workspace: WeakViewHandle<Workspace>,
    local_timezone: UtcOffset,
    has_focus: bool,
}

#[derive(Serialize, Deserialize)]
struct SerializedNotificationPanel {
    width: Option<f32>,
}

#[derive(Debug)]
pub enum Event {
    DockPositionChanged,
    Focus,
    Dismissed,
}

actions!(chat_panel, [ToggleFocus]);

pub fn init(_cx: &mut AppContext) {}

impl NotificationPanel {
    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> ViewHandle<Self> {
        let fs = workspace.app_state().fs.clone();
        let client = workspace.app_state().client.clone();
        let user_store = workspace.app_state().user_store.clone();
        let workspace_handle = workspace.weak_handle();

        let notification_list =
            ListState::<Self>::new(0, Orientation::Top, 1000., move |this, ix, cx| {
                this.render_notification(ix, cx)
            });

        cx.add_view(|cx| {
            let mut status = client.status();

            cx.spawn(|this, mut cx| async move {
                while let Some(_) = status.next().await {
                    if this
                        .update(&mut cx, |_, cx| {
                            cx.notify();
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            })
            .detach();

            let mut this = Self {
                fs,
                client,
                user_store,
                local_timezone: cx.platform().local_timezone(),
                channel_store: ChannelStore::global(cx),
                notification_store: NotificationStore::global(cx),
                notification_list,
                pending_serialization: Task::ready(None),
                workspace: workspace_handle,
                has_focus: false,
                subscriptions: Vec::new(),
                active: false,
                width: None,
            };

            let mut old_dock_position = this.position(cx);
            this.subscriptions.extend([
                cx.subscribe(&this.notification_store, Self::on_notification_event),
                cx.observe_global::<SettingsStore, _>(move |this: &mut Self, cx| {
                    let new_dock_position = this.position(cx);
                    if new_dock_position != old_dock_position {
                        old_dock_position = new_dock_position;
                        cx.emit(Event::DockPositionChanged);
                    }
                    cx.notify();
                }),
            ]);
            this
        })
    }

    pub fn load(
        workspace: WeakViewHandle<Workspace>,
        cx: AsyncAppContext,
    ) -> Task<Result<ViewHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            let serialized_panel = if let Some(panel) = cx
                .background()
                .spawn(async move { KEY_VALUE_STORE.read_kvp(NOTIFICATION_PANEL_KEY) })
                .await
                .log_err()
                .flatten()
            {
                Some(serde_json::from_str::<SerializedNotificationPanel>(&panel)?)
            } else {
                None
            };

            workspace.update(&mut cx, |workspace, cx| {
                let panel = Self::new(workspace, cx);
                if let Some(serialized_panel) = serialized_panel {
                    panel.update(cx, |panel, cx| {
                        panel.width = serialized_panel.width;
                        cx.notify();
                    });
                }
                panel
            })
        })
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        NOTIFICATION_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedNotificationPanel { width })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn render_notification(&mut self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        self.try_render_notification(ix, cx)
            .unwrap_or_else(|| Empty::new().into_any())
    }

    fn try_render_notification(
        &mut self,
        ix: usize,
        cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement<Self>> {
        let notification_store = self.notification_store.read(cx);
        let user_store = self.user_store.read(cx);
        let channel_store = self.channel_store.read(cx);
        let entry = notification_store.notification_at(ix)?;
        let now = OffsetDateTime::now_utc();
        let timestamp = entry.timestamp;

        let icon;
        let text;
        let actor;
        match entry.notification {
            Notification::ContactRequest {
                actor_id: requester_id,
            } => {
                actor = user_store.get_cached_user(requester_id)?;
                icon = "icons/plus.svg";
                text = format!("{} wants to add you as a contact", actor.github_login);
            }
            Notification::ContactRequestAccepted {
                actor_id: contact_id,
            } => {
                actor = user_store.get_cached_user(contact_id)?;
                icon = "icons/plus.svg";
                text = format!("{} accepted your contact invite", actor.github_login);
            }
            Notification::ChannelInvitation {
                actor_id: inviter_id,
                channel_id,
            } => {
                actor = user_store.get_cached_user(inviter_id)?;
                let channel = channel_store.channel_for_id(channel_id).or_else(|| {
                    channel_store
                        .channel_invitations()
                        .iter()
                        .find(|c| c.id == channel_id)
                })?;

                icon = "icons/hash.svg";
                text = format!(
                    "{} invited you to join the #{} channel",
                    actor.github_login, channel.name
                );
            }
            Notification::ChannelMessageMention {
                actor_id: sender_id,
                channel_id,
                message_id,
            } => {
                actor = user_store.get_cached_user(sender_id)?;
                let channel = channel_store.channel_for_id(channel_id)?;
                let message = notification_store.channel_message_for_id(message_id)?;

                icon = "icons/conversations.svg";
                text = format!(
                    "{} mentioned you in the #{} channel:\n{}",
                    actor.github_login, channel.name, message.body,
                );
            }
        }

        let theme = theme::current(cx);
        let style = &theme.chat_panel.message;

        Some(
            MouseEventHandler::new::<NotificationEntry, _>(ix, cx, |state, _| {
                let container = style.container.style_for(state);

                Flex::column()
                    .with_child(
                        Flex::row()
                            .with_child(render_avatar(actor.avatar.clone(), &theme))
                            .with_child(render_icon_button(&theme.chat_panel.icon_button, icon))
                            .with_child(
                                Label::new(
                                    format_timestamp(timestamp, now, self.local_timezone),
                                    style.timestamp.text.clone(),
                                )
                                .contained()
                                .with_style(style.timestamp.container),
                            )
                            .align_children_center(),
                    )
                    .with_child(Text::new(text, style.body.clone()))
                    .contained()
                    .with_style(*container)
                    .into_any()
            })
            .into_any(),
        )
    }

    fn render_sign_in_prompt(
        &self,
        theme: &Arc<Theme>,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        enum SignInPromptLabel {}

        MouseEventHandler::new::<SignInPromptLabel, _>(0, cx, |mouse_state, _| {
            Label::new(
                "Sign in to view your notifications".to_string(),
                theme
                    .chat_panel
                    .sign_in_prompt
                    .style_for(mouse_state)
                    .clone(),
            )
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, this, cx| {
            let client = this.client.clone();
            cx.spawn(|_, cx| async move {
                client.authenticate_and_connect(true, &cx).log_err().await;
            })
            .detach();
        })
        .aligned()
        .into_any()
    }

    fn render_empty_state(
        &self,
        theme: &Arc<Theme>,
        _cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        Label::new(
            "You have no notifications".to_string(),
            theme.chat_panel.sign_in_prompt.default.clone(),
        )
        .aligned()
        .into_any()
    }

    fn on_notification_event(
        &mut self,
        _: ModelHandle<NotificationStore>,
        event: &NotificationEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            NotificationEvent::NewNotification { entry } => self.add_toast(entry, cx),
            NotificationEvent::NotificationRemoved { entry } => self.remove_toast(entry, cx),
            NotificationEvent::NotificationsUpdated {
                old_range,
                new_count,
            } => {
                self.notification_list.splice(old_range.clone(), *new_count);
                cx.notify();
            }
        }
    }

    fn add_toast(&mut self, entry: &NotificationEntry, cx: &mut ViewContext<Self>) {
        let id = entry.id as usize;
        match entry.notification {
            Notification::ContactRequest { actor_id }
            | Notification::ContactRequestAccepted { actor_id } => {
                let user_store = self.user_store.clone();
                let Some(user) = user_store.read(cx).get_cached_user(actor_id) else {
                    return;
                };
                self.workspace
                    .update(cx, |workspace, cx| {
                        workspace.show_notification(id, cx, |cx| {
                            cx.add_view(|_| {
                                ContactNotification::new(
                                    user,
                                    entry.notification.clone(),
                                    user_store,
                                )
                            })
                        })
                    })
                    .ok();
            }
            Notification::ChannelInvitation { .. } => {}
            Notification::ChannelMessageMention { .. } => {}
        }
    }

    fn remove_toast(&mut self, entry: &NotificationEntry, cx: &mut ViewContext<Self>) {
        let id = entry.id as usize;
        match entry.notification {
            Notification::ContactRequest { .. } | Notification::ContactRequestAccepted { .. } => {
                self.workspace
                    .update(cx, |workspace, cx| {
                        workspace.dismiss_notification::<ContactNotification>(id, cx)
                    })
                    .ok();
            }
            Notification::ChannelInvitation { .. } => {}
            Notification::ChannelMessageMention { .. } => {}
        }
    }
}

impl Entity for NotificationPanel {
    type Event = Event;
}

impl View for NotificationPanel {
    fn ui_name() -> &'static str {
        "NotificationPanel"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = theme::current(cx);
        let element = if self.client.user_id().is_none() {
            self.render_sign_in_prompt(&theme, cx)
        } else if self.notification_list.item_count() == 0 {
            self.render_empty_state(&theme, cx)
        } else {
            List::new(self.notification_list.clone())
                .contained()
                .with_style(theme.chat_panel.list)
                .into_any()
        };
        element
            .contained()
            .with_style(theme.chat_panel.container)
            .constrained()
            .with_min_width(150.)
            .into_any()
    }

    fn focus_in(&mut self, _: AnyViewHandle, _: &mut ViewContext<Self>) {
        self.has_focus = true;
    }

    fn focus_out(&mut self, _: AnyViewHandle, _: &mut ViewContext<Self>) {
        self.has_focus = false;
    }
}

impl Panel for NotificationPanel {
    fn position(&self, cx: &gpui::WindowContext) -> DockPosition {
        settings::get::<NotificationPanelSettings>(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<NotificationPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings| settings.dock = Some(position),
        );
    }

    fn size(&self, cx: &gpui::WindowContext) -> f32 {
        self.width
            .unwrap_or_else(|| settings::get::<NotificationPanelSettings>(cx).default_width)
    }

    fn set_size(&mut self, size: Option<f32>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        self.active = active;
        if active {
            if !is_channels_feature_enabled(cx) {
                cx.emit(Event::Dismissed);
            }
        }
    }

    fn icon_path(&self, cx: &gpui::WindowContext) -> Option<&'static str> {
        (settings::get::<NotificationPanelSettings>(cx).button && is_channels_feature_enabled(cx))
            .then(|| "icons/bell.svg")
    }

    fn icon_tooltip(&self) -> (String, Option<Box<dyn gpui::Action>>) {
        (
            "Notification Panel".to_string(),
            Some(Box::new(ToggleFocus)),
        )
    }

    fn icon_label(&self, cx: &WindowContext) -> Option<String> {
        let count = self.notification_store.read(cx).unread_notification_count();
        if count == 0 {
            None
        } else {
            Some(count.to_string())
        }
    }

    fn should_change_position_on_event(event: &Self::Event) -> bool {
        matches!(event, Event::DockPositionChanged)
    }

    fn should_close_on_event(event: &Self::Event) -> bool {
        matches!(event, Event::Dismissed)
    }

    fn has_focus(&self, _cx: &gpui::WindowContext) -> bool {
        self.has_focus
    }

    fn is_focus_event(event: &Self::Event) -> bool {
        matches!(event, Event::Focus)
    }
}

fn render_icon_button<V: View>(style: &IconButton, svg_path: &'static str) -> impl Element<V> {
    Svg::new(svg_path)
        .with_color(style.color)
        .constrained()
        .with_width(style.icon_width)
        .aligned()
        .constrained()
        .with_width(style.button_width)
        .with_height(style.button_width)
        .contained()
        .with_style(style.container)
}
