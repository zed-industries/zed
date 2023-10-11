use collections::{HashMap, HashSet};
use rpc::{
    proto::{self},
    ConnectionId,
};

use crate::{
    db::{
        queries::channels::ChannelGraph,
        tests::{graph, TEST_RELEASE_CHANNEL},
        ChannelId, ChannelRole, Database, NewUserParams,
    },
    test_both_dbs,
};
use std::sync::Arc;

test_both_dbs!(test_channels, test_channels_postgres, test_channels_sqlite);

async fn test_channels(db: &Arc<Database>) {
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

    let b_id = db
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

    let zed_id = db.create_root_channel("zed", a_id).await.unwrap();

    // Make sure that people cannot read channels they haven't been invited to
    assert!(db.get_channel(zed_id, b_id).await.unwrap().is_none());

    db.invite_channel_member(zed_id, b_id, a_id, ChannelRole::Member)
        .await
        .unwrap();

    db.respond_to_channel_invite(zed_id, b_id, true)
        .await
        .unwrap();

    let crdb_id = db.create_channel("crdb", Some(zed_id), a_id).await.unwrap();
    let livestreaming_id = db
        .create_channel("livestreaming", Some(zed_id), a_id)
        .await
        .unwrap();
    let replace_id = db
        .create_channel("replace", Some(zed_id), a_id)
        .await
        .unwrap();

    let mut members = db.get_channel_members(replace_id).await.unwrap();
    members.sort();
    assert_eq!(members, &[a_id, b_id]);

    let rust_id = db.create_root_channel("rust", a_id).await.unwrap();
    let cargo_id = db
        .create_channel("cargo", Some(rust_id), a_id)
        .await
        .unwrap();

    let cargo_ra_id = db
        .create_channel("cargo-ra", Some(cargo_id), a_id)
        .await
        .unwrap();

    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_eq!(
        result.channels,
        graph(
            &[
                (zed_id, "zed"),
                (crdb_id, "crdb"),
                (livestreaming_id, "livestreaming"),
                (replace_id, "replace"),
                (rust_id, "rust"),
                (cargo_id, "cargo"),
                (cargo_ra_id, "cargo-ra")
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
                (zed_id, "zed"),
                (crdb_id, "crdb"),
                (livestreaming_id, "livestreaming"),
                (replace_id, "replace")
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
                (zed_id, "zed"),
                (crdb_id, "crdb"),
                (livestreaming_id, "livestreaming"),
                (replace_id, "replace")
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
    assert!(db.get_channel(crdb_id, a_id).await.unwrap().is_none());

    // Remove a channel tree
    let (mut channel_ids, user_ids) = db.delete_channel(rust_id, a_id).await.unwrap();
    channel_ids.sort();
    assert_eq!(channel_ids, &[rust_id, cargo_id, cargo_ra_id]);
    assert_eq!(user_ids, &[a_id]);

    assert!(db.get_channel(rust_id, a_id).await.unwrap().is_none());
    assert!(db.get_channel(cargo_id, a_id).await.unwrap().is_none());
    assert!(db.get_channel(cargo_ra_id, a_id).await.unwrap().is_none());
}

test_both_dbs!(
    test_joining_channels,
    test_joining_channels_postgres,
    test_joining_channels_sqlite
);

async fn test_joining_channels(db: &Arc<Database>) {
    let owner_id = db.create_server("test").await.unwrap().0 as u32;

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

    let channel_1 = db.create_root_channel("channel_1", user_1).await.unwrap();
    let room_1 = db
        .get_or_create_channel_room(channel_1, "1", TEST_RELEASE_CHANNEL)
        .await
        .unwrap();

    // can join a room with membership to its channel
    let joined_room = db
        .join_room(
            room_1,
            user_1,
            ConnectionId { owner_id, id: 1 },
            TEST_RELEASE_CHANNEL,
        )
        .await
        .unwrap();
    assert_eq!(joined_room.room.participants.len(), 1);

    drop(joined_room);
    // cannot join a room without membership to its channel
    assert!(db
        .join_room(
            room_1,
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

    let user_3 = db
        .create_user(
            "user3@example.com",
            false,
            NewUserParams {
                github_login: "user3".into(),
                github_user_id: 7,
                invite_count: 0,
            },
        )
        .await
        .unwrap()
        .user_id;

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

    let members = db
        .get_channel_member_details(channel_1_1, user_1)
        .await
        .unwrap();
    assert_eq!(
        members,
        &[
            proto::ChannelMember {
                user_id: user_1.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                admin: true,
            },
            proto::ChannelMember {
                user_id: user_2.to_proto(),
                kind: proto::channel_member::Kind::Invitee.into(),
                admin: false,
            },
            proto::ChannelMember {
                user_id: user_3.to_proto(),
                kind: proto::channel_member::Kind::Invitee.into(),
                admin: true,
            },
        ]
    );

    db.respond_to_channel_invite(channel_1_1, user_2, true)
        .await
        .unwrap();

    let channel_1_3 = db
        .create_channel("channel_3", Some(channel_1_1), user_1)
        .await
        .unwrap();

    let members = db
        .get_channel_member_details(channel_1_3, user_1)
        .await
        .unwrap();
    assert_eq!(
        members,
        &[
            proto::ChannelMember {
                user_id: user_1.to_proto(),
                kind: proto::channel_member::Kind::Member.into(),
                admin: true,
            },
            proto::ChannelMember {
                user_id: user_2.to_proto(),
                kind: proto::channel_member::Kind::AncestorMember.into(),
                admin: false,
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

    let (channel, _) = db
        .get_channel(zed_archive_id, user_1)
        .await
        .unwrap()
        .unwrap();
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

    let crdb_id = db.create_channel("crdb", Some(zed_id), a_id).await.unwrap();

    let gpui2_id = db
        .create_channel("gpui2", Some(zed_id), a_id)
        .await
        .unwrap();

    let livestreaming_id = db
        .create_channel("livestreaming", Some(crdb_id), a_id)
        .await
        .unwrap();

    let livestreaming_dag_id = db
        .create_channel("livestreaming_dag", Some(livestreaming_id), a_id)
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

    // ========================================================================
    // Make a link
    db.link_channel(a_id, livestreaming_id, zed_id)
        .await
        .unwrap();

    // DAG is now:
    //     /- gpui2
    // zed -- crdb - livestreaming - livestreaming_dag
    //    \---------/
    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (crdb_id, Some(zed_id)),
            (gpui2_id, Some(zed_id)),
            (livestreaming_id, Some(zed_id)),
            (livestreaming_id, Some(crdb_id)),
            (livestreaming_dag_id, Some(livestreaming_id)),
        ],
    );

    // ========================================================================
    // Create a new channel below a channel with multiple parents
    let livestreaming_dag_sub_id = db
        .create_channel("livestreaming_dag_sub", Some(livestreaming_dag_id), a_id)
        .await
        .unwrap();

    // DAG is now:
    //     /- gpui2
    // zed -- crdb - livestreaming - livestreaming_dag - livestreaming_dag_sub_id
    //    \---------/
    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (crdb_id, Some(zed_id)),
            (gpui2_id, Some(zed_id)),
            (livestreaming_id, Some(zed_id)),
            (livestreaming_id, Some(crdb_id)),
            (livestreaming_dag_id, Some(livestreaming_id)),
            (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
        ],
    );

    // ========================================================================
    // Test a complex DAG by making another link
    let returned_channels = db
        .link_channel(a_id, livestreaming_dag_sub_id, livestreaming_id)
        .await
        .unwrap();

    // DAG is now:
    //    /- gpui2                /---------------------\
    // zed - crdb - livestreaming - livestreaming_dag - livestreaming_dag_sub_id
    //    \--------/

    // make sure we're getting just the new link
    // Not using the assert_dag helper because we want to make sure we're returning the full data
    pretty_assertions::assert_eq!(
        returned_channels,
        graph(
            &[(livestreaming_dag_sub_id, "livestreaming_dag_sub")],
            &[(livestreaming_dag_sub_id, livestreaming_id)]
        )
    );

    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (crdb_id, Some(zed_id)),
            (gpui2_id, Some(zed_id)),
            (livestreaming_id, Some(zed_id)),
            (livestreaming_id, Some(crdb_id)),
            (livestreaming_dag_id, Some(livestreaming_id)),
            (livestreaming_dag_sub_id, Some(livestreaming_id)),
            (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
        ],
    );

    // ========================================================================
    // Test a complex DAG by making another link
    let returned_channels = db
        .link_channel(a_id, livestreaming_id, gpui2_id)
        .await
        .unwrap();

    // DAG is now:
    //    /- gpui2 -\             /---------------------\
    // zed - crdb -- livestreaming - livestreaming_dag - livestreaming_dag_sub_id
    //    \---------/

    // Make sure that we're correctly getting the full sub-dag
    pretty_assertions::assert_eq!(
        returned_channels,
        graph(
            &[
                (livestreaming_id, "livestreaming"),
                (livestreaming_dag_id, "livestreaming_dag"),
                (livestreaming_dag_sub_id, "livestreaming_dag_sub"),
            ],
            &[
                (livestreaming_id, gpui2_id),
                (livestreaming_dag_id, livestreaming_id),
                (livestreaming_dag_sub_id, livestreaming_id),
                (livestreaming_dag_sub_id, livestreaming_dag_id),
            ]
        )
    );

    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (crdb_id, Some(zed_id)),
            (gpui2_id, Some(zed_id)),
            (livestreaming_id, Some(zed_id)),
            (livestreaming_id, Some(crdb_id)),
            (livestreaming_id, Some(gpui2_id)),
            (livestreaming_dag_id, Some(livestreaming_id)),
            (livestreaming_dag_sub_id, Some(livestreaming_id)),
            (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
        ],
    );

    // ========================================================================
    // Test unlinking in a complex DAG by removing the inner link
    db.unlink_channel(a_id, livestreaming_dag_sub_id, livestreaming_id)
        .await
        .unwrap();

    // DAG is now:
    //    /- gpui2 -\
    // zed - crdb -- livestreaming - livestreaming_dag - livestreaming_dag_sub
    //    \---------/

    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (crdb_id, Some(zed_id)),
            (gpui2_id, Some(zed_id)),
            (livestreaming_id, Some(gpui2_id)),
            (livestreaming_id, Some(zed_id)),
            (livestreaming_id, Some(crdb_id)),
            (livestreaming_dag_id, Some(livestreaming_id)),
            (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
        ],
    );

    // ========================================================================
    // Test unlinking in a complex DAG by removing the inner link
    db.unlink_channel(a_id, livestreaming_id, gpui2_id)
        .await
        .unwrap();

    // DAG is now:
    //    /- gpui2
    // zed - crdb -- livestreaming - livestreaming_dag - livestreaming_dag_sub
    //    \---------/
    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (crdb_id, Some(zed_id)),
            (gpui2_id, Some(zed_id)),
            (livestreaming_id, Some(zed_id)),
            (livestreaming_id, Some(crdb_id)),
            (livestreaming_dag_id, Some(livestreaming_id)),
            (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
        ],
    );

    // ========================================================================
    // Test moving DAG nodes by moving livestreaming to be below gpui2
    db.move_channel(a_id, livestreaming_id, crdb_id, gpui2_id)
        .await
        .unwrap();

    // DAG is now:
    //    /- gpui2 -- livestreaming - livestreaming_dag - livestreaming_dag_sub
    // zed - crdb    /
    //    \---------/
    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (crdb_id, Some(zed_id)),
            (gpui2_id, Some(zed_id)),
            (livestreaming_id, Some(zed_id)),
            (livestreaming_id, Some(gpui2_id)),
            (livestreaming_dag_id, Some(livestreaming_id)),
            (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
        ],
    );

    // ========================================================================
    // Deleting a channel should not delete children that still have other parents
    db.delete_channel(gpui2_id, a_id).await.unwrap();

    // DAG is now:
    // zed - crdb
    //    \- livestreaming - livestreaming_dag - livestreaming_dag_sub
    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (crdb_id, Some(zed_id)),
            (livestreaming_id, Some(zed_id)),
            (livestreaming_dag_id, Some(livestreaming_id)),
            (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
        ],
    );

    // ========================================================================
    // Unlinking a channel from it's parent should automatically promote it to a root channel
    db.unlink_channel(a_id, crdb_id, zed_id).await.unwrap();

    // DAG is now:
    // crdb
    // zed
    //    \- livestreaming - livestreaming_dag - livestreaming_dag_sub

    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (crdb_id, None),
            (livestreaming_id, Some(zed_id)),
            (livestreaming_dag_id, Some(livestreaming_id)),
            (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
        ],
    );

    // ========================================================================
    // You should be able to move a root channel into a non-root channel
    db.link_channel(a_id, crdb_id, zed_id).await.unwrap();

    // DAG is now:
    // zed - crdb
    //    \- livestreaming - livestreaming_dag - livestreaming_dag_sub

    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (crdb_id, Some(zed_id)),
            (livestreaming_id, Some(zed_id)),
            (livestreaming_dag_id, Some(livestreaming_id)),
            (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
        ],
    );

    // ========================================================================
    // Prep for DAG deletion test
    db.link_channel(a_id, livestreaming_id, crdb_id)
        .await
        .unwrap();

    // DAG is now:
    // zed - crdb - livestreaming - livestreaming_dag - livestreaming_dag_sub
    //    \--------/

    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (crdb_id, Some(zed_id)),
            (livestreaming_id, Some(zed_id)),
            (livestreaming_id, Some(crdb_id)),
            (livestreaming_dag_id, Some(livestreaming_id)),
            (livestreaming_dag_sub_id, Some(livestreaming_dag_id)),
        ],
    );

    // Deleting the parent of a DAG should delete the whole DAG:
    db.delete_channel(zed_id, a_id).await.unwrap();
    let result = db.get_channels_for_user(a_id).await.unwrap();

    assert!(result.channels.is_empty())
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
        .create_channel("projects", Some(zed_id), user_id)
        .await
        .unwrap();

    let livestreaming_id = db
        .create_channel("livestreaming", Some(projects_id), user_id)
        .await
        .unwrap();

    // Dag is: zed - projects - livestreaming

    // Move to same parent should be a no-op
    assert!(db
        .move_channel(user_id, projects_id, zed_id, zed_id)
        .await
        .unwrap()
        .is_empty());

    // Stranding a channel should retain it's sub channels
    db.unlink_channel(user_id, projects_id, zed_id)
        .await
        .unwrap();

    let result = db.get_channels_for_user(user_id).await.unwrap();
    assert_dag(
        result.channels,
        &[
            (zed_id, None),
            (projects_id, None),
            (livestreaming_id, Some(projects_id)),
        ],
    );
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
