use crate::test_both_dbs;

use collab::db::RoomId;
use collab::db::*;
use pretty_assertions::assert_eq;
use rpc::{ConnectionId, proto};
use std::sync::Arc;

test_both_dbs!(
    test_add_contacts,
    test_add_contacts_postgres,
    test_add_contacts_sqlite
);

async fn test_add_contacts(db: &Arc<Database>) {
    let mut user_ids = Vec::new();
    for _ in 0..3 {
        user_ids.push(db.create_user(false).await.unwrap().user_id);
    }

    let user_1 = user_ids[0];
    let user_2 = user_ids[1];
    let user_3 = user_ids[2];

    // User starts with no contacts
    assert_eq!(db.get_contacts(user_1).await.unwrap(), &[]);

    // User requests a contact. Both users see the pending request.
    db.send_contact_request(user_1, user_2).await.unwrap();
    assert!(!db.has_contact(user_1, user_2).await.unwrap());
    assert!(!db.has_contact(user_2, user_1).await.unwrap());
    assert_eq!(
        db.get_contacts(user_1).await.unwrap(),
        &[Contact::Outgoing { user_id: user_2 }],
    );
    assert_eq!(
        db.get_contacts(user_2).await.unwrap(),
        &[Contact::Incoming { user_id: user_1 }]
    );

    // User 2 dismisses the contact request notification without accepting or rejecting.
    // We shouldn't notify them again.
    db.dismiss_contact_notification(user_1, user_2)
        .await
        .unwrap_err();
    db.dismiss_contact_notification(user_2, user_1)
        .await
        .unwrap();
    assert_eq!(
        db.get_contacts(user_2).await.unwrap(),
        &[Contact::Incoming { user_id: user_1 }]
    );

    // User can't accept their own contact request
    db.respond_to_contact_request(user_1, user_2, true)
        .await
        .unwrap_err();

    // User accepts a contact request. Both users see the contact.
    db.respond_to_contact_request(user_2, user_1, true)
        .await
        .unwrap();
    assert_eq!(
        db.get_contacts(user_1).await.unwrap(),
        &[Contact::Accepted {
            user_id: user_2,
            busy: false,
        }],
    );
    assert!(db.has_contact(user_1, user_2).await.unwrap());
    assert!(db.has_contact(user_2, user_1).await.unwrap());
    assert_eq!(
        db.get_contacts(user_2).await.unwrap(),
        &[Contact::Accepted {
            user_id: user_1,
            busy: false,
        }]
    );

    // Users cannot re-request existing contacts.
    db.send_contact_request(user_1, user_2).await.unwrap_err();
    db.send_contact_request(user_2, user_1).await.unwrap_err();

    // Users can't dismiss notifications of them accepting other users' requests.
    db.dismiss_contact_notification(user_2, user_1)
        .await
        .unwrap_err();
    assert_eq!(
        db.get_contacts(user_1).await.unwrap(),
        &[Contact::Accepted {
            user_id: user_2,
            busy: false,
        }]
    );

    // Users can dismiss notifications of other users accepting their requests.
    db.dismiss_contact_notification(user_1, user_2)
        .await
        .unwrap();
    assert_eq!(
        db.get_contacts(user_1).await.unwrap(),
        &[Contact::Accepted {
            user_id: user_2,
            busy: false,
        }]
    );

    // Users send each other concurrent contact requests and
    // see that they are immediately accepted.
    db.send_contact_request(user_1, user_3).await.unwrap();
    db.send_contact_request(user_3, user_1).await.unwrap();
    assert_eq!(
        db.get_contacts(user_1).await.unwrap(),
        &[
            Contact::Accepted {
                user_id: user_2,
                busy: false,
            },
            Contact::Accepted {
                user_id: user_3,
                busy: false,
            }
        ]
    );
    assert_eq!(
        db.get_contacts(user_3).await.unwrap(),
        &[Contact::Accepted {
            user_id: user_1,
            busy: false,
        }],
    );

    // User declines a contact request. Both users see that it is gone.
    db.send_contact_request(user_2, user_3).await.unwrap();
    db.respond_to_contact_request(user_3, user_2, false)
        .await
        .unwrap();
    assert!(!db.has_contact(user_2, user_3).await.unwrap());
    assert!(!db.has_contact(user_3, user_2).await.unwrap());
    assert_eq!(
        db.get_contacts(user_2).await.unwrap(),
        &[Contact::Accepted {
            user_id: user_1,
            busy: false,
        }]
    );
    assert_eq!(
        db.get_contacts(user_3).await.unwrap(),
        &[Contact::Accepted {
            user_id: user_1,
            busy: false,
        }],
    );
}

test_both_dbs!(
    test_project_count,
    test_project_count_postgres,
    test_project_count_sqlite
);

async fn test_project_count(db: &Arc<Database>) {
    let owner_id = db.create_server("test").await.unwrap().0 as u32;

    let user1 = db.create_user(true).await.unwrap();
    let user2 = db.create_user(false).await.unwrap();

    let room_id = RoomId::from_proto(
        db.create_room(user1.user_id, ConnectionId { owner_id, id: 0 }, "")
            .await
            .unwrap()
            .id,
    );
    db.call(
        room_id,
        user1.user_id,
        ConnectionId { owner_id, id: 0 },
        user2.user_id,
        None,
    )
    .await
    .unwrap();
    db.join_room(room_id, user2.user_id, ConnectionId { owner_id, id: 1 })
        .await
        .unwrap();
    assert_eq!(db.project_count_excluding_admins().await.unwrap(), 0);

    db.share_project(
        room_id,
        ConnectionId { owner_id, id: 1 },
        &[],
        false,
        false,
        &[],
    )
    .await
    .unwrap();
    assert_eq!(db.project_count_excluding_admins().await.unwrap(), 1);

    db.share_project(
        room_id,
        ConnectionId { owner_id, id: 1 },
        &[],
        false,
        false,
        &[],
    )
    .await
    .unwrap();
    assert_eq!(db.project_count_excluding_admins().await.unwrap(), 2);

    // Projects shared by admins aren't counted.
    db.share_project(
        room_id,
        ConnectionId { owner_id, id: 0 },
        &[],
        false,
        false,
        &[],
    )
    .await
    .unwrap();
    assert_eq!(db.project_count_excluding_admins().await.unwrap(), 2);

    db.leave_room(ConnectionId { owner_id, id: 1 })
        .await
        .unwrap();
    assert_eq!(db.project_count_excluding_admins().await.unwrap(), 0);
}

test_both_dbs!(
    test_worktree_always_included_persistence,
    test_worktree_always_included_persistence_postgres,
    test_worktree_always_included_persistence_sqlite
);

async fn test_worktree_always_included_persistence(db: &Arc<Database>) {
    let owner_id = db.create_server("test").await.unwrap().0 as u32;
    let host = db.create_user(false).await.unwrap().user_id;
    let guest = db.create_user(false).await.unwrap().user_id;
    let host_connection = ConnectionId { owner_id, id: 0 };
    let guest_connection = ConnectionId { owner_id, id: 1 };
    let room_id = RoomId::from_proto(db.create_room(host, host_connection, "").await.unwrap().id);
    db.call(room_id, host, host_connection, guest, None)
        .await
        .unwrap();
    db.join_room(room_id, guest, guest_connection)
        .await
        .unwrap();

    let worktree = proto::WorktreeMetadata {
        id: 1,
        root_name: String::from("project"),
        abs_path: String::from("/project"),
        visible: true,
        ..proto::WorktreeMetadata::default()
    };
    let project_id = db
        .share_project(room_id, host_connection, &[worktree], false, false, &[])
        .await
        .unwrap()
        .into_inner()
        .0;
    let mut update = proto::UpdateWorktree {
        project_id: project_id.to_proto(),
        worktree_id: 1,
        root_name: String::from("project"),
        abs_path: String::from("/project"),
        is_last_update: true,
        updated_entries: [
            (false, false, false),
            (false, false, true),
            (false, true, false),
            (false, true, true),
            (true, false, false),
            (true, false, true),
            (true, true, false),
            (true, true, true),
        ]
        .into_iter()
        .enumerate()
        .map(
            |(index, (is_dir, is_ignored, is_always_included))| proto::Entry {
                id: index as u64 + 1,
                path: format!("entry-{index}"),
                mtime: Some(proto::Timestamp::default()),
                is_dir,
                is_ignored,
                is_always_included,
                ..proto::Entry::default()
            },
        )
        .collect(),
        ..proto::UpdateWorktree::default()
    };

    for scan_id in [1, 2] {
        update.scan_id = scan_id;
        if scan_id == 2 {
            for entry in &mut update.updated_entries {
                entry.is_always_included = !entry.is_always_included;
            }
        }
        for chunk in proto::split_worktree_update(update.clone()) {
            assert!(db.update_worktree(&chunk, guest_connection).await.is_err());
            db.update_worktree(&chunk, host_connection).await.unwrap();
        }

        let (mut joined_project, _) = db
            .join_project(project_id, guest_connection, guest, None, None)
            .await
            .unwrap()
            .into_inner();
        assert_eq!(joined_project.worktrees.len(), 1);
        let joined_worktree = joined_project.worktrees.get_mut(&1).unwrap();
        joined_worktree.entries.sort_by_key(|entry| entry.id);
        assert_eq!(joined_worktree.entries, update.updated_entries);

        for previous_scan_id in [None, Some(scan_id - 1), Some(scan_id)] {
            let mut rejoined_room = db
                .rejoin_room(
                    proto::RejoinRoom {
                        id: room_id.to_proto(),
                        rejoined_projects: vec![proto::RejoinProject {
                            id: project_id.to_proto(),
                            worktrees: previous_scan_id
                                .map(|scan_id| proto::RejoinWorktree { id: 1, scan_id })
                                .into_iter()
                                .collect(),
                            ..proto::RejoinProject::default()
                        }],
                        ..proto::RejoinRoom::default()
                    },
                    guest,
                    guest_connection,
                )
                .await
                .unwrap()
                .into_inner();
            assert_eq!(rejoined_room.rejoined_projects.len(), 1);
            let rejoined_project = rejoined_room.rejoined_projects.first_mut().unwrap();
            assert_eq!(rejoined_project.worktrees.len(), 1);
            let rejoined_worktree = rejoined_project.worktrees.first_mut().unwrap();
            assert_eq!(rejoined_worktree.scan_id, scan_id);
            assert_eq!(rejoined_worktree.removed_entries, Vec::<u64>::new());
            rejoined_worktree
                .updated_entries
                .sort_by_key(|entry| entry.id);
            let expected_entries = if previous_scan_id == Some(scan_id) {
                &[][..]
            } else {
                update.updated_entries.as_slice()
            };
            assert_eq!(rejoined_worktree.updated_entries, expected_entries);
        }

        db.leave_project(project_id, guest_connection)
            .await
            .unwrap();
    }
}
