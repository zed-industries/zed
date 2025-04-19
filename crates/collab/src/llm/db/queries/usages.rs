use crate::db::UserId;
use crate::llm::Cents;
use chrono::Datelike;
use futures::StreamExt as _;
use std::str::FromStr;
use strum::IntoEnumIterator as _;

use super::*;

impl LlmDatabase {
    pub async fn initialize_usage_measures(&mut self) -> Result<()> {
        let all_measures = self
            .transaction(|tx| async move {
                let existing_measures = usage_measure::Entity::find().all(&*tx).await?;

                let new_measures = UsageMeasure::iter()
                    .filter(|measure| {
                        !existing_measures
                            .iter()
                            .any(|m| m.name == measure.to_string())
                    })
                    .map(|measure| usage_measure::ActiveModel {
                        name: ActiveValue::set(measure.to_string()),
                        ..Default::default()
                    })
                    .collect::<Vec<_>>();

                if !new_measures.is_empty() {
                    usage_measure::Entity::insert_many(new_measures)
                        .exec(&*tx)
                        .await?;
                }

                Ok(usage_measure::Entity::find().all(&*tx).await?)
            })
            .await?;

        self.usage_measure_ids = all_measures
            .into_iter()
            .filter_map(|measure| {
                UsageMeasure::from_str(&measure.name)
                    .ok()
                    .map(|um| (um, measure.id))
            })
            .collect();
        Ok(())
    }

    pub async fn get_user_spending_for_month(
        &self,
        user_id: UserId,
        now: DateTimeUtc,
    ) -> Result<Cents> {
        self.transaction(|tx| async move {
            let month = now.date_naive().month() as i32;
            let year = now.date_naive().year();

            let mut monthly_usages = monthly_usage::Entity::find()
                .filter(
                    monthly_usage::Column::UserId
                        .eq(user_id)
                        .and(monthly_usage::Column::Month.eq(month))
                        .and(monthly_usage::Column::Year.eq(year)),
                )
                .stream(&*tx)
                .await?;
            let mut monthly_spending = Cents::ZERO;

            while let Some(usage) = monthly_usages.next().await {
                let usage = usage?;
                let Ok(model) = self.model_by_id(usage.model_id) else {
                    continue;
                };

                monthly_spending += calculate_spending(
                    model,
                    usage.input_tokens as usize,
                    usage.cache_creation_input_tokens as usize,
                    usage.cache_read_input_tokens as usize,
                    usage.output_tokens as usize,
                );
            }

            Ok(monthly_spending)
        })
        .await
    }
}

fn calculate_spending(
    model: &model::Model,
    input_tokens_this_month: usize,
    cache_creation_input_tokens_this_month: usize,
    cache_read_input_tokens_this_month: usize,
    output_tokens_this_month: usize,
) -> Cents {
    let input_token_cost =
        input_tokens_this_month * model.price_per_million_input_tokens as usize / 1_000_000;
    let cache_creation_input_token_cost = cache_creation_input_tokens_this_month
        * model.price_per_million_cache_creation_input_tokens as usize
        / 1_000_000;
    let cache_read_input_token_cost = cache_read_input_tokens_this_month
        * model.price_per_million_cache_read_input_tokens as usize
        / 1_000_000;
    let output_token_cost =
        output_tokens_this_month * model.price_per_million_output_tokens as usize / 1_000_000;
    let spending = input_token_cost
        + cache_creation_input_token_cost
        + cache_read_input_token_cost
        + output_token_cost;
    Cents::new(spending as u32)
}
