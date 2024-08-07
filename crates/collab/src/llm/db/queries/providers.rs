use std::str::FromStr;

use sea_orm::QueryOrder;
use strum::IntoEnumIterator as _;

use super::*;

const KNOWN_MODELS: &[(LanguageModelProvider, &str)] = &[
    (LanguageModelProvider::Anthropic, "claude-3-5-sonnet"),
    (LanguageModelProvider::Anthropic, "claude-3-opus"),
    (LanguageModelProvider::Anthropic, "claude-3-sonnet"),
    (LanguageModelProvider::Anthropic, "claude-3-haiku"),
    (LanguageModelProvider::OpenAi, "gpt-3.5-turbo"),
    (LanguageModelProvider::OpenAi, "gpt-4"),
    (LanguageModelProvider::OpenAi, "gpt-4-turbo-preview"),
    (LanguageModelProvider::OpenAi, "gpt-4o"),
    (LanguageModelProvider::OpenAi, "gpt-4o-mini"),
];

impl LlmDatabase {
    pub async fn initialize_providers(&mut self) -> Result<()> {
        let (all_providers, all_models) = self
            .transaction(|tx| async move {
                let existing_providers = provider::Entity::find().all(&*tx).await?;

                let new_providers = LanguageModelProvider::iter()
                    .filter(|provider| {
                        !existing_providers
                            .iter()
                            .any(|p| p.name == provider.to_string())
                    })
                    .map(|provider| provider::ActiveModel {
                        name: ActiveValue::set(provider.to_string()),
                        ..Default::default()
                    });

                provider::Entity::insert_many(new_providers)
                    .exec(&*tx)
                    .await?;

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

                let existing_models = model::Entity::find().all(&*tx).await?;

                let new_models = KNOWN_MODELS.iter().filter_map(|(provider, model_name)| {
                    let provider_id = all_providers.get(provider)?;
                    if !existing_models
                        .iter()
                        .any(|m| m.name == *model_name && m.provider_id == *provider_id)
                    {
                        Some(model::ActiveModel {
                            provider_id: ActiveValue::set(*provider_id),
                            name: ActiveValue::set(model_name.to_string()),
                            ..Default::default()
                        })
                    } else {
                        None
                    }
                });

                model::Entity::insert_many(new_models).exec(&*tx).await?;

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
                        Some(((*provider, model.name), model.id))
                    })
                    .collect();

                Ok((all_providers, all_models))
            })
            .await?;

        self.provider_ids = all_providers;
        self.model_ids = all_models;

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
