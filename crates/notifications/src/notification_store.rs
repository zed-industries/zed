use anyhow::{Context, Result};
use channel::{ChannelMessage, ChannelMessageId, ChannelStore};
use client::{ChannelId, Client, UserStore};
use collections::HashMap;
use db::smol::stream::StreamExt;
use gpui::{
    AppContext, AsyncAppContext, Context as _, EventEmitter, Global, Model, ModelContext, Task,
};
use rpc::{proto, Notification, TypedEnvelope};
use std::{ops::Range, sync::Arc};
use sum_tree::{Bias, SumTree};
use time::OffsetDateTime;
use util::ResultExt;

pub fn init(client: Arc<Client>, user_store: Model<UserStore>, cx: &mut AppContext) {
    let notification_store = cx.new_model(|cx| NotificationStore::new(client, user_store, cx));
    cx.set_global(GlobalNotificationStore(notification_store));
}

struct GlobalNotificationStore(Model<NotificationStore>);

impl Global for GlobalNotificationStore {}

pub struct NotificationStore {
    client: Arc<Client>,
    user_store: Model<UserStore>,
    channel_messages: HashMap<u64, ChannelMessage>,
    channel_store: Model<ChannelStore>,
    notifications: SumTree<NotificationEntry>,
    loaded_all_notifications: bool,
    _watch_connection_status: Task<Option<()>>,
    _subscriptions: Vec<client::Subscription>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NotificationEvent {
    NotificationsUpdated {
        old_range: Range<usize>,
        new_count: usize,
    },
    NewNotification {
        entry: NotificationEntry,
    },
    NotificationRemoved {
        entry: NotificationEntry,
    },
    NotificationRead {
        entry: NotificationEntry,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NotificationEntry {
    pub id: u64,
    pub notification: Notification,
    pub timestamp: OffsetDateTime,
    pub is_read: bool,
    pub response: Option<bool>,
}

#[derive(Clone, Debug, Default)]
pub struct NotificationSummary {
    max_id: u64,
    count: usize,
    unread_count: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Count(usize);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct UnreadCount(usize);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct NotificationId(u64);

impl NotificationStore {
    pub fn global(cx: &AppContext) -> Model<Self> {
        cx.global::<GlobalNotificationStore>().0.clone()
    }

    pub fn new(
        client: Arc<Client>,
        user_store: Model<UserStore>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let mut connection_status = client.status();
        let watch_connection_status = cx.spawn(|this, mut cx| async move {
            while let Some(status) = connection_status.next().await {
                let this = this.upgrade()?;
                match status {
                    client::Status::Connected { .. } => {
                        if let Some(task) = this
                            .update(&mut cx, |this, cx| this.handle_connect(cx))
                            .log_err()?
                        {
                            task.await.log_err()?;
                        }
                    }
                    _ => this
                        .update(&mut cx, |this, cx| this.handle_disconnect(cx))
                        .log_err()?,
                }
            }
            Some(())
        });

        Self {
            channel_store: ChannelStore::global(cx),
            notifications: Default::default(),
            loaded_all_notifications: false,
            channel_messages: Default::default(),
            _watch_connection_status: watch_connection_status,
            _subscriptions: vec![
                client.add_message_handler(cx.weak_model(), Self::handle_new_notification),
                client.add_message_handler(cx.weak_model(), Self::handle_delete_notification),
            ],
            user_store,
            client,
        }
    }

    pub fn notification_count(&self) -> usize {
        self.notifications.summary().count
    }

    pub fn unread_notification_count(&self) -> usize {
        self.notifications.summary().unread_count
    }

    pub fn channel_message_for_id(&self, id: u64) -> Option<&ChannelMessage> {
        self.channel_messages.get(&id)
    }

    // Get the nth newest notification.
    pub fn notification_at(&self, ix: usize) -> Option<&NotificationEntry> {
        let count = self.notifications.summary().count;
        if ix >= count {
            return None;
        }
        let ix = count - 1 - ix;
        let mut cursor = self.notifications.cursor::<Count>();
        cursor.seek(&Count(ix), Bias::Right, &());
        cursor.item()
    }

    pub fn notification_for_id(&self, id: u64) -> Option<&NotificationEntry> {
        let mut cursor = self.notifications.cursor::<NotificationId>();
        cursor.seek(&NotificationId(id), Bias::Left, &());
        if let Some(item) = cursor.item() {
            if item.id == id {
                return Some(item);
            }
        }
        None
    }

    pub fn load_more_notifications(
        &self,
        clear_old: bool,
        cx: &mut ModelContext<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.loaded_all_notifications && !clear_old {
            return None;
        }

        let before_id = if clear_old {
            None
        } else {
            self.notifications.first().map(|entry| entry.id)
        };
        let request = self.client.request(proto::GetNotifications { before_id });
        Some(cx.spawn(|this, mut cx| async move {
            let this = this
                .upgrade()
                .context("Notification store was dropped while loading notifications")?;

            let response = request.await?;
            this.update(&mut cx, |this, _| {
                this.loaded_all_notifications = response.done
            })?;
            Self::add_notifications(
                this,
                response.notifications,
                AddNotificationsOptions {
                    is_new: false,
                    clear_old,
                    includes_first: response.done,
                },
                cx,
            )
            .await?;
            Ok(())
        }))
    }

    fn handle_connect(&mut self, cx: &mut ModelContext<Self>) -> Option<Task<Result<()>>> {
        self.notifications = Default::default();
        self.channel_messages = Default::default();
        cx.notify();
        self.load_more_notifications(true, cx)
    }

    fn handle_disconnect(&mut self, cx: &mut ModelContext<Self>) {
        cx.notify()
    }

    async fn handle_new_notification(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::AddNotification>,
        _: Arc<Client>,
        cx: AsyncAppContext,
    ) -> Result<()> {
        Self::add_notifications(
            this,
            envelope.payload.notification.into_iter().collect(),
            AddNotificationsOptions {
                is_new: true,
                clear_old: false,
                includes_first: false,
            },
            cx,
        )
        .await
    }

    async fn handle_delete_notification(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::DeleteNotification>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.splice_notifications([(envelope.payload.notification_id, None)], false, cx);
            Ok(())
        })?
    }

    async fn add_notifications(
        this: Model<Self>,
        notifications: Vec<proto::Notification>,
        options: AddNotificationsOptions,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let mut user_ids = Vec::new();
        let mut message_ids = Vec::new();

        let notifications = notifications
            .into_iter()
            .filter_map(|message| {
                Some(NotificationEntry {
                    id: message.id,
                    is_read: message.is_read,
                    timestamp: OffsetDateTime::from_unix_timestamp(message.timestamp as i64)
                        .ok()?,
                    notification: Notification::from_proto(&message)?,
                    response: message.response,
                })
            })
            .collect::<Vec<_>>();
        if notifications.is_empty() {
            return Ok(());
        }

        for entry in &notifications {
            match entry.notification {
                Notification::ChannelInvitation { inviter_id, .. } => {
                    user_ids.push(inviter_id);
                }
                Notification::ContactRequest {
                    sender_id: requester_id,
                } => {
                    user_ids.push(requester_id);
                }
                Notification::ContactRequestAccepted {
                    responder_id: contact_id,
                } => {
                    user_ids.push(contact_id);
                }
                Notification::ChannelMessageMention {
                    sender_id,
                    message_id,
                    ..
                } => {
                    user_ids.push(sender_id);
                    message_ids.push(message_id);
                }
            }
        }

        let (user_store, channel_store) = this.read_with(&cx, |this, _| {
            (this.user_store.clone(), this.channel_store.clone())
        })?;

        user_store
            .update(&mut cx, |store, cx| store.get_users(user_ids, cx))?
            .await?;
        let messages = channel_store
            .update(&mut cx, |store, cx| {
                store.fetch_channel_messages(message_ids, cx)
            })?
            .await?;
        this.update(&mut cx, |this, cx| {
            if options.clear_old {
                cx.emit(NotificationEvent::NotificationsUpdated {
                    old_range: 0..this.notifications.summary().count,
                    new_count: 0,
                });
                this.notifications = SumTree::default();
                this.channel_messages.clear();
                this.loaded_all_notifications = false;
            }

            if options.includes_first {
                this.loaded_all_notifications = true;
            }

            this.channel_messages
                .extend(messages.into_iter().filter_map(|message| {
                    if let ChannelMessageId::Saved(id) = message.id {
                        Some((id, message))
                    } else {
                        None
                    }
                }));

            this.splice_notifications(
                notifications
                    .into_iter()
                    .map(|notification| (notification.id, Some(notification))),
                options.is_new,
                cx,
            );
        })
        .log_err();

        Ok(())
    }

    fn splice_notifications(
        &mut self,
        notifications: impl IntoIterator<Item = (u64, Option<NotificationEntry>)>,
        is_new: bool,
        cx: &mut ModelContext<'_, NotificationStore>,
    ) {
        let mut cursor = self.notifications.cursor::<(NotificationId, Count)>();
        let mut new_notifications = SumTree::new();
        let mut old_range = 0..0;

        for (i, (id, new_notification)) in notifications.into_iter().enumerate() {
            new_notifications.append(cursor.slice(&NotificationId(id), Bias::Left, &()), &());

            if i == 0 {
                old_range.start = cursor.start().1 .0;
            }

            let old_notification = cursor.item();
            if let Some(old_notification) = old_notification {
                if old_notification.id == id {
                    cursor.next(&());

                    if let Some(new_notification) = &new_notification {
                        if new_notification.is_read {
                            cx.emit(NotificationEvent::NotificationRead {
                                entry: new_notification.clone(),
                            });
                        }
                    } else {
                        cx.emit(NotificationEvent::NotificationRemoved {
                            entry: old_notification.clone(),
                        });
                    }
                }
            } else if let Some(new_notification) = &new_notification {
                if is_new {
                    cx.emit(NotificationEvent::NewNotification {
                        entry: new_notification.clone(),
                    });
                }
            }

            if let Some(notification) = new_notification {
                new_notifications.push(notification, &());
            }
        }

        old_range.end = cursor.start().1 .0;
        let new_count = new_notifications.summary().count - old_range.start;
        new_notifications.append(cursor.suffix(&()), &());
        drop(cursor);

        self.notifications = new_notifications;
        cx.emit(NotificationEvent::NotificationsUpdated {
            old_range,
            new_count,
        });
    }

    pub fn respond_to_notification(
        &mut self,
        notification: Notification,
        response: bool,
        cx: &mut ModelContext<Self>,
    ) {
        match notification {
            Notification::ContactRequest { sender_id } => {
                self.user_store
                    .update(cx, |store, cx| {
                        store.respond_to_contact_request(sender_id, response, cx)
                    })
                    .detach();
            }
            Notification::ChannelInvitation { channel_id, .. } => {
                self.channel_store
                    .update(cx, |store, cx| {
                        store.respond_to_channel_invite(ChannelId(channel_id), response, cx)
                    })
                    .detach();
            }
            _ => {}
        }
    }
}

impl EventEmitter<NotificationEvent> for NotificationStore {}

impl sum_tree::Item for NotificationEntry {
    type Summary = NotificationSummary;

    fn summary(&self) -> Self::Summary {
        NotificationSummary {
            max_id: self.id,
            count: 1,
            unread_count: if self.is_read { 0 } else { 1 },
        }
    }
}

impl sum_tree::Summary for NotificationSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        self.max_id = self.max_id.max(summary.max_id);
        self.count += summary.count;
        self.unread_count += summary.unread_count;
    }
}

impl<'a> sum_tree::Dimension<'a, NotificationSummary> for NotificationId {
    fn add_summary(&mut self, summary: &NotificationSummary, _: &()) {
        debug_assert!(summary.max_id > self.0);
        self.0 = summary.max_id;
    }
}

impl<'a> sum_tree::Dimension<'a, NotificationSummary> for Count {
    fn add_summary(&mut self, summary: &NotificationSummary, _: &()) {
        self.0 += summary.count;
    }
}

impl<'a> sum_tree::Dimension<'a, NotificationSummary> for UnreadCount {
    fn add_summary(&mut self, summary: &NotificationSummary, _: &()) {
        self.0 += summary.unread_count;
    }
}

struct AddNotificationsOptions {
    is_new: bool,
    clear_old: bool,
    includes_first: bool,
}
