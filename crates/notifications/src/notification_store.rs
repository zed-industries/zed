use anyhow::Result;
use channel::{ChannelMessage, ChannelMessageId, ChannelStore};
use client::{Client, UserStore};
use collections::HashMap;
use db::smol::stream::StreamExt;
use gpui::{AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, Task};
use rpc::{proto, Notification, TypedEnvelope};
use std::{ops::Range, sync::Arc};
use sum_tree::{Bias, SumTree};
use time::OffsetDateTime;
use util::ResultExt;

pub fn init(client: Arc<Client>, user_store: ModelHandle<UserStore>, cx: &mut AppContext) {
    let notification_store = cx.add_model(|cx| NotificationStore::new(client, user_store, cx));
    cx.set_global(notification_store);
}

pub struct NotificationStore {
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
    channel_messages: HashMap<u64, ChannelMessage>,
    channel_store: ModelHandle<ChannelStore>,
    notifications: SumTree<NotificationEntry>,
    _watch_connection_status: Task<Option<()>>,
    _subscriptions: Vec<client::Subscription>,
}

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
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NotificationEntry {
    pub id: u64,
    pub notification: Notification,
    pub timestamp: OffsetDateTime,
    pub is_read: bool,
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
    pub fn global(cx: &AppContext) -> ModelHandle<Self> {
        cx.global::<ModelHandle<Self>>().clone()
    }

    pub fn new(
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let mut connection_status = client.status();
        let watch_connection_status = cx.spawn_weak(|this, mut cx| async move {
            while let Some(status) = connection_status.next().await {
                let this = this.upgrade(&cx)?;
                match status {
                    client::Status::Connected { .. } => {
                        this.update(&mut cx, |this, cx| this.handle_connect(cx))
                            .await
                            .log_err()?;
                    }
                    _ => this.update(&mut cx, |this, cx| this.handle_disconnect(cx)),
                }
            }
            Some(())
        });

        Self {
            channel_store: ChannelStore::global(cx),
            notifications: Default::default(),
            channel_messages: Default::default(),
            _watch_connection_status: watch_connection_status,
            _subscriptions: vec![
                client.add_message_handler(cx.handle(), Self::handle_new_notification),
                client.add_message_handler(cx.handle(), Self::handle_delete_notification),
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

    pub fn notification_at(&self, ix: usize) -> Option<&NotificationEntry> {
        let mut cursor = self.notifications.cursor::<Count>();
        cursor.seek(&Count(ix), Bias::Right, &());
        cursor.item()
    }

    pub fn load_more_notifications(&self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let request = self
            .client
            .request(proto::GetNotifications { before_id: None });
        cx.spawn(|this, cx| async move {
            let response = request.await?;
            Self::add_notifications(this, false, response.notifications, cx).await?;
            Ok(())
        })
    }

    fn handle_connect(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        self.notifications = Default::default();
        self.channel_messages = Default::default();
        self.load_more_notifications(cx)
    }

    fn handle_disconnect(&mut self, cx: &mut ModelContext<Self>) {
        cx.notify()
    }

    async fn handle_new_notification(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::NewNotification>,
        _: Arc<Client>,
        cx: AsyncAppContext,
    ) -> Result<()> {
        Self::add_notifications(
            this,
            true,
            envelope.payload.notification.into_iter().collect(),
            cx,
        )
        .await
    }

    async fn handle_delete_notification(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::DeleteNotification>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.splice_notifications([(envelope.payload.notification_id, None)], false, cx);
            Ok(())
        })
    }

    async fn add_notifications(
        this: ModelHandle<Self>,
        is_new: bool,
        notifications: Vec<proto::Notification>,
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
                })
            })
            .collect::<Vec<_>>();
        if notifications.is_empty() {
            return Ok(());
        }

        for entry in &notifications {
            match entry.notification {
                Notification::ChannelInvitation {
                    actor_id: inviter_id,
                    ..
                } => {
                    user_ids.push(inviter_id);
                }
                Notification::ContactRequest {
                    actor_id: requester_id,
                } => {
                    user_ids.push(requester_id);
                }
                Notification::ContactRequestAccepted {
                    actor_id: contact_id,
                } => {
                    user_ids.push(contact_id);
                }
                Notification::ChannelMessageMention {
                    actor_id: sender_id,
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
        });

        user_store
            .update(&mut cx, |store, cx| store.get_users(user_ids, cx))
            .await?;
        let messages = channel_store
            .update(&mut cx, |store, cx| {
                store.fetch_channel_messages(message_ids, cx)
            })
            .await?;
        this.update(&mut cx, |this, cx| {
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
                is_new,
                cx,
            );
        });

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

            if let Some(existing_notification) = cursor.item() {
                if existing_notification.id == id {
                    if new_notification.is_none() {
                        cx.emit(NotificationEvent::NotificationRemoved {
                            entry: existing_notification.clone(),
                        });
                    }
                    cursor.next(&());
                }
            }

            if let Some(notification) = new_notification {
                if is_new {
                    cx.emit(NotificationEvent::NewNotification {
                        entry: notification.clone(),
                    });
                }

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
}

impl Entity for NotificationStore {
    type Event = NotificationEvent;
}

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
