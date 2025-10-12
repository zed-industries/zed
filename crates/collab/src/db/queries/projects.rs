use anyhow::Context as _;
use collections::HashSet;
use util::ResultExt;

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
        is_ssh_project: bool,
        windows_paths: bool,
    ) -> Result<TransactionGuard<(ProjectId, proto::Room)>> {
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
                .context("could not find participant")?;
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
                windows_paths: ActiveValue::set(windows_paths),
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

            let replica_id = if is_ssh_project { 1 } else { 0 };

            project_collaborator::ActiveModel {
                project_id: ActiveValue::set(project.id),
                connection_id: ActiveValue::set(connection.id as i32),
                connection_server_id: ActiveValue::set(ServerId(connection.owner_id as i32)),
                user_id: ActiveValue::set(participant.user_id),
                replica_id: ActiveValue::set(ReplicaId(replica_id)),
                is_host: ActiveValue::set(true),
                id: ActiveValue::NotSet,
                committer_name: ActiveValue::Set(None),
                committer_email: ActiveValue::Set(None),
            }
            .insert(&*tx)
            .await?;

            let room = self.get_room(room_id, &tx).await?;
            Ok((project.id, room))
        })
        .await
    }

    pub async fn delete_project(&self, project_id: ProjectId) -> Result<()> {
        self.transaction(|tx| async move {
            project::Entity::delete_by_id(project_id).exec(&*tx).await?;
            Ok(())
        })
        .await
    }

    /// Unshares the given project.
    pub async fn unshare_project(
        &self,
        project_id: ProjectId,
        connection: ConnectionId,
    ) -> Result<TransactionGuard<(bool, Option<proto::Room>, Vec<ConnectionId>)>> {
        self.project_transaction(project_id, |tx| async move {
            let guest_connection_ids = self.project_guest_connection_ids(project_id, &tx).await?;
            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .context("project not found")?;
            let room = if let Some(room_id) = project.room_id {
                Some(self.get_room(room_id, &tx).await?)
            } else {
                None
            };
            if project.host_connection()? == connection {
                return Ok((true, room, guest_connection_ids));
            }
            Err(anyhow!("cannot unshare a project hosted by another user"))?
        })
        .await
    }

    /// Updates the worktrees associated with the given project.
    pub async fn update_project(
        &self,
        project_id: ProjectId,
        connection: ConnectionId,
        worktrees: &[proto::WorktreeMetadata],
    ) -> Result<TransactionGuard<(Option<proto::Room>, Vec<ConnectionId>)>> {
        self.project_transaction(project_id, |tx| async move {
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
                .context("no such project")?;

            self.update_project_worktrees(project.id, worktrees, &tx)
                .await?;

            let guest_connection_ids = self.project_guest_connection_ids(project.id, &tx).await?;

            let room = if let Some(room_id) = project.room_id {
                Some(self.get_room(room_id, &tx).await?)
            } else {
                None
            };

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
            .exec(tx)
            .await?;
        }

        worktree::Entity::delete_many()
            .filter(worktree::Column::ProjectId.eq(project_id).and(
                worktree::Column::Id.is_not_in(worktrees.iter().map(|worktree| worktree.id as i64)),
            ))
            .exec(tx)
            .await?;

        Ok(())
    }

    pub async fn update_worktree(
        &self,
        update: &proto::UpdateWorktree,
        connection: ConnectionId,
    ) -> Result<TransactionGuard<Vec<ConnectionId>>> {
        if update.removed_entries.len() > proto::MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE
            || update.updated_entries.len() > proto::MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE
        {
            return Err(anyhow!(
                "invalid worktree update. removed entries: {}, updated entries: {}",
                update.removed_entries.len(),
                update.updated_entries.len()
            ))?;
        }

        let project_id = ProjectId::from_proto(update.project_id);
        let worktree_id = update.worktree_id as i64;
        self.project_transaction(project_id, |tx| async move {
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
                .with_context(|| format!("no such project: {project_id}"))?;

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
                        canonical_path: ActiveValue::set(entry.canonical_path.clone()),
                        is_ignored: ActiveValue::set(entry.is_ignored),
                        git_status: ActiveValue::set(None),
                        is_external: ActiveValue::set(entry.is_external),
                        is_deleted: ActiveValue::set(false),
                        is_hidden: ActiveValue::set(entry.is_hidden),
                        scan_id: ActiveValue::set(update.scan_id as i64),
                        is_fifo: ActiveValue::set(entry.is_fifo),
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
                        worktree_entry::Column::CanonicalPath,
                        worktree_entry::Column::IsIgnored,
                        worktree_entry::Column::IsHidden,
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

            // Backward-compatibility for old Zed clients.
            //
            // Remove this block when Zed 1.80 stable has been out for a week.
            {
                if !update.updated_repositories.is_empty() {
                    project_repository::Entity::insert_many(
                        update.updated_repositories.iter().map(|repository| {
                            project_repository::ActiveModel {
                                project_id: ActiveValue::set(project_id),
                                legacy_worktree_id: ActiveValue::set(Some(worktree_id)),
                                id: ActiveValue::set(repository.repository_id as i64),
                                scan_id: ActiveValue::set(update.scan_id as i64),
                                is_deleted: ActiveValue::set(false),
                                branch_summary: ActiveValue::Set(
                                    repository
                                        .branch_summary
                                        .as_ref()
                                        .map(|summary| serde_json::to_string(summary).unwrap()),
                                ),
                                current_merge_conflicts: ActiveValue::Set(Some(
                                    serde_json::to_string(&repository.current_merge_conflicts)
                                        .unwrap(),
                                )),
                                // Old clients do not use abs path, entry ids, head_commit_details, or merge_message.
                                abs_path: ActiveValue::set(String::new()),
                                entry_ids: ActiveValue::set("[]".into()),
                                head_commit_details: ActiveValue::set(None),
                                merge_message: ActiveValue::set(None),
                            }
                        }),
                    )
                    .on_conflict(
                        OnConflict::columns([
                            project_repository::Column::ProjectId,
                            project_repository::Column::Id,
                        ])
                        .update_columns([
                            project_repository::Column::ScanId,
                            project_repository::Column::BranchSummary,
                            project_repository::Column::CurrentMergeConflicts,
                        ])
                        .to_owned(),
                    )
                    .exec(&*tx)
                    .await?;

                    let has_any_statuses = update
                        .updated_repositories
                        .iter()
                        .any(|repository| !repository.updated_statuses.is_empty());

                    if has_any_statuses {
                        project_repository_statuses::Entity::insert_many(
                            update.updated_repositories.iter().flat_map(
                                |repository: &proto::RepositoryEntry| {
                                    repository.updated_statuses.iter().map(|status_entry| {
                                        let (repo_path, status_kind, first_status, second_status) =
                                            proto_status_to_db(status_entry.clone());
                                        project_repository_statuses::ActiveModel {
                                            project_id: ActiveValue::set(project_id),
                                            repository_id: ActiveValue::set(
                                                repository.repository_id as i64,
                                            ),
                                            scan_id: ActiveValue::set(update.scan_id as i64),
                                            is_deleted: ActiveValue::set(false),
                                            repo_path: ActiveValue::set(repo_path),
                                            status: ActiveValue::set(0),
                                            status_kind: ActiveValue::set(status_kind),
                                            first_status: ActiveValue::set(first_status),
                                            second_status: ActiveValue::set(second_status),
                                        }
                                    })
                                },
                            ),
                        )
                        .on_conflict(
                            OnConflict::columns([
                                project_repository_statuses::Column::ProjectId,
                                project_repository_statuses::Column::RepositoryId,
                                project_repository_statuses::Column::RepoPath,
                            ])
                            .update_columns([
                                project_repository_statuses::Column::ScanId,
                                project_repository_statuses::Column::StatusKind,
                                project_repository_statuses::Column::FirstStatus,
                                project_repository_statuses::Column::SecondStatus,
                            ])
                            .to_owned(),
                        )
                        .exec(&*tx)
                        .await?;
                    }

                    for repo in &update.updated_repositories {
                        if !repo.removed_statuses.is_empty() {
                            project_repository_statuses::Entity::update_many()
                                .filter(
                                    project_repository_statuses::Column::ProjectId
                                        .eq(project_id)
                                        .and(
                                            project_repository_statuses::Column::RepositoryId
                                                .eq(repo.repository_id),
                                        )
                                        .and(
                                            project_repository_statuses::Column::RepoPath
                                                .is_in(repo.removed_statuses.iter()),
                                        ),
                                )
                                .set(project_repository_statuses::ActiveModel {
                                    is_deleted: ActiveValue::Set(true),
                                    scan_id: ActiveValue::Set(update.scan_id as i64),
                                    ..Default::default()
                                })
                                .exec(&*tx)
                                .await?;
                        }
                    }
                }

                if !update.removed_repositories.is_empty() {
                    project_repository::Entity::update_many()
                        .filter(
                            project_repository::Column::ProjectId
                                .eq(project_id)
                                .and(project_repository::Column::LegacyWorktreeId.eq(worktree_id))
                                .and(project_repository::Column::Id.is_in(
                                    update.removed_repositories.iter().map(|id| *id as i64),
                                )),
                        )
                        .set(project_repository::ActiveModel {
                            is_deleted: ActiveValue::Set(true),
                            scan_id: ActiveValue::Set(update.scan_id as i64),
                            ..Default::default()
                        })
                        .exec(&*tx)
                        .await?;
                }
            }

            let connection_ids = self.project_guest_connection_ids(project_id, &tx).await?;
            Ok(connection_ids)
        })
        .await
    }

    pub async fn update_repository(
        &self,
        update: &proto::UpdateRepository,
        _connection: ConnectionId,
    ) -> Result<TransactionGuard<Vec<ConnectionId>>> {
        let project_id = ProjectId::from_proto(update.project_id);
        let repository_id = update.id as i64;
        self.project_transaction(project_id, |tx| async move {
            project_repository::Entity::insert(project_repository::ActiveModel {
                project_id: ActiveValue::set(project_id),
                id: ActiveValue::set(repository_id),
                legacy_worktree_id: ActiveValue::set(None),
                abs_path: ActiveValue::set(update.abs_path.clone()),
                entry_ids: ActiveValue::Set(serde_json::to_string(&update.entry_ids).unwrap()),
                scan_id: ActiveValue::set(update.scan_id as i64),
                is_deleted: ActiveValue::set(false),
                branch_summary: ActiveValue::Set(
                    update
                        .branch_summary
                        .as_ref()
                        .map(|summary| serde_json::to_string(summary).unwrap()),
                ),
                head_commit_details: ActiveValue::Set(
                    update
                        .head_commit_details
                        .as_ref()
                        .map(|details| serde_json::to_string(details).unwrap()),
                ),
                current_merge_conflicts: ActiveValue::Set(Some(
                    serde_json::to_string(&update.current_merge_conflicts).unwrap(),
                )),
                merge_message: ActiveValue::set(update.merge_message.clone()),
            })
            .on_conflict(
                OnConflict::columns([
                    project_repository::Column::ProjectId,
                    project_repository::Column::Id,
                ])
                .update_columns([
                    project_repository::Column::ScanId,
                    project_repository::Column::BranchSummary,
                    project_repository::Column::EntryIds,
                    project_repository::Column::AbsPath,
                    project_repository::Column::CurrentMergeConflicts,
                    project_repository::Column::HeadCommitDetails,
                    project_repository::Column::MergeMessage,
                ])
                .to_owned(),
            )
            .exec(&*tx)
            .await?;

            let has_any_statuses = !update.updated_statuses.is_empty();

            if has_any_statuses {
                project_repository_statuses::Entity::insert_many(
                    update.updated_statuses.iter().map(|status_entry| {
                        let (repo_path, status_kind, first_status, second_status) =
                            proto_status_to_db(status_entry.clone());
                        project_repository_statuses::ActiveModel {
                            project_id: ActiveValue::set(project_id),
                            repository_id: ActiveValue::set(repository_id),
                            scan_id: ActiveValue::set(update.scan_id as i64),
                            is_deleted: ActiveValue::set(false),
                            repo_path: ActiveValue::set(repo_path),
                            status: ActiveValue::set(0),
                            status_kind: ActiveValue::set(status_kind),
                            first_status: ActiveValue::set(first_status),
                            second_status: ActiveValue::set(second_status),
                        }
                    }),
                )
                .on_conflict(
                    OnConflict::columns([
                        project_repository_statuses::Column::ProjectId,
                        project_repository_statuses::Column::RepositoryId,
                        project_repository_statuses::Column::RepoPath,
                    ])
                    .update_columns([
                        project_repository_statuses::Column::ScanId,
                        project_repository_statuses::Column::StatusKind,
                        project_repository_statuses::Column::FirstStatus,
                        project_repository_statuses::Column::SecondStatus,
                    ])
                    .to_owned(),
                )
                .exec(&*tx)
                .await?;
            }

            let has_any_removed_statuses = !update.removed_statuses.is_empty();

            if has_any_removed_statuses {
                project_repository_statuses::Entity::update_many()
                    .filter(
                        project_repository_statuses::Column::ProjectId
                            .eq(project_id)
                            .and(
                                project_repository_statuses::Column::RepositoryId.eq(repository_id),
                            )
                            .and(
                                project_repository_statuses::Column::RepoPath
                                    .is_in(update.removed_statuses.iter()),
                            ),
                    )
                    .set(project_repository_statuses::ActiveModel {
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

    pub async fn remove_repository(
        &self,
        remove: &proto::RemoveRepository,
        _connection: ConnectionId,
    ) -> Result<TransactionGuard<Vec<ConnectionId>>> {
        let project_id = ProjectId::from_proto(remove.project_id);
        let repository_id = remove.id as i64;
        self.project_transaction(project_id, |tx| async move {
            project_repository::Entity::update_many()
                .filter(
                    project_repository::Column::ProjectId
                        .eq(project_id)
                        .and(project_repository::Column::Id.eq(repository_id)),
                )
                .set(project_repository::ActiveModel {
                    is_deleted: ActiveValue::Set(true),
                    // scan_id: ActiveValue::Set(update.scan_id as i64),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;

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
    ) -> Result<TransactionGuard<Vec<ConnectionId>>> {
        let project_id = ProjectId::from_proto(update.project_id);
        let worktree_id = update.worktree_id as i64;
        self.project_transaction(project_id, |tx| async move {
            let summary = update.summary.as_ref().context("invalid summary")?;

            // Ensure the update comes from the host.
            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .context("no such project")?;
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
    ) -> Result<TransactionGuard<Vec<ConnectionId>>> {
        let project_id = ProjectId::from_proto(update.project_id);
        self.project_transaction(project_id, |tx| async move {
            let server = update.server.as_ref().context("invalid language server")?;

            // Ensure the update comes from the host.
            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .context("no such project")?;
            if project.host_connection()? != connection {
                return Err(anyhow!("can't update a project hosted by someone else"))?;
            }

            // Add the newly-started language server.
            language_server::Entity::insert(language_server::ActiveModel {
                project_id: ActiveValue::set(project_id),
                id: ActiveValue::set(server.id as i64),
                name: ActiveValue::set(server.name.clone()),
                worktree_id: ActiveValue::set(server.worktree_id.map(|id| id as i64)),
                capabilities: ActiveValue::set(update.capabilities.clone()),
            })
            .on_conflict(
                OnConflict::columns([
                    language_server::Column::ProjectId,
                    language_server::Column::Id,
                ])
                .update_columns([
                    language_server::Column::Name,
                    language_server::Column::Capabilities,
                    language_server::Column::WorktreeId,
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

    /// Updates the worktree settings for the given connection.
    pub async fn update_worktree_settings(
        &self,
        update: &proto::UpdateWorktreeSettings,
        connection: ConnectionId,
    ) -> Result<TransactionGuard<Vec<ConnectionId>>> {
        let project_id = ProjectId::from_proto(update.project_id);
        let kind = match update.kind {
            Some(kind) => proto::LocalSettingsKind::from_i32(kind)
                .with_context(|| format!("unknown worktree settings kind: {kind}"))?,
            None => proto::LocalSettingsKind::Settings,
        };
        let kind = LocalSettingsKind::from_proto(kind);
        self.project_transaction(project_id, |tx| async move {
            // Ensure the update comes from the host.
            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .context("no such project")?;
            if project.host_connection()? != connection {
                return Err(anyhow!("can't update a project hosted by someone else"))?;
            }

            if let Some(content) = &update.content {
                worktree_settings_file::Entity::insert(worktree_settings_file::ActiveModel {
                    project_id: ActiveValue::Set(project_id),
                    worktree_id: ActiveValue::Set(update.worktree_id as i64),
                    path: ActiveValue::Set(update.path.clone()),
                    content: ActiveValue::Set(content.clone()),
                    kind: ActiveValue::Set(kind),
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

    pub async fn get_project(&self, id: ProjectId) -> Result<project::Model> {
        self.transaction(|tx| async move {
            Ok(project::Entity::find_by_id(id)
                .one(&*tx)
                .await?
                .context("no such project")?)
        })
        .await
    }

    /// Adds the given connection to the specified project
    /// in the current room.
    pub async fn join_project(
        &self,
        project_id: ProjectId,
        connection: ConnectionId,
        user_id: UserId,
        committer_name: Option<String>,
        committer_email: Option<String>,
    ) -> Result<TransactionGuard<(Project, ReplicaId)>> {
        self.project_transaction(project_id, move |tx| {
            let committer_name = committer_name.clone();
            let committer_email = committer_email.clone();
            async move {
                let (project, role) = self
                    .access_project(project_id, connection, Capability::ReadOnly, &tx)
                    .await?;
                self.join_project_internal(
                    project,
                    user_id,
                    committer_name,
                    committer_email,
                    connection,
                    role,
                    &tx,
                )
                .await
            }
        })
        .await
    }

    async fn join_project_internal(
        &self,
        project: project::Model,
        user_id: UserId,
        committer_name: Option<String>,
        committer_email: Option<String>,
        connection: ConnectionId,
        role: ChannelRole,
        tx: &DatabaseTransaction,
    ) -> Result<(Project, ReplicaId)> {
        let mut collaborators = project
            .find_related(project_collaborator::Entity)
            .all(tx)
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
            id: ActiveValue::NotSet,
            committer_name: ActiveValue::set(committer_name),
            committer_email: ActiveValue::set(committer_email),
        }
        .insert(tx)
        .await?;
        collaborators.push(new_collaborator);

        let db_worktrees = project.find_related(worktree::Entity).all(tx).await?;
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
                        diagnostic_summaries: Default::default(),
                        settings_files: Default::default(),
                        scan_id: db_worktree.scan_id as u64,
                        completed_scan_id: db_worktree.completed_scan_id as u64,
                        legacy_repository_entries: Default::default(),
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
                .stream(tx)
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
                        canonical_path: db_entry.canonical_path,
                        is_ignored: db_entry.is_ignored,
                        is_external: db_entry.is_external,
                        is_hidden: db_entry.is_hidden,
                        // This is only used in the summarization backlog, so if it's None,
                        // that just means we won't be able to detect when to resummarize
                        // based on total number of backlogged bytes - instead, we'd go
                        // on number of files only. That shouldn't be a huge deal in practice.
                        size: None,
                        is_fifo: db_entry.is_fifo,
                    });
                }
            }
        }

        // Populate repository entries.
        let mut repositories = Vec::new();
        {
            let db_repository_entries = project_repository::Entity::find()
                .filter(
                    Condition::all()
                        .add(project_repository::Column::ProjectId.eq(project.id))
                        .add(project_repository::Column::IsDeleted.eq(false)),
                )
                .all(tx)
                .await?;
            for db_repository_entry in db_repository_entries {
                let mut repository_statuses = project_repository_statuses::Entity::find()
                    .filter(
                        Condition::all()
                            .add(project_repository_statuses::Column::ProjectId.eq(project.id))
                            .add(
                                project_repository_statuses::Column::RepositoryId
                                    .eq(db_repository_entry.id),
                            )
                            .add(project_repository_statuses::Column::IsDeleted.eq(false)),
                    )
                    .stream(tx)
                    .await?;
                let mut updated_statuses = Vec::new();
                while let Some(status_entry) = repository_statuses.next().await {
                    let status_entry = status_entry?;
                    updated_statuses.push(db_status_to_proto(status_entry)?);
                }

                let current_merge_conflicts = db_repository_entry
                    .current_merge_conflicts
                    .as_ref()
                    .map(|conflicts| serde_json::from_str(conflicts))
                    .transpose()?
                    .unwrap_or_default();

                let branch_summary = db_repository_entry
                    .branch_summary
                    .as_ref()
                    .map(|branch_summary| serde_json::from_str(branch_summary))
                    .transpose()?
                    .unwrap_or_default();

                let head_commit_details = db_repository_entry
                    .head_commit_details
                    .as_ref()
                    .map(|head_commit_details| serde_json::from_str(head_commit_details))
                    .transpose()?
                    .unwrap_or_default();

                let entry_ids = serde_json::from_str(&db_repository_entry.entry_ids)
                    .context("failed to deserialize repository's entry ids")?;

                if let Some(worktree_id) = db_repository_entry.legacy_worktree_id {
                    if let Some(worktree) = worktrees.get_mut(&(worktree_id as u64)) {
                        worktree.legacy_repository_entries.insert(
                            db_repository_entry.id as u64,
                            proto::RepositoryEntry {
                                repository_id: db_repository_entry.id as u64,
                                updated_statuses,
                                removed_statuses: Vec::new(),
                                current_merge_conflicts,
                                branch_summary,
                            },
                        );
                    }
                } else {
                    repositories.push(proto::UpdateRepository {
                        project_id: db_repository_entry.project_id.0 as u64,
                        id: db_repository_entry.id as u64,
                        abs_path: db_repository_entry.abs_path,
                        entry_ids,
                        updated_statuses,
                        removed_statuses: Vec::new(),
                        current_merge_conflicts,
                        branch_summary,
                        head_commit_details,
                        scan_id: db_repository_entry.scan_id as u64,
                        is_last_update: true,
                        merge_message: db_repository_entry.merge_message,
                        stash_entries: Vec::new(),
                    });
                }
            }
        }

        // Populate worktree diagnostic summaries.
        {
            let mut db_summaries = worktree_diagnostic_summary::Entity::find()
                .filter(worktree_diagnostic_summary::Column::ProjectId.eq(project.id))
                .stream(tx)
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
                .stream(tx)
                .await?;
            while let Some(db_settings_file) = db_settings_files.next().await {
                let db_settings_file = db_settings_file?;
                if let Some(worktree) = worktrees.get_mut(&(db_settings_file.worktree_id as u64)) {
                    worktree.settings_files.push(WorktreeSettingsFile {
                        path: db_settings_file.path,
                        content: db_settings_file.content,
                        kind: db_settings_file.kind,
                    });
                }
            }
        }

        // Populate language servers.
        let language_servers = project
            .find_related(language_server::Entity)
            .all(tx)
            .await?;

        let path_style = if project.windows_paths {
            PathStyle::Windows
        } else {
            PathStyle::Posix
        };

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
                    committer_name: collaborator.committer_name,
                    committer_email: collaborator.committer_email,
                })
                .collect(),
            worktrees,
            repositories,
            language_servers: language_servers
                .into_iter()
                .map(|language_server| LanguageServer {
                    server: proto::LanguageServer {
                        id: language_server.id as u64,
                        name: language_server.name,
                        worktree_id: language_server.worktree_id.map(|id| id as u64),
                    },
                    capabilities: language_server.capabilities,
                })
                .collect(),
            path_style,
        };
        Ok((project, replica_id as ReplicaId))
    }

    /// Removes the given connection from the specified project.
    pub async fn leave_project(
        &self,
        project_id: ProjectId,
        connection: ConnectionId,
    ) -> Result<TransactionGuard<(Option<proto::Room>, LeftProject)>> {
        self.project_transaction(project_id, |tx| async move {
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
                .context("no such project")?;
            let collaborators = project
                .find_related(project_collaborator::Entity)
                .all(&*tx)
                .await?;
            let connection_ids: Vec<ConnectionId> = collaborators
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

            let room = if let Some(room_id) = project.room_id {
                Some(self.get_room(room_id, &tx).await?)
            } else {
                None
            };

            let left_project = LeftProject {
                id: project_id,
                should_unshare: connection == project.host_connection()?,
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
        self.project_transaction(project_id, |tx| async move {
            project::Entity::find()
                .filter(
                    Condition::all()
                        .add(project::Column::Id.eq(project_id))
                        .add(project::Column::HostConnectionId.eq(Some(connection_id.id as i32)))
                        .add(
                            project::Column::HostConnectionServerId
                                .eq(Some(connection_id.owner_id as i32)),
                        ),
                )
                .one(&*tx)
                .await?
                .context("failed to read project host")?;

            Ok(())
        })
        .await
        .map(|guard| guard.into_inner())
    }

    /// Returns the current project if the given user is authorized to access it with the specified capability.
    pub async fn access_project(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
        capability: Capability,
        tx: &DatabaseTransaction,
    ) -> Result<(project::Model, ChannelRole)> {
        let project = project::Entity::find_by_id(project_id)
            .one(tx)
            .await?
            .context("no such project")?;

        let role_from_room = if let Some(room_id) = project.room_id {
            room_participant::Entity::find()
                .filter(room_participant::Column::RoomId.eq(room_id))
                .filter(room_participant::Column::AnsweringConnectionId.eq(connection_id.id))
                .one(tx)
                .await?
                .and_then(|participant| participant.role)
        } else {
            None
        };

        let role = role_from_room.unwrap_or(ChannelRole::Banned);

        match capability {
            Capability::ReadWrite => {
                if !role.can_edit_projects() {
                    return Err(anyhow!("not authorized to edit projects"))?;
                }
            }
            Capability::ReadOnly => {
                if !role.can_read_projects() {
                    return Err(anyhow!("not authorized to read projects"))?;
                }
            }
        }

        Ok((project, role))
    }

    /// Returns the host connection for a read-only request to join a shared project.
    pub async fn host_for_read_only_project_request(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<ConnectionId> {
        self.project_transaction(project_id, |tx| async move {
            let (project, _) = self
                .access_project(project_id, connection_id, Capability::ReadOnly, &tx)
                .await?;
            project.host_connection()
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
        self.project_transaction(project_id, |tx| async move {
            let (project, _) = self
                .access_project(project_id, connection_id, Capability::ReadWrite, &tx)
                .await?;
            project.host_connection()
        })
        .await
        .map(|guard| guard.into_inner())
    }

    pub async fn connections_for_buffer_update(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
        capability: Capability,
    ) -> Result<TransactionGuard<(ConnectionId, Vec<ConnectionId>)>> {
        self.project_transaction(project_id, |tx| async move {
            // Authorize
            let (project, _) = self
                .access_project(project_id, connection_id, capability, &tx)
                .await?;

            let host_connection_id = project.host_connection()?;

            let collaborators = project_collaborator::Entity::find()
                .filter(project_collaborator::Column::ProjectId.eq(project_id))
                .all(&*tx)
                .await?;

            let guest_connection_ids = collaborators
                .into_iter()
                .filter_map(|collaborator| {
                    if collaborator.is_host {
                        None
                    } else {
                        Some(collaborator.connection())
                    }
                })
                .collect();

            Ok((host_connection_id, guest_connection_ids))
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
        exclude_dev_server: bool,
    ) -> Result<TransactionGuard<HashSet<ConnectionId>>> {
        self.project_transaction(project_id, |tx| async move {
            self.internal_project_connection_ids(project_id, connection_id, exclude_dev_server, &tx)
                .await
        })
        .await
    }

    async fn internal_project_connection_ids(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
        exclude_dev_server: bool,
        tx: &DatabaseTransaction,
    ) -> Result<HashSet<ConnectionId>> {
        let project = project::Entity::find_by_id(project_id)
            .one(tx)
            .await?
            .context("no such project")?;

        let mut collaborators = project_collaborator::Entity::find()
            .filter(project_collaborator::Column::ProjectId.eq(project_id))
            .stream(tx)
            .await?;

        let mut connection_ids = HashSet::default();
        if let Some(host_connection) = project.host_connection().log_err()
            && !exclude_dev_server
        {
            connection_ids.insert(host_connection);
        }

        while let Some(collaborator) = collaborators.next().await {
            let collaborator = collaborator?;
            connection_ids.insert(collaborator.connection());
        }

        if connection_ids.contains(&connection_id)
            || Some(connection_id) == project.host_connection().ok()
        {
            Ok(connection_ids)
        } else {
            Err(anyhow!(
                "can only send project updates to a project you're in"
            ))?
        }
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
    pub async fn room_id_for_project(&self, project_id: ProjectId) -> Result<Option<RoomId>> {
        self.transaction(|tx| async move {
            Ok(project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .and_then(|project| project.room_id))
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
    ) -> Result<TransactionGuard<proto::Room>> {
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
    ) -> Result<TransactionGuard<proto::Room>> {
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
