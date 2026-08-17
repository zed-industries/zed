use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "bits")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(
        column_type = r#"custom("BIT")"#,
        select_as = "BIGINT",
        save_as = "BIT"
    )]
    pub bit0: i64,
    #[sea_orm(
        column_type = r#"custom("BIT(1)")"#,
        select_as = "BIGINT",
        save_as = "BIT(1)"
    )]
    pub bit1: i64,
    #[sea_orm(
        column_type = r#"custom("BIT(8)")"#,
        select_as = "BIGINT",
        save_as = "BIT(8)"
    )]
    pub bit8: i64,
    #[sea_orm(
        column_type = r#"custom("BIT(16)")"#,
        select_as = "BIGINT",
        save_as = "BIT(16)"
    )]
    pub bit16: i64,
    #[sea_orm(
        column_type = r#"custom("BIT(32)")"#,
        select_as = "BIGINT",
        save_as = "BIT(32)"
    )]
    pub bit32: i64,
    #[sea_orm(
        column_type = r#"custom("BIT(64)")"#,
        select_as = "BIGINT",
        save_as = "BIT(64)"
    )]
    pub bit64: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
