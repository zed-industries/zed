use super::TestDb;
use crate::db::embedding;
use collections::HashMap;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, sea_query::Expr};
use std::ops::Sub;
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

// SQLite does not support array arguments, so we only test this against a real postgres instance
#[gpui::test]
async fn test_get_embeddings_postgres(cx: &mut gpui::TestAppContext) {
    let test_db = TestDb::postgres(cx.executor());
    let db = test_db.db();

    let provider = "test_model";
    let digest1 = vec![1, 2, 3];
    let digest2 = vec![4, 5, 6];
    let embeddings = HashMap::from_iter([
        (digest1.clone(), vec![0.1, 0.2, 0.3]),
        (digest2.clone(), vec![0.4, 0.5, 0.6]),
    ]);

    // Save embeddings
    db.save_embeddings(provider, &embeddings).await.unwrap();

    // Retrieve embeddings
    let retrieved_embeddings = db
        .get_embeddings(provider, &[digest1.clone(), digest2.clone()])
        .await
        .unwrap();
    assert_eq!(retrieved_embeddings.len(), 2);
    assert!(retrieved_embeddings.contains_key(&digest1));
    assert!(retrieved_embeddings.contains_key(&digest2));

    // Check if the retrieved embeddings are correct
    assert_eq!(retrieved_embeddings[&digest1], vec![0.1, 0.2, 0.3]);
    assert_eq!(retrieved_embeddings[&digest2], vec![0.4, 0.5, 0.6]);
}

#[gpui::test]
async fn test_purge_old_embeddings(cx: &mut gpui::TestAppContext) {
    let test_db = TestDb::postgres(cx.executor());
    let db = test_db.db();

    let model = "test_model";
    let digest = vec![7, 8, 9];
    let embeddings = HashMap::from_iter([(digest.clone(), vec![0.7, 0.8, 0.9])]);

    // Save old embeddings
    db.save_embeddings(model, &embeddings).await.unwrap();

    // Reach into the DB and change the retrieved at to be > 60 days
    db.transaction(|tx| {
        let digest = digest.clone();
        async move {
            let sixty_days_ago = OffsetDateTime::now_utc().sub(Duration::days(61));
            let retrieved_at = PrimitiveDateTime::new(sixty_days_ago.date(), sixty_days_ago.time());

            embedding::Entity::update_many()
                .filter(
                    embedding::Column::Model
                        .eq(model)
                        .and(embedding::Column::Digest.eq(digest)),
                )
                .col_expr(embedding::Column::RetrievedAt, Expr::value(retrieved_at))
                .exec(&*tx)
                .await
                .unwrap();

            Ok(())
        }
    })
    .await
    .unwrap();

    // Purge old embeddings
    db.purge_old_embeddings().await.unwrap();

    // Try to retrieve the purged embeddings
    let retrieved_embeddings = db
        .get_embeddings(model, std::slice::from_ref(&digest))
        .await
        .unwrap();
    assert!(
        retrieved_embeddings.is_empty(),
        "Old embeddings should have been purged"
    );
}
