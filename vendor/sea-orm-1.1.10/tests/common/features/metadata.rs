use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "metadata")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uuid: Uuid,
    #[sea_orm(column_name = "type", enum_name = "Type")]
    pub ty: String,
    pub key: String,
    pub value: String,
    #[sea_orm(column_type = "var_binary(32)")]
    pub bytes: Vec<u8>,
    pub date: Option<Date>,
    pub time: Option<Time>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
