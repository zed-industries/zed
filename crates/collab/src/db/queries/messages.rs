use super::*;
use rpc::Notification;
use sea_orm::{SelectColumns, TryInsertResult};
use time::OffsetDateTime;
use util::ResultExt;

impl Database {
    /// Inserts a record representing a user joining the chat for a given channel.
    pub async fn join_channel_chat(
        &self,
        channel_id: ChannelId,
        connection_id: ConnectionId,
        user_id: UserId,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            let channel = self.get_channel_internal(channel_id, &tx).await?;
            self.check_user_is_channel_participant(&channel, user_id, &tx)
                .await?;
            channel_chat_participant::ActiveModel {
                id: ActiveValue::NotSet,
                channel_id: ActiveValue::Set(channel_id),
                user_id: ActiveValue::Set(user_id),
                connection_id: ActiveValue::Set(connection_id.id as i32),
                connection_server_id: ActiveValue::Set(ServerId(connection_id.owner_id as i32)),
            }
            .insert(&*tx)
            .await?;
            Ok(())
        })
        .await
    }

    /// Removes `channel_chat_participant` records associated with the given connection ID.
    pub async fn channel_chat_connection_lost(
        &self,
        connection_id: ConnectionId,
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        channel_chat_participant::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(
                        channel_chat_participant::Column::ConnectionServerId
                            .eq(connection_id.owner_id),
                    )
                    .add(channel_chat_participant::Column::ConnectionId.eq(connection_id.id)),
            )
            .exec(tx)
            .await?;
        Ok(())
    }

    /// Removes `channel_chat_participant` records associated with the given user ID so they
    /// will no longer get chat notifications.
    pub async fn leave_channel_chat(
        &self,
        channel_id: ChannelId,
        connection_id: ConnectionId,
        _user_id: UserId,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            channel_chat_participant::Entity::delete_many()
                .filter(
                    Condition::all()
                        .add(
                            channel_chat_participant::Column::ConnectionServerId
                                .eq(connection_id.owner_id),
                        )
                        .add(channel_chat_participant::Column::ConnectionId.eq(connection_id.id))
                        .add(channel_chat_participant::Column::ChannelId.eq(channel_id)),
                )
                .exec(&*tx)
                .await?;

            Ok(())
        })
        .await
    }

    /// Retrieves the messages in the specified channel.
    ///
    /// Use `before_message_id` to paginate through the channel's messages.
    pub async fn get_channel_messages(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        count: usize,
        before_message_id: Option<MessageId>,
    ) -> Result<Vec<proto::ChannelMessage>> {
        self.transaction(|tx| async move {
            let channel = self.get_channel_internal(channel_id, &tx).await?;
            self.check_user_is_channel_participant(&channel, user_id, &tx)
                .await?;

            let mut condition =
                Condition::all().add(channel_message::Column::ChannelId.eq(channel_id));

            if let Some(before_message_id) = before_message_id {
                condition = condition.add(channel_message::Column::Id.lt(before_message_id));
            }

            let rows = channel_message::Entity::find()
                .filter(condition)
                .order_by_desc(channel_message::Column::Id)
                .limit(count as u64)
                .all(&*tx)
                .await?;

            self.load_channel_messages(rows, &tx).await
        })
        .await
    }

    /// Returns the channel messages with the given IDs.
    pub async fn get_channel_messages_by_id(
        &self,
        user_id: UserId,
        message_ids: &[MessageId],
    ) -> Result<Vec<proto::ChannelMessage>> {
        self.transaction(|tx| async move {
            let rows = channel_message::Entity::find()
                .filter(channel_message::Column::Id.is_in(message_ids.iter().copied()))
                .order_by_desc(channel_message::Column::Id)
                .all(&*tx)
                .await?;

            let mut channels = HashMap::<ChannelId, channel::Model>::default();
            for row in &rows {
                channels.insert(
                    row.channel_id,
                    self.get_channel_internal(row.channel_id, &tx).await?,
                );
            }

            for (_, channel) in channels {
                self.check_user_is_channel_participant(&channel, user_id, &tx)
                    .await?;
            }

            let messages = self.load_channel_messages(rows, &tx).await?;
            Ok(messages)
        })
        .await
    }

    async fn load_channel_messages(
        &self,
        rows: Vec<channel_message::Model>,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<proto::ChannelMessage>> {
        let mut messages = rows
            .into_iter()
            .map(|row| {
                let nonce = row.nonce.as_u64_pair();
                proto::ChannelMessage {
                    id: row.id.to_proto(),
                    sender_id: row.sender_id.to_proto(),
                    body: row.body,
                    timestamp: row.sent_at.assume_utc().unix_timestamp() as u64,
                    mentions: vec![],
                    nonce: Some(proto::Nonce {
                        upper_half: nonce.0,
                        lower_half: nonce.1,
                    }),
                    reply_to_message_id: row.reply_to_message_id.map(|id| id.to_proto()),
                    edited_at: row
                        .edited_at
                        .map(|t| t.assume_utc().unix_timestamp() as u64),
                }
            })
            .collect::<Vec<_>>();
        messages.reverse();

        let mut mentions = channel_message_mention::Entity::find()
            .filter(channel_message_mention::Column::MessageId.is_in(messages.iter().map(|m| m.id)))
            .order_by_asc(channel_message_mention::Column::MessageId)
            .order_by_asc(channel_message_mention::Column::StartOffset)
            .stream(tx)
            .await?;

        let mut message_ix = 0;
        while let Some(mention) = mentions.next().await {
            let mention = mention?;
            let message_id = mention.message_id.to_proto();
            while let Some(message) = messages.get_mut(message_ix) {
                if message.id < message_id {
                    message_ix += 1;
                } else {
                    if message.id == message_id {
                        message.mentions.push(proto::ChatMention {
                            range: Some(proto::Range {
                                start: mention.start_offset as u64,
                                end: mention.end_offset as u64,
                            }),
                            user_id: mention.user_id.to_proto(),
                        });
                    }
                    break;
                }
            }
        }

        Ok(messages)
    }

    fn format_mentions_to_entities(
        &self,
        message_id: MessageId,
        body: &str,
        mentions: &[proto::ChatMention],
    ) -> Result<Vec<tables::channel_message_mention::ActiveModel>> {
        Ok(mentions
            .iter()
            .filter_map(|mention| {
                let range = mention.range.as_ref()?;
                if !body.is_char_boundary(range.start as usize)
                    || !body.is_char_boundary(range.end as usize)
                {
                    return None;
                }
                Some(channel_message_mention::ActiveModel {
                    message_id: ActiveValue::Set(message_id),
                    start_offset: ActiveValue::Set(range.start as i32),
                    end_offset: ActiveValue::Set(range.end as i32),
                    user_id: ActiveValue::Set(UserId::from_proto(mention.user_id)),
                })
            })
            .collect::<Vec<_>>())
    }

    /// Creates a new channel message.
    pub async fn create_channel_message(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        body: &str,
        mentions: &[proto::ChatMention],
        timestamp: OffsetDateTime,
        nonce: u128,
        reply_to_message_id: Option<MessageId>,
    ) -> Result<CreatedChannelMessage> {
        self.transaction(|tx| async move {
            let channel = self.get_channel_internal(channel_id, &tx).await?;
            self.check_user_is_channel_participant(&channel, user_id, &tx)
                .await?;

            let mut rows = channel_chat_participant::Entity::find()
                .filter(channel_chat_participant::Column::ChannelId.eq(channel_id))
                .stream(&*tx)
                .await?;

            let mut is_participant = false;
            let mut participant_connection_ids = HashSet::default();
            let mut participant_user_ids = Vec::new();
            while let Some(row) = rows.next().await {
                let row = row?;
                if row.user_id == user_id {
                    is_participant = true;
                }
                participant_user_ids.push(row.user_id);
                participant_connection_ids.insert(row.connection());
            }
            drop(rows);

            if !is_participant {
                Err(anyhow!("not a chat participant"))?;
            }

            let timestamp = timestamp.to_offset(time::UtcOffset::UTC);
            let timestamp = time::PrimitiveDateTime::new(timestamp.date(), timestamp.time());

            let result = channel_message::Entity::insert(channel_message::ActiveModel {
                channel_id: ActiveValue::Set(channel_id),
                sender_id: ActiveValue::Set(user_id),
                body: ActiveValue::Set(body.to_string()),
                sent_at: ActiveValue::Set(timestamp),
                nonce: ActiveValue::Set(Uuid::from_u128(nonce)),
                id: ActiveValue::NotSet,
                reply_to_message_id: ActiveValue::Set(reply_to_message_id),
                edited_at: ActiveValue::NotSet,
            })
            .on_conflict(
                OnConflict::columns([
                    channel_message::Column::SenderId,
                    channel_message::Column::Nonce,
                ])
                .do_nothing()
                .to_owned(),
            )
            .do_nothing()
            .exec(&*tx)
            .await?;

            let message_id;
            let mut notifications = Vec::new();
            match result {
                TryInsertResult::Inserted(result) => {
                    message_id = result.last_insert_id;
                    let mentioned_user_ids =
                        mentions.iter().map(|m| m.user_id).collect::<HashSet<_>>();

                    let mentions = self.format_mentions_to_entities(message_id, body, mentions)?;
                    if !mentions.is_empty() {
                        channel_message_mention::Entity::insert_many(mentions)
                            .exec(&*tx)
                            .await?;
                    }

                    for mentioned_user in mentioned_user_ids {
                        notifications.extend(
                            self.create_notification(
                                UserId::from_proto(mentioned_user),
                                rpc::Notification::ChannelMessageMention {
                                    message_id: message_id.to_proto(),
                                    sender_id: user_id.to_proto(),
                                    channel_id: channel_id.to_proto(),
                                },
                                false,
                                &tx,
                            )
                            .await?,
                        );
                    }

                    self.observe_channel_message_internal(channel_id, user_id, message_id, &tx)
                        .await?;
                }
                _ => {
                    message_id = channel_message::Entity::find()
                        .filter(channel_message::Column::Nonce.eq(Uuid::from_u128(nonce)))
                        .one(&*tx)
                        .await?
                        .ok_or_else(|| anyhow!("failed to insert message"))?
                        .id;
                }
            }

            Ok(CreatedChannelMessage {
                message_id,
                participant_connection_ids,
                notifications,
            })
        })
        .await
    }

    pub async fn observe_channel_message(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        message_id: MessageId,
    ) -> Result<NotificationBatch> {
        self.transaction(|tx| async move {
            self.observe_channel_message_internal(channel_id, user_id, message_id, &tx)
                .await?;
            let mut batch = NotificationBatch::default();
            batch.extend(
                self.mark_notification_as_read(
                    user_id,
                    &Notification::ChannelMessageMention {
                        message_id: message_id.to_proto(),
                        sender_id: Default::default(),
                        channel_id: Default::default(),
                    },
                    &tx,
                )
                .await?,
            );
            Ok(batch)
        })
        .await
    }

    async fn observe_channel_message_internal(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        message_id: MessageId,
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        observed_channel_messages::Entity::insert(observed_channel_messages::ActiveModel {
            user_id: ActiveValue::Set(user_id),
            channel_id: ActiveValue::Set(channel_id),
            channel_message_id: ActiveValue::Set(message_id),
        })
        .on_conflict(
            OnConflict::columns([
                observed_channel_messages::Column::ChannelId,
                observed_channel_messages::Column::UserId,
            ])
            .update_column(observed_channel_messages::Column::ChannelMessageId)
            .action_cond_where(observed_channel_messages::Column::ChannelMessageId.lt(message_id))
            .to_owned(),
        )
        // TODO: Try to upgrade SeaORM so we don't have to do this hack around their bug
        .exec_without_returning(tx)
        .await?;
        Ok(())
    }

    pub async fn observed_channel_messages(
        &self,
        channel_ids: &[ChannelId],
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<proto::ChannelMessageId>> {
        let rows = observed_channel_messages::Entity::find()
            .filter(observed_channel_messages::Column::UserId.eq(user_id))
            .filter(
                observed_channel_messages::Column::ChannelId
                    .is_in(channel_ids.iter().map(|id| id.0)),
            )
            .all(tx)
            .await?;

        Ok(rows
            .into_iter()
            .map(|message| proto::ChannelMessageId {
                channel_id: message.channel_id.to_proto(),
                message_id: message.channel_message_id.to_proto(),
            })
            .collect())
    }

    pub async fn latest_channel_messages(
        &self,
        channel_ids: &[ChannelId],
        tx: &DatabaseTransaction,
    ) -> Result<Vec<proto::ChannelMessageId>> {
        let mut values = String::new();
        for id in channel_ids {
            if !values.is_empty() {
                values.push_str(", ");
            }
            write!(&mut values, "({})", id).unwrap();
        }

        if values.is_empty() {
            return Ok(Vec::default());
        }

        let sql = format!(
            r#"
            SELECT
                *
            FROM (
                SELECT
                    *,
                    row_number() OVER (
                        PARTITION BY channel_id
                        ORDER BY id DESC
                    ) as row_number
                FROM channel_messages
                WHERE
                    channel_id in ({values})
            ) AS messages
            WHERE
                row_number = 1
            "#,
        );

        let stmt = Statement::from_string(self.pool.get_database_backend(), sql);
        let mut last_messages = channel_message::Model::find_by_statement(stmt)
            .stream(tx)
            .await?;

        let mut results = Vec::new();
        while let Some(result) = last_messages.next().await {
            let message = result?;
            results.push(proto::ChannelMessageId {
                channel_id: message.channel_id.to_proto(),
                message_id: message.id.to_proto(),
            });
        }

        Ok(results)
    }

    fn get_notification_kind_id_by_name(&self, notification_kind: &str) -> Option<i32> {
        self.notification_kinds_by_id
            .iter()
            .find(|(_, kind)| **kind == notification_kind)
            .map(|kind| kind.0.0)
    }

    /// Removes the channel message with the given ID.
    pub async fn remove_channel_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        user_id: UserId,
    ) -> Result<(Vec<ConnectionId>, Vec<NotificationId>)> {
        self.transaction(|tx| async move {
            let mut rows = channel_chat_participant::Entity::find()
                .filter(channel_chat_participant::Column::ChannelId.eq(channel_id))
                .stream(&*tx)
                .await?;

            let mut is_participant = false;
            let mut participant_connection_ids = Vec::new();
            while let Some(row) = rows.next().await {
                let row = row?;
                if row.user_id == user_id {
                    is_participant = true;
                }
                participant_connection_ids.push(row.connection());
            }
            drop(rows);

            if !is_participant {
                Err(anyhow!("not a chat participant"))?;
            }

            let result = channel_message::Entity::delete_by_id(message_id)
                .filter(channel_message::Column::SenderId.eq(user_id))
                .exec(&*tx)
                .await?;

            if result.rows_affected == 0 {
                let channel = self.get_channel_internal(channel_id, &tx).await?;
                if self
                    .check_user_is_channel_admin(&channel, user_id, &tx)
                    .await
                    .is_ok()
                {
                    let result = channel_message::Entity::delete_by_id(message_id)
                        .exec(&*tx)
                        .await?;
                    if result.rows_affected == 0 {
                        Err(anyhow!("no such message"))?;
                    }
                } else {
                    Err(anyhow!("operation could not be completed"))?;
                }
            }

            let notification_kind_id =
                self.get_notification_kind_id_by_name("ChannelMessageMention");

            let existing_notifications = notification::Entity::find()
                .filter(notification::Column::EntityId.eq(message_id))
                .filter(notification::Column::Kind.eq(notification_kind_id))
                .select_column(notification::Column::Id)
                .all(&*tx)
                .await?;

            let existing_notification_ids = existing_notifications
                .into_iter()
                .map(|notification| notification.id)
                .collect();

            // remove all the mention notifications for this message
            notification::Entity::delete_many()
                .filter(notification::Column::EntityId.eq(message_id))
                .filter(notification::Column::Kind.eq(notification_kind_id))
                .exec(&*tx)
                .await?;

            Ok((participant_connection_ids, existing_notification_ids))
        })
        .await
    }

    /// Updates the channel message with the given ID, body and timestamp(edited_at).
    pub async fn update_channel_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        user_id: UserId,
        body: &str,
        mentions: &[proto::ChatMention],
        edited_at: OffsetDateTime,
    ) -> Result<UpdatedChannelMessage> {
        self.transaction(|tx| async move {
            let channel = self.get_channel_internal(channel_id, &tx).await?;
            self.check_user_is_channel_participant(&channel, user_id, &tx)
                .await?;

            let mut rows = channel_chat_participant::Entity::find()
                .filter(channel_chat_participant::Column::ChannelId.eq(channel_id))
                .stream(&*tx)
                .await?;

            let mut is_participant = false;
            let mut participant_connection_ids = Vec::new();
            let mut participant_user_ids = Vec::new();
            while let Some(row) = rows.next().await {
                let row = row?;
                if row.user_id == user_id {
                    is_participant = true;
                }
                participant_user_ids.push(row.user_id);
                participant_connection_ids.push(row.connection());
            }
            drop(rows);

            if !is_participant {
                Err(anyhow!("not a chat participant"))?;
            }

            let channel_message = channel_message::Entity::find_by_id(message_id)
                .filter(channel_message::Column::SenderId.eq(user_id))
                .one(&*tx)
                .await?;

            let Some(channel_message) = channel_message else {
                Err(anyhow!("Channel message not found"))?
            };

            let edited_at = edited_at.to_offset(time::UtcOffset::UTC);
            let edited_at = time::PrimitiveDateTime::new(edited_at.date(), edited_at.time());

            let updated_message = channel_message::ActiveModel {
                body: ActiveValue::Set(body.to_string()),
                edited_at: ActiveValue::Set(Some(edited_at)),
                reply_to_message_id: ActiveValue::Unchanged(channel_message.reply_to_message_id),
                id: ActiveValue::Unchanged(message_id),
                channel_id: ActiveValue::Unchanged(channel_id),
                sender_id: ActiveValue::Unchanged(user_id),
                sent_at: ActiveValue::Unchanged(channel_message.sent_at),
                nonce: ActiveValue::Unchanged(channel_message.nonce),
            };

            let result = channel_message::Entity::update_many()
                .set(updated_message)
                .filter(channel_message::Column::Id.eq(message_id))
                .filter(channel_message::Column::SenderId.eq(user_id))
                .exec(&*tx)
                .await?;
            if result.rows_affected == 0 {
                return Err(anyhow!(
                    "Attempted to edit a message (id: {message_id}) which does not exist anymore."
                ))?;
            }

            // we have to fetch the old mentions,
            // so we don't send a notification when the message has been edited that you are mentioned in
            let old_mentions = channel_message_mention::Entity::find()
                .filter(channel_message_mention::Column::MessageId.eq(message_id))
                .all(&*tx)
                .await?;

            // remove all existing mentions
            channel_message_mention::Entity::delete_many()
                .filter(channel_message_mention::Column::MessageId.eq(message_id))
                .exec(&*tx)
                .await?;

            let new_mentions = self.format_mentions_to_entities(message_id, body, mentions)?;
            if !new_mentions.is_empty() {
                // insert new mentions
                channel_message_mention::Entity::insert_many(new_mentions)
                    .exec(&*tx)
                    .await?;
            }

            let mut update_mention_user_ids = HashSet::default();
            let mut new_mention_user_ids =
                mentions.iter().map(|m| m.user_id).collect::<HashSet<_>>();
            // Filter out users that were mentioned before
            for mention in &old_mentions {
                if new_mention_user_ids.contains(&mention.user_id.to_proto()) {
                    update_mention_user_ids.insert(mention.user_id.to_proto());
                }

                new_mention_user_ids.remove(&mention.user_id.to_proto());
            }

            let notification_kind_id =
                self.get_notification_kind_id_by_name("ChannelMessageMention");

            let existing_notifications = notification::Entity::find()
                .filter(notification::Column::EntityId.eq(message_id))
                .filter(notification::Column::Kind.eq(notification_kind_id))
                .all(&*tx)
                .await?;

            // determine which notifications should be updated or deleted
            let mut deleted_notification_ids = HashSet::default();
            let mut updated_mention_notifications = Vec::new();
            for notification in existing_notifications {
                if update_mention_user_ids.contains(&notification.recipient_id.to_proto()) {
                    if let Some(notification) =
                        self::notifications::model_to_proto(self, notification).log_err()
                    {
                        updated_mention_notifications.push(notification);
                    }
                } else {
                    deleted_notification_ids.insert(notification.id);
                }
            }

            let mut notifications = Vec::new();
            for mentioned_user in new_mention_user_ids {
                notifications.extend(
                    self.create_notification(
                        UserId::from_proto(mentioned_user),
                        rpc::Notification::ChannelMessageMention {
                            message_id: message_id.to_proto(),
                            sender_id: user_id.to_proto(),
                            channel_id: channel_id.to_proto(),
                        },
                        false,
                        &tx,
                    )
                    .await?,
                );
            }

            Ok(UpdatedChannelMessage {
                message_id,
                participant_connection_ids,
                notifications,
                reply_to_message_id: channel_message.reply_to_message_id,
                timestamp: channel_message.sent_at,
                deleted_mention_notification_ids: deleted_notification_ids
                    .into_iter()
                    .collect::<Vec<_>>(),
                updated_mention_notifications,
            })
        })
        .await
    }
}
