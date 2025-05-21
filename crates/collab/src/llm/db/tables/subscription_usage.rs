use crate::db::UserId;
use crate::db::billing_subscription::SubscriptionKind;
use sea_orm::entity::prelude::*;
use time::PrimitiveDateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "subscription_usages_v2")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: UserId,
    pub period_start_at: PrimitiveDateTime,
    pub period_end_at: PrimitiveDateTime,
    pub plan: SubscriptionKind,
    pub model_requests: i32,
    pub edit_predictions: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
