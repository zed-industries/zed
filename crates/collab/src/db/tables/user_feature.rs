use sea_orm::entity::prelude::*;

use crate::db::{FlagId, UserId};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_features")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: UserId,
    #[sea_orm(primary_key)]
    pub feature_id: FlagId,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::feature_flag::Entity",
        from = "Column::FeatureId",
        to = "super::feature_flag::Column::Id"
    )]
    Flag,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::feature_flag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Flag.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
