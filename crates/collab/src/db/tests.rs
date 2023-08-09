use super::*;
use gpui::executor::{Background, Deterministic};
use std::sync::Arc;

#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

macro_rules! test_both_dbs {
    ($postgres_test_name:ident, $sqlite_test_name:ident, $db:ident, $body:block) => {
        #[gpui::test]
        async fn $postgres_test_name() {
            let test_db = TestDb::postgres(Deterministic::new(0).build_background());
            let $db = test_db.db();
            $body
        }

        #[gpui::test]
        async fn $sqlite_test_name() {
            let test_db = TestDb::sqlite(Deterministic::new(0).build_background());
            let $db = test_db.db();
            $body
        }
    };
}

test_both_dbs!(
    test_get_users_by_ids_postgres,
    test_get_users_by_ids_sqlite,
    db,
    {
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
                        invite_count: 0,
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
);

test_both_dbs!(
    test_get_or_create_user_by_github_account_postgres,
    test_get_or_create_user_by_github_account_sqlite,
    db,
    {
        let user_id1 = db
            .create_user(
                "user1@example.com",
                false,
                NewUserParams {
                    github_login: "login1".into(),
                    github_user_id: 101,
                    invite_count: 0,
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
                    invite_count: 0,
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
);

test_both_dbs!(
    test_create_access_tokens_postgres,
    test_create_access_tokens_sqlite,
    db,
    {
        let user = db
            .create_user(
                "u1@example.com",
                false,
                NewUserParams {
                    github_login: "u1".into(),
                    github_user_id: 1,
                    invite_count: 0,
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
);

test_both_dbs!(test_add_contacts_postgres, test_add_contacts_sqlite, db, {
    let mut user_ids = Vec::new();
    for i in 0..3 {
        user_ids.push(
            db.create_user(
                &format!("user{i}@example.com"),
                false,
                NewUserParams {
                    github_login: format!("user{i}"),
                    github_user_id: i,
                    invite_count: 0,
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
        &[Contact::Incoming {
            user_id: user_1,
            should_notify: true
        }]
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
        &[Contact::Incoming {
            user_id: user_1,
            should_notify: false
        }]
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
            should_notify: true,
            busy: false,
        }],
    );
    assert!(db.has_contact(user_1, user_2).await.unwrap());
    assert!(db.has_contact(user_2, user_1).await.unwrap());
    assert_eq!(
        db.get_contacts(user_2).await.unwrap(),
        &[Contact::Accepted {
            user_id: user_1,
            should_notify: false,
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
            should_notify: true,
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
            should_notify: false,
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
                should_notify: false,
                busy: false,
            },
            Contact::Accepted {
                user_id: user_3,
                should_notify: false,
                busy: false,
            }
        ]
    );
    assert_eq!(
        db.get_contacts(user_3).await.unwrap(),
        &[Contact::Accepted {
            user_id: user_1,
            should_notify: false,
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
            should_notify: false,
            busy: false,
        }]
    );
    assert_eq!(
        db.get_contacts(user_3).await.unwrap(),
        &[Contact::Accepted {
            user_id: user_1,
            should_notify: false,
            busy: false,
        }],
    );
});

test_both_dbs!(test_metrics_id_postgres, test_metrics_id_sqlite, db, {
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
                invite_count: 5,
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
                invite_count: 5,
            },
        )
        .await
        .unwrap();

    assert_eq!(db.get_user_metrics_id(user1).await.unwrap(), metrics_id1);
    assert_eq!(db.get_user_metrics_id(user2).await.unwrap(), metrics_id2);
    assert_eq!(metrics_id1.len(), 36);
    assert_eq!(metrics_id2.len(), 36);
    assert_ne!(metrics_id1, metrics_id2);
});

test_both_dbs!(
    test_project_count_postgres,
    test_project_count_sqlite,
    db,
    {
        let owner_id = db.create_server("test").await.unwrap().0 as u32;

        let user1 = db
            .create_user(
                &format!("admin@example.com"),
                true,
                NewUserParams {
                    github_login: "admin".into(),
                    github_user_id: 0,
                    invite_count: 0,
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
                    invite_count: 0,
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
        db.join_room(
            room_id,
            user2.user_id,
            None,
            ConnectionId { owner_id, id: 1 },
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
);

#[test]
fn test_fuzzy_like_string() {
    assert_eq!(Database::fuzzy_like_string("abcd"), "%a%b%c%d%");
    assert_eq!(Database::fuzzy_like_string("x y"), "%x%y%");
    assert_eq!(Database::fuzzy_like_string(" z  "), "%z%");
}

#[gpui::test]
async fn test_fuzzy_search_users() {
    let test_db = TestDb::postgres(build_background_executor());
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
                invite_count: 0,
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

#[gpui::test]
async fn test_invite_codes() {
    let test_db = TestDb::postgres(build_background_executor());
    let db = test_db.db();

    let NewUserResult { user_id: user1, .. } = db
        .create_user(
            "user1@example.com",
            false,
            NewUserParams {
                github_login: "user1".into(),
                github_user_id: 0,
                invite_count: 0,
            },
        )
        .await
        .unwrap();

    // Initially, user 1 has no invite code
    assert_eq!(db.get_invite_code_for_user(user1).await.unwrap(), None);

    // Setting invite count to 0 when no code is assigned does not assign a new code
    db.set_invite_count_for_user(user1, 0).await.unwrap();
    assert!(db.get_invite_code_for_user(user1).await.unwrap().is_none());

    // User 1 creates an invite code that can be used twice.
    db.set_invite_count_for_user(user1, 2).await.unwrap();
    let (invite_code, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
    assert_eq!(invite_count, 2);

    // User 2 redeems the invite code and becomes a contact of user 1.
    let user2_invite = db
        .create_invite_from_code(
            &invite_code,
            "user2@example.com",
            Some("user-2-device-id"),
            true,
        )
        .await
        .unwrap();
    let NewUserResult {
        user_id: user2,
        inviting_user_id,
        signup_device_id,
        metrics_id,
    } = db
        .create_user_from_invite(
            &user2_invite,
            NewUserParams {
                github_login: "user2".into(),
                github_user_id: 2,
                invite_count: 7,
            },
        )
        .await
        .unwrap()
        .unwrap();
    let (_, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
    assert_eq!(invite_count, 1);
    assert_eq!(inviting_user_id, Some(user1));
    assert_eq!(signup_device_id.unwrap(), "user-2-device-id");
    assert_eq!(db.get_user_metrics_id(user2).await.unwrap(), metrics_id);
    assert_eq!(
        db.get_contacts(user1).await.unwrap(),
        [Contact::Accepted {
            user_id: user2,
            should_notify: true,
            busy: false,
        }]
    );
    assert_eq!(
        db.get_contacts(user2).await.unwrap(),
        [Contact::Accepted {
            user_id: user1,
            should_notify: false,
            busy: false,
        }]
    );
    assert!(db.has_contact(user1, user2).await.unwrap());
    assert!(db.has_contact(user2, user1).await.unwrap());
    assert_eq!(
        db.get_invite_code_for_user(user2).await.unwrap().unwrap().1,
        7
    );

    // User 3 redeems the invite code and becomes a contact of user 1.
    let user3_invite = db
        .create_invite_from_code(&invite_code, "user3@example.com", None, true)
        .await
        .unwrap();
    let NewUserResult {
        user_id: user3,
        inviting_user_id,
        signup_device_id,
        ..
    } = db
        .create_user_from_invite(
            &user3_invite,
            NewUserParams {
                github_login: "user-3".into(),
                github_user_id: 3,
                invite_count: 3,
            },
        )
        .await
        .unwrap()
        .unwrap();
    let (_, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
    assert_eq!(invite_count, 0);
    assert_eq!(inviting_user_id, Some(user1));
    assert!(signup_device_id.is_none());
    assert_eq!(
        db.get_contacts(user1).await.unwrap(),
        [
            Contact::Accepted {
                user_id: user2,
                should_notify: true,
                busy: false,
            },
            Contact::Accepted {
                user_id: user3,
                should_notify: true,
                busy: false,
            }
        ]
    );
    assert_eq!(
        db.get_contacts(user3).await.unwrap(),
        [Contact::Accepted {
            user_id: user1,
            should_notify: false,
            busy: false,
        }]
    );
    assert!(db.has_contact(user1, user3).await.unwrap());
    assert!(db.has_contact(user3, user1).await.unwrap());
    assert_eq!(
        db.get_invite_code_for_user(user3).await.unwrap().unwrap().1,
        3
    );

    // Trying to reedem the code for the third time results in an error.
    db.create_invite_from_code(
        &invite_code,
        "user4@example.com",
        Some("user-4-device-id"),
        true,
    )
    .await
    .unwrap_err();

    // Invite count can be updated after the code has been created.
    db.set_invite_count_for_user(user1, 2).await.unwrap();
    let (latest_code, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
    assert_eq!(latest_code, invite_code); // Invite code doesn't change when we increment above 0
    assert_eq!(invite_count, 2);

    // User 4 can now redeem the invite code and becomes a contact of user 1.
    let user4_invite = db
        .create_invite_from_code(
            &invite_code,
            "user4@example.com",
            Some("user-4-device-id"),
            true,
        )
        .await
        .unwrap();
    let user4 = db
        .create_user_from_invite(
            &user4_invite,
            NewUserParams {
                github_login: "user-4".into(),
                github_user_id: 4,
                invite_count: 5,
            },
        )
        .await
        .unwrap()
        .unwrap()
        .user_id;

    let (_, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
    assert_eq!(invite_count, 1);
    assert_eq!(
        db.get_contacts(user1).await.unwrap(),
        [
            Contact::Accepted {
                user_id: user2,
                should_notify: true,
                busy: false,
            },
            Contact::Accepted {
                user_id: user3,
                should_notify: true,
                busy: false,
            },
            Contact::Accepted {
                user_id: user4,
                should_notify: true,
                busy: false,
            }
        ]
    );
    assert_eq!(
        db.get_contacts(user4).await.unwrap(),
        [Contact::Accepted {
            user_id: user1,
            should_notify: false,
            busy: false,
        }]
    );
    assert!(db.has_contact(user1, user4).await.unwrap());
    assert!(db.has_contact(user4, user1).await.unwrap());
    assert_eq!(
        db.get_invite_code_for_user(user4).await.unwrap().unwrap().1,
        5
    );

    // An existing user cannot redeem invite codes.
    db.create_invite_from_code(
        &invite_code,
        "user2@example.com",
        Some("user-2-device-id"),
        true,
    )
    .await
    .unwrap_err();
    let (_, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
    assert_eq!(invite_count, 1);

    // A newer user can invite an existing one via a different email address
    // than the one they used to sign up.
    let user5 = db
        .create_user(
            "user5@example.com",
            false,
            NewUserParams {
                github_login: "user5".into(),
                github_user_id: 5,
                invite_count: 0,
            },
        )
        .await
        .unwrap()
        .user_id;
    db.set_invite_count_for_user(user5, 5).await.unwrap();
    let (user5_invite_code, _) = db.get_invite_code_for_user(user5).await.unwrap().unwrap();
    let user5_invite_to_user1 = db
        .create_invite_from_code(&user5_invite_code, "user1@different.com", None, true)
        .await
        .unwrap();
    let user1_2 = db
        .create_user_from_invite(
            &user5_invite_to_user1,
            NewUserParams {
                github_login: "user1".into(),
                github_user_id: 1,
                invite_count: 5,
            },
        )
        .await
        .unwrap()
        .unwrap()
        .user_id;
    assert_eq!(user1_2, user1);
    assert_eq!(
        db.get_contacts(user1).await.unwrap(),
        [
            Contact::Accepted {
                user_id: user2,
                should_notify: true,
                busy: false,
            },
            Contact::Accepted {
                user_id: user3,
                should_notify: true,
                busy: false,
            },
            Contact::Accepted {
                user_id: user4,
                should_notify: true,
                busy: false,
            },
            Contact::Accepted {
                user_id: user5,
                should_notify: false,
                busy: false,
            }
        ]
    );
    assert_eq!(
        db.get_contacts(user5).await.unwrap(),
        [Contact::Accepted {
            user_id: user1,
            should_notify: true,
            busy: false,
        }]
    );
    assert!(db.has_contact(user1, user5).await.unwrap());
    assert!(db.has_contact(user5, user1).await.unwrap());
}

test_both_dbs!(test_channels_postgres, test_channels_sqlite, db, {
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
});

test_both_dbs!(
    test_joining_channels_postgres,
    test_joining_channels_sqlite,
    db,
    {
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
            .join_room(
                room_1,
                user_1,
                Some(channel_1),
                ConnectionId { owner_id, id: 1 },
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
                Some(channel_1),
                ConnectionId { owner_id, id: 1 }
            )
            .await
            .is_err());
    }
);

test_both_dbs!(
    test_channel_invites_postgres,
    test_channel_invites_sqlite,
    db,
    {
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
);

test_both_dbs!(
    test_channel_renames_postgres,
    test_channel_renames_sqlite,
    db,
    {
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
);

#[gpui::test]
async fn test_multiple_signup_overwrite() {
    let test_db = TestDb::postgres(build_background_executor());
    let db = test_db.db();

    let email_address = "user_1@example.com".to_string();

    let initial_signup_created_at_milliseconds = 0;

    let initial_signup = NewSignup {
        email_address: email_address.clone(),
        platform_mac: false,
        platform_linux: true,
        platform_windows: false,
        editor_features: vec!["speed".into()],
        programming_languages: vec!["rust".into(), "c".into()],
        device_id: Some(format!("device_id")),
        added_to_mailing_list: false,
        created_at: Some(
            DateTime::from_timestamp_millis(initial_signup_created_at_milliseconds).unwrap(),
        ),
    };

    db.create_signup(&initial_signup).await.unwrap();

    let initial_signup_from_db = db.get_signup(&email_address).await.unwrap();

    assert_eq!(
        initial_signup_from_db.clone(),
        signup::Model {
            email_address: initial_signup.email_address,
            platform_mac: initial_signup.platform_mac,
            platform_linux: initial_signup.platform_linux,
            platform_windows: initial_signup.platform_windows,
            editor_features: Some(initial_signup.editor_features),
            programming_languages: Some(initial_signup.programming_languages),
            added_to_mailing_list: initial_signup.added_to_mailing_list,
            ..initial_signup_from_db
        }
    );

    let subsequent_signup = NewSignup {
        email_address: email_address.clone(),
        platform_mac: true,
        platform_linux: false,
        platform_windows: true,
        editor_features: vec!["git integration".into(), "clean design".into()],
        programming_languages: vec!["d".into(), "elm".into()],
        device_id: Some(format!("different_device_id")),
        added_to_mailing_list: true,
        // subsequent signup happens next day
        created_at: Some(
            DateTime::from_timestamp_millis(
                initial_signup_created_at_milliseconds + (1000 * 60 * 60 * 24),
            )
            .unwrap(),
        ),
    };

    db.create_signup(&subsequent_signup).await.unwrap();

    let subsequent_signup_from_db = db.get_signup(&email_address).await.unwrap();

    assert_eq!(
        subsequent_signup_from_db.clone(),
        signup::Model {
            platform_mac: subsequent_signup.platform_mac,
            platform_linux: subsequent_signup.platform_linux,
            platform_windows: subsequent_signup.platform_windows,
            editor_features: Some(subsequent_signup.editor_features),
            programming_languages: Some(subsequent_signup.programming_languages),
            device_id: subsequent_signup.device_id,
            added_to_mailing_list: subsequent_signup.added_to_mailing_list,
            // shouldn't overwrite their creation Datetime - user shouldn't lose their spot in line
            created_at: initial_signup_from_db.created_at,
            ..subsequent_signup_from_db
        }
    );
}

#[gpui::test]
async fn test_signups() {
    let test_db = TestDb::postgres(build_background_executor());
    let db = test_db.db();

    let usernames = (0..8).map(|i| format!("person-{i}")).collect::<Vec<_>>();

    let all_signups = usernames
        .iter()
        .enumerate()
        .map(|(i, username)| NewSignup {
            email_address: format!("{username}@example.com"),
            platform_mac: true,
            platform_linux: i % 2 == 0,
            platform_windows: i % 4 == 0,
            editor_features: vec!["speed".into()],
            programming_languages: vec!["rust".into(), "c".into()],
            device_id: Some(format!("device_id_{i}")),
            added_to_mailing_list: i != 0, // One user failed to subscribe
            created_at: Some(DateTime::from_timestamp_millis(i as i64).unwrap()), // Signups are consecutive
        })
        .collect::<Vec<NewSignup>>();

    // people sign up on the waitlist
    for signup in &all_signups {
        // users can sign up multiple times without issues
        for _ in 0..2 {
            db.create_signup(&signup).await.unwrap();
        }
    }

    assert_eq!(
        db.get_waitlist_summary().await.unwrap(),
        WaitlistSummary {
            count: 8,
            mac_count: 8,
            linux_count: 4,
            windows_count: 2,
            unknown_count: 0,
        }
    );

    // retrieve the next batch of signup emails to send
    let signups_batch1 = db.get_unsent_invites(3).await.unwrap();
    let addresses = signups_batch1
        .iter()
        .map(|s| &s.email_address)
        .collect::<Vec<_>>();
    assert_eq!(
        addresses,
        &[
            all_signups[0].email_address.as_str(),
            all_signups[1].email_address.as_str(),
            all_signups[2].email_address.as_str()
        ]
    );
    assert_ne!(
        signups_batch1[0].email_confirmation_code,
        signups_batch1[1].email_confirmation_code
    );

    // the waitlist isn't updated until we record that the emails
    // were successfully sent.
    let signups_batch = db.get_unsent_invites(3).await.unwrap();
    assert_eq!(signups_batch, signups_batch1);

    // once the emails go out, we can retrieve the next batch
    // of signups.
    db.record_sent_invites(&signups_batch1).await.unwrap();
    let signups_batch2 = db.get_unsent_invites(3).await.unwrap();
    let addresses = signups_batch2
        .iter()
        .map(|s| &s.email_address)
        .collect::<Vec<_>>();
    assert_eq!(
        addresses,
        &[
            all_signups[3].email_address.as_str(),
            all_signups[4].email_address.as_str(),
            all_signups[5].email_address.as_str()
        ]
    );

    // the sent invites are excluded from the summary.
    assert_eq!(
        db.get_waitlist_summary().await.unwrap(),
        WaitlistSummary {
            count: 5,
            mac_count: 5,
            linux_count: 2,
            windows_count: 1,
            unknown_count: 0,
        }
    );

    // user completes the signup process by providing their
    // github account.
    let NewUserResult {
        user_id,
        inviting_user_id,
        signup_device_id,
        ..
    } = db
        .create_user_from_invite(
            &Invite {
                ..signups_batch1[0].clone()
            },
            NewUserParams {
                github_login: usernames[0].clone(),
                github_user_id: 0,
                invite_count: 5,
            },
        )
        .await
        .unwrap()
        .unwrap();
    let user = db.get_user_by_id(user_id).await.unwrap().unwrap();
    assert!(inviting_user_id.is_none());
    assert_eq!(user.github_login, usernames[0]);
    assert_eq!(
        user.email_address,
        Some(all_signups[0].email_address.clone())
    );
    assert_eq!(user.invite_count, 5);
    assert_eq!(signup_device_id.unwrap(), "device_id_0");

    // cannot redeem the same signup again.
    assert!(db
        .create_user_from_invite(
            &Invite {
                email_address: signups_batch1[0].email_address.clone(),
                email_confirmation_code: signups_batch1[0].email_confirmation_code.clone(),
            },
            NewUserParams {
                github_login: "some-other-github_account".into(),
                github_user_id: 1,
                invite_count: 5,
            },
        )
        .await
        .unwrap()
        .is_none());

    // cannot redeem a signup with the wrong confirmation code.
    db.create_user_from_invite(
        &Invite {
            email_address: signups_batch1[1].email_address.clone(),
            email_confirmation_code: "the-wrong-code".to_string(),
        },
        NewUserParams {
            github_login: usernames[1].clone(),
            github_user_id: 2,
            invite_count: 5,
        },
    )
    .await
    .unwrap_err();
}

fn build_background_executor() -> Arc<Background> {
    Deterministic::new(0).build_background()
}
