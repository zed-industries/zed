use super::db::*;
use collections::HashMap;
use gpui::executor::{Background, Deterministic};
use std::{sync::Arc, time::Duration};
use time::OffsetDateTime;

#[tokio::test(flavor = "multi_thread")]
async fn test_get_users_by_ids() {
    for test_db in [
        TestDb::postgres().await,
        TestDb::fake(build_background_executor()),
    ] {
        let db = test_db.db();

        let user1 = db
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
            .unwrap();
        let user2 = db
            .create_user(
                "u2@example.com",
                false,
                NewUserParams {
                    github_login: "u2".into(),
                    github_user_id: 2,
                    invite_count: 0,
                },
            )
            .await
            .unwrap();
        let user3 = db
            .create_user(
                "u3@example.com",
                false,
                NewUserParams {
                    github_login: "u3".into(),
                    github_user_id: 3,
                    invite_count: 0,
                },
            )
            .await
            .unwrap();
        let user4 = db
            .create_user(
                "u4@example.com",
                false,
                NewUserParams {
                    github_login: "u4".into(),
                    github_user_id: 4,
                    invite_count: 0,
                },
            )
            .await
            .unwrap();

        assert_eq!(
            db.get_users_by_ids(vec![user1, user2, user3, user4])
                .await
                .unwrap(),
            vec![
                User {
                    id: user1,
                    github_login: "u1".to_string(),
                    github_user_id: Some(1),
                    email_address: Some("u1@example.com".to_string()),
                    admin: false,
                    ..Default::default()
                },
                User {
                    id: user2,
                    github_login: "u2".to_string(),
                    github_user_id: Some(2),
                    email_address: Some("u2@example.com".to_string()),
                    admin: false,
                    ..Default::default()
                },
                User {
                    id: user3,
                    github_login: "u3".to_string(),
                    github_user_id: Some(3),
                    email_address: Some("u3@example.com".to_string()),
                    admin: false,
                    ..Default::default()
                },
                User {
                    id: user4,
                    github_login: "u4".to_string(),
                    github_user_id: Some(4),
                    email_address: Some("u4@example.com".to_string()),
                    admin: false,
                    ..Default::default()
                }
            ]
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_user_by_github_account() {
    for test_db in [
        TestDb::postgres().await,
        TestDb::fake(build_background_executor()),
    ] {
        let db = test_db.db();
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
            .unwrap();
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
            .unwrap();

        let user = db
            .get_user_by_github_account("login1", None)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(user.id, user_id1);
        assert_eq!(&user.github_login, "login1");
        assert_eq!(user.github_user_id, Some(101));

        assert!(db
            .get_user_by_github_account("non-existent-login", None)
            .await
            .unwrap()
            .is_none());

        let user = db
            .get_user_by_github_account("the-new-login2", Some(102))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(user.id, user_id2);
        assert_eq!(&user.github_login, "the-new-login2");
        assert_eq!(user.github_user_id, Some(102));
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_worktree_extensions() {
    let test_db = TestDb::postgres().await;
    let db = test_db.db();

    let user = db
        .create_user(
            "u1@example.com",
            false,
            NewUserParams {
                github_login: "u1".into(),
                github_user_id: 0,
                invite_count: 0,
            },
        )
        .await
        .unwrap();
    let project = db.register_project(user).await.unwrap();

    db.update_worktree_extensions(project, 100, Default::default())
        .await
        .unwrap();
    db.update_worktree_extensions(
        project,
        100,
        [("rs".to_string(), 5), ("md".to_string(), 3)]
            .into_iter()
            .collect(),
    )
    .await
    .unwrap();
    db.update_worktree_extensions(
        project,
        100,
        [("rs".to_string(), 6), ("md".to_string(), 5)]
            .into_iter()
            .collect(),
    )
    .await
    .unwrap();
    db.update_worktree_extensions(
        project,
        101,
        [("ts".to_string(), 2), ("md".to_string(), 1)]
            .into_iter()
            .collect(),
    )
    .await
    .unwrap();

    assert_eq!(
        db.get_project_extensions(project).await.unwrap(),
        [
            (
                100,
                [("rs".into(), 6), ("md".into(), 5),]
                    .into_iter()
                    .collect::<HashMap<_, _>>()
            ),
            (
                101,
                [("ts".into(), 2), ("md".into(), 1),]
                    .into_iter()
                    .collect::<HashMap<_, _>>()
            )
        ]
        .into_iter()
        .collect()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_user_activity() {
    let test_db = TestDb::postgres().await;
    let db = test_db.db();

    let user_1 = db
        .create_user(
            "u1@example.com",
            false,
            NewUserParams {
                github_login: "u1".into(),
                github_user_id: 0,
                invite_count: 0,
            },
        )
        .await
        .unwrap();
    let user_2 = db
        .create_user(
            "u2@example.com",
            false,
            NewUserParams {
                github_login: "u2".into(),
                github_user_id: 0,
                invite_count: 0,
            },
        )
        .await
        .unwrap();
    let user_3 = db
        .create_user(
            "u3@example.com",
            false,
            NewUserParams {
                github_login: "u3".into(),
                github_user_id: 0,
                invite_count: 0,
            },
        )
        .await
        .unwrap();
    let project_1 = db.register_project(user_1).await.unwrap();
    db.update_worktree_extensions(
        project_1,
        1,
        HashMap::from_iter([("rs".into(), 5), ("md".into(), 7)]),
    )
    .await
    .unwrap();
    let project_2 = db.register_project(user_2).await.unwrap();
    let t0 = OffsetDateTime::now_utc() - Duration::from_secs(60 * 60);

    // User 2 opens a project
    let t1 = t0 + Duration::from_secs(10);
    db.record_user_activity(t0..t1, &[(user_2, project_2)])
        .await
        .unwrap();

    let t2 = t1 + Duration::from_secs(10);
    db.record_user_activity(t1..t2, &[(user_2, project_2)])
        .await
        .unwrap();

    // User 1 joins the project
    let t3 = t2 + Duration::from_secs(10);
    db.record_user_activity(t2..t3, &[(user_2, project_2), (user_1, project_2)])
        .await
        .unwrap();

    // User 1 opens another project
    let t4 = t3 + Duration::from_secs(10);
    db.record_user_activity(
        t3..t4,
        &[
            (user_2, project_2),
            (user_1, project_2),
            (user_1, project_1),
        ],
    )
    .await
    .unwrap();

    // User 3 joins that project
    let t5 = t4 + Duration::from_secs(10);
    db.record_user_activity(
        t4..t5,
        &[
            (user_2, project_2),
            (user_1, project_2),
            (user_1, project_1),
            (user_3, project_1),
        ],
    )
    .await
    .unwrap();

    // User 2 leaves
    let t6 = t5 + Duration::from_secs(5);
    db.record_user_activity(t5..t6, &[(user_1, project_1), (user_3, project_1)])
        .await
        .unwrap();

    let t7 = t6 + Duration::from_secs(60);
    let t8 = t7 + Duration::from_secs(10);
    db.record_user_activity(t7..t8, &[(user_1, project_1)])
        .await
        .unwrap();

    assert_eq!(
        db.get_top_users_activity_summary(t0..t6, 10).await.unwrap(),
        &[
            UserActivitySummary {
                id: user_1,
                github_login: "u1".to_string(),
                project_activity: vec![
                    ProjectActivitySummary {
                        id: project_1,
                        duration: Duration::from_secs(25),
                        max_collaborators: 2
                    },
                    ProjectActivitySummary {
                        id: project_2,
                        duration: Duration::from_secs(30),
                        max_collaborators: 2
                    }
                ]
            },
            UserActivitySummary {
                id: user_2,
                github_login: "u2".to_string(),
                project_activity: vec![ProjectActivitySummary {
                    id: project_2,
                    duration: Duration::from_secs(50),
                    max_collaborators: 2
                }]
            },
            UserActivitySummary {
                id: user_3,
                github_login: "u3".to_string(),
                project_activity: vec![ProjectActivitySummary {
                    id: project_1,
                    duration: Duration::from_secs(15),
                    max_collaborators: 2
                }]
            },
        ]
    );

    assert_eq!(
        db.get_active_user_count(t0..t6, Duration::from_secs(56), false)
            .await
            .unwrap(),
        0
    );
    assert_eq!(
        db.get_active_user_count(t0..t6, Duration::from_secs(56), true)
            .await
            .unwrap(),
        0
    );
    assert_eq!(
        db.get_active_user_count(t0..t6, Duration::from_secs(54), false)
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        db.get_active_user_count(t0..t6, Duration::from_secs(54), true)
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        db.get_active_user_count(t0..t6, Duration::from_secs(30), false)
            .await
            .unwrap(),
        2
    );
    assert_eq!(
        db.get_active_user_count(t0..t6, Duration::from_secs(30), true)
            .await
            .unwrap(),
        2
    );
    assert_eq!(
        db.get_active_user_count(t0..t6, Duration::from_secs(10), false)
            .await
            .unwrap(),
        3
    );
    assert_eq!(
        db.get_active_user_count(t0..t6, Duration::from_secs(10), true)
            .await
            .unwrap(),
        3
    );
    assert_eq!(
        db.get_active_user_count(t0..t1, Duration::from_secs(5), false)
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        db.get_active_user_count(t0..t1, Duration::from_secs(5), true)
            .await
            .unwrap(),
        0
    );

    assert_eq!(
        db.get_user_activity_timeline(t3..t6, user_1).await.unwrap(),
        &[
            UserActivityPeriod {
                project_id: project_1,
                start: t3,
                end: t6,
                extensions: HashMap::from_iter([("rs".to_string(), 5), ("md".to_string(), 7)]),
            },
            UserActivityPeriod {
                project_id: project_2,
                start: t3,
                end: t5,
                extensions: Default::default(),
            },
        ]
    );
    assert_eq!(
        db.get_user_activity_timeline(t0..t8, user_1).await.unwrap(),
        &[
            UserActivityPeriod {
                project_id: project_2,
                start: t2,
                end: t5,
                extensions: Default::default(),
            },
            UserActivityPeriod {
                project_id: project_1,
                start: t3,
                end: t6,
                extensions: HashMap::from_iter([("rs".to_string(), 5), ("md".to_string(), 7)]),
            },
            UserActivityPeriod {
                project_id: project_1,
                start: t7,
                end: t8,
                extensions: HashMap::from_iter([("rs".to_string(), 5), ("md".to_string(), 7)]),
            },
        ]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_recent_channel_messages() {
    for test_db in [
        TestDb::postgres().await,
        TestDb::fake(build_background_executor()),
    ] {
        let db = test_db.db();
        let user = db
            .create_user(
                "u@example.com",
                false,
                NewUserParams {
                    github_login: "u".into(),
                    github_user_id: 1,
                    invite_count: 0,
                },
            )
            .await
            .unwrap();
        let org = db.create_org("org", "org").await.unwrap();
        let channel = db.create_org_channel(org, "channel").await.unwrap();
        for i in 0..10 {
            db.create_channel_message(channel, user, &i.to_string(), OffsetDateTime::now_utc(), i)
                .await
                .unwrap();
        }

        let messages = db.get_channel_messages(channel, 5, None).await.unwrap();
        assert_eq!(
            messages.iter().map(|m| &m.body).collect::<Vec<_>>(),
            ["5", "6", "7", "8", "9"]
        );

        let prev_messages = db
            .get_channel_messages(channel, 4, Some(messages[0].id))
            .await
            .unwrap();
        assert_eq!(
            prev_messages.iter().map(|m| &m.body).collect::<Vec<_>>(),
            ["1", "2", "3", "4"]
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_channel_message_nonces() {
    for test_db in [
        TestDb::postgres().await,
        TestDb::fake(build_background_executor()),
    ] {
        let db = test_db.db();
        let user = db
            .create_user(
                "user@example.com",
                false,
                NewUserParams {
                    github_login: "user".into(),
                    github_user_id: 1,
                    invite_count: 0,
                },
            )
            .await
            .unwrap();
        let org = db.create_org("org", "org").await.unwrap();
        let channel = db.create_org_channel(org, "channel").await.unwrap();

        let msg1_id = db
            .create_channel_message(channel, user, "1", OffsetDateTime::now_utc(), 1)
            .await
            .unwrap();
        let msg2_id = db
            .create_channel_message(channel, user, "2", OffsetDateTime::now_utc(), 2)
            .await
            .unwrap();
        let msg3_id = db
            .create_channel_message(channel, user, "3", OffsetDateTime::now_utc(), 1)
            .await
            .unwrap();
        let msg4_id = db
            .create_channel_message(channel, user, "4", OffsetDateTime::now_utc(), 2)
            .await
            .unwrap();

        assert_ne!(msg1_id, msg2_id);
        assert_eq!(msg1_id, msg3_id);
        assert_eq!(msg2_id, msg4_id);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_create_access_tokens() {
    let test_db = TestDb::postgres().await;
    let db = test_db.db();
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
        .unwrap();

    db.create_access_token_hash(user, "h1", 3).await.unwrap();
    db.create_access_token_hash(user, "h2", 3).await.unwrap();
    assert_eq!(
        db.get_access_token_hashes(user).await.unwrap(),
        &["h2".to_string(), "h1".to_string()]
    );

    db.create_access_token_hash(user, "h3", 3).await.unwrap();
    assert_eq!(
        db.get_access_token_hashes(user).await.unwrap(),
        &["h3".to_string(), "h2".to_string(), "h1".to_string(),]
    );

    db.create_access_token_hash(user, "h4", 3).await.unwrap();
    assert_eq!(
        db.get_access_token_hashes(user).await.unwrap(),
        &["h4".to_string(), "h3".to_string(), "h2".to_string(),]
    );

    db.create_access_token_hash(user, "h5", 3).await.unwrap();
    assert_eq!(
        db.get_access_token_hashes(user).await.unwrap(),
        &["h5".to_string(), "h4".to_string(), "h3".to_string()]
    );
}

#[test]
fn test_fuzzy_like_string() {
    assert_eq!(PostgresDb::fuzzy_like_string("abcd"), "%a%b%c%d%");
    assert_eq!(PostgresDb::fuzzy_like_string("x y"), "%x%y%");
    assert_eq!(PostgresDb::fuzzy_like_string(" z  "), "%z%");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fuzzy_search_users() {
    let test_db = TestDb::postgres().await;
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

    async fn fuzzy_search_user_names(db: &Arc<dyn Db>, query: &str) -> Vec<String> {
        db.fuzzy_search_users(query, 10)
            .await
            .unwrap()
            .into_iter()
            .map(|user| user.github_login)
            .collect::<Vec<_>>()
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_add_contacts() {
    for test_db in [
        TestDb::postgres().await,
        TestDb::fake(build_background_executor()),
    ] {
        let db = test_db.db();

        let user_1 = db
            .create_user(
                "u1@example.com",
                false,
                NewUserParams {
                    github_login: "u1".into(),
                    github_user_id: 0,
                    invite_count: 0,
                },
            )
            .await
            .unwrap();
        let user_2 = db
            .create_user(
                "u2@example.com",
                false,
                NewUserParams {
                    github_login: "u2".into(),
                    github_user_id: 1,
                    invite_count: 0,
                },
            )
            .await
            .unwrap();
        let user_3 = db
            .create_user(
                "u3@example.com",
                false,
                NewUserParams {
                    github_login: "u3".into(),
                    github_user_id: 2,
                    invite_count: 0,
                },
            )
            .await
            .unwrap();

        // User starts with no contacts
        assert_eq!(
            db.get_contacts(user_1).await.unwrap(),
            vec![Contact::Accepted {
                user_id: user_1,
                should_notify: false
            }],
        );

        // User requests a contact. Both users see the pending request.
        db.send_contact_request(user_1, user_2).await.unwrap();
        assert!(!db.has_contact(user_1, user_2).await.unwrap());
        assert!(!db.has_contact(user_2, user_1).await.unwrap());
        assert_eq!(
            db.get_contacts(user_1).await.unwrap(),
            &[
                Contact::Accepted {
                    user_id: user_1,
                    should_notify: false
                },
                Contact::Outgoing { user_id: user_2 }
            ],
        );
        assert_eq!(
            db.get_contacts(user_2).await.unwrap(),
            &[
                Contact::Incoming {
                    user_id: user_1,
                    should_notify: true
                },
                Contact::Accepted {
                    user_id: user_2,
                    should_notify: false
                },
            ]
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
            &[
                Contact::Incoming {
                    user_id: user_1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user_2,
                    should_notify: false
                },
            ]
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
            &[
                Contact::Accepted {
                    user_id: user_1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user_2,
                    should_notify: true
                }
            ],
        );
        assert!(db.has_contact(user_1, user_2).await.unwrap());
        assert!(db.has_contact(user_2, user_1).await.unwrap());
        assert_eq!(
            db.get_contacts(user_2).await.unwrap(),
            &[
                Contact::Accepted {
                    user_id: user_1,
                    should_notify: false,
                },
                Contact::Accepted {
                    user_id: user_2,
                    should_notify: false,
                },
            ]
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
            &[
                Contact::Accepted {
                    user_id: user_1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user_2,
                    should_notify: true,
                },
            ]
        );

        // Users can dismiss notifications of other users accepting their requests.
        db.dismiss_contact_notification(user_1, user_2)
            .await
            .unwrap();
        assert_eq!(
            db.get_contacts(user_1).await.unwrap(),
            &[
                Contact::Accepted {
                    user_id: user_1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user_2,
                    should_notify: false,
                },
            ]
        );

        // Users send each other concurrent contact requests and
        // see that they are immediately accepted.
        db.send_contact_request(user_1, user_3).await.unwrap();
        db.send_contact_request(user_3, user_1).await.unwrap();
        assert_eq!(
            db.get_contacts(user_1).await.unwrap(),
            &[
                Contact::Accepted {
                    user_id: user_1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user_2,
                    should_notify: false,
                },
                Contact::Accepted {
                    user_id: user_3,
                    should_notify: false
                },
            ]
        );
        assert_eq!(
            db.get_contacts(user_3).await.unwrap(),
            &[
                Contact::Accepted {
                    user_id: user_1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user_3,
                    should_notify: false
                }
            ],
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
            &[
                Contact::Accepted {
                    user_id: user_1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user_2,
                    should_notify: false
                }
            ]
        );
        assert_eq!(
            db.get_contacts(user_3).await.unwrap(),
            &[
                Contact::Accepted {
                    user_id: user_1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user_3,
                    should_notify: false
                }
            ],
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_invite_codes() {
    let postgres = TestDb::postgres().await;
    let db = postgres.db();
    let user1 = db
        .create_user(
            "u1@example.com",
            false,
            NewUserParams {
                github_login: "u1".into(),
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
        .create_invite_from_code(&invite_code, "u2@example.com")
        .await
        .unwrap();
    let (user2, inviter, _) = db
        .create_user_from_invite(
            &user2_invite,
            NewUserParams {
                github_login: "user2".into(),
                github_user_id: 2,
                invite_count: 7,
            },
        )
        .await
        .unwrap();
    let (_, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
    assert_eq!(invite_count, 1);
    assert_eq!(inviter, Some(user1));
    assert_eq!(
        db.get_contacts(user1).await.unwrap(),
        [
            Contact::Accepted {
                user_id: user1,
                should_notify: false
            },
            Contact::Accepted {
                user_id: user2,
                should_notify: true
            }
        ]
    );
    assert_eq!(
        db.get_contacts(user2).await.unwrap(),
        [
            Contact::Accepted {
                user_id: user1,
                should_notify: false
            },
            Contact::Accepted {
                user_id: user2,
                should_notify: false
            }
        ]
    );
    assert_eq!(
        db.get_invite_code_for_user(user2).await.unwrap().unwrap().1,
        7
    );

    // User 3 redeems the invite code and becomes a contact of user 1.
    let user3_invite = db
        .create_invite_from_code(&invite_code, "u3@example.com")
        .await
        .unwrap();
    let (user3, inviter, _) = db
        .create_user_from_invite(
            &user3_invite,
            NewUserParams {
                github_login: "user-3".into(),
                github_user_id: 3,
                invite_count: 3,
            },
        )
        .await
        .unwrap();
    let (_, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
    assert_eq!(invite_count, 0);
    assert_eq!(inviter, Some(user1));
    assert_eq!(
        db.get_contacts(user1).await.unwrap(),
        [
            Contact::Accepted {
                user_id: user1,
                should_notify: false
            },
            Contact::Accepted {
                user_id: user2,
                should_notify: true
            },
            Contact::Accepted {
                user_id: user3,
                should_notify: true
            }
        ]
    );
    assert_eq!(
        db.get_contacts(user3).await.unwrap(),
        [
            Contact::Accepted {
                user_id: user1,
                should_notify: false
            },
            Contact::Accepted {
                user_id: user3,
                should_notify: false
            },
        ]
    );
    assert_eq!(
        db.get_invite_code_for_user(user3).await.unwrap().unwrap().1,
        3
    );

    // Trying to reedem the code for the third time results in an error.
    db.create_invite_from_code(&invite_code, "u4@example.com")
        .await
        .unwrap_err();

    // Invite count can be updated after the code has been created.
    db.set_invite_count_for_user(user1, 2).await.unwrap();
    let (latest_code, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
    assert_eq!(latest_code, invite_code); // Invite code doesn't change when we increment above 0
    assert_eq!(invite_count, 2);

    // User 4 can now redeem the invite code and becomes a contact of user 1.
    let user4_invite = db
        .create_invite_from_code(&invite_code, "u4@example.com")
        .await
        .unwrap();
    let (user4, _, _) = db
        .create_user_from_invite(
            &user4_invite,
            NewUserParams {
                github_login: "user-4".into(),
                github_user_id: 4,
                invite_count: 5,
            },
        )
        .await
        .unwrap();

    let (_, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
    assert_eq!(invite_count, 1);
    assert_eq!(
        db.get_contacts(user1).await.unwrap(),
        [
            Contact::Accepted {
                user_id: user1,
                should_notify: false
            },
            Contact::Accepted {
                user_id: user2,
                should_notify: true
            },
            Contact::Accepted {
                user_id: user3,
                should_notify: true
            },
            Contact::Accepted {
                user_id: user4,
                should_notify: true
            }
        ]
    );
    assert_eq!(
        db.get_contacts(user4).await.unwrap(),
        [
            Contact::Accepted {
                user_id: user1,
                should_notify: false
            },
            Contact::Accepted {
                user_id: user4,
                should_notify: false
            },
        ]
    );
    assert_eq!(
        db.get_invite_code_for_user(user4).await.unwrap().unwrap().1,
        5
    );

    // An existing user cannot redeem invite codes.
    db.create_invite_from_code(&invite_code, "u2@example.com")
        .await
        .unwrap_err();
    let (_, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
    assert_eq!(invite_count, 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signups() {
    let postgres = TestDb::postgres().await;
    let db = postgres.db();

    // people sign up on the waitlist
    for i in 0..8 {
        db.create_signup(Signup {
            email_address: format!("person-{i}@example.com"),
            platform_mac: true,
            platform_linux: i % 2 == 0,
            platform_windows: i % 4 == 0,
            editor_features: vec!["speed".into()],
            programming_languages: vec!["rust".into(), "c".into()],
            device_id: format!("device_id_{i}"),
        })
        .await
        .unwrap();
    }

    assert_eq!(
        db.get_waitlist_summary().await.unwrap(),
        WaitlistSummary {
            count: 8,
            mac_count: 8,
            linux_count: 4,
            windows_count: 2,
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
            "person-0@example.com",
            "person-1@example.com",
            "person-2@example.com"
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
            "person-3@example.com",
            "person-4@example.com",
            "person-5@example.com"
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
        }
    );

    // user completes the signup process by providing their
    // github account.
    let (user_id, inviter_id, signup_device_id) = db
        .create_user_from_invite(
            &Invite {
                email_address: signups_batch1[0].email_address.clone(),
                email_confirmation_code: signups_batch1[0].email_confirmation_code.clone(),
            },
            NewUserParams {
                github_login: "person-0".into(),
                github_user_id: 0,
                invite_count: 5,
            },
        )
        .await
        .unwrap();
    let user = db.get_user_by_id(user_id).await.unwrap().unwrap();
    assert!(inviter_id.is_none());
    assert_eq!(user.github_login, "person-0");
    assert_eq!(user.email_address.as_deref(), Some("person-0@example.com"));
    assert_eq!(user.invite_count, 5);
    assert_eq!(signup_device_id, "device_id_0");

    // cannot redeem the same signup again.
    db.create_user_from_invite(
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
    .unwrap_err();

    // cannot redeem a signup with the wrong confirmation code.
    db.create_user_from_invite(
        &Invite {
            email_address: signups_batch1[1].email_address.clone(),
            email_confirmation_code: "the-wrong-code".to_string(),
        },
        NewUserParams {
            github_login: "person-1".into(),
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
