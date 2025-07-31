use crate::db::{BillingCustomerId, UserId};
use sea_orm::entity::prelude::*;

/// A billing customer.
#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "billing_customers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: BillingCustomerId,
    pub user_id: UserId,
    pub stripe_customer_id: String,
    pub has_overdue_invoices: bool,
    pub trial_started_at: Option<DateTime>,
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
    #[sea_orm(has_many = "super::billing_subscription::Entity")]
    BillingSubscription,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::billing_subscription::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BillingSubscription.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
