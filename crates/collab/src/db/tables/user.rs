use crate::db::UserId;
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::Serialize;

/// A user model.
#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: UserId,
    pub github_login: String,
    pub github_user_id: i32,
    pub github_user_created_at: Option<NaiveDateTime>,
    pub email_address: Option<String>,
    pub name: Option<String>,
    pub admin: bool,
    pub invite_code: Option<String>,
    pub invite_count: i32,
    pub inviter_id: Option<UserId>,
    pub connected_once: bool,
    pub metrics_id: Uuid,
    pub created_at: NaiveDateTime,
    pub accepted_tos_at: Option<NaiveDateTime>,
    pub custom_llm_monthly_allowance_in_cents: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::access_token::Entity")]
    AccessToken,
    #[sea_orm(has_one = "super::room_participant::Entity")]
    RoomParticipant,
    #[sea_orm(has_many = "super::project::Entity")]
    HostedProjects,
    #[sea_orm(has_many = "super::channel_member::Entity")]
    ChannelMemberships,
    #[sea_orm(has_one = "super::contributor::Entity")]
    Contributor,
}

impl Related<super::access_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccessToken.def()
    }
}

impl Related<super::room_participant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoomParticipant.def()
    }
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HostedProjects.def()
    }
}

impl Related<super::channel_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChannelMemberships.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
