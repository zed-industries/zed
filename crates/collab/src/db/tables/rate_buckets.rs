use crate::db::{RateBucketId, UserId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "rate_buckets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: RateBucketId,
    pub user_id: UserId,
    pub rate_limit_name: String,
    pub token_count: i32,
    pub last_refill: DateTime,
}

impl Model {}

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
