use super::*;
use prost::Message;
use text::{EditOperation, InsertionTimestamp, UndoOperation};

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
                buffer_snapshot::ActiveModel {
                    buffer_id: ActiveValue::Set(buffer.id),
                    epoch: ActiveValue::Set(0),
                    text: ActiveValue::Set(String::new()),
                    operation_serialization_version: ActiveValue::Set(
                        storage::SERIALIZATION_VERSION,
                    ),
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
            let (base_text, operations) = self.get_buffer_state(&buffer, &tx).await?;

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

        drop(rows);

        if connections.is_empty() {
            self.snapshot_buffer(channel_id, &tx).await?;
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
        self.transaction(move |tx| async move {
            self.check_user_is_channel_member(channel_id, user, &*tx)
                .await?;

            let buffer = buffer::Entity::find()
                .filter(buffer::Column::ChannelId.eq(channel_id))
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such buffer"))?;

            #[derive(Debug, Clone, Copy, EnumIter, DeriveColumn)]
            enum QueryVersion {
                OperationSerializationVersion,
            }

            let serialization_version: i32 = buffer
                .find_related(buffer_snapshot::Entity)
                .select_only()
                .column(buffer_snapshot::Column::OperationSerializationVersion)
                .filter(buffer_snapshot::Column::Epoch.eq(buffer.epoch))
                .into_values::<_, QueryVersion>()
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("missing buffer snapshot"))?;

            let operations = operations
                .iter()
                .filter_map(|op| operation_to_storage(op, &buffer, serialization_version))
                .collect::<Vec<_>>();
            if !operations.is_empty() {
                buffer_operation::Entity::insert_many(operations)
                    .exec(&*tx)
                    .await?;
            }

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

    async fn get_buffer_state(
        &self,
        buffer: &buffer::Model,
        tx: &DatabaseTransaction,
    ) -> Result<(String, Vec<proto::Operation>)> {
        let id = buffer.id;
        let (base_text, version) = if buffer.epoch > 0 {
            let snapshot = buffer_snapshot::Entity::find()
                .filter(
                    buffer_snapshot::Column::BufferId
                        .eq(id)
                        .and(buffer_snapshot::Column::Epoch.eq(buffer.epoch)),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such snapshot"))?;

            let version = snapshot.operation_serialization_version;
            (snapshot.text, version)
        } else {
            (String::new(), storage::SERIALIZATION_VERSION)
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

            let operation = operation_from_storage(row, version)?;
            operations.push(proto::Operation {
                variant: Some(operation),
            })
        }

        Ok((base_text, operations))
    }

    async fn snapshot_buffer(&self, channel_id: ChannelId, tx: &DatabaseTransaction) -> Result<()> {
        let buffer = channel::Model {
            id: channel_id,
            ..Default::default()
        }
        .find_related(buffer::Entity)
        .one(&*tx)
        .await?
        .ok_or_else(|| anyhow!("no such buffer"))?;

        let (base_text, operations) = self.get_buffer_state(&buffer, tx).await?;
        if operations.is_empty() {
            return Ok(());
        }

        let mut text_buffer = text::Buffer::new(0, 0, base_text);
        text_buffer
            .apply_ops(operations.into_iter().filter_map(operation_from_wire))
            .unwrap();

        let base_text = text_buffer.text();
        let epoch = buffer.epoch + 1;

        buffer_snapshot::Model {
            buffer_id: buffer.id,
            epoch,
            text: base_text,
            operation_serialization_version: storage::SERIALIZATION_VERSION,
        }
        .into_active_model()
        .insert(tx)
        .await?;

        buffer::ActiveModel {
            id: ActiveValue::Unchanged(buffer.id),
            epoch: ActiveValue::Set(epoch),
            ..Default::default()
        }
        .save(tx)
        .await?;

        Ok(())
    }
}

fn operation_to_storage(
    operation: &proto::Operation,
    buffer: &buffer::Model,
    _format: i32,
) -> Option<buffer_operation::ActiveModel> {
    let (replica_id, lamport_timestamp, value) = match operation.variant.as_ref()? {
        proto::operation::Variant::Edit(operation) => (
            operation.replica_id,
            operation.lamport_timestamp,
            storage::Operation {
                local_timestamp: operation.local_timestamp,
                version: version_to_storage(&operation.version),
                is_undo: false,
                edit_ranges: operation
                    .ranges
                    .iter()
                    .map(|range| storage::Range {
                        start: range.start,
                        end: range.end,
                    })
                    .collect(),
                edit_texts: operation.new_text.clone(),
                undo_counts: Vec::new(),
            },
        ),
        proto::operation::Variant::Undo(operation) => (
            operation.replica_id,
            operation.lamport_timestamp,
            storage::Operation {
                local_timestamp: operation.local_timestamp,
                version: version_to_storage(&operation.version),
                is_undo: true,
                edit_ranges: Vec::new(),
                edit_texts: Vec::new(),
                undo_counts: operation
                    .counts
                    .iter()
                    .map(|entry| storage::UndoCount {
                        replica_id: entry.replica_id,
                        local_timestamp: entry.local_timestamp,
                        count: entry.count,
                    })
                    .collect(),
            },
        ),
        _ => None?,
    };

    Some(buffer_operation::ActiveModel {
        buffer_id: ActiveValue::Set(buffer.id),
        epoch: ActiveValue::Set(buffer.epoch),
        replica_id: ActiveValue::Set(replica_id as i32),
        lamport_timestamp: ActiveValue::Set(lamport_timestamp as i32),
        value: ActiveValue::Set(value.encode_to_vec()),
    })
}

fn operation_from_storage(
    row: buffer_operation::Model,
    _format_version: i32,
) -> Result<proto::operation::Variant, Error> {
    let operation =
        storage::Operation::decode(row.value.as_slice()).map_err(|error| anyhow!("{}", error))?;
    let version = version_from_storage(&operation.version);
    Ok(if operation.is_undo {
        proto::operation::Variant::Undo(proto::operation::Undo {
            replica_id: row.replica_id as u32,
            local_timestamp: operation.local_timestamp as u32,
            lamport_timestamp: row.lamport_timestamp as u32,
            version,
            counts: operation
                .undo_counts
                .iter()
                .map(|entry| proto::UndoCount {
                    replica_id: entry.replica_id,
                    local_timestamp: entry.local_timestamp,
                    count: entry.count,
                })
                .collect(),
        })
    } else {
        proto::operation::Variant::Edit(proto::operation::Edit {
            replica_id: row.replica_id as u32,
            local_timestamp: operation.local_timestamp as u32,
            lamport_timestamp: row.lamport_timestamp as u32,
            version,
            ranges: operation
                .edit_ranges
                .into_iter()
                .map(|range| proto::Range {
                    start: range.start,
                    end: range.end,
                })
                .collect(),
            new_text: operation.edit_texts,
        })
    })
}

fn version_to_storage(version: &Vec<proto::VectorClockEntry>) -> Vec<storage::VectorClockEntry> {
    version
        .iter()
        .map(|entry| storage::VectorClockEntry {
            replica_id: entry.replica_id,
            timestamp: entry.timestamp,
        })
        .collect()
}

fn version_from_storage(version: &Vec<storage::VectorClockEntry>) -> Vec<proto::VectorClockEntry> {
    version
        .iter()
        .map(|entry| proto::VectorClockEntry {
            replica_id: entry.replica_id,
            timestamp: entry.timestamp,
        })
        .collect()
}

// This is currently a manual copy of the deserialization code in the client's langauge crate
pub fn operation_from_wire(operation: proto::Operation) -> Option<text::Operation> {
    match operation.variant? {
        proto::operation::Variant::Edit(edit) => Some(text::Operation::Edit(EditOperation {
            timestamp: InsertionTimestamp {
                replica_id: edit.replica_id as text::ReplicaId,
                local: edit.local_timestamp,
                lamport: edit.lamport_timestamp,
            },
            version: version_from_wire(&edit.version),
            ranges: edit
                .ranges
                .into_iter()
                .map(|range| {
                    text::FullOffset(range.start as usize)..text::FullOffset(range.end as usize)
                })
                .collect(),
            new_text: edit.new_text.into_iter().map(Arc::from).collect(),
        })),
        proto::operation::Variant::Undo(undo) => Some(text::Operation::Undo {
            lamport_timestamp: clock::Lamport {
                replica_id: undo.replica_id as text::ReplicaId,
                value: undo.lamport_timestamp,
            },
            undo: UndoOperation {
                id: clock::Local {
                    replica_id: undo.replica_id as text::ReplicaId,
                    value: undo.local_timestamp,
                },
                version: version_from_wire(&undo.version),
                counts: undo
                    .counts
                    .into_iter()
                    .map(|c| {
                        (
                            clock::Local {
                                replica_id: c.replica_id as text::ReplicaId,
                                value: c.local_timestamp,
                            },
                            c.count,
                        )
                    })
                    .collect(),
            },
        }),
        _ => None,
    }
}

fn version_from_wire(message: &[proto::VectorClockEntry]) -> clock::Global {
    let mut version = clock::Global::new();
    for entry in message {
        version.observe(clock::Local {
            replica_id: entry.replica_id as text::ReplicaId,
            value: entry.timestamp,
        });
    }
    version
}

mod storage {
    #![allow(non_snake_case)]
    use prost::Message;
    pub const SERIALIZATION_VERSION: i32 = 1;

    #[derive(Message)]
    pub struct Operation {
        #[prost(uint32, tag = "1")]
        pub local_timestamp: u32,
        #[prost(message, repeated, tag = "2")]
        pub version: Vec<VectorClockEntry>,
        #[prost(bool, tag = "3")]
        pub is_undo: bool,
        #[prost(message, repeated, tag = "4")]
        pub edit_ranges: Vec<Range>,
        #[prost(string, repeated, tag = "5")]
        pub edit_texts: Vec<String>,
        #[prost(message, repeated, tag = "6")]
        pub undo_counts: Vec<UndoCount>,
    }

    #[derive(Message)]
    pub struct VectorClockEntry {
        #[prost(uint32, tag = "1")]
        pub replica_id: u32,
        #[prost(uint32, tag = "2")]
        pub timestamp: u32,
    }

    #[derive(Message)]
    pub struct Range {
        #[prost(uint64, tag = "1")]
        pub start: u64,
        #[prost(uint64, tag = "2")]
        pub end: u64,
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
