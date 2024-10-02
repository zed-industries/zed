use crate::db::{BillingCustomerId, BillingSubscriptionId};
use sea_orm::entity::prelude::*;
use serde::Serialize;

/// A billing subscription.
#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "billing_subscriptions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: BillingSubscriptionId,
    pub billing_customer_id: BillingCustomerId,
    pub stripe_subscription_id: String,
    pub stripe_subscription_status: StripeSubscriptionStatus,
    pub stripe_cancel_at: Option<DateTime>,
    pub created_at: DateTime,
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
