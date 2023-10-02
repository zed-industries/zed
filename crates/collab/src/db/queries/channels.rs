use rpc::proto::ChannelEdge;
use smallvec::SmallVec;

use super::*;

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

    pub async fn create_root_channel(
        &self,
        name: &str,
        live_kit_room: &str,
        creator_id: UserId,
    ) -> Result<ChannelId> {
        self.create_channel(name, None, live_kit_room, creator_id)
            .await
    }

    pub async fn create_channel(
        &self,
        name: &str,
        parent: Option<ChannelId>,
        live_kit_room: &str,
        creator_id: UserId,
    ) -> Result<ChannelId> {
        let name = Self::sanitize_channel_name(name)?;
        self.transaction(move |tx| async move {
            if let Some(parent) = parent {
                self.check_user_is_channel_admin(parent, creator_id, &*tx)
                    .await?;
            }

            let channel = channel::ActiveModel {
                name: ActiveValue::Set(name.to_string()),
                ..Default::default()
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
                channel_id: ActiveValue::Set(channel.id),
                user_id: ActiveValue::Set(creator_id),
                accepted: ActiveValue::Set(true),
                admin: ActiveValue::Set(true),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;

            room::ActiveModel {
                channel_id: ActiveValue::Set(Some(channel.id)),
                live_kit_room: ActiveValue::Set(live_kit_room.to_string()),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;

            Ok(channel.id)
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
            let mut channels_to_remove = self.get_channel_descendants([channel_id], &*tx).await?;
            {
                let mut channels_to_keep = channel_path::Entity::find()
                    .filter(
                        channel_path::Column::ChannelId
                            .is_in(
                                channels_to_remove
                                    .keys()
                                    .copied()
                                    .filter(|&id| id != channel_id),
                            )
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
                .filter(channel::Column::Id.is_in(channels_to_remove.keys().copied()))
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

            Ok((channels_to_remove.into_keys().collect(), members_to_notify))
        })
        .await
    }

    pub async fn invite_channel_member(
        &self,
        channel_id: ChannelId,
        invitee_id: UserId,
        inviter_id: UserId,
        is_admin: bool,
    ) -> Result<()> {
        self.transaction(move |tx| async move {
            self.check_user_is_channel_admin(channel_id, inviter_id, &*tx)
                .await?;

            channel_member::ActiveModel {
                channel_id: ActiveValue::Set(channel_id),
                user_id: ActiveValue::Set(invitee_id),
                accepted: ActiveValue::Set(false),
                admin: ActiveValue::Set(is_admin),
                ..Default::default()
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
        remover_id: UserId,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            self.check_user_is_channel_admin(channel_id, remover_id, &*tx)
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
        let parents_by_child_id = self
            .get_channel_descendants(channel_memberships.iter().map(|m| m.channel_id), &*tx)
            .await?;

        let channels_with_admin_privileges = channel_memberships
            .iter()
            .filter_map(|membership| membership.admin.then_some(membership.channel_id))
            .collect();

        let graph = self
            .get_channel_graph(parents_by_child_id, true, &tx)
            .await?;

        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryUserIdsAndChannelIds {
            ChannelId,
            UserId,
        }

        let mut channel_participants: HashMap<ChannelId, Vec<UserId>> = HashMap::default();
        {
            let mut rows = room_participant::Entity::find()
                .inner_join(room::Entity)
                .filter(room::Column::ChannelId.is_in(graph.channels.iter().map(|c| c.id)))
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

        let channel_ids = graph.channels.iter().map(|c| c.id).collect::<Vec<_>>();
        let channels_with_changed_notes = self
            .channels_with_changed_notes(user_id, &channel_ids, &*tx)
            .await?;

        let channels_with_new_messages = self
            .channels_with_new_messages(user_id, &channel_ids, &*tx)
            .await?;

        Ok(ChannelsForUser {
            channels: graph,
            channel_participants,
            channels_with_admin_privileges,
            channels_with_changed_notes,
            channels_with_new_messages,
        })
    }

    pub async fn get_channel_members(&self, id: ChannelId) -> Result<Vec<UserId>> {
        self.transaction(|tx| async move { self.get_channel_members_internal(id, &*tx).await })
            .await
    }

    pub async fn set_channel_member_admin(
        &self,
        channel_id: ChannelId,
        from: UserId,
        for_user: UserId,
        admin: bool,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            self.check_user_is_channel_admin(channel_id, from, &*tx)
                .await?;

            let result = channel_member::Entity::update_many()
                .filter(
                    channel_member::Column::ChannelId
                        .eq(channel_id)
                        .and(channel_member::Column::UserId.eq(for_user)),
                )
                .set(channel_member::ActiveModel {
                    admin: ActiveValue::set(admin),
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

    pub async fn get_channel_member_details(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> Result<Vec<proto::ChannelMember>> {
        self.transaction(|tx| async move {
            self.check_user_is_channel_admin(channel_id, user_id, &*tx)
                .await?;

            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            enum QueryMemberDetails {
                UserId,
                Admin,
                IsDirectMember,
                Accepted,
            }

            let tx = tx;
            let ancestor_ids = self.get_channel_ancestors(channel_id, &*tx).await?;
            let mut stream = channel_member::Entity::find()
                .distinct()
                .filter(channel_member::Column::ChannelId.is_in(ancestor_ids.iter().copied()))
                .select_only()
                .column(channel_member::Column::UserId)
                .column(channel_member::Column::Admin)
                .column_as(
                    channel_member::Column::ChannelId.eq(channel_id),
                    QueryMemberDetails::IsDirectMember,
                )
                .column(channel_member::Column::Accepted)
                .order_by_asc(channel_member::Column::UserId)
                .into_values::<_, QueryMemberDetails>()
                .stream(&*tx)
                .await?;

            let mut rows = Vec::<proto::ChannelMember>::new();
            while let Some(row) = stream.next().await {
                let (user_id, is_admin, is_direct_member, is_invite_accepted): (
                    UserId,
                    bool,
                    bool,
                    bool,
                ) = row?;
                let kind = match (is_direct_member, is_invite_accepted) {
                    (true, true) => proto::channel_member::Kind::Member,
                    (true, false) => proto::channel_member::Kind::Invitee,
                    (false, true) => proto::channel_member::Kind::AncestorMember,
                    (false, false) => continue,
                };
                let user_id = user_id.to_proto();
                let kind = kind.into();
                if let Some(last_row) = rows.last_mut() {
                    if last_row.user_id == user_id {
                        if is_direct_member {
                            last_row.kind = kind;
                            last_row.admin = is_admin;
                        }
                        continue;
                    }
                }
                rows.push(proto::ChannelMember {
                    user_id,
                    kind,
                    admin: is_admin,
                });
            }

            Ok(rows)
        })
        .await
    }

    pub async fn get_channel_members_internal(
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

    pub async fn check_user_is_channel_member(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        let channel_ids = self.get_channel_ancestors(channel_id, tx).await?;
        channel_member::Entity::find()
            .filter(
                channel_member::Column::ChannelId
                    .is_in(channel_ids)
                    .and(channel_member::Column::UserId.eq(user_id)),
            )
            .one(&*tx)
            .await?
            .ok_or_else(|| anyhow!("user is not a channel member or channel does not exist"))?;
        Ok(())
    }

    pub async fn check_user_is_channel_admin(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        let channel_ids = self.get_channel_ancestors(channel_id, tx).await?;
        channel_member::Entity::find()
            .filter(
                channel_member::Column::ChannelId
                    .is_in(channel_ids)
                    .and(channel_member::Column::UserId.eq(user_id))
                    .and(channel_member::Column::Admin.eq(true)),
            )
            .one(&*tx)
            .await?
            .ok_or_else(|| anyhow!("user is not a channel admin or channel does not exist"))?;
        Ok(())
    }

    /// Returns the channel ancestors, deepest first
    pub async fn get_channel_ancestors(
        &self,
        channel_id: ChannelId,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<ChannelId>> {
        let paths = channel_path::Entity::find()
            .filter(channel_path::Column::ChannelId.eq(channel_id))
            .order_by(channel_path::Column::IdPath, sea_query::Order::Desc)
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

    pub async fn room_id_for_channel(&self, channel_id: ChannelId) -> Result<RoomId> {
        self.transaction(|tx| async move {
            let tx = tx;
            let room = channel::Model {
                id: channel_id,
                ..Default::default()
            }
            .find_related(room::Entity)
            .one(&*tx)
            .await?
            .ok_or_else(|| anyhow!("invalid channel"))?;
            Ok(room.id)
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
        to: ChannelId,
        tx: &DatabaseTransaction,
    ) -> Result<ChannelGraph> {
        self.check_user_is_channel_admin(to, user, &*tx).await?;

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
            .filter(channel_path::Column::ChannelId.eq(to))
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
            channel.insert(to);
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
