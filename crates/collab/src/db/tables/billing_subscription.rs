use crate::db::{BillingCustomerId, BillingSubscriptionId};
use chrono::{Datelike as _, NaiveDate, Utc};
use sea_orm::entity::prelude::*;
use serde::Serialize;

/// A billing subscription.
#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "billing_subscriptions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: BillingSubscriptionId,
    pub billing_customer_id: BillingCustomerId,
    pub kind: Option<SubscriptionKind>,
    pub stripe_subscription_id: String,
    pub stripe_subscription_status: StripeSubscriptionStatus,
    pub stripe_cancel_at: Option<DateTime>,
    pub stripe_cancellation_reason: Option<StripeCancellationReason>,
    pub stripe_current_period_start: Option<i64>,
    pub stripe_current_period_end: Option<i64>,
    pub created_at: DateTime,
}

impl Model {
    pub fn current_period_start_at(&self) -> Option<DateTimeUtc> {
        let period_start = self.stripe_current_period_start?;
        chrono::DateTime::from_timestamp(period_start, 0)
    }

    pub fn current_period_end_at(&self) -> Option<DateTimeUtc> {
        let period_end = self.stripe_current_period_end?;
        chrono::DateTime::from_timestamp(period_end, 0)
    }

    pub fn current_period(
        subscription: Option<Self>,
        is_staff: bool,
    ) -> Option<(DateTimeUtc, DateTimeUtc)> {
        if is_staff {
            let now = Utc::now();
            let year = now.year();
            let month = now.month();

            let first_day_of_this_month =
                NaiveDate::from_ymd_opt(year, month, 1)?.and_hms_opt(0, 0, 0)?;

            let next_month = if month == 12 { 1 } else { month + 1 };
            let next_month_year = if month == 12 { year + 1 } else { year };
            let first_day_of_next_month =
                NaiveDate::from_ymd_opt(next_month_year, next_month, 1)?.and_hms_opt(23, 59, 59)?;

            let last_day_of_this_month = first_day_of_next_month - chrono::Days::new(1);

            Some((
                first_day_of_this_month.and_utc(),
                last_day_of_this_month.and_utc(),
            ))
        } else {
            let subscription = subscription?;
            let period_start_at = subscription.current_period_start_at()?;
            let period_end_at = subscription.current_period_end_at()?;

            Some((period_start_at, period_end_at))
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::billing_customer::Entity",
        from = "Column::BillingCustomerId",
        to = "super::billing_customer::Column::Id"
    )]
    BillingCustomer,
}

impl Related<super::billing_customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BillingCustomer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Eq, PartialEq, Copy, Clone, Debug, EnumIter, DeriveActiveEnum, Hash, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionKind {
    #[sea_orm(string_value = "zed_pro")]
    ZedPro,
    #[sea_orm(string_value = "zed_pro_trial")]
    ZedProTrial,
    #[sea_orm(string_value = "zed_free")]
    ZedFree,
}

impl From<SubscriptionKind> for zed_llm_client::Plan {
    fn from(value: SubscriptionKind) -> Self {
        match value {
            SubscriptionKind::ZedPro => Self::ZedPro,
            SubscriptionKind::ZedProTrial => Self::ZedProTrial,
            SubscriptionKind::ZedFree => Self::ZedFree,
        }
    }
}

/// The status of a Stripe subscription.
///
/// [Stripe docs](https://docs.stripe.com/api/subscriptions/object#subscription_object-status)
#[derive(
    Eq, PartialEq, Copy, Clone, Debug, EnumIter, DeriveActiveEnum, Default, Hash, Serialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(rename_all = "snake_case")]
pub enum StripeSubscriptionStatus {
    #[default]
    #[sea_orm(string_value = "incomplete")]
    Incomplete,
    #[sea_orm(string_value = "incomplete_expired")]
    IncompleteExpired,
    #[sea_orm(string_value = "trialing")]
    Trialing,
    #[sea_orm(string_value = "active")]
    Active,
    #[sea_orm(string_value = "past_due")]
    PastDue,
    #[sea_orm(string_value = "canceled")]
    Canceled,
    #[sea_orm(string_value = "unpaid")]
    Unpaid,
    #[sea_orm(string_value = "paused")]
    Paused,
}

impl StripeSubscriptionStatus {
    pub fn is_cancelable(&self) -> bool {
        match self {
            Self::Trialing | Self::Active | Self::PastDue => true,
            Self::Incomplete
            | Self::IncompleteExpired
            | Self::Canceled
            | Self::Unpaid
            | Self::Paused => false,
        }
    }
}

/// The cancellation reason for a Stripe subscription.
///
/// [Stripe docs](https://docs.stripe.com/api/subscriptions/object#subscription_object-cancellation_details-reason)
#[derive(Eq, PartialEq, Copy, Clone, Debug, EnumIter, DeriveActiveEnum, Hash, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(rename_all = "snake_case")]
pub enum StripeCancellationReason {
    #[sea_orm(string_value = "cancellation_requested")]
    CancellationRequested,
    #[sea_orm(string_value = "payment_disputed")]
    PaymentDisputed,
    #[sea_orm(string_value = "payment_failed")]
    PaymentFailed,
}
