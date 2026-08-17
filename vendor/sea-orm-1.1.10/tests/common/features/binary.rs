use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "binary")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Binary(1)")]
    pub binary: Vec<u8>,
    #[sea_orm(column_type = "Binary(10)")]
    pub binary_10: Vec<u8>,
    #[sea_orm(column_type = "VarBinary(StringLen::N(16))")]
    pub var_binary_16: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
