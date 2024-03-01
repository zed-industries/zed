use super::*;

impl Database {
    /// Clears all room participants in rooms attached to a stale server.
    pub async fn clear_stale_room_participants(
        &self,
        room_id: RoomId,
        new_server_id: ServerId,
    ) -> Result<RoomGuard<RefreshedRoom>> {
        self.room_transaction(room_id, |tx| async move {
            let stale_participant_filter = Condition::all()
                .add(room_participant::Column::RoomId.eq(room_id))
                .add(room_participant::Column::AnsweringConnectionId.is_not_null())
                .add(room_participant::Column::AnsweringConnectionServerId.ne(new_server_id));

            let stale_participant_user_ids = room_participant::Entity::find()
                .filter(stale_participant_filter.clone())
                .all(&*tx)
                .await?
                .into_iter()
                .map(|participant| participant.user_id)
                .collect::<Vec<_>>();

            // Delete participants who failed to reconnect and cancel their calls.
            let mut canceled_calls_to_user_ids = Vec::new();
            room_participant::Entity::delete_many()
                .filter(stale_participant_filter)
                .exec(&*tx)
                .await?;
            let called_participants = room_participant::Entity::find()
                .filter(
                    Condition::all()
                        .add(
                            room_participant::Column::CallingUserId
                                .is_in(stale_participant_user_ids.iter().copied()),
                        )
                        .add(room_participant::Column::AnsweringConnectionId.is_null()),
                )
                .all(&*tx)
                .await?;
            room_participant::Entity::delete_many()
                .filter(
                    room_participant::Column::Id
                        .is_in(called_participants.iter().map(|participant| participant.id)),
                )
                .exec(&*tx)
                .await?;
            canceled_calls_to_user_ids.extend(
                called_participants
                    .into_iter()
                    .map(|participant| participant.user_id),
            );

            let (channel, room) = self.get_channel_room(room_id, &tx).await?;
            let channel_members;
            if let Some(channel) = &channel {
                channel_members = self.get_channel_participants(channel, &tx).await?;
            } else {
                channel_members = Vec::new();

                // Delete the room if it becomes empty.
                if room.participants.is_empty() {
                    project::Entity::delete_many()
                        .filter(project::Column::RoomId.eq(room_id))
                        .exec(&*tx)
                        .await?;
                    room::Entity::delete_by_id(room_id).exec(&*tx).await?;
                }
            };

            Ok(RefreshedRoom {
                room,
                channel_id: channel.map(|channel| channel.id),
                channel_members,
                stale_participant_user_ids,
                canceled_calls_to_user_ids,
            })
        })
        .await
    }

    /// Returns the incoming calls for user with the given ID.
    pub async fn incoming_call_for_user(
        &self,
        user_id: UserId,
    ) -> Result<Option<proto::IncomingCall>> {
        self.transaction(|tx| async move {
            let pending_participant = room_participant::Entity::find()
                .filter(
                    room_participant::Column::UserId
                        .eq(user_id)
                        .and(room_participant::Column::AnsweringConnectionId.is_null()),
                )
                .one(&*tx)
                .await?;

            if let Some(pending_participant) = pending_participant {
                let room = self.get_room(pending_participant.room_id, &tx).await?;
                Ok(Self::build_incoming_call(&room, user_id))
            } else {
                Ok(None)
            }
        })
        .await
    }

    /// Creates a new room.
    pub async fn create_room(
        &self,
        user_id: UserId,
        connection: ConnectionId,
        live_kit_room: &str,
    ) -> Result<proto::Room> {
        self.transaction(|tx| async move {
            let room = room::ActiveModel {
                live_kit_room: ActiveValue::set(live_kit_room.into()),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;
            room_participant::ActiveModel {
                room_id: ActiveValue::set(room.id),
                user_id: ActiveValue::set(user_id),
                answering_connection_id: ActiveValue::set(Some(connection.id as i32)),
                answering_connection_server_id: ActiveValue::set(Some(ServerId(
                    connection.owner_id as i32,
                ))),
                answering_connection_lost: ActiveValue::set(false),
                calling_user_id: ActiveValue::set(user_id),
                calling_connection_id: ActiveValue::set(connection.id as i32),
                calling_connection_server_id: ActiveValue::set(Some(ServerId(
                    connection.owner_id as i32,
                ))),
                participant_index: ActiveValue::set(Some(0)),
                role: ActiveValue::set(Some(ChannelRole::Admin)),

                id: ActiveValue::NotSet,
                location_kind: ActiveValue::NotSet,
                location_project_id: ActiveValue::NotSet,
                initial_project_id: ActiveValue::NotSet,
            }
            .insert(&*tx)
            .await?;

            let room = self.get_room(room.id, &tx).await?;
            Ok(room)
        })
        .await
    }

    pub async fn call(
        &self,
        room_id: RoomId,
        calling_user_id: UserId,
        calling_connection: ConnectionId,
        called_user_id: UserId,
        initial_project_id: Option<ProjectId>,
    ) -> Result<RoomGuard<(proto::Room, proto::IncomingCall)>> {
        self.room_transaction(room_id, |tx| async move {
            let caller = room_participant::Entity::find()
                .filter(
                    room_participant::Column::UserId
                        .eq(calling_user_id)
                        .and(room_participant::Column::RoomId.eq(room_id)),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("user is not in the room"))?;

            let called_user_role = match caller.role.unwrap_or(ChannelRole::Member) {
                ChannelRole::Admin | ChannelRole::Member => ChannelRole::Member,
                ChannelRole::Guest | ChannelRole::Talker => ChannelRole::Guest,
                ChannelRole::Banned => return Err(anyhow!("banned users cannot invite").into()),
            };

            room_participant::ActiveModel {
                room_id: ActiveValue::set(room_id),
                user_id: ActiveValue::set(called_user_id),
                answering_connection_lost: ActiveValue::set(false),
                participant_index: ActiveValue::NotSet,
                calling_user_id: ActiveValue::set(calling_user_id),
                calling_connection_id: ActiveValue::set(calling_connection.id as i32),
                calling_connection_server_id: ActiveValue::set(Some(ServerId(
                    calling_connection.owner_id as i32,
                ))),
                initial_project_id: ActiveValue::set(initial_project_id),
                role: ActiveValue::set(Some(called_user_role)),

                id: ActiveValue::NotSet,
                answering_connection_id: ActiveValue::NotSet,
                answering_connection_server_id: ActiveValue::NotSet,
                location_kind: ActiveValue::NotSet,
                location_project_id: ActiveValue::NotSet,
            }
            .insert(&*tx)
            .await?;

            let room = self.get_room(room_id, &tx).await?;
            let incoming_call = Self::build_incoming_call(&room, called_user_id)
                .ok_or_else(|| anyhow!("failed to build incoming call"))?;
            Ok((room, incoming_call))
        })
        .await
    }

    pub async fn call_failed(
        &self,
        room_id: RoomId,
        called_user_id: UserId,
    ) -> Result<RoomGuard<proto::Room>> {
        self.room_transaction(room_id, |tx| async move {
            room_participant::Entity::delete_many()
                .filter(
                    room_participant::Column::RoomId
                        .eq(room_id)
                        .and(room_participant::Column::UserId.eq(called_user_id)),
                )
                .exec(&*tx)
                .await?;
            let room = self.get_room(room_id, &tx).await?;
            Ok(room)
        })
        .await
    }

    pub async fn decline_call(
        &self,
        expected_room_id: Option<RoomId>,
        user_id: UserId,
    ) -> Result<Option<RoomGuard<proto::Room>>> {
        self.optional_room_transaction(|tx| async move {
            let mut filter = Condition::all()
                .add(room_participant::Column::UserId.eq(user_id))
                .add(room_participant::Column::AnsweringConnectionId.is_null());
            if let Some(room_id) = expected_room_id {
                filter = filter.add(room_participant::Column::RoomId.eq(room_id));
            }
            let participant = room_participant::Entity::find()
                .filter(filter)
                .one(&*tx)
                .await?;

            let participant = if let Some(participant) = participant {
                participant
            } else if expected_room_id.is_some() {
                return Err(anyhow!("could not find call to decline"))?;
            } else {
                return Ok(None);
            };

            let room_id = participant.room_id;
            room_participant::Entity::delete(participant.into_active_model())
                .exec(&*tx)
                .await?;

            let room = self.get_room(room_id, &tx).await?;
            Ok(Some((room_id, room)))
        })
        .await
    }

    pub async fn cancel_call(
        &self,
        room_id: RoomId,
        calling_connection: ConnectionId,
        called_user_id: UserId,
    ) -> Result<RoomGuard<proto::Room>> {
        self.room_transaction(room_id, |tx| async move {
            let participant = room_participant::Entity::find()
                .filter(
                    Condition::all()
                        .add(room_participant::Column::UserId.eq(called_user_id))
                        .add(room_participant::Column::RoomId.eq(room_id))
                        .add(
                            room_participant::Column::CallingConnectionId
                                .eq(calling_connection.id as i32),
                        )
                        .add(
                            room_participant::Column::CallingConnectionServerId
                                .eq(calling_connection.owner_id as i32),
                        )
                        .add(room_participant::Column::AnsweringConnectionId.is_null()),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no call to cancel"))?;

            room_participant::Entity::delete(participant.into_active_model())
                .exec(&*tx)
                .await?;

            let room = self.get_room(room_id, &tx).await?;
            Ok(room)
        })
        .await
    }

    pub async fn join_room(
        &self,
        room_id: RoomId,
        user_id: UserId,
        connection: ConnectionId,
    ) -> Result<RoomGuard<JoinRoom>> {
        self.room_transaction(room_id, |tx| async move {
            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            enum QueryChannelId {
                ChannelId,
            }

            let channel_id: Option<ChannelId> = room::Entity::find()
                .select_only()
                .column(room::Column::ChannelId)
                .filter(room::Column::Id.eq(room_id))
                .into_values::<_, QueryChannelId>()
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such room"))?;

            if channel_id.is_some() {
                Err(anyhow!("tried to join channel call directly"))?
            }

            let participant_index = self
                .get_next_participant_index_internal(room_id, &*tx)
                .await?;

            let result = room_participant::Entity::update_many()
                .filter(
                    Condition::all()
                        .add(room_participant::Column::RoomId.eq(room_id))
                        .add(room_participant::Column::UserId.eq(user_id))
                        .add(room_participant::Column::AnsweringConnectionId.is_null()),
                )
                .set(room_participant::ActiveModel {
                    participant_index: ActiveValue::Set(Some(participant_index)),
                    answering_connection_id: ActiveValue::set(Some(connection.id as i32)),
                    answering_connection_server_id: ActiveValue::set(Some(ServerId(
                        connection.owner_id as i32,
                    ))),
                    answering_connection_lost: ActiveValue::set(false),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            if result.rows_affected == 0 {
                Err(anyhow!("room does not exist or was already joined"))?;
            }

            let room = self.get_room(room_id, &tx).await?;
            Ok(JoinRoom {
                room,
                channel_id: None,
                channel_members: vec![],
            })
        })
        .await
    }

    async fn get_next_participant_index_internal(
        &self,
        room_id: RoomId,
        tx: &DatabaseTransaction,
    ) -> Result<i32> {
        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryParticipantIndices {
            ParticipantIndex,
        }
        let existing_participant_indices: Vec<i32> = room_participant::Entity::find()
            .filter(
                room_participant::Column::RoomId
                    .eq(room_id)
                    .and(room_participant::Column::ParticipantIndex.is_not_null()),
            )
            .select_only()
            .column(room_participant::Column::ParticipantIndex)
            .into_values::<_, QueryParticipantIndices>()
            .all(&*tx)
            .await?;

        let mut participant_index = 0;
        while existing_participant_indices.contains(&participant_index) {
            participant_index += 1;
        }

        Ok(participant_index)
    }

    /// Returns the channel ID for the given room, if it has one.
    pub async fn channel_id_for_room(&self, room_id: RoomId) -> Result<Option<ChannelId>> {
        self.transaction(|tx| async move {
            let room: Option<room::Model> = room::Entity::find()
                .filter(room::Column::Id.eq(room_id))
                .one(&*tx)
                .await?;

            Ok(room.and_then(|room| room.channel_id))
        })
        .await
    }

    pub(crate) async fn join_channel_room_internal(
        &self,
        room_id: RoomId,
        user_id: UserId,
        connection: ConnectionId,
        role: ChannelRole,
        tx: &DatabaseTransaction,
    ) -> Result<JoinRoom> {
        let participant_index = self
            .get_next_participant_index_internal(room_id, &*tx)
            .await?;

        room_participant::Entity::insert_many([room_participant::ActiveModel {
            room_id: ActiveValue::set(room_id),
            user_id: ActiveValue::set(user_id),
            answering_connection_id: ActiveValue::set(Some(connection.id as i32)),
            answering_connection_server_id: ActiveValue::set(Some(ServerId(
                connection.owner_id as i32,
            ))),
            answering_connection_lost: ActiveValue::set(false),
            calling_user_id: ActiveValue::set(user_id),
            calling_connection_id: ActiveValue::set(connection.id as i32),
            calling_connection_server_id: ActiveValue::set(Some(ServerId(
                connection.owner_id as i32,
            ))),
            participant_index: ActiveValue::Set(Some(participant_index)),
            role: ActiveValue::set(Some(role)),
            id: ActiveValue::NotSet,
            location_kind: ActiveValue::NotSet,
            location_project_id: ActiveValue::NotSet,
            initial_project_id: ActiveValue::NotSet,
        }])
        .on_conflict(
            OnConflict::columns([room_participant::Column::UserId])
                .update_columns([
                    room_participant::Column::AnsweringConnectionId,
                    room_participant::Column::AnsweringConnectionServerId,
                    room_participant::Column::AnsweringConnectionLost,
                    room_participant::Column::ParticipantIndex,
                    room_participant::Column::Role,
                ])
                .to_owned(),
        )
        .exec(&*tx)
        .await?;

        let (channel, room) = self.get_channel_room(room_id, &tx).await?;
        let channel = channel.ok_or_else(|| anyhow!("no channel for room"))?;
        let channel_members = self.get_channel_participants(&channel, &*tx).await?;
        Ok(JoinRoom {
            room,
            channel_id: Some(channel.id),
            channel_members,
        })
    }

    pub async fn rejoin_room(
        &self,
        rejoin_room: proto::RejoinRoom,
        user_id: UserId,
        connection: ConnectionId,
    ) -> Result<RoomGuard<RejoinedRoom>> {
        let room_id = RoomId::from_proto(rejoin_room.id);
        self.room_transaction(room_id, |tx| async {
            let tx = tx;
            let participant_update = room_participant::Entity::update_many()
                .filter(
                    Condition::all()
                        .add(room_participant::Column::RoomId.eq(room_id))
                        .add(room_participant::Column::UserId.eq(user_id))
                        .add(room_participant::Column::AnsweringConnectionId.is_not_null()),
                )
                .set(room_participant::ActiveModel {
                    answering_connection_id: ActiveValue::set(Some(connection.id as i32)),
                    answering_connection_server_id: ActiveValue::set(Some(ServerId(
                        connection.owner_id as i32,
                    ))),
                    answering_connection_lost: ActiveValue::set(false),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            if participant_update.rows_affected == 0 {
                return Err(anyhow!("room does not exist or was already joined"))?;
            }

            let mut reshared_projects = Vec::new();
            for reshared_project in &rejoin_room.reshared_projects {
                let project_id = ProjectId::from_proto(reshared_project.project_id);
                let project = project::Entity::find_by_id(project_id)
                    .one(&*tx)
                    .await?
                    .ok_or_else(|| anyhow!("project does not exist"))?;
                if project.host_user_id != user_id {
                    return Err(anyhow!("no such project"))?;
                }

                let mut collaborators = project
                    .find_related(project_collaborator::Entity)
                    .all(&*tx)
                    .await?;
                let host_ix = collaborators
                    .iter()
                    .position(|collaborator| {
                        collaborator.user_id == user_id && collaborator.is_host
                    })
                    .ok_or_else(|| anyhow!("host not found among collaborators"))?;
                let host = collaborators.swap_remove(host_ix);
                let old_connection_id = host.connection();

                project::Entity::update(project::ActiveModel {
                    host_connection_id: ActiveValue::set(Some(connection.id as i32)),
                    host_connection_server_id: ActiveValue::set(Some(ServerId(
                        connection.owner_id as i32,
                    ))),
                    ..project.into_active_model()
                })
                .exec(&*tx)
                .await?;
                project_collaborator::Entity::update(project_collaborator::ActiveModel {
                    connection_id: ActiveValue::set(connection.id as i32),
                    connection_server_id: ActiveValue::set(ServerId(connection.owner_id as i32)),
                    ..host.into_active_model()
                })
                .exec(&*tx)
                .await?;

                self.update_project_worktrees(project_id, &reshared_project.worktrees, &tx)
                    .await?;

                reshared_projects.push(ResharedProject {
                    id: project_id,
                    old_connection_id,
                    collaborators: collaborators
                        .iter()
                        .map(|collaborator| ProjectCollaborator {
                            connection_id: collaborator.connection(),
                            user_id: collaborator.user_id,
                            replica_id: collaborator.replica_id,
                            is_host: collaborator.is_host,
                        })
                        .collect(),
                    worktrees: reshared_project.worktrees.clone(),
                });
            }

            project::Entity::delete_many()
                .filter(
                    Condition::all()
                        .add(project::Column::RoomId.eq(room_id))
                        .add(project::Column::HostUserId.eq(user_id))
                        .add(
                            project::Column::Id
                                .is_not_in(reshared_projects.iter().map(|project| project.id)),
                        ),
                )
                .exec(&*tx)
                .await?;

            let mut rejoined_projects = Vec::new();
            for rejoined_project in &rejoin_room.rejoined_projects {
                let project_id = ProjectId::from_proto(rejoined_project.id);
                let Some(project) = project::Entity::find_by_id(project_id).one(&*tx).await? else {
                    continue;
                };

                let mut worktrees = Vec::new();
                let db_worktrees = project.find_related(worktree::Entity).all(&*tx).await?;
                for db_worktree in db_worktrees {
                    let mut worktree = RejoinedWorktree {
                        id: db_worktree.id as u64,
                        abs_path: db_worktree.abs_path,
                        root_name: db_worktree.root_name,
                        visible: db_worktree.visible,
                        updated_entries: Default::default(),
                        removed_entries: Default::default(),
                        updated_repositories: Default::default(),
                        removed_repositories: Default::default(),
                        diagnostic_summaries: Default::default(),
                        settings_files: Default::default(),
                        scan_id: db_worktree.scan_id as u64,
                        completed_scan_id: db_worktree.completed_scan_id as u64,
                    };

                    let rejoined_worktree = rejoined_project
                        .worktrees
                        .iter()
                        .find(|worktree| worktree.id == db_worktree.id as u64);

                    // File entries
                    {
                        let entry_filter = if let Some(rejoined_worktree) = rejoined_worktree {
                            worktree_entry::Column::ScanId.gt(rejoined_worktree.scan_id)
                        } else {
                            worktree_entry::Column::IsDeleted.eq(false)
                        };

                        let mut db_entries = worktree_entry::Entity::find()
                            .filter(
                                Condition::all()
                                    .add(worktree_entry::Column::ProjectId.eq(project.id))
                                    .add(worktree_entry::Column::WorktreeId.eq(worktree.id))
                                    .add(entry_filter),
                            )
                            .stream(&*tx)
                            .await?;

                        while let Some(db_entry) = db_entries.next().await {
                            let db_entry = db_entry?;
                            if db_entry.is_deleted {
                                worktree.removed_entries.push(db_entry.id as u64);
                            } else {
                                worktree.updated_entries.push(proto::Entry {
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

                    // Repository Entries
                    {
                        let repository_entry_filter =
                            if let Some(rejoined_worktree) = rejoined_worktree {
                                worktree_repository::Column::ScanId.gt(rejoined_worktree.scan_id)
                            } else {
                                worktree_repository::Column::IsDeleted.eq(false)
                            };

                        let mut db_repositories = worktree_repository::Entity::find()
                            .filter(
                                Condition::all()
                                    .add(worktree_repository::Column::ProjectId.eq(project.id))
                                    .add(worktree_repository::Column::WorktreeId.eq(worktree.id))
                                    .add(repository_entry_filter),
                            )
                            .stream(&*tx)
                            .await?;

                        while let Some(db_repository) = db_repositories.next().await {
                            let db_repository = db_repository?;
                            if db_repository.is_deleted {
                                worktree
                                    .removed_repositories
                                    .push(db_repository.work_directory_id as u64);
                            } else {
                                worktree.updated_repositories.push(proto::RepositoryEntry {
                                    work_directory_id: db_repository.work_directory_id as u64,
                                    branch: db_repository.branch,
                                });
                            }
                        }
                    }

                    worktrees.push(worktree);
                }

                let language_servers = project
                    .find_related(language_server::Entity)
                    .all(&*tx)
                    .await?
                    .into_iter()
                    .map(|language_server| proto::LanguageServer {
                        id: language_server.id as u64,
                        name: language_server.name,
                    })
                    .collect::<Vec<_>>();

                {
                    let mut db_settings_files = worktree_settings_file::Entity::find()
                        .filter(worktree_settings_file::Column::ProjectId.eq(project_id))
                        .stream(&*tx)
                        .await?;
                    while let Some(db_settings_file) = db_settings_files.next().await {
                        let db_settings_file = db_settings_file?;
                        if let Some(worktree) = worktrees
                            .iter_mut()
                            .find(|w| w.id == db_settings_file.worktree_id as u64)
                        {
                            worktree.settings_files.push(WorktreeSettingsFile {
                                path: db_settings_file.path,
                                content: db_settings_file.content,
                            });
                        }
                    }
                }

                let mut collaborators = project
                    .find_related(project_collaborator::Entity)
                    .all(&*tx)
                    .await?;
                let self_collaborator = if let Some(self_collaborator_ix) = collaborators
                    .iter()
                    .position(|collaborator| collaborator.user_id == user_id)
                {
                    collaborators.swap_remove(self_collaborator_ix)
                } else {
                    continue;
                };
                let old_connection_id = self_collaborator.connection();
                project_collaborator::Entity::update(project_collaborator::ActiveModel {
                    connection_id: ActiveValue::set(connection.id as i32),
                    connection_server_id: ActiveValue::set(ServerId(connection.owner_id as i32)),
                    ..self_collaborator.into_active_model()
                })
                .exec(&*tx)
                .await?;

                let collaborators = collaborators
                    .into_iter()
                    .map(|collaborator| ProjectCollaborator {
                        connection_id: collaborator.connection(),
                        user_id: collaborator.user_id,
                        replica_id: collaborator.replica_id,
                        is_host: collaborator.is_host,
                    })
                    .collect::<Vec<_>>();

                rejoined_projects.push(RejoinedProject {
                    id: project_id,
                    old_connection_id,
                    collaborators,
                    worktrees,
                    language_servers,
                });
            }

            let (channel, room) = self.get_channel_room(room_id, &tx).await?;
            let channel_members = if let Some(channel) = &channel {
                self.get_channel_participants(&channel, &tx).await?
            } else {
                Vec::new()
            };

            Ok(RejoinedRoom {
                room,
                channel_id: channel.map(|channel| channel.id),
                channel_members,
                rejoined_projects,
                reshared_projects,
            })
        })
        .await
    }

    pub async fn leave_room(
        &self,
        connection: ConnectionId,
    ) -> Result<Option<RoomGuard<LeftRoom>>> {
        self.optional_room_transaction(|tx| async move {
            let leaving_participant = room_participant::Entity::find()
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
                .await?;

            if let Some(leaving_participant) = leaving_participant {
                // Leave room.
                let room_id = leaving_participant.room_id;
                room_participant::Entity::delete_by_id(leaving_participant.id)
                    .exec(&*tx)
                    .await?;

                // Cancel pending calls initiated by the leaving user.
                let called_participants = room_participant::Entity::find()
                    .filter(
                        Condition::all()
                            .add(
                                room_participant::Column::CallingUserId
                                    .eq(leaving_participant.user_id),
                            )
                            .add(room_participant::Column::AnsweringConnectionId.is_null()),
                    )
                    .all(&*tx)
                    .await?;
                room_participant::Entity::delete_many()
                    .filter(
                        room_participant::Column::Id
                            .is_in(called_participants.iter().map(|participant| participant.id)),
                    )
                    .exec(&*tx)
                    .await?;
                let canceled_calls_to_user_ids = called_participants
                    .into_iter()
                    .map(|participant| participant.user_id)
                    .collect();

                // Detect left projects.
                #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
                enum QueryProjectIds {
                    ProjectId,
                }
                let project_ids: Vec<ProjectId> = project_collaborator::Entity::find()
                    .select_only()
                    .column_as(
                        project_collaborator::Column::ProjectId,
                        QueryProjectIds::ProjectId,
                    )
                    .filter(
                        Condition::all()
                            .add(
                                project_collaborator::Column::ConnectionId.eq(connection.id as i32),
                            )
                            .add(
                                project_collaborator::Column::ConnectionServerId
                                    .eq(connection.owner_id as i32),
                            ),
                    )
                    .into_values::<_, QueryProjectIds>()
                    .all(&*tx)
                    .await?;
                let mut left_projects = HashMap::default();
                let mut collaborators = project_collaborator::Entity::find()
                    .filter(project_collaborator::Column::ProjectId.is_in(project_ids))
                    .stream(&*tx)
                    .await?;
                while let Some(collaborator) = collaborators.next().await {
                    let collaborator = collaborator?;
                    let left_project =
                        left_projects
                            .entry(collaborator.project_id)
                            .or_insert(LeftProject {
                                id: collaborator.project_id,
                                host_user_id: Default::default(),
                                connection_ids: Default::default(),
                                host_connection_id: None,
                            });

                    let collaborator_connection_id = collaborator.connection();
                    if collaborator_connection_id != connection {
                        left_project.connection_ids.push(collaborator_connection_id);
                    }

                    if collaborator.is_host {
                        left_project.host_user_id = collaborator.user_id;
                        left_project.host_connection_id = Some(collaborator_connection_id);
                    }
                }
                drop(collaborators);

                // Leave projects.
                project_collaborator::Entity::delete_many()
                    .filter(
                        Condition::all()
                            .add(
                                project_collaborator::Column::ConnectionId.eq(connection.id as i32),
                            )
                            .add(
                                project_collaborator::Column::ConnectionServerId
                                    .eq(connection.owner_id as i32),
                            ),
                    )
                    .exec(&*tx)
                    .await?;

                follower::Entity::delete_many()
                    .filter(
                        Condition::all()
                            .add(follower::Column::FollowerConnectionId.eq(connection.id as i32)),
                    )
                    .exec(&*tx)
                    .await?;

                // Unshare projects.
                project::Entity::delete_many()
                    .filter(
                        Condition::all()
                            .add(project::Column::RoomId.eq(room_id))
                            .add(project::Column::HostConnectionId.eq(connection.id as i32))
                            .add(
                                project::Column::HostConnectionServerId
                                    .eq(connection.owner_id as i32),
                            ),
                    )
                    .exec(&*tx)
                    .await?;

                let (channel, room) = self.get_channel_room(room_id, &tx).await?;
                let deleted = if room.participants.is_empty() {
                    let result = room::Entity::delete_by_id(room_id).exec(&*tx).await?;
                    result.rows_affected > 0
                } else {
                    false
                };

                let channel_members = if let Some(channel) = &channel {
                    self.get_channel_participants(channel, &tx).await?
                } else {
                    Vec::new()
                };
                let left_room = LeftRoom {
                    room,
                    channel_id: channel.map(|channel| channel.id),
                    channel_members,
                    left_projects,
                    canceled_calls_to_user_ids,
                    deleted,
                };

                if left_room.room.participants.is_empty() {
                    self.rooms.remove(&room_id);
                }

                Ok(Some((room_id, left_room)))
            } else {
                Ok(None)
            }
        })
        .await
    }

    /// Updates the location of a participant in the given room.
    pub async fn update_room_participant_location(
        &self,
        room_id: RoomId,
        connection: ConnectionId,
        location: proto::ParticipantLocation,
    ) -> Result<RoomGuard<proto::Room>> {
        self.room_transaction(room_id, |tx| async {
            let tx = tx;
            let location_kind;
            let location_project_id;
            match location
                .variant
                .as_ref()
                .ok_or_else(|| anyhow!("invalid location"))?
            {
                proto::participant_location::Variant::SharedProject(project) => {
                    location_kind = 0;
                    location_project_id = Some(ProjectId::from_proto(project.id));
                }
                proto::participant_location::Variant::UnsharedProject(_) => {
                    location_kind = 1;
                    location_project_id = None;
                }
                proto::participant_location::Variant::External(_) => {
                    location_kind = 2;
                    location_project_id = None;
                }
            }

            let result = room_participant::Entity::update_many()
                .filter(
                    Condition::all()
                        .add(room_participant::Column::RoomId.eq(room_id))
                        .add(
                            room_participant::Column::AnsweringConnectionId
                                .eq(connection.id as i32),
                        )
                        .add(
                            room_participant::Column::AnsweringConnectionServerId
                                .eq(connection.owner_id as i32),
                        ),
                )
                .set(room_participant::ActiveModel {
                    location_kind: ActiveValue::set(Some(location_kind)),
                    location_project_id: ActiveValue::set(location_project_id),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;

            if result.rows_affected == 1 {
                let room = self.get_room(room_id, &tx).await?;
                Ok(room)
            } else {
                Err(anyhow!("could not update room participant location"))?
            }
        })
        .await
    }

    /// Sets the role of a participant in the given room.
    pub async fn set_room_participant_role(
        &self,
        admin_id: UserId,
        room_id: RoomId,
        user_id: UserId,
        role: ChannelRole,
    ) -> Result<RoomGuard<proto::Room>> {
        self.room_transaction(room_id, |tx| async move {
            room_participant::Entity::find()
                .filter(
                    Condition::all()
                        .add(room_participant::Column::RoomId.eq(room_id))
                        .add(room_participant::Column::UserId.eq(admin_id))
                        .add(room_participant::Column::Role.eq(ChannelRole::Admin)),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("only admins can set participant role"))?;

            if role.requires_cla() {
                self.check_user_has_signed_cla(user_id, room_id, &*tx)
                    .await?;
            }

            let result = room_participant::Entity::update_many()
                .filter(
                    Condition::all()
                        .add(room_participant::Column::RoomId.eq(room_id))
                        .add(room_participant::Column::UserId.eq(user_id)),
                )
                .set(room_participant::ActiveModel {
                    role: ActiveValue::set(Some(ChannelRole::from(role))),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;

            if result.rows_affected != 1 {
                Err(anyhow!("could not update room participant role"))?;
            }
            Ok(self.get_room(room_id, &tx).await?)
        })
        .await
    }

    async fn check_user_has_signed_cla(
        &self,
        user_id: UserId,
        room_id: RoomId,
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        let channel = room::Entity::find_by_id(room_id)
            .one(&*tx)
            .await?
            .ok_or_else(|| anyhow!("could not find room"))?
            .find_related(channel::Entity)
            .one(&*tx)
            .await?;

        if let Some(channel) = channel {
            let requires_zed_cla = channel.requires_zed_cla
                || channel::Entity::find()
                    .filter(
                        channel::Column::Id
                            .is_in(channel.ancestors())
                            .and(channel::Column::RequiresZedCla.eq(true)),
                    )
                    .count(&*tx)
                    .await?
                    > 0;
            if requires_zed_cla {
                if contributor::Entity::find()
                    .filter(contributor::Column::UserId.eq(user_id))
                    .one(&*tx)
                    .await?
                    .is_none()
                {
                    Err(anyhow!("user has not signed the Zed CLA"))?;
                }
            }
        }
        Ok(())
    }

    pub async fn connection_lost(&self, connection: ConnectionId) -> Result<()> {
        self.transaction(|tx| async move {
            self.room_connection_lost(connection, &*tx).await?;
            self.channel_buffer_connection_lost(connection, &*tx)
                .await?;
            self.channel_chat_connection_lost(connection, &*tx).await?;
            Ok(())
        })
        .await
    }

    pub async fn room_connection_lost(
        &self,
        connection: ConnectionId,
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        let participant = room_participant::Entity::find()
            .filter(
                Condition::all()
                    .add(room_participant::Column::AnsweringConnectionId.eq(connection.id as i32))
                    .add(
                        room_participant::Column::AnsweringConnectionServerId
                            .eq(connection.owner_id as i32),
                    ),
            )
            .one(&*tx)
            .await?;

        if let Some(participant) = participant {
            room_participant::Entity::update(room_participant::ActiveModel {
                answering_connection_lost: ActiveValue::set(true),
                ..participant.into_active_model()
            })
            .exec(&*tx)
            .await?;
        }
        Ok(())
    }

    fn build_incoming_call(
        room: &proto::Room,
        called_user_id: UserId,
    ) -> Option<proto::IncomingCall> {
        let pending_participant = room
            .pending_participants
            .iter()
            .find(|participant| participant.user_id == called_user_id.to_proto())?;

        Some(proto::IncomingCall {
            room_id: room.id,
            calling_user_id: pending_participant.calling_user_id,
            participant_user_ids: room
                .participants
                .iter()
                .map(|participant| participant.user_id)
                .collect(),
            initial_project: room.participants.iter().find_map(|participant| {
                let initial_project_id = pending_participant.initial_project_id?;
                participant
                    .projects
                    .iter()
                    .find(|project| project.id == initial_project_id)
                    .cloned()
            }),
        })
    }

    pub async fn get_room(&self, room_id: RoomId, tx: &DatabaseTransaction) -> Result<proto::Room> {
        let (_, room) = self.get_channel_room(room_id, tx).await?;
        Ok(room)
    }

    pub async fn room_connection_ids(
        &self,
        room_id: RoomId,
        connection_id: ConnectionId,
    ) -> Result<RoomGuard<HashSet<ConnectionId>>> {
        self.room_transaction(room_id, |tx| async move {
            let mut participants = room_participant::Entity::find()
                .filter(room_participant::Column::RoomId.eq(room_id))
                .stream(&*tx)
                .await?;

            let mut is_participant = false;
            let mut connection_ids = HashSet::default();
            while let Some(participant) = participants.next().await {
                let participant = participant?;
                if let Some(answering_connection) = participant.answering_connection() {
                    if answering_connection == connection_id {
                        is_participant = true;
                    } else {
                        connection_ids.insert(answering_connection);
                    }
                }
            }

            if !is_participant {
                Err(anyhow!("not a room participant"))?;
            }

            Ok(connection_ids)
        })
        .await
    }

    async fn get_channel_room(
        &self,
        room_id: RoomId,
        tx: &DatabaseTransaction,
    ) -> Result<(Option<channel::Model>, proto::Room)> {
        let db_room = room::Entity::find_by_id(room_id)
            .one(tx)
            .await?
            .ok_or_else(|| anyhow!("could not find room"))?;

        let mut db_participants = db_room
            .find_related(room_participant::Entity)
            .stream(tx)
            .await?;
        let mut participants = HashMap::default();
        let mut pending_participants = Vec::new();
        while let Some(db_participant) = db_participants.next().await {
            let db_participant = db_participant?;
            if let (
                Some(answering_connection_id),
                Some(answering_connection_server_id),
                Some(participant_index),
            ) = (
                db_participant.answering_connection_id,
                db_participant.answering_connection_server_id,
                db_participant.participant_index,
            ) {
                let location = match (
                    db_participant.location_kind,
                    db_participant.location_project_id,
                ) {
                    (Some(0), Some(project_id)) => {
                        Some(proto::participant_location::Variant::SharedProject(
                            proto::participant_location::SharedProject {
                                id: project_id.to_proto(),
                            },
                        ))
                    }
                    (Some(1), _) => Some(proto::participant_location::Variant::UnsharedProject(
                        Default::default(),
                    )),
                    _ => Some(proto::participant_location::Variant::External(
                        Default::default(),
                    )),
                };

                let answering_connection = ConnectionId {
                    owner_id: answering_connection_server_id.0 as u32,
                    id: answering_connection_id as u32,
                };
                participants.insert(
                    answering_connection,
                    proto::Participant {
                        user_id: db_participant.user_id.to_proto(),
                        peer_id: Some(answering_connection.into()),
                        projects: Default::default(),
                        location: Some(proto::ParticipantLocation { variant: location }),
                        participant_index: participant_index as u32,
                        role: db_participant.role.unwrap_or(ChannelRole::Member).into(),
                    },
                );
            } else {
                pending_participants.push(proto::PendingParticipant {
                    user_id: db_participant.user_id.to_proto(),
                    calling_user_id: db_participant.calling_user_id.to_proto(),
                    initial_project_id: db_participant.initial_project_id.map(|id| id.to_proto()),
                });
            }
        }
        drop(db_participants);

        let mut db_projects = db_room
            .find_related(project::Entity)
            .find_with_related(worktree::Entity)
            .stream(tx)
            .await?;

        while let Some(row) = db_projects.next().await {
            let (db_project, db_worktree) = row?;
            let host_connection = db_project.host_connection()?;
            if let Some(participant) = participants.get_mut(&host_connection) {
                let project = if let Some(project) = participant
                    .projects
                    .iter_mut()
                    .find(|project| project.id == db_project.id.to_proto())
                {
                    project
                } else {
                    participant.projects.push(proto::ParticipantProject {
                        id: db_project.id.to_proto(),
                        worktree_root_names: Default::default(),
                    });
                    participant.projects.last_mut().unwrap()
                };

                if let Some(db_worktree) = db_worktree {
                    if db_worktree.visible {
                        project.worktree_root_names.push(db_worktree.root_name);
                    }
                }
            }
        }
        drop(db_projects);

        let mut db_followers = db_room.find_related(follower::Entity).stream(tx).await?;
        let mut followers = Vec::new();
        while let Some(db_follower) = db_followers.next().await {
            let db_follower = db_follower?;
            followers.push(proto::Follower {
                leader_id: Some(db_follower.leader_connection().into()),
                follower_id: Some(db_follower.follower_connection().into()),
                project_id: db_follower.project_id.to_proto(),
            });
        }
        drop(db_followers);

        let channel = if let Some(channel_id) = db_room.channel_id {
            Some(self.get_channel_internal(channel_id, &*tx).await?)
        } else {
            None
        };

        Ok((
            channel,
            proto::Room {
                id: db_room.id.to_proto(),
                live_kit_room: db_room.live_kit_room,
                participants: participants.into_values().collect(),
                pending_participants,
                followers,
            },
        ))
    }
}
