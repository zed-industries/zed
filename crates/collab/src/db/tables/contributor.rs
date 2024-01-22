use crate::db::UserId;
use sea_orm::entity::prelude::*;
use serde::Serialize;

/// A user who has signed the CLA.
#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "contributors")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: UserId,
    pub signed_at: DateTime,
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

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
