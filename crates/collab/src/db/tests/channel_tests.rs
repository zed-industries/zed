use collections::{HashMap, HashSet};
use rpc::{
    proto::{self},
    ConnectionId,
};

use crate::{
    db::{
        queries::channels::ChannelGraph,
        tests::{graph, TEST_RELEASE_CHANNEL},
        ChannelId, ChannelRole, Database, NewUserParams, RoomId, ServerId, UserId,
    },
    test_both_dbs,
};
use std::sync::{
    atomic::{AtomicI32, AtomicU32, Ordering},
    Arc,
};

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

    let mut members = db
        .transaction(|tx| async move { Ok(db.get_channel_participants(replace_id, &*tx).await?) })
        .await
        .unwrap();
    members.sort();
    assert_eq!(members, &[a_id, b_id]);

    let rust_id = db.create_root_channel("rust", a_id).await.unwrap();
    let cargo_id = db.create_sub_channel("cargo", rust_id, a_id).await.unwrap();

    let cargo_ra_id = db
        .create_sub_channel("cargo-ra", cargo_id, a_id)
        .await
        .unwrap();

    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_eq!(
        result.channels,
        graph(
            &[
                (zed_id, "zed", ChannelRole::Admin),
                (crdb_id, "crdb", ChannelRole::Admin),
                (livestreaming_id, "livestreaming", ChannelRole::Admin),
                (replace_id, "replace", ChannelRole::Admin),
                (rust_id, "rust", ChannelRole::Admin),
                (cargo_id, "cargo", ChannelRole::Admin),
                (cargo_ra_id, "cargo-ra", ChannelRole::Admin)
            ],
            &[
                (crdb_id, zed_id),
                (livestreaming_id, zed_id),
                (replace_id, zed_id),
                (cargo_id, rust_id),
                (cargo_ra_id, cargo_id),
            ]
        )
    );

    let result = db.get_channels_for_user(b_id).await.unwrap();
    assert_eq!(
        result.channels,
        graph(
            &[
                (zed_id, "zed", ChannelRole::Member),
                (crdb_id, "crdb", ChannelRole::Member),
                (livestreaming_id, "livestreaming", ChannelRole::Member),
                (replace_id, "replace", ChannelRole::Member)
            ],
            &[
                (crdb_id, zed_id),
                (livestreaming_id, zed_id),
                (replace_id, zed_id)
            ]
        )
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
        graph(
            &[
                (zed_id, "zed", ChannelRole::Admin),
                (crdb_id, "crdb", ChannelRole::Admin),
                (livestreaming_id, "livestreaming", ChannelRole::Admin),
                (replace_id, "replace", ChannelRole::Admin)
            ],
            &[
                (crdb_id, zed_id),
                (livestreaming_id, zed_id),
                (replace_id, zed_id)
            ]
        )
    );

    // Remove a single channel
    db.delete_channel(crdb_id, a_id).await.unwrap();
    assert!(db.get_channel(crdb_id, a_id).await.is_err());

    // Remove a channel tree
    let (mut channel_ids, user_ids) = db.delete_channel(rust_id, a_id).await.unwrap();
    channel_ids.sort();
    assert_eq!(channel_ids, &[rust_id, cargo_id, cargo_ra_id]);
    assert_eq!(user_ids, &[a_id]);

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
    let (joined_room, _) = db
        .join_channel(
            channel_1,
            user_1,
            ConnectionId { owner_id, id: 1 },
            TEST_RELEASE_CHANNEL,
        )
        .await
        .unwrap();
    assert_eq!(joined_room.room.participants.len(), 1);

    let room_id = RoomId::from_proto(joined_room.room.id);
    drop(joined_room);
    // cannot join a room without membership to its channel
    assert!(db
        .join_room(
            room_id,
            user_2,
            ConnectionId { owner_id, id: 1 },
            TEST_RELEASE_CHANNEL
        )
        .await
        .is_err());
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
        .get_channel_invites_for_user(user_2) // -> [channel_1_1, channel_1_2]
        .await
        .unwrap()
        .into_iter()
        .map(|channel| channel.id)
        .collect::<Vec<_>>();

    assert_eq!(user_2_invites, &[channel_1_1, channel_1_2]);

    let user_3_invites = db
        .get_channel_invites_for_user(user_3) // -> [channel_1_1]
        .await
        .unwrap()
        .into_iter()
        .map(|channel| channel.id)
        .collect::<Vec<_>>();

    assert_eq!(user_3_invites, &[channel_1_1]);

    let mut members = db
        .get_channel_participant_details(channel_1_1, user_1)
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

    let members = db
        .get_channel_participant_details(channel_1_3, user_1)
        .await
        .unwrap();
    assert_eq!(
        members,
        &[
            proto::ChannelMember {
                user_id: user_1.to_proto(),
                kind: proto::channel_member::Kind::AncestorMember.into(),
                role: proto::ChannelRole::Admin.into(),
            },
            proto::ChannelMember {
                user_id: user_2.to_proto(),
                kind: proto::channel_member::Kind::AncestorMember.into(),
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
            false,
            NewUserParams {
                github_login: "user1".into(),
                github_user_id: 5,
                invite_count: 0,
            },
        )
        .await
        .unwrap()
        .user_id;

    let user_2 = db
        .create_user(
            "user2@example.com",
            false,
            NewUserParams {
                github_login: "user2".into(),
                github_user_id: 6,
                invite_count: 0,
            },
        )
        .await
        .unwrap()
        .user_id;

    let zed_id = db.create_root_channel("zed", user_1).await.unwrap();

    db.rename_channel(zed_id, user_1, "#zed-archive")
        .await
        .unwrap();

    let zed_archive_id = zed_id;

    let channel = db.get_channel(zed_archive_id, user_1).await.unwrap();
    assert_eq!(channel.name, "zed-archive");

    let non_permissioned_rename = db
        .rename_channel(zed_archive_id, user_2, "hacked-lol")
        .await;
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
            false,
            NewUserParams {
                github_login: "user1".into(),
                github_user_id: 5,
                invite_count: 0,
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
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (crdb_id, Some(zed_id)),
            (gpui2_id, Some(zed_id)),
            (livestreaming_id, Some(crdb_id)),
            (livestreaming_dag_id, Some(livestreaming_id)),
        ],
    );

    // Attempt to make a cycle
    assert!(db
        .link_channel(a_id, zed_id, livestreaming_id)
        .await
        .is_err());

    //  // ========================================================================
    //  // Make a link
    //  db.link_channel(a_id, livestreaming_id, zed_id)
    //      .await
    //      .unwrap();

    //  // DAG is now:
    //  //     /- gpui2
    //  // zed -- crdb - livestreaming - livestreaming_dag
    //  //    \---------/
    //  let result = db.get_channels_for_user(a_id).await.unwrap();
    //  assert_dag(
    //      result.channels,
    //      &[
    //          (zed_id, None),
    //          (crdb_id, Some(zed_id)),
    //          (gpui2_id, Some(zed_id)),
    //          (livestreaming_id, Some(zed_id)),
    //          (livestreaming_id, Some(crdb_id)),
    //          (livestreaming_dag_id, Some(livestreaming_id)),
    //      ],
    //  );

    //  // ========================================================================
    //  // Create a new channel below a channel with multiple parents
    //  let livestreaming_dag_sub_id = db
    //      .create_channel("livestreaming_dag_sub", Some(livestreaming_dag_id), a_id)
    //      .await
    //      .unwrap();

    //  // DAG is now:
    //  //     /- gpui2
    //  // zed -- crdb - livestreaming - livestreaming_dag - livestreaming_dag_sub_id
    //  //    \---------/
    //  let result = db.get_channels_for_user(a_id).await.unwrap();
    //  assert_dag(
    //      result.channels,
    //      &[
    //          (zed_id, None),
    //          (crdb_id, Some(zed_id)),
    //          (gpui2_id, Some(zed_id)),
    //          (livestreaming_id, Some(zed_id)),
    //          (livestreaming_id, Some(crdb_id)),
    //          (livestreaming_dag_id, Some(livestreaming_id)),
    //          (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
    //      ],
    //  );

    //  // ========================================================================
    //  // Test a complex DAG by making another link
    //  let returned_channels = db
    //      .link_channel(a_id, livestreaming_dag_sub_id, livestreaming_id)
    //      .await
    //      .unwrap();

    //  // DAG is now:
    //  //    /- gpui2                /---------------------\
    //  // zed - crdb - livestreaming - livestreaming_dag - livestreaming_dag_sub_id
    //  //    \--------/

    //  // make sure we're getting just the new link
    //  // Not using the assert_dag helper because we want to make sure we're returning the full data
    //  pretty_assertions::assert_eq!(
    //      returned_channels,
    //      graph(
    //          &[(
    //              livestreaming_dag_sub_id,
    //              "livestreaming_dag_sub",
    //              ChannelRole::Admin
    //          )],
    //          &[(livestreaming_dag_sub_id, livestreaming_id)]
    //      )
    //  );

    //  let result = db.get_channels_for_user(a_id).await.unwrap();
    //  assert_dag(
    //      result.channels,
    //      &[
    //          (zed_id, None),
    //          (crdb_id, Some(zed_id)),
    //          (gpui2_id, Some(zed_id)),
    //          (livestreaming_id, Some(zed_id)),
    //          (livestreaming_id, Some(crdb_id)),
    //          (livestreaming_dag_id, Some(livestreaming_id)),
    //          (livestreaming_dag_sub_id, Some(livestreaming_id)),
    //          (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
    //      ],
    //  );

    //  // ========================================================================
    //  // Test a complex DAG by making another link
    //  let returned_channels = db
    //      .link_channel(a_id, livestreaming_id, gpui2_id)
    //      .await
    //      .unwrap();

    //  // DAG is now:
    //  //    /- gpui2 -\             /---------------------\
    //  // zed - crdb -- livestreaming - livestreaming_dag - livestreaming_dag_sub_id
    //  //    \---------/

    //  // Make sure that we're correctly getting the full sub-dag
    //  pretty_assertions::assert_eq!(
    //      returned_channels,
    //      graph(
    //          &[
    //              (livestreaming_id, "livestreaming", ChannelRole::Admin),
    //              (
    //                  livestreaming_dag_id,
    //                  "livestreaming_dag",
    //                  ChannelRole::Admin
    //              ),
    //              (
    //                  livestreaming_dag_sub_id,
    //                  "livestreaming_dag_sub",
    //                  ChannelRole::Admin
    //              ),
    //          ],
    //          &[
    //              (livestreaming_id, gpui2_id),
    //              (livestreaming_dag_id, livestreaming_id),
    //              (livestreaming_dag_sub_id, livestreaming_id),
    //              (livestreaming_dag_sub_id, livestreaming_dag_id),
    //          ]
    //      )
    //  );

    //  let result = db.get_channels_for_user(a_id).await.unwrap();
    //  assert_dag(
    //      result.channels,
    //      &[
    //          (zed_id, None),
    //          (crdb_id, Some(zed_id)),
    //          (gpui2_id, Some(zed_id)),
    //          (livestreaming_id, Some(zed_id)),
    //          (livestreaming_id, Some(crdb_id)),
    //          (livestreaming_id, Some(gpui2_id)),
    //          (livestreaming_dag_id, Some(livestreaming_id)),
    //          (livestreaming_dag_sub_id, Some(livestreaming_id)),
    //          (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
    //      ],
    //  );

    //  // ========================================================================
    //  // Test unlinking in a complex DAG by removing the inner link
    //  db.unlink_channel(a_id, livestreaming_dag_sub_id, livestreaming_id)
    //      .await
    //      .unwrap();

    //  // DAG is now:
    //  //    /- gpui2 -\
    //  // zed - crdb -- livestreaming - livestreaming_dag - livestreaming_dag_sub
    //  //    \---------/

    //  let result = db.get_channels_for_user(a_id).await.unwrap();
    //  assert_dag(
    //      result.channels,
    //      &[
    //          (zed_id, None),
    //          (crdb_id, Some(zed_id)),
    //          (gpui2_id, Some(zed_id)),
    //          (livestreaming_id, Some(gpui2_id)),
    //          (livestreaming_id, Some(zed_id)),
    //          (livestreaming_id, Some(crdb_id)),
    //          (livestreaming_dag_id, Some(livestreaming_id)),
    //          (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
    //      ],
    //  );

    //  // ========================================================================
    //  // Test unlinking in a complex DAG by removing the inner link
    //  db.unlink_channel(a_id, livestreaming_id, gpui2_id)
    //      .await
    //      .unwrap();

    //  // DAG is now:
    //  //    /- gpui2
    //  // zed - crdb -- livestreaming - livestreaming_dag - livestreaming_dag_sub
    //  //    \---------/
    //  let result = db.get_channels_for_user(a_id).await.unwrap();
    //  assert_dag(
    //      result.channels,
    //      &[
    //          (zed_id, None),
    //          (crdb_id, Some(zed_id)),
    //          (gpui2_id, Some(zed_id)),
    //          (livestreaming_id, Some(zed_id)),
    //          (livestreaming_id, Some(crdb_id)),
    //          (livestreaming_dag_id, Some(livestreaming_id)),
    //          (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
    //      ],
    //  );

    //  // ========================================================================
    //  // Test moving DAG nodes by moving livestreaming to be below gpui2
    //  db.move_channel(livestreaming_id, Some(crdb_id), gpui2_id, a_id)
    //      .await
    //      .unwrap();

    //  // DAG is now:
    //  //    /- gpui2 -- livestreaming - livestreaming_dag - livestreaming_dag_sub
    //  // zed - crdb    /
    //  //    \---------/
    //  let result = db.get_channels_for_user(a_id).await.unwrap();
    //  assert_dag(
    //      result.channels,
    //      &[
    //          (zed_id, None),
    //          (crdb_id, Some(zed_id)),
    //          (gpui2_id, Some(zed_id)),
    //          (livestreaming_id, Some(zed_id)),
    //          (livestreaming_id, Some(gpui2_id)),
    //          (livestreaming_dag_id, Some(livestreaming_id)),
    //          (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
    //      ],
    //  );

    //  // ========================================================================
    //  // Deleting a channel should not delete children that still have other parents
    //  db.delete_channel(gpui2_id, a_id).await.unwrap();

    //  // DAG is now:
    //  // zed - crdb
    //  //    \- livestreaming - livestreaming_dag - livestreaming_dag_sub
    //  let result = db.get_channels_for_user(a_id).await.unwrap();
    //  assert_dag(
    //      result.channels,
    //      &[
    //          (zed_id, None),
    //          (crdb_id, Some(zed_id)),
    //          (livestreaming_id, Some(zed_id)),
    //          (livestreaming_dag_id, Some(livestreaming_id)),
    //          (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
    //      ],
    //  );

    //  // ========================================================================
    //  // Unlinking a channel from it's parent should automatically promote it to a root channel
    //  db.unlink_channel(a_id, crdb_id, zed_id).await.unwrap();

    //  // DAG is now:
    //  // crdb
    //  // zed
    //  //    \- livestreaming - livestreaming_dag - livestreaming_dag_sub

    //  let result = db.get_channels_for_user(a_id).await.unwrap();
    //  assert_dag(
    //      result.channels,
    //      &[
    //          (zed_id, None),
    //          (crdb_id, None),
    //          (livestreaming_id, Some(zed_id)),
    //          (livestreaming_dag_id, Some(livestreaming_id)),
    //          (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
    //      ],
    //  );

    //  // ========================================================================
    //  // You should be able to move a root channel into a non-root channel
    //  db.link_channel(a_id, crdb_id, zed_id).await.unwrap();

    //  // DAG is now:
    //  // zed - crdb
    //  //    \- livestreaming - livestreaming_dag - livestreaming_dag_sub

    //  let result = db.get_channels_for_user(a_id).await.unwrap();
    //  assert_dag(
    //      result.channels,
    //      &[
    //          (zed_id, None),
    //          (crdb_id, Some(zed_id)),
    //          (livestreaming_id, Some(zed_id)),
    //          (livestreaming_dag_id, Some(livestreaming_id)),
    //          (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
    //      ],
    //  );

    //  // ========================================================================
    //  // Prep for DAG deletion test
    //  db.link_channel(a_id, livestreaming_id, crdb_id)
    //      .await
    //      .unwrap();

    //  // DAG is now:
    //  // zed - crdb - livestreaming - livestreaming_dag - livestreaming_dag_sub
    //  //    \--------/

    //  let result = db.get_channels_for_user(a_id).await.unwrap();
    //  assert_dag(
    //      result.channels,
    //      &[
    //          (zed_id, None),
    //          (crdb_id, Some(zed_id)),
    //          (livestreaming_id, Some(zed_id)),
    //          (livestreaming_id, Some(crdb_id)),
    //          (livestreaming_dag_id, Some(livestreaming_id)),
    //          (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
    //      ],
    //  );

    //  // Deleting the parent of a DAG should delete the whole DAG:
    //  db.delete_channel(zed_id, a_id).await.unwrap();
    //  let result = db.get_channels_for_user(a_id).await.unwrap();

    //  assert!(result.channels.is_empty())
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
            false,
            NewUserParams {
                github_login: "user1".into(),
                github_user_id: 5,
                invite_count: 0,
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

    // Dag is: zed - projects - livestreaming

    // Move to same parent should be a no-op
    assert!(db
        .move_channel(projects_id, Some(zed_id), zed_id, user_id)
        .await
        .unwrap()
        .is_none());

    let result = db.get_channels_for_user(user_id).await.unwrap();
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (projects_id, Some(zed_id)),
            (livestreaming_id, Some(projects_id)),
        ],
    );

    // Stranding a channel should retain it's sub channels
    // Commented out as we don't fix permissions when this happens yet.
    //
    // db.unlink_channel(user_id, projects_id, zed_id)
    //     .await
    //     .unwrap();

    // let result = db.get_channels_for_user(user_id).await.unwrap();
    // assert_dag(
    //     result.channels,
    //     &[
    //         (zed_id, None),
    //         (projects_id, None),
    //         (livestreaming_id, Some(projects_id)),
    //     ],
    // );
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
    let active_channel = db
        .create_sub_channel("active", zed_channel, admin)
        .await
        .unwrap();
    let vim_channel = db
        .create_sub_channel("vim", active_channel, admin)
        .await
        .unwrap();

    db.set_channel_visibility(vim_channel, crate::db::ChannelVisibility::Public, admin)
        .await
        .unwrap();
    db.invite_channel_member(active_channel, member, admin, ChannelRole::Member)
        .await
        .unwrap();
    db.invite_channel_member(vim_channel, guest, admin, ChannelRole::Guest)
        .await
        .unwrap();

    db.respond_to_channel_invite(active_channel, member, true)
        .await
        .unwrap();

    db.transaction(|tx| async move {
        db.check_user_is_channel_participant(vim_channel, admin, &*tx)
            .await
    })
    .await
    .unwrap();
    db.transaction(|tx| async move {
        db.check_user_is_channel_participant(vim_channel, member, &*tx)
            .await
    })
    .await
    .unwrap();

    let mut members = db
        .get_channel_participant_details(vim_channel, admin)
        .await
        .unwrap();

    members.sort_by_key(|member| member.user_id);

    assert_eq!(
        members,
        &[
            proto::ChannelMember {
                user_id: admin.to_proto(),
                kind: proto::channel_member::Kind::AncestorMember.into(),
                role: proto::ChannelRole::Admin.into(),
            },
            proto::ChannelMember {
                user_id: member.to_proto(),
                kind: proto::channel_member::Kind::AncestorMember.into(),
                role: proto::ChannelRole::Member.into(),
            },
            proto::ChannelMember {
                user_id: guest.to_proto(),
                kind: proto::channel_member::Kind::Invitee.into(),
                role: proto::ChannelRole::Guest.into(),
            },
        ]
    );

    db.respond_to_channel_invite(vim_channel, guest, true)
        .await
        .unwrap();

    db.transaction(|tx| async move {
        db.check_user_is_channel_participant(vim_channel, guest, &*tx)
            .await
    })
    .await
    .unwrap();

    let channels = db.get_channels_for_user(guest).await.unwrap().channels;
    assert_dag(channels, &[(vim_channel, None)]);
    let channels = db.get_channels_for_user(member).await.unwrap().channels;
    assert_dag(
        channels,
        &[(active_channel, None), (vim_channel, Some(active_channel))],
    );

    db.set_channel_member_role(vim_channel, admin, guest, ChannelRole::Banned)
        .await
        .unwrap();
    assert!(db
        .transaction(|tx| async move {
            db.check_user_is_channel_participant(vim_channel, guest, &*tx)
                .await
        })
        .await
        .is_err());

    let mut members = db
        .get_channel_participant_details(vim_channel, admin)
        .await
        .unwrap();

    members.sort_by_key(|member| member.user_id);

    assert_eq!(
        members,
        &[
            proto::ChannelMember {
                user_id: admin.to_proto(),
                kind: proto::channel_member::Kind::AncestorMember.into(),
                role: proto::ChannelRole::Admin.into(),
            },
            proto::ChannelMember {
                user_id: member.to_proto(),
                kind: proto::channel_member::Kind::AncestorMember.into(),
                role: proto::ChannelRole::Member.into(),
            },
            proto::ChannelMember {
                user_id: guest.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                role: proto::ChannelRole::Banned.into(),
            },
        ]
    );

    db.remove_channel_member(vim_channel, guest, admin)
        .await
        .unwrap();

    db.set_channel_visibility(zed_channel, crate::db::ChannelVisibility::Public, admin)
        .await
        .unwrap();

    db.invite_channel_member(zed_channel, guest, admin, ChannelRole::Guest)
        .await
        .unwrap();

    // currently people invited to parent channels are not shown here
    let mut members = db
        .get_channel_participant_details(vim_channel, admin)
        .await
        .unwrap();

    members.sort_by_key(|member| member.user_id);

    assert_eq!(
        members,
        &[
            proto::ChannelMember {
                user_id: admin.to_proto(),
                kind: proto::channel_member::Kind::AncestorMember.into(),
                role: proto::ChannelRole::Admin.into(),
            },
            proto::ChannelMember {
                user_id: member.to_proto(),
                kind: proto::channel_member::Kind::AncestorMember.into(),
                role: proto::ChannelRole::Member.into(),
            },
        ]
    );

    db.respond_to_channel_invite(zed_channel, guest, true)
        .await
        .unwrap();

    db.transaction(|tx| async move {
        db.check_user_is_channel_participant(zed_channel, guest, &*tx)
            .await
    })
    .await
    .unwrap();
    assert!(db
        .transaction(|tx| async move {
            db.check_user_is_channel_participant(active_channel, guest, &*tx)
                .await
        })
        .await
        .is_err(),);

    db.transaction(|tx| async move {
        db.check_user_is_channel_participant(vim_channel, guest, &*tx)
            .await
    })
    .await
    .unwrap();

    let mut members = db
        .get_channel_participant_details(vim_channel, admin)
        .await
        .unwrap();

    members.sort_by_key(|member| member.user_id);

    assert_eq!(
        members,
        &[
            proto::ChannelMember {
                user_id: admin.to_proto(),
                kind: proto::channel_member::Kind::AncestorMember.into(),
                role: proto::ChannelRole::Admin.into(),
            },
            proto::ChannelMember {
                user_id: member.to_proto(),
                kind: proto::channel_member::Kind::AncestorMember.into(),
                role: proto::ChannelRole::Member.into(),
            },
            proto::ChannelMember {
                user_id: guest.to_proto(),
                kind: proto::channel_member::Kind::AncestorMember.into(),
                role: proto::ChannelRole::Guest.into(),
            },
        ]
    );

    let channels = db.get_channels_for_user(guest).await.unwrap().channels;
    assert_dag(
        channels,
        &[(zed_channel, None), (vim_channel, Some(zed_channel))],
    )
}

test_both_dbs!(
    test_user_joins_correct_channel,
    test_user_joins_correct_channel_postgres,
    test_user_joins_correct_channel_sqlite
);

async fn test_user_joins_correct_channel(db: &Arc<Database>) {
    let admin = new_test_user(db, "admin@example.com").await;

    let zed_channel = db.create_root_channel("zed", admin).await.unwrap();

    let active_channel = db
        .create_sub_channel("active", zed_channel, admin)
        .await
        .unwrap();

    let vim_channel = db
        .create_sub_channel("vim", active_channel, admin)
        .await
        .unwrap();

    let vim2_channel = db
        .create_sub_channel("vim2", vim_channel, admin)
        .await
        .unwrap();

    db.set_channel_visibility(zed_channel, crate::db::ChannelVisibility::Public, admin)
        .await
        .unwrap();

    db.set_channel_visibility(vim_channel, crate::db::ChannelVisibility::Public, admin)
        .await
        .unwrap();

    db.set_channel_visibility(vim2_channel, crate::db::ChannelVisibility::Public, admin)
        .await
        .unwrap();

    let most_public = db
        .transaction(|tx| async move {
            Ok(db
                .public_path_to_channel(vim_channel, &tx)
                .await?
                .first()
                .cloned())
        })
        .await
        .unwrap();

    assert_eq!(most_public, Some(zed_channel))
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

    assert!(db
        .join_channel_chat(zed_channel, guest_connection, guest)
        .await
        .is_err());

    db.join_channel(zed_channel, guest, guest_connection, TEST_RELEASE_CHANNEL)
        .await
        .unwrap();

    assert!(db
        .join_channel_chat(zed_channel, guest_connection, guest)
        .await
        .is_ok())
}

#[track_caller]
fn assert_dag(actual: ChannelGraph, expected: &[(ChannelId, Option<ChannelId>)]) {
    let mut actual_map: HashMap<ChannelId, HashSet<ChannelId>> = HashMap::default();
    for channel in actual.channels {
        actual_map.insert(channel.id, HashSet::default());
    }
    for edge in actual.edges {
        actual_map
            .get_mut(&ChannelId::from_proto(edge.channel_id))
            .unwrap()
            .insert(ChannelId::from_proto(edge.parent_id));
    }

    let mut expected_map: HashMap<ChannelId, HashSet<ChannelId>> = HashMap::default();

    for (child, parent) in expected {
        let entry = expected_map.entry(*child).or_default();
        if let Some(parent) = parent {
            entry.insert(*parent);
        }
    }

    pretty_assertions::assert_eq!(actual_map, expected_map)
}

static GITHUB_USER_ID: AtomicI32 = AtomicI32::new(5);

async fn new_test_user(db: &Arc<Database>, email: &str) -> UserId {
    db.create_user(
        email,
        false,
        NewUserParams {
            github_login: email[0..email.find("@").unwrap()].to_string(),
            github_user_id: GITHUB_USER_ID.fetch_add(1, Ordering::SeqCst),
            invite_count: 0,
        },
    )
    .await
    .unwrap()
    .user_id
}

static TEST_CONNECTION_ID: AtomicU32 = AtomicU32::new(1);
fn new_test_connection(server: ServerId) -> ConnectionId {
    ConnectionId {
        id: TEST_CONNECTION_ID.fetch_add(1, Ordering::SeqCst),
        owner_id: server.0 as u32,
    }
}
