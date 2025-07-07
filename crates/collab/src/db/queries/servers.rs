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

    /// Delete all channel chat participants from previous servers
    pub async fn delete_stale_channel_chat_participants(
        &self,
        environment: &str,
        new_server_id: ServerId,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            let stale_server_epochs = self
                .stale_server_ids(environment, new_server_id, &tx)
                .await?;

            channel_chat_participant::Entity::delete_many()
                .filter(
                    channel_chat_participant::Column::ConnectionServerId
                        .is_in(stale_server_epochs.iter().copied()),
                )
                .exec(&*tx)
                .await?;

            Ok(())
        })
        .await
    }

    pub async fn clear_old_worktree_entries(&self, server_id: ServerId) -> Result<()> {
        self.transaction(|tx| async move {
            use sea_orm::Statement;
            use sea_orm::sea_query::{Expr, Query};

            loop {
                let delete_query = Query::delete()
                    .from_table(worktree_entry::Entity)
                    .and_where(
                        Expr::tuple([
                            Expr::col((worktree_entry::Entity, worktree_entry::Column::ProjectId))
                                .into(),
                            Expr::col((worktree_entry::Entity, worktree_entry::Column::WorktreeId))
                                .into(),
                            Expr::col((worktree_entry::Entity, worktree_entry::Column::Id)).into(),
                        ])
                        .in_subquery(
                            Query::select()
                                .columns([
                                    (worktree_entry::Entity, worktree_entry::Column::ProjectId),
                                    (worktree_entry::Entity, worktree_entry::Column::WorktreeId),
                                    (worktree_entry::Entity, worktree_entry::Column::Id),
                                ])
                                .from(worktree_entry::Entity)
                                .inner_join(
                                    project::Entity,
                                    Expr::col((project::Entity, project::Column::Id)).equals((
                                        worktree_entry::Entity,
                                        worktree_entry::Column::ProjectId,
                                    )),
                                )
                                .and_where(project::Column::HostConnectionServerId.ne(server_id))
                                .limit(10000)
                                .to_owned(),
                        ),
                    )
                    .to_owned();

                let statement = Statement::from_sql_and_values(
                    tx.get_database_backend(),
                    delete_query
                        .to_string(sea_orm::sea_query::PostgresQueryBuilder)
                        .as_str(),
                    vec![],
                );

                let result = tx.execute(statement).await?;
                if result.rows_affected() == 0 {
                    break;
                }
            }

            loop {
                let delete_query = Query::delete()
                    .from_table(project_repository_statuses::Entity)
                    .and_where(
                        Expr::tuple([Expr::col((
                            project_repository_statuses::Entity,
                            project_repository_statuses::Column::ProjectId,
                        ))
                        .into()])
                        .in_subquery(
                            Query::select()
                                .columns([(
                                    project_repository_statuses::Entity,
                                    project_repository_statuses::Column::ProjectId,
                                )])
                                .from(project_repository_statuses::Entity)
                                .inner_join(
                                    project::Entity,
                                    Expr::col((project::Entity, project::Column::Id)).equals((
                                        project_repository_statuses::Entity,
                                        project_repository_statuses::Column::ProjectId,
                                    )),
                                )
                                .and_where(project::Column::HostConnectionServerId.ne(server_id))
                                .limit(10000)
                                .to_owned(),
                        ),
                    )
                    .to_owned();

                let statement = Statement::from_sql_and_values(
                    tx.get_database_backend(),
                    delete_query
                        .to_string(sea_orm::sea_query::PostgresQueryBuilder)
                        .as_str(),
                    vec![],
                );

                let result = tx.execute(statement).await?;
                if result.rows_affected() == 0 {
                    break;
                }
            }

            Ok(())
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

    pub async fn stale_server_ids(
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
