use crate::test_both_dbs;

use super::*;
use collab::db::RoomId;
use collab::db::*;
use pretty_assertions::assert_eq;
use rpc::ConnectionId;
use std::sync::Arc;

test_both_dbs!(
    test_add_contacts,
    test_add_contacts_postgres,
    test_add_contacts_sqlite
);

async fn test_add_contacts(db: &Arc<Database>) {
    let mut user_ids = Vec::new();
    for i in 0..3 {
        user_ids.push(
            db.create_user(
                &format!("user{i}@example.com"),
                None,
                false,
                NewUserParams {
                    github_login: format!("user{i}"),
                    github_user_id: i,
                },
            )
            .await
            .unwrap()
            .user_id,
        );
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

    let user1 = db
        .create_user(
            "admin@example.com",
            None,
            true,
            NewUserParams {
                github_login: "admin".into(),
                github_user_id: 0,
            },
        )
        .await
        .unwrap();
    let user2 = db
        .create_user(
            "user@example.com",
            None,
            false,
            NewUserParams {
                github_login: "user".into(),
                github_user_id: 1,
            },
        )
        .await
        .unwrap();

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
    test_upsert_shared_thread,
    test_upsert_shared_thread_postgres,
    test_upsert_shared_thread_sqlite
);

async fn test_upsert_shared_thread(db: &Arc<Database>) {
    use collab::db::SharedThreadId;
    use uuid::Uuid;

    let user_id = new_test_user(db, "user1@example.com").await;

    let thread_id = SharedThreadId(Uuid::new_v4());
    let title = "My Test Thread";
    let data = b"test thread data".to_vec();

    db.upsert_shared_thread(thread_id, user_id, title, data.clone())
        .await
        .unwrap();

    let result = db.get_shared_thread(thread_id).await.unwrap();
    assert!(result.is_some(), "Should find the shared thread");

    let (thread, username) = result.unwrap();
    assert_eq!(thread.title, title);
    assert_eq!(thread.data, data);
    assert_eq!(thread.user_id, user_id);
    assert_eq!(username, "user1");
}

test_both_dbs!(
    test_upsert_shared_thread_updates_existing,
    test_upsert_shared_thread_updates_existing_postgres,
    test_upsert_shared_thread_updates_existing_sqlite
);

async fn test_upsert_shared_thread_updates_existing(db: &Arc<Database>) {
    use collab::db::SharedThreadId;
    use uuid::Uuid;

    let user_id = new_test_user(db, "user1@example.com").await;

    let thread_id = SharedThreadId(Uuid::new_v4());

    // Create initial thread.
    db.upsert_shared_thread(
        thread_id,
        user_id,
        "Original Title",
        b"original data".to_vec(),
    )
    .await
    .unwrap();

    // Update the same thread.
    db.upsert_shared_thread(
        thread_id,
        user_id,
        "Updated Title",
        b"updated data".to_vec(),
    )
    .await
    .unwrap();

    let result = db.get_shared_thread(thread_id).await.unwrap();
    let (thread, _) = result.unwrap();

    assert_eq!(thread.title, "Updated Title");
    assert_eq!(thread.data, b"updated data".to_vec());
}

test_both_dbs!(
    test_cannot_update_another_users_shared_thread,
    test_cannot_update_another_users_shared_thread_postgres,
    test_cannot_update_another_users_shared_thread_sqlite
);

async fn test_cannot_update_another_users_shared_thread(db: &Arc<Database>) {
    use collab::db::SharedThreadId;
    use uuid::Uuid;

    let user1_id = new_test_user(db, "user1@example.com").await;
    let user2_id = new_test_user(db, "user2@example.com").await;

    let thread_id = SharedThreadId(Uuid::new_v4());

    db.upsert_shared_thread(thread_id, user1_id, "User 1 Thread", b"user1 data".to_vec())
        .await
        .unwrap();

    let result = db
        .upsert_shared_thread(thread_id, user2_id, "User 2 Title", b"user2 data".to_vec())
        .await;

    assert!(
        result.is_err(),
        "Should not allow updating another user's thread"
    );
}

test_both_dbs!(
    test_get_nonexistent_shared_thread,
    test_get_nonexistent_shared_thread_postgres,
    test_get_nonexistent_shared_thread_sqlite
);

async fn test_get_nonexistent_shared_thread(db: &Arc<Database>) {
    use collab::db::SharedThreadId;
    use uuid::Uuid;

    let result = db
        .get_shared_thread(SharedThreadId(Uuid::new_v4()))
        .await
        .unwrap();

    assert!(result.is_none(), "Should not find non-existent thread");
}
