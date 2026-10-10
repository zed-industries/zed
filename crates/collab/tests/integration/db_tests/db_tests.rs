use crate::test_both_dbs;

use collab::db::RoomId;
use collab::db::*;
use pretty_assertions::assert_eq;
use rpc::ConnectionId;
use rpc::proto;
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
    test_language_server_memory_usage_requires_host,
    test_language_server_memory_usage_requires_host_postgres,
    test_language_server_memory_usage_requires_host_sqlite
);

async fn test_language_server_memory_usage_requires_host(db: &Arc<Database>) {
    let owner_id = db.create_server("test").await.unwrap().0 as u32;
    let host = db.create_user(true).await.unwrap();
    let guest = db.create_user(false).await.unwrap();

    let host_connection = ConnectionId { owner_id, id: 0 };
    let guest_connection = ConnectionId { owner_id, id: 1 };

    let room_id = RoomId::from_proto(
        db.create_room(host.user_id, host_connection, "")
            .await
            .unwrap()
            .id,
    );

    db.call(room_id, host.user_id, host_connection, guest.user_id, None)
        .await
        .unwrap();

    db.join_room(room_id, guest.user_id, guest_connection)
        .await
        .unwrap();

    let (project_id, _) = db
        .share_project(room_id, host_connection, &[], false, false, &[])
        .await
        .unwrap()
        .into_inner();

    let server_id = 1;

    db.start_language_server(
        &proto::StartLanguageServer {
            project_id: project_id.to_proto(),
            server: Some(proto::LanguageServer {
                id: server_id,
                name: "test".to_string(),
                worktree_id: None,
                language_name: Some("Rust".to_string()),
                server_version: None,
            }),
            capabilities: String::new(),
        },
        host_connection,
    )
    .await
    .unwrap();

    db.cache_language_server_memory_usage_for_connection(
        project_id,
        host_connection,
        server_id,
        100,
    )
    .await
    .unwrap();

    let result = db
        .cache_language_server_memory_usage_for_connection(
            project_id,
            guest_connection,
            server_id,
            999,
        )
        .await;

    assert!(result.is_err());
    assert_eq!(
        db.language_server_memory_usage(project_id, server_id),
        Some(100)
    );
}
