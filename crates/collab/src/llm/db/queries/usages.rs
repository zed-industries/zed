use crate::db::UserId;
use crate::llm::Cents;
use chrono::{Datelike, Duration};
use futures::StreamExt as _;
use rpc::LanguageModelProvider;
use sea_orm::QuerySelect;
use std::{iter, str::FromStr};
use strum::IntoEnumIterator as _;

use super::*;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct TokenUsage {
    pub input: usize,
    pub input_cache_creation: usize,
    pub input_cache_read: usize,
    pub output: usize,
}

impl TokenUsage {
    pub fn total(&self) -> usize {
        self.input + self.input_cache_creation + self.input_cache_read + self.output
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Usage {
    pub requests_this_minute: usize,
    pub tokens_this_minute: usize,
    pub tokens_this_day: usize,
    pub tokens_this_month: TokenUsage,
    pub spending_this_month: Cents,
    pub lifetime_spending: Cents,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ApplicationWideUsage {
    pub provider: LanguageModelProvider,
    pub model: String,
    pub requests_this_minute: usize,
    pub tokens_this_minute: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ActiveUserCount {
    pub users_in_recent_minutes: usize,
    pub users_in_recent_days: usize,
}

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

    pub async fn get_application_wide_usages_by_model(
        &self,
        now: DateTimeUtc,
    ) -> Result<Vec<ApplicationWideUsage>> {
        self.transaction(|tx| async move {
            let past_minute = now - Duration::minutes(1);
            let requests_per_minute = self.usage_measure_ids[&UsageMeasure::RequestsPerMinute];
            let tokens_per_minute = self.usage_measure_ids[&UsageMeasure::TokensPerMinute];

            let mut results = Vec::new();
            for ((provider, model_name), model) in self.models.iter() {
                let mut usages = usage::Entity::find()
                    .filter(
                        usage::Column::Timestamp
                            .gte(past_minute.naive_utc())
                            .and(usage::Column::IsStaff.eq(false))
                            .and(usage::Column::ModelId.eq(model.id))
                            .and(
                                usage::Column::MeasureId
                                    .eq(requests_per_minute)
                                    .or(usage::Column::MeasureId.eq(tokens_per_minute)),
                            ),
                    )
                    .stream(&*tx)
                    .await?;

                let mut requests_this_minute = 0;
                let mut tokens_this_minute = 0;
                while let Some(usage) = usages.next().await {
                    let usage = usage?;
                    if usage.measure_id == requests_per_minute {
                        requests_this_minute += Self::get_live_buckets(
                            &usage,
                            now.naive_utc(),
                            UsageMeasure::RequestsPerMinute,
                        )
                        .0
                        .iter()
                        .copied()
                        .sum::<i64>() as usize;
                    } else if usage.measure_id == tokens_per_minute {
                        tokens_this_minute += Self::get_live_buckets(
                            &usage,
                            now.naive_utc(),
                            UsageMeasure::TokensPerMinute,
                        )
                        .0
                        .iter()
                        .copied()
                        .sum::<i64>() as usize;
                    }
                }

                results.push(ApplicationWideUsage {
                    provider: *provider,
                    model: model_name.clone(),
                    requests_this_minute,
                    tokens_this_minute,
                })
            }

            Ok(results)
        })
        .await
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

    pub async fn get_usage(
        &self,
        user_id: UserId,
        provider: LanguageModelProvider,
        model_name: &str,
        now: DateTimeUtc,
    ) -> Result<Usage> {
        self.transaction(|tx| async move {
            let model = self
                .models
                .get(&(provider, model_name.to_string()))
                .ok_or_else(|| anyhow!("unknown model {provider}:{model_name}"))?;

            let usages = usage::Entity::find()
                .filter(
                    usage::Column::UserId
                        .eq(user_id)
                        .and(usage::Column::ModelId.eq(model.id)),
                )
                .all(&*tx)
                .await?;

            let month = now.date_naive().month() as i32;
            let year = now.date_naive().year();
            let monthly_usage = monthly_usage::Entity::find()
                .filter(
                    monthly_usage::Column::UserId
                        .eq(user_id)
                        .and(monthly_usage::Column::ModelId.eq(model.id))
                        .and(monthly_usage::Column::Month.eq(month))
                        .and(monthly_usage::Column::Year.eq(year)),
                )
                .one(&*tx)
                .await?;
            let lifetime_usage = lifetime_usage::Entity::find()
                .filter(
                    lifetime_usage::Column::UserId
                        .eq(user_id)
                        .and(lifetime_usage::Column::ModelId.eq(model.id)),
                )
                .one(&*tx)
                .await?;

            let requests_this_minute =
                self.get_usage_for_measure(&usages, now, UsageMeasure::RequestsPerMinute)?;
            let tokens_this_minute =
                self.get_usage_for_measure(&usages, now, UsageMeasure::TokensPerMinute)?;
            let tokens_this_day =
                self.get_usage_for_measure(&usages, now, UsageMeasure::TokensPerDay)?;
            let spending_this_month = if let Some(monthly_usage) = &monthly_usage {
                calculate_spending(
                    model,
                    monthly_usage.input_tokens as usize,
                    monthly_usage.cache_creation_input_tokens as usize,
                    monthly_usage.cache_read_input_tokens as usize,
                    monthly_usage.output_tokens as usize,
                )
            } else {
                Cents::ZERO
            };
            let lifetime_spending = if let Some(lifetime_usage) = &lifetime_usage {
                calculate_spending(
                    model,
                    lifetime_usage.input_tokens as usize,
                    lifetime_usage.cache_creation_input_tokens as usize,
                    lifetime_usage.cache_read_input_tokens as usize,
                    lifetime_usage.output_tokens as usize,
                )
            } else {
                Cents::ZERO
            };

            Ok(Usage {
                requests_this_minute,
                tokens_this_minute,
                tokens_this_day,
                tokens_this_month: TokenUsage {
                    input: monthly_usage
                        .as_ref()
                        .map_or(0, |usage| usage.input_tokens as usize),
                    input_cache_creation: monthly_usage
                        .as_ref()
                        .map_or(0, |usage| usage.cache_creation_input_tokens as usize),
                    input_cache_read: monthly_usage
                        .as_ref()
                        .map_or(0, |usage| usage.cache_read_input_tokens as usize),
                    output: monthly_usage
                        .as_ref()
                        .map_or(0, |usage| usage.output_tokens as usize),
                },
                spending_this_month,
                lifetime_spending,
            })
        })
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn record_usage(
        &self,
        user_id: UserId,
        is_staff: bool,
        provider: LanguageModelProvider,
        model_name: &str,
        tokens: TokenUsage,
        has_llm_subscription: bool,
        max_monthly_spend: Cents,
        free_tier_monthly_spending_limit: Cents,
        now: DateTimeUtc,
    ) -> Result<Usage> {
        self.transaction(|tx| async move {
            let model = self.model(provider, model_name)?;

            let usages = usage::Entity::find()
                .filter(
                    usage::Column::UserId
                        .eq(user_id)
                        .and(usage::Column::ModelId.eq(model.id)),
                )
                .all(&*tx)
                .await?;

            let requests_this_minute = self
                .update_usage_for_measure(
                    user_id,
                    is_staff,
                    model.id,
                    &usages,
                    UsageMeasure::RequestsPerMinute,
                    now,
                    1,
                    &tx,
                )
                .await?;
            let tokens_this_minute = self
                .update_usage_for_measure(
                    user_id,
                    is_staff,
                    model.id,
                    &usages,
                    UsageMeasure::TokensPerMinute,
                    now,
                    tokens.total(),
                    &tx,
                )
                .await?;
            let tokens_this_day = self
                .update_usage_for_measure(
                    user_id,
                    is_staff,
                    model.id,
                    &usages,
                    UsageMeasure::TokensPerDay,
                    now,
                    tokens.total(),
                    &tx,
                )
                .await?;

            let month = now.date_naive().month() as i32;
            let year = now.date_naive().year();

            // Update monthly usage
            let monthly_usage = monthly_usage::Entity::find()
                .filter(
                    monthly_usage::Column::UserId
                        .eq(user_id)
                        .and(monthly_usage::Column::ModelId.eq(model.id))
                        .and(monthly_usage::Column::Month.eq(month))
                        .and(monthly_usage::Column::Year.eq(year)),
                )
                .one(&*tx)
                .await?;

            let monthly_usage = match monthly_usage {
                Some(usage) => {
                    monthly_usage::Entity::update(monthly_usage::ActiveModel {
                        id: ActiveValue::unchanged(usage.id),
                        input_tokens: ActiveValue::set(usage.input_tokens + tokens.input as i64),
                        cache_creation_input_tokens: ActiveValue::set(
                            usage.cache_creation_input_tokens + tokens.input_cache_creation as i64,
                        ),
                        cache_read_input_tokens: ActiveValue::set(
                            usage.cache_read_input_tokens + tokens.input_cache_read as i64,
                        ),
                        output_tokens: ActiveValue::set(usage.output_tokens + tokens.output as i64),
                        ..Default::default()
                    })
                    .exec(&*tx)
                    .await?
                }
                None => {
                    monthly_usage::ActiveModel {
                        user_id: ActiveValue::set(user_id),
                        model_id: ActiveValue::set(model.id),
                        month: ActiveValue::set(month),
                        year: ActiveValue::set(year),
                        input_tokens: ActiveValue::set(tokens.input as i64),
                        cache_creation_input_tokens: ActiveValue::set(
                            tokens.input_cache_creation as i64,
                        ),
                        cache_read_input_tokens: ActiveValue::set(tokens.input_cache_read as i64),
                        output_tokens: ActiveValue::set(tokens.output as i64),
                        ..Default::default()
                    }
                    .insert(&*tx)
                    .await?
                }
            };

            let spending_this_month = calculate_spending(
                model,
                monthly_usage.input_tokens as usize,
                monthly_usage.cache_creation_input_tokens as usize,
                monthly_usage.cache_read_input_tokens as usize,
                monthly_usage.output_tokens as usize,
            );

            if !is_staff
                && spending_this_month > free_tier_monthly_spending_limit
                && has_llm_subscription
                && (spending_this_month - free_tier_monthly_spending_limit) <= max_monthly_spend
            {
                billing_event::ActiveModel {
                    id: ActiveValue::not_set(),
                    idempotency_key: ActiveValue::not_set(),
                    user_id: ActiveValue::set(user_id),
                    model_id: ActiveValue::set(model.id),
                    input_tokens: ActiveValue::set(tokens.input as i64),
                    input_cache_creation_tokens: ActiveValue::set(
                        tokens.input_cache_creation as i64,
                    ),
                    input_cache_read_tokens: ActiveValue::set(tokens.input_cache_read as i64),
                    output_tokens: ActiveValue::set(tokens.output as i64),
                }
                .insert(&*tx)
                .await?;
            }

            // Update lifetime usage
            let lifetime_usage = lifetime_usage::Entity::find()
                .filter(
                    lifetime_usage::Column::UserId
                        .eq(user_id)
                        .and(lifetime_usage::Column::ModelId.eq(model.id)),
                )
                .one(&*tx)
                .await?;

            let lifetime_usage = match lifetime_usage {
                Some(usage) => {
                    lifetime_usage::Entity::update(lifetime_usage::ActiveModel {
                        id: ActiveValue::unchanged(usage.id),
                        input_tokens: ActiveValue::set(usage.input_tokens + tokens.input as i64),
                        cache_creation_input_tokens: ActiveValue::set(
                            usage.cache_creation_input_tokens + tokens.input_cache_creation as i64,
                        ),
                        cache_read_input_tokens: ActiveValue::set(
                            usage.cache_read_input_tokens + tokens.input_cache_read as i64,
                        ),
                        output_tokens: ActiveValue::set(usage.output_tokens + tokens.output as i64),
                        ..Default::default()
                    })
                    .exec(&*tx)
                    .await?
                }
                None => {
                    lifetime_usage::ActiveModel {
                        user_id: ActiveValue::set(user_id),
                        model_id: ActiveValue::set(model.id),
                        input_tokens: ActiveValue::set(tokens.input as i64),
                        cache_creation_input_tokens: ActiveValue::set(
                            tokens.input_cache_creation as i64,
                        ),
                        cache_read_input_tokens: ActiveValue::set(tokens.input_cache_read as i64),
                        output_tokens: ActiveValue::set(tokens.output as i64),
                        ..Default::default()
                    }
                    .insert(&*tx)
                    .await?
                }
            };

            let lifetime_spending = calculate_spending(
                model,
                lifetime_usage.input_tokens as usize,
                lifetime_usage.cache_creation_input_tokens as usize,
                lifetime_usage.cache_read_input_tokens as usize,
                lifetime_usage.output_tokens as usize,
            );

            Ok(Usage {
                requests_this_minute,
                tokens_this_minute,
                tokens_this_day,
                tokens_this_month: TokenUsage {
                    input: monthly_usage.input_tokens as usize,
                    input_cache_creation: monthly_usage.cache_creation_input_tokens as usize,
                    input_cache_read: monthly_usage.cache_read_input_tokens as usize,
                    output: monthly_usage.output_tokens as usize,
                },
                spending_this_month,
                lifetime_spending,
            })
        })
        .await
    }

    /// Returns the active user count for the specified model.
    pub async fn get_active_user_count(
        &self,
        provider: LanguageModelProvider,
        model_name: &str,
        now: DateTimeUtc,
    ) -> Result<ActiveUserCount> {
        self.transaction(|tx| async move {
            let minute_since = now - Duration::minutes(5);
            let day_since = now - Duration::days(5);

            let model = self
                .models
                .get(&(provider, model_name.to_string()))
                .ok_or_else(|| anyhow!("unknown model {provider}:{model_name}"))?;

            let tokens_per_minute = self.usage_measure_ids[&UsageMeasure::TokensPerMinute];

            let users_in_recent_minutes = usage::Entity::find()
                .filter(
                    usage::Column::ModelId
                        .eq(model.id)
                        .and(usage::Column::MeasureId.eq(tokens_per_minute))
                        .and(usage::Column::Timestamp.gte(minute_since.naive_utc()))
                        .and(usage::Column::IsStaff.eq(false)),
                )
                .select_only()
                .column(usage::Column::UserId)
                .group_by(usage::Column::UserId)
                .count(&*tx)
                .await? as usize;

            let users_in_recent_days = usage::Entity::find()
                .filter(
                    usage::Column::ModelId
                        .eq(model.id)
                        .and(usage::Column::MeasureId.eq(tokens_per_minute))
                        .and(usage::Column::Timestamp.gte(day_since.naive_utc()))
                        .and(usage::Column::IsStaff.eq(false)),
                )
                .select_only()
                .column(usage::Column::UserId)
                .group_by(usage::Column::UserId)
                .count(&*tx)
                .await? as usize;

            Ok(ActiveUserCount {
                users_in_recent_minutes,
                users_in_recent_days,
            })
        })
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn update_usage_for_measure(
        &self,
        user_id: UserId,
        is_staff: bool,
        model_id: ModelId,
        usages: &[usage::Model],
        usage_measure: UsageMeasure,
        now: DateTimeUtc,
        usage_to_add: usize,
        tx: &DatabaseTransaction,
    ) -> Result<usize> {
        let now = now.naive_utc();
        let measure_id = *self
            .usage_measure_ids
            .get(&usage_measure)
            .ok_or_else(|| anyhow!("usage measure {usage_measure} not found"))?;

        let mut id = None;
        let mut timestamp = now;
        let mut buckets = vec![0_i64];

        if let Some(old_usage) = usages.iter().find(|usage| usage.measure_id == measure_id) {
            id = Some(old_usage.id);
            let (live_buckets, buckets_since) =
                Self::get_live_buckets(old_usage, now, usage_measure);
            if !live_buckets.is_empty() {
                buckets.clear();
                buckets.extend_from_slice(live_buckets);
                buckets.extend(iter::repeat(0).take(buckets_since));
                timestamp =
                    old_usage.timestamp + (usage_measure.bucket_duration() * buckets_since as i32);
            }
        }

        *buckets.last_mut().unwrap() += usage_to_add as i64;
        let total_usage = buckets.iter().sum::<i64>() as usize;

        let mut model = usage::ActiveModel {
            user_id: ActiveValue::set(user_id),
            is_staff: ActiveValue::set(is_staff),
            model_id: ActiveValue::set(model_id),
            measure_id: ActiveValue::set(measure_id),
            timestamp: ActiveValue::set(timestamp),
            buckets: ActiveValue::set(buckets),
            ..Default::default()
        };

        if let Some(id) = id {
            model.id = ActiveValue::unchanged(id);
            model.update(tx).await?;
        } else {
            usage::Entity::insert(model)
                .exec_without_returning(tx)
                .await?;
        }

        Ok(total_usage)
    }

    fn get_usage_for_measure(
        &self,
        usages: &[usage::Model],
        now: DateTimeUtc,
        usage_measure: UsageMeasure,
    ) -> Result<usize> {
        let now = now.naive_utc();
        let measure_id = *self
            .usage_measure_ids
            .get(&usage_measure)
            .ok_or_else(|| anyhow!("usage measure {usage_measure} not found"))?;
        let Some(usage) = usages.iter().find(|usage| usage.measure_id == measure_id) else {
            return Ok(0);
        };

        let (live_buckets, _) = Self::get_live_buckets(usage, now, usage_measure);
        Ok(live_buckets.iter().sum::<i64>() as _)
    }

    fn get_live_buckets(
        usage: &usage::Model,
        now: chrono::NaiveDateTime,
        measure: UsageMeasure,
    ) -> (&[i64], usize) {
        let seconds_since_usage = (now - usage.timestamp).num_seconds().max(0);
        let buckets_since_usage =
            seconds_since_usage as f32 / measure.bucket_duration().num_seconds() as f32;
        let buckets_since_usage = buckets_since_usage.ceil() as usize;
        let mut live_buckets = &[] as &[i64];
        if buckets_since_usage < measure.bucket_count() {
            let expired_bucket_count =
                (usage.buckets.len() + buckets_since_usage).saturating_sub(measure.bucket_count());
            live_buckets = &usage.buckets[expired_bucket_count..];
            while live_buckets.first() == Some(&0) {
                live_buckets = &live_buckets[1..];
            }
        }
        (live_buckets, buckets_since_usage)
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

const MINUTE_BUCKET_COUNT: usize = 12;
const DAY_BUCKET_COUNT: usize = 48;

impl UsageMeasure {
    fn bucket_count(&self) -> usize {
        match self {
            UsageMeasure::RequestsPerMinute => MINUTE_BUCKET_COUNT,
            UsageMeasure::TokensPerMinute => MINUTE_BUCKET_COUNT,
            UsageMeasure::TokensPerDay => DAY_BUCKET_COUNT,
        }
    }

    fn total_duration(&self) -> Duration {
        match self {
            UsageMeasure::RequestsPerMinute => Duration::minutes(1),
            UsageMeasure::TokensPerMinute => Duration::minutes(1),
            UsageMeasure::TokensPerDay => Duration::hours(24),
        }
    }

    fn bucket_duration(&self) -> Duration {
        self.total_duration() / self.bucket_count() as i32
    }
}
