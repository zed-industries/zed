use crate::db::BufferId;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "buffers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: BufferId,
    pub epoch: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::buffer_operation::Entity")]
    Operations,
    #[sea_orm(has_many = "super::buffer_snapshot::Entity")]
    Snapshots,
}

impl Related<super::buffer_operation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Operations.def()
    }
}

impl Related<super::buffer_snapshot::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Snapshots.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
