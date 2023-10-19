use crate::{
    chat_panel::ChatPanel, format_timestamp, is_channels_feature_enabled, render_avatar,
    NotificationPanelSettings,
};
use anyhow::Result;
use channel::ChannelStore;
use client::{Client, Notification, User, UserStore};
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
use std::{sync::Arc, time::Duration};
use theme::{IconButton, Theme};
use time::{OffsetDateTime, UtcOffset};
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel},
    Workspace,
};

const TOAST_DURATION: Duration = Duration::from_secs(5);
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
    current_notification_toast: Option<(u64, Task<()>)>,
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

pub struct NotificationPresenter {
    pub actor: Option<Arc<client::User>>,
    pub text: String,
    pub icon: &'static str,
    pub needs_response: bool,
    pub can_navigate: bool,
}

actions!(notification_panel, [ToggleFocus]);

pub fn init(_cx: &mut AppContext) {}

impl NotificationPanel {
    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> ViewHandle<Self> {
        let fs = workspace.app_state().fs.clone();
        let client = workspace.app_state().client.clone();
        let user_store = workspace.app_state().user_store.clone();
        let workspace_handle = workspace.weak_handle();

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

            let notification_list =
                ListState::<Self>::new(0, Orientation::Top, 1000., move |this, ix, cx| {
                    this.render_notification(ix, cx)
                        .unwrap_or_else(|| Empty::new().into_any())
                });

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
                current_notification_toast: None,
                subscriptions: Vec::new(),
                active: false,
                width: None,
            };

            let mut old_dock_position = this.position(cx);
            this.subscriptions.extend([
                cx.observe(&this.notification_store, |_, _, cx| cx.notify()),
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

    fn render_notification(
        &mut self,
        ix: usize,
        cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement<Self>> {
        let entry = self.notification_store.read(cx).notification_at(ix)?;
        let now = OffsetDateTime::now_utc();
        let timestamp = entry.timestamp;
        let NotificationPresenter {
            actor,
            text,
            icon,
            needs_response,
            can_navigate,
        } = self.present_notification(entry, cx)?;

        let theme = theme::current(cx);
        let style = &theme.notification_panel;
        let response = entry.response;
        let notification = entry.notification.clone();

        let message_style = if entry.is_read {
            style.read_text.clone()
        } else {
            style.unread_text.clone()
        };

        enum Decline {}
        enum Accept {}

        Some(
            MouseEventHandler::new::<NotificationEntry, _>(ix, cx, |_, cx| {
                let container = message_style.container;

                Flex::column()
                    .with_child(
                        Flex::row()
                            .with_children(
                                actor.map(|actor| render_avatar(actor.avatar.clone(), &theme)),
                            )
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
                    .with_child(Text::new(text, message_style.text.clone()))
                    .with_children(if let Some(is_accepted) = response {
                        Some(
                            Label::new(
                                if is_accepted { "Accepted" } else { "Declined" },
                                style.button.text.clone(),
                            )
                            .into_any(),
                        )
                    } else if needs_response {
                        Some(
                            Flex::row()
                                .with_children([
                                    MouseEventHandler::new::<Decline, _>(ix, cx, |state, _| {
                                        let button = style.button.style_for(state);
                                        Label::new("Decline", button.text.clone())
                                            .contained()
                                            .with_style(button.container)
                                    })
                                    .with_cursor_style(CursorStyle::PointingHand)
                                    .on_click(
                                        MouseButton::Left,
                                        {
                                            let notification = notification.clone();
                                            move |_, view, cx| {
                                                view.respond_to_notification(
                                                    notification.clone(),
                                                    false,
                                                    cx,
                                                );
                                            }
                                        },
                                    ),
                                    MouseEventHandler::new::<Accept, _>(ix, cx, |state, _| {
                                        let button = style.button.style_for(state);
                                        Label::new("Accept", button.text.clone())
                                            .contained()
                                            .with_style(button.container)
                                    })
                                    .with_cursor_style(CursorStyle::PointingHand)
                                    .on_click(
                                        MouseButton::Left,
                                        {
                                            let notification = notification.clone();
                                            move |_, view, cx| {
                                                view.respond_to_notification(
                                                    notification.clone(),
                                                    true,
                                                    cx,
                                                );
                                            }
                                        },
                                    ),
                                ])
                                .aligned()
                                .right()
                                .into_any(),
                        )
                    } else {
                        None
                    })
                    .contained()
                    .with_style(container)
                    .into_any()
            })
            .with_cursor_style(if can_navigate {
                CursorStyle::PointingHand
            } else {
                CursorStyle::default()
            })
            .on_click(MouseButton::Left, {
                let notification = notification.clone();
                move |_, this, cx| this.did_click_notification(&notification, cx)
            })
            .into_any(),
        )
    }

    fn present_notification(
        &self,
        entry: &NotificationEntry,
        cx: &AppContext,
    ) -> Option<NotificationPresenter> {
        let user_store = self.user_store.read(cx);
        let channel_store = self.channel_store.read(cx);
        match entry.notification {
            Notification::ContactRequest { sender_id } => {
                let requester = user_store.get_cached_user(sender_id)?;
                Some(NotificationPresenter {
                    icon: "icons/plus.svg",
                    text: format!("{} wants to add you as a contact", requester.github_login),
                    needs_response: user_store.is_contact_request_pending(&requester),
                    actor: Some(requester),
                    can_navigate: false,
                })
            }
            Notification::ContactRequestAccepted { responder_id } => {
                let responder = user_store.get_cached_user(responder_id)?;
                Some(NotificationPresenter {
                    icon: "icons/plus.svg",
                    text: format!("{} accepted your contact invite", responder.github_login),
                    needs_response: false,
                    actor: Some(responder),
                    can_navigate: false,
                })
            }
            Notification::ChannelInvitation {
                ref channel_name,
                channel_id,
                inviter_id,
            } => {
                let inviter = user_store.get_cached_user(inviter_id)?;
                Some(NotificationPresenter {
                    icon: "icons/hash.svg",
                    text: format!(
                        "{} invited you to join the #{channel_name} channel",
                        inviter.github_login
                    ),
                    needs_response: channel_store.has_channel_invitation(channel_id),
                    actor: Some(inviter),
                    can_navigate: false,
                })
            }
            Notification::ChannelMessageMention {
                sender_id,
                channel_id,
                message_id,
            } => {
                let sender = user_store.get_cached_user(sender_id)?;
                let channel = channel_store.channel_for_id(channel_id)?;
                let message = self
                    .notification_store
                    .read(cx)
                    .channel_message_for_id(message_id)?;
                Some(NotificationPresenter {
                    icon: "icons/conversations.svg",
                    text: format!(
                        "{} mentioned you in the #{} channel:\n{}",
                        sender.github_login, channel.name, message.body,
                    ),
                    needs_response: false,
                    actor: Some(sender),
                    can_navigate: true,
                })
            }
        }
    }

    fn did_click_notification(&mut self, notification: &Notification, cx: &mut ViewContext<Self>) {
        if let Notification::ChannelMessageMention {
            message_id,
            channel_id,
            ..
        } = notification.clone()
        {
            if let Some(workspace) = self.workspace.upgrade(cx) {
                cx.app_context().defer(move |cx| {
                    workspace.update(cx, |workspace, cx| {
                        if let Some(panel) = workspace.focus_panel::<ChatPanel>(cx) {
                            panel.update(cx, |panel, cx| {
                                panel
                                    .select_channel(channel_id, Some(message_id), cx)
                                    .detach_and_log_err(cx);
                            });
                        }
                    });
                });
            }
        }
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
            NotificationEvent::NotificationRemoved { entry }
            | NotificationEvent::NotificationRead { entry } => self.remove_toast(entry.id, cx),
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
        let Some(NotificationPresenter { actor, text, .. }) = self.present_notification(entry, cx)
        else {
            return;
        };

        let notification_id = entry.id;
        self.current_notification_toast = Some((
            notification_id,
            cx.spawn(|this, mut cx| async move {
                cx.background().timer(TOAST_DURATION).await;
                this.update(&mut cx, |this, cx| this.remove_toast(notification_id, cx))
                    .ok();
            }),
        ));

        self.workspace
            .update(cx, |workspace, cx| {
                workspace.show_notification(0, cx, |cx| {
                    let workspace = cx.weak_handle();
                    cx.add_view(|_| NotificationToast {
                        notification_id,
                        actor,
                        text,
                        workspace,
                    })
                })
            })
            .ok();
    }

    fn remove_toast(&mut self, notification_id: u64, cx: &mut ViewContext<Self>) {
        if let Some((current_id, _)) = &self.current_notification_toast {
            if *current_id == notification_id {
                self.current_notification_toast.take();
                self.workspace
                    .update(cx, |workspace, cx| {
                        workspace.dismiss_notification::<NotificationToast>(0, cx)
                    })
                    .ok();
            }
        }
    }

    fn respond_to_notification(
        &mut self,
        notification: Notification,
        response: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.notification_store.update(cx, |store, cx| {
            store.respond_to_notification(notification, response, cx);
        });
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

pub struct NotificationToast {
    notification_id: u64,
    actor: Option<Arc<User>>,
    text: String,
    workspace: WeakViewHandle<Workspace>,
}

pub enum ToastEvent {
    Dismiss,
}

impl NotificationToast {
    fn focus_notification_panel(&self, cx: &mut AppContext) {
        let workspace = self.workspace.clone();
        let notification_id = self.notification_id;
        cx.defer(move |cx| {
            workspace
                .update(cx, |workspace, cx| {
                    if let Some(panel) = workspace.focus_panel::<NotificationPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            let store = panel.notification_store.read(cx);
                            if let Some(entry) = store.notification_for_id(notification_id) {
                                panel.did_click_notification(&entry.clone().notification, cx);
                            }
                        });
                    }
                })
                .ok();
        })
    }
}

impl Entity for NotificationToast {
    type Event = ToastEvent;
}

impl View for NotificationToast {
    fn ui_name() -> &'static str {
        "ContactNotification"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let user = self.actor.clone();
        let theme = theme::current(cx).clone();
        let theme = &theme.contact_notification;

        MouseEventHandler::new::<Self, _>(0, cx, |_, cx| {
            Flex::row()
                .with_children(user.and_then(|user| {
                    Some(
                        Image::from_data(user.avatar.clone()?)
                            .with_style(theme.header_avatar)
                            .aligned()
                            .constrained()
                            .with_height(
                                cx.font_cache()
                                    .line_height(theme.header_message.text.font_size),
                            )
                            .aligned()
                            .top(),
                    )
                }))
                .with_child(
                    Text::new(self.text.clone(), theme.header_message.text.clone())
                        .contained()
                        .with_style(theme.header_message.container)
                        .aligned()
                        .top()
                        .left()
                        .flex(1., true),
                )
                .with_child(
                    MouseEventHandler::new::<ToastEvent, _>(0, cx, |state, _| {
                        let style = theme.dismiss_button.style_for(state);
                        Svg::new("icons/x.svg")
                            .with_color(style.color)
                            .constrained()
                            .with_width(style.icon_width)
                            .aligned()
                            .contained()
                            .with_style(style.container)
                            .constrained()
                            .with_width(style.button_width)
                            .with_height(style.button_width)
                    })
                    .with_cursor_style(CursorStyle::PointingHand)
                    .with_padding(Padding::uniform(5.))
                    .on_click(MouseButton::Left, move |_, _, cx| {
                        cx.emit(ToastEvent::Dismiss)
                    })
                    .aligned()
                    .constrained()
                    .with_height(
                        cx.font_cache()
                            .line_height(theme.header_message.text.font_size),
                    )
                    .aligned()
                    .top()
                    .flex_float(),
                )
                .contained()
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, this, cx| {
            this.focus_notification_panel(cx);
            cx.emit(ToastEvent::Dismiss);
        })
        .into_any()
    }
}

impl workspace::notifications::Notification for NotificationToast {
    fn should_dismiss_notification_on_event(&self, event: &<Self as Entity>::Event) -> bool {
        matches!(event, ToastEvent::Dismiss)
    }
}
