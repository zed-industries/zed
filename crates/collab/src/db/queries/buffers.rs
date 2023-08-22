use super::*;
use prost::Message;

impl Database {
    pub async fn join_channel_buffer(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        connection: ConnectionId,
    ) -> Result<proto::JoinChannelBufferResponse> {
        self.transaction(|tx| async move {
            let tx = tx;

            self.check_user_is_channel_member(channel_id, user_id, &tx)
                .await?;

            let buffer = channel::Model {
                id: channel_id,
                ..Default::default()
            }
            .find_related(buffer::Entity)
            .one(&*tx)
            .await?;

            let buffer = if let Some(buffer) = buffer {
                buffer
            } else {
                let buffer = buffer::ActiveModel {
                    channel_id: ActiveValue::Set(channel_id),
                    ..Default::default()
                }
                .insert(&*tx)
                .await?;
                buffer
            };

            // Join the collaborators
            let mut collaborators = channel_buffer_collaborator::Entity::find()
                .filter(channel_buffer_collaborator::Column::ChannelId.eq(channel_id))
                .all(&*tx)
                .await?;
            let replica_ids = collaborators
                .iter()
                .map(|c| c.replica_id)
                .collect::<HashSet<_>>();
            let mut replica_id = ReplicaId(0);
            while replica_ids.contains(&replica_id) {
                replica_id.0 += 1;
            }
            let collaborator = channel_buffer_collaborator::ActiveModel {
                channel_id: ActiveValue::Set(channel_id),
                connection_id: ActiveValue::Set(connection.id as i32),
                connection_server_id: ActiveValue::Set(ServerId(connection.owner_id as i32)),
                user_id: ActiveValue::Set(user_id),
                replica_id: ActiveValue::Set(replica_id),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;
            collaborators.push(collaborator);

            // Assemble the buffer state
            let id = buffer.id;
            let base_text = if buffer.epoch > 0 {
                buffer_snapshot::Entity::find()
                    .filter(
                        buffer_snapshot::Column::BufferId
                            .eq(id)
                            .and(buffer_snapshot::Column::Epoch.eq(buffer.epoch)),
                    )
                    .one(&*tx)
                    .await?
                    .ok_or_else(|| anyhow!("no such snapshot"))?
                    .text
            } else {
                String::new()
            };

            let mut rows = buffer_operation::Entity::find()
                .filter(
                    buffer_operation::Column::BufferId
                        .eq(id)
                        .and(buffer_operation::Column::Epoch.eq(buffer.epoch)),
                )
                .stream(&*tx)
                .await?;
            let mut operations = Vec::new();
            while let Some(row) = rows.next().await {
                let row = row?;
                let version = deserialize_version(&row.version)?;
                let operation = if row.is_undo {
                    let counts = deserialize_undo_operation(&row.value)?;
                    proto::operation::Variant::Undo(proto::operation::Undo {
                        replica_id: row.replica_id as u32,
                        local_timestamp: row.local_timestamp as u32,
                        lamport_timestamp: row.lamport_timestamp as u32,
                        version,
                        counts,
                    })
                } else {
                    let (ranges, new_text) = deserialize_edit_operation(&row.value)?;
                    proto::operation::Variant::Edit(proto::operation::Edit {
                        replica_id: row.replica_id as u32,
                        local_timestamp: row.local_timestamp as u32,
                        lamport_timestamp: row.lamport_timestamp as u32,
                        version,
                        ranges,
                        new_text,
                    })
                };
                operations.push(proto::Operation {
                    variant: Some(operation),
                })
            }

            Ok(proto::JoinChannelBufferResponse {
                buffer_id: buffer.id.to_proto(),
                replica_id: replica_id.to_proto() as u32,
                base_text,
                operations,
                collaborators: collaborators
                    .into_iter()
                    .map(|collaborator| proto::Collaborator {
                        peer_id: Some(collaborator.connection().into()),
                        user_id: collaborator.user_id.to_proto(),
                        replica_id: collaborator.replica_id.0 as u32,
                    })
                    .collect(),
            })
        })
        .await
    }

    pub async fn leave_channel_buffer(
        &self,
        channel_id: ChannelId,
        connection: ConnectionId,
    ) -> Result<Vec<ConnectionId>> {
        self.transaction(|tx| async move {
            self.leave_channel_buffer_internal(channel_id, connection, &*tx)
                .await
        })
        .await
    }

    pub async fn leave_channel_buffer_internal(
        &self,
        channel_id: ChannelId,
        connection: ConnectionId,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<ConnectionId>> {
        let result = channel_buffer_collaborator::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(channel_buffer_collaborator::Column::ChannelId.eq(channel_id))
                    .add(channel_buffer_collaborator::Column::ConnectionId.eq(connection.id as i32))
                    .add(
                        channel_buffer_collaborator::Column::ConnectionServerId
                            .eq(connection.owner_id as i32),
                    ),
            )
            .exec(&*tx)
            .await?;
        if result.rows_affected == 0 {
            Err(anyhow!("not a collaborator on this project"))?;
        }

        let mut connections = Vec::new();
        let mut rows = channel_buffer_collaborator::Entity::find()
            .filter(
                Condition::all().add(channel_buffer_collaborator::Column::ChannelId.eq(channel_id)),
            )
            .stream(&*tx)
            .await?;
        while let Some(row) = rows.next().await {
            let row = row?;
            connections.push(ConnectionId {
                id: row.connection_id as u32,
                owner_id: row.connection_server_id.0 as u32,
            });
        }

        Ok(connections)
    }

    pub async fn leave_channel_buffers(
        &self,
        connection: ConnectionId,
    ) -> Result<Vec<(ChannelId, Vec<ConnectionId>)>> {
        self.transaction(|tx| async move {
            #[derive(Debug, Clone, Copy, EnumIter, DeriveColumn)]
            enum QueryChannelIds {
                ChannelId,
            }

            let channel_ids: Vec<ChannelId> = channel_buffer_collaborator::Entity::find()
                .select_only()
                .column(channel_buffer_collaborator::Column::ChannelId)
                .filter(Condition::all().add(
                    channel_buffer_collaborator::Column::ConnectionId.eq(connection.id as i32),
                ))
                .into_values::<_, QueryChannelIds>()
                .all(&*tx)
                .await?;

            let mut result = Vec::new();
            for channel_id in channel_ids {
                let collaborators = self
                    .leave_channel_buffer_internal(channel_id, connection, &*tx)
                    .await?;
                result.push((channel_id, collaborators));
            }

            Ok(result)
        })
        .await
    }

    #[cfg(debug_assertions)]
    pub async fn get_channel_buffer_collaborators(
        &self,
        channel_id: ChannelId,
    ) -> Result<Vec<UserId>> {
        self.transaction(|tx| async move {
            #[derive(Debug, Clone, Copy, EnumIter, DeriveColumn)]
            enum QueryUserIds {
                UserId,
            }

            let users: Vec<UserId> = channel_buffer_collaborator::Entity::find()
                .select_only()
                .column(channel_buffer_collaborator::Column::UserId)
                .filter(
                    Condition::all()
                        .add(channel_buffer_collaborator::Column::ChannelId.eq(channel_id)),
                )
                .into_values::<_, QueryUserIds>()
                .all(&*tx)
                .await?;

            Ok(users)
        })
        .await
    }

    pub async fn update_channel_buffer(
        &self,
        channel_id: ChannelId,
        user: UserId,
        operations: &[proto::Operation],
    ) -> Result<Vec<ConnectionId>> {
        self.transaction(|tx| async move {
            self.check_user_is_channel_member(channel_id, user, &*tx)
                .await?;

            let buffer = buffer::Entity::find()
                .filter(buffer::Column::ChannelId.eq(channel_id))
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such buffer"))?;
            let buffer_id = buffer.id;
            buffer_operation::Entity::insert_many(operations.iter().filter_map(|operation| {
                match operation.variant.as_ref()? {
                    proto::operation::Variant::Edit(operation) => {
                        let value =
                            serialize_edit_operation(&operation.ranges, &operation.new_text);
                        let version = serialize_version(&operation.version);
                        Some(buffer_operation::ActiveModel {
                            buffer_id: ActiveValue::Set(buffer_id),
                            epoch: ActiveValue::Set(buffer.epoch),
                            replica_id: ActiveValue::Set(operation.replica_id as i32),
                            lamport_timestamp: ActiveValue::Set(operation.lamport_timestamp as i32),
                            local_timestamp: ActiveValue::Set(operation.local_timestamp as i32),
                            is_undo: ActiveValue::Set(false),
                            version: ActiveValue::Set(version),
                            value: ActiveValue::Set(value),
                        })
                    }
                    proto::operation::Variant::Undo(operation) => {
                        let value = serialize_undo_operation(&operation.counts);
                        let version = serialize_version(&operation.version);
                        Some(buffer_operation::ActiveModel {
                            buffer_id: ActiveValue::Set(buffer_id),
                            epoch: ActiveValue::Set(buffer.epoch),
                            replica_id: ActiveValue::Set(operation.replica_id as i32),
                            lamport_timestamp: ActiveValue::Set(operation.lamport_timestamp as i32),
                            local_timestamp: ActiveValue::Set(operation.local_timestamp as i32),
                            is_undo: ActiveValue::Set(true),
                            version: ActiveValue::Set(version),
                            value: ActiveValue::Set(value),
                        })
                    }
                    proto::operation::Variant::UpdateSelections(_) => None,
                    proto::operation::Variant::UpdateDiagnostics(_) => None,
                    proto::operation::Variant::UpdateCompletionTriggers(_) => None,
                }
            }))
            .exec(&*tx)
            .await?;

            let mut connections = Vec::new();
            let mut rows = channel_buffer_collaborator::Entity::find()
                .filter(
                    Condition::all()
                        .add(channel_buffer_collaborator::Column::ChannelId.eq(channel_id)),
                )
                .stream(&*tx)
                .await?;
            while let Some(row) = rows.next().await {
                let row = row?;
                connections.push(ConnectionId {
                    id: row.connection_id as u32,
                    owner_id: row.connection_server_id.0 as u32,
                });
            }

            Ok(connections)
        })
        .await
    }
}

mod storage {
    #![allow(non_snake_case)]

    use prost::Message;

    pub const VERSION: usize = 1;

    #[derive(Message)]
    pub struct VectorClock {
        #[prost(message, repeated, tag = "1")]
        pub entries: Vec<VectorClockEntry>,
    }

    #[derive(Message)]
    pub struct VectorClockEntry {
        #[prost(uint32, tag = "1")]
        pub replica_id: u32,
        #[prost(uint32, tag = "2")]
        pub timestamp: u32,
    }

    #[derive(Message)]
    pub struct TextEdit {
        #[prost(message, repeated, tag = "1")]
        pub ranges: Vec<Range>,
        #[prost(string, repeated, tag = "2")]
        pub texts: Vec<String>,
    }

    #[derive(Message)]
    pub struct Range {
        #[prost(uint64, tag = "1")]
        pub start: u64,
        #[prost(uint64, tag = "2")]
        pub end: u64,
    }

    #[derive(Message)]
    pub struct Undo {
        #[prost(message, repeated, tag = "1")]
        pub entries: Vec<UndoCount>,
    }

    #[derive(Message)]
    pub struct UndoCount {
        #[prost(uint32, tag = "1")]
        pub replica_id: u32,
        #[prost(uint32, tag = "2")]
        pub local_timestamp: u32,
        #[prost(uint32, tag = "3")]
        pub count: u32,
    }
}

fn serialize_version(version: &Vec<proto::VectorClockEntry>) -> Vec<u8> {
    storage::VectorClock {
        entries: version
            .iter()
            .map(|entry| storage::VectorClockEntry {
                replica_id: entry.replica_id,
                timestamp: entry.timestamp,
            })
            .collect(),
    }
    .encode_to_vec()
}

fn deserialize_version(bytes: &[u8]) -> Result<Vec<proto::VectorClockEntry>> {
    let clock = storage::VectorClock::decode(bytes).map_err(|error| anyhow!("{}", error))?;
    Ok(clock
        .entries
        .into_iter()
        .map(|entry| proto::VectorClockEntry {
            replica_id: entry.replica_id,
            timestamp: entry.timestamp,
        })
        .collect())
}

fn serialize_edit_operation(ranges: &[proto::Range], texts: &[String]) -> Vec<u8> {
    storage::TextEdit {
        ranges: ranges
            .iter()
            .map(|range| storage::Range {
                start: range.start,
                end: range.end,
            })
            .collect(),
        texts: texts.to_vec(),
    }
    .encode_to_vec()
}

fn deserialize_edit_operation(bytes: &[u8]) -> Result<(Vec<proto::Range>, Vec<String>)> {
    let edit = storage::TextEdit::decode(bytes).map_err(|error| anyhow!("{}", error))?;
    let ranges = edit
        .ranges
        .into_iter()
        .map(|range| proto::Range {
            start: range.start,
            end: range.end,
        })
        .collect();
    Ok((ranges, edit.texts))
}

fn serialize_undo_operation(counts: &Vec<proto::UndoCount>) -> Vec<u8> {
    storage::Undo {
        entries: counts
            .iter()
            .map(|entry| storage::UndoCount {
                replica_id: entry.replica_id,
                local_timestamp: entry.local_timestamp,
                count: entry.count,
            })
            .collect(),
    }
    .encode_to_vec()
}

fn deserialize_undo_operation(bytes: &[u8]) -> Result<Vec<proto::UndoCount>> {
    let undo = storage::Undo::decode(bytes).map_err(|error| anyhow!("{}", error))?;
    Ok(undo
        .entries
        .iter()
        .map(|entry| proto::UndoCount {
            replica_id: entry.replica_id,
            local_timestamp: entry.local_timestamp,
            count: entry.count,
        })
        .collect())
}
