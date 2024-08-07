use sea_orm::sea_query::OnConflict;
use sea_orm::QueryOrder;

use super::*;

impl LlmDatabase {
    pub async fn initialize_providers(&self) -> Result<()> {
        self.transaction(|tx| async move {
            let providers_and_models = vec![
                ("anthropic", "claude-3-5-sonnet"),
                ("anthropic", "claude-3-opus"),
                ("anthropic", "claude-3-sonnet"),
                ("anthropic", "claude-3-haiku"),
            ];

            for (provider_name, model_name) in providers_and_models {
                let insert_provider = provider::Entity::insert(provider::ActiveModel {
                    name: ActiveValue::set(provider_name.to_owned()),
                    ..Default::default()
                })
                .on_conflict(
                    OnConflict::columns([provider::Column::Name])
                        .update_column(provider::Column::Name)
                        .to_owned(),
                );

                let provider = if tx.support_returning() {
                    insert_provider.exec_with_returning(&*tx).await?
                } else {
                    insert_provider.exec_without_returning(&*tx).await?;
                    provider::Entity::find()
                        .filter(provider::Column::Name.eq(provider_name))
                        .one(&*tx)
                        .await?
                        .ok_or_else(|| anyhow!("failed to insert provider"))?
                };

                model::Entity::insert(model::ActiveModel {
                    provider_id: ActiveValue::set(provider.id),
                    name: ActiveValue::set(model_name.to_owned()),
                    ..Default::default()
                })
                .on_conflict(
                    OnConflict::columns([model::Column::ProviderId, model::Column::Name])
                        .update_column(model::Column::Name)
                        .to_owned(),
                )
                .exec_without_returning(&*tx)
                .await?;
            }

            Ok(())
        })
        .await
    }

    /// Returns the list of LLM providers.
    pub async fn list_providers(&self) -> Result<Vec<provider::Model>> {
        self.transaction(|tx| async move {
            Ok(provider::Entity::find()
                .order_by_asc(provider::Column::Name)
                .all(&*tx)
                .await?)
        })
        .await
    }
}
