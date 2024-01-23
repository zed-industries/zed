use super::Database;
use crate::{db::NewUserParams, test_both_dbs};
use std::sync::Arc;

test_both_dbs!(
    test_contributors,
    test_contributors_postgres,
    test_contributors_sqlite
);

async fn test_contributors(db: &Arc<Database>) {
    db.create_user(
        &format!("user1@example.com"),
        false,
        NewUserParams {
            github_login: format!("user1"),
            github_user_id: 1,
        },
    )
    .await
    .unwrap()
    .user_id;

    assert_eq!(db.get_contributors().await.unwrap(), Vec::<String>::new());

    db.add_contributor("user1", Some(1), None).await.unwrap();
    assert_eq!(
        db.get_contributors().await.unwrap(),
        vec!["user1".to_string()]
    );

    db.add_contributor("user2", Some(2), None).await.unwrap();
    assert_eq!(
        db.get_contributors().await.unwrap(),
        vec!["user1".to_string(), "user2".to_string()]
    );
}
