use crate::db::{ContactId, UserId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "contacts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ContactId,
    pub user_id_a: UserId,
    pub user_id_b: UserId,
    pub a_to_b: bool,
    pub should_notify: bool,
    pub accepted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::room_participant::Entity",
        from = "Column::UserIdA",
        to = "super::room_participant::Column::UserId"
    )]
    UserARoomParticipant,
    #[sea_orm(
        belongs_to = "super::room_participant::Entity",
        from = "Column::UserIdB",
        to = "super::room_participant::Column::UserId"
    )]
    UserBRoomParticipant,
}

impl ActiveModelBehavior for ActiveModel {}
