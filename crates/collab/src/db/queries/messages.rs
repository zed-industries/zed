use super::*;
use futures::Stream;
use sea_orm::TryInsertResult;
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

            let rows = channel_message::Entity::find()
                .filter(condition)
                .order_by_desc(channel_message::Column::Id)
                .limit(count as u64)
                .stream(&*tx)
                .await?;

            self.load_channel_messages(rows, &*tx).await
        })
        .await
    }

    pub async fn get_channel_messages_by_id(
        &self,
        user_id: UserId,
        message_ids: &[MessageId],
    ) -> Result<Vec<proto::ChannelMessage>> {
        self.transaction(|tx| async move {
            let rows = channel_message::Entity::find()
                .filter(channel_message::Column::Id.is_in(message_ids.iter().copied()))
                .order_by_desc(channel_message::Column::Id)
                .stream(&*tx)
                .await?;

            let mut channel_ids = HashSet::<ChannelId>::default();
            let messages = self
                .load_channel_messages(
                    rows.map(|row| {
                        row.map(|row| {
                            channel_ids.insert(row.channel_id);
                            row
                        })
                    }),
                    &*tx,
                )
                .await?;

            for channel_id in channel_ids {
                self.check_user_is_channel_member(channel_id, user_id, &*tx)
                    .await?;
            }

            Ok(messages)
        })
        .await
    }

    async fn load_channel_messages(
        &self,
        mut rows: impl Send + Unpin + Stream<Item = Result<channel_message::Model, sea_orm::DbErr>>,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<proto::ChannelMessage>> {
        let mut messages = Vec::new();
        while let Some(row) = rows.next().await {
            let row = row?;
            let nonce = row.nonce.as_u64_pair();
            messages.push(proto::ChannelMessage {
                id: row.id.to_proto(),
                sender_id: row.sender_id.to_proto(),
                body: row.body,
                timestamp: row.sent_at.assume_utc().unix_timestamp() as u64,
                mentions: vec![],
                nonce: Some(proto::Nonce {
                    upper_half: nonce.0,
                    lower_half: nonce.1,
                }),
            });
        }
        drop(rows);
        messages.reverse();

        let mut mentions = channel_message_mention::Entity::find()
            .filter(channel_message_mention::Column::MessageId.is_in(messages.iter().map(|m| m.id)))
            .order_by_asc(channel_message_mention::Column::MessageId)
            .order_by_asc(channel_message_mention::Column::StartOffset)
            .stream(&*tx)
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

    pub async fn create_channel_message(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        body: &str,
        mentions: &[proto::ChatMention],
        timestamp: OffsetDateTime,
        nonce: u128,
    ) -> Result<CreatedChannelMessage> {
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

            let result = channel_message::Entity::insert(channel_message::ActiveModel {
                channel_id: ActiveValue::Set(channel_id),
                sender_id: ActiveValue::Set(user_id),
                body: ActiveValue::Set(body.to_string()),
                sent_at: ActiveValue::Set(timestamp),
                nonce: ActiveValue::Set(Uuid::from_u128(nonce)),
                id: ActiveValue::NotSet,
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
                    let mentions = mentions
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
                        .collect::<Vec<_>>();
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
                                &*tx,
                            )
                            .await?,
                        );
                    }

                    self.observe_channel_message_internal(channel_id, user_id, message_id, &*tx)
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

            let mut channel_members = self
                .get_channel_participants_internal(channel_id, &*tx)
                .await?;
            channel_members.retain(|member| !participant_user_ids.contains(member));

            Ok(CreatedChannelMessage {
                message_id,
                participant_connection_ids,
                channel_members,
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
    ) -> Result<()> {
        self.transaction(|tx| async move {
            self.observe_channel_message_internal(channel_id, user_id, message_id, &*tx)
                .await?;
            Ok(())
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
        .exec_without_returning(&*tx)
        .await?;
        Ok(())
    }

    pub async fn unseen_channel_messages(
        &self,
        user_id: UserId,
        channel_ids: &[ChannelId],
        tx: &DatabaseTransaction,
    ) -> Result<Vec<proto::UnseenChannelMessage>> {
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

        let mut changes = Vec::new();
        for last_message in last_messages {
            if let Some(observed_message) =
                observed_messages_by_channel_id.get(&last_message.channel_id)
            {
                if observed_message.channel_message_id == last_message.id {
                    continue;
                }
            }
            changes.push(proto::UnseenChannelMessage {
                channel_id: last_message.channel_id.to_proto(),
                message_id: last_message.id.to_proto(),
            });
        }

        Ok(changes)
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
                if self
                    .check_user_is_channel_admin(channel_id, user_id, &*tx)
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

            Ok(participant_connection_ids)
        })
        .await
    }
}
