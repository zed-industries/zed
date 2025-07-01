use crate::db::{BillingPreferencesId, UserId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "billing_preferences")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: BillingPreferencesId,
    pub created_at: DateTime,
    pub user_id: UserId,
    pub max_monthly_llm_usage_spending_in_cents: i32,
    pub model_request_overages_enabled: bool,
    pub model_request_overages_spend_limit_in_cents: i32,
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
