#![allow(unused_imports, dead_code)]

pub mod common;

pub use common::{
    features::{
        event_trigger::{Event, Events},
        *,
    },
    setup::*,
    TestContext,
};
use pretty_assertions::assert_eq;
use sea_orm::{entity::prelude::*, entity::*, DatabaseConnection};

#[sea_orm_macros::test]
#[cfg(all(feature = "sqlx-postgres", feature = "postgres-array"))]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("event_trigger_tests").await;
    create_tables(&ctx.db).await?;
    insert_event_trigger(&ctx.db).await?;
    ctx.delete().await;

    Ok(())
}

pub async fn insert_event_trigger(db: &DatabaseConnection) -> Result<(), DbErr> {
    let event_trigger = event_trigger::Model {
        id: 1,
        events: Events(
            ["A", "B", "C"]
                .into_iter()
                .map(|s| Event(s.to_owned()))
                .collect(),
        ),
    };

    let result = event_trigger.clone().into_active_model().insert(db).await?;

    assert_eq!(result, event_trigger);

    let model = event_trigger::Entity::find()
        .filter(event_trigger::Column::Id.eq(event_trigger.id))
        .one(db)
        .await?;

    assert_eq!(model, Some(event_trigger));

    Ok(())
}
