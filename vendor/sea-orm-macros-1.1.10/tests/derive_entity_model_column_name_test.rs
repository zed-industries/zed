use sea_orm::prelude::*;
use sea_orm::Iden;
use sea_orm::Iterable;
use sea_orm_macros::DeriveEntityModel;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i32,
    username: String,
    first_name: String,
    middle_name: String,
    #[sea_orm(column_name = "lAsTnAmE")]
    last_name: String,
    orders_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[test]
fn test_column_names() {
    let columns: Vec<String> = Column::iter().map(|item| item.to_string()).collect();

    assert_eq!(
        columns,
        vec![
            "id",
            "username",
            "firstName",
            "middleName",
            "lAsTnAmE",
            "ordersCount",
        ]
    );
}
