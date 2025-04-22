use chrono::Timelike;
use time::PrimitiveDateTime;

use crate::db::billing_subscription::SubscriptionKind;
use crate::db::{UserId, billing_subscription};

use super::*;

fn convert_chrono_to_time(datetime: DateTimeUtc) -> anyhow::Result<PrimitiveDateTime> {
    use chrono::{Datelike as _, Timelike as _};

    let date = time::Date::from_calendar_date(
        datetime.year(),
        time::Month::try_from(datetime.month() as u8).unwrap(),
        datetime.day() as u8,
    )?;

    let time = time::Time::from_hms_nano(
        datetime.hour() as u8,
        datetime.minute() as u8,
        datetime.second() as u8,
        datetime.nanosecond(),
    )?;

    Ok(PrimitiveDateTime::new(date, time))
}

impl LlmDatabase {
    pub async fn create_subscription_usage(
        &self,
        user_id: UserId,
        period_start_at: DateTimeUtc,
        period_end_at: DateTimeUtc,
        plan: SubscriptionKind,
        model_requests: i32,
        edit_predictions: i32,
    ) -> Result<subscription_usage::Model> {
        self.transaction(|tx| async move {
            self.create_subscription_usage_in_tx(
                user_id,
                period_start_at,
                period_end_at,
                plan,
                model_requests,
                edit_predictions,
                &tx,
            )
            .await
        })
        .await
    }

    async fn create_subscription_usage_in_tx(
        &self,
        user_id: UserId,
        period_start_at: DateTimeUtc,
        period_end_at: DateTimeUtc,
        plan: SubscriptionKind,
        model_requests: i32,
        edit_predictions: i32,
        tx: &DatabaseTransaction,
    ) -> Result<subscription_usage::Model> {
        // Clear out the nanoseconds so that these timestamps are comparable with Unix timestamps.
        let period_start_at = period_start_at.with_nanosecond(0).unwrap();
        let period_end_at = period_end_at.with_nanosecond(0).unwrap();

        let period_start_at = convert_chrono_to_time(period_start_at)?;
        let period_end_at = convert_chrono_to_time(period_end_at)?;

        Ok(
            subscription_usage::Entity::insert(subscription_usage::ActiveModel {
                id: ActiveValue::not_set(),
                user_id: ActiveValue::set(user_id),
                period_start_at: ActiveValue::set(period_start_at),
                period_end_at: ActiveValue::set(period_end_at),
                plan: ActiveValue::set(plan),
                model_requests: ActiveValue::set(model_requests),
                edit_predictions: ActiveValue::set(edit_predictions),
            })
            .exec_with_returning(&*tx)
            .await?,
        )
    }

    pub async fn get_subscription_usage_for_period(
        &self,
        user_id: UserId,
        period_start_at: DateTimeUtc,
        period_end_at: DateTimeUtc,
    ) -> Result<Option<subscription_usage::Model>> {
        self.transaction(|tx| async move {
            self.get_subscription_usage_for_period_in_tx(
                user_id,
                period_start_at,
                period_end_at,
                &tx,
            )
            .await
        })
        .await
    }

    async fn get_subscription_usage_for_period_in_tx(
        &self,
        user_id: UserId,
        period_start_at: DateTimeUtc,
        period_end_at: DateTimeUtc,
        tx: &DatabaseTransaction,
    ) -> Result<Option<subscription_usage::Model>> {
        Ok(subscription_usage::Entity::find()
            .filter(subscription_usage::Column::UserId.eq(user_id))
            .filter(subscription_usage::Column::PeriodStartAt.eq(period_start_at))
            .filter(subscription_usage::Column::PeriodEndAt.eq(period_end_at))
            .one(&*tx)
            .await?)
    }

    pub async fn transfer_existing_subscription_usage(
        &self,
        user_id: UserId,
        existing_subscription: &billing_subscription::Model,
        new_subscription_kind: Option<SubscriptionKind>,
        new_period_start_at: DateTimeUtc,
        new_period_end_at: DateTimeUtc,
    ) -> Result<Option<subscription_usage::Model>> {
        self.transaction(|tx| async move {
            match existing_subscription.kind {
                Some(SubscriptionKind::ZedProTrial) => {
                    let trial_period_start_at = existing_subscription
                        .current_period_start_at()
                        .ok_or_else(|| anyhow!("No trial subscription period start"))?;
                    let trial_period_end_at = existing_subscription
                        .current_period_end_at()
                        .ok_or_else(|| anyhow!("No trial subscription period end"))?;

                    let existing_usage = self
                        .get_subscription_usage_for_period_in_tx(
                            user_id,
                            trial_period_start_at,
                            trial_period_end_at,
                            &tx,
                        )
                        .await?;
                    if let Some(existing_usage) = existing_usage {
                        return Ok(Some(
                            self.create_subscription_usage_in_tx(
                                user_id,
                                new_period_start_at,
                                new_period_end_at,
                                new_subscription_kind.unwrap_or(existing_usage.plan),
                                existing_usage.model_requests,
                                existing_usage.edit_predictions,
                                &tx,
                            )
                            .await?,
                        ));
                    }
                }
                _ => {}
            }

            Ok(None)
        })
        .await
    }
}
