use crate::db::{SignupId, UserId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "signups")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: SignupId,
    pub email_address: String,
    pub email_confirmation_code: String,
    pub email_confirmation_sent: bool,
    pub created_at: DateTime,
    pub device_id: Option<String>,
    pub user_id: Option<UserId>,
    pub inviting_user_id: Option<UserId>,
    pub platform_mac: bool,
    pub platform_linux: bool,
    pub platform_windows: bool,
    pub platform_unknown: bool,
    pub editor_features: Option<Vec<String>>,
    pub programming_languages: Option<Vec<String>>,
    pub added_to_mailing_list: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
