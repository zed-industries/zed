use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "applog", comment = "app logs")]
pub struct Model {
    #[sea_orm(primary_key, comment = "ID")]
    pub id: i32,
    #[sea_orm(comment = "action")]
    pub action: String,
    #[sea_orm(comment = "action data")]
    pub json: Json,
    #[sea_orm(comment = "create time")]
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
