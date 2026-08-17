#![allow(unused_imports, dead_code)]

pub mod common;

pub use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::{entity::prelude::*, ConnectionTrait, DatabaseConnection};

#[sea_orm_macros::test]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("execute_unprepared_tests").await;
    create_tables(&ctx.db).await?;
    execute_unprepared(&ctx.db).await?;
    ctx.delete().await;

    Ok(())
}

pub async fn execute_unprepared(db: &DatabaseConnection) -> Result<(), DbErr> {
    use insert_default::*;

    db.execute_unprepared(
        [
            "INSERT INTO insert_default (id) VALUES (1), (2), (3), (4), (5)",
            "DELETE FROM insert_default WHERE id % 2 = 0",
        ]
        .join(";")
        .as_str(),
    )
    .await?;

    assert_eq!(
        Entity::find().all(db).await?,
        [Model { id: 1 }, Model { id: 3 }, Model { id: 5 }]
    );

    Ok(())
}
