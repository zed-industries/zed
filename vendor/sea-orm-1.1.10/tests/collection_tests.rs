#![allow(unused_imports, dead_code)]

pub mod common;

pub use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::{
    entity::prelude::*, entity::*, DatabaseConnection, DerivePartialModel, FromQueryResult,
};
use serde_json::json;

#[sea_orm_macros::test]
#[cfg(all(feature = "sqlx-postgres", feature = "postgres-array"))]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("collection_tests").await;
    create_tables(&ctx.db).await?;
    insert_collection(&ctx.db).await?;
    update_collection(&ctx.db).await?;
    select_collection(&ctx.db).await?;
    ctx.delete().await;

    Ok(())
}

pub async fn insert_collection(db: &DatabaseConnection) -> Result<(), DbErr> {
    use collection::*;

    let uuid = Uuid::new_v4();

    assert_eq!(
        Model {
            id: 1,
            name: "Collection 1".into(),
            integers: vec![1, 2, 3],
            integers_opt: Some(vec![1, 2, 3]),
            teas: vec![Tea::BreakfastTea],
            teas_opt: Some(vec![Tea::BreakfastTea]),
            colors: vec![Color::Black],
            colors_opt: Some(vec![Color::Black]),
            uuid: vec![uuid],
            uuid_hyphenated: vec![uuid.hyphenated()],
        }
        .into_active_model()
        .insert(db)
        .await?,
        Model {
            id: 1,
            name: "Collection 1".into(),
            integers: vec![1, 2, 3],
            integers_opt: Some(vec![1, 2, 3]),
            teas: vec![Tea::BreakfastTea],
            teas_opt: Some(vec![Tea::BreakfastTea]),
            colors: vec![Color::Black],
            colors_opt: Some(vec![Color::Black]),
            uuid: vec![uuid],
            uuid_hyphenated: vec![uuid.hyphenated()],
        }
    );

    assert_eq!(
        Model {
            id: 2,
            name: "Collection 2".into(),
            integers: vec![10, 9],
            integers_opt: None,
            teas: vec![Tea::BreakfastTea],
            teas_opt: None,
            colors: vec![Color::Black],
            colors_opt: None,
            uuid: vec![uuid],
            uuid_hyphenated: vec![uuid.hyphenated()],
        }
        .into_active_model()
        .insert(db)
        .await?,
        Model {
            id: 2,
            name: "Collection 2".into(),
            integers: vec![10, 9],
            integers_opt: None,
            teas: vec![Tea::BreakfastTea],
            teas_opt: None,
            colors: vec![Color::Black],
            colors_opt: None,
            uuid: vec![uuid],
            uuid_hyphenated: vec![uuid.hyphenated()],
        }
    );

    assert_eq!(
        Model {
            id: 3,
            name: "Collection 3".into(),
            integers: vec![],
            integers_opt: Some(vec![]),
            teas: vec![],
            teas_opt: Some(vec![]),
            colors: vec![],
            colors_opt: Some(vec![]),
            uuid: vec![uuid],
            uuid_hyphenated: vec![uuid.hyphenated()],
        }
        .into_active_model()
        .insert(db)
        .await?,
        Model {
            id: 3,
            name: "Collection 3".into(),
            integers: vec![],
            integers_opt: Some(vec![]),
            teas: vec![],
            teas_opt: Some(vec![]),
            colors: vec![],
            colors_opt: Some(vec![]),
            uuid: vec![uuid],
            uuid_hyphenated: vec![uuid.hyphenated()],
        }
    );

    assert_eq!(
        Entity::find_by_id(1).into_json().one(db).await?,
        Some(json!({
            "id": 1,
            "name": "Collection 1",
            "integers": [1, 2, 3],
            "integers_opt": [1, 2, 3],
            "teas": ["BreakfastTea"],
            "teas_opt": ["BreakfastTea"],
            "colors": [0],
            "colors_opt": [0],
            "uuid": [uuid],
            "uuid_hyphenated": [uuid.hyphenated()],
        }))
    );

    assert_eq!(
        Entity::find_by_id(2).into_json().one(db).await?,
        Some(json!({
            "id": 2,
            "name": "Collection 2",
            "integers": [10, 9],
            "integers_opt": null,
            "teas": ["BreakfastTea"],
            "teas_opt": null,
            "colors": [0],
            "colors_opt": null,
            "uuid": [uuid],
            "uuid_hyphenated": [uuid.hyphenated()],
        }))
    );

    assert_eq!(
        Entity::find_by_id(3).into_json().one(db).await?,
        Some(json!({
            "id": 3,
            "name": "Collection 3",
            "integers": [],
            "integers_opt": [],
            "teas": [],
            "teas_opt": [],
            "colors": [],
            "colors_opt": [],
            "uuid": [uuid],
            "uuid_hyphenated": [uuid.hyphenated()],
        }))
    );

    Ok(())
}

pub async fn update_collection(db: &DatabaseConnection) -> Result<(), DbErr> {
    use collection::*;

    let uuid = Uuid::new_v4();
    let model = Entity::find_by_id(1).one(db).await?.unwrap();

    ActiveModel {
        integers: Set(vec![4, 5, 6]),
        integers_opt: Set(Some(vec![4, 5, 6])),
        teas: Set(vec![Tea::EverydayTea]),
        teas_opt: Set(Some(vec![Tea::EverydayTea])),
        colors: Set(vec![Color::White]),
        colors_opt: Set(Some(vec![Color::White])),
        ..model.into_active_model()
    }
    .update(db)
    .await?;

    ActiveModel {
        id: Unchanged(3),
        name: Set("Collection 3".into()),
        integers: Set(vec![3, 1, 4]),
        integers_opt: Set(None),
        teas: Set(vec![Tea::EverydayTea]),
        teas_opt: Set(None),
        colors: Set(vec![Color::White]),
        colors_opt: Set(None),
        uuid: Set(vec![uuid]),
        uuid_hyphenated: Set(vec![uuid.hyphenated()]),
    }
    .update(db)
    .await?;

    Ok(())
}

pub async fn select_collection(db: &DatabaseConnection) -> Result<(), DbErr> {
    use collection::*;

    #[derive(DerivePartialModel, FromQueryResult, Debug, PartialEq)]
    #[sea_orm(entity = "Entity")]
    struct PartialSelectResult {
        name: String,
    }

    let result = Entity::find_by_id(1)
        .into_partial_model::<PartialSelectResult>()
        .one(db)
        .await?;

    assert_eq!(
        result,
        Some(PartialSelectResult {
            name: "Collection 1".into(),
        })
    );

    Ok(())
}
