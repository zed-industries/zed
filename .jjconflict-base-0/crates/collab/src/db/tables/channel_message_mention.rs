use crate::db::{MessageId, UserId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "channel_message_mentions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub message_id: MessageId,
    #[sea_orm(primary_key)]
    pub start_offset: i32,
    pub end_offset: i32,
    pub user_id: UserId,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::channel_message::Entity",
        from = "Column::MessageId",
        to = "super::channel_message::Column::Id"
    )]
    Message,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    MentionedUser,
}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MentionedUser.def()
    }
}
