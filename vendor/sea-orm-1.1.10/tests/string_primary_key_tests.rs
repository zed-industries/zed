#![allow(unused_imports, dead_code)]

pub mod common;

pub use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::{entity::prelude::*, entity::*, DatabaseConnection};
use serde_json::json;

#[sea_orm_macros::test]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("features_schema_string_primary_key_tests").await;
    create_tables(&ctx.db).await?;
    create_and_update_repository(&ctx.db).await?;
    insert_and_delete_repository(&ctx.db).await?;
    ctx.delete().await;

    Ok(())
}

pub async fn insert_and_delete_repository(db: &DatabaseConnection) -> Result<(), DbErr> {
    let repository = repository::Model {
        id: "unique-id-001".to_owned(),
        owner: "GC".to_owned(),
        name: "G.C.".to_owned(),
        description: None,
    }
    .into_active_model();

    let result = repository.clone().insert(db).await?;

    assert_eq!(
        result,
        repository::Model {
            id: "unique-id-001".to_owned(),
            owner: "GC".to_owned(),
            name: "G.C.".to_owned(),
            description: None,
        }
    );

    #[cfg(any(feature = "sqlx-sqlite", feature = "sqlx-postgres"))]
    {
        use sea_orm::sea_query::OnConflict;

        let err = Repository::insert(repository)
            // MySQL does not support DO NOTHING, we might workaround that later
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec(db)
            .await;

        assert_eq!(err.err(), Some(DbErr::RecordNotInserted));
    }

    result.delete(db).await?;

    assert_eq!(
        edit_log::Entity::find().all(db).await?,
        [
            edit_log::Model {
                id: 1,
                action: "before_save".into(),
                values: json!({
                    "description": null,
                    "id": "unique-id-001",
                    "name": "G.C.",
                    "owner": "GC",
                }),
            },
            edit_log::Model {
                id: 2,
                action: "after_save".into(),
                values: json!({
                    "description": null,
                    "id": "unique-id-001",
                    "name": "G.C.",
                    "owner": "GC",
                }),
            },
            edit_log::Model {
                id: 3,
                action: "before_delete".into(),
                values: json!({
                    "description": null,
                    "id": "unique-id-001",
                    "name": "G.C.",
                    "owner": "GC",
                }),
            },
            edit_log::Model {
                id: 4,
                action: "after_delete".into(),
                values: json!({
                    "description": null,
                    "id": "unique-id-001",
                    "name": "G.C.",
                    "owner": "GC",
                }),
            },
        ]
    );

    Ok(())
}

pub async fn create_and_update_repository(db: &DatabaseConnection) -> Result<(), DbErr> {
    let repository = repository::Model {
        id: "unique-id-002".to_owned(),
        owner: "GC".to_owned(),
        name: "G.C.".to_owned(),
        description: None,
    };

    let res = Repository::insert(repository.clone().into_active_model())
        .exec(db)
        .await?;

    assert_eq!(Repository::find().one(db).await?, Some(repository.clone()));

    assert_eq!(res.last_insert_id, repository.id);

    let updated_active_model = repository::ActiveModel {
        description: Set(Some("description...".to_owned())),
        ..repository.clone().into_active_model()
    };

    let update_res = Repository::update(updated_active_model.clone())
        .filter(repository::Column::Id.eq("not-exists-id".to_owned()))
        .exec(db)
        .await;

    assert_eq!(update_res, Err(DbErr::RecordNotUpdated));

    let update_res = Repository::update(updated_active_model)
        .filter(repository::Column::Id.eq("unique-id-002".to_owned()))
        .exec(db)
        .await?;

    assert_eq!(
        update_res,
        repository::Model {
            id: "unique-id-002".to_owned(),
            owner: "GC".to_owned(),
            name: "G.C.".to_owned(),
            description: Some("description...".to_owned()),
        }
    );

    let updated_active_model = repository::ActiveModel {
        description: Set(None),
        ..repository.clone().into_active_model()
    };

    let update_res = Repository::update(updated_active_model.clone())
        .filter(repository::Column::Id.eq("unique-id-002".to_owned()))
        .exec(db)
        .await?;

    assert_eq!(
        update_res,
        repository::Model {
            id: "unique-id-002".to_owned(),
            owner: "GC".to_owned(),
            name: "G.C.".to_owned(),
            description: None,
        }
    );

    Ok(())
}
