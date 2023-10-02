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
                dbg!(&max_id);
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
            dbg!(&max_id);

            if let Some(max_id) = max_id {
                let has_older_message = dbg!(
                    observed_channel_messages::Entity::find()
                        .filter(
                            observed_channel_messages::Column::UserId
                                .eq(user_id)
                                .and(observed_channel_messages::Column::ChannelId.eq(channel_id))
                                .and(
                                    observed_channel_messages::Column::ChannelMessageId.lt(max_id)
                                ),
                        )
                        .one(&*tx)
                        .await
                )?
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
    ) -> Result<(MessageId, Vec<ConnectionId>)> {
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

            // Observe this message for all participants
            observed_channel_messages::Entity::insert_many(participant_user_ids.iter().map(
                |pariticpant_id| observed_channel_messages::ActiveModel {
                    user_id: ActiveValue::Set(*pariticpant_id),
                    channel_id: ActiveValue::Set(channel_id),
                    channel_message_id: ActiveValue::Set(message.last_insert_id),
                },
            ))
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

            Ok((message.last_insert_id, participant_connection_ids))
        })
        .await
    }

    #[cfg(test)]
    pub async fn has_new_message_tx(&self, channel_id: ChannelId, user_id: UserId) -> Result<bool> {
        self.transaction(|tx| async move { self.has_new_message(channel_id, user_id, &*tx).await })
            .await
    }

    #[cfg(test)]
    pub async fn dbg_print_messages(&self) -> Result<()> {
        self.transaction(|tx| async move {
            dbg!(observed_channel_messages::Entity::find()
                .all(&*tx)
                .await
                .unwrap());
            dbg!(channel_message::Entity::find().all(&*tx).await.unwrap());

            Ok(())
        })
        .await
    }

    pub async fn has_new_message(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<bool> {
        self.check_user_is_channel_member(channel_id, user_id, &*tx)
            .await?;

        let latest_message_id = channel_message::Entity::find()
            .filter(Condition::all().add(channel_message::Column::ChannelId.eq(channel_id)))
            .order_by(channel_message::Column::SentAt, sea_query::Order::Desc)
            .limit(1 as u64)
            .one(&*tx)
            .await?
            .map(|model| model.id);

        let last_message_read = observed_channel_messages::Entity::find()
            .filter(observed_channel_messages::Column::ChannelId.eq(channel_id))
            .filter(observed_channel_messages::Column::UserId.eq(user_id))
            .one(&*tx)
            .await?
            .map(|model| model.channel_message_id);

        Ok(dbg!(last_message_read) != dbg!(latest_message_id))
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
