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
