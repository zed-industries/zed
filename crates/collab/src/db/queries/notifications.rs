use super::*;
use rpc::Notification;

impl Database {
    pub async fn initialize_notification_enum(&mut self) -> Result<()> {
        notification_kind::Entity::insert_many(Notification::all_kinds().iter().map(|kind| {
            notification_kind::ActiveModel {
                name: ActiveValue::Set(kind.to_string()),
                ..Default::default()
            }
        }))
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .exec_without_returning(&self.pool)
        .await?;

        let mut rows = notification_kind::Entity::find().stream(&self.pool).await?;
        while let Some(row) = rows.next().await {
            let row = row?;
            self.notification_kinds_by_name.insert(row.name, row.id);
        }

        Ok(())
    }

    pub async fn get_notifications(
        &self,
        recipient_id: UserId,
        limit: usize,
        before_id: Option<NotificationId>,
    ) -> Result<Vec<proto::Notification>> {
        self.transaction(|tx| async move {
            let mut result = Vec::new();
            let mut condition =
                Condition::all().add(notification::Column::RecipientId.eq(recipient_id));

            if let Some(before_id) = before_id {
                condition = condition.add(notification::Column::Id.lt(before_id));
            }

            let mut rows = notification::Entity::find()
                .filter(condition)
                .order_by_desc(notification::Column::Id)
                .limit(limit as u64)
                .stream(&*tx)
                .await?;
            while let Some(row) = rows.next().await {
                let row = row?;
                let Some(kind) = self.notification_kinds_by_id.get(&row.kind) else {
                    continue;
                };
                result.push(proto::Notification {
                    id: row.id.to_proto(),
                    kind: kind.to_string(),
                    timestamp: row.created_at.assume_utc().unix_timestamp() as u64,
                    is_read: row.is_read,
                    content: row.content,
                    actor_id: row.actor_id.map(|id| id.to_proto()),
                });
            }
            result.reverse();
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
        let notification = notification.to_any();
        let kind = *self
            .notification_kinds_by_name
            .get(notification.kind.as_ref())
            .ok_or_else(|| anyhow!("invalid notification kind {:?}", notification.kind))?;

        let model = notification::ActiveModel {
            recipient_id: ActiveValue::Set(recipient_id),
            kind: ActiveValue::Set(kind),
            content: ActiveValue::Set(notification.content.clone()),
            actor_id: ActiveValue::Set(notification.actor_id.map(|id| UserId::from_proto(id))),
            is_read: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            id: ActiveValue::NotSet,
        }
        .save(&*tx)
        .await?;

        Ok(proto::Notification {
            id: model.id.as_ref().to_proto(),
            kind: notification.kind.to_string(),
            timestamp: model.created_at.as_ref().assume_utc().unix_timestamp() as u64,
            is_read: false,
            content: notification.content,
            actor_id: notification.actor_id,
        })
    }
}
