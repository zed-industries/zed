use crate::{
    db::{Database, NewUserParams},
    test_both_dbs,
};
use pretty_assertions::assert_eq;
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

    const FEATURE_FLAG_ONE: &str = "brand-new-ux";
    const FEATURE_FLAG_TWO: &str = "cool-feature";
    const FEATURE_FLAG_THREE: &str = "feature-enabled-for-everyone";

    let feature_flag_one = db.create_user_flag(FEATURE_FLAG_ONE, false).await.unwrap();
    let feature_flag_two = db.create_user_flag(FEATURE_FLAG_TWO, false).await.unwrap();
    db.create_user_flag(FEATURE_FLAG_THREE, true).await.unwrap();

    db.add_user_flag(user_1, feature_flag_one).await.unwrap();
    db.add_user_flag(user_1, feature_flag_two).await.unwrap();

    db.add_user_flag(user_2, feature_flag_one).await.unwrap();

    let mut user_1_flags = db.get_user_flags(user_1).await.unwrap();
    user_1_flags.sort();
    assert_eq!(
        user_1_flags,
        &[FEATURE_FLAG_ONE, FEATURE_FLAG_TWO, FEATURE_FLAG_THREE]
    );

    let mut user_2_flags = db.get_user_flags(user_2).await.unwrap();
    user_2_flags.sort();
    assert_eq!(user_2_flags, &[FEATURE_FLAG_ONE, FEATURE_FLAG_THREE]);
}
