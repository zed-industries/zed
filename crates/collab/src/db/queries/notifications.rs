use super::*;
use rpc::{Notification, NotificationKind};

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
            while let Some(row) = rows.next().await {
                let row = row?;
                result.notifications.push(proto::Notification {
                    id: row.id.to_proto(),
                    kind: row.kind as u32,
                    timestamp: row.created_at.assume_utc().unix_timestamp() as u64,
                    is_read: row.is_read,
                    entity_id_1: row.entity_id_1.map(|id| id as u64),
                    entity_id_2: row.entity_id_2.map(|id| id as u64),
                    entity_id_3: row.entity_id_3.map(|id| id as u64),
                });
            }
            result.notifications.reverse();
            Ok(result)
        })
        .await
    }

    pub async fn create_notification(
        &self,
        recipient_id: UserId,
        notification: Notification,
        tx: &DatabaseTransaction,
    ) -> Result<proto::Notification> {
        let (kind, associated_entities) = notification.to_parts();
        let model = notification::ActiveModel {
            recipient_id: ActiveValue::Set(recipient_id),
            kind: ActiveValue::Set(kind as i32),
            entity_id_1: ActiveValue::Set(associated_entities[0].map(|id| id as i32)),
            entity_id_2: ActiveValue::Set(associated_entities[1].map(|id| id as i32)),
            entity_id_3: ActiveValue::Set(associated_entities[2].map(|id| id as i32)),
            ..Default::default()
        }
        .save(&*tx)
        .await?;

        Ok(proto::Notification {
            id: model.id.as_ref().to_proto(),
            kind: *model.kind.as_ref() as u32,
            timestamp: model.created_at.as_ref().assume_utc().unix_timestamp() as u64,
            is_read: false,
            entity_id_1: model.entity_id_1.as_ref().map(|id| id as u64),
            entity_id_2: model.entity_id_2.as_ref().map(|id| id as u64),
            entity_id_3: model.entity_id_3.as_ref().map(|id| id as u64),
        })
    }
}
