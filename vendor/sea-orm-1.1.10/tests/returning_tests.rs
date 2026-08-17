#![allow(unused_imports, dead_code)]

pub mod common;

pub use common::{bakery_chain, setup::*, TestContext};
use sea_orm::{entity::prelude::*, IntoActiveModel, NotSet, Set};
pub use sea_query::{Expr, Query};
use serde_json::json;

#[sea_orm_macros::test]
async fn main() -> Result<(), DbErr> {
    use bakery_chain::bakery::*;

    let ctx = TestContext::new("returning_tests").await;
    let db = &ctx.db;
    let builder = db.get_database_backend();

    let mut insert = Query::insert();
    insert
        .into_table(Entity)
        .columns([Column::Name, Column::ProfitMargin])
        .values_panic(["Bakery Shop".into(), 0.5.into()]);

    let mut update = Query::update();
    update
        .table(Entity)
        .values([
            (Column::Name, "Bakery Shop".into()),
            (Column::ProfitMargin, 0.5.into()),
        ])
        .and_where(Column::Id.eq(1));

    let columns = [Column::Id, Column::Name, Column::ProfitMargin];
    let returning =
        Query::returning().exprs(columns.into_iter().map(|c| c.into_returning_expr(builder)));

    bakery_chain::create_tables(db).await?;

    if db.support_returning() {
        insert.returning(returning.clone());
        let insert_res = db
            .query_one(builder.build(&insert))
            .await?
            .expect("Insert failed with query_one");
        let _id: i32 = insert_res.try_get("", "id")?;
        let _name: String = insert_res.try_get("", "name")?;
        let _profit_margin: f64 = insert_res.try_get("", "profit_margin")?;

        update.returning(returning.clone());
        let update_res = db
            .query_one(builder.build(&update))
            .await?
            .expect("Update filed with query_one");
        let _id: i32 = update_res.try_get("", "id")?;
        let _name: String = update_res.try_get("", "name")?;
        let _profit_margin: f64 = update_res.try_get("", "profit_margin")?;
    } else {
        let insert_res = db.execute(builder.build(&insert)).await?;
        assert!(insert_res.rows_affected() > 0);

        let update_res = db.execute(builder.build(&update)).await?;
        assert!(update_res.rows_affected() > 0);
    }

    ctx.delete().await;

    Ok(())
}

#[sea_orm_macros::test]
#[cfg_attr(
    any(
        feature = "sqlx-mysql",
        all(
            feature = "sqlx-sqlite",
            not(feature = "sqlite-use-returning-for-3_35")
        )
    ),
    should_panic(expected = "Database backend doesn't support RETURNING")
)]
async fn insert_many() {
    pub use common::{features::*, TestContext};
    use edit_log::*;

    let ctx = TestContext::new("returning_tests_insert_many").await;
    let db = &ctx.db;

    create_tables(db).await.unwrap();

    Entity::insert(ActiveModel {
        id: NotSet,
        action: Set("one".into()),
        values: Set(json!({ "id": "unique-id-001" })),
    })
    .exec(db)
    .await
    .unwrap();

    assert_eq!(
        Entity::find().all(db).await.unwrap(),
        [Model {
            id: 1,
            action: "one".into(),
            values: json!({ "id": "unique-id-001" }),
        },]
    );

    assert_eq!(
        Entity::insert_many([
            ActiveModel {
                id: NotSet,
                action: Set("two".into()),
                values: Set(json!({ "id": "unique-id-002" })),
            },
            ActiveModel {
                id: NotSet,
                action: Set("three".into()),
                values: Set(json!({ "id": "unique-id-003" })),
            },
        ])
        .exec_with_returning_many(db)
        .await
        .unwrap(),
        [
            Model {
                id: 2,
                action: "two".into(),
                values: json!({ "id": "unique-id-002" }),
            },
            Model {
                id: 3,
                action: "three".into(),
                values: json!({ "id": "unique-id-003" }),
            },
        ]
    );

    assert_eq!(
        Entity::insert_many([
            ActiveModel {
                id: NotSet,
                action: Set("four".into()),
                values: Set(json!({ "id": "unique-id-004" })),
            },
            ActiveModel {
                id: NotSet,
                action: Set("five".into()),
                values: Set(json!({ "id": "unique-id-005" })),
            },
        ])
        .exec_with_returning_keys(db)
        .await
        .unwrap(),
        [4, 5]
    );
}

#[sea_orm_macros::test]
#[cfg_attr(
    any(
        feature = "sqlx-mysql",
        all(
            feature = "sqlx-sqlite",
            not(feature = "sqlite-use-returning-for-3_35")
        )
    ),
    should_panic(expected = "Database backend doesn't support RETURNING")
)]
async fn insert_many_composite_key() {
    use bakery_chain::{baker, cake, cakes_bakers};
    pub use common::{features::*, TestContext};

    let ctx = TestContext::new("returning_tests_insert_many_composite_key").await;
    let db = &ctx.db;

    bakery_chain::create_tables(db).await.unwrap();

    baker::Entity::insert_many([
        baker::ActiveModel {
            id: NotSet,
            name: Set("Baker 1".to_owned()),
            contact_details: Set(json!(null)),
            bakery_id: Set(None),
        },
        baker::ActiveModel {
            id: NotSet,
            name: Set("Baker 2".to_owned()),
            contact_details: Set(json!(null)),
            bakery_id: Set(None),
        },
    ])
    .exec(db)
    .await
    .unwrap();

    cake::Entity::insert_many([
        cake::ActiveModel {
            id: NotSet,
            name: Set("Cake 1".to_owned()),
            price: Set(Default::default()),
            bakery_id: Set(None),
            gluten_free: Set(false),
            serial: Set(Default::default()),
        },
        cake::ActiveModel {
            id: NotSet,
            name: Set("Cake 2".to_owned()),
            price: Set(Default::default()),
            bakery_id: Set(None),
            gluten_free: Set(false),
            serial: Set(Default::default()),
        },
    ])
    .exec(db)
    .await
    .unwrap();

    assert_eq!(
        cakes_bakers::Entity::insert_many([
            cakes_bakers::ActiveModel {
                cake_id: Set(1),
                baker_id: Set(2),
            },
            cakes_bakers::ActiveModel {
                cake_id: Set(2),
                baker_id: Set(1),
            },
        ])
        .exec_with_returning_keys(db)
        .await
        .unwrap(),
        [(1, 2), (2, 1)]
    );
}

#[sea_orm_macros::test]
#[cfg_attr(
    any(
        feature = "sqlx-mysql",
        all(
            feature = "sqlx-sqlite",
            not(feature = "sqlite-use-returning-for-3_35")
        )
    ),
    should_panic(expected = "Database backend doesn't support RETURNING")
)]
async fn update_many() {
    pub use common::{features::*, TestContext};
    use edit_log::*;

    let run = || async {
        let ctx = TestContext::new("returning_tests_update_many").await;
        let db = &ctx.db;

        create_tables(db).await?;

        Entity::insert(
            Model {
                id: 1,
                action: "before_save".into(),
                values: json!({ "id": "unique-id-001" }),
            }
            .into_active_model(),
        )
        .exec(db)
        .await?;

        Entity::insert(
            Model {
                id: 2,
                action: "before_save".into(),
                values: json!({ "id": "unique-id-002" }),
            }
            .into_active_model(),
        )
        .exec(db)
        .await?;

        Entity::insert(
            Model {
                id: 3,
                action: "before_save".into(),
                values: json!({ "id": "unique-id-003" }),
            }
            .into_active_model(),
        )
        .exec(db)
        .await?;

        assert_eq!(
            Entity::find().all(db).await?,
            [
                Model {
                    id: 1,
                    action: "before_save".into(),
                    values: json!({ "id": "unique-id-001" }),
                },
                Model {
                    id: 2,
                    action: "before_save".into(),
                    values: json!({ "id": "unique-id-002" }),
                },
                Model {
                    id: 3,
                    action: "before_save".into(),
                    values: json!({ "id": "unique-id-003" }),
                },
            ]
        );

        // Update many with returning
        assert_eq!(
            Entity::update_many()
                .col_expr(
                    Column::Values,
                    Expr::value(json!({ "remarks": "save log" }))
                )
                .filter(Column::Action.eq("before_save"))
                .exec_with_returning(db)
                .await?,
            [
                Model {
                    id: 1,
                    action: "before_save".into(),
                    values: json!({ "remarks": "save log" }),
                },
                Model {
                    id: 2,
                    action: "before_save".into(),
                    values: json!({ "remarks": "save log" }),
                },
                Model {
                    id: 3,
                    action: "before_save".into(),
                    values: json!({ "remarks": "save log" }),
                },
            ]
        );

        // No-op
        assert_eq!(
            Entity::update_many()
                .filter(Column::Action.eq("before_save"))
                .exec_with_returning(db)
                .await?,
            []
        );

        Result::<(), DbErr>::Ok(())
    };

    run().await.unwrap();
}

#[sea_orm_macros::test]
#[cfg_attr(
    any(
        feature = "sqlx-mysql",
        all(
            feature = "sqlx-sqlite",
            not(feature = "sqlite-use-returning-for-3_35")
        )
    ),
    should_panic(expected = "Database backend doesn't support RETURNING")
)]
async fn delete_many() {
    pub use common::{features::*, TestContext};
    use edit_log::*;

    let run = || async {
        let ctx = TestContext::new("returning_tests_delete_many").await;
        let db = &ctx.db;

        create_tables(db).await?;

        let inserted_models = [
            Model {
                id: 1,
                action: "before_save".to_string(),
                values: json!({ "id": "unique-id-001" }),
            },
            Model {
                id: 2,
                action: "before_save".to_string(),
                values: json!({ "id": "unique-id-002" }),
            },
        ];
        // Delete many with returning
        assert_eq!(
            Entity::insert_many(vec![
                ActiveModel {
                    id: NotSet,
                    action: Set("before_save".to_string()),
                    values: Set(json!({ "id": "unique-id-001" })),
                },
                ActiveModel {
                    id: NotSet,
                    action: Set("before_save".to_string()),
                    values: Set(json!({ "id": "unique-id-002" })),
                },
            ])
            .exec_with_returning_many(db)
            .await?,
            inserted_models
        );

        assert_eq!(
            Entity::delete_many()
                .filter(Column::Action.eq("before_save"))
                .exec_with_returning(db)
                .await?,
            inserted_models
        );

        let inserted_model_3 = Model {
            id: 3,
            action: "before_save".to_string(),
            values: json!({ "id": "unique-id-003" }),
        };

        Entity::insert(ActiveModel {
            id: NotSet,
            action: Set("before_save".to_string()),
            values: Set(json!({ "id": "unique-id-003" })),
        })
        .exec(db)
        .await?;

        // One
        assert_eq!(
            Entity::delete(ActiveModel {
                id: Set(3),
                ..Default::default()
            })
            .exec_with_returning(db)
            .await?,
            Some(inserted_model_3)
        );

        // No-op
        assert_eq!(Entity::delete_many().exec_with_returning(db).await?, []);

        Result::<(), DbErr>::Ok(())
    };

    run().await.unwrap();
}
