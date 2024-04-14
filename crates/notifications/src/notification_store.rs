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
    let notification_store = cx.new_model(|cx| NotificationStore::new(client.clone(), user_store.clone(), cx));
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
                        this.handle_connect(&mut cx).await.log_err()?;
                    }
                    _ => this.handle_disconnect(&mut cx),
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
                client.add_message_handler(cx.weak_model(), Self::handle_update_notification),
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
        Some(cx.spawn(async move {
            let response = request.await?;
            self.loaded_all_notifications = response.done;
            self.add_notifications(response.notifications, AddNotificationsOptions {
                is_new: false,
                clear_old,
                includes_first: response.done,
            }, cx).await?;
            Ok(())
        }))
    }

    async fn handle_connect(&mut self, cx: &mut ModelContext<Self>) {
        self.notifications = Default::default();
        self.channel_messages = Default::default();
        cx.notify();
        if let Some(task) = self.load_more_notifications(true, cx) {
            task.await.log_err().ok();
        }
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
        this.add_notifications(
            envelope.payload.notification.into_iter().collect(),
            AddNotificationsOptions {
                is_new: true,
                clear_old: false,
                includes_first: false,
            },
            cx,
        ).await
    }

    async fn handle_delete_notification(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::DeleteNotification>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.splice_notifications([(envelope.payload.notification_id, None)], false, &mut cx);
        Ok(())
    }

    async fn handle_update_notification(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateNotification>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        if let Some(notification) = envelope.payload.notification {
            if let Some(rpc::Notification::ChannelMessageMention {
                message_id,
                sender_id: _,
                channel_id: _,
            }) = Notification::from_proto(&notification)
            {
                let fetch_message_task = this.channel_store.update(&mut cx, |this, cx| {
                    this.fetch_channel_messages(vec![message_id], cx)
                });

                if let Ok(messages) = fetch_message_task.await {
                    this.update_messages(messages);
                }
            }
        }
        Ok(())
    }

    async fn add_notifications(
        &self,
        notifications: Vec<proto::Notification>,
        options: AddNotificationsOptions,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        // Similar implementation as before...
    }

    fn update_messages(&mut self, messages: Vec<ChannelMessage>) {
        for message in messages {
            if let ChannelMessageId::Saved(id) = message.id {
                self.channel_messages.insert(id, message);
            }
        }
    }

    fn splice_notifications(
        &mut self,
        notifications: impl IntoIterator<Item = (u64, Option<NotificationEntry>)>,
        is_new: bool,
        cx: &mut ModelContext<Self>,
    ) {
        // Similar implementation as before...
    }

    pub fn respond_to_notification(
        &mut self,
        notification: Notification,
        response: bool,
        cx: &mut ModelContext<Self>,
    ) {
        // Similar implementation as before...
    }
}

impl EventEmitter<NotificationEvent> for NotificationStore {}

impl sum_tree::Item for NotificationEntry {
    // Similar implementation as before...
}

impl sum_tree::Summary for NotificationSummary {
    // Similar implementation as before...
}

impl<'a> sum_tree::Dimension<'a, NotificationSummary> for NotificationId {
    // Similar implementation as before...
}

impl<'a> sum_tree::Dimension<'a, NotificationSummary> for Count {
    // Similar implementation as before...
}

impl<'a> sum_tree::Dimension<'a, NotificationSummary> for UnreadCount {
    // Similar implementation as before...
}

struct AddNotificationsOptions {
    is_new: bool,
    clear_old: bool,
    includes_first: bool,
}
