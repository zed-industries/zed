use crate::db::BufferId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "buffer_operations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub buffer_id: BufferId,
    #[sea_orm(primary_key)]
    pub epoch: i32,
    #[sea_orm(primary_key)]
    pub lamport_timestamp: i32,
    #[sea_orm(primary_key)]
    pub replica_id: i32,
    pub value: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::buffer::Entity",
        from = "Column::BufferId",
        to = "super::buffer::Column::Id"
    )]
    Buffer,
}

impl Related<super::buffer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Buffer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
