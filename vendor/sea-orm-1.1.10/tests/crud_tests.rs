#![allow(unused_imports, dead_code)]

pub mod common;
mod crud;

pub use common::{bakery_chain::*, setup::*, TestContext};
pub use crud::*;
use sea_orm::DatabaseConnection;

// Run the test locally:
// DATABASE_URL="sqlite::memory:" cargo test --features sqlx-sqlite,runtime-async-std-native-tls --test crud_tests
// DATABASE_URL="mysql://root:root@localhost" cargo test --features sqlx-mysql,runtime-async-std-native-tls --test crud_tests
// DATABASE_URL="postgres://root:root@localhost" cargo test --features sqlx-postgres,runtime-async-std-native-tls --test crud_tests
#[sea_orm_macros::test]
async fn main() {
    let ctx = TestContext::new("bakery_chain_schema_crud_tests").await;
    create_tables(&ctx.db).await.unwrap();
    create_entities(&ctx.db).await;
    ctx.delete().await;
}

pub async fn create_entities(db: &DatabaseConnection) {
    test_create_bakery(db).await;
    test_create_baker(db).await;
    test_create_customer(db).await;
    test_create_cake(db).await;
    test_create_lineitem(db).await;
    test_create_order(db).await;

    test_update_cake(db).await;
    test_update_bakery(db).await;
    test_update_deleted_customer(db).await;

    test_delete_cake(db).await;
    test_cake_error_sqlx(db).await;
    test_delete_bakery(db).await;
}
