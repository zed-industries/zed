#![allow(unused_imports, dead_code)]

pub mod common;

use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::{entity::prelude::*, entity::*, DatabaseConnection};
use std::str::FromStr;

#[sea_orm_macros::test]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("pi_tests").await;
    create_tables(&ctx.db).await?;
    create_and_update_pi(&ctx.db).await?;
    ctx.delete().await;

    Ok(())
}

pub async fn create_and_update_pi(db: &DatabaseConnection) -> Result<(), DbErr> {
    fn trunc_dec_scale(mut model: pi::Model) -> pi::Model {
        model.decimal = model.decimal.trunc_with_scale(3);
        model.big_decimal = model.big_decimal.with_scale(3);
        model.decimal_opt = model.decimal_opt.map(|decimal| decimal.trunc_with_scale(3));
        model.big_decimal_opt = model
            .big_decimal_opt
            .map(|big_decimal| big_decimal.with_scale(3));
        model
    }

    let pi = trunc_dec_scale(pi::Model {
        id: 1,
        decimal: rust_dec(3.1415926536),
        big_decimal: BigDecimal::from_str("3.1415926536").unwrap(),
        decimal_opt: None,
        big_decimal_opt: None,
    });

    let res = trunc_dec_scale(pi.clone().into_active_model().insert(db).await?);

    let model = trunc_dec_scale(Pi::find().one(db).await?.unwrap());
    assert_eq!(model, res);
    assert_eq!(model, pi.clone());

    let res = trunc_dec_scale(
        pi::ActiveModel {
            decimal_opt: Set(Some(rust_dec(3.1415926536))),
            big_decimal_opt: Set(Some(BigDecimal::from_str("3.1415926536").unwrap())),
            ..pi.clone().into_active_model()
        }
        .update(db)
        .await?,
    );

    let model = trunc_dec_scale(Pi::find().one(db).await?.unwrap());
    assert_eq!(model, res);
    assert_eq!(
        model,
        trunc_dec_scale(pi::Model {
            id: 1,
            decimal: rust_dec(3.1415926536),
            big_decimal: BigDecimal::from_str("3.1415926536").unwrap(),
            decimal_opt: Some(rust_dec(3.1415926536)),
            big_decimal_opt: Some(BigDecimal::from_str("3.1415926536").unwrap()),
        })
    );

    Ok(())
}
