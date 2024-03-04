use super::*;

impl Database {
    /// Returns the count of all projects, excluding ones marked as admin.
    pub async fn project_count_excluding_admins(&self) -> Result<usize> {
        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryAs {
            Count,
        }

        self.transaction(|tx| async move {
            Ok(project::Entity::find()
                .select_only()
                .column_as(project::Column::Id.count(), QueryAs::Count)
                .inner_join(user::Entity)
                .filter(user::Column::Admin.eq(false))
                .into_values::<_, QueryAs>()
                .one(&*tx)
                .await?
                .unwrap_or(0i64) as usize)
        })
        .await
    }

    /// Shares a project with the given room.
    pub async fn share_project(
        &self,
        room_id: RoomId,
        connection: ConnectionId,
        worktrees: &[proto::WorktreeMetadata],
    ) -> Result<RoomGuard<(ProjectId, proto::Room)>> {
        self.room_transaction(room_id, |tx| async move {
            let participant = room_participant::Entity::find()
                .filter(
                    Condition::all()
                        .add(
                            room_participant::Column::AnsweringConnectionId
                                .eq(connection.id as i32),
                        )
                        .add(
                            room_participant::Column::AnsweringConnectionServerId
                                .eq(connection.owner_id as i32),
                        ),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("could not find participant"))?;
            if participant.room_id != room_id {
                return Err(anyhow!("shared project on unexpected room"))?;
            }
            if !participant
                .role
                .unwrap_or(ChannelRole::Member)
                .can_edit_projects()
            {
                return Err(anyhow!("guests cannot share projects"))?;
            }

            let project = project::ActiveModel {
                room_id: ActiveValue::set(Some(participant.room_id)),
                host_user_id: ActiveValue::set(Some(participant.user_id)),
                host_connection_id: ActiveValue::set(Some(connection.id as i32)),
                host_connection_server_id: ActiveValue::set(Some(ServerId(
                    connection.owner_id as i32,
                ))),
                id: ActiveValue::NotSet,
                hosted_project_id: ActiveValue::Set(None),
            }
            .insert(&*tx)
            .await?;

            if !worktrees.is_empty() {
                worktree::Entity::insert_many(worktrees.iter().map(|worktree| {
                    worktree::ActiveModel {
                        id: ActiveValue::set(worktree.id as i64),
                        project_id: ActiveValue::set(project.id),
                        abs_path: ActiveValue::set(worktree.abs_path.clone()),
                        root_name: ActiveValue::set(worktree.root_name.clone()),
                        visible: ActiveValue::set(worktree.visible),
                        scan_id: ActiveValue::set(0),
                        completed_scan_id: ActiveValue::set(0),
                    }
                }))
                .exec(&*tx)
                .await?;
            }

            project_collaborator::ActiveModel {
                project_id: ActiveValue::set(project.id),
                connection_id: ActiveValue::set(connection.id as i32),
                connection_server_id: ActiveValue::set(ServerId(connection.owner_id as i32)),
                user_id: ActiveValue::set(participant.user_id),
                replica_id: ActiveValue::set(ReplicaId(0)),
                is_host: ActiveValue::set(true),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;

            let room = self.get_room(room_id, &tx).await?;
            Ok((project.id, room))
        })
        .await
    }

    /// Unshares the given project.
    pub async fn unshare_project(
        &self,
        project_id: ProjectId,
        connection: ConnectionId,
    ) -> Result<RoomGuard<(proto::Room, Vec<ConnectionId>)>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let guest_connection_ids = self.project_guest_connection_ids(project_id, &tx).await?;

            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("project not found"))?;
            if project.host_connection()? == connection {
                project::Entity::delete(project.into_active_model())
                    .exec(&*tx)
                    .await?;
                let room = self.get_room(room_id, &tx).await?;
                Ok((room, guest_connection_ids))
            } else {
                Err(anyhow!("cannot unshare a project hosted by another user"))?
            }
        })
        .await
    }

    /// Updates the worktrees associated with the given project.
    pub async fn update_project(
        &self,
        project_id: ProjectId,
        connection: ConnectionId,
        worktrees: &[proto::WorktreeMetadata],
    ) -> Result<RoomGuard<(proto::Room, Vec<ConnectionId>)>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let project = project::Entity::find_by_id(project_id)
                .filter(
                    Condition::all()
                        .add(project::Column::HostConnectionId.eq(connection.id as i32))
                        .add(
                            project::Column::HostConnectionServerId.eq(connection.owner_id as i32),
                        ),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;

            self.update_project_worktrees(project.id, worktrees, &tx)
                .await?;

            let room_id = project
                .room_id
                .ok_or_else(|| anyhow!("project not in a room"))?;

            let guest_connection_ids = self.project_guest_connection_ids(project.id, &tx).await?;
            let room = self.get_room(room_id, &tx).await?;
            Ok((room, guest_connection_ids))
        })
        .await
    }

    pub(in crate::db) async fn update_project_worktrees(
        &self,
        project_id: ProjectId,
        worktrees: &[proto::WorktreeMetadata],
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        if !worktrees.is_empty() {
            worktree::Entity::insert_many(worktrees.iter().map(|worktree| worktree::ActiveModel {
                id: ActiveValue::set(worktree.id as i64),
                project_id: ActiveValue::set(project_id),
                abs_path: ActiveValue::set(worktree.abs_path.clone()),
                root_name: ActiveValue::set(worktree.root_name.clone()),
                visible: ActiveValue::set(worktree.visible),
                scan_id: ActiveValue::set(0),
                completed_scan_id: ActiveValue::set(0),
            }))
            .on_conflict(
                OnConflict::columns([worktree::Column::ProjectId, worktree::Column::Id])
                    .update_column(worktree::Column::RootName)
                    .to_owned(),
            )
            .exec(&*tx)
            .await?;
        }

        worktree::Entity::delete_many()
            .filter(worktree::Column::ProjectId.eq(project_id).and(
                worktree::Column::Id.is_not_in(worktrees.iter().map(|worktree| worktree.id as i64)),
            ))
            .exec(&*tx)
            .await?;

        Ok(())
    }

    pub async fn update_worktree(
        &self,
        update: &proto::UpdateWorktree,
        connection: ConnectionId,
    ) -> Result<RoomGuard<Vec<ConnectionId>>> {
        let project_id = ProjectId::from_proto(update.project_id);
        let worktree_id = update.worktree_id as i64;
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            // Ensure the update comes from the host.
            let _project = project::Entity::find_by_id(project_id)
                .filter(
                    Condition::all()
                        .add(project::Column::HostConnectionId.eq(connection.id as i32))
                        .add(
                            project::Column::HostConnectionServerId.eq(connection.owner_id as i32),
                        ),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;

            // Update metadata.
            worktree::Entity::update(worktree::ActiveModel {
                id: ActiveValue::set(worktree_id),
                project_id: ActiveValue::set(project_id),
                root_name: ActiveValue::set(update.root_name.clone()),
                scan_id: ActiveValue::set(update.scan_id as i64),
                completed_scan_id: if update.is_last_update {
                    ActiveValue::set(update.scan_id as i64)
                } else {
                    ActiveValue::default()
                },
                abs_path: ActiveValue::set(update.abs_path.clone()),
                ..Default::default()
            })
            .exec(&*tx)
            .await?;

            if !update.updated_entries.is_empty() {
                worktree_entry::Entity::insert_many(update.updated_entries.iter().map(|entry| {
                    let mtime = entry.mtime.clone().unwrap_or_default();
                    worktree_entry::ActiveModel {
                        project_id: ActiveValue::set(project_id),
                        worktree_id: ActiveValue::set(worktree_id),
                        id: ActiveValue::set(entry.id as i64),
                        is_dir: ActiveValue::set(entry.is_dir),
                        path: ActiveValue::set(entry.path.clone()),
                        inode: ActiveValue::set(entry.inode as i64),
                        mtime_seconds: ActiveValue::set(mtime.seconds as i64),
                        mtime_nanos: ActiveValue::set(mtime.nanos as i32),
                        is_symlink: ActiveValue::set(entry.is_symlink),
                        is_ignored: ActiveValue::set(entry.is_ignored),
                        is_external: ActiveValue::set(entry.is_external),
                        git_status: ActiveValue::set(entry.git_status.map(|status| status as i64)),
                        is_deleted: ActiveValue::set(false),
                        scan_id: ActiveValue::set(update.scan_id as i64),
                    }
                }))
                .on_conflict(
                    OnConflict::columns([
                        worktree_entry::Column::ProjectId,
                        worktree_entry::Column::WorktreeId,
                        worktree_entry::Column::Id,
                    ])
                    .update_columns([
                        worktree_entry::Column::IsDir,
                        worktree_entry::Column::Path,
                        worktree_entry::Column::Inode,
                        worktree_entry::Column::MtimeSeconds,
                        worktree_entry::Column::MtimeNanos,
                        worktree_entry::Column::IsSymlink,
                        worktree_entry::Column::IsIgnored,
                        worktree_entry::Column::GitStatus,
                        worktree_entry::Column::ScanId,
                    ])
                    .to_owned(),
                )
                .exec(&*tx)
                .await?;
            }

            if !update.removed_entries.is_empty() {
                worktree_entry::Entity::update_many()
                    .filter(
                        worktree_entry::Column::ProjectId
                            .eq(project_id)
                            .and(worktree_entry::Column::WorktreeId.eq(worktree_id))
                            .and(
                                worktree_entry::Column::Id
                                    .is_in(update.removed_entries.iter().map(|id| *id as i64)),
                            ),
                    )
                    .set(worktree_entry::ActiveModel {
                        is_deleted: ActiveValue::Set(true),
                        scan_id: ActiveValue::Set(update.scan_id as i64),
                        ..Default::default()
                    })
                    .exec(&*tx)
                    .await?;
            }

            if !update.updated_repositories.is_empty() {
                worktree_repository::Entity::insert_many(update.updated_repositories.iter().map(
                    |repository| worktree_repository::ActiveModel {
                        project_id: ActiveValue::set(project_id),
                        worktree_id: ActiveValue::set(worktree_id),
                        work_directory_id: ActiveValue::set(repository.work_directory_id as i64),
                        scan_id: ActiveValue::set(update.scan_id as i64),
                        branch: ActiveValue::set(repository.branch.clone()),
                        is_deleted: ActiveValue::set(false),
                    },
                ))
                .on_conflict(
                    OnConflict::columns([
                        worktree_repository::Column::ProjectId,
                        worktree_repository::Column::WorktreeId,
                        worktree_repository::Column::WorkDirectoryId,
                    ])
                    .update_columns([
                        worktree_repository::Column::ScanId,
                        worktree_repository::Column::Branch,
                    ])
                    .to_owned(),
                )
                .exec(&*tx)
                .await?;
            }

            if !update.removed_repositories.is_empty() {
                worktree_repository::Entity::update_many()
                    .filter(
                        worktree_repository::Column::ProjectId
                            .eq(project_id)
                            .and(worktree_repository::Column::WorktreeId.eq(worktree_id))
                            .and(
                                worktree_repository::Column::WorkDirectoryId
                                    .is_in(update.removed_repositories.iter().map(|id| *id as i64)),
                            ),
                    )
                    .set(worktree_repository::ActiveModel {
                        is_deleted: ActiveValue::Set(true),
                        scan_id: ActiveValue::Set(update.scan_id as i64),
                        ..Default::default()
                    })
                    .exec(&*tx)
                    .await?;
            }

            let connection_ids = self.project_guest_connection_ids(project_id, &tx).await?;
            Ok(connection_ids)
        })
        .await
    }

    /// Updates the diagnostic summary for the given connection.
    pub async fn update_diagnostic_summary(
        &self,
        update: &proto::UpdateDiagnosticSummary,
        connection: ConnectionId,
    ) -> Result<RoomGuard<Vec<ConnectionId>>> {
        let project_id = ProjectId::from_proto(update.project_id);
        let worktree_id = update.worktree_id as i64;
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let summary = update
                .summary
                .as_ref()
                .ok_or_else(|| anyhow!("invalid summary"))?;

            // Ensure the update comes from the host.
            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;
            if project.host_connection()? != connection {
                return Err(anyhow!("can't update a project hosted by someone else"))?;
            }

            // Update summary.
            worktree_diagnostic_summary::Entity::insert(worktree_diagnostic_summary::ActiveModel {
                project_id: ActiveValue::set(project_id),
                worktree_id: ActiveValue::set(worktree_id),
                path: ActiveValue::set(summary.path.clone()),
                language_server_id: ActiveValue::set(summary.language_server_id as i64),
                error_count: ActiveValue::set(summary.error_count as i32),
                warning_count: ActiveValue::set(summary.warning_count as i32),
            })
            .on_conflict(
                OnConflict::columns([
                    worktree_diagnostic_summary::Column::ProjectId,
                    worktree_diagnostic_summary::Column::WorktreeId,
                    worktree_diagnostic_summary::Column::Path,
                ])
                .update_columns([
                    worktree_diagnostic_summary::Column::LanguageServerId,
                    worktree_diagnostic_summary::Column::ErrorCount,
                    worktree_diagnostic_summary::Column::WarningCount,
                ])
                .to_owned(),
            )
            .exec(&*tx)
            .await?;

            let connection_ids = self.project_guest_connection_ids(project_id, &tx).await?;
            Ok(connection_ids)
        })
        .await
    }

    /// Starts the language server for the given connection.
    pub async fn start_language_server(
        &self,
        update: &proto::StartLanguageServer,
        connection: ConnectionId,
    ) -> Result<RoomGuard<Vec<ConnectionId>>> {
        let project_id = ProjectId::from_proto(update.project_id);
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let server = update
                .server
                .as_ref()
                .ok_or_else(|| anyhow!("invalid language server"))?;

            // Ensure the update comes from the host.
            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;
            if project.host_connection()? != connection {
                return Err(anyhow!("can't update a project hosted by someone else"))?;
            }

            // Add the newly-started language server.
            language_server::Entity::insert(language_server::ActiveModel {
                project_id: ActiveValue::set(project_id),
                id: ActiveValue::set(server.id as i64),
                name: ActiveValue::set(server.name.clone()),
            })
            .on_conflict(
                OnConflict::columns([
                    language_server::Column::ProjectId,
                    language_server::Column::Id,
                ])
                .update_column(language_server::Column::Name)
                .to_owned(),
            )
            .exec(&*tx)
            .await?;

            let connection_ids = self.project_guest_connection_ids(project_id, &tx).await?;
            Ok(connection_ids)
        })
        .await
    }

    /// Updates the worktree settings for the given connection.
    pub async fn update_worktree_settings(
        &self,
        update: &proto::UpdateWorktreeSettings,
        connection: ConnectionId,
    ) -> Result<RoomGuard<Vec<ConnectionId>>> {
        let project_id = ProjectId::from_proto(update.project_id);
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            // Ensure the update comes from the host.
            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;
            if project.host_connection()? != connection {
                return Err(anyhow!("can't update a project hosted by someone else"))?;
            }

            if let Some(content) = &update.content {
                worktree_settings_file::Entity::insert(worktree_settings_file::ActiveModel {
                    project_id: ActiveValue::Set(project_id),
                    worktree_id: ActiveValue::Set(update.worktree_id as i64),
                    path: ActiveValue::Set(update.path.clone()),
                    content: ActiveValue::Set(content.clone()),
                })
                .on_conflict(
                    OnConflict::columns([
                        worktree_settings_file::Column::ProjectId,
                        worktree_settings_file::Column::WorktreeId,
                        worktree_settings_file::Column::Path,
                    ])
                    .update_column(worktree_settings_file::Column::Content)
                    .to_owned(),
                )
                .exec(&*tx)
                .await?;
            } else {
                worktree_settings_file::Entity::delete(worktree_settings_file::ActiveModel {
                    project_id: ActiveValue::Set(project_id),
                    worktree_id: ActiveValue::Set(update.worktree_id as i64),
                    path: ActiveValue::Set(update.path.clone()),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            }

            let connection_ids = self.project_guest_connection_ids(project_id, &tx).await?;
            Ok(connection_ids)
        })
        .await
    }

    /// Adds the given connection to the specified hosted project
    pub async fn join_hosted_project(
        &self,
        id: HostedProjectId,
        user_id: UserId,
        connection: ConnectionId,
    ) -> Result<(Project, ReplicaId)> {
        self.transaction(|tx| async move {
            let (hosted_project, role) = self.get_hosted_project(id, user_id, &tx).await?;
            let project = project::Entity::find()
                .filter(project::Column::HostedProjectId.eq(hosted_project.id))
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("hosted project is no longer shared"))?;

            self.join_project_internal(project, user_id, connection, role, &tx)
                .await
        })
        .await
    }

    /// Adds the given connection to the specified project
    /// in the current room.
    pub async fn join_project_in_room(
        &self,
        project_id: ProjectId,
        connection: ConnectionId,
    ) -> Result<RoomGuard<(Project, ReplicaId)>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let participant = room_participant::Entity::find()
                .filter(
                    Condition::all()
                        .add(
                            room_participant::Column::AnsweringConnectionId
                                .eq(connection.id as i32),
                        )
                        .add(
                            room_participant::Column::AnsweringConnectionServerId
                                .eq(connection.owner_id as i32),
                        ),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("must join a room first"))?;

            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;
            if project.room_id != Some(participant.room_id) {
                return Err(anyhow!("no such project"))?;
            }
            self.join_project_internal(
                project,
                participant.user_id,
                connection,
                participant.role.unwrap_or(ChannelRole::Member),
                &tx,
            )
            .await
        })
        .await
    }

    async fn join_project_internal(
        &self,
        project: project::Model,
        user_id: UserId,
        connection: ConnectionId,
        role: ChannelRole,
        tx: &DatabaseTransaction,
    ) -> Result<(Project, ReplicaId)> {
        let mut collaborators = project
            .find_related(project_collaborator::Entity)
            .all(&*tx)
            .await?;
        let replica_ids = collaborators
            .iter()
            .map(|c| c.replica_id)
            .collect::<HashSet<_>>();
        let mut replica_id = ReplicaId(1);
        while replica_ids.contains(&replica_id) {
            replica_id.0 += 1;
        }
        let new_collaborator = project_collaborator::ActiveModel {
            project_id: ActiveValue::set(project.id),
            connection_id: ActiveValue::set(connection.id as i32),
            connection_server_id: ActiveValue::set(ServerId(connection.owner_id as i32)),
            user_id: ActiveValue::set(user_id),
            replica_id: ActiveValue::set(replica_id),
            is_host: ActiveValue::set(false),
            ..Default::default()
        }
        .insert(&*tx)
        .await?;
        collaborators.push(new_collaborator);

        let db_worktrees = project.find_related(worktree::Entity).all(&*tx).await?;
        let mut worktrees = db_worktrees
            .into_iter()
            .map(|db_worktree| {
                (
                    db_worktree.id as u64,
                    Worktree {
                        id: db_worktree.id as u64,
                        abs_path: db_worktree.abs_path,
                        root_name: db_worktree.root_name,
                        visible: db_worktree.visible,
                        entries: Default::default(),
                        repository_entries: Default::default(),
                        diagnostic_summaries: Default::default(),
                        settings_files: Default::default(),
                        scan_id: db_worktree.scan_id as u64,
                        completed_scan_id: db_worktree.completed_scan_id as u64,
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();

        // Populate worktree entries.
        {
            let mut db_entries = worktree_entry::Entity::find()
                .filter(
                    Condition::all()
                        .add(worktree_entry::Column::ProjectId.eq(project.id))
                        .add(worktree_entry::Column::IsDeleted.eq(false)),
                )
                .stream(&*tx)
                .await?;
            while let Some(db_entry) = db_entries.next().await {
                let db_entry = db_entry?;
                if let Some(worktree) = worktrees.get_mut(&(db_entry.worktree_id as u64)) {
                    worktree.entries.push(proto::Entry {
                        id: db_entry.id as u64,
                        is_dir: db_entry.is_dir,
                        path: db_entry.path,
                        inode: db_entry.inode as u64,
                        mtime: Some(proto::Timestamp {
                            seconds: db_entry.mtime_seconds as u64,
                            nanos: db_entry.mtime_nanos as u32,
                        }),
                        is_symlink: db_entry.is_symlink,
                        is_ignored: db_entry.is_ignored,
                        is_external: db_entry.is_external,
                        git_status: db_entry.git_status.map(|status| status as i32),
                    });
                }
            }
        }

        // Populate repository entries.
        {
            let mut db_repository_entries = worktree_repository::Entity::find()
                .filter(
                    Condition::all()
                        .add(worktree_repository::Column::ProjectId.eq(project.id))
                        .add(worktree_repository::Column::IsDeleted.eq(false)),
                )
                .stream(&*tx)
                .await?;
            while let Some(db_repository_entry) = db_repository_entries.next().await {
                let db_repository_entry = db_repository_entry?;
                if let Some(worktree) = worktrees.get_mut(&(db_repository_entry.worktree_id as u64))
                {
                    worktree.repository_entries.insert(
                        db_repository_entry.work_directory_id as u64,
                        proto::RepositoryEntry {
                            work_directory_id: db_repository_entry.work_directory_id as u64,
                            branch: db_repository_entry.branch,
                        },
                    );
                }
            }
        }

        // Populate worktree diagnostic summaries.
        {
            let mut db_summaries = worktree_diagnostic_summary::Entity::find()
                .filter(worktree_diagnostic_summary::Column::ProjectId.eq(project.id))
                .stream(&*tx)
                .await?;
            while let Some(db_summary) = db_summaries.next().await {
                let db_summary = db_summary?;
                if let Some(worktree) = worktrees.get_mut(&(db_summary.worktree_id as u64)) {
                    worktree
                        .diagnostic_summaries
                        .push(proto::DiagnosticSummary {
                            path: db_summary.path,
                            language_server_id: db_summary.language_server_id as u64,
                            error_count: db_summary.error_count as u32,
                            warning_count: db_summary.warning_count as u32,
                        });
                }
            }
        }

        // Populate worktree settings files
        {
            let mut db_settings_files = worktree_settings_file::Entity::find()
                .filter(worktree_settings_file::Column::ProjectId.eq(project.id))
                .stream(&*tx)
                .await?;
            while let Some(db_settings_file) = db_settings_files.next().await {
                let db_settings_file = db_settings_file?;
                if let Some(worktree) = worktrees.get_mut(&(db_settings_file.worktree_id as u64)) {
                    worktree.settings_files.push(WorktreeSettingsFile {
                        path: db_settings_file.path,
                        content: db_settings_file.content,
                    });
                }
            }
        }

        // Populate language servers.
        let language_servers = project
            .find_related(language_server::Entity)
            .all(&*tx)
            .await?;

        let project = Project {
            id: project.id,
            role,
            collaborators: collaborators
                .into_iter()
                .map(|collaborator| ProjectCollaborator {
                    connection_id: collaborator.connection(),
                    user_id: collaborator.user_id,
                    replica_id: collaborator.replica_id,
                    is_host: collaborator.is_host,
                })
                .collect(),
            worktrees,
            language_servers: language_servers
                .into_iter()
                .map(|language_server| proto::LanguageServer {
                    id: language_server.id as u64,
                    name: language_server.name,
                })
                .collect(),
        };
        Ok((project, replica_id as ReplicaId))
    }

    pub async fn leave_hosted_project(
        &self,
        project_id: ProjectId,
        connection: ConnectionId,
    ) -> Result<LeftProject> {
        self.transaction(|tx| async move {
            let result = project_collaborator::Entity::delete_many()
                .filter(
                    Condition::all()
                        .add(project_collaborator::Column::ProjectId.eq(project_id))
                        .add(project_collaborator::Column::ConnectionId.eq(connection.id as i32))
                        .add(
                            project_collaborator::Column::ConnectionServerId
                                .eq(connection.owner_id as i32),
                        ),
                )
                .exec(&*tx)
                .await?;
            if result.rows_affected == 0 {
                return Err(anyhow!("not in the project"))?;
            }

            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;
            let collaborators = project
                .find_related(project_collaborator::Entity)
                .all(&*tx)
                .await?;
            let connection_ids = collaborators
                .into_iter()
                .map(|collaborator| collaborator.connection())
                .collect();
            Ok(LeftProject {
                id: project.id,
                connection_ids,
                host_user_id: None,
                host_connection_id: None,
            })
        })
        .await
    }

    /// Removes the given connection from the specified project.
    pub async fn leave_project(
        &self,
        project_id: ProjectId,
        connection: ConnectionId,
    ) -> Result<RoomGuard<(proto::Room, LeftProject)>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let result = project_collaborator::Entity::delete_many()
                .filter(
                    Condition::all()
                        .add(project_collaborator::Column::ProjectId.eq(project_id))
                        .add(project_collaborator::Column::ConnectionId.eq(connection.id as i32))
                        .add(
                            project_collaborator::Column::ConnectionServerId
                                .eq(connection.owner_id as i32),
                        ),
                )
                .exec(&*tx)
                .await?;
            if result.rows_affected == 0 {
                Err(anyhow!("not a collaborator on this project"))?;
            }

            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;
            let collaborators = project
                .find_related(project_collaborator::Entity)
                .all(&*tx)
                .await?;
            let connection_ids = collaborators
                .into_iter()
                .map(|collaborator| collaborator.connection())
                .collect();

            follower::Entity::delete_many()
                .filter(
                    Condition::any()
                        .add(
                            Condition::all()
                                .add(follower::Column::ProjectId.eq(Some(project_id)))
                                .add(
                                    follower::Column::LeaderConnectionServerId
                                        .eq(connection.owner_id),
                                )
                                .add(follower::Column::LeaderConnectionId.eq(connection.id)),
                        )
                        .add(
                            Condition::all()
                                .add(follower::Column::ProjectId.eq(Some(project_id)))
                                .add(
                                    follower::Column::FollowerConnectionServerId
                                        .eq(connection.owner_id),
                                )
                                .add(follower::Column::FollowerConnectionId.eq(connection.id)),
                        ),
                )
                .exec(&*tx)
                .await?;

            let room = self.get_room(room_id, &tx).await?;
            let left_project = LeftProject {
                id: project_id,
                host_user_id: project.host_user_id,
                host_connection_id: Some(project.host_connection()?),
                connection_ids,
            };
            Ok((room, left_project))
        })
        .await
    }

    pub async fn check_user_is_project_host(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<()> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            project_collaborator::Entity::find()
                .filter(
                    Condition::all()
                        .add(project_collaborator::Column::ProjectId.eq(project_id))
                        .add(project_collaborator::Column::IsHost.eq(true))
                        .add(project_collaborator::Column::ConnectionId.eq(connection_id.id))
                        .add(
                            project_collaborator::Column::ConnectionServerId
                                .eq(connection_id.owner_id),
                        ),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("failed to read project host"))?;

            Ok(())
        })
        .await
        .map(|guard| guard.into_inner())
    }

    /// Returns the host connection for a read-only request to join a shared project.
    pub async fn host_for_read_only_project_request(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<ConnectionId> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let current_participant = room_participant::Entity::find()
                .filter(room_participant::Column::RoomId.eq(room_id))
                .filter(room_participant::Column::AnsweringConnectionId.eq(connection_id.id))
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such room"))?;

            if !current_participant
                .role
                .map_or(false, |role| role.can_read_projects())
            {
                Err(anyhow!("not authorized to read projects"))?;
            }

            let host = project_collaborator::Entity::find()
                .filter(
                    project_collaborator::Column::ProjectId
                        .eq(project_id)
                        .and(project_collaborator::Column::IsHost.eq(true)),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("failed to read project host"))?;

            Ok(host.connection())
        })
        .await
        .map(|guard| guard.into_inner())
    }

    /// Returns the host connection for a request to join a shared project.
    pub async fn host_for_mutating_project_request(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<ConnectionId> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let current_participant = room_participant::Entity::find()
                .filter(room_participant::Column::RoomId.eq(room_id))
                .filter(room_participant::Column::AnsweringConnectionId.eq(connection_id.id))
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such room"))?;

            if !current_participant
                .role
                .map_or(false, |role| role.can_edit_projects())
            {
                Err(anyhow!("not authorized to edit projects"))?;
            }

            let host = project_collaborator::Entity::find()
                .filter(
                    project_collaborator::Column::ProjectId
                        .eq(project_id)
                        .and(project_collaborator::Column::IsHost.eq(true)),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("failed to read project host"))?;

            Ok(host.connection())
        })
        .await
        .map(|guard| guard.into_inner())
    }

    pub async fn project_collaborators_for_buffer_update(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
        requires_write: bool,
    ) -> Result<RoomGuard<Vec<ProjectCollaborator>>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let current_participant = room_participant::Entity::find()
                .filter(room_participant::Column::RoomId.eq(room_id))
                .filter(room_participant::Column::AnsweringConnectionId.eq(connection_id.id))
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such room"))?;

            if requires_write
                && !current_participant
                    .role
                    .map_or(false, |role| role.can_edit_projects())
            {
                Err(anyhow!("not authorized to edit projects"))?;
            }

            let collaborators = project_collaborator::Entity::find()
                .filter(project_collaborator::Column::ProjectId.eq(project_id))
                .all(&*tx)
                .await?
                .into_iter()
                .map(|collaborator| ProjectCollaborator {
                    connection_id: collaborator.connection(),
                    user_id: collaborator.user_id,
                    replica_id: collaborator.replica_id,
                    is_host: collaborator.is_host,
                })
                .collect::<Vec<_>>();

            if collaborators
                .iter()
                .any(|collaborator| collaborator.connection_id == connection_id)
            {
                Ok(collaborators)
            } else {
                Err(anyhow!("no such project"))?
            }
        })
        .await
    }

    /// Returns the connection IDs in the given project.
    ///
    /// The provided `connection_id` must also be a collaborator in the project,
    /// otherwise an error will be returned.
    pub async fn project_connection_ids(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<RoomGuard<HashSet<ConnectionId>>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let mut collaborators = project_collaborator::Entity::find()
                .filter(project_collaborator::Column::ProjectId.eq(project_id))
                .stream(&*tx)
                .await?;

            let mut connection_ids = HashSet::default();
            while let Some(collaborator) = collaborators.next().await {
                let collaborator = collaborator?;
                connection_ids.insert(collaborator.connection());
            }

            if connection_ids.contains(&connection_id) {
                Ok(connection_ids)
            } else {
                Err(anyhow!("no such project"))?
            }
        })
        .await
    }

    async fn project_guest_connection_ids(
        &self,
        project_id: ProjectId,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<ConnectionId>> {
        let mut collaborators = project_collaborator::Entity::find()
            .filter(
                project_collaborator::Column::ProjectId
                    .eq(project_id)
                    .and(project_collaborator::Column::IsHost.eq(false)),
            )
            .stream(tx)
            .await?;

        let mut guest_connection_ids = Vec::new();
        while let Some(collaborator) = collaborators.next().await {
            let collaborator = collaborator?;
            guest_connection_ids.push(collaborator.connection());
        }
        Ok(guest_connection_ids)
    }

    /// Returns the [`RoomId`] for the given project.
    pub async fn room_id_for_project(&self, project_id: ProjectId) -> Result<RoomId> {
        self.transaction(|tx| async move {
            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("project {} not found", project_id))?;
            Ok(project
                .room_id
                .ok_or_else(|| anyhow!("project not in room"))?)
        })
        .await
    }

    pub async fn check_room_participants(
        &self,
        room_id: RoomId,
        leader_id: ConnectionId,
        follower_id: ConnectionId,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            use room_participant::Column;

            let count = room_participant::Entity::find()
                .filter(
                    Condition::all().add(Column::RoomId.eq(room_id)).add(
                        Condition::any()
                            .add(Column::AnsweringConnectionId.eq(leader_id.id as i32).and(
                                Column::AnsweringConnectionServerId.eq(leader_id.owner_id as i32),
                            ))
                            .add(Column::AnsweringConnectionId.eq(follower_id.id as i32).and(
                                Column::AnsweringConnectionServerId.eq(follower_id.owner_id as i32),
                            )),
                    ),
                )
                .count(&*tx)
                .await?;

            if count < 2 {
                Err(anyhow!("not room participants"))?;
            }

            Ok(())
        })
        .await
    }

    /// Adds the given follower connection as a follower of the given leader connection.
    pub async fn follow(
        &self,
        room_id: RoomId,
        project_id: ProjectId,
        leader_connection: ConnectionId,
        follower_connection: ConnectionId,
    ) -> Result<RoomGuard<proto::Room>> {
        self.room_transaction(room_id, |tx| async move {
            follower::ActiveModel {
                room_id: ActiveValue::set(room_id),
                project_id: ActiveValue::set(project_id),
                leader_connection_server_id: ActiveValue::set(ServerId(
                    leader_connection.owner_id as i32,
                )),
                leader_connection_id: ActiveValue::set(leader_connection.id as i32),
                follower_connection_server_id: ActiveValue::set(ServerId(
                    follower_connection.owner_id as i32,
                )),
                follower_connection_id: ActiveValue::set(follower_connection.id as i32),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;

            let room = self.get_room(room_id, &tx).await?;
            Ok(room)
        })
        .await
    }

    /// Removes the given follower connection as a follower of the given leader connection.
    pub async fn unfollow(
        &self,
        room_id: RoomId,
        project_id: ProjectId,
        leader_connection: ConnectionId,
        follower_connection: ConnectionId,
    ) -> Result<RoomGuard<proto::Room>> {
        self.room_transaction(room_id, |tx| async move {
            follower::Entity::delete_many()
                .filter(
                    Condition::all()
                        .add(follower::Column::RoomId.eq(room_id))
                        .add(follower::Column::ProjectId.eq(project_id))
                        .add(
                            follower::Column::LeaderConnectionServerId
                                .eq(leader_connection.owner_id),
                        )
                        .add(follower::Column::LeaderConnectionId.eq(leader_connection.id))
                        .add(
                            follower::Column::FollowerConnectionServerId
                                .eq(follower_connection.owner_id),
                        )
                        .add(follower::Column::FollowerConnectionId.eq(follower_connection.id)),
                )
                .exec(&*tx)
                .await?;

            let room = self.get_room(room_id, &tx).await?;
            Ok(room)
        })
        .await
    }
}
