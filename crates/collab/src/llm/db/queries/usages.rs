use chrono::Duration;
use rpc::LanguageModelProvider;
use std::str::FromStr;
use strum::IntoEnumIterator as _;

use super::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Usage {
    pub requests_this_minute: usize,
    pub tokens_this_minute: usize,
    pub tokens_this_day: usize,
    pub tokens_this_month: usize,
}

const MINUTE_BUCKET_COUNT: usize = 6;
const MINUTE_BUCKET_DURATION: Duration = Duration::seconds(60 / MINUTE_BUCKET_COUNT as i64);

const DAY_BUCKET_COUNT: usize = 24;
const DAY_BUCKET_DURATION: Duration = Duration::minutes(24 * 60 / DAY_BUCKET_COUNT as i64);

const MONTH_BUCKET_COUNT: usize = 30;
const MONTH_BUCKET_DURATION: Duration = Duration::hours(24);

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

    pub async fn get_usage(
        &self,
        user_id: i32,
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

            let requests_this_minute = self.get_usage_for_measure(
                &usages,
                now,
                UsageMeasure::RequestsPerMinute,
                MINUTE_BUCKET_DURATION,
            )?;
            let tokens_this_minute = self.get_usage_for_measure(
                &usages,
                now,
                UsageMeasure::TokensPerMinute,
                MINUTE_BUCKET_DURATION,
            )?;
            let tokens_this_day = self.get_usage_for_measure(
                &usages,
                now,
                UsageMeasure::TokensPerDay,
                DAY_BUCKET_DURATION,
            )?;
            let tokens_this_month = self.get_usage_for_measure(
                &usages,
                now,
                UsageMeasure::TokensPerMonth,
                MONTH_BUCKET_DURATION,
            )?;

            Ok(Usage {
                requests_this_minute,
                tokens_this_minute,
                tokens_this_day,
                tokens_this_month,
            })
        })
        .await
    }

    pub async fn record_usage(
        &self,
        user_id: i32,
        provider: LanguageModelProvider,
        model_name: &str,
        token_count: usize,
        now: DateTimeUtc,
    ) -> Result<()> {
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

            self.update_usage_for_measure(
                user_id,
                model.id,
                &usages,
                UsageMeasure::RequestsPerMinute,
                now,
                1,
                MINUTE_BUCKET_COUNT,
                MINUTE_BUCKET_DURATION,
                &tx,
            )
            .await?;
            self.update_usage_for_measure(
                user_id,
                model.id,
                &usages,
                UsageMeasure::TokensPerMinute,
                now,
                token_count,
                MINUTE_BUCKET_COUNT,
                MINUTE_BUCKET_DURATION,
                &tx,
            )
            .await?;
            self.update_usage_for_measure(
                user_id,
                model.id,
                &usages,
                UsageMeasure::TokensPerDay,
                now,
                token_count,
                DAY_BUCKET_COUNT,
                DAY_BUCKET_DURATION,
                &tx,
            )
            .await?;
            self.update_usage_for_measure(
                user_id,
                model.id,
                &usages,
                UsageMeasure::TokensPerMonth,
                now,
                token_count,
                MONTH_BUCKET_COUNT,
                MONTH_BUCKET_DURATION,
                &tx,
            )
            .await?;

            Ok(())
        })
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn update_usage_for_measure(
        &self,
        user_id: i32,
        model_id: ModelId,
        usages: &[usage::Model],
        usage_measure: UsageMeasure,
        now: DateTimeUtc,
        count: usize,
        bucket_count: usize,
        bucket_duration: Duration,
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        let now = now.naive_utc();
        let measure_id = self.usage_measure_ids[&usage_measure];

        let mut new_timestamp = now;
        let mut new_buckets = vec![0; bucket_count];
        let mut existing_id = None;

        if let Some(usage) = usages.iter().find(|usage| usage.measure_id == measure_id) {
            existing_id = Some(usage.id);
            let time_delta = now - usage.timestamp;
            if time_delta < bucket_duration * bucket_count as i32 {
                let num_buckets_advanced = (time_delta.num_seconds() as f32
                    / bucket_duration.num_seconds() as f32)
                    .ceil() as usize;
                new_timestamp = usage.timestamp + bucket_duration * num_buckets_advanced as i32;

                for (bucket_ix, bucket) in new_buckets.iter_mut().enumerate() {
                    *bucket = usage
                        .buckets
                        .get(bucket_ix + num_buckets_advanced)
                        .copied()
                        .unwrap_or(0);
                }
            }
        }

        *new_buckets.last_mut().unwrap() += count as i64;

        let mut model = usage::ActiveModel {
            user_id: ActiveValue::set(user_id),
            model_id: ActiveValue::set(model_id),
            measure_id: ActiveValue::set(measure_id),
            timestamp: ActiveValue::set(new_timestamp),
            buckets: ActiveValue::set(new_buckets),
            ..Default::default()
        };

        if let Some(id) = existing_id {
            model.id = ActiveValue::unchanged(id);
            model.update(tx).await?;
        } else {
            usage::Entity::insert(model)
                .exec_without_returning(tx)
                .await?;
        }

        Ok(())
    }

    fn get_usage_for_measure(
        &self,
        usages: &[usage::Model],
        now: DateTimeUtc,
        measure: UsageMeasure,
        bucket_duration: Duration,
    ) -> Result<usize> {
        let measure_id = self
            .usage_measure_ids
            .get(&measure)
            .ok_or_else(|| anyhow!("usage measure {measure} not found"))?;
        let Some(usage) = usages.iter().find(|usage| usage.measure_id == *measure_id) else {
            return Ok(0);
        };

        let now = now.naive_utc();
        let buckets_elapsed =
            (now - usage.timestamp).num_seconds().max(0) / bucket_duration.num_seconds();
        Ok(usage
            .buckets
            .iter()
            .skip(buckets_elapsed as usize)
            .sum::<i64>() as _)
    }
}
