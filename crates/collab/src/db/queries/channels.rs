use std::cmp::Ordering;

use super::*;
use rpc::proto::{channel_member::Kind, ChannelEdge};
use smallvec::SmallVec;

type ChannelDescendants = HashMap<ChannelId, SmallSet<ChannelId>>;

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

    pub async fn create_root_channel(&self, name: &str, creator_id: UserId) -> Result<ChannelId> {
        self.create_channel(name, None, creator_id).await
    }

    pub async fn create_channel(
        &self,
        name: &str,
        parent: Option<ChannelId>,
        creator_id: UserId,
    ) -> Result<ChannelId> {
        let name = Self::sanitize_channel_name(name)?;
        self.transaction(move |tx| async move {
            if let Some(parent) = parent {
                self.check_user_is_channel_admin(parent, creator_id, &*tx)
                    .await?;
            }

            let channel = channel::ActiveModel {
                id: ActiveValue::NotSet,
                name: ActiveValue::Set(name.to_string()),
                visibility: ActiveValue::Set(ChannelVisibility::ChannelMembers),
            }
            .insert(&*tx)
            .await?;

            if let Some(parent) = parent {
                let sql = r#"
                    INSERT INTO channel_paths
                    (id_path, channel_id)
                    SELECT
                        id_path || $1 || '/', $2
                    FROM
                        channel_paths
                    WHERE
                        channel_id = $3
                "#;
                let channel_paths_stmt = Statement::from_sql_and_values(
                    self.pool.get_database_backend(),
                    sql,
                    [
                        channel.id.to_proto().into(),
                        channel.id.to_proto().into(),
                        parent.to_proto().into(),
                    ],
                );
                tx.execute(channel_paths_stmt).await?;
            } else {
                channel_path::Entity::insert(channel_path::ActiveModel {
                    channel_id: ActiveValue::Set(channel.id),
                    id_path: ActiveValue::Set(format!("/{}/", channel.id)),
                })
                .exec(&*tx)
                .await?;
            }

            channel_member::ActiveModel {
                id: ActiveValue::NotSet,
                channel_id: ActiveValue::Set(channel.id),
                user_id: ActiveValue::Set(creator_id),
                accepted: ActiveValue::Set(true),
                admin: ActiveValue::Set(true),
                role: ActiveValue::Set(Some(ChannelRole::Admin)),
            }
            .insert(&*tx)
            .await?;

            Ok(channel.id)
        })
        .await
    }

    pub async fn set_channel_visibility(
        &self,
        channel_id: ChannelId,
        visibility: ChannelVisibility,
        user_id: UserId,
    ) -> Result<()> {
        self.transaction(move |tx| async move {
            self.check_user_is_channel_admin(channel_id, user_id, &*tx)
                .await?;

            channel::ActiveModel {
                id: ActiveValue::Unchanged(channel_id),
                visibility: ActiveValue::Set(visibility),
                ..Default::default()
            }
            .update(&*tx)
            .await?;

            Ok(())
        })
        .await
    }

    pub async fn delete_channel(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> Result<(Vec<ChannelId>, Vec<UserId>)> {
        self.transaction(move |tx| async move {
            self.check_user_is_channel_admin(channel_id, user_id, &*tx)
                .await?;

            // Don't remove descendant channels that have additional parents.
            let mut channels_to_remove: HashSet<ChannelId> = HashSet::default();
            channels_to_remove.insert(channel_id);

            let graph = self.get_channel_descendants_2([channel_id], &*tx).await?;
            for edge in graph.iter() {
                channels_to_remove.insert(ChannelId::from_proto(edge.channel_id));
            }

            {
                let mut channels_to_keep = channel_path::Entity::find()
                    .filter(
                        channel_path::Column::ChannelId
                            .is_in(channels_to_remove.clone())
                            .and(
                                channel_path::Column::IdPath
                                    .not_like(&format!("%/{}/%", channel_id)),
                            ),
                    )
                    .stream(&*tx)
                    .await?;
                while let Some(row) = channels_to_keep.next().await {
                    let row = row?;
                    channels_to_remove.remove(&row.channel_id);
                }
            }

            let channel_ancestors = self.get_channel_ancestors(channel_id, &*tx).await?;
            let members_to_notify: Vec<UserId> = channel_member::Entity::find()
                .filter(channel_member::Column::ChannelId.is_in(channel_ancestors))
                .select_only()
                .column(channel_member::Column::UserId)
                .distinct()
                .into_values::<_, QueryUserIds>()
                .all(&*tx)
                .await?;

            channel::Entity::delete_many()
                .filter(channel::Column::Id.is_in(channels_to_remove.clone()))
                .exec(&*tx)
                .await?;

            // Delete any other paths that include this channel
            let sql = r#"
                    DELETE FROM channel_paths
                    WHERE
                        id_path LIKE '%' || $1 || '%'
                "#;
            let channel_paths_stmt = Statement::from_sql_and_values(
                self.pool.get_database_backend(),
                sql,
                [channel_id.to_proto().into()],
            );
            tx.execute(channel_paths_stmt).await?;

            Ok((channels_to_remove.into_iter().collect(), members_to_notify))
        })
        .await
    }

    pub async fn invite_channel_member(
        &self,
        channel_id: ChannelId,
        invitee_id: UserId,
        admin_id: UserId,
        role: ChannelRole,
    ) -> Result<()> {
        self.transaction(move |tx| async move {
            self.check_user_is_channel_admin(channel_id, admin_id, &*tx)
                .await?;

            channel_member::ActiveModel {
                id: ActiveValue::NotSet,
                channel_id: ActiveValue::Set(channel_id),
                user_id: ActiveValue::Set(invitee_id),
                accepted: ActiveValue::Set(false),
                admin: ActiveValue::Set(role == ChannelRole::Admin),
                role: ActiveValue::Set(Some(role)),
            }
            .insert(&*tx)
            .await?;

            Ok(())
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

    pub async fn rename_channel(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        new_name: &str,
    ) -> Result<String> {
        self.transaction(move |tx| async move {
            let new_name = Self::sanitize_channel_name(new_name)?.to_string();

            self.check_user_is_channel_admin(channel_id, user_id, &*tx)
                .await?;

            channel::ActiveModel {
                id: ActiveValue::Unchanged(channel_id),
                name: ActiveValue::Set(new_name.clone()),
                ..Default::default()
            }
            .update(&*tx)
            .await?;

            Ok(new_name)
        })
        .await
    }

    pub async fn respond_to_channel_invite(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        accept: bool,
    ) -> Result<()> {
        self.transaction(move |tx| async move {
            let rows_affected = if accept {
                channel_member::Entity::update_many()
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
                    .rows_affected
            } else {
                channel_member::ActiveModel {
                    channel_id: ActiveValue::Unchanged(channel_id),
                    user_id: ActiveValue::Unchanged(user_id),
                    ..Default::default()
                }
                .delete(&*tx)
                .await?
                .rows_affected
            };

            if rows_affected == 0 {
                Err(anyhow!("no such invitation"))?;
            }

            Ok(())
        })
        .await
    }

    pub async fn remove_channel_member(
        &self,
        channel_id: ChannelId,
        member_id: UserId,
        admin_id: UserId,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            self.check_user_is_channel_admin(channel_id, admin_id, &*tx)
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

            Ok(())
        })
        .await
    }

    pub async fn get_channel_invites_for_user(&self, user_id: UserId) -> Result<Vec<Channel>> {
        self.transaction(|tx| async move {
            let channel_invites = channel_member::Entity::find()
                .filter(
                    channel_member::Column::UserId
                        .eq(user_id)
                        .and(channel_member::Column::Accepted.eq(false)),
                )
                .all(&*tx)
                .await?;

            let channels = channel::Entity::find()
                .filter(
                    channel::Column::Id.is_in(
                        channel_invites
                            .into_iter()
                            .map(|channel_member| channel_member.channel_id),
                    ),
                )
                .all(&*tx)
                .await?;

            let channels = channels
                .into_iter()
                .map(|channel| Channel {
                    id: channel.id,
                    name: channel.name,
                })
                .collect();

            Ok(channels)
        })
        .await
    }

    async fn get_channel_graph(
        &self,
        parents_by_child_id: ChannelDescendants,
        trim_dangling_parents: bool,
        tx: &DatabaseTransaction,
    ) -> Result<ChannelGraph> {
        let mut channels = Vec::with_capacity(parents_by_child_id.len());
        {
            let mut rows = channel::Entity::find()
                .filter(channel::Column::Id.is_in(parents_by_child_id.keys().copied()))
                .stream(&*tx)
                .await?;
            while let Some(row) = rows.next().await {
                let row = row?;
                channels.push(Channel {
                    id: row.id,
                    name: row.name,
                })
            }
        }

        let mut edges = Vec::with_capacity(parents_by_child_id.len());
        for (channel, parents) in parents_by_child_id.iter() {
            for parent in parents.into_iter() {
                if trim_dangling_parents {
                    if parents_by_child_id.contains_key(parent) {
                        edges.push(ChannelEdge {
                            channel_id: channel.to_proto(),
                            parent_id: parent.to_proto(),
                        });
                    }
                } else {
                    edges.push(ChannelEdge {
                        channel_id: channel.to_proto(),
                        parent_id: parent.to_proto(),
                    });
                }
            }
        }

        Ok(ChannelGraph { channels, edges })
    }

    pub async fn get_channels_for_user(&self, user_id: UserId) -> Result<ChannelsForUser> {
        self.transaction(|tx| async move {
            let tx = tx;

            let channel_memberships = channel_member::Entity::find()
                .filter(
                    channel_member::Column::UserId
                        .eq(user_id)
                        .and(channel_member::Column::Accepted.eq(true)),
                )
                .all(&*tx)
                .await?;

            self.get_user_channels(user_id, channel_memberships, &tx)
                .await
        })
        .await
    }

    pub async fn get_channel_for_user(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> Result<ChannelsForUser> {
        self.transaction(|tx| async move {
            let tx = tx;

            let channel_membership = channel_member::Entity::find()
                .filter(
                    channel_member::Column::UserId
                        .eq(user_id)
                        .and(channel_member::Column::ChannelId.eq(channel_id))
                        .and(channel_member::Column::Accepted.eq(true)),
                )
                .all(&*tx)
                .await?;

            self.get_user_channels(user_id, channel_membership, &tx)
                .await
        })
        .await
    }

    pub async fn get_user_channels(
        &self,
        user_id: UserId,
        channel_memberships: Vec<channel_member::Model>,
        tx: &DatabaseTransaction,
    ) -> Result<ChannelsForUser> {
        let mut edges = self
            .get_channel_descendants_2(channel_memberships.iter().map(|m| m.channel_id), &*tx)
            .await?;

        let mut role_for_channel: HashMap<ChannelId, ChannelRole> = HashMap::default();

        for membership in channel_memberships.iter() {
            role_for_channel.insert(
                membership.channel_id,
                membership.role.unwrap_or(if membership.admin {
                    ChannelRole::Admin
                } else {
                    ChannelRole::Member
                }),
            );
        }

        for ChannelEdge {
            parent_id,
            channel_id,
        } in edges.iter()
        {
            let parent_id = ChannelId::from_proto(*parent_id);
            let channel_id = ChannelId::from_proto(*channel_id);
            debug_assert!(role_for_channel.get(&parent_id).is_some());
            let parent_role = role_for_channel[&parent_id];
            if let Some(existing_role) = role_for_channel.get(&channel_id) {
                if existing_role.should_override(parent_role) {
                    continue;
                }
            }
            role_for_channel.insert(channel_id, parent_role);
        }

        let mut channels: Vec<Channel> = Vec::new();
        let mut channels_with_admin_privileges: HashSet<ChannelId> = HashSet::default();
        let mut channels_to_remove: HashSet<u64> = HashSet::default();

        let mut rows = channel::Entity::find()
            .filter(channel::Column::Id.is_in(role_for_channel.keys().cloned()))
            .stream(&*tx)
            .await?;

        while let Some(row) = rows.next().await {
            let channel = row?;
            let role = role_for_channel[&channel.id];

            if role == ChannelRole::Banned
                || role == ChannelRole::Guest && channel.visibility != ChannelVisibility::Public
            {
                channels_to_remove.insert(channel.id.0 as u64);
                continue;
            }

            channels.push(Channel {
                id: channel.id,
                name: channel.name,
            });

            if role == ChannelRole::Admin {
                channels_with_admin_privileges.insert(channel.id);
            }
        }
        drop(rows);

        if !channels_to_remove.is_empty() {
            // Note: this code assumes each channel has one parent.
            let mut replacement_parent: HashMap<u64, u64> = HashMap::default();
            for ChannelEdge {
                parent_id,
                channel_id,
            } in edges.iter()
            {
                if channels_to_remove.contains(channel_id) {
                    replacement_parent.insert(*channel_id, *parent_id);
                }
            }

            let mut new_edges: Vec<ChannelEdge> = Vec::new();
            'outer: for ChannelEdge {
                mut parent_id,
                channel_id,
            } in edges.iter()
            {
                if channels_to_remove.contains(channel_id) {
                    continue;
                }
                while channels_to_remove.contains(&parent_id) {
                    if let Some(new_parent_id) = replacement_parent.get(&parent_id) {
                        parent_id = *new_parent_id;
                    } else {
                        continue 'outer;
                    }
                }
                new_edges.push(ChannelEdge {
                    parent_id,
                    channel_id: *channel_id,
                })
            }
            edges = new_edges;
        }

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
            channels: ChannelGraph { channels, edges },
            channel_participants,
            channels_with_admin_privileges,
            unseen_buffer_changes: channel_buffer_changes,
            channel_messages: unseen_messages,
        })
    }

    pub async fn get_channel_members(&self, id: ChannelId) -> Result<Vec<UserId>> {
        self.transaction(|tx| async move { self.get_channel_participants_internal(id, &*tx).await })
            .await
    }

    pub async fn set_channel_member_role(
        &self,
        channel_id: ChannelId,
        admin_id: UserId,
        for_user: UserId,
        role: ChannelRole,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            self.check_user_is_channel_admin(channel_id, admin_id, &*tx)
                .await?;

            let result = channel_member::Entity::update_many()
                .filter(
                    channel_member::Column::ChannelId
                        .eq(channel_id)
                        .and(channel_member::Column::UserId.eq(for_user)),
                )
                .set(channel_member::ActiveModel {
                    admin: ActiveValue::set(role == ChannelRole::Admin),
                    role: ActiveValue::set(Some(role)),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;

            if result.rows_affected == 0 {
                Err(anyhow!("no such member"))?;
            }

            Ok(())
        })
        .await
    }

    pub async fn get_channel_participant_details(
        &self,
        channel_id: ChannelId,
        admin_id: UserId,
    ) -> Result<Vec<proto::ChannelMember>> {
        self.transaction(|tx| async move {
            self.check_user_is_channel_admin(channel_id, admin_id, &*tx)
                .await?;

            let channel_visibility = channel::Entity::find()
                .filter(channel::Column::Id.eq(channel_id))
                .one(&*tx)
                .await?
                .map(|channel| channel.visibility)
                .unwrap_or(ChannelVisibility::ChannelMembers);

            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            enum QueryMemberDetails {
                UserId,
                Admin,
                Role,
                IsDirectMember,
                Accepted,
                Visibility,
            }

            let tx = tx;
            let ancestor_ids = self.get_channel_ancestors(channel_id, &*tx).await?;
            let mut stream = channel_member::Entity::find()
                .left_join(channel::Entity)
                .filter(channel_member::Column::ChannelId.is_in(ancestor_ids.iter().copied()))
                .select_only()
                .column(channel_member::Column::UserId)
                .column(channel_member::Column::Admin)
                .column(channel_member::Column::Role)
                .column_as(
                    channel_member::Column::ChannelId.eq(channel_id),
                    QueryMemberDetails::IsDirectMember,
                )
                .column(channel_member::Column::Accepted)
                .column(channel::Column::Visibility)
                .into_values::<_, QueryMemberDetails>()
                .stream(&*tx)
                .await?;

            struct UserDetail {
                kind: Kind,
                channel_role: ChannelRole,
            }
            let mut user_details: HashMap<UserId, UserDetail> = HashMap::default();

            while let Some(row) = stream.next().await {
                let (
                    user_id,
                    is_admin,
                    channel_role,
                    is_direct_member,
                    is_invite_accepted,
                    visibility,
                ): (
                    UserId,
                    bool,
                    Option<ChannelRole>,
                    bool,
                    bool,
                    ChannelVisibility,
                ) = row?;
                let kind = match (is_direct_member, is_invite_accepted) {
                    (true, true) => proto::channel_member::Kind::Member,
                    (true, false) => proto::channel_member::Kind::Invitee,
                    (false, true) => proto::channel_member::Kind::AncestorMember,
                    (false, false) => continue,
                };
                let channel_role = channel_role.unwrap_or(if is_admin {
                    ChannelRole::Admin
                } else {
                    ChannelRole::Member
                });

                if channel_role == ChannelRole::Guest
                    && visibility != ChannelVisibility::Public
                    && channel_visibility != ChannelVisibility::Public
                {
                    continue;
                }

                if let Some(details_mut) = user_details.get_mut(&user_id) {
                    if channel_role.should_override(details_mut.channel_role) {
                        details_mut.channel_role = channel_role;
                    }
                    if kind == Kind::Member {
                        details_mut.kind = kind;
                    // the UI is going to be a bit confusing if you already have permissions
                    // that are greater than or equal to the ones you're being invited to.
                    } else if kind == Kind::Invitee && details_mut.kind == Kind::AncestorMember {
                        details_mut.kind = kind;
                    }
                } else {
                    user_details.insert(user_id, UserDetail { kind, channel_role });
                }
            }

            // sort by permissions descending, within each section, show members, then ancestor members, then invitees.
            let mut results: Vec<(UserId, UserDetail)> = user_details.into_iter().collect();
            results.sort_by(|a, b| {
                if a.1.channel_role.should_override(b.1.channel_role) {
                    return Ordering::Less;
                } else if b.1.channel_role.should_override(a.1.channel_role) {
                    return Ordering::Greater;
                }

                if a.1.kind == Kind::Member && b.1.kind != Kind::Member {
                    return Ordering::Less;
                } else if b.1.kind == Kind::Member && a.1.kind != Kind::Member {
                    return Ordering::Greater;
                }

                if a.1.kind == Kind::AncestorMember && b.1.kind != Kind::AncestorMember {
                    return Ordering::Less;
                } else if b.1.kind == Kind::AncestorMember && a.1.kind != Kind::AncestorMember {
                    return Ordering::Greater;
                }

                // would be nice to sort alphabetically instead of by user id.
                // (or defer all sorting to the UI, but we need something to help the tests)
                return a.0.cmp(&b.0);
            });

            Ok(results
                .into_iter()
                .map(|(user_id, details)| proto::ChannelMember {
                    user_id: user_id.to_proto(),
                    kind: details.kind.into(),
                    role: details.channel_role.into(),
                })
                .collect())
        })
        .await
    }

    pub async fn get_channel_participants_internal(
        &self,
        id: ChannelId,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<UserId>> {
        let ancestor_ids = self.get_channel_ancestors(id, tx).await?;
        let user_ids = channel_member::Entity::find()
            .distinct()
            .filter(
                channel_member::Column::ChannelId
                    .is_in(ancestor_ids.iter().copied())
                    .and(channel_member::Column::Accepted.eq(true)),
            )
            .select_only()
            .column(channel_member::Column::UserId)
            .into_values::<_, QueryUserIds>()
            .all(&*tx)
            .await?;
        Ok(user_ids)
    }

    pub async fn check_user_is_channel_admin(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        match self.channel_role_for_user(channel_id, user_id, tx).await? {
            Some(ChannelRole::Admin) => Ok(()),
            Some(ChannelRole::Member)
            | Some(ChannelRole::Banned)
            | Some(ChannelRole::Guest)
            | None => Err(anyhow!(
                "user is not a channel admin or channel does not exist"
            ))?,
        }
    }

    pub async fn check_user_is_channel_member(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        match self.channel_role_for_user(channel_id, user_id, tx).await? {
            Some(ChannelRole::Admin) | Some(ChannelRole::Member) => Ok(()),
            Some(ChannelRole::Banned) | Some(ChannelRole::Guest) | None => Err(anyhow!(
                "user is not a channel member or channel does not exist"
            ))?,
        }
    }

    pub async fn check_user_is_channel_participant(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        match self.channel_role_for_user(channel_id, user_id, tx).await? {
            Some(ChannelRole::Admin) | Some(ChannelRole::Member) | Some(ChannelRole::Guest) => {
                Ok(())
            }
            Some(ChannelRole::Banned) | None => Err(anyhow!(
                "user is not a channel participant or channel does not exist"
            ))?,
        }
    }

    pub async fn channel_role_for_user(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<Option<ChannelRole>> {
        let channel_ids = self.get_channel_ancestors(channel_id, tx).await?;

        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryChannelMembership {
            ChannelId,
            Role,
            Admin,
            Visibility,
        }

        let mut rows = channel_member::Entity::find()
            .left_join(channel::Entity)
            .filter(
                channel_member::Column::ChannelId
                    .is_in(channel_ids)
                    .and(channel_member::Column::UserId.eq(user_id)),
            )
            .select_only()
            .column(channel_member::Column::ChannelId)
            .column(channel_member::Column::Role)
            .column(channel_member::Column::Admin)
            .column(channel::Column::Visibility)
            .into_values::<_, QueryChannelMembership>()
            .stream(&*tx)
            .await?;

        let mut is_admin = false;
        let mut is_member = false;
        let mut is_participant = false;
        let mut is_banned = false;
        let mut current_channel_visibility = None;

        // note these channels are not iterated in any particular order,
        // our current logic takes the highest permission available.
        while let Some(row) = rows.next().await {
            let (ch_id, role, admin, visibility): (
                ChannelId,
                Option<ChannelRole>,
                bool,
                ChannelVisibility,
            ) = row?;
            match role {
                Some(ChannelRole::Admin) => is_admin = true,
                Some(ChannelRole::Member) => is_member = true,
                Some(ChannelRole::Guest) => {
                    if visibility == ChannelVisibility::Public {
                        is_participant = true
                    }
                }
                Some(ChannelRole::Banned) => is_banned = true,
                None => {
                    // rows created from pre-role collab server.
                    if admin {
                        is_admin = true
                    } else {
                        is_member = true
                    }
                }
            }
            if channel_id == ch_id {
                current_channel_visibility = Some(visibility);
            }
        }
        // free up database connection
        drop(rows);

        Ok(if is_admin {
            Some(ChannelRole::Admin)
        } else if is_member {
            Some(ChannelRole::Member)
        } else if is_banned {
            Some(ChannelRole::Banned)
        } else if is_participant {
            if current_channel_visibility.is_none() {
                current_channel_visibility = channel::Entity::find()
                    .filter(channel::Column::Id.eq(channel_id))
                    .one(&*tx)
                    .await?
                    .map(|channel| channel.visibility);
            }
            if current_channel_visibility == Some(ChannelVisibility::Public) {
                Some(ChannelRole::Guest)
            } else {
                None
            }
        } else {
            None
        })
    }

    /// Returns the channel ancestors, include itself, deepest first
    pub async fn get_channel_ancestors(
        &self,
        channel_id: ChannelId,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<ChannelId>> {
        let paths = channel_path::Entity::find()
            .filter(channel_path::Column::ChannelId.eq(channel_id))
            .order_by(channel_path::Column::IdPath, sea_orm::Order::Desc)
            .all(tx)
            .await?;
        let mut channel_ids = Vec::new();
        for path in paths {
            for id in path.id_path.trim_matches('/').split('/') {
                if let Ok(id) = id.parse() {
                    let id = ChannelId::from_proto(id);
                    if let Err(ix) = channel_ids.binary_search(&id) {
                        channel_ids.insert(ix, id);
                    }
                }
            }
        }
        Ok(channel_ids)
    }

    // Returns the channel desendants as a sorted list of edges for further processing.
    // The edges are sorted such that you will see unknown channel ids as children
    // before you see them as parents.
    async fn get_channel_descendants_2(
        &self,
        channel_ids: impl IntoIterator<Item = ChannelId>,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<ChannelEdge>> {
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
            SELECT
                descendant_paths.*
            FROM
                channel_paths parent_paths, channel_paths descendant_paths
            WHERE
                parent_paths.channel_id IN ({values}) AND
                descendant_paths.id_path != parent_paths.id_path AND
                descendant_paths.id_path LIKE (parent_paths.id_path || '%')
            ORDER BY
                descendant_paths.id_path
        "#
        );

        let stmt = Statement::from_string(self.pool.get_database_backend(), sql);

        let mut paths = channel_path::Entity::find()
            .from_raw_sql(stmt)
            .stream(tx)
            .await?;

        let mut results: Vec<ChannelEdge> = Vec::new();
        while let Some(path) = paths.next().await {
            let path = path?;
            let ids: Vec<&str> = path.id_path.trim_matches('/').split('/').collect();

            debug_assert!(ids.len() >= 2);
            debug_assert!(ids[ids.len() - 1] == path.channel_id.to_string());

            results.push(ChannelEdge {
                parent_id: ids[ids.len() - 2].parse().unwrap(),
                channel_id: ids[ids.len() - 1].parse().unwrap(),
            })
        }

        Ok(results)
    }

    /// Returns the channel descendants,
    /// Structured as a map from child ids to their parent ids
    /// For example, the descendants of 'a' in this DAG:
    ///
    ///   /- b -\
    /// a -- c -- d
    ///
    /// would be:
    /// {
    ///     a: [],
    ///     b: [a],
    ///     c: [a],
    ///     d: [a, c],
    /// }
    async fn get_channel_descendants(
        &self,
        channel_ids: impl IntoIterator<Item = ChannelId>,
        tx: &DatabaseTransaction,
    ) -> Result<ChannelDescendants> {
        let mut values = String::new();
        for id in channel_ids {
            if !values.is_empty() {
                values.push_str(", ");
            }
            write!(&mut values, "({})", id).unwrap();
        }

        if values.is_empty() {
            return Ok(HashMap::default());
        }

        let sql = format!(
            r#"
            SELECT
                descendant_paths.*
            FROM
                channel_paths parent_paths, channel_paths descendant_paths
            WHERE
                parent_paths.channel_id IN ({values}) AND
                descendant_paths.id_path LIKE (parent_paths.id_path || '%')
        "#
        );

        let stmt = Statement::from_string(self.pool.get_database_backend(), sql);

        let mut parents_by_child_id: ChannelDescendants = HashMap::default();
        let mut paths = channel_path::Entity::find()
            .from_raw_sql(stmt)
            .stream(tx)
            .await?;

        while let Some(path) = paths.next().await {
            let path = path?;
            let ids = path.id_path.trim_matches('/').split('/');
            let mut parent_id = None;
            for id in ids {
                if let Ok(id) = id.parse() {
                    let id = ChannelId::from_proto(id);
                    if id == path.channel_id {
                        break;
                    }
                    parent_id = Some(id);
                }
            }
            let entry = parents_by_child_id.entry(path.channel_id).or_default();
            if let Some(parent_id) = parent_id {
                entry.insert(parent_id);
            }
        }

        Ok(parents_by_child_id)
    }

    /// Returns the channel with the given ID and:
    /// - true if the user is a member
    /// - false if the user hasn't accepted the invitation yet
    pub async fn get_channel(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> Result<Option<(Channel, bool)>> {
        self.transaction(|tx| async move {
            let tx = tx;

            let channel = channel::Entity::find_by_id(channel_id).one(&*tx).await?;

            if let Some(channel) = channel {
                if self
                    .check_user_is_channel_member(channel_id, user_id, &*tx)
                    .await
                    .is_err()
                {
                    return Ok(None);
                }

                let channel_membership = channel_member::Entity::find()
                    .filter(
                        channel_member::Column::ChannelId
                            .eq(channel_id)
                            .and(channel_member::Column::UserId.eq(user_id)),
                    )
                    .one(&*tx)
                    .await?;

                let is_accepted = channel_membership
                    .map(|membership| membership.accepted)
                    .unwrap_or(false);

                Ok(Some((
                    Channel {
                        id: channel.id,
                        name: channel.name,
                    },
                    is_accepted,
                )))
            } else {
                Ok(None)
            }
        })
        .await
    }

    pub async fn get_or_create_channel_room(
        &self,
        channel_id: ChannelId,
        live_kit_room: &str,
        enviroment: &str,
    ) -> Result<RoomId> {
        self.transaction(|tx| async move {
            let tx = tx;

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
                    enviroment: ActiveValue::Set(Some(enviroment.to_string())),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;

                result.last_insert_id
            };

            Ok(room_id)
        })
        .await
    }

    // Insert an edge from the given channel to the given other channel.
    pub async fn link_channel(
        &self,
        user: UserId,
        channel: ChannelId,
        to: ChannelId,
    ) -> Result<ChannelGraph> {
        self.transaction(|tx| async move {
            // Note that even with these maxed permissions, this linking operation
            // is still insecure because you can't remove someone's permissions to a
            // channel if they've linked the channel to one where they're an admin.
            self.check_user_is_channel_admin(channel, user, &*tx)
                .await?;

            self.link_channel_internal(user, channel, to, &*tx).await
        })
        .await
    }

    pub async fn link_channel_internal(
        &self,
        user: UserId,
        channel: ChannelId,
        new_parent: ChannelId,
        tx: &DatabaseTransaction,
    ) -> Result<ChannelGraph> {
        self.check_user_is_channel_admin(new_parent, user, &*tx)
            .await?;

        let paths = channel_path::Entity::find()
            .filter(channel_path::Column::IdPath.like(&format!("%/{}/%", channel)))
            .all(tx)
            .await?;

        let mut new_path_suffixes = HashSet::default();
        for path in paths {
            if let Some(start_offset) = path.id_path.find(&format!("/{}/", channel)) {
                new_path_suffixes.insert((
                    path.channel_id,
                    path.id_path[(start_offset + 1)..].to_string(),
                ));
            }
        }

        let paths_to_new_parent = channel_path::Entity::find()
            .filter(channel_path::Column::ChannelId.eq(new_parent))
            .all(tx)
            .await?;

        let mut new_paths = Vec::new();
        for path in paths_to_new_parent {
            if path.id_path.contains(&format!("/{}/", channel)) {
                Err(anyhow!("cycle"))?;
            }

            new_paths.extend(new_path_suffixes.iter().map(|(channel_id, path_suffix)| {
                channel_path::ActiveModel {
                    channel_id: ActiveValue::Set(*channel_id),
                    id_path: ActiveValue::Set(format!("{}{}", &path.id_path, path_suffix)),
                }
            }));
        }

        channel_path::Entity::insert_many(new_paths)
            .exec(&*tx)
            .await?;

        // remove any root edges for the channel we just linked
        {
            channel_path::Entity::delete_many()
                .filter(channel_path::Column::IdPath.like(&format!("/{}/%", channel)))
                .exec(&*tx)
                .await?;
        }

        let mut channel_descendants = self.get_channel_descendants([channel], &*tx).await?;
        if let Some(channel) = channel_descendants.get_mut(&channel) {
            // Remove the other parents
            channel.clear();
            channel.insert(new_parent);
        }

        let channels = self
            .get_channel_graph(channel_descendants, false, &*tx)
            .await?;

        Ok(channels)
    }

    /// Unlink a channel from a given parent. This will add in a root edge if
    /// the channel has no other parents after this operation.
    pub async fn unlink_channel(
        &self,
        user: UserId,
        channel: ChannelId,
        from: ChannelId,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            // Note that even with these maxed permissions, this linking operation
            // is still insecure because you can't remove someone's permissions to a
            // channel if they've linked the channel to one where they're an admin.
            self.check_user_is_channel_admin(channel, user, &*tx)
                .await?;

            self.unlink_channel_internal(user, channel, from, &*tx)
                .await?;

            Ok(())
        })
        .await
    }

    pub async fn unlink_channel_internal(
        &self,
        user: UserId,
        channel: ChannelId,
        from: ChannelId,
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        self.check_user_is_channel_admin(from, user, &*tx).await?;

        let sql = r#"
            DELETE FROM channel_paths
            WHERE
                id_path LIKE '%/' || $1 || '/' || $2 || '/%'
            RETURNING id_path, channel_id
        "#;

        let paths = channel_path::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                self.pool.get_database_backend(),
                sql,
                [from.to_proto().into(), channel.to_proto().into()],
            ))
            .all(&*tx)
            .await?;

        let is_stranded = channel_path::Entity::find()
            .filter(channel_path::Column::ChannelId.eq(channel))
            .count(&*tx)
            .await?
            == 0;

        // Make sure that there is always at least one path to the channel
        if is_stranded {
            let root_paths: Vec<_> = paths
                .iter()
                .map(|path| {
                    let start_offset = path.id_path.find(&format!("/{}/", channel)).unwrap();
                    channel_path::ActiveModel {
                        channel_id: ActiveValue::Set(path.channel_id),
                        id_path: ActiveValue::Set(path.id_path[start_offset..].to_string()),
                    }
                })
                .collect();
            channel_path::Entity::insert_many(root_paths)
                .exec(&*tx)
                .await?;
        }

        Ok(())
    }

    /// Move a channel from one parent to another, returns the
    /// Channels that were moved for notifying clients
    pub async fn move_channel(
        &self,
        user: UserId,
        channel: ChannelId,
        from: ChannelId,
        to: ChannelId,
    ) -> Result<ChannelGraph> {
        if from == to {
            return Ok(ChannelGraph {
                channels: vec![],
                edges: vec![],
            });
        }

        self.transaction(|tx| async move {
            self.check_user_is_channel_admin(channel, user, &*tx)
                .await?;

            let moved_channels = self.link_channel_internal(user, channel, to, &*tx).await?;

            self.unlink_channel_internal(user, channel, from, &*tx)
                .await?;

            Ok(moved_channels)
        })
        .await
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
enum QueryUserIds {
    UserId,
}

#[derive(Debug)]
pub struct ChannelGraph {
    pub channels: Vec<Channel>,
    pub edges: Vec<ChannelEdge>,
}

impl ChannelGraph {
    pub fn is_empty(&self) -> bool {
        self.channels.is_empty() && self.edges.is_empty()
    }
}

#[cfg(test)]
impl PartialEq for ChannelGraph {
    fn eq(&self, other: &Self) -> bool {
        // Order independent comparison for tests
        let channels_set = self.channels.iter().collect::<HashSet<_>>();
        let other_channels_set = other.channels.iter().collect::<HashSet<_>>();
        let edges_set = self
            .edges
            .iter()
            .map(|edge| (edge.channel_id, edge.parent_id))
            .collect::<HashSet<_>>();
        let other_edges_set = other
            .edges
            .iter()
            .map(|edge| (edge.channel_id, edge.parent_id))
            .collect::<HashSet<_>>();

        channels_set == other_channels_set && edges_set == other_edges_set
    }
}

#[cfg(not(test))]
impl PartialEq for ChannelGraph {
    fn eq(&self, other: &Self) -> bool {
        self.channels == other.channels && self.edges == other.edges
    }
}

struct SmallSet<T>(SmallVec<[T; 1]>);

impl<T> Deref for SmallSet<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T> Default for SmallSet<T> {
    fn default() -> Self {
        Self(SmallVec::new())
    }
}

impl<T> SmallSet<T> {
    fn insert(&mut self, value: T) -> bool
    where
        T: Ord,
    {
        match self.binary_search(&value) {
            Ok(_) => false,
            Err(ix) => {
                self.0.insert(ix, value);
                true
            }
        }
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}
