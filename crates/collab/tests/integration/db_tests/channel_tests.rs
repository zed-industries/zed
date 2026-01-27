use super::{assert_channel_tree_matches, channel_tree, new_test_user};
use crate::test_both_dbs;
use collab::db::{Channel, ChannelId, ChannelRole, Database, NewUserParams, RoomId, UserId};
use rpc::{
    ConnectionId,
    proto::{self, reorder_channel},
};
use std::{collections::HashSet, sync::Arc};

test_both_dbs!(test_channels, test_channels_postgres, test_channels_sqlite);

async fn test_channels(db: &Arc<Database>) {
    let a_id = new_test_user(db, "user1@example.com").await;
    let b_id = new_test_user(db, "user2@example.com").await;

    let zed_id = db.create_root_channel("zed", a_id).await.unwrap();

    // Make sure that people cannot read channels they haven't been invited to
    assert!(db.get_channel(zed_id, b_id).await.is_err());

    db.invite_channel_member(zed_id, b_id, a_id, ChannelRole::Member)
        .await
        .unwrap();

    db.respond_to_channel_invite(zed_id, b_id, true)
        .await
        .unwrap();

    let crdb_id = db.create_sub_channel("crdb", zed_id, a_id).await.unwrap();
    let livestreaming_id = db
        .create_sub_channel("livestreaming", zed_id, a_id)
        .await
        .unwrap();
    let replace_id = db
        .create_sub_channel("replace", zed_id, a_id)
        .await
        .unwrap();

    let (members, _) = db
        .get_channel_participant_details(replace_id, "", 10, a_id)
        .await
        .unwrap();
    let ids = members
        .into_iter()
        .map(|m| UserId::from_proto(m.user_id))
        .collect::<Vec<_>>();
    assert_eq!(ids, &[a_id, b_id]);

    let rust_id = db.create_root_channel("rust", a_id).await.unwrap();
    let cargo_id = db.create_sub_channel("cargo", rust_id, a_id).await.unwrap();

    let cargo_ra_id = db
        .create_sub_channel("cargo-ra", cargo_id, a_id)
        .await
        .unwrap();

    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_channel_tree_matches(
        result.channels,
        channel_tree(&[
            (zed_id, &[], "zed"),
            (crdb_id, &[zed_id], "crdb"),
            (livestreaming_id, &[zed_id], "livestreaming"),
            (replace_id, &[zed_id], "replace"),
            (rust_id, &[], "rust"),
            (cargo_id, &[rust_id], "cargo"),
            (cargo_ra_id, &[rust_id, cargo_id], "cargo-ra"),
        ]),
    );

    let result = db.get_channels_for_user(b_id).await.unwrap();
    assert_channel_tree_matches(
        result.channels,
        channel_tree(&[
            (zed_id, &[], "zed"),
            (crdb_id, &[zed_id], "crdb"),
            (livestreaming_id, &[zed_id], "livestreaming"),
            (replace_id, &[zed_id], "replace"),
        ]),
    );

    // Update member permissions
    let set_subchannel_admin = db
        .set_channel_member_role(crdb_id, a_id, b_id, ChannelRole::Admin)
        .await;
    assert!(set_subchannel_admin.is_err());
    let set_channel_admin = db
        .set_channel_member_role(zed_id, a_id, b_id, ChannelRole::Admin)
        .await;
    assert!(set_channel_admin.is_ok());

    let result = db.get_channels_for_user(b_id).await.unwrap();
    assert_channel_tree_matches(
        result.channels,
        channel_tree(&[
            (zed_id, &[], "zed"),
            (crdb_id, &[zed_id], "crdb"),
            (livestreaming_id, &[zed_id], "livestreaming"),
            (replace_id, &[zed_id], "replace"),
        ]),
    );

    // Remove a single channel
    db.delete_channel(crdb_id, a_id).await.unwrap();
    assert!(db.get_channel(crdb_id, a_id).await.is_err());

    // Remove a channel tree
    let (_, mut channel_ids) = db.delete_channel(rust_id, a_id).await.unwrap();
    channel_ids.sort();
    assert_eq!(channel_ids, &[rust_id, cargo_id, cargo_ra_id]);

    assert!(db.get_channel(rust_id, a_id).await.is_err());
    assert!(db.get_channel(cargo_id, a_id).await.is_err());
    assert!(db.get_channel(cargo_ra_id, a_id).await.is_err());
}

test_both_dbs!(
    test_joining_channels,
    test_joining_channels_postgres,
    test_joining_channels_sqlite
);

async fn test_joining_channels(db: &Arc<Database>) {
    let owner_id = db.create_server("test").await.unwrap().0 as u32;

    let user_1 = new_test_user(db, "user1@example.com").await;
    let user_2 = new_test_user(db, "user2@example.com").await;

    let channel_1 = db.create_root_channel("channel_1", user_1).await.unwrap();

    // can join a room with membership to its channel
    let (joined_room, _, _) = db
        .join_channel(channel_1, user_1, ConnectionId { owner_id, id: 1 })
        .await
        .unwrap();
    assert_eq!(joined_room.room.participants.len(), 1);

    let room_id = RoomId::from_proto(joined_room.room.id);
    drop(joined_room);
    // cannot join a room without membership to its channel
    assert!(
        db.join_room(room_id, user_2, ConnectionId { owner_id, id: 1 },)
            .await
            .is_err()
    );
}

test_both_dbs!(
    test_channel_invites,
    test_channel_invites_postgres,
    test_channel_invites_sqlite
);

async fn test_channel_invites(db: &Arc<Database>) {
    db.create_server("test").await.unwrap();

    let user_1 = new_test_user(db, "user1@example.com").await;
    let user_2 = new_test_user(db, "user2@example.com").await;
    let user_3 = new_test_user(db, "user3@example.com").await;

    let channel_1_1 = db.create_root_channel("channel_1", user_1).await.unwrap();

    let channel_1_2 = db.create_root_channel("channel_2", user_1).await.unwrap();

    db.invite_channel_member(channel_1_1, user_2, user_1, ChannelRole::Member)
        .await
        .unwrap();
    db.invite_channel_member(channel_1_2, user_2, user_1, ChannelRole::Member)
        .await
        .unwrap();
    db.invite_channel_member(channel_1_1, user_3, user_1, ChannelRole::Admin)
        .await
        .unwrap();

    let user_2_invites = db
        .get_channels_for_user(user_2)
        .await
        .unwrap()
        .invited_channels
        .into_iter()
        .map(|channel| channel.id)
        .collect::<Vec<_>>();
    assert_eq!(user_2_invites, &[channel_1_1, channel_1_2]);

    let user_3_invites = db
        .get_channels_for_user(user_3)
        .await
        .unwrap()
        .invited_channels
        .into_iter()
        .map(|channel| channel.id)
        .collect::<Vec<_>>();
    assert_eq!(user_3_invites, &[channel_1_1]);

    let (mut members, _) = db
        .get_channel_participant_details(channel_1_1, "", 100, user_1)
        .await
        .unwrap();

    members.sort_by_key(|member| member.user_id);
    assert_eq!(
        members,
        &[
            proto::ChannelMember {
                user_id: user_1.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Admin.into(),
            },
            proto::ChannelMember {
                user_id: user_2.to_proto(),
                kind: proto::channel_member::Kind::Invitee.into(),
                role: proto::ChannelRole::Member.into(),
            },
            proto::ChannelMember {
                user_id: user_3.to_proto(),
                kind: proto::channel_member::Kind::Invitee.into(),
                role: proto::ChannelRole::Admin.into(),
            },
        ]
    );

    db.respond_to_channel_invite(channel_1_1, user_2, true)
        .await
        .unwrap();

    let channel_1_3 = db
        .create_sub_channel("channel_3", channel_1_1, user_1)
        .await
        .unwrap();

    let (members, _) = db
        .get_channel_participant_details(channel_1_3, "", 100, user_1)
        .await
        .unwrap();
    assert_eq!(
        members,
        &[
            proto::ChannelMember {
                user_id: user_1.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Admin.into(),
            },
            proto::ChannelMember {
                user_id: user_3.to_proto(),
                kind: proto::channel_member::Kind::Invitee.into(),
                role: proto::ChannelRole::Admin.into(),
            },
            proto::ChannelMember {
                user_id: user_2.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Member.into(),
            },
        ]
    );
}

test_both_dbs!(
    test_channel_renames,
    test_channel_renames_postgres,
    test_channel_renames_sqlite
);

async fn test_channel_renames(db: &Arc<Database>) {
    db.create_server("test").await.unwrap();

    let user_1 = db
        .create_user(
            "user1@example.com",
            None,
            false,
            NewUserParams {
                github_login: "user1".into(),
                github_user_id: 5,
            },
        )
        .await
        .unwrap()
        .user_id;

    let user_2 = db
        .create_user(
            "user2@example.com",
            None,
            false,
            NewUserParams {
                github_login: "user2".into(),
                github_user_id: 6,
            },
        )
        .await
        .unwrap()
        .user_id;

    let zed_id = db.create_root_channel("zed", user_1).await.unwrap();

    db.rename_channel(zed_id, user_1, "#zed-archive")
        .await
        .unwrap();

    let channel = db.get_channel(zed_id, user_1).await.unwrap();
    assert_eq!(channel.name, "zed-archive");

    let non_permissioned_rename = db.rename_channel(zed_id, user_2, "hacked-lol").await;
    assert!(non_permissioned_rename.is_err());

    let bad_name_rename = db.rename_channel(zed_id, user_1, "#").await;
    assert!(bad_name_rename.is_err())
}

test_both_dbs!(
    test_db_channel_moving,
    test_db_channel_moving_postgres,
    test_db_channel_moving_sqlite
);

async fn test_db_channel_moving(db: &Arc<Database>) {
    let a_id = db
        .create_user(
            "user1@example.com",
            None,
            false,
            NewUserParams {
                github_login: "user1".into(),
                github_user_id: 5,
            },
        )
        .await
        .unwrap()
        .user_id;

    let zed_id = db.create_root_channel("zed", a_id).await.unwrap();

    let crdb_id = db.create_sub_channel("crdb", zed_id, a_id).await.unwrap();

    let gpui2_id = db.create_sub_channel("gpui2", zed_id, a_id).await.unwrap();

    let livestreaming_id = db
        .create_sub_channel("livestreaming", crdb_id, a_id)
        .await
        .unwrap();

    let livestreaming_sub_id = db
        .create_sub_channel("livestreaming_sub", livestreaming_id, a_id)
        .await
        .unwrap();

    // sanity check
    //     /- gpui2
    // zed -- crdb - livestreaming - livestreaming_sub
    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_channel_tree(
        result.channels,
        &[
            (zed_id, &[]),
            (crdb_id, &[zed_id]),
            (livestreaming_id, &[zed_id, crdb_id]),
            (livestreaming_sub_id, &[zed_id, crdb_id, livestreaming_id]),
            (gpui2_id, &[zed_id]),
        ],
    );

    // Check that we can do a simple leaf -> leaf move
    db.move_channel(livestreaming_sub_id, crdb_id, a_id)
        .await
        .unwrap();

    //     /- gpui2
    // zed -- crdb -- livestreaming
    //             \- livestreaming_sub
    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_channel_tree(
        result.channels,
        &[
            (zed_id, &[]),
            (crdb_id, &[zed_id]),
            (livestreaming_id, &[zed_id, crdb_id]),
            (livestreaming_sub_id, &[zed_id, crdb_id]),
            (gpui2_id, &[zed_id]),
        ],
    );

    // Check that we can move a whole subtree at once
    db.move_channel(crdb_id, gpui2_id, a_id).await.unwrap();

    // zed -- gpui2 -- crdb -- livestreaming
    //                      \- livestreaming_sub
    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_channel_tree(
        result.channels,
        &[
            (zed_id, &[]),
            (gpui2_id, &[zed_id]),
            (crdb_id, &[zed_id, gpui2_id]),
            (livestreaming_id, &[zed_id, gpui2_id, crdb_id]),
            (livestreaming_sub_id, &[zed_id, gpui2_id, crdb_id]),
        ],
    );
}

test_both_dbs!(
    test_channel_reordering,
    test_channel_reordering_postgres,
    test_channel_reordering_sqlite
);

async fn test_channel_reordering(db: &Arc<Database>) {
    let admin_id = db
        .create_user(
            "admin@example.com",
            None,
            false,
            NewUserParams {
                github_login: "admin".into(),
                github_user_id: 1,
            },
        )
        .await
        .unwrap()
        .user_id;

    let user_id = db
        .create_user(
            "user@example.com",
            None,
            false,
            NewUserParams {
                github_login: "user".into(),
                github_user_id: 2,
            },
        )
        .await
        .unwrap()
        .user_id;

    // Create a root channel with some sub-channels
    let root_id = db.create_root_channel("root", admin_id).await.unwrap();

    // Invite user to root channel so they can see the sub-channels
    db.invite_channel_member(root_id, user_id, admin_id, ChannelRole::Member)
        .await
        .unwrap();
    db.respond_to_channel_invite(root_id, user_id, true)
        .await
        .unwrap();

    let alpha_id = db
        .create_sub_channel("alpha", root_id, admin_id)
        .await
        .unwrap();
    let beta_id = db
        .create_sub_channel("beta", root_id, admin_id)
        .await
        .unwrap();
    let gamma_id = db
        .create_sub_channel("gamma", root_id, admin_id)
        .await
        .unwrap();

    // Initial order should be: root, alpha (order=1), beta (order=2), gamma (order=3)
    let result = db.get_channels_for_user(admin_id).await.unwrap();
    assert_channel_tree_order(
        result.channels,
        &[
            (root_id, &[], 1),
            (alpha_id, &[root_id], 1),
            (beta_id, &[root_id], 2),
            (gamma_id, &[root_id], 3),
        ],
    );

    // Test moving beta up (should swap with alpha)
    let updated_channels = db
        .reorder_channel(beta_id, reorder_channel::Direction::Up, admin_id)
        .await
        .unwrap();

    // Verify that beta and alpha were returned as updated
    assert_eq!(updated_channels.len(), 2);
    let updated_ids: std::collections::HashSet<_> = updated_channels.iter().map(|c| c.id).collect();
    assert!(updated_ids.contains(&alpha_id));
    assert!(updated_ids.contains(&beta_id));

    // Now order should be: root, beta (order=1), alpha (order=2), gamma (order=3)
    let result = db.get_channels_for_user(admin_id).await.unwrap();
    assert_channel_tree_order(
        result.channels,
        &[
            (root_id, &[], 1),
            (beta_id, &[root_id], 1),
            (alpha_id, &[root_id], 2),
            (gamma_id, &[root_id], 3),
        ],
    );

    // Test moving gamma down (should be no-op since it's already last)
    let updated_channels = db
        .reorder_channel(gamma_id, reorder_channel::Direction::Down, admin_id)
        .await
        .unwrap();

    // Should return just nothing
    assert_eq!(updated_channels.len(), 0);

    // Test moving alpha down (should swap with gamma)
    let updated_channels = db
        .reorder_channel(alpha_id, reorder_channel::Direction::Down, admin_id)
        .await
        .unwrap();

    // Verify that alpha and gamma were returned as updated
    assert_eq!(updated_channels.len(), 2);
    let updated_ids: std::collections::HashSet<_> = updated_channels.iter().map(|c| c.id).collect();
    assert!(updated_ids.contains(&alpha_id));
    assert!(updated_ids.contains(&gamma_id));

    // Now order should be: root, beta (order=1), gamma (order=2), alpha (order=3)
    let result = db.get_channels_for_user(admin_id).await.unwrap();
    assert_channel_tree_order(
        result.channels,
        &[
            (root_id, &[], 1),
            (beta_id, &[root_id], 1),
            (gamma_id, &[root_id], 2),
            (alpha_id, &[root_id], 3),
        ],
    );

    // Test that non-admin cannot reorder
    let reorder_result = db
        .reorder_channel(beta_id, reorder_channel::Direction::Up, user_id)
        .await;
    assert!(reorder_result.is_err());

    // Test moving beta up (should be no-op since it's already first)
    let updated_channels = db
        .reorder_channel(beta_id, reorder_channel::Direction::Up, admin_id)
        .await
        .unwrap();

    // Should return nothing
    assert_eq!(updated_channels.len(), 0);

    // Adding a channel to an existing ordering should add it to the end
    let delta_id = db
        .create_sub_channel("delta", root_id, admin_id)
        .await
        .unwrap();

    let result = db.get_channels_for_user(admin_id).await.unwrap();
    assert_channel_tree_order(
        result.channels,
        &[
            (root_id, &[], 1),
            (beta_id, &[root_id], 1),
            (gamma_id, &[root_id], 2),
            (alpha_id, &[root_id], 3),
            (delta_id, &[root_id], 4),
        ],
    );

    // And moving a channel into an existing ordering should add it to the end
    let eta_id = db
        .create_sub_channel("eta", delta_id, admin_id)
        .await
        .unwrap();

    let result = db.get_channels_for_user(admin_id).await.unwrap();
    assert_channel_tree_order(
        result.channels,
        &[
            (root_id, &[], 1),
            (beta_id, &[root_id], 1),
            (gamma_id, &[root_id], 2),
            (alpha_id, &[root_id], 3),
            (delta_id, &[root_id], 4),
            (eta_id, &[root_id, delta_id], 1),
        ],
    );

    db.move_channel(eta_id, root_id, admin_id).await.unwrap();
    let result = db.get_channels_for_user(admin_id).await.unwrap();
    assert_channel_tree_order(
        result.channels,
        &[
            (root_id, &[], 1),
            (beta_id, &[root_id], 1),
            (gamma_id, &[root_id], 2),
            (alpha_id, &[root_id], 3),
            (delta_id, &[root_id], 4),
            (eta_id, &[root_id], 5),
        ],
    );
}

test_both_dbs!(
    test_db_channel_moving_bugs,
    test_db_channel_moving_bugs_postgres,
    test_db_channel_moving_bugs_sqlite
);

async fn test_db_channel_moving_bugs(db: &Arc<Database>) {
    let user_id = db
        .create_user(
            "user1@example.com",
            None,
            false,
            NewUserParams {
                github_login: "user1".into(),
                github_user_id: 5,
            },
        )
        .await
        .unwrap()
        .user_id;

    let zed_id = db.create_root_channel("zed", user_id).await.unwrap();

    let projects_id = db
        .create_sub_channel("projects", zed_id, user_id)
        .await
        .unwrap();

    let livestreaming_id = db
        .create_sub_channel("livestreaming", projects_id, user_id)
        .await
        .unwrap();

    let result = db.get_channels_for_user(user_id).await.unwrap();
    assert_channel_tree(
        result.channels,
        &[
            (zed_id, &[]),
            (projects_id, &[zed_id]),
            (livestreaming_id, &[zed_id, projects_id]),
        ],
    );

    // Can't move a channel into its ancestor
    db.move_channel(projects_id, livestreaming_id, user_id)
        .await
        .unwrap_err();
    let result = db.get_channels_for_user(user_id).await.unwrap();
    assert_channel_tree(
        result.channels,
        &[
            (zed_id, &[]),
            (projects_id, &[zed_id]),
            (livestreaming_id, &[zed_id, projects_id]),
        ],
    );

    // Can't un-root a root channel
    db.move_channel(zed_id, livestreaming_id, user_id)
        .await
        .unwrap_err();
    let result = db.get_channels_for_user(user_id).await.unwrap();
    assert_channel_tree(
        result.channels,
        &[
            (zed_id, &[]),
            (projects_id, &[zed_id]),
            (livestreaming_id, &[zed_id, projects_id]),
        ],
    );
}

test_both_dbs!(
    test_user_is_channel_participant,
    test_user_is_channel_participant_postgres,
    test_user_is_channel_participant_sqlite
);

async fn test_user_is_channel_participant(db: &Arc<Database>) {
    let admin = new_test_user(db, "admin@example.com").await;
    let member = new_test_user(db, "member@example.com").await;
    let guest = new_test_user(db, "guest@example.com").await;

    let zed_channel = db.create_root_channel("zed", admin).await.unwrap();
    let internal_channel_id = db
        .create_sub_channel("active", zed_channel, admin)
        .await
        .unwrap();
    let public_channel_id = db
        .create_sub_channel("vim", zed_channel, admin)
        .await
        .unwrap();

    db.set_channel_visibility(zed_channel, collab::db::ChannelVisibility::Public, admin)
        .await
        .unwrap();
    db.set_channel_visibility(
        public_channel_id,
        collab::db::ChannelVisibility::Public,
        admin,
    )
    .await
    .unwrap();
    db.invite_channel_member(zed_channel, member, admin, ChannelRole::Member)
        .await
        .unwrap();
    db.invite_channel_member(zed_channel, guest, admin, ChannelRole::Guest)
        .await
        .unwrap();

    db.respond_to_channel_invite(zed_channel, member, true)
        .await
        .unwrap();

    db.transaction(|tx| async move {
        db.check_user_is_channel_participant(
            &db.get_channel_internal(public_channel_id, &tx).await?,
            admin,
            &tx,
        )
        .await
    })
    .await
    .unwrap();
    db.transaction(|tx| async move {
        db.check_user_is_channel_participant(
            &db.get_channel_internal(public_channel_id, &tx).await?,
            member,
            &tx,
        )
        .await
    })
    .await
    .unwrap();

    let (mut members, _) = db
        .get_channel_participant_details(public_channel_id, "", 100, admin)
        .await
        .unwrap();

    members.sort_by_key(|member| member.user_id);

    assert_eq!(
        members,
        &[
            proto::ChannelMember {
                user_id: admin.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Admin.into(),
            },
            proto::ChannelMember {
                user_id: member.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Member.into(),
            },
            proto::ChannelMember {
                user_id: guest.to_proto(),
                kind: proto::channel_member::Kind::Invitee.into(),
                role: proto::ChannelRole::Guest.into(),
            },
        ]
    );

    db.respond_to_channel_invite(zed_channel, guest, true)
        .await
        .unwrap();

    db.transaction(|tx| async move {
        db.check_user_is_channel_participant(
            &db.get_channel_internal(public_channel_id, &tx).await?,
            guest,
            &tx,
        )
        .await
    })
    .await
    .unwrap();

    let channels = db.get_channels_for_user(guest).await.unwrap().channels;
    assert_channel_tree(
        channels,
        &[(zed_channel, &[]), (public_channel_id, &[zed_channel])],
    );
    let channels = db.get_channels_for_user(member).await.unwrap().channels;
    assert_channel_tree(
        channels,
        &[
            (zed_channel, &[]),
            (internal_channel_id, &[zed_channel]),
            (public_channel_id, &[zed_channel]),
        ],
    );

    db.set_channel_member_role(zed_channel, admin, guest, ChannelRole::Banned)
        .await
        .unwrap();
    assert!(
        db.transaction(|tx| async move {
            db.check_user_is_channel_participant(
                &db.get_channel_internal(public_channel_id, &tx)
                    .await
                    .unwrap(),
                guest,
                &tx,
            )
            .await
        })
        .await
        .is_err()
    );

    let (mut members, _) = db
        .get_channel_participant_details(public_channel_id, "", 100, admin)
        .await
        .unwrap();

    members.sort_by_key(|member| member.user_id);

    assert_eq!(
        members,
        &[
            proto::ChannelMember {
                user_id: admin.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Admin.into(),
            },
            proto::ChannelMember {
                user_id: member.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Member.into(),
            },
            proto::ChannelMember {
                user_id: guest.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Banned.into(),
            },
        ]
    );

    db.remove_channel_member(zed_channel, guest, admin)
        .await
        .unwrap();

    db.invite_channel_member(zed_channel, guest, admin, ChannelRole::Guest)
        .await
        .unwrap();

    // currently people invited to parent channels are not shown here
    let (mut members, _) = db
        .get_channel_participant_details(public_channel_id, "", 100, admin)
        .await
        .unwrap();

    members.sort_by_key(|member| member.user_id);

    assert_eq!(
        members,
        &[
            proto::ChannelMember {
                user_id: admin.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Admin.into(),
            },
            proto::ChannelMember {
                user_id: member.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Member.into(),
            },
            proto::ChannelMember {
                user_id: guest.to_proto(),
                kind: proto::channel_member::Kind::Invitee.into(),
                role: proto::ChannelRole::Guest.into(),
            },
        ]
    );

    db.respond_to_channel_invite(zed_channel, guest, true)
        .await
        .unwrap();

    db.transaction(|tx| async move {
        db.check_user_is_channel_participant(
            &db.get_channel_internal(zed_channel, &tx).await.unwrap(),
            guest,
            &tx,
        )
        .await
    })
    .await
    .unwrap();
    assert!(
        db.transaction(|tx| async move {
            db.check_user_is_channel_participant(
                &db.get_channel_internal(internal_channel_id, &tx)
                    .await
                    .unwrap(),
                guest,
                &tx,
            )
            .await
        })
        .await
        .is_err(),
    );

    db.transaction(|tx| async move {
        db.check_user_is_channel_participant(
            &db.get_channel_internal(public_channel_id, &tx)
                .await
                .unwrap(),
            guest,
            &tx,
        )
        .await
    })
    .await
    .unwrap();

    let (mut members, _) = db
        .get_channel_participant_details(public_channel_id, "", 100, admin)
        .await
        .unwrap();

    members.sort_by_key(|member| member.user_id);

    assert_eq!(
        members,
        &[
            proto::ChannelMember {
                user_id: admin.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Admin.into(),
            },
            proto::ChannelMember {
                user_id: member.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Member.into(),
            },
            proto::ChannelMember {
                user_id: guest.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Guest.into(),
            },
        ]
    );

    let channels = db.get_channels_for_user(guest).await.unwrap().channels;
    assert_channel_tree(
        channels,
        &[(zed_channel, &[]), (public_channel_id, &[zed_channel])],
    )
}

#[track_caller]
fn assert_channel_tree(actual: Vec<Channel>, expected: &[(ChannelId, &[ChannelId])]) {
    let actual = actual
        .iter()
        .map(|channel| (channel.id, channel.parent_path.as_slice()))
        .collect::<HashSet<_>>();
    let expected = expected
        .iter()
        .map(|(id, parents)| (*id, *parents))
        .collect::<HashSet<_>>();
    pretty_assertions::assert_eq!(actual, expected, "wrong channel ids and parent paths");
}

#[track_caller]
fn assert_channel_tree_order(actual: Vec<Channel>, expected: &[(ChannelId, &[ChannelId], i32)]) {
    let actual = actual
        .iter()
        .map(|channel| {
            (
                channel.id,
                channel.parent_path.as_slice(),
                channel.channel_order,
            )
        })
        .collect::<HashSet<_>>();
    let expected = expected
        .iter()
        .map(|(id, parents, order)| (*id, *parents, *order))
        .collect::<HashSet<_>>();
    pretty_assertions::assert_eq!(actual, expected, "wrong channel ids and parent paths");
}
