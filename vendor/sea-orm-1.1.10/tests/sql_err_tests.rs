#![allow(unused_imports, dead_code)]

pub mod common;
pub use common::{bakery_chain::*, setup::*, TestContext};
pub use sea_orm::{
    entity::*, error::DbErr, error::SqlErr, tests_cfg, ConnectionTrait, DatabaseConnection,
    DbBackend, EntityName, ExecResult,
};
use uuid::Uuid;

#[sea_orm_macros::test]
async fn main() {
    let ctx = TestContext::new("bakery_chain_sql_err_tests").await;
    create_tables(&ctx.db).await.unwrap();
    test_error(&ctx.db).await;
    ctx.delete().await;
}

pub async fn test_error(db: &DatabaseConnection) {
    let mud_cake = cake::ActiveModel {
        name: Set("Moldy Cake".to_owned()),
        price: Set(rust_dec(10.25)),
        gluten_free: Set(false),
        serial: Set(Uuid::new_v4()),
        bakery_id: Set(None),
        ..Default::default()
    };

    let cake = mud_cake.save(db).await.expect("could not insert cake");

    // if compiling without sqlx, this assignment will complain,
    // but the whole test is useless in that case anyway.
    #[allow(unused_variables)]
    let error: DbErr = cake
        .into_active_model()
        .insert(db)
        .await
        .expect_err("inserting should fail due to duplicate primary key");

    assert!(matches!(
        error.sql_err(),
        Some(SqlErr::UniqueConstraintViolation(_))
    ));

    let fk_cake = cake::ActiveModel {
        name: Set("fk error Cake".to_owned()),
        price: Set(rust_dec(10.25)),
        gluten_free: Set(false),
        serial: Set(Uuid::new_v4()),
        bakery_id: Set(Some(1000)),
        ..Default::default()
    };

    let fk_error = fk_cake
        .insert(db)
        .await
        .expect_err("create foreign key should fail with non-primary key");

    assert!(matches!(
        fk_error.sql_err(),
        Some(SqlErr::ForeignKeyConstraintViolation(_))
    ));

    let invalid_error = DbErr::Custom("random error".to_string());
    assert_eq!(invalid_error.sql_err(), None)
}
