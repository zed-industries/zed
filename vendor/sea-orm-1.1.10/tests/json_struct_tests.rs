#![allow(unused_imports, dead_code)]

pub mod common;

pub use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::{entity::prelude::*, entity::*, DatabaseConnection};
use serde_json::json;

#[sea_orm_macros::test]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("json_struct_tests").await;
    create_tables(&ctx.db).await?;
    insert_json_struct_1(&ctx.db).await?;
    insert_json_struct_2(&ctx.db).await?;
    ctx.delete().await;

    Ok(())
}

pub async fn insert_json_struct_1(db: &DatabaseConnection) -> Result<(), DbErr> {
    use json_struct::*;

    let model = Model {
        id: 1,
        json: json!({
            "id": 1,
            "name": "apple",
            "price": 12.01,
            "notes": "hand picked, organic",
        }),
        json_value: KeyValue {
            id: 1,
            name: "apple".into(),
            price: 12.01,
            notes: Some("hand picked, organic".into()),
        },
        json_value_opt: Some(KeyValue {
            id: 1,
            name: "apple".into(),
            price: 12.01,
            notes: Some("hand picked, organic".into()),
        }),
    };

    let result = model.clone().into_active_model().insert(db).await?;

    assert_eq!(result, model);

    assert_eq!(
        Entity::find()
            .filter(Column::Id.eq(model.id))
            .one(db)
            .await?,
        Some(model)
    );

    Ok(())
}

pub async fn insert_json_struct_2(db: &DatabaseConnection) -> Result<(), DbErr> {
    use json_struct::*;

    let model = Model {
        id: 2,
        json: json!({
            "id": 2,
            "name": "orange",
            "price": 10.93,
            "notes": "sweet & juicy",
        }),
        json_value: KeyValue {
            id: 1,
            name: "orange".into(),
            price: 10.93,
            notes: None,
        },
        json_value_opt: None,
    };

    let result = model.clone().into_active_model().insert(db).await?;

    assert_eq!(result, model);

    assert_eq!(
        Entity::find()
            .filter(Column::Id.eq(model.id))
            .one(db)
            .await?,
        Some(model)
    );

    Ok(())
}
