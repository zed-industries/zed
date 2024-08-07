use rpc::LanguageModelProvider;

use super::*;

impl LlmDatabase {
    pub async fn find_or_create_usage(
        &self,
        user_id: i32,
        provider: LanguageModelProvider,
        model_name: &str,
    ) -> Result<usage::Model> {
        self.transaction(|tx| async move {
            let provider_name = match provider {
                LanguageModelProvider::Anthropic => "anthropic",
                LanguageModelProvider::OpenAi => "open_ai",
                LanguageModelProvider::Google => "google",
                LanguageModelProvider::Zed => "zed",
            };

            let model = model::Entity::find()
                .inner_join(provider::Entity)
                .filter(
                    provider::Column::Name
                        .eq(provider_name)
                        .and(model::Column::Name.eq(model_name)),
                )
                .one(&*tx)
                .await?
                // TODO: Create the model, if one doesn't exist.
                .ok_or_else(|| anyhow!("no model found for {provider_name}:{model_name}"))?;
            let model_id = model.id;

            let existing_usage = usage::Entity::find()
                .filter(
                    usage::Column::UserId
                        .eq(user_id)
                        .and(usage::Column::ModelId.eq(model_id)),
                )
                .one(&*tx)
                .await?;
            if let Some(usage) = existing_usage {
                return Ok(usage);
            }

            let usage = usage::Entity::insert(usage::ActiveModel {
                user_id: ActiveValue::set(user_id),
                model_id: ActiveValue::set(model_id),
                ..Default::default()
            })
            .exec_with_returning(&*tx)
            .await?;

            Ok(usage)
        })
        .await
    }
}
