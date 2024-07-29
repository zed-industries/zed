use crate::db::{BillingSubscriptionId, UserId};
use sea_orm::entity::prelude::*;
use serde::Serialize;

/// A billing subscription.
#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "billing_subscriptions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: BillingSubscriptionId,
    pub user_id: UserId,
    pub stripe_customer_id: String,
    pub stripe_subscription_id: String,
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
