use super::*;
use time::Duration;
use time::OffsetDateTime;

impl Database {
    pub async fn get_embeddings(
        &self,
        model: &str,
        digests: &[Vec<u8>],
    ) -> Result<HashMap<Vec<u8>, Vec<f32>>> {
        self.transaction(|tx| async move {
            let embeddings = {
                let mut db_embeddings = embedding::Entity::find()
                    .filter(
                        embedding::Column::Model.eq(model).and(
                            embedding::Column::Digest
                                .is_in(digests.iter().map(|digest| digest.as_slice())),
                        ),
                    )
                    .stream(&*tx)
                    .await?;

                let mut embeddings = HashMap::default();
                while let Some(db_embedding) = db_embeddings.next().await {
                    let db_embedding = db_embedding?;
                    embeddings.insert(db_embedding.digest, db_embedding.dimensions);
                }
                embeddings
            };

            if !embeddings.is_empty() {
                let now = OffsetDateTime::now_utc();
                let retrieved_at = PrimitiveDateTime::new(now.date(), now.time());

                embedding::Entity::update_many()
                    .filter(
                        embedding::Column::Digest
                            .is_in(embeddings.keys().map(|digest| digest.as_slice())),
                    )
                    .col_expr(embedding::Column::RetrievedAt, Expr::value(retrieved_at))
                    .exec(&*tx)
                    .await?;
            }

            Ok(embeddings)
        })
        .await
    }

    pub async fn save_embeddings(
        &self,
        model: &str,
        embeddings: &HashMap<Vec<u8>, Vec<f32>>,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            embedding::Entity::insert_many(embeddings.iter().map(|(digest, dimensions)| {
                let now_offset_datetime = OffsetDateTime::now_utc();
                let retrieved_at =
                    PrimitiveDateTime::new(now_offset_datetime.date(), now_offset_datetime.time());

                embedding::ActiveModel {
                    model: ActiveValue::set(model.to_string()),
                    digest: ActiveValue::set(digest.clone()),
                    dimensions: ActiveValue::set(dimensions.clone()),
                    retrieved_at: ActiveValue::set(retrieved_at),
                }
            }))
            .on_conflict(
                OnConflict::columns([embedding::Column::Model, embedding::Column::Digest])
                    .do_nothing()
                    .to_owned(),
            )
            .exec_without_returning(&*tx)
            .await?;
            Ok(())
        })
        .await
    }

    pub async fn purge_old_embeddings(&self) -> Result<()> {
        self.transaction(|tx| async move {
            embedding::Entity::delete_many()
                .filter(
                    embedding::Column::RetrievedAt
                        .lte(OffsetDateTime::now_utc() - Duration::days(60)),
                )
                .exec(&*tx)
                .await?;

            Ok(())
        })
        .await
    }
}
