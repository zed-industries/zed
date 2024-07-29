use crate::db::{BillingSubscriptionId, UserId};
use sea_orm::entity::prelude::*;

/// A billing subscription.
#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "billing_subscriptions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: BillingSubscriptionId,
    pub user_id: UserId,
    pub stripe_customer_id: String,
    pub stripe_subscription_id: String,
    pub stripe_subscription_status: StripeSubscriptionStatus,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// The status of a Stripe subscription.
///
/// [Stripe docs](https://docs.stripe.com/api/subscriptions/object#subscription_object-status)
#[derive(Eq, PartialEq, Copy, Clone, Debug, EnumIter, DeriveActiveEnum, Default, Hash)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
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
