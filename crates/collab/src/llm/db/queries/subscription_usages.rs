use time::PrimitiveDateTime;

use crate::db::UserId;

use super::*;

pub fn convert_chrono_to_time(datetime: DateTimeUtc) -> anyhow::Result<PrimitiveDateTime> {
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
            .one(tx)
            .await?)
    }
}
