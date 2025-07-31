use sea_orm::entity::prelude::*;

use crate::db::FlagId;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "feature_flags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: FlagId,
    pub flag: String,
    pub enabled_for_all: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_feature::Entity")]
    UserFeature,
}

impl Related<super::user_feature::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserFeature.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub struct FlaggedUsers;

impl Linked for FlaggedUsers {
    type FromEntity = Entity;

    type ToEntity = super::user::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            super::user_feature::Relation::Flag.def().rev(),
            super::user_feature::Relation::User.def(),
        ]
    }
}
