use super::*;

impl Database {
    /// Creates a new server in the given environment.
    pub async fn create_server(&self, environment: &str) -> Result<ServerId> {
        self.transaction(|tx| async move {
            let server = server::ActiveModel {
                environment: ActiveValue::set(environment.into()),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;
            Ok(server.id)
        })
        .await
    }

    /// Returns the IDs of resources associated with stale servers.
    ///
    /// A server is stale if it is in the specified `environment` and does not
    /// match the provided `new_server_id`.
    pub async fn stale_server_resource_ids(
        &self,
        environment: &str,
        new_server_id: ServerId,
    ) -> Result<(Vec<RoomId>, Vec<ChannelId>)> {
        self.transaction(|tx| async move {
            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            enum QueryRoomIds {
                RoomId,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            enum QueryChannelIds {
                ChannelId,
            }

            let stale_server_epochs = self
                .stale_server_ids(environment, new_server_id, &tx)
                .await?;
            let room_ids = room_participant::Entity::find()
                .select_only()
                .column(room_participant::Column::RoomId)
                .distinct()
                .filter(
                    room_participant::Column::AnsweringConnectionServerId
                        .is_in(stale_server_epochs.iter().copied()),
                )
                .into_values::<_, QueryRoomIds>()
                .all(&*tx)
                .await?;
            let channel_ids = channel_buffer_collaborator::Entity::find()
                .select_only()
                .column(channel_buffer_collaborator::Column::ChannelId)
                .distinct()
                .filter(
                    channel_buffer_collaborator::Column::ConnectionServerId
                        .is_in(stale_server_epochs.iter().copied()),
                )
                .into_values::<_, QueryChannelIds>()
                .all(&*tx)
                .await?;

            Ok((room_ids, channel_ids))
        })
        .await
    }

    /// Deletes any stale servers in the environment that don't match the `new_server_id`.
    pub async fn delete_stale_servers(
        &self,
        environment: &str,
        new_server_id: ServerId,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            server::Entity::delete_many()
                .filter(
                    Condition::all()
                        .add(server::Column::Environment.eq(environment))
                        .add(server::Column::Id.ne(new_server_id)),
                )
                .exec(&*tx)
                .await?;
            Ok(())
        })
        .await
    }

    async fn stale_server_ids(
        &self,
        environment: &str,
        new_server_id: ServerId,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<ServerId>> {
        let stale_servers = server::Entity::find()
            .filter(
                Condition::all()
                    .add(server::Column::Environment.eq(environment))
                    .add(server::Column::Id.ne(new_server_id)),
            )
            .all(tx)
            .await?;
        Ok(stale_servers.into_iter().map(|server| server.id).collect())
    }
}
