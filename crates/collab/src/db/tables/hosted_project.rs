use crate::db::{ChannelId, ChannelVisibility, HostedProjectId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "hosted_projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: HostedProjectId,
    pub channel_id: ChannelId,
    pub name: String,
    pub visibility: ChannelVisibility,
    pub deleted_at: Option<DateTime>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::project::Entity")]
    Project,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}
