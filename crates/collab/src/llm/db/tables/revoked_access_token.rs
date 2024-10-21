use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

use crate::llm::db::RevokedAccessTokenId;

/// A revoked access token.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "revoked_access_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: RevokedAccessTokenId,
    pub jti: String,
    pub revoked_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
