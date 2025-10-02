use chrono::Utc;

use crate::{
    db::{Database, NewUserParams},
    test_both_dbs,
};
use std::sync::Arc;

test_both_dbs!(
    test_accepted_tos,
    test_accepted_tos_postgres,
    test_accepted_tos_sqlite
);

async fn test_accepted_tos(db: &Arc<Database>) {
    let user_id = db
        .create_user(
            "user1@example.com",
            None,
            false,
            NewUserParams {
                github_login: "user1".to_string(),
                github_user_id: 1,
            },
        )
        .await
        .unwrap()
        .user_id;

    let user = db.get_user_by_id(user_id).await.unwrap().unwrap();
    assert!(user.accepted_tos_at.is_none());

    let accepted_tos_at = Utc::now().naive_utc();
    db.set_user_accepted_tos_at(user_id, Some(accepted_tos_at))
        .await
        .unwrap();

    let user = db.get_user_by_id(user_id).await.unwrap().unwrap();
    assert!(user.accepted_tos_at.is_some());
    assert_eq!(user.accepted_tos_at, Some(accepted_tos_at));

    db.set_user_accepted_tos_at(user_id, None).await.unwrap();

    let user = db.get_user_by_id(user_id).await.unwrap().unwrap();
    assert!(user.accepted_tos_at.is_none());
}

test_both_dbs!(
    test_destroy_user_cascade_deletes_access_tokens,
    test_destroy_user_cascade_deletes_access_tokens_postgres,
    test_destroy_user_cascade_deletes_access_tokens_sqlite
);

async fn test_destroy_user_cascade_deletes_access_tokens(db: &Arc<Database>) {
    let user_id = db
        .create_user(
            "user1@example.com",
            Some("user1"),
            false,
            NewUserParams {
                github_login: "user1".to_string(),
                github_user_id: 12345,
            },
        )
        .await
        .unwrap()
        .user_id;

    let user = db.get_user_by_id(user_id).await.unwrap();
    assert!(user.is_some());

    let token_1_id = db
        .create_access_token(user_id, None, "token-1", 10)
        .await
        .unwrap();

    let token_2_id = db
        .create_access_token(user_id, None, "token-2", 10)
        .await
        .unwrap();

    let token_1 = db.get_access_token(token_1_id).await;
    let token_2 = db.get_access_token(token_2_id).await;
    assert!(token_1.is_ok());
    assert!(token_2.is_ok());

    db.destroy_user(user_id).await.unwrap();

    let user = db.get_user_by_id(user_id).await.unwrap();
    assert!(user.is_none());

    let token_1 = db.get_access_token(token_1_id).await;
    let token_2 = db.get_access_token(token_2_id).await;
    assert!(token_1.is_err());
    assert!(token_2.is_err());
}
