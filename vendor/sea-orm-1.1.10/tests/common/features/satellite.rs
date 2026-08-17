use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "satellite")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub satellite_name: String,
    #[sea_orm(default_value = "2022-01-26 16:24:00")]
    pub launch_date: DateTimeUtc,
    #[sea_orm(default_value = "2022-01-26 16:24:00")]
    pub deployment_date: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
