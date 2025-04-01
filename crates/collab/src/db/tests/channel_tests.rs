use crate::{
    db::{
        Channel, ChannelId, ChannelRole, Database, NewUserParams, RoomId, UserId,
        tests::{channel_tree, new_test_connection, new_test_user},
    },
    test_both_dbs,
};
use rpc::{
    ConnectionId,
    proto::{self},
};
use std::sync::Arc;

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
    assert_eq!(
        result.channels,
        channel_tree(&[
            (zed_id, &[], "zed"),
            (crdb_id, &[zed_id], "crdb"),
            (livestreaming_id, &[zed_id], "livestreaming",),
            (replace_id, &[zed_id], "replace"),
            (rust_id, &[], "rust"),
            (cargo_id, &[rust_id], "cargo"),
            (cargo_ra_id, &[rust_id, cargo_id], "cargo-ra",)
        ],)
    );

    let result = db.get_channels_for_user(b_id).await.unwrap();
    assert_eq!(
        result.channels,
        channel_tree(&[
            (zed_id, &[], "zed"),
            (crdb_id, &[zed_id], "crdb"),
            (livestreaming_id, &[zed_id], "livestreaming",),
            (replace_id, &[zed_id], "replace")
        ],)
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
    assert_eq!(
        result.channels,
        channel_tree(&[
            (zed_id, &[], "zed"),
            (crdb_id, &[zed_id], "crdb"),
            (livestreaming_id, &[zed_id], "livestreaming",),
            (replace_id, &[zed_id], "replace")
        ],)
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
    test_channels_moving_postgres,
    test_channels_moving_sqlite
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

    let livestreaming_dag_id = db
        .create_sub_channel("livestreaming_dag", livestreaming_id, a_id)
        .await
        .unwrap();

    // ========================================================================
    // sanity check
    // Initial DAG:
    //     /- gpui2
    // zed -- crdb - livestreaming - livestreaming_dag
    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_channel_tree(
        result.channels,
        &[
            (zed_id, &[]),
            (crdb_id, &[zed_id]),
            (livestreaming_id, &[zed_id, crdb_id]),
            (livestreaming_dag_id, &[zed_id, crdb_id, livestreaming_id]),
            (gpui2_id, &[zed_id]),
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

    db.set_channel_visibility(zed_channel, crate::db::ChannelVisibility::Public, admin)
        .await
        .unwrap();
    db.set_channel_visibility(
        public_channel_id,
        crate::db::ChannelVisibility::Public,
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

test_both_dbs!(
    test_guest_access,
    test_guest_access_postgres,
    test_guest_access_sqlite
);

async fn test_guest_access(db: &Arc<Database>) {
    let server = db.create_server("test").await.unwrap();

    let admin = new_test_user(db, "admin@example.com").await;
    let guest = new_test_user(db, "guest@example.com").await;
    let guest_connection = new_test_connection(server);

    let zed_channel = db.create_root_channel("zed", admin).await.unwrap();
    db.set_channel_visibility(zed_channel, crate::db::ChannelVisibility::Public, admin)
        .await
        .unwrap();

    assert!(
        db.join_channel_chat(zed_channel, guest_connection, guest)
            .await
            .is_err()
    );

    db.join_channel(zed_channel, guest, guest_connection)
        .await
        .unwrap();

    assert!(
        db.join_channel_chat(zed_channel, guest_connection, guest)
            .await
            .is_ok()
    )
}

#[track_caller]
fn assert_channel_tree(actual: Vec<Channel>, expected: &[(ChannelId, &[ChannelId])]) {
    let actual = actual
        .iter()
        .map(|channel| (channel.id, channel.parent_path.as_slice()))
        .collect::<Vec<_>>();
    pretty_assertions::assert_eq!(
        actual,
        expected.to_vec(),
        "wrong channel ids and parent paths"
    );
}
