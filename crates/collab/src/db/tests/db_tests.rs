use super::*;
use crate::test_both_dbs;
use gpui::TestAppContext;
use pretty_assertions::{assert_eq, assert_ne};
use std::sync::Arc;
use tests::TestDb;

test_both_dbs!(
    test_get_users,
    test_get_users_by_ids_postgres,
    test_get_users_by_ids_sqlite
);

async fn test_get_users(db: &Arc<Database>) {
    let mut user_ids = Vec::new();
    let mut user_metric_ids = Vec::new();
    for i in 1..=4 {
        let user = db
            .create_user(
                &format!("user{i}@example.com"),
                false,
                NewUserParams {
                    github_login: format!("user{i}"),
                    github_user_id: i,
                },
            )
            .await
            .unwrap();
        user_ids.push(user.user_id);
        user_metric_ids.push(user.metrics_id);
    }

    assert_eq!(
        db.get_users_by_ids(user_ids.clone()).await.unwrap(),
        vec![
            User {
                id: user_ids[0],
                github_login: "user1".to_string(),
                github_user_id: Some(1),
                email_address: Some("user1@example.com".to_string()),
                admin: false,
                metrics_id: user_metric_ids[0].parse().unwrap(),
                ..Default::default()
            },
            User {
                id: user_ids[1],
                github_login: "user2".to_string(),
                github_user_id: Some(2),
                email_address: Some("user2@example.com".to_string()),
                admin: false,
                metrics_id: user_metric_ids[1].parse().unwrap(),
                ..Default::default()
            },
            User {
                id: user_ids[2],
                github_login: "user3".to_string(),
                github_user_id: Some(3),
                email_address: Some("user3@example.com".to_string()),
                admin: false,
                metrics_id: user_metric_ids[2].parse().unwrap(),
                ..Default::default()
            },
            User {
                id: user_ids[3],
                github_login: "user4".to_string(),
                github_user_id: Some(4),
                email_address: Some("user4@example.com".to_string()),
                admin: false,
                metrics_id: user_metric_ids[3].parse().unwrap(),
                ..Default::default()
            }
        ]
    );
}

test_both_dbs!(
    test_get_or_create_user_by_github_account,
    test_get_or_create_user_by_github_account_postgres,
    test_get_or_create_user_by_github_account_sqlite
);

async fn test_get_or_create_user_by_github_account(db: &Arc<Database>) {
    let user_id1 = db
        .create_user(
            "user1@example.com",
            false,
            NewUserParams {
                github_login: "login1".into(),
                github_user_id: 101,
            },
        )
        .await
        .unwrap()
        .user_id;
    let user_id2 = db
        .create_user(
            "user2@example.com",
            false,
            NewUserParams {
                github_login: "login2".into(),
                github_user_id: 102,
            },
        )
        .await
        .unwrap()
        .user_id;

    let user = db
        .get_or_create_user_by_github_account("login1", None, None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user.id, user_id1);
    assert_eq!(&user.github_login, "login1");
    assert_eq!(user.github_user_id, Some(101));

    assert!(db
        .get_or_create_user_by_github_account("non-existent-login", None, None)
        .await
        .unwrap()
        .is_none());

    let user = db
        .get_or_create_user_by_github_account("the-new-login2", Some(102), None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user.id, user_id2);
    assert_eq!(&user.github_login, "the-new-login2");
    assert_eq!(user.github_user_id, Some(102));

    let user = db
        .get_or_create_user_by_github_account("login3", Some(103), Some("user3@example.com"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(&user.github_login, "login3");
    assert_eq!(user.github_user_id, Some(103));
    assert_eq!(user.email_address, Some("user3@example.com".into()));
}

test_both_dbs!(
    test_create_access_tokens,
    test_create_access_tokens_postgres,
    test_create_access_tokens_sqlite
);

async fn test_create_access_tokens(db: &Arc<Database>) {
    let user = db
        .create_user(
            "u1@example.com",
            false,
            NewUserParams {
                github_login: "u1".into(),
                github_user_id: 1,
            },
        )
        .await
        .unwrap()
        .user_id;

    let token_1 = db.create_access_token(user, "h1", 2).await.unwrap();
    let token_2 = db.create_access_token(user, "h2", 2).await.unwrap();
    assert_eq!(
        db.get_access_token(token_1).await.unwrap(),
        access_token::Model {
            id: token_1,
            user_id: user,
            hash: "h1".into(),
        }
    );
    assert_eq!(
        db.get_access_token(token_2).await.unwrap(),
        access_token::Model {
            id: token_2,
            user_id: user,
            hash: "h2".into()
        }
    );

    let token_3 = db.create_access_token(user, "h3", 2).await.unwrap();
    assert_eq!(
        db.get_access_token(token_3).await.unwrap(),
        access_token::Model {
            id: token_3,
            user_id: user,
            hash: "h3".into()
        }
    );
    assert_eq!(
        db.get_access_token(token_2).await.unwrap(),
        access_token::Model {
            id: token_2,
            user_id: user,
            hash: "h2".into()
        }
    );
    assert!(db.get_access_token(token_1).await.is_err());

    let token_4 = db.create_access_token(user, "h4", 2).await.unwrap();
    assert_eq!(
        db.get_access_token(token_4).await.unwrap(),
        access_token::Model {
            id: token_4,
            user_id: user,
            hash: "h4".into()
        }
    );
    assert_eq!(
        db.get_access_token(token_3).await.unwrap(),
        access_token::Model {
            id: token_3,
            user_id: user,
            hash: "h3".into()
        }
    );
    assert!(db.get_access_token(token_2).await.is_err());
    assert!(db.get_access_token(token_1).await.is_err());
}

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
    test_metrics_id,
    test_metrics_id_postgres,
    test_metrics_id_sqlite
);

async fn test_metrics_id(db: &Arc<Database>) {
    let NewUserResult {
        user_id: user1,
        metrics_id: metrics_id1,
        ..
    } = db
        .create_user(
            "person1@example.com",
            false,
            NewUserParams {
                github_login: "person1".into(),
                github_user_id: 101,
            },
        )
        .await
        .unwrap();
    let NewUserResult {
        user_id: user2,
        metrics_id: metrics_id2,
        ..
    } = db
        .create_user(
            "person2@example.com",
            false,
            NewUserParams {
                github_login: "person2".into(),
                github_user_id: 102,
            },
        )
        .await
        .unwrap();

    assert_eq!(db.get_user_metrics_id(user1).await.unwrap(), metrics_id1);
    assert_eq!(db.get_user_metrics_id(user2).await.unwrap(), metrics_id2);
    assert_eq!(metrics_id1.len(), 36);
    assert_eq!(metrics_id2.len(), 36);
    assert_ne!(metrics_id1, metrics_id2);
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
            &format!("admin@example.com"),
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
            &format!("user@example.com"),
            false,
            NewUserParams {
                github_login: "user".into(),
                github_user_id: 1,
            },
        )
        .await
        .unwrap();

    let room_id = RoomId::from_proto(
        db.create_room(user1.user_id, ConnectionId { owner_id, id: 0 }, "", "dev")
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
    db.join_room(
        room_id,
        user2.user_id,
        ConnectionId { owner_id, id: 1 },
        "dev",
    )
    .await
    .unwrap();
    assert_eq!(db.project_count_excluding_admins().await.unwrap(), 0);

    db.share_project(room_id, ConnectionId { owner_id, id: 1 }, &[])
        .await
        .unwrap();
    assert_eq!(db.project_count_excluding_admins().await.unwrap(), 1);

    db.share_project(room_id, ConnectionId { owner_id, id: 1 }, &[])
        .await
        .unwrap();
    assert_eq!(db.project_count_excluding_admins().await.unwrap(), 2);

    // Projects shared by admins aren't counted.
    db.share_project(room_id, ConnectionId { owner_id, id: 0 }, &[])
        .await
        .unwrap();
    assert_eq!(db.project_count_excluding_admins().await.unwrap(), 2);

    db.leave_room(ConnectionId { owner_id, id: 1 })
        .await
        .unwrap();
    assert_eq!(db.project_count_excluding_admins().await.unwrap(), 0);
}

#[test]
fn test_fuzzy_like_string() {
    assert_eq!(Database::fuzzy_like_string("abcd"), "%a%b%c%d%");
    assert_eq!(Database::fuzzy_like_string("x y"), "%x%y%");
    assert_eq!(Database::fuzzy_like_string(" z  "), "%z%");
}

#[gpui::test]
async fn test_fuzzy_search_users(cx: &mut TestAppContext) {
    let test_db = TestDb::postgres(cx.executor());
    let db = test_db.db();
    for (i, github_login) in [
        "California",
        "colorado",
        "oregon",
        "washington",
        "florida",
        "delaware",
        "rhode-island",
    ]
    .into_iter()
    .enumerate()
    {
        db.create_user(
            &format!("{github_login}@example.com"),
            false,
            NewUserParams {
                github_login: github_login.into(),
                github_user_id: i as i32,
            },
        )
        .await
        .unwrap();
    }

    assert_eq!(
        fuzzy_search_user_names(db, "clr").await,
        &["colorado", "California"]
    );
    assert_eq!(
        fuzzy_search_user_names(db, "ro").await,
        &["rhode-island", "colorado", "oregon"],
    );

    async fn fuzzy_search_user_names(db: &Database, query: &str) -> Vec<String> {
        db.fuzzy_search_users(query, 10)
            .await
            .unwrap()
            .into_iter()
            .map(|user| user.github_login)
            .collect::<Vec<_>>()
    }
}

test_both_dbs!(
    test_non_matching_release_channels,
    test_non_matching_release_channels_postgres,
    test_non_matching_release_channels_sqlite
);

async fn test_non_matching_release_channels(db: &Arc<Database>) {
    let owner_id = db.create_server("test").await.unwrap().0 as u32;

    let user1 = db
        .create_user(
            &format!("admin@example.com"),
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
            &format!("user@example.com"),
            false,
            NewUserParams {
                github_login: "user".into(),
                github_user_id: 1,
            },
        )
        .await
        .unwrap();

    let room = db
        .create_room(
            user1.user_id,
            ConnectionId { owner_id, id: 0 },
            "",
            "stable",
        )
        .await
        .unwrap();

    db.call(
        RoomId::from_proto(room.id),
        user1.user_id,
        ConnectionId { owner_id, id: 0 },
        user2.user_id,
        None,
    )
    .await
    .unwrap();

    // User attempts to join from preview
    let result = db
        .join_room(
            RoomId::from_proto(room.id),
            user2.user_id,
            ConnectionId { owner_id, id: 1 },
            "preview",
        )
        .await;

    assert!(result.is_err());

    // User switches to stable
    let result = db
        .join_room(
            RoomId::from_proto(room.id),
            user2.user_id,
            ConnectionId { owner_id, id: 1 },
            "stable",
        )
        .await;

    assert!(result.is_ok())
}
