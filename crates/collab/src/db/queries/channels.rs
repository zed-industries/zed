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
        Ok(self.create_channel(name, None, creator_id).await?.0.id)
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
            .0
            .id)
    }

    /// Creates a new channel.
    pub async fn create_channel(
        &self,
        name: &str,
        parent_channel_id: Option<ChannelId>,
        admin_id: UserId,
    ) -> Result<(
        Channel,
        Option<channel_member::Model>,
        Vec<channel_member::Model>,
    )> {
        let name = Self::sanitize_channel_name(name)?;
        self.transaction(move |tx| async move {
            let mut parent = None;
            let mut membership = None;

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

            if parent.is_none() {
                membership = Some(
                    channel_member::ActiveModel {
                        id: ActiveValue::NotSet,
                        channel_id: ActiveValue::Set(channel.id),
                        user_id: ActiveValue::Set(admin_id),
                        accepted: ActiveValue::Set(true),
                        role: ActiveValue::Set(ChannelRole::Admin),
                    }
                    .insert(&*tx)
                    .await?,
                );
            }

            let channel_members = channel_member::Entity::find()
                .filter(channel_member::Column::ChannelId.eq(channel.root_id()))
                .all(&*tx)
                .await?;

            Ok((Channel::from_model(channel), membership, channel_members))
        })
        .await
    }

    /// Adds a user to the specified channel.
    pub async fn join_channel(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        connection: ConnectionId,
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
                    channel_member::Entity::insert(channel_member::ActiveModel {
                        id: ActiveValue::NotSet,
                        channel_id: ActiveValue::Set(channel.root_id()),
                        user_id: ActiveValue::Set(user_id),
                        accepted: ActiveValue::Set(true),
                        role: ActiveValue::Set(ChannelRole::Guest),
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
                }
            }

            if role.is_none() || role == Some(ChannelRole::Banned) {
                Err(ErrorCode::Forbidden.anyhow())?
            }
            let role = role.unwrap();

            let live_kit_room = format!("channel-{}", nanoid::nanoid!(30));
            let room_id = self
                .get_or_create_channel_room(channel_id, &live_kit_room, &*tx)
                .await?;

            self.join_channel_room_internal(room_id, user_id, connection, role, &*tx)
                .await
                .map(|jr| (jr, accept_invite_result, role))
        })
        .await
    }

    /// Sets the visibility of the given channel.
    pub async fn set_channel_visibility(
        &self,
        channel_id: ChannelId,
        visibility: ChannelVisibility,
        admin_id: UserId,
    ) -> Result<(Channel, Vec<channel_member::Model>)> {
        self.transaction(move |tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            self.check_user_is_channel_admin(&channel, admin_id, &*tx)
                .await?;

            if visibility == ChannelVisibility::Public {
                if let Some(parent_id) = channel.parent_id() {
                    let parent = self.get_channel_internal(parent_id, &*tx).await?;

                    if parent.visibility != ChannelVisibility::Public {
                        Err(ErrorCode::BadPublicNesting
                            .with_tag("direction", "parent")
                            .anyhow())?;
                    }
                }
            } else if visibility == ChannelVisibility::Members {
                if self
                    .get_channel_descendants_excluding_self([&channel], &*tx)
                    .await?
                    .into_iter()
                    .any(|channel| channel.visibility == ChannelVisibility::Public)
                {
                    Err(ErrorCode::BadPublicNesting
                        .with_tag("direction", "children")
                        .anyhow())?;
                }
            }

            let mut model = channel.into_active_model();
            model.visibility = ActiveValue::Set(visibility);
            let channel = model.update(&*tx).await?;

            let channel_members = channel_member::Entity::find()
                .filter(channel_member::Column::ChannelId.eq(channel.root_id()))
                .all(&*tx)
                .await?;

            Ok((Channel::from_model(channel), channel_members))
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
                .filter(channel_member::Column::ChannelId.eq(channel.root_id()))
                .select_only()
                .column(channel_member::Column::UserId)
                .distinct()
                .into_values::<_, QueryUserIds>()
                .all(&*tx)
                .await?;

            let channels_to_remove = self
                .get_channel_descendants_excluding_self([&channel], &*tx)
                .await?
                .into_iter()
                .map(|channel| channel.id)
                .chain(Some(channel_id))
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
            if !channel.is_root() {
                Err(ErrorCode::NotARootChannel.anyhow())?
            }

            channel_member::ActiveModel {
                id: ActiveValue::NotSet,
                channel_id: ActiveValue::Set(channel_id),
                user_id: ActiveValue::Set(invitee_id),
                accepted: ActiveValue::Set(false),
                role: ActiveValue::Set(role),
            }
            .insert(&*tx)
            .await?;

            let channel = Channel::from_model(channel);

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
    ) -> Result<(Channel, Vec<channel_member::Model>)> {
        self.transaction(move |tx| async move {
            let new_name = Self::sanitize_channel_name(new_name)?.to_string();

            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            self.check_user_is_channel_admin(&channel, admin_id, &*tx)
                .await?;

            let mut model = channel.into_active_model();
            model.name = ActiveValue::Set(new_name.clone());
            let channel = model.update(&*tx).await?;

            let channel_members = channel_member::Entity::find()
                .filter(channel_member::Column::ChannelId.eq(channel.root_id()))
                .all(&*tx)
                .await?;

            Ok((Channel::from_model(channel), channel_members))
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
            .get_channel_descendants_excluding_self([channel], &*tx)
            .await?
            .into_iter()
            .map(|channel| channel.id)
            .chain([channel.id])
            .filter(|channel_id| !new_channels.channels.iter().any(|c| c.id == *channel_id))
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

            if member_id != admin_id {
                self.check_user_is_channel_admin(&channel, admin_id, &*tx)
                    .await?;
            }

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
                .filter_map(|channel| Some(Channel::from_model(channel)))
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
        let mut filter = channel_member::Column::UserId
            .eq(user_id)
            .and(channel_member::Column::Accepted.eq(true));

        if let Some(ancestor) = ancestor_channel {
            filter = filter.and(channel_member::Column::ChannelId.eq(ancestor.root_id()));
        }

        let channel_memberships = channel_member::Entity::find()
            .filter(filter)
            .all(&*tx)
            .await?;

        let channels = channel::Entity::find()
            .filter(channel::Column::Id.is_in(channel_memberships.iter().map(|m| m.channel_id)))
            .all(&*tx)
            .await?;

        let mut descendants = self
            .get_channel_descendants_excluding_self(channels.iter(), &*tx)
            .await?;

        for channel in channels {
            if let Err(ix) = descendants.binary_search_by_key(&channel.path(), |c| c.path()) {
                descendants.insert(ix, channel);
            }
        }

        let roles_by_channel_id = channel_memberships
            .iter()
            .map(|membership| (membership.channel_id, membership.role))
            .collect::<HashMap<_, _>>();

        let channels: Vec<Channel> = descendants
            .into_iter()
            .filter_map(|channel| {
                let parent_role = roles_by_channel_id.get(&channel.root_id())?;
                if parent_role.can_see_channel(channel.visibility) {
                    Some(Channel::from_model(channel))
                } else {
                    None
                }
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

        let mut channel_ids_by_buffer_id = HashMap::default();
        let mut rows = buffer::Entity::find()
            .filter(buffer::Column::ChannelId.is_in(channel_ids.iter().copied()))
            .stream(&*tx)
            .await?;
        while let Some(row) = rows.next().await {
            let row = row?;
            channel_ids_by_buffer_id.insert(row.id, row.channel_id);
        }
        drop(rows);

        let latest_buffer_versions = self
            .latest_channel_buffer_changes(&channel_ids_by_buffer_id, &*tx)
            .await?;

        let latest_channel_messages = self.latest_channel_messages(&channel_ids, &*tx).await?;

        let observed_buffer_versions = self
            .observed_channel_buffer_changes(&channel_ids_by_buffer_id, user_id, &*tx)
            .await?;

        let observed_channel_messages = self
            .observed_channel_messages(&channel_ids, user_id, &*tx)
            .await?;

        let hosted_projects = self
            .get_hosted_projects(&channel_ids, &roles_by_channel_id, &*tx)
            .await?;

        Ok(ChannelsForUser {
            channel_memberships,
            channels,
            hosted_projects,
            channel_participants,
            latest_buffer_versions,
            latest_channel_messages,
            observed_buffer_versions,
            observed_channel_messages,
        })
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
                    channel,
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
                .map(|channel_member| proto::ChannelMember {
                    role: channel_member.role.into(),
                    user_id: channel_member.user_id.to_proto(),
                    kind: if channel_member.accepted {
                        Kind::Member
                    } else {
                        Kind::Invitee
                    }
                    .into(),
                })
                .collect())
        } else {
            return Ok(members
                .into_iter()
                .filter_map(|member| {
                    if !member.accepted {
                        return None;
                    }
                    Some(proto::ChannelMember {
                        role: member.role.into(),
                        user_id: member.user_id.to_proto(),
                        kind: Kind::Member.into(),
                    })
                })
                .collect());
        }
    }

    async fn get_channel_participant_details_internal(
        &self,
        channel: &channel::Model,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<channel_member::Model>> {
        Ok(channel_member::Entity::find()
            .filter(channel_member::Column::ChannelId.eq(channel.root_id()))
            .all(tx)
            .await?)
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
            | Some(ChannelRole::Talker)
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
            Some(ChannelRole::Banned)
            | Some(ChannelRole::Guest)
            | Some(ChannelRole::Talker)
            | None => Err(anyhow!(
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
            Some(ChannelRole::Admin)
            | Some(ChannelRole::Member)
            | Some(ChannelRole::Guest)
            | Some(ChannelRole::Talker) => Ok(role.unwrap()),
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
            .filter(channel_member::Column::ChannelId.eq(channel.root_id()))
            .filter(channel_member::Column::UserId.eq(user_id))
            .filter(channel_member::Column::Accepted.eq(false))
            .one(&*tx)
            .await?;

        Ok(row)
    }

    /// Returns the role for a user in the given channel.
    pub async fn channel_role_for_user(
        &self,
        channel: &channel::Model,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<Option<ChannelRole>> {
        let membership = channel_member::Entity::find()
            .filter(
                channel_member::Column::ChannelId
                    .eq(channel.root_id())
                    .and(channel_member::Column::UserId.eq(user_id))
                    .and(channel_member::Column::Accepted.eq(true)),
            )
            .one(&*tx)
            .await?;

        let Some(membership) = membership else {
            return Ok(None);
        };

        if !membership.role.can_see_channel(channel.visibility) {
            return Ok(None);
        }

        Ok(Some(membership.role))
    }

    // Get the descendants of the given set if channels, ordered by their
    // path.
    pub(crate) async fn get_channel_descendants_excluding_self(
        &self,
        channels: impl IntoIterator<Item = &channel::Model>,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<channel::Model>> {
        let mut filter = Condition::any();
        for channel in channels.into_iter() {
            filter = filter.add(channel::Column::ParentPath.like(channel.descendant_path_filter()));
        }

        if filter.is_empty() {
            return Ok(vec![]);
        }

        Ok(channel::Entity::find()
            .filter(filter)
            .order_by_asc(Expr::cust("parent_path || id || '/'"))
            .all(tx)
            .await?)
    }

    /// Returns the channel with the given ID.
    pub async fn get_channel(&self, channel_id: ChannelId, user_id: UserId) -> Result<Channel> {
        self.transaction(|tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            self.check_user_is_channel_participant(&channel, user_id, &*tx)
                .await?;

            Ok(Channel::from_model(channel))
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
        tx: &DatabaseTransaction,
    ) -> Result<RoomId> {
        let room = room::Entity::find()
            .filter(room::Column::ChannelId.eq(channel_id))
            .one(&*tx)
            .await?;

        let room_id = if let Some(room) = room {
            room.id
        } else {
            let result = room::Entity::insert(room::ActiveModel {
                channel_id: ActiveValue::Set(Some(channel_id)),
                live_kit_room: ActiveValue::Set(live_kit_room.to_string()),
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
        new_parent_id: ChannelId,
        admin_id: UserId,
    ) -> Result<(Vec<Channel>, Vec<channel_member::Model>)> {
        self.transaction(|tx| async move {
            let channel = self.get_channel_internal(channel_id, &*tx).await?;
            self.check_user_is_channel_admin(&channel, admin_id, &*tx)
                .await?;
            let new_parent = self.get_channel_internal(new_parent_id, &*tx).await?;

            if new_parent.root_id() != channel.root_id() {
                Err(anyhow!(ErrorCode::WrongMoveTarget))?;
            }

            if new_parent
                .ancestors_including_self()
                .any(|id| id == channel.id)
            {
                Err(anyhow!(ErrorCode::CircularNesting))?;
            }

            if channel.visibility == ChannelVisibility::Public
                && new_parent.visibility != ChannelVisibility::Public
            {
                Err(anyhow!(ErrorCode::BadPublicNesting))?;
            }

            let root_id = channel.root_id();
            let old_path = format!("{}{}/", channel.parent_path, channel.id);
            let new_path = format!("{}{}/", new_parent.path(), channel.id);

            let mut model = channel.into_active_model();
            model.parent_path = ActiveValue::Set(new_parent.path());
            let channel = model.update(&*tx).await?;

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

            let all_moved_ids = Some(channel.id).into_iter().chain(descendent_ids);

            let channels = channel::Entity::find()
                .filter(channel::Column::Id.is_in(all_moved_ids))
                .all(&*tx)
                .await?
                .into_iter()
                .map(|c| Channel::from_model(c))
                .collect::<Vec<_>>();

            let channel_members = channel_member::Entity::find()
                .filter(channel_member::Column::ChannelId.eq(root_id))
                .all(&*tx)
                .await?;

            Ok((channels, channel_members))
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
