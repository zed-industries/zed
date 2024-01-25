use super::*;
use rpc::{proto::channel_member::Kind, ErrorCode, ErrorCodeExt};
use sea_orm::TryGetableMany;

impl Database {
    #[cfg(test)]
    pub async fn all_channels(&self) -> Result<Vec<(ChannelId, String)>> {
        self.transaction(move |tx| async move {
            let mut channels = Vec::new();
            let mut rows = channel::Entity::find().stream(&*tx).await?;
            while let Some(row) = rows.next().await {
                let row = row?;
                channels.push((row.id, row.name));
            }
            Ok(channels)
        })
        .await
    }

    #[cfg(test)]
    pub async fn create_root_channel(&self, name: &str, creator_id: UserId) -> Result<ChannelId> {
        Ok(self
            .create_channel(name, None, creator_id)
            .await?
            .channel
            .id)
    }

    #[cfg(test)]
    pub async fn create_sub_channel(
        &self,
        name: &str,
        parent: ChannelId,
        creator_id: UserId,
    ) -> Result<ChannelId> {
        Ok(self
            .create_channel(name, Some(parent), creator_id)
            .await?
            .channel
            .id)
    }

    /// Creates a new channel.
    pub async fn create_channel(
        &self,
        name: &str,
        parent_channel_id: Option<ChannelId>,
        admin_id: UserId,
    ) -> Result<CreateChannelResult> {
        let name = Self::sanitize_channel_name(name)?;
        self.transaction(move |tx| async move {
            let mut parent = None;

            if let Some(parent_channel_id) = parent_channel_id {
                let parent_channel = self.get_channel_internal(parent_channel_id, &*tx).await?;
                self.check_user_is_channel_admin(&parent_channel, admin_id, &*tx)
                    .await?;
                parent = Some(parent_channel);
            }

            let channel = channel::ActiveModel {
                id: ActiveValue::NotSet,
                name: ActiveValue::Set(name.to_string()),
                visibility: ActiveValue::Set(ChannelVisibility::Members),
                parent_path: ActiveValue::Set(
                    parent
                        .as_ref()
                        .map_or(String::new(), |parent| parent.path()),
                ),
                requires_zed_cla: ActiveValue::NotSet,
            }
            .insert(&*tx)
            .await?;

            let participants_to_update;
            if let Some(parent) = &parent {
                participants_to_update = self
                    .participants_to_notify_for_channel_change(parent, &*tx)
                    .await?;
            } else {
                participants_to_update = vec![];

                channel_member::ActiveModel {
                    id: ActiveValue::NotSet,
                    channel_id: ActiveValue::Set(channel.id),
                    user_id: ActiveValue::Set(admin_id),
                    accepted: ActiveValue::Set(true),
                    role: ActiveValue::Set(ChannelRole::Admin),
                }
                .insert(&*tx)
                .await?;
            };

            Ok(CreateChannelResult {
                channel: Channel::from_model(channel, ChannelRole::Admin),
                participants_to_update,
            })
        })
        .await
    }

    /// Adds a user to the specified channel.
    pub async fn join_channel(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        connection: ConnectionId,
        environment: &str,
    ) -> Result<(JoinRoom, Option<MembershipUpdated>, ChannelRole)> {
        self.transaction(move |tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            let mut role = self.channel_role_for_user(&channel, user_id, &*tx).await?;

            let mut accept_invite_result = None;

            if role.is_none() {
                if let Some(invitation) = self
                    .pending_invite_for_channel(&channel, user_id, &*tx)
                    .await?
                {
                    // note, this may be a parent channel
                    role = Some(invitation.role);
                    channel_member::Entity::update(channel_member::ActiveModel {
                        accepted: ActiveValue::Set(true),
                        ..invitation.into_active_model()
                    })
                    .exec(&*tx)
                    .await?;

                    accept_invite_result = Some(
                        self.calculate_membership_updated(&channel, user_id, &*tx)
                            .await?,
                    );

                    debug_assert!(
                        self.channel_role_for_user(&channel, user_id, &*tx).await? == role
                    );
                } else if channel.visibility == ChannelVisibility::Public {
                    role = Some(ChannelRole::Guest);
                    let channel_to_join = self
                        .public_ancestors_including_self(&channel, &*tx)
                        .await?
                        .first()
                        .cloned()
                        .unwrap_or(channel.clone());

                    channel_member::Entity::insert(channel_member::ActiveModel {
                        id: ActiveValue::NotSet,
                        channel_id: ActiveValue::Set(channel_to_join.id),
                        user_id: ActiveValue::Set(user_id),
                        accepted: ActiveValue::Set(true),
                        role: ActiveValue::Set(ChannelRole::Guest),
                    })
                    .exec(&*tx)
                    .await?;

                    accept_invite_result = Some(
                        self.calculate_membership_updated(&channel_to_join, user_id, &*tx)
                            .await?,
                    );

                    debug_assert!(
                        self.channel_role_for_user(&channel, user_id, &*tx).await? == role
                    );
                }
            }

            if role.is_none() || role == Some(ChannelRole::Banned) {
                Err(ErrorCode::Forbidden.anyhow())?
            }
            let role = role.unwrap();

            let live_kit_room = format!("channel-{}", nanoid::nanoid!(30));
            let room_id = self
                .get_or_create_channel_room(channel_id, &live_kit_room, environment, &*tx)
                .await?;

            self.join_channel_room_internal(room_id, user_id, connection, role, &*tx)
                .await
                .map(|jr| (jr, accept_invite_result, role))
        })
        .await
    }

    /// Sets the visibiltity of the given channel.
    pub async fn set_channel_visibility(
        &self,
        channel_id: ChannelId,
        visibility: ChannelVisibility,
        admin_id: UserId,
    ) -> Result<SetChannelVisibilityResult> {
        self.transaction(move |tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;

            self.check_user_is_channel_admin(&channel, admin_id, &*tx)
                .await?;

            let previous_members = self
                .get_channel_participant_details_internal(&channel, &*tx)
                .await?;

            let mut model = channel.into_active_model();
            model.visibility = ActiveValue::Set(visibility);
            let channel = model.update(&*tx).await?;

            let mut participants_to_update: HashMap<UserId, ChannelsForUser> = self
                .participants_to_notify_for_channel_change(&channel, &*tx)
                .await?
                .into_iter()
                .collect();

            let mut channels_to_remove: Vec<ChannelId> = vec![];
            let mut participants_to_remove: HashSet<UserId> = HashSet::default();
            match visibility {
                ChannelVisibility::Members => {
                    let all_descendents: Vec<ChannelId> = self
                        .get_channel_descendants_including_self(vec![channel_id], &*tx)
                        .await?
                        .into_iter()
                        .map(|channel| channel.id)
                        .collect();

                    channels_to_remove = channel::Entity::find()
                        .filter(
                            channel::Column::Id
                                .is_in(all_descendents)
                                .and(channel::Column::Visibility.eq(ChannelVisibility::Public)),
                        )
                        .all(&*tx)
                        .await?
                        .into_iter()
                        .map(|channel| channel.id)
                        .collect();

                    channels_to_remove.push(channel_id);

                    for member in previous_members {
                        if member.role.can_only_see_public_descendants() {
                            participants_to_remove.insert(member.user_id);
                        }
                    }
                }
                ChannelVisibility::Public => {
                    if let Some(public_parent) = self.public_parent_channel(&channel, &*tx).await? {
                        let parent_updates = self
                            .participants_to_notify_for_channel_change(&public_parent, &*tx)
                            .await?;

                        for (user_id, channels) in parent_updates {
                            participants_to_update.insert(user_id, channels);
                        }
                    }
                }
            }

            Ok(SetChannelVisibilityResult {
                participants_to_update,
                participants_to_remove,
                channels_to_remove,
            })
        })
        .await
    }

    #[cfg(test)]
    pub async fn set_channel_requires_zed_cla(
        &self,
        channel_id: ChannelId,
        requires_zed_cla: bool,
    ) -> Result<()> {
        self.transaction(move |tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            let mut model = channel.into_active_model();
            model.requires_zed_cla = ActiveValue::Set(requires_zed_cla);
            model.update(&*tx).await?;
            Ok(())
        })
        .await
    }

    /// Deletes the channel with the specified ID.
    pub async fn delete_channel(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> Result<(Vec<ChannelId>, Vec<UserId>)> {
        self.transaction(move |tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            self.check_user_is_channel_admin(&channel, user_id, &*tx)
                .await?;

            let members_to_notify: Vec<UserId> = channel_member::Entity::find()
                .filter(channel_member::Column::ChannelId.is_in(channel.ancestors_including_self()))
                .select_only()
                .column(channel_member::Column::UserId)
                .distinct()
                .into_values::<_, QueryUserIds>()
                .all(&*tx)
                .await?;

            let channels_to_remove = self
                .get_channel_descendants_including_self(vec![channel.id], &*tx)
                .await?
                .into_iter()
                .map(|channel| channel.id)
                .collect::<Vec<_>>();

            channel::Entity::delete_many()
                .filter(channel::Column::Id.is_in(channels_to_remove.iter().copied()))
                .exec(&*tx)
                .await?;

            Ok((channels_to_remove, members_to_notify))
        })
        .await
    }

    /// Invites a user to a channel as a member.
    pub async fn invite_channel_member(
        &self,
        channel_id: ChannelId,
        invitee_id: UserId,
        inviter_id: UserId,
        role: ChannelRole,
    ) -> Result<InviteMemberResult> {
        self.transaction(move |tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            self.check_user_is_channel_admin(&channel, inviter_id, &*tx)
                .await?;

            channel_member::ActiveModel {
                id: ActiveValue::NotSet,
                channel_id: ActiveValue::Set(channel_id),
                user_id: ActiveValue::Set(invitee_id),
                accepted: ActiveValue::Set(false),
                role: ActiveValue::Set(role),
            }
            .insert(&*tx)
            .await?;

            let channel = Channel::from_model(channel, role);

            let notifications = self
                .create_notification(
                    invitee_id,
                    rpc::Notification::ChannelInvitation {
                        channel_id: channel_id.to_proto(),
                        channel_name: channel.name.clone(),
                        inviter_id: inviter_id.to_proto(),
                    },
                    true,
                    &*tx,
                )
                .await?
                .into_iter()
                .collect();

            Ok(InviteMemberResult {
                channel,
                notifications,
            })
        })
        .await
    }

    fn sanitize_channel_name(name: &str) -> Result<&str> {
        let new_name = name.trim().trim_start_matches('#');
        if new_name == "" {
            Err(anyhow!("channel name can't be blank"))?;
        }
        Ok(new_name)
    }

    /// Renames the specified channel.
    pub async fn rename_channel(
        &self,
        channel_id: ChannelId,
        admin_id: UserId,
        new_name: &str,
    ) -> Result<RenameChannelResult> {
        self.transaction(move |tx| async move {
            let new_name = Self::sanitize_channel_name(new_name)?.to_string();

            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            let role = self
                .check_user_is_channel_admin(&channel, admin_id, &*tx)
                .await?;

            let mut model = channel.into_active_model();
            model.name = ActiveValue::Set(new_name.clone());
            let channel = model.update(&*tx).await?;

            let participants = self
                .get_channel_participant_details_internal(&channel, &*tx)
                .await?;

            Ok(RenameChannelResult {
                channel: Channel::from_model(channel.clone(), role),
                participants_to_update: participants
                    .iter()
                    .map(|participant| {
                        (
                            participant.user_id,
                            Channel::from_model(channel.clone(), participant.role),
                        )
                    })
                    .collect(),
            })
        })
        .await
    }

    /// accept or decline an invite to join a channel
    pub async fn respond_to_channel_invite(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        accept: bool,
    ) -> Result<RespondToChannelInvite> {
        self.transaction(move |tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;

            let membership_update = if accept {
                let rows_affected = channel_member::Entity::update_many()
                    .set(channel_member::ActiveModel {
                        accepted: ActiveValue::Set(accept),
                        ..Default::default()
                    })
                    .filter(
                        channel_member::Column::ChannelId
                            .eq(channel_id)
                            .and(channel_member::Column::UserId.eq(user_id))
                            .and(channel_member::Column::Accepted.eq(false)),
                    )
                    .exec(&*tx)
                    .await?
                    .rows_affected;

                if rows_affected == 0 {
                    Err(anyhow!("no such invitation"))?;
                }

                Some(
                    self.calculate_membership_updated(&channel, user_id, &*tx)
                        .await?,
                )
            } else {
                let rows_affected = channel_member::Entity::delete_many()
                    .filter(
                        channel_member::Column::ChannelId
                            .eq(channel_id)
                            .and(channel_member::Column::UserId.eq(user_id))
                            .and(channel_member::Column::Accepted.eq(false)),
                    )
                    .exec(&*tx)
                    .await?
                    .rows_affected;
                if rows_affected == 0 {
                    Err(anyhow!("no such invitation"))?;
                }

                None
            };

            Ok(RespondToChannelInvite {
                membership_update,
                notifications: self
                    .mark_notification_as_read_with_response(
                        user_id,
                        &rpc::Notification::ChannelInvitation {
                            channel_id: channel_id.to_proto(),
                            channel_name: Default::default(),
                            inviter_id: Default::default(),
                        },
                        accept,
                        &*tx,
                    )
                    .await?
                    .into_iter()
                    .collect(),
            })
        })
        .await
    }

    async fn calculate_membership_updated(
        &self,
        channel: &channel::Model,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<MembershipUpdated> {
        let new_channels = self.get_user_channels(user_id, Some(channel), &*tx).await?;
        let removed_channels = self
            .get_channel_descendants_including_self(vec![channel.id], &*tx)
            .await?
            .into_iter()
            .filter_map(|channel| {
                if !new_channels.channels.iter().any(|c| c.id == channel.id) {
                    Some(channel.id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(MembershipUpdated {
            channel_id: channel.id,
            new_channels,
            removed_channels,
        })
    }

    /// Removes a channel member.
    pub async fn remove_channel_member(
        &self,
        channel_id: ChannelId,
        member_id: UserId,
        admin_id: UserId,
    ) -> Result<RemoveChannelMemberResult> {
        self.transaction(|tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            self.check_user_is_channel_admin(&channel, admin_id, &*tx)
                .await?;

            let result = channel_member::Entity::delete_many()
                .filter(
                    channel_member::Column::ChannelId
                        .eq(channel_id)
                        .and(channel_member::Column::UserId.eq(member_id)),
                )
                .exec(&*tx)
                .await?;

            if result.rows_affected == 0 {
                Err(anyhow!("no such member"))?;
            }

            Ok(RemoveChannelMemberResult {
                membership_update: self
                    .calculate_membership_updated(&channel, member_id, &*tx)
                    .await?,
                notification_id: self
                    .remove_notification(
                        member_id,
                        rpc::Notification::ChannelInvitation {
                            channel_id: channel_id.to_proto(),
                            channel_name: Default::default(),
                            inviter_id: Default::default(),
                        },
                        &*tx,
                    )
                    .await?,
            })
        })
        .await
    }

    /// Returns all channel invites for the user with the given ID.
    pub async fn get_channel_invites_for_user(&self, user_id: UserId) -> Result<Vec<Channel>> {
        self.transaction(|tx| async move {
            let mut role_for_channel: HashMap<ChannelId, ChannelRole> = HashMap::default();

            let channel_invites = channel_member::Entity::find()
                .filter(
                    channel_member::Column::UserId
                        .eq(user_id)
                        .and(channel_member::Column::Accepted.eq(false)),
                )
                .all(&*tx)
                .await?;

            for invite in channel_invites {
                role_for_channel.insert(invite.channel_id, invite.role);
            }

            let channels = channel::Entity::find()
                .filter(channel::Column::Id.is_in(role_for_channel.keys().copied()))
                .all(&*tx)
                .await?;

            let channels = channels
                .into_iter()
                .filter_map(|channel| {
                    let role = *role_for_channel.get(&channel.id)?;
                    Some(Channel::from_model(channel, role))
                })
                .collect();

            Ok(channels)
        })
        .await
    }

    /// Returns all channels for the user with the given ID.
    pub async fn get_channels_for_user(&self, user_id: UserId) -> Result<ChannelsForUser> {
        self.transaction(|tx| async move {
            let tx = tx;

            self.get_user_channels(user_id, None, &tx).await
        })
        .await
    }

    /// Returns all channels for the user with the given ID that are descendants
    /// of the specified ancestor channel.
    pub async fn get_user_channels(
        &self,
        user_id: UserId,
        ancestor_channel: Option<&channel::Model>,
        tx: &DatabaseTransaction,
    ) -> Result<ChannelsForUser> {
        let channel_memberships = channel_member::Entity::find()
            .filter(
                channel_member::Column::UserId
                    .eq(user_id)
                    .and(channel_member::Column::Accepted.eq(true)),
            )
            .all(&*tx)
            .await?;

        let descendants = self
            .get_channel_descendants_including_self(
                channel_memberships.iter().map(|m| m.channel_id),
                &*tx,
            )
            .await?;

        let mut roles_by_channel_id: HashMap<ChannelId, ChannelRole> = HashMap::default();
        for membership in channel_memberships.iter() {
            roles_by_channel_id.insert(membership.channel_id, membership.role);
        }

        let mut visible_channel_ids: HashSet<ChannelId> = HashSet::default();

        let channels: Vec<Channel> = descendants
            .into_iter()
            .filter_map(|channel| {
                let parent_role = channel
                    .parent_id()
                    .and_then(|parent_id| roles_by_channel_id.get(&parent_id));

                let role = if let Some(parent_role) = parent_role {
                    let role = if let Some(existing_role) = roles_by_channel_id.get(&channel.id) {
                        existing_role.max(*parent_role)
                    } else {
                        *parent_role
                    };
                    roles_by_channel_id.insert(channel.id, role);
                    role
                } else {
                    *roles_by_channel_id.get(&channel.id)?
                };

                let can_see_parent_paths = role.can_see_all_descendants()
                    || role.can_only_see_public_descendants()
                        && channel.visibility == ChannelVisibility::Public;
                if !can_see_parent_paths {
                    return None;
                }

                visible_channel_ids.insert(channel.id);

                if let Some(ancestor) = ancestor_channel {
                    if !channel
                        .ancestors_including_self()
                        .any(|id| id == ancestor.id)
                    {
                        return None;
                    }
                }

                let mut channel = Channel::from_model(channel, role);
                channel
                    .parent_path
                    .retain(|id| visible_channel_ids.contains(&id));

                Some(channel)
            })
            .collect();

        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryUserIdsAndChannelIds {
            ChannelId,
            UserId,
        }

        let mut channel_participants: HashMap<ChannelId, Vec<UserId>> = HashMap::default();
        {
            let mut rows = room_participant::Entity::find()
                .inner_join(room::Entity)
                .filter(room::Column::ChannelId.is_in(channels.iter().map(|c| c.id)))
                .select_only()
                .column(room::Column::ChannelId)
                .column(room_participant::Column::UserId)
                .into_values::<_, QueryUserIdsAndChannelIds>()
                .stream(&*tx)
                .await?;
            while let Some(row) = rows.next().await {
                let row: (ChannelId, UserId) = row?;
                channel_participants.entry(row.0).or_default().push(row.1)
            }
        }

        let channel_ids = channels.iter().map(|c| c.id).collect::<Vec<_>>();
        let channel_buffer_changes = self
            .unseen_channel_buffer_changes(user_id, &channel_ids, &*tx)
            .await?;

        let unseen_messages = self
            .unseen_channel_messages(user_id, &channel_ids, &*tx)
            .await?;

        Ok(ChannelsForUser {
            channels,
            channel_participants,
            unseen_buffer_changes: channel_buffer_changes,
            channel_messages: unseen_messages,
        })
    }

    async fn participants_to_notify_for_channel_change(
        &self,
        new_parent: &channel::Model,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<(UserId, ChannelsForUser)>> {
        let mut results: Vec<(UserId, ChannelsForUser)> = Vec::new();

        let members = self
            .get_channel_participant_details_internal(new_parent, &*tx)
            .await?;

        for member in members.iter() {
            if !member.role.can_see_all_descendants() {
                continue;
            }
            results.push((
                member.user_id,
                self.get_user_channels(member.user_id, Some(new_parent), &*tx)
                    .await?,
            ))
        }

        let public_parents = self
            .public_ancestors_including_self(new_parent, &*tx)
            .await?;
        let public_parent = public_parents.last();

        let Some(public_parent) = public_parent else {
            return Ok(results);
        };

        // could save some time in the common case by skipping this if the
        // new channel is not public and has no public descendants.
        let public_members = if public_parent == new_parent {
            members
        } else {
            self.get_channel_participant_details_internal(public_parent, &*tx)
                .await?
        };

        for member in public_members {
            if !member.role.can_only_see_public_descendants() {
                continue;
            };
            results.push((
                member.user_id,
                self.get_user_channels(member.user_id, Some(public_parent), &*tx)
                    .await?,
            ))
        }

        Ok(results)
    }

    /// Sets the role for the specified channel member.
    pub async fn set_channel_member_role(
        &self,
        channel_id: ChannelId,
        admin_id: UserId,
        for_user: UserId,
        role: ChannelRole,
    ) -> Result<SetMemberRoleResult> {
        self.transaction(|tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            self.check_user_is_channel_admin(&channel, admin_id, &*tx)
                .await?;

            let membership = channel_member::Entity::find()
                .filter(
                    channel_member::Column::ChannelId
                        .eq(channel_id)
                        .and(channel_member::Column::UserId.eq(for_user)),
                )
                .one(&*tx)
                .await?;

            let Some(membership) = membership else {
                Err(anyhow!("no such member"))?
            };

            let mut update = membership.into_active_model();
            update.role = ActiveValue::Set(role);
            let updated = channel_member::Entity::update(update).exec(&*tx).await?;

            if updated.accepted {
                Ok(SetMemberRoleResult::MembershipUpdated(
                    self.calculate_membership_updated(&channel, for_user, &*tx)
                        .await?,
                ))
            } else {
                Ok(SetMemberRoleResult::InviteUpdated(Channel::from_model(
                    channel, role,
                )))
            }
        })
        .await
    }

    /// Returns the details for the specified channel member.
    pub async fn get_channel_participant_details(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> Result<Vec<proto::ChannelMember>> {
        let (role, members) = self
            .transaction(move |tx| async move {
                let channel = self.get_channel_internal(channel_id, &*tx).await?;
                let role = self
                    .check_user_is_channel_participant(&channel, user_id, &*tx)
                    .await?;
                Ok((
                    role,
                    self.get_channel_participant_details_internal(&channel, &*tx)
                        .await?,
                ))
            })
            .await?;

        if role == ChannelRole::Admin {
            Ok(members
                .into_iter()
                .map(|channel_member| channel_member.to_proto())
                .collect())
        } else {
            return Ok(members
                .into_iter()
                .filter_map(|member| {
                    if member.kind == proto::channel_member::Kind::Invitee {
                        return None;
                    }
                    Some(ChannelMember {
                        role: member.role,
                        user_id: member.user_id,
                        kind: proto::channel_member::Kind::Member,
                    })
                })
                .map(|channel_member| channel_member.to_proto())
                .collect());
        }
    }

    async fn get_channel_participant_details_internal(
        &self,
        channel: &channel::Model,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<ChannelMember>> {
        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryMemberDetails {
            UserId,
            Role,
            IsDirectMember,
            Accepted,
            Visibility,
        }

        let mut stream = channel_member::Entity::find()
            .left_join(channel::Entity)
            .filter(channel_member::Column::ChannelId.is_in(channel.ancestors_including_self()))
            .select_only()
            .column(channel_member::Column::UserId)
            .column(channel_member::Column::Role)
            .column_as(
                channel_member::Column::ChannelId.eq(channel.id),
                QueryMemberDetails::IsDirectMember,
            )
            .column(channel_member::Column::Accepted)
            .column(channel::Column::Visibility)
            .into_values::<_, QueryMemberDetails>()
            .stream(&*tx)
            .await?;

        let mut user_details: HashMap<UserId, ChannelMember> = HashMap::default();

        while let Some(user_membership) = stream.next().await {
            let (user_id, channel_role, is_direct_member, is_invite_accepted, visibility): (
                UserId,
                ChannelRole,
                bool,
                bool,
                ChannelVisibility,
            ) = user_membership?;
            let kind = match (is_direct_member, is_invite_accepted) {
                (true, true) => proto::channel_member::Kind::Member,
                (true, false) => proto::channel_member::Kind::Invitee,
                (false, true) => proto::channel_member::Kind::AncestorMember,
                (false, false) => continue,
            };

            if channel_role == ChannelRole::Guest
                && visibility != ChannelVisibility::Public
                && channel.visibility != ChannelVisibility::Public
            {
                continue;
            }

            if let Some(details_mut) = user_details.get_mut(&user_id) {
                if channel_role.should_override(details_mut.role) {
                    details_mut.role = channel_role;
                }
                if kind == Kind::Member {
                    details_mut.kind = kind;
                // the UI is going to be a bit confusing if you already have permissions
                // that are greater than or equal to the ones you're being invited to.
                } else if kind == Kind::Invitee && details_mut.kind == Kind::AncestorMember {
                    details_mut.kind = kind;
                }
            } else {
                user_details.insert(
                    user_id,
                    ChannelMember {
                        user_id,
                        kind,
                        role: channel_role,
                    },
                );
            }
        }

        Ok(user_details
            .into_iter()
            .map(|(_, details)| details)
            .collect())
    }

    /// Returns the participants in the given channel.
    pub async fn get_channel_participants(
        &self,
        channel: &channel::Model,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<UserId>> {
        let participants = self
            .get_channel_participant_details_internal(channel, &*tx)
            .await?;
        Ok(participants
            .into_iter()
            .map(|member| member.user_id)
            .collect())
    }

    /// Returns whether the given user is an admin in the specified channel.
    pub async fn check_user_is_channel_admin(
        &self,
        channel: &channel::Model,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<ChannelRole> {
        let role = self.channel_role_for_user(channel, user_id, tx).await?;
        match role {
            Some(ChannelRole::Admin) => Ok(role.unwrap()),
            Some(ChannelRole::Member)
            | Some(ChannelRole::Banned)
            | Some(ChannelRole::Guest)
            | None => Err(anyhow!(
                "user is not a channel admin or channel does not exist"
            ))?,
        }
    }

    /// Returns whether the given user is a member of the specified channel.
    pub async fn check_user_is_channel_member(
        &self,
        channel: &channel::Model,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<ChannelRole> {
        let channel_role = self.channel_role_for_user(channel, user_id, tx).await?;
        match channel_role {
            Some(ChannelRole::Admin) | Some(ChannelRole::Member) => Ok(channel_role.unwrap()),
            Some(ChannelRole::Banned) | Some(ChannelRole::Guest) | None => Err(anyhow!(
                "user is not a channel member or channel does not exist"
            ))?,
        }
    }

    /// Returns whether the given user is a participant in the specified channel.
    pub async fn check_user_is_channel_participant(
        &self,
        channel: &channel::Model,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<ChannelRole> {
        let role = self.channel_role_for_user(channel, user_id, tx).await?;
        match role {
            Some(ChannelRole::Admin) | Some(ChannelRole::Member) | Some(ChannelRole::Guest) => {
                Ok(role.unwrap())
            }
            Some(ChannelRole::Banned) | None => Err(anyhow!(
                "user is not a channel participant or channel does not exist"
            ))?,
        }
    }

    /// Returns a user's pending invite for the given channel, if one exists.
    pub async fn pending_invite_for_channel(
        &self,
        channel: &channel::Model,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<Option<channel_member::Model>> {
        let row = channel_member::Entity::find()
            .filter(channel_member::Column::ChannelId.is_in(channel.ancestors_including_self()))
            .filter(channel_member::Column::UserId.eq(user_id))
            .filter(channel_member::Column::Accepted.eq(false))
            .one(&*tx)
            .await?;

        Ok(row)
    }

    async fn public_parent_channel(
        &self,
        channel: &channel::Model,
        tx: &DatabaseTransaction,
    ) -> Result<Option<channel::Model>> {
        let mut path = self.public_ancestors_including_self(channel, &*tx).await?;
        if path.last().unwrap().id == channel.id {
            path.pop();
        }
        Ok(path.pop())
    }

    pub(crate) async fn public_ancestors_including_self(
        &self,
        channel: &channel::Model,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<channel::Model>> {
        let visible_channels = channel::Entity::find()
            .filter(channel::Column::Id.is_in(channel.ancestors_including_self()))
            .filter(channel::Column::Visibility.eq(ChannelVisibility::Public))
            .order_by_asc(channel::Column::ParentPath)
            .all(&*tx)
            .await?;

        Ok(visible_channels)
    }

    /// Returns the role for a user in the given channel.
    pub async fn channel_role_for_user(
        &self,
        channel: &channel::Model,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<Option<ChannelRole>> {
        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryChannelMembership {
            ChannelId,
            Role,
            Visibility,
        }

        let mut rows = channel_member::Entity::find()
            .left_join(channel::Entity)
            .filter(
                channel_member::Column::ChannelId
                    .is_in(channel.ancestors_including_self())
                    .and(channel_member::Column::UserId.eq(user_id))
                    .and(channel_member::Column::Accepted.eq(true)),
            )
            .select_only()
            .column(channel_member::Column::ChannelId)
            .column(channel_member::Column::Role)
            .column(channel::Column::Visibility)
            .into_values::<_, QueryChannelMembership>()
            .stream(&*tx)
            .await?;

        let mut user_role: Option<ChannelRole> = None;

        let mut is_participant = false;
        let mut current_channel_visibility = None;

        // note these channels are not iterated in any particular order,
        // our current logic takes the highest permission available.
        while let Some(row) = rows.next().await {
            let (membership_channel, role, visibility): (
                ChannelId,
                ChannelRole,
                ChannelVisibility,
            ) = row?;

            match role {
                ChannelRole::Admin | ChannelRole::Member | ChannelRole::Banned => {
                    if let Some(users_role) = user_role {
                        user_role = Some(users_role.max(role));
                    } else {
                        user_role = Some(role)
                    }
                }
                ChannelRole::Guest if visibility == ChannelVisibility::Public => {
                    is_participant = true
                }
                ChannelRole::Guest => {}
            }
            if channel.id == membership_channel {
                current_channel_visibility = Some(visibility);
            }
        }
        // free up database connection
        drop(rows);

        if is_participant && user_role.is_none() {
            if current_channel_visibility.is_none() {
                current_channel_visibility = channel::Entity::find()
                    .filter(channel::Column::Id.eq(channel.id))
                    .one(&*tx)
                    .await?
                    .map(|channel| channel.visibility);
            }
            if current_channel_visibility == Some(ChannelVisibility::Public) {
                user_role = Some(ChannelRole::Guest);
            }
        }

        Ok(user_role)
    }

    // Get the descendants of the given set if channels, ordered by their
    // path.
    async fn get_channel_descendants_including_self(
        &self,
        channel_ids: impl IntoIterator<Item = ChannelId>,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<channel::Model>> {
        let mut values = String::new();
        for id in channel_ids {
            if !values.is_empty() {
                values.push_str(", ");
            }
            write!(&mut values, "({})", id).unwrap();
        }

        if values.is_empty() {
            return Ok(vec![]);
        }

        let sql = format!(
            r#"
            SELECT DISTINCT
                descendant_channels.*,
                descendant_channels.parent_path || descendant_channels.id as full_path
            FROM
                channels parent_channels, channels descendant_channels
            WHERE
                descendant_channels.id IN ({values}) OR
                (
                    parent_channels.id IN ({values}) AND
                    descendant_channels.parent_path LIKE (parent_channels.parent_path || parent_channels.id || '/%')
                )
            ORDER BY
                full_path ASC
            "#
        );

        Ok(channel::Entity::find()
            .from_raw_sql(Statement::from_string(
                self.pool.get_database_backend(),
                sql,
            ))
            .all(tx)
            .await?)
    }

    /// Returns the channel with the given ID.
    pub async fn get_channel(&self, channel_id: ChannelId, user_id: UserId) -> Result<Channel> {
        self.transaction(|tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            let role = self
                .check_user_is_channel_participant(&channel, user_id, &*tx)
                .await?;

            Ok(Channel::from_model(channel, role))
        })
        .await
    }

    pub(crate) async fn get_channel_internal(
        &self,
        channel_id: ChannelId,
        tx: &DatabaseTransaction,
    ) -> Result<channel::Model> {
        Ok(channel::Entity::find_by_id(channel_id)
            .one(&*tx)
            .await?
            .ok_or_else(|| proto::ErrorCode::NoSuchChannel.anyhow())?)
    }

    pub(crate) async fn get_or_create_channel_room(
        &self,
        channel_id: ChannelId,
        live_kit_room: &str,
        environment: &str,
        tx: &DatabaseTransaction,
    ) -> Result<RoomId> {
        let room = room::Entity::find()
            .filter(room::Column::ChannelId.eq(channel_id))
            .one(&*tx)
            .await?;

        let room_id = if let Some(room) = room {
            if let Some(env) = room.environment {
                if &env != environment {
                    Err(ErrorCode::WrongReleaseChannel
                        .with_tag("required", &env)
                        .anyhow())?;
                }
            }
            room.id
        } else {
            let result = room::Entity::insert(room::ActiveModel {
                channel_id: ActiveValue::Set(Some(channel_id)),
                live_kit_room: ActiveValue::Set(live_kit_room.to_string()),
                environment: ActiveValue::Set(Some(environment.to_string())),
                ..Default::default()
            })
            .exec(&*tx)
            .await?;

            result.last_insert_id
        };

        Ok(room_id)
    }

    /// Move a channel from one parent to another
    pub async fn move_channel(
        &self,
        channel_id: ChannelId,
        new_parent_id: Option<ChannelId>,
        admin_id: UserId,
    ) -> Result<Option<MoveChannelResult>> {
        self.transaction(|tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            self.check_user_is_channel_admin(&channel, admin_id, &*tx)
                .await?;

            let new_parent_path;
            let new_parent_channel;
            if let Some(new_parent_id) = new_parent_id {
                let new_parent = self.get_channel_internal(new_parent_id, &*tx).await?;
                self.check_user_is_channel_admin(&new_parent, admin_id, &*tx)
                    .await?;

                if new_parent
                    .ancestors_including_self()
                    .any(|id| id == channel.id)
                {
                    Err(anyhow!("cannot move a channel into one of its descendants"))?;
                }

                new_parent_path = new_parent.path();
                new_parent_channel = Some(new_parent);
            } else {
                new_parent_path = String::new();
                new_parent_channel = None;
            };

            let previous_participants = self
                .get_channel_participant_details_internal(&channel, &*tx)
                .await?;

            let old_path = format!("{}{}/", channel.parent_path, channel.id);
            let new_path = format!("{}{}/", new_parent_path, channel.id);

            if old_path == new_path {
                return Ok(None);
            }

            let mut model = channel.into_active_model();
            model.parent_path = ActiveValue::Set(new_parent_path);
            let channel = model.update(&*tx).await?;

            if new_parent_channel.is_none() {
                channel_member::ActiveModel {
                    id: ActiveValue::NotSet,
                    channel_id: ActiveValue::Set(channel_id),
                    user_id: ActiveValue::Set(admin_id),
                    accepted: ActiveValue::Set(true),
                    role: ActiveValue::Set(ChannelRole::Admin),
                }
                .insert(&*tx)
                .await?;
            }

            let descendent_ids =
                ChannelId::find_by_statement::<QueryIds>(Statement::from_sql_and_values(
                    self.pool.get_database_backend(),
                    "
                    UPDATE channels SET parent_path = REPLACE(parent_path, $1, $2)
                    WHERE parent_path LIKE $3 || '%'
                    RETURNING id
                ",
                    [old_path.clone().into(), new_path.into(), old_path.into()],
                ))
                .all(&*tx)
                .await?;

            let participants_to_update: HashMap<_, _> = self
                .participants_to_notify_for_channel_change(
                    new_parent_channel.as_ref().unwrap_or(&channel),
                    &*tx,
                )
                .await?
                .into_iter()
                .collect();

            let mut moved_channels: HashSet<ChannelId> = HashSet::default();
            for id in descendent_ids {
                moved_channels.insert(id);
            }
            moved_channels.insert(channel_id);

            let mut participants_to_remove: HashSet<UserId> = HashSet::default();
            for participant in previous_participants {
                if participant.kind == proto::channel_member::Kind::AncestorMember {
                    if !participants_to_update.contains_key(&participant.user_id) {
                        participants_to_remove.insert(participant.user_id);
                    }
                }
            }

            Ok(Some(MoveChannelResult {
                participants_to_remove,
                participants_to_update,
                moved_channels,
            }))
        })
        .await
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
enum QueryIds {
    Id,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
enum QueryUserIds {
    UserId,
}
