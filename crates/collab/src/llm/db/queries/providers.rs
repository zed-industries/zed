use std::str::FromStr;

use sea_orm::QueryOrder;
use strum::IntoEnumIterator as _;

use super::*;

impl LlmDatabase {
    pub async fn initialize_providers(&mut self) -> Result<()> {
        let (all_providers, all_models) = self
            .transaction(|tx| async move {
                let existing_providers = provider::Entity::find().all(&*tx).await?;

                let mut new_providers = LanguageModelProvider::iter()
                    .filter(|provider| {
                        !existing_providers
                            .iter()
                            .any(|p| p.name == provider.to_string())
                    })
                    .map(|provider| provider::ActiveModel {
                        name: ActiveValue::set(provider.to_string()),
                        ..Default::default()
                    })
                    .peekable();

                if new_providers.peek().is_some() {
                    provider::Entity::insert_many(new_providers)
                        .exec(&*tx)
                        .await?;
                }

                let all_providers: HashMap<_, _> = provider::Entity::find()
                    .all(&*tx)
                    .await?
                    .iter()
                    .filter_map(|provider| {
                        LanguageModelProvider::from_str(&provider.name)
                            .ok()
                            .map(|p| (p, provider.id))
                    })
                    .collect();

                let all_models: HashMap<_, _> = model::Entity::find()
                    .all(&*tx)
                    .await?
                    .into_iter()
                    .filter_map(|model| {
                        let provider = all_providers.iter().find_map(|(provider, id)| {
                            if *id == model.provider_id {
                                Some(provider)
                            } else {
                                None
                            }
                        })?;
                        Some(((*provider, model.name.clone()), model))
                    })
                    .collect();

                Ok((all_providers, all_models))
            })
            .await?;

        self.provider_ids = all_providers;
        self.models = all_models;

        Ok(())
    }

    /// Returns the list of LLM providers.
    pub async fn list_providers(&self) -> Result<Vec<LanguageModelProvider>> {
        self.transaction(|tx| async move {
            Ok(provider::Entity::find()
                .order_by_asc(provider::Column::Name)
                .all(&*tx)
                .await?
                .into_iter()
                .filter_map(|p| LanguageModelProvider::from_str(&p.name).ok())
                .collect())
        })
        .await
    }
}
