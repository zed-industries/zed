use crate::db::{ChannelId, ChannelVisibility};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "channels")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ChannelId,
    pub name: String,
    pub visibility: ChannelVisibility,
    pub parent_path: String,
    pub requires_zed_cla: bool,
    /// The order of this channel relative to its siblings within the same parent.
    /// Lower values appear first. Channels are sorted by parent_path first, then by channel_order.
    pub channel_order: i32,
}

impl Model {
    pub fn parent_id(&self) -> Option<ChannelId> {
        self.ancestors().last()
    }

    pub fn is_root(&self) -> bool {
        self.parent_path.is_empty()
    }

    pub fn root_id(&self) -> ChannelId {
        self.ancestors().next().unwrap_or(self.id)
    }

    pub fn ancestors(&self) -> impl Iterator<Item = ChannelId> + '_ {
        self.parent_path
            .trim_end_matches('/')
            .split('/')
            .filter_map(|id| Some(ChannelId::from_proto(id.parse().ok()?)))
    }

    pub fn ancestors_including_self(&self) -> impl Iterator<Item = ChannelId> + '_ {
        self.ancestors().chain(Some(self.id))
    }

    pub fn path(&self) -> String {
        format!("{}{}/", self.parent_path, self.id)
    }

    pub fn descendant_path_filter(&self) -> String {
        format!("{}{}/%", self.parent_path, self.id)
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::room::Entity")]
    Room,
    #[sea_orm(has_one = "super::buffer::Entity")]
    Buffer,
    #[sea_orm(has_many = "super::channel_member::Entity")]
    Member,
    #[sea_orm(has_many = "super::channel_buffer_collaborator::Entity")]
    BufferCollaborators,
    #[sea_orm(has_many = "super::channel_chat_participant::Entity")]
    ChatParticipants,
}

impl Related<super::channel_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl Related<super::buffer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Buffer.def()
    }
}

impl Related<super::channel_buffer_collaborator::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BufferCollaborators.def()
    }
}

impl Related<super::channel_chat_participant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChatParticipants.def()
    }
}
