#![allow(unused_imports, dead_code)]

pub mod common;

use common::features::*;
use pretty_assertions::assert_eq;
use sea_orm::{entity::prelude::*, entity::*, DatabaseConnection};

#[sea_orm_macros::test]
#[cfg(feature = "sqlx-postgres")]
async fn main() -> Result<(), DbErr> {
    let ctx = common::TestContext::new("bits_tests").await;
    create_tables(&ctx.db).await?;
    create_and_update(&ctx.db).await?;
    ctx.delete().await;

    Ok(())
}

pub async fn create_and_update(db: &DatabaseConnection) -> Result<(), DbErr> {
    let bits = bits::Model {
        id: 1,
        bit0: 0,
        bit1: 1,
        bit8: 8,
        bit16: 16,
        bit32: 32,
        bit64: 64,
    };

    let res = bits.clone().into_active_model().insert(db).await?;

    let model = Bits::find().one(db).await?;
    assert_eq!(model, Some(res));
    assert_eq!(model, Some(bits.clone()));

    let res = bits::ActiveModel {
        bit32: Set(320),
        bit64: Set(640),
        ..bits.clone().into_active_model()
    }
    .update(db)
    .await?;

    let model = Bits::find().one(db).await?;
    assert_eq!(model, Some(res));
    assert_eq!(
        model,
        Some(bits::Model {
            id: 1,
            bit0: 0,
            bit1: 1,
            bit8: 8,
            bit16: 16,
            bit32: 320,
            bit64: 640,
        })
    );

    Ok(())
}
