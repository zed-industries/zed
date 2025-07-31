use crate::db::{BufferId, UserId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "observed_buffer_edits")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: UserId,
    pub buffer_id: BufferId,
    pub epoch: i32,
    pub lamport_timestamp: i32,
    pub replica_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::buffer::Entity",
        from = "Column::BufferId",
        to = "super::buffer::Column::Id"
    )]
    Buffer,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::buffer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Buffer.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
