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

            let mut messages = Vec::new();
            while let Some(row) = rows.next().await {
                let row = row?;
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

            Ok((message.last_insert_id, participant_connection_ids))
        })
        .await
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
