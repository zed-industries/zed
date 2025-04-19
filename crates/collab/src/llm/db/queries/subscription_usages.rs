use time::PrimitiveDateTime;

use crate::db::UserId;

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
        model_requests: i32,
        edit_predictions: i32,
    ) -> Result<subscription_usage::Model> {
        let period_start_at = convert_chrono_to_time(period_start_at)?;
        let period_end_at = convert_chrono_to_time(period_end_at)?;

        self.transaction(|tx| async move {
            Ok(
                subscription_usage::Entity::insert(subscription_usage::ActiveModel {
                    id: ActiveValue::not_set(),
                    user_id: ActiveValue::set(user_id),
                    period_start_at: ActiveValue::set(period_start_at),
                    period_end_at: ActiveValue::set(period_end_at),
                    model_requests: ActiveValue::set(model_requests),
                    edit_predictions: ActiveValue::set(edit_predictions),
                })
                .exec_with_returning(&*tx)
                .await?,
            )
        })
        .await
    }

    pub async fn get_subscription_usage_for_period(
        &self,
        user_id: UserId,
        period_start_at: DateTimeUtc,
        period_end_at: DateTimeUtc,
    ) -> Result<Option<subscription_usage::Model>> {
        self.transaction(|tx| async move {
            Ok(subscription_usage::Entity::find()
                .filter(subscription_usage::Column::UserId.eq(user_id))
                .filter(subscription_usage::Column::PeriodStartAt.eq(period_start_at))
                .filter(subscription_usage::Column::PeriodEndAt.eq(period_end_at))
                .one(&*tx)
                .await?)
        })
        .await
    }
}
