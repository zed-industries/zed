use rpc::{proto, ConnectionId};

use crate::{
    db::{Channel, Database, NewUserParams},
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

    let zed_id = db.create_root_channel("zed", "1", a_id).await.unwrap();

    // Make sure that people cannot read channels they haven't been invited to
    assert!(db.get_channel(zed_id, b_id).await.unwrap().is_none());

    db.invite_channel_member(zed_id, b_id, a_id, false)
        .await
        .unwrap();

    db.respond_to_channel_invite(zed_id, b_id, true)
        .await
        .unwrap();

    let crdb_id = db
        .create_channel("crdb", Some(zed_id), "2", a_id)
        .await
        .unwrap();
    let livestreaming_id = db
        .create_channel("livestreaming", Some(zed_id), "3", a_id)
        .await
        .unwrap();
    let replace_id = db
        .create_channel("replace", Some(zed_id), "4", a_id)
        .await
        .unwrap();

    let mut members = db.get_channel_members(replace_id).await.unwrap();
    members.sort();
    assert_eq!(members, &[a_id, b_id]);

    let rust_id = db.create_root_channel("rust", "5", a_id).await.unwrap();
    let cargo_id = db
        .create_channel("cargo", Some(rust_id), "6", a_id)
        .await
        .unwrap();

    let cargo_ra_id = db
        .create_channel("cargo-ra", Some(cargo_id), "7", a_id)
        .await
        .unwrap();

    let result = db.get_channels_for_user(a_id).await.unwrap();
    assert_eq!(
        result.channels,
        vec![
            Channel {
                id: zed_id,
                name: "zed".to_string(),
                parent_id: None,
            },
            Channel {
                id: crdb_id,
                name: "crdb".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: replace_id,
                name: "replace".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: rust_id,
                name: "rust".to_string(),
                parent_id: None,
            },
            Channel {
                id: cargo_id,
                name: "cargo".to_string(),
                parent_id: Some(rust_id),
            },
            Channel {
                id: cargo_ra_id,
                name: "cargo-ra".to_string(),
                parent_id: Some(cargo_id),
            }
        ]
    );

    let result = db.get_channels_for_user(b_id).await.unwrap();
    assert_eq!(
        result.channels,
        vec![
            Channel {
                id: zed_id,
                name: "zed".to_string(),
                parent_id: None,
            },
            Channel {
                id: crdb_id,
                name: "crdb".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: replace_id,
                name: "replace".to_string(),
                parent_id: Some(zed_id),
            },
        ]
    );

    // Update member permissions
    let set_subchannel_admin = db.set_channel_member_admin(crdb_id, a_id, b_id, true).await;
    assert!(set_subchannel_admin.is_err());
    let set_channel_admin = db.set_channel_member_admin(zed_id, a_id, b_id, true).await;
    assert!(set_channel_admin.is_ok());

    let result = db.get_channels_for_user(b_id).await.unwrap();
    assert_eq!(
        result.channels,
        vec![
            Channel {
                id: zed_id,
                name: "zed".to_string(),
                parent_id: None,
            },
            Channel {
                id: crdb_id,
                name: "crdb".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: replace_id,
                name: "replace".to_string(),
                parent_id: Some(zed_id),
            },
        ]
    );

    // Remove a single channel
    db.remove_channel(crdb_id, a_id).await.unwrap();
    assert!(db.get_channel(crdb_id, a_id).await.unwrap().is_none());

    // Remove a channel tree
    let (mut channel_ids, user_ids) = db.remove_channel(rust_id, a_id).await.unwrap();
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

    let channel_1 = db
        .create_root_channel("channel_1", "1", user_1)
        .await
        .unwrap();
    let room_1 = db.room_id_for_channel(channel_1).await.unwrap();

    // can join a room with membership to its channel
    let joined_room = db
        .join_room(room_1, user_1, ConnectionId { owner_id, id: 1 })
        .await
        .unwrap();
    assert_eq!(joined_room.room.participants.len(), 1);

    drop(joined_room);
    // cannot join a room without membership to its channel
    assert!(db
        .join_room(room_1, user_2, ConnectionId { owner_id, id: 1 })
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

    let channel_1_1 = db
        .create_root_channel("channel_1", "1", user_1)
        .await
        .unwrap();

    let channel_1_2 = db
        .create_root_channel("channel_2", "2", user_1)
        .await
        .unwrap();

    db.invite_channel_member(channel_1_1, user_2, user_1, false)
        .await
        .unwrap();
    db.invite_channel_member(channel_1_2, user_2, user_1, false)
        .await
        .unwrap();
    db.invite_channel_member(channel_1_1, user_3, user_1, true)
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
        .create_channel("channel_3", Some(channel_1_1), "1", user_1)
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

    let zed_id = db.create_root_channel("zed", "1", user_1).await.unwrap();

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
    test_channels_moving,
    test_channels_moving_postgres,
    test_channels_moving_sqlite
);

async fn test_channels_moving(db: &Arc<Database>) {
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

    let zed_id = db.create_root_channel("zed", "1", a_id).await.unwrap();

    let crdb_id = db
        .create_channel("crdb", Some(zed_id), "2", a_id)
        .await
        .unwrap();

    let gpui2_id = db
        .create_channel("gpui2", Some(zed_id), "3", a_id)
        .await
        .unwrap();

    let livestreaming_id = db
        .create_channel("livestreaming", Some(crdb_id), "4", a_id)
        .await
        .unwrap();

    let livestreaming_dag_id = db
        .create_channel("livestreaming_dag", Some(livestreaming_id), "5", a_id)
        .await
        .unwrap();

    // sanity check
    let result = db.get_channels_for_user(a_id).await.unwrap();
    pretty_assertions::assert_eq!(
        result.channels,
        vec![
            Channel {
                id: zed_id,
                name: "zed".to_string(),
                parent_id: None,
            },
            Channel {
                id: crdb_id,
                name: "crdb".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: gpui2_id,
                name: "gpui2".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(crdb_id),
            },
            Channel {
                id: livestreaming_dag_id,
                name: "livestreaming_dag".to_string(),
                parent_id: Some(livestreaming_id),
            },
        ]
    );
    // Initial DAG:
    //     /- gpui2
    // zed -- crdb - livestreaming - livestreaming_dag

    // Attemp to make a cycle
    assert!(db
        .link_channel(a_id, zed_id, livestreaming_id)
        .await
        .is_err());

    // Make a link
    db.link_channel(a_id, livestreaming_id, zed_id)
        .await
        .unwrap();

    // DAG is now:
    //     /- gpui2
    // zed -- crdb - livestreaming - livestreaming_dag
    //    \---------/

    let result = db.get_channels_for_user(a_id).await.unwrap();
    pretty_assertions::assert_eq!(
        dbg!(result.channels),
        vec![
            Channel {
                id: zed_id,
                name: "zed".to_string(),
                parent_id: None,
            },
            Channel {
                id: crdb_id,
                name: "crdb".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: gpui2_id,
                name: "gpui2".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(crdb_id),
            },
            Channel {
                id: livestreaming_dag_id,
                name: "livestreaming_dag".to_string(),
                parent_id: Some(livestreaming_id),
            },
        ]
    );

    let livestreaming_dag_sub_id = db
        .create_channel("livestreaming_dag_sub", Some(livestreaming_dag_id), "6", a_id)
        .await
        .unwrap();

    // DAG is now:
    //     /- gpui2
    // zed -- crdb - livestreaming - livestreaming_dag - livestreaming_dag_sub_id
    //    \---------/

    let result = db.get_channels_for_user(a_id).await.unwrap();
    pretty_assertions::assert_eq!(
        dbg!(result.channels),
        vec![
            Channel {
                id: zed_id,
                name: "zed".to_string(),
                parent_id: None,
            },
            Channel {
                id: crdb_id,
                name: "crdb".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: gpui2_id,
                name: "gpui2".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(crdb_id),
            },
            Channel {
                id: livestreaming_dag_id,
                name: "livestreaming_dag".to_string(),
                parent_id: Some(livestreaming_id),
            },
            Channel {
                id: livestreaming_dag_sub_id,
                name: "livestreaming_dag_sub".to_string(),
                parent_id: Some(livestreaming_dag_id),
            },
        ]
    );

    // Make a link
    db.link_channel(a_id, livestreaming_dag_sub_id, livestreaming_id)
        .await
        .unwrap();

    // DAG is now:
    //    /- gpui2                /---------------------\
    // zed - crdb - livestreaming - livestreaming_dag - livestreaming_dag_sub_id
    //    \--------/

    let result = db.get_channels_for_user(a_id).await.unwrap();
    pretty_assertions::assert_eq!(
        dbg!(result.channels),
        vec![
            Channel {
                id: zed_id,
                name: "zed".to_string(),
                parent_id: None,
            },
            Channel {
                id: crdb_id,
                name: "crdb".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: gpui2_id,
                name: "gpui2".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(crdb_id),
            },
            Channel {
                id: livestreaming_dag_id,
                name: "livestreaming_dag".to_string(),
                parent_id: Some(livestreaming_id),
            },
            Channel {
                id: livestreaming_dag_sub_id,
                name: "livestreaming_dag_sub".to_string(),
                parent_id: Some(livestreaming_id),
            },
            Channel {
                id: livestreaming_dag_sub_id,
                name: "livestreaming_dag_sub".to_string(),
                parent_id: Some(livestreaming_dag_id),
            },
        ]
    );

    // Make another link
    db.link_channel(a_id, livestreaming_id, gpui2_id)
        .await
        .unwrap();

    // DAG is now:
    //    /- gpui2 -\             /---------------------\
    // zed - crdb -- livestreaming - livestreaming_dag - livestreaming_dag_sub_id
    //    \---------/

    let result = db.get_channels_for_user(a_id).await.unwrap();
    pretty_assertions::assert_eq!(
        dbg!(result.channels),
        vec![
            Channel {
                id: zed_id,
                name: "zed".to_string(),
                parent_id: None,
            },
            Channel {
                id: crdb_id,
                name: "crdb".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: gpui2_id,
                name: "gpui2".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(gpui2_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(crdb_id),
            },
            Channel {
                id: livestreaming_dag_id,
                name: "livestreaming_dag".to_string(),
                parent_id: Some(livestreaming_id),
            },
            Channel {
                id: livestreaming_dag_sub_id,
                name: "livestreaming_dag_sub".to_string(),
                parent_id: Some(livestreaming_id),
            },
            Channel {
                id: livestreaming_dag_sub_id,
                name: "livestreaming_dag_sub".to_string(),
                parent_id: Some(livestreaming_dag_id),
            },
        ]
    );

    // // Attempt to make a cycle
    // assert!(db
    //     .move_channel(a_id, zed_id, Some(livestreaming_id))
    //     .await
    //     .is_err());

    // // Move channel up
    // db.move_channel(a_id, livestreaming_id, Some(zed_id))
    //     .await
    //     .unwrap();

    // let result = db.get_channels_for_user(a_id).await.unwrap();
    // pretty_assertions::assert_eq!(
    //     result.channels,
    //     vec![
    //         Channel {
    //             id: zed_id,
    //             name: "zed".to_string(),
    //             parent_id: None,
    //         },
    //         Channel {
    //             id: crdb_id,
    //             name: "crdb".to_string(),
    //             parent_id: Some(zed_id),
    //         },
    //         Channel {
    //             id: crdb_id,
    //             name: "crdb".to_string(),
    //             parent_id: Some(livestreaming_id),
    //         },
    //         Channel {
    //             id: livestreaming_id,
    //             name: "livestreaming".to_string(),
    //             parent_id: Some(zed_id),
    //         },
    //     ]
    // );
}
