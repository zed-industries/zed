use crate::db::{NotificationId, NotificationKindId, UserId};
use sea_orm::entity::prelude::*;
use time::PrimitiveDateTime;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "notifications")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: NotificationId,
    pub created_at: PrimitiveDateTime,
    pub recipient_id: UserId,
    pub kind: NotificationKindId,
    pub entity_id: Option<i32>,
    pub content: String,
    pub is_read: bool,
    pub response: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::RecipientId",
        to = "super::user::Column::Id"
    )]
    Recipient,
}

impl ActiveModelBehavior for ActiveModel {}
