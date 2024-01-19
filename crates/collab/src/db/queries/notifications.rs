use super::*;
use rpc::Notification;

impl Database {
    /// Initializes the different kinds of notifications by upserting records for them.
    pub async fn initialize_notification_kinds(&mut self) -> Result<()> {
        notification_kind::Entity::insert_many(Notification::all_variant_names().iter().map(
            |kind| notification_kind::ActiveModel {
                name: ActiveValue::Set(kind.to_string()),
                ..Default::default()
            },
        ))
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .exec_without_returning(&self.pool)
        .await?;

        let mut rows = notification_kind::Entity::find().stream(&self.pool).await?;
        while let Some(row) = rows.next().await {
            let row = row?;
            self.notification_kinds_by_name.insert(row.name, row.id);
        }

        for name in Notification::all_variant_names() {
            if let Some(id) = self.notification_kinds_by_name.get(*name).copied() {
                self.notification_kinds_by_id.insert(id, name);
            }
        }

        Ok(())
    }

    /// Returns the notifications for the given recipient.
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
                let kind = row.kind;
                if let Some(proto) = model_to_proto(self, row) {
                    result.push(proto);
                } else {
                    log::warn!("unknown notification kind {:?}", kind);
                }
            }
            result.reverse();
            Ok(result)
        })
        .await
    }

    /// Creates a notification. If `avoid_duplicates` is set to true, then avoid
    /// creating a new notification if the given recipient already has an
    /// unread notification with the given kind and entity id.
    pub async fn create_notification(
        &self,
        recipient_id: UserId,
        notification: Notification,
        avoid_duplicates: bool,
        tx: &DatabaseTransaction,
    ) -> Result<Option<(UserId, proto::Notification)>> {
        if avoid_duplicates {
            if self
                .find_notification(recipient_id, &notification, tx)
                .await?
                .is_some()
            {
                return Ok(None);
            }
        }

        let proto = notification.to_proto();
        let kind = notification_kind_from_proto(self, &proto)?;
        let model = notification::ActiveModel {
            recipient_id: ActiveValue::Set(recipient_id),
            kind: ActiveValue::Set(kind),
            entity_id: ActiveValue::Set(proto.entity_id.map(|id| id as i32)),
            content: ActiveValue::Set(proto.content.clone()),
            ..Default::default()
        }
        .save(&*tx)
        .await?;

        Ok(Some((
            recipient_id,
            proto::Notification {
                id: model.id.as_ref().to_proto(),
                kind: proto.kind,
                timestamp: model.created_at.as_ref().assume_utc().unix_timestamp() as u64,
                is_read: false,
                response: None,
                content: proto.content,
                entity_id: proto.entity_id,
            },
        )))
    }

    /// Remove an unread notification with the given recipient, kind and
    /// entity id.
    pub async fn remove_notification(
        &self,
        recipient_id: UserId,
        notification: Notification,
        tx: &DatabaseTransaction,
    ) -> Result<Option<NotificationId>> {
        let id = self
            .find_notification(recipient_id, &notification, tx)
            .await?;
        if let Some(id) = id {
            notification::Entity::delete_by_id(id).exec(tx).await?;
        }
        Ok(id)
    }

    /// Populate the response for the notification with the given kind and
    /// entity id.
    pub async fn mark_notification_as_read_with_response(
        &self,
        recipient_id: UserId,
        notification: &Notification,
        response: bool,
        tx: &DatabaseTransaction,
    ) -> Result<Option<(UserId, proto::Notification)>> {
        self.mark_notification_as_read_internal(recipient_id, notification, Some(response), tx)
            .await
    }

    /// Marks the given notification as read.
    pub async fn mark_notification_as_read(
        &self,
        recipient_id: UserId,
        notification: &Notification,
        tx: &DatabaseTransaction,
    ) -> Result<Option<(UserId, proto::Notification)>> {
        self.mark_notification_as_read_internal(recipient_id, notification, None, tx)
            .await
    }

    /// Marks the notification with the given ID as read.
    pub async fn mark_notification_as_read_by_id(
        &self,
        recipient_id: UserId,
        notification_id: NotificationId,
    ) -> Result<NotificationBatch> {
        self.transaction(|tx| async move {
            let row = notification::Entity::update(notification::ActiveModel {
                id: ActiveValue::Unchanged(notification_id),
                recipient_id: ActiveValue::Unchanged(recipient_id),
                is_read: ActiveValue::Set(true),
                ..Default::default()
            })
            .exec(&*tx)
            .await?;
            Ok(model_to_proto(self, row)
                .map(|notification| (recipient_id, notification))
                .into_iter()
                .collect())
        })
        .await
    }

    async fn mark_notification_as_read_internal(
        &self,
        recipient_id: UserId,
        notification: &Notification,
        response: Option<bool>,
        tx: &DatabaseTransaction,
    ) -> Result<Option<(UserId, proto::Notification)>> {
        if let Some(id) = self
            .find_notification(recipient_id, notification, &*tx)
            .await?
        {
            let row = notification::Entity::update(notification::ActiveModel {
                id: ActiveValue::Unchanged(id),
                recipient_id: ActiveValue::Unchanged(recipient_id),
                is_read: ActiveValue::Set(true),
                response: if let Some(response) = response {
                    ActiveValue::Set(Some(response))
                } else {
                    ActiveValue::NotSet
                },
                ..Default::default()
            })
            .exec(tx)
            .await?;
            Ok(model_to_proto(self, row).map(|notification| (recipient_id, notification)))
        } else {
            Ok(None)
        }
    }

    /// Find an unread notification by its recipient, kind and entity id.
    async fn find_notification(
        &self,
        recipient_id: UserId,
        notification: &Notification,
        tx: &DatabaseTransaction,
    ) -> Result<Option<NotificationId>> {
        let proto = notification.to_proto();
        let kind = notification_kind_from_proto(self, &proto)?;

        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryIds {
            Id,
        }

        Ok(notification::Entity::find()
            .select_only()
            .column(notification::Column::Id)
            .filter(
                Condition::all()
                    .add(notification::Column::RecipientId.eq(recipient_id))
                    .add(notification::Column::IsRead.eq(false))
                    .add(notification::Column::Kind.eq(kind))
                    .add(if proto.entity_id.is_some() {
                        notification::Column::EntityId.eq(proto.entity_id)
                    } else {
                        notification::Column::EntityId.is_null()
                    }),
            )
            .into_values::<_, QueryIds>()
            .one(&*tx)
            .await?)
    }
}

fn model_to_proto(this: &Database, row: notification::Model) -> Option<proto::Notification> {
    let kind = this.notification_kinds_by_id.get(&row.kind)?;
    Some(proto::Notification {
        id: row.id.to_proto(),
        kind: kind.to_string(),
        timestamp: row.created_at.assume_utc().unix_timestamp() as u64,
        is_read: row.is_read,
        response: row.response,
        content: row.content,
        entity_id: row.entity_id.map(|id| id as u64),
    })
}

fn notification_kind_from_proto(
    this: &Database,
    proto: &proto::Notification,
) -> Result<NotificationKindId> {
    Ok(this
        .notification_kinds_by_name
        .get(&proto.kind)
        .copied()
        .ok_or_else(|| anyhow!("invalid notification kind {:?}", proto.kind))?)
}
