#![allow(unused_imports, dead_code)]

pub mod common;
pub use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::{entity::prelude::*, DatabaseConnection, IntoActiveModel};

#[sea_orm_macros::test]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("bakery_chain_schema_timestamp_tests").await;
    create_tables(&ctx.db).await?;
    create_applog(&ctx.db).await?;
    create_satellites_log(&ctx.db).await?;

    ctx.delete().await;

    Ok(())
}

pub async fn create_applog(db: &DatabaseConnection) -> Result<(), DbErr> {
    let log = applog::Model {
        id: 1,
        action: "Testing".to_owned(),
        json: Json::String("HI".to_owned()),
        created_at: "2021-09-17T17:50:20+08:00".parse().unwrap(),
    };

    let res = Applog::insert(log.clone().into_active_model())
        .exec(db)
        .await?;

    assert_eq!(log.id, res.last_insert_id);
    assert_eq!(Applog::find().one(db).await?, Some(log.clone()));

    #[cfg(feature = "sqlx-sqlite")]
    assert_eq!(
        Applog::find().into_json().one(db).await?,
        Some(serde_json::json!({
            "id": 1,
            "action": "Testing",
            "json": r#""HI""#,
            "created_at": "2021-09-17T17:50:20+08:00",
        }))
    );
    #[cfg(feature = "sqlx-mysql")]
    assert_eq!(
        Applog::find().into_json().one(db).await?,
        Some(serde_json::json!({
            "id": 1,
            "action": "Testing",
            "json": "HI",
            "created_at": "2021-09-17T09:50:20Z",
        }))
    );
    #[cfg(feature = "sqlx-postgres")]
    assert_eq!(
        Applog::find().into_json().one(db).await?,
        Some(serde_json::json!({
            "id": 1,
            "action": "Testing",
            "json": "HI",
            "created_at": "2021-09-17T09:50:20Z",
        }))
    );

    Ok(())
}

pub async fn create_satellites_log(db: &DatabaseConnection) -> Result<(), DbErr> {
    let archive = satellite::Model {
        id: 1,
        satellite_name: "Sea-00001-2022".to_owned(),
        launch_date: "2022-01-07T12:11:23Z".parse().unwrap(),
        deployment_date: "2022-01-07T12:11:23Z".parse().unwrap(),
    };

    let res = Satellite::insert(archive.clone().into_active_model())
        .exec(db)
        .await?;

    assert_eq!(archive.id, res.last_insert_id);
    assert_eq!(Satellite::find().one(db).await?, Some(archive.clone()));

    #[cfg(feature = "sqlx-sqlite")]
    assert_eq!(
        Satellite::find().into_json().one(db).await?,
        Some(serde_json::json!({
            "id": 1,
            "satellite_name": "Sea-00001-2022",
            "launch_date": "2022-01-07T12:11:23+00:00",
            "deployment_date": "2022-01-07T12:11:23Z".parse::<DateTimeLocal>().unwrap().to_rfc3339(),
        }))
    );
    #[cfg(feature = "sqlx-mysql")]
    assert_eq!(
        Satellite::find().into_json().one(db).await?,
        Some(serde_json::json!({
            "id": 1,
            "satellite_name": "Sea-00001-2022",
            "launch_date": "2022-01-07T12:11:23Z",
            "deployment_date": "2022-01-07T12:11:23Z",
        }))
    );
    #[cfg(feature = "sqlx-postgres")]
    assert_eq!(
        Satellite::find().into_json().one(db).await?,
        Some(serde_json::json!({
            "id": 1,
            "satellite_name": "Sea-00001-2022",
            "launch_date": "2022-01-07T12:11:23Z",
            "deployment_date": "2022-01-07T12:11:23Z",
        }))
    );

    Ok(())
}
