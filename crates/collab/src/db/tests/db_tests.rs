use super::*;
use crate::test_both_dbs;
use chrono::Utc;
use pretty_assertions::assert_eq;
use std::sync::Arc;

test_both_dbs!(
    test_get_users,
    test_get_users_by_ids_postgres,
    test_get_users_by_ids_sqlite
);

async fn test_get_users(db: &Arc<Database>) {
    let mut user_ids = Vec::new();
    for i in 1..=4 {
        let user = db
            .create_user(
                &format!("user{i}@example.com"),
                None,
                false,
                NewUserParams {
                    github_login: format!("user{i}"),
                    github_user_id: i,
                },
            )
            .await
            .unwrap();
        user_ids.push(user.user_id);
    }

    assert_eq!(
        db.get_users_by_ids(user_ids.clone())
            .await
            .unwrap()
            .into_iter()
            .map(|user| (
                user.id,
                user.github_login,
                user.github_user_id,
                user.email_address
            ))
            .collect::<Vec<_>>(),
        vec![
            (
                user_ids[0],
                "user1".to_string(),
                1,
                Some("user1@example.com".to_string()),
            ),
            (
                user_ids[1],
                "user2".to_string(),
                2,
                Some("user2@example.com".to_string()),
            ),
            (
                user_ids[2],
                "user3".to_string(),
                3,
                Some("user3@example.com".to_string()),
            ),
            (
                user_ids[3],
                "user4".to_string(),
                4,
                Some("user4@example.com".to_string()),
            )
        ]
    );
}

test_both_dbs!(
    test_update_or_create_user_by_github_account,
    test_update_or_create_user_by_github_account_postgres,
    test_update_or_create_user_by_github_account_sqlite
);

async fn test_update_or_create_user_by_github_account(db: &Arc<Database>) {
    db.create_user(
        "user1@example.com",
        None,
        false,
        NewUserParams {
            github_login: "login1".into(),
            github_user_id: 101,
        },
    )
    .await
    .unwrap();
    let user_id2 = db
        .create_user(
            "user2@example.com",
            None,
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
        .update_or_create_user_by_github_account(
            "the-new-login2",
            102,
            None,
            None,
            Utc::now(),
            None,
        )
        .await
        .unwrap();
    assert_eq!(user.id, user_id2);
    assert_eq!(&user.github_login, "the-new-login2");
    assert_eq!(user.github_user_id, 102);

    let user = db
        .update_or_create_user_by_github_account(
            "login3",
            103,
            Some("user3@example.com"),
            None,
            Utc::now(),
            None,
        )
        .await
        .unwrap();
    assert_eq!(&user.github_login, "login3");
    assert_eq!(user.github_user_id, 103);
    assert_eq!(user.email_address, Some("user3@example.com".into()));
}

test_both_dbs!(
    test_create_access_tokens,
    test_create_access_tokens_postgres,
    test_create_access_tokens_sqlite
);

async fn test_create_access_tokens(db: &Arc<Database>) {
    let user_1 = db
        .create_user(
            "u1@example.com",
            None,
            false,
            NewUserParams {
                github_login: "u1".into(),
                github_user_id: 1,
            },
        )
        .await
        .unwrap()
        .user_id;
    let user_2 = db
        .create_user(
            "u2@example.com",
            None,
            false,
            NewUserParams {
                github_login: "u2".into(),
                github_user_id: 2,
            },
        )
        .await
        .unwrap()
        .user_id;

    let token_1 = db.create_access_token(user_1, None, "h1", 2).await.unwrap();
    let token_2 = db.create_access_token(user_1, None, "h2", 2).await.unwrap();
    assert_eq!(
        db.get_access_token(token_1).await.unwrap(),
        access_token::Model {
            id: token_1,
            user_id: user_1,
            impersonated_user_id: None,
            hash: "h1".into(),
        }
    );
    assert_eq!(
        db.get_access_token(token_2).await.unwrap(),
        access_token::Model {
            id: token_2,
            user_id: user_1,
            impersonated_user_id: None,
            hash: "h2".into()
        }
    );

    let token_3 = db.create_access_token(user_1, None, "h3", 2).await.unwrap();
    assert_eq!(
        db.get_access_token(token_3).await.unwrap(),
        access_token::Model {
            id: token_3,
            user_id: user_1,
            impersonated_user_id: None,
            hash: "h3".into()
        }
    );
    assert_eq!(
        db.get_access_token(token_2).await.unwrap(),
        access_token::Model {
            id: token_2,
            user_id: user_1,
            impersonated_user_id: None,
            hash: "h2".into()
        }
    );
    assert!(db.get_access_token(token_1).await.is_err());

    let token_4 = db.create_access_token(user_1, None, "h4", 2).await.unwrap();
    assert_eq!(
        db.get_access_token(token_4).await.unwrap(),
        access_token::Model {
            id: token_4,
            user_id: user_1,
            impersonated_user_id: None,
            hash: "h4".into()
        }
    );
    assert_eq!(
        db.get_access_token(token_3).await.unwrap(),
        access_token::Model {
            id: token_3,
            user_id: user_1,
            impersonated_user_id: None,
            hash: "h3".into()
        }
    );
    assert!(db.get_access_token(token_2).await.is_err());
    assert!(db.get_access_token(token_1).await.is_err());

    // An access token for user 2 impersonating user 1 does not
    // count against user 1's access token limit (of 2).
    let token_5 = db
        .create_access_token(user_2, Some(user_1), "h5", 2)
        .await
        .unwrap();
    assert_eq!(
        db.get_access_token(token_5).await.unwrap(),
        access_token::Model {
            id: token_5,
            user_id: user_2,
            impersonated_user_id: Some(user_1),
            hash: "h5".into()
        }
    );
    assert_eq!(
        db.get_access_token(token_3).await.unwrap(),
        access_token::Model {
            id: token_3,
            user_id: user_1,
            impersonated_user_id: None,
            hash: "h3".into()
        }
    );

    // Only a limited number (2) of access tokens are stored for user 2
    // impersonating other users.
    let token_6 = db
        .create_access_token(user_2, Some(user_1), "h6", 2)
        .await
        .unwrap();
    let token_7 = db
        .create_access_token(user_2, Some(user_1), "h7", 2)
        .await
        .unwrap();
    assert_eq!(
        db.get_access_token(token_6).await.unwrap(),
        access_token::Model {
            id: token_6,
            user_id: user_2,
            impersonated_user_id: Some(user_1),
            hash: "h6".into()
        }
    );
    assert_eq!(
        db.get_access_token(token_7).await.unwrap(),
        access_token::Model {
            id: token_7,
            user_id: user_2,
            impersonated_user_id: Some(user_1),
            hash: "h7".into()
        }
    );
    assert!(db.get_access_token(token_5).await.is_err());
    assert_eq!(
        db.get_access_token(token_3).await.unwrap(),
        access_token::Model {
            id: token_3,
            user_id: user_1,
            impersonated_user_id: None,
            hash: "h3".into()
        }
    );
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

    db.share_project(room_id, ConnectionId { owner_id, id: 1 }, &[], false, false)
        .await
        .unwrap();
    assert_eq!(db.project_count_excluding_admins().await.unwrap(), 1);

    db.share_project(room_id, ConnectionId { owner_id, id: 1 }, &[], false, false)
        .await
        .unwrap();
    assert_eq!(db.project_count_excluding_admins().await.unwrap(), 2);

    // Projects shared by admins aren't counted.
    db.share_project(room_id, ConnectionId { owner_id, id: 0 }, &[], false, false)
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

#[cfg(target_os = "macos")]
#[gpui::test]
async fn test_fuzzy_search_users(cx: &mut gpui::TestAppContext) {
    let test_db = tests::TestDb::postgres(cx.executor());
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
            None,
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
