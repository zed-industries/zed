#![allow(unused_imports, dead_code)]

pub mod common;

use std::sync::Arc;
use std::vec;

pub use common::{
    features::{
        value_type::{value_type_general, value_type_pg, Integer, StringVec, Tag1, Tag2},
        *,
    },
    setup::*,
    TestContext,
};
use pretty_assertions::assert_eq;
use sea_orm::{entity::prelude::*, entity::*, DatabaseConnection};
use sea_query::{ArrayType, ColumnType, Value, ValueType, ValueTypeErr};

#[sea_orm_macros::test]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("value_type_tests").await;
    create_tables(&ctx.db).await?;
    insert_value(&ctx.db).await?;
    ctx.delete().await;

    if cfg!(feature = "sqlx-postgres") {
        let ctx = TestContext::new("value_type_postgres_tests").await;
        create_tables(&ctx.db).await?;
        postgres_insert_value(&ctx.db).await?;
        ctx.delete().await;
    }

    type_test();
    conversion_test();

    Ok(())
}

pub async fn insert_value(db: &DatabaseConnection) -> Result<(), DbErr> {
    let model = value_type_general::Model {
        id: 1,
        number: 48.into(),
        tag_1: Tag1::Hard,
        tag_2: Tag2::Grey,
    };
    let result = model.clone().into_active_model().insert(db).await?;
    assert_eq!(result, model);

    Ok(())
}

pub async fn postgres_insert_value(db: &DatabaseConnection) -> Result<(), DbErr> {
    let model = value_type_pg::Model {
        id: 1,
        number: 48.into(),
        str_vec: StringVec(vec!["ab".to_string(), "cd".to_string()]),
    };
    let result = model.clone().into_active_model().insert(db).await?;
    assert_eq!(result, model);

    Ok(())
}

pub fn type_test() {
    assert_eq!(Integer::type_name(), "Integer");
    assert_eq!(StringVec::type_name(), "StringVec");

    assert_eq!(Integer::column_type(), ColumnType::Integer);
    assert_eq!(Integer::array_type(), ArrayType::Int);

    assert!(matches!(Tag1::column_type(), ColumnType::String(_)));
    assert_eq!(Tag1::array_type(), ArrayType::String);

    assert_eq!(
        StringVec::column_type(),
        ColumnType::Array(Arc::new(ColumnType::String(StringLen::None)))
    );
    assert_eq!(StringVec::array_type(), ArrayType::String);
}

pub fn conversion_test() {
    let stringvec = StringVec(vec!["ab".to_string(), "cd".to_string()]);
    let string: Value = stringvec.into();
    assert_eq!(
        string,
        Value::Array(
            ArrayType::String,
            Some(Box::new(vec![
                "ab".to_string().into(),
                "cd".to_string().into()
            ]))
        )
    );

    let value_random_int = Value::Int(Some(523));
    let unwrap_int = Integer::unwrap(value_random_int.clone());
    let try_from_int =
        <Integer as ValueType>::try_from(value_random_int).expect("should be ok to convert");

    // tests for unwrap and try_from
    let direct_int: Integer = 523.into();
    assert_eq!(direct_int, unwrap_int);
    assert_eq!(direct_int, try_from_int);

    // test for error
    let try_from_string_vec = <StringVec as ValueType>::try_from(Value::Char(Some('a')))
        .expect_err("should not be ok to convert char to stringvec");
    assert_eq!(try_from_string_vec.to_string(), ValueTypeErr.to_string());
}
