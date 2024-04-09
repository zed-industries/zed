use super::*;

impl Database {
    pub async fn get_embeddings(
        &self,
        provider: &str,
        digests: &[Vec<u8>],
    ) -> Result<HashMap<Vec<u8>, Vec<f32>>> {
        self.weak_transaction(|tx| async move {
            let mut db_embeddings = embedding::Entity::find()
                .filter(embedding::Column::Provider.eq(provider).and(
                    embedding::Column::Digest.is_in(digests.iter().map(|digest| digest.as_slice())),
                ))
                .stream(&*tx)
                .await?;

            let mut embeddings = HashMap::default();
            while let Some(db_embedding) = db_embeddings.next().await {
                let db_embedding = db_embedding?;
                embeddings.insert(db_embedding.digest, db_embedding.dimensions);
            }
            Ok(embeddings)
        })
        .await
    }

    pub async fn save_embeddings(
        &self,
        provider: &str,
        embeddings: &HashMap<Vec<u8>, Vec<f32>>,
    ) -> Result<()> {
        self.weak_transaction(|tx| async move {
            embedding::Entity::insert_many(embeddings.iter().map(|(digest, dimensions)| {
                embedding::ActiveModel {
                    provider: ActiveValue::set(provider.to_string()),
                    digest: ActiveValue::set(digest.clone()),
                    dimensions: ActiveValue::set(dimensions.clone()),
                    retrieved_at: ActiveValue::set(OffsetDateTime::now()),
                }
            }))
            .on_conflict(
                OnConflict::columns([embedding::Column::Provider, embedding::Column::Digest])
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
        self.weak_transaction(|tx| async move {
            embedding::Entity::delete_many().filter(embedding::Column::RetrievedAt.lte())
        })
        .await
    }
}
