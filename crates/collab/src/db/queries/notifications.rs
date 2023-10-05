use super::*;
use rpc::{Notification, NotificationEntityKind, NotificationKind};

impl Database {
    pub async fn ensure_notification_kinds(&self) -> Result<()> {
        self.transaction(|tx| async move {
            notification_kind::Entity::insert_many(NotificationKind::all().map(|kind| {
                notification_kind::ActiveModel {
                    id: ActiveValue::Set(kind as i32),
                    name: ActiveValue::Set(kind.to_string()),
                }
            }))
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec(&*tx)
            .await?;
            Ok(())
        })
        .await
    }

    pub async fn get_notifications(
        &self,
        recipient_id: UserId,
        limit: usize,
    ) -> Result<proto::AddNotifications> {
        self.transaction(|tx| async move {
            let mut result = proto::AddNotifications::default();

            let mut rows = notification::Entity::find()
                .filter(notification::Column::RecipientId.eq(recipient_id))
                .order_by_desc(notification::Column::Id)
                .limit(limit as u64)
                .stream(&*tx)
                .await?;

            let mut user_ids = Vec::new();
            let mut channel_ids = Vec::new();
            let mut message_ids = Vec::new();
            while let Some(row) = rows.next().await {
                let row = row?;

                let Some(kind) = NotificationKind::from_i32(row.kind) else {
                    continue;
                };
                let Some(notification) = Notification::from_parts(
                    kind,
                    [
                        row.entity_id_1.map(|id| id as u64),
                        row.entity_id_2.map(|id| id as u64),
                        row.entity_id_3.map(|id| id as u64),
                    ],
                ) else {
                    continue;
                };

                // Gather the ids of all associated entities.
                let (_, associated_entities) = notification.to_parts();
                for entity in associated_entities {
                    let Some((id, kind)) = entity else {
                        break;
                    };
                    match kind {
                        NotificationEntityKind::User => &mut user_ids,
                        NotificationEntityKind::Channel => &mut channel_ids,
                        NotificationEntityKind::ChannelMessage => &mut message_ids,
                    }
                    .push(id);
                }

                result.notifications.push(proto::Notification {
                    kind: row.kind as u32,
                    timestamp: row.created_at.assume_utc().unix_timestamp() as u64,
                    is_read: row.is_read,
                    entity_id_1: row.entity_id_1.map(|id| id as u64),
                    entity_id_2: row.entity_id_2.map(|id| id as u64),
                    entity_id_3: row.entity_id_3.map(|id| id as u64),
                });
            }

            let users = user::Entity::find()
                .filter(user::Column::Id.is_in(user_ids))
                .all(&*tx)
                .await?;
            let channels = channel::Entity::find()
                .filter(user::Column::Id.is_in(channel_ids))
                .all(&*tx)
                .await?;
            let messages = channel_message::Entity::find()
                .filter(user::Column::Id.is_in(message_ids))
                .all(&*tx)
                .await?;

            for user in users {
                result.users.push(proto::User {
                    id: user.id.to_proto(),
                    github_login: user.github_login,
                    avatar_url: String::new(),
                });
            }
            for channel in channels {
                result.channels.push(proto::Channel {
                    id: channel.id.to_proto(),
                    name: channel.name,
                });
            }
            for message in messages {
                result.messages.push(proto::ChannelMessage {
                    id: message.id.to_proto(),
                    body: message.body,
                    timestamp: message.sent_at.assume_utc().unix_timestamp() as u64,
                    sender_id: message.sender_id.to_proto(),
                    nonce: None,
                });
            }

            Ok(result)
        })
        .await
    }

    pub async fn create_notification(
        &self,
        recipient_id: UserId,
        notification: Notification,
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        let (kind, associated_entities) = notification.to_parts();
        notification::ActiveModel {
            recipient_id: ActiveValue::Set(recipient_id),
            kind: ActiveValue::Set(kind as i32),
            entity_id_1: ActiveValue::Set(associated_entities[0].map(|(id, _)| id as i32)),
            entity_id_2: ActiveValue::Set(associated_entities[1].map(|(id, _)| id as i32)),
            entity_id_3: ActiveValue::Set(associated_entities[2].map(|(id, _)| id as i32)),
            ..Default::default()
        }
        .save(&*tx)
        .await?;
        Ok(())
    }
}
