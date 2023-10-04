use crate::db::{NotificationId, UserId};
use sea_orm::entity::prelude::*;
use time::PrimitiveDateTime;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "notifications")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: NotificationId,
    pub recipient_id: UserId,
    pub kind: i32,
    pub is_read: bool,
    pub created_at: PrimitiveDateTime,
    pub entity_id_1: Option<i32>,
    pub entity_id_2: Option<i32>,
    pub entity_id_3: Option<i32>,
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
