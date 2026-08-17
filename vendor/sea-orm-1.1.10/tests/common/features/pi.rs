use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "pi")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Decimal(Some((11, 10)))")]
    pub decimal: Decimal,
    #[sea_orm(column_type = "Decimal(Some((11, 10)))")]
    pub big_decimal: BigDecimal,
    #[sea_orm(column_type = "Decimal(Some((11, 10)))")]
    pub decimal_opt: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((11, 10)))")]
    pub big_decimal_opt: Option<BigDecimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
