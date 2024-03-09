use crate::{
    db::{Database, NewUserParams},
    test_both_dbs,
};
use std::sync::Arc;

test_both_dbs!(
    test_get_user_flags,
    test_get_user_flags_postgres,
    test_get_user_flags_sqlite
);

async fn test_get_user_flags(db: &Arc<Database>) {
    let user_1 = db
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

    let user_2 = db
        .create_user(
            "user2@example.com",
            false,
            NewUserParams {
                github_login: "user2".to_string(),
                github_user_id: 2,
            },
        )
        .await
        .unwrap()
        .user_id;

    const CHANNELS_ALPHA: &str = "channels-alpha";
    const NEW_SEARCH: &str = "new-search";

    let channels_flag = db.create_user_flag(CHANNELS_ALPHA).await.unwrap();
    let search_flag = db.create_user_flag(NEW_SEARCH).await.unwrap();

    db.add_user_flag(user_1, channels_flag).await.unwrap();
    db.add_user_flag(user_1, search_flag).await.unwrap();

    db.add_user_flag(user_2, channels_flag).await.unwrap();

    let mut user_1_flags = db.get_user_flags(user_1).await.unwrap();
    user_1_flags.sort();
    assert_eq!(user_1_flags, &[CHANNELS_ALPHA, NEW_SEARCH]);

    let mut user_2_flags = db.get_user_flags(user_2).await.unwrap();
    user_2_flags.sort();
    assert_eq!(user_2_flags, &[CHANNELS_ALPHA]);
}
