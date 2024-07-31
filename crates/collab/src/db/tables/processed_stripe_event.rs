use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "processed_stripe_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub stripe_event_id: String,
    pub stripe_event_type: String,
    pub stripe_event_created_timestamp: i64,
    pub processed_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
