#![allow(unused_imports, dead_code)]

pub mod common;

pub use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::entity::prelude::*;

#[sea_orm_macros::test]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("insert_default_tests").await;
    create_tables(&ctx.db).await?;
    create_insert_default(&ctx.db).await?;
    ctx.delete().await;

    Ok(())
}

pub async fn create_insert_default(db: &DatabaseConnection) -> Result<(), DbErr> {
    use insert_default::*;

    let active_model = ActiveModel {
        ..Default::default()
    };

    active_model.clone().insert(db).await?;
    active_model.clone().insert(db).await?;
    active_model.insert(db).await?;

    assert_eq!(
        Entity::find().all(db).await?,
        [Model { id: 1 }, Model { id: 2 }, Model { id: 3 }]
    );

    Ok(())
}
