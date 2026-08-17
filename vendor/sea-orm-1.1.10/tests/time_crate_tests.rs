#![allow(unused_imports, dead_code)]

pub mod common;
pub use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::{entity::prelude::*, DatabaseConnection, IntoActiveModel};
use serde_json::json;
use time::macros::{date, time};

#[sea_orm_macros::test]
async fn main() {
    let ctx = TestContext::new("time_crate_tests").await;
    create_tables(&ctx.db).await.unwrap();
    create_transaction_log(&ctx.db).await.unwrap();

    ctx.delete().await;
}

pub async fn create_transaction_log(db: &DatabaseConnection) -> Result<(), DbErr> {
    let transaction_log = transaction_log::Model {
        id: 1,
        date: date!(2022 - 03 - 13),
        time: time!(16:24:00),
        date_time: date!(2022 - 03 - 13).with_time(time!(16:24:00)),
        date_time_tz: date!(2022 - 03 - 13)
            .with_time(time!(16:24:00))
            .assume_utc(),
    };

    let res = TransactionLog::insert(transaction_log.clone().into_active_model())
        .exec(db)
        .await?;

    assert_eq!(transaction_log.id, res.last_insert_id);
    assert_eq!(
        TransactionLog::find().one(db).await?,
        Some(transaction_log.clone())
    );

    let json = TransactionLog::find().into_json().one(db).await?.unwrap();

    #[cfg(feature = "sqlx-postgres")]
    assert_eq!(
        json,
        json!({
            "id": 1,
            "date": "2022-03-13",
            "time": "16:24:00",
            "date_time": "2022-03-13T16:24:00",
            "date_time_tz": "2022-03-13T16:24:00Z",
        })
    );

    #[cfg(feature = "sqlx-mysql")]
    assert_eq!(
        json,
        json!({
            "id": 1,
            "date": "2022-03-13",
            "time": "16:24:00",
            "date_time": "2022-03-13T16:24:00",
            "date_time_tz": "2022-03-13T16:24:00Z",
        })
    );

    #[cfg(feature = "sqlx-sqlite")]
    assert_eq!(
        json,
        json!({
            "id": 1,
            "date": "2022-03-13",
            "time": "16:24:00.0",
            "date_time": "2022-03-13 16:24:00.0",
            "date_time_tz": "2022-03-13T16:24:00Z",
        })
    );

    assert_ne!(json, "");

    Ok(())
}
