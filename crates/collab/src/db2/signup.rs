use super::{SignupId, UserId};
use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, PartialEq, Eq, FromQueryResult)]
pub struct Invite {
    pub email_address: String,
    pub email_confirmation_code: String,
}

#[derive(Clone, Deserialize)]
pub struct NewSignup {
    pub email_address: String,
    pub platform_mac: bool,
    pub platform_windows: bool,
    pub platform_linux: bool,
    pub editor_features: Vec<String>,
    pub programming_languages: Vec<String>,
    pub device_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, FromQueryResult)]
pub struct WaitlistSummary {
    pub count: i64,
    pub linux_count: i64,
    pub mac_count: i64,
    pub windows_count: i64,
    pub unknown_count: i64,
}
