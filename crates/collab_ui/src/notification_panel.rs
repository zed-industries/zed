use crate::{chat_panel::ChatPanel, NotificationPanelSettings};
use anyhow::Result;
use channel::ChannelStore;
use client::{ChannelId, Client, Notification, User, UserStore};
use collections::HashMap;
use db::kvp::KEY_VALUE_STORE;
use futures::StreamExt;
use gpui::{
    actions, div, img, list, px, AnyElement, AppContext, AsyncWindowContext, CursorStyle,
    DismissEvent, Element, EventEmitter, FocusHandle, FocusableView, InteractiveElement,
    IntoElement, ListAlignment, ListScrollEvent, ListState, Model, ParentElement, Render,
    StatefulInteractiveElement, Styled, Task, View, ViewContext, VisualContext, WeakView,
    WindowContext,
};
use notifications::{NotificationEntry, NotificationEvent, NotificationStore};
use project::Fs;
use rpc::proto;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::{sync::Arc, time::Duration};
use time::{OffsetDateTime, UtcOffset};
use ui::{h_flex, prelude::*, v_flex, Avatar, Button, Icon, IconButton, IconName, Label};
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

const LOADING_THRESHOLD: usize = 30;
const MARK_AS_READ_DELAY: Duration = Duration::from_secs(1);
const TOAST_DURATION: Duration = Duration::from_secs(5);
const NOTIFICATION_PANEL_KEY: &str = "NotificationPanel";

pub struct NotificationPanel {
    client: Arc<Client>,
    user_store: Model<UserStore>,
    channel_store: Model<ChannelStore>,
    notification_store: Model<NotificationStore>,
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    active: bool,
    notification_list: ListState,
    pending_serialization: Task<Option<()>>,
    subscriptions: Vec<gpui::Subscription>,
    workspace: WeakView<Workspace>,
    current_notification_toast: Option<(u64, Task<()>)>,
    local_timezone: UtcOffset,
    focus_handle: FocusHandle,
    mark_as_read_tasks: HashMap<u64, Task<Result<()>>>,
    unseen_notifications: Vec<NotificationEntry>,
}

#[derive(Serialize, Deserialize)]
struct SerializedNotificationPanel {
    width: Option<Pixels>,
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

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, cx| {
            workspace.toggle_panel_focus::<NotificationPanel>(cx);
        });
    })
    .detach();
}

impl NotificationPanel {
    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let fs = workspace.app_state().fs.clone();
        let client = workspace.app_state().client.clone();
        let user_store = workspace.app_state().user_store.clone();
        let workspace_handle = workspace.weak_handle();

        cx.new_view(|cx: &mut ViewContext<Self>| {
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

            let view = cx.view().downgrade();
            let notification_list =
                ListState::new(0, ListAlignment::Top, px(1000.), move |ix, cx| {
                    view.upgrade()
                        .and_then(|view| {
                            view.update(cx, |this, cx| this.render_notification(ix, cx))
                        })
                        .unwrap_or_else(|| div().into_any())
                });
            notification_list.set_scroll_handler(cx.listener(
                |this, event: &ListScrollEvent, cx| {
                    if event.count.saturating_sub(event.visible_range.end) < LOADING_THRESHOLD {
                        if let Some(task) = this
                            .notification_store
                            .update(cx, |store, cx| store.load_more_notifications(false, cx))
                        {
                            task.detach();
                        }
                    }
                },
            ));

            let mut this = Self {
                fs,
                client,
                user_store,
                local_timezone: cx.local_timezone(),
                channel_store: ChannelStore::global(cx),
                notification_store: NotificationStore::global(cx),
                notification_list,
                pending_serialization: Task::ready(None),
                workspace: workspace_handle,
                focus_handle: cx.focus_handle(),
                current_notification_toast: None,
                subscriptions: Vec::new(),
                active: false,
                mark_as_read_tasks: HashMap::default(),
                width: None,
                unseen_notifications: Vec::new(),
            };

            let mut old_dock_position = this.position(cx);
            this.subscriptions.extend([
                cx.observe(&this.notification_store, |_, _, cx| cx.notify()),
                cx.subscribe(&this.notification_store, Self::on_notification_event),
                cx.observe_global::<SettingsStore>(move |this: &mut Self, cx| {
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
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let serialized_panel = if let Some(panel) = cx
                .background_executor()
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
                        panel.width = serialized_panel.width.map(|w| w.round());
                        cx.notify();
                    });
                }
                panel
            })
        })
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background_executor().spawn(
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

    fn render_notification(&mut self, ix: usize, cx: &mut ViewContext<Self>) -> Option<AnyElement> {
        let entry = self.notification_store.read(cx).notification_at(ix)?;
        let notification_id = entry.id;
        let now = OffsetDateTime::now_utc();
        let timestamp = entry.timestamp;
        let NotificationPresenter {
            actor,
            text,
            needs_response,
            can_navigate,
            ..
        } = self.present_notification(entry, cx)?;

        let response = entry.response;
        let notification = entry.notification.clone();

        if self.active && !entry.is_read {
            self.did_render_notification(notification_id, &notification, cx);
        }

        Some(
            div()
                .id(ix)
                .flex()
                .flex_row()
                .size_full()
                .px_2()
                .py_1()
                .gap_2()
                .when(can_navigate, |el| {
                    el.cursor(CursorStyle::PointingHand).on_click({
                        let notification = notification.clone();
                        cx.listener(move |this, _, cx| {
                            this.did_click_notification(&notification, cx)
                        })
                    })
                })
                .children(actor.map(|actor| {
                    img(actor.avatar_uri.clone())
                        .flex_none()
                        .w_8()
                        .h_8()
                        .rounded_full()
                }))
                .child(
                    v_flex()
                        .gap_1()
                        .size_full()
                        .overflow_hidden()
                        .child(Label::new(text.clone()))
                        .child(
                            h_flex()
                                .child(
                                    Label::new(format_timestamp(
                                        timestamp,
                                        now,
                                        self.local_timezone,
                                    ))
                                    .color(Color::Muted),
                                )
                                .children(if let Some(is_accepted) = response {
                                    Some(div().flex().flex_grow().justify_end().child(Label::new(
                                        if is_accepted {
                                            "You accepted"
                                        } else {
                                            "You declined"
                                        },
                                    )))
                                } else if needs_response {
                                    Some(
                                        h_flex()
                                            .flex_grow()
                                            .justify_end()
                                            .child(Button::new("decline", "Decline").on_click({
                                                let notification = notification.clone();
                                                let view = cx.view().clone();
                                                move |_, cx| {
                                                    view.update(cx, |this, cx| {
                                                        this.respond_to_notification(
                                                            notification.clone(),
                                                            false,
                                                            cx,
                                                        )
                                                    });
                                                }
                                            }))
                                            .child(Button::new("accept", "Accept").on_click({
                                                let notification = notification.clone();
                                                let view = cx.view().clone();
                                                move |_, cx| {
                                                    view.update(cx, |this, cx| {
                                                        this.respond_to_notification(
                                                            notification.clone(),
                                                            true,
                                                            cx,
                                                        )
                                                    });
                                                }
                                            })),
                                    )
                                } else {
                                    None
                                }),
                        ),
                )
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
                    needs_response: user_store.has_incoming_contact_request(requester.id),
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
                    needs_response: channel_store.has_channel_invitation(ChannelId(channel_id)),
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
                let channel = channel_store.channel_for_id(ChannelId(channel_id))?;
                let message = self
                    .notification_store
                    .read(cx)
                    .channel_message_for_id(message_id)?;
                Some(NotificationPresenter {
                    icon: "icons/conversations.svg",
                    text: format!(
                        "{} mentioned you in #{}:\n{}",
                        sender.github_login, channel.name, message.body,
                    ),
                    needs_response: false,
                    actor: Some(sender),
                    can_navigate: true,
                })
            }
        }
    }

    fn did_render_notification(
        &mut self,
        notification_id: u64,
        notification: &Notification,
        cx: &mut ViewContext<Self>,
    ) {
        let should_mark_as_read = match notification {
            Notification::ContactRequestAccepted { .. } => true,
            Notification::ContactRequest { .. }
            | Notification::ChannelInvitation { .. }
            | Notification::ChannelMessageMention { .. } => false,
        };

        if should_mark_as_read {
            self.mark_as_read_tasks
                .entry(notification_id)
                .or_insert_with(|| {
                    let client = self.client.clone();
                    cx.spawn(|this, mut cx| async move {
                        cx.background_executor().timer(MARK_AS_READ_DELAY).await;
                        client
                            .request(proto::MarkNotificationRead { notification_id })
                            .await?;
                        this.update(&mut cx, |this, _| {
                            this.mark_as_read_tasks.remove(&notification_id);
                        })?;
                        Ok(())
                    })
                });
        }
    }

    fn did_click_notification(&mut self, notification: &Notification, cx: &mut ViewContext<Self>) {
        if let Notification::ChannelMessageMention {
            message_id,
            channel_id,
            ..
        } = notification.clone()
        {
            if let Some(workspace) = self.workspace.upgrade() {
                cx.window_context().defer(move |cx| {
                    workspace.update(cx, |workspace, cx| {
                        if let Some(panel) = workspace.focus_panel::<ChatPanel>(cx) {
                            panel.update(cx, |panel, cx| {
                                panel
                                    .select_channel(ChannelId(channel_id), Some(message_id), cx)
                                    .detach_and_log_err(cx);
                            });
                        }
                    });
                });
            }
        }
    }

    fn is_showing_notification(&self, notification: &Notification, cx: &ViewContext<Self>) -> bool {
        if !self.active {
            return false;
        }

        if let Notification::ChannelMessageMention { channel_id, .. } = &notification {
            if let Some(workspace) = self.workspace.upgrade() {
                return if let Some(panel) = workspace.read(cx).panel::<ChatPanel>(cx) {
                    let panel = panel.read(cx);
                    panel.is_scrolled_to_bottom()
                        && panel
                            .active_chat()
                            .map_or(false, |chat| chat.read(cx).channel_id.0 == *channel_id)
                } else {
                    false
                };
            }
        }

        false
    }

    fn on_notification_event(
        &mut self,
        _: Model<NotificationStore>,
        event: &NotificationEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            NotificationEvent::NewNotification { entry } => {
                if !self.is_showing_notification(&entry.notification, cx) {
                    self.unseen_notifications.push(entry.clone());
                }
                self.add_toast(entry, cx);
            }
            NotificationEvent::NotificationRemoved { entry }
            | NotificationEvent::NotificationRead { entry } => {
                self.unseen_notifications.retain(|n| n.id != entry.id);
                self.remove_toast(entry.id, cx);
            }
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
        if self.is_showing_notification(&entry.notification, cx) {
            return;
        }

        let Some(NotificationPresenter { actor, text, .. }) = self.present_notification(entry, cx)
        else {
            return;
        };

        let notification_id = entry.id;
        self.current_notification_toast = Some((
            notification_id,
            cx.spawn(|this, mut cx| async move {
                cx.background_executor().timer(TOAST_DURATION).await;
                this.update(&mut cx, |this, cx| this.remove_toast(notification_id, cx))
                    .ok();
            }),
        ));

        self.workspace
            .update(cx, |workspace, cx| {
                workspace.dismiss_notification::<NotificationToast>(0, cx);
                workspace.show_notification(0, cx, |cx| {
                    let workspace = cx.view().downgrade();
                    cx.new_view(|_| NotificationToast {
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

impl Render for NotificationPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(
                h_flex()
                    .justify_between()
                    .px_2()
                    .py_1()
                    // Match the height of the tab bar so they line up.
                    .h(rems(ui::Tab::CONTAINER_HEIGHT_IN_REMS))
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(Label::new("Notifications"))
                    .child(Icon::new(IconName::Envelope)),
            )
            .map(|this| {
                if self.client.user_id().is_none() {
                    this.child(
                        v_flex()
                            .gap_2()
                            .p_4()
                            .child(
                                Button::new("sign_in_prompt_button", "Sign in")
                                    .icon_color(Color::Muted)
                                    .icon(IconName::Github)
                                    .icon_position(IconPosition::Start)
                                    .style(ButtonStyle::Filled)
                                    .full_width()
                                    .on_click({
                                        let client = self.client.clone();
                                        move |_, cx| {
                                            let client = client.clone();
                                            cx.spawn(move |cx| async move {
                                                client
                                                    .authenticate_and_connect(true, &cx)
                                                    .log_err()
                                                    .await;
                                            })
                                            .detach()
                                        }
                                    }),
                            )
                            .child(
                                div().flex().w_full().items_center().child(
                                    Label::new("Sign in to view notifications.")
                                        .color(Color::Muted)
                                        .size(LabelSize::Small),
                                ),
                            ),
                    )
                } else if self.notification_list.item_count() == 0 {
                    this.child(
                        v_flex().p_4().child(
                            div().flex().w_full().items_center().child(
                                Label::new("You have no notifications.")
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            ),
                        ),
                    )
                } else {
                    this.child(list(self.notification_list.clone()).size_full())
                }
            })
    }
}

impl FocusableView for NotificationPanel {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<Event> for NotificationPanel {}
impl EventEmitter<PanelEvent> for NotificationPanel {}

impl Panel for NotificationPanel {
    fn persistent_name() -> &'static str {
        "NotificationPanel"
    }

    fn position(&self, cx: &gpui::WindowContext) -> DockPosition {
        NotificationPanelSettings::get_global(cx).dock
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

    fn size(&self, cx: &gpui::WindowContext) -> Pixels {
        self.width
            .unwrap_or_else(|| NotificationPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        self.active = active;

        if self.active {
            self.unseen_notifications = Vec::new();
            cx.notify();
        }

        if self.notification_store.read(cx).notification_count() == 0 {
            cx.emit(Event::Dismissed);
        }
    }

    fn icon(&self, cx: &gpui::WindowContext) -> Option<IconName> {
        let show_button = NotificationPanelSettings::get_global(cx).button;
        if !show_button {
            return None;
        }

        if self.unseen_notifications.is_empty() {
            return Some(IconName::Bell);
        }

        Some(IconName::BellDot)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Notification Panel")
    }

    fn icon_label(&self, cx: &WindowContext) -> Option<String> {
        let count = self.notification_store.read(cx).unread_notification_count();
        if count == 0 {
            None
        } else {
            Some(count.to_string())
        }
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }
}

pub struct NotificationToast {
    notification_id: u64,
    actor: Option<Arc<User>>,
    text: String,
    workspace: WeakView<Workspace>,
}

impl NotificationToast {
    fn focus_notification_panel(&self, cx: &mut ViewContext<Self>) {
        let workspace = self.workspace.clone();
        let notification_id = self.notification_id;
        cx.window_context().defer(move |cx| {
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

impl Render for NotificationToast {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let user = self.actor.clone();

        h_flex()
            .id("notification_panel_toast")
            .elevation_3(cx)
            .p_2()
            .gap_2()
            .children(user.map(|user| Avatar::new(user.avatar_uri.clone())))
            .child(Label::new(self.text.clone()))
            .child(
                IconButton::new("close", IconName::Close)
                    .on_click(cx.listener(|_, _, cx| cx.emit(DismissEvent))),
            )
            .on_click(cx.listener(|this, _, cx| {
                this.focus_notification_panel(cx);
                cx.emit(DismissEvent);
            }))
    }
}

impl EventEmitter<DismissEvent> for NotificationToast {}

fn format_timestamp(
    mut timestamp: OffsetDateTime,
    mut now: OffsetDateTime,
    local_timezone: UtcOffset,
) -> String {
    timestamp = timestamp.to_offset(local_timezone);
    now = now.to_offset(local_timezone);

    let today = now.date();
    let date = timestamp.date();
    if date == today {
        let difference = now - timestamp;
        if difference >= Duration::from_secs(3600) {
            format!("{}h", difference.whole_seconds() / 3600)
        } else if difference >= Duration::from_secs(60) {
            format!("{}m", difference.whole_seconds() / 60)
        } else {
            "just now".to_string()
        }
    } else if date.next_day() == Some(today) {
        format!("yesterday")
    } else {
        format!("{:02}/{}/{}", date.month() as u32, date.day(), date.year())
    }
}
