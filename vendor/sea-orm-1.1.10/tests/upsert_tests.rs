#![allow(unused_imports, dead_code)]

pub mod common;

pub use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::entity::prelude::*;
use sea_orm::TryInsertResult;
use sea_orm::{sea_query::OnConflict, Set};

#[sea_orm_macros::test]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("upsert_tests").await;
    create_tables(&ctx.db).await?;
    create_insert_default(&ctx.db).await?;
    ctx.delete().await;

    Ok(())
}

pub async fn create_insert_default(db: &DatabaseConnection) -> Result<(), DbErr> {
    use insert_default::*;

    let on_conflict = OnConflict::column(Column::Id)
        .do_nothing_on([Column::Id])
        .to_owned();

    let res = Entity::insert_many([
        ActiveModel { id: Set(1) },
        ActiveModel { id: Set(2) },
        ActiveModel { id: Set(3) },
    ])
    .on_conflict(on_conflict.clone())
    .exec(db)
    .await;

    assert_eq!(res?.last_insert_id, 3);

    let res = Entity::insert_many([
        ActiveModel { id: Set(1) },
        ActiveModel { id: Set(2) },
        ActiveModel { id: Set(3) },
        ActiveModel { id: Set(4) },
    ])
    .on_conflict(on_conflict.clone())
    .exec(db)
    .await;

    assert_eq!(res?.last_insert_id, 4);

    let res = Entity::insert_many([
        ActiveModel { id: Set(1) },
        ActiveModel { id: Set(2) },
        ActiveModel { id: Set(3) },
        ActiveModel { id: Set(4) },
    ])
    .on_conflict(on_conflict.clone())
    .exec(db)
    .await;

    assert!(matches!(res, Err(DbErr::RecordNotInserted)));

    let res = Entity::insert_many([
        ActiveModel { id: Set(1) },
        ActiveModel { id: Set(2) },
        ActiveModel { id: Set(3) },
        ActiveModel { id: Set(4) },
    ])
    .on_conflict(on_conflict)
    .do_nothing()
    .exec(db)
    .await;

    assert!(matches!(res, Ok(TryInsertResult::Conflicted)));

    Ok(())
}
