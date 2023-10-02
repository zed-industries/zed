use super::*;
use time::OffsetDateTime;

impl Database {
    pub async fn join_channel_chat(
        &self,
        channel_id: ChannelId,
        connection_id: ConnectionId,
        user_id: UserId,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            self.check_user_is_channel_member(channel_id, user_id, &*tx)
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

    pub async fn get_channel_messages(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        count: usize,
        before_message_id: Option<MessageId>,
    ) -> Result<Vec<proto::ChannelMessage>> {
        self.transaction(|tx| async move {
            self.check_user_is_channel_member(channel_id, user_id, &*tx)
                .await?;

            let mut condition =
                Condition::all().add(channel_message::Column::ChannelId.eq(channel_id));

            if let Some(before_message_id) = before_message_id {
                condition = condition.add(channel_message::Column::Id.lt(before_message_id));
            }

            let mut rows = channel_message::Entity::find()
                .filter(condition)
                .limit(count as u64)
                .stream(&*tx)
                .await?;

            let mut max_id = None;
            let mut messages = Vec::new();
            while let Some(row) = rows.next().await {
                let row = row?;

                max_assign(&mut max_id, row.id);

                let nonce = row.nonce.as_u64_pair();
                messages.push(proto::ChannelMessage {
                    id: row.id.to_proto(),
                    sender_id: row.sender_id.to_proto(),
                    body: row.body,
                    timestamp: row.sent_at.assume_utc().unix_timestamp() as u64,
                    nonce: Some(proto::Nonce {
                        upper_half: nonce.0,
                        lower_half: nonce.1,
                    }),
                });
            }
            drop(rows);

            if let Some(max_id) = max_id {
                let has_older_message = observed_channel_messages::Entity::find()
                    .filter(
                        observed_channel_messages::Column::UserId
                            .eq(user_id)
                            .and(observed_channel_messages::Column::ChannelId.eq(channel_id))
                            .and(observed_channel_messages::Column::ChannelMessageId.lt(max_id)),
                    )
                    .one(&*tx)
                    .await?
                    .is_some();

                if has_older_message {
                    observed_channel_messages::Entity::update(
                        observed_channel_messages::ActiveModel {
                            user_id: ActiveValue::Unchanged(user_id),
                            channel_id: ActiveValue::Unchanged(channel_id),
                            channel_message_id: ActiveValue::Set(max_id),
                        },
                    )
                    .exec(&*tx)
                    .await?;
                } else {
                    observed_channel_messages::Entity::insert(
                        observed_channel_messages::ActiveModel {
                            user_id: ActiveValue::Set(user_id),
                            channel_id: ActiveValue::Set(channel_id),
                            channel_message_id: ActiveValue::Set(max_id),
                        },
                    )
                    .on_conflict(
                        OnConflict::columns([
                            observed_channel_messages::Column::UserId,
                            observed_channel_messages::Column::ChannelId,
                        ])
                        .update_columns([observed_channel_messages::Column::ChannelMessageId])
                        .to_owned(),
                    )
                    .exec(&*tx)
                    .await?;
                }
            }

            Ok(messages)
        })
        .await
    }

    pub async fn create_channel_message(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        body: &str,
        timestamp: OffsetDateTime,
        nonce: u128,
    ) -> Result<(MessageId, Vec<ConnectionId>, Vec<UserId>)> {
        self.transaction(|tx| async move {
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

            let timestamp = timestamp.to_offset(time::UtcOffset::UTC);
            let timestamp = time::PrimitiveDateTime::new(timestamp.date(), timestamp.time());

            let message = channel_message::Entity::insert(channel_message::ActiveModel {
                channel_id: ActiveValue::Set(channel_id),
                sender_id: ActiveValue::Set(user_id),
                body: ActiveValue::Set(body.to_string()),
                sent_at: ActiveValue::Set(timestamp),
                nonce: ActiveValue::Set(Uuid::from_u128(nonce)),
                id: ActiveValue::NotSet,
            })
            .on_conflict(
                OnConflict::column(channel_message::Column::Nonce)
                    .update_column(channel_message::Column::Nonce)
                    .to_owned(),
            )
            .exec(&*tx)
            .await?;

            #[derive(Debug, Clone, Copy, EnumIter, DeriveColumn)]
            enum QueryConnectionId {
                ConnectionId,
            }

            // Observe this message for the sender
            observed_channel_messages::Entity::insert(observed_channel_messages::ActiveModel {
                user_id: ActiveValue::Set(user_id),
                channel_id: ActiveValue::Set(channel_id),
                channel_message_id: ActiveValue::Set(message.last_insert_id),
            })
            .on_conflict(
                OnConflict::columns([
                    observed_channel_messages::Column::ChannelId,
                    observed_channel_messages::Column::UserId,
                ])
                .update_column(observed_channel_messages::Column::ChannelMessageId)
                .to_owned(),
            )
            .exec(&*tx)
            .await?;

            let mut channel_members = self.get_channel_members_internal(channel_id, &*tx).await?;
            channel_members.retain(|member| !participant_user_ids.contains(member));

            Ok((
                message.last_insert_id,
                participant_connection_ids,
                channel_members,
            ))
        })
        .await
    }

    pub async fn channels_with_new_messages(
        &self,
        user_id: UserId,
        channel_ids: &[ChannelId],
        tx: &DatabaseTransaction,
    ) -> Result<HashSet<ChannelId>> {
        let mut observed_messages_by_channel_id = HashMap::default();
        let mut rows = observed_channel_messages::Entity::find()
            .filter(observed_channel_messages::Column::UserId.eq(user_id))
            .filter(observed_channel_messages::Column::ChannelId.is_in(channel_ids.iter().copied()))
            .stream(&*tx)
            .await?;

        while let Some(row) = rows.next().await {
            let row = row?;
            observed_messages_by_channel_id.insert(row.channel_id, row);
        }
        drop(rows);
        let mut values = String::new();
        for id in channel_ids {
            if !values.is_empty() {
                values.push_str(", ");
            }
            write!(&mut values, "({})", id).unwrap();
        }

        if values.is_empty() {
            return Ok(Default::default());
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
        let last_messages = channel_message::Model::find_by_statement(stmt)
            .all(&*tx)
            .await?;

        let mut channels_with_new_changes = HashSet::default();
        for last_message in last_messages {
            if let Some(observed_message) =
                observed_messages_by_channel_id.get(&last_message.channel_id)
            {
                if observed_message.channel_message_id == last_message.id {
                    continue;
                }
            }
            channels_with_new_changes.insert(last_message.channel_id);
        }

        Ok(channels_with_new_changes)
    }

    pub async fn remove_channel_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        user_id: UserId,
    ) -> Result<Vec<ConnectionId>> {
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
                Err(anyhow!("no such message"))?;
            }

            Ok(participant_connection_ids)
        })
        .await
    }
}
