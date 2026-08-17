#![allow(unused_imports, dead_code)]

pub mod common;

pub use chrono::offset::Utc;
pub use common::{bakery_chain::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::sea_query::{Expr, Func, SimpleExpr};
use sea_orm::{
    entity::*,
    prelude::{DateTime, Decimal, Uuid},
    query::*,
    DbErr, DerivePartialModel, FromQueryResult,
};

// Run the test locally:
// DATABASE_URL="mysql://root:@localhost" cargo test --features sqlx-mysql,runtime-async-std-native-tls --test relational_tests
#[sea_orm_macros::test]
pub async fn left_join() {
    let ctx = TestContext::new("test_left_join").await;
    create_tables(&ctx.db).await.unwrap();

    let bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert bakery");

    let _baker_1 = baker::ActiveModel {
        name: Set("Baker 1".to_owned()),
        contact_details: Set(serde_json::json!({
            "mobile": "+61424000000",
            "home": "0395555555",
            "address": "12 Test St, Testville, Vic, Australia"
        })),
        bakery_id: Set(Some(bakery.id)),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert baker");

    let _baker_2 = baker::ActiveModel {
        name: Set("Baker 2".to_owned()),
        contact_details: Set(serde_json::json!({})),
        bakery_id: Set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert baker");

    #[derive(Debug, FromQueryResult)]
    struct SelectResult {
        name: String,
        bakery_name: Option<String>,
    }

    let select = baker::Entity::find()
        .left_join(bakery::Entity)
        .select_only()
        .column(baker::Column::Name)
        .column_as(bakery::Column::Name, "bakery_name")
        .filter(baker::Column::Name.contains("Baker 1"));

    let result = select
        .clone()
        .into_model::<SelectResult>()
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(result.name.as_str(), "Baker 1");
    assert_eq!(result.bakery_name, Some("SeaSide Bakery".to_string()));

    #[derive(DerivePartialModel, FromQueryResult, Debug, PartialEq)]
    #[sea_orm(entity = "Baker")]
    struct PartialSelectResult {
        name: String,
        #[sea_orm(from_expr = "Expr::col((bakery::Entity, bakery::Column::Name))")]
        bakery_name: Option<String>,
        #[sea_orm(
            from_expr = r#"SimpleExpr::FunctionCall(Func::upper(Expr::col((bakery::Entity, bakery::Column::Name))))"#
        )]
        bakery_name_upper: Option<String>,
    }

    let result = select
        .into_partial_model::<PartialSelectResult>()
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(result.name.as_str(), "Baker 1");
    assert_eq!(result.bakery_name, Some("SeaSide Bakery".to_string()));
    assert_eq!(result.bakery_name_upper, Some("SEASIDE BAKERY".to_string()));

    let select = baker::Entity::find()
        .left_join(bakery::Entity)
        .select_only()
        .column(baker::Column::Name)
        .column_as(bakery::Column::Name, "bakery_name")
        .filter(baker::Column::Name.contains("Baker 2"));

    let result = select
        .into_model::<SelectResult>()
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(result.bakery_name, None);

    ctx.delete().await;
}

#[sea_orm_macros::test]
#[cfg(any(feature = "sqlx-mysql", feature = "sqlx-postgres"))]
pub async fn right_join() {
    let ctx = TestContext::new("test_right_join").await;
    create_tables(&ctx.db).await.unwrap();

    let bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert bakery");

    let customer_kate = customer::ActiveModel {
        name: Set("Kate".to_owned()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert customer");

    let _customer_jim = customer::ActiveModel {
        name: Set("Jim".to_owned()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert customer");

    let _order = order::ActiveModel {
        bakery_id: Set(bakery.id),
        customer_id: Set(customer_kate.id),
        total: Set(rust_dec(15.10)),
        placed_at: Set(Utc::now().naive_utc()),

        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert order");

    #[derive(FromQueryResult)]
    #[allow(dead_code)]
    struct SelectResult {
        name: String,
        order_total: Option<Decimal>,
    }

    let select = order::Entity::find()
        .right_join(customer::Entity)
        .select_only()
        .column(customer::Column::Name)
        .column_as(order::Column::Total, "order_total")
        .filter(customer::Column::Name.contains("Kate"));

    let result = select
        .into_model::<SelectResult>()
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(result.order_total, Some(rust_dec(15.10)));

    let select = order::Entity::find()
        .right_join(customer::Entity)
        .select_only()
        .column(customer::Column::Name)
        .column_as(order::Column::Total, "order_total")
        .filter(customer::Column::Name.contains("Jim"));

    let result = select
        .into_model::<SelectResult>()
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(result.order_total, None);

    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn inner_join() {
    let ctx = TestContext::new("test_inner_join").await;
    create_tables(&ctx.db).await.unwrap();

    let bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert bakery");

    let customer_kate = customer::ActiveModel {
        name: Set("Kate".to_owned()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert customer");

    let _customer_jim = customer::ActiveModel {
        name: Set("Jim".to_owned()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert customer");

    let kate_order_1 = order::ActiveModel {
        bakery_id: Set(bakery.id),
        customer_id: Set(customer_kate.id),
        total: Set(rust_dec(15.10)),
        placed_at: Set(Utc::now().naive_utc()),

        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert order");

    let kate_order_2 = order::ActiveModel {
        bakery_id: Set(bakery.id),
        customer_id: Set(customer_kate.id),
        total: Set(rust_dec(100.00)),
        placed_at: Set(Utc::now().naive_utc()),

        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert order");

    #[derive(Debug, FromQueryResult)]
    struct SelectResult {
        name: String,
        order_total: Option<Decimal>,
    }

    let select = order::Entity::find()
        .inner_join(customer::Entity)
        .select_only()
        .column(customer::Column::Name)
        .column_as(order::Column::Total, "order_total");

    let results = select
        .into_model::<SelectResult>()
        .all(&ctx.db)
        .await
        .unwrap();

    assert_eq!(results.len(), 2);
    assert!(results
        .iter()
        .any(|result| result.name == customer_kate.name.clone()
            && result.order_total == Some(kate_order_1.total)));
    assert!(results
        .iter()
        .any(|result| result.name == customer_kate.name.clone()
            && result.order_total == Some(kate_order_2.total)));

    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn group_by() {
    let ctx = TestContext::new("test_group_by").await;
    create_tables(&ctx.db).await.unwrap();

    let bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert bakery");

    let customer_kate = customer::ActiveModel {
        name: Set("Kate".to_owned()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert customer");

    let kate_order_1 = order::ActiveModel {
        bakery_id: Set(bakery.id),
        customer_id: Set(customer_kate.id),
        total: Set(rust_dec(99.95)),
        placed_at: Set(Utc::now().naive_utc()),

        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert order");

    let kate_order_2 = order::ActiveModel {
        bakery_id: Set(bakery.id),
        customer_id: Set(customer_kate.id),
        total: Set(rust_dec(200.00)),
        placed_at: Set(Utc::now().naive_utc()),

        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert order");

    #[cfg(any(feature = "sqlx-postgres"))]
    type Type = i64;
    #[cfg(not(any(feature = "sqlx-postgres")))]
    type Type = i32;

    #[derive(Debug, FromQueryResult)]
    struct SelectResult {
        name: String,
        number_orders: Option<Type>,
        total_spent: Option<Decimal>,
        min_spent: Option<Decimal>,
        max_spent: Option<Decimal>,
    }

    let select = customer::Entity::find()
        .left_join(order::Entity)
        .select_only()
        .column(customer::Column::Name)
        .column_as(order::Column::Total.count(), "number_orders")
        .column_as(order::Column::Total.sum(), "total_spent")
        .column_as(order::Column::Total.min(), "min_spent")
        .column_as(order::Column::Total.max(), "max_spent")
        .group_by(customer::Column::Name);

    let result = select
        .into_model::<SelectResult>()
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(result.name.as_str(), "Kate");
    assert_eq!(result.number_orders, Some(2));
    assert_eq!(
        result.total_spent,
        Some(kate_order_1.total + kate_order_2.total)
    );
    assert_eq!(
        result.min_spent,
        Some(kate_order_1.total.min(kate_order_2.total))
    );
    assert_eq!(
        result.max_spent,
        Some(kate_order_1.total.max(kate_order_2.total))
    );
    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn having() {
    // customers with orders with total equal to $90
    let ctx = TestContext::new("test_having").await;
    create_tables(&ctx.db).await.unwrap();

    let bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert bakery");

    let customer_kate = customer::ActiveModel {
        name: Set("Kate".to_owned()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert customer");

    let kate_order_1 = order::ActiveModel {
        bakery_id: Set(bakery.id),
        customer_id: Set(customer_kate.id),
        total: Set(rust_dec(100.00)),
        placed_at: Set(Utc::now().naive_utc()),

        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert order");

    let _kate_order_2 = order::ActiveModel {
        bakery_id: Set(bakery.id),
        customer_id: Set(customer_kate.id),
        total: Set(rust_dec(12.00)),
        placed_at: Set(Utc::now().naive_utc()),

        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert order");

    let customer_bob = customer::ActiveModel {
        name: Set("Bob".to_owned()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert customer");

    let _bob_order_1 = order::ActiveModel {
        bakery_id: Set(bakery.id),
        customer_id: Set(customer_bob.id),
        total: Set(rust_dec(50.0)),
        placed_at: Set(Utc::now().naive_utc()),

        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert order");

    let _bob_order_2 = order::ActiveModel {
        bakery_id: Set(bakery.id),
        customer_id: Set(customer_bob.id),
        total: Set(rust_dec(50.0)),
        placed_at: Set(Utc::now().naive_utc()),

        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .expect("could not insert order");

    #[derive(Debug, FromQueryResult)]
    struct SelectResult {
        name: String,
        order_total: Option<Decimal>,
    }

    let results = customer::Entity::find()
        .inner_join(order::Entity)
        .select_only()
        .column(customer::Column::Name)
        .column_as(order::Column::Total, "order_total")
        .group_by(customer::Column::Name)
        .group_by(order::Column::Total)
        .having(order::Column::Total.gt(rust_dec(90.00)))
        .into_model::<SelectResult>()
        .all(&ctx.db)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, customer_kate.name.clone());
    assert_eq!(results[0].order_total, Some(kate_order_1.total));

    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn related() -> Result<(), DbErr> {
    use sea_orm::{SelectA, SelectB};

    let ctx = TestContext::new("test_related").await;
    create_tables(&ctx.db).await?;

    // SeaSide Bakery
    let seaside_bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    };
    let seaside_bakery_res = Bakery::insert(seaside_bakery).exec(&ctx.db).await?;

    // Bob's Baker
    let baker_bob = baker::ActiveModel {
        name: Set("Baker Bob".to_owned()),
        contact_details: Set(serde_json::json!({
            "mobile": "+61424000000",
            "home": "0395555555",
            "address": "12 Test St, Testville, Vic, Australia"
        })),
        bakery_id: Set(Some(seaside_bakery_res.last_insert_id)),
        ..Default::default()
    };
    let _baker_bob_res = Baker::insert(baker_bob).exec(&ctx.db).await?;

    // Bobby's Baker
    let baker_bobby = baker::ActiveModel {
        name: Set("Baker Bobby".to_owned()),
        contact_details: Set(serde_json::json!({
            "mobile": "+85212345678",
        })),
        bakery_id: Set(Some(seaside_bakery_res.last_insert_id)),
        ..Default::default()
    };
    let _baker_bobby_res = Baker::insert(baker_bobby).exec(&ctx.db).await?;

    // Terres Bakery
    let terres_bakery = bakery::ActiveModel {
        name: Set("Terres Bakery".to_owned()),
        profit_margin: Set(13.5),
        ..Default::default()
    };
    let terres_bakery_res = Bakery::insert(terres_bakery).exec(&ctx.db).await?;

    // Ada's Baker
    let baker_ada = baker::ActiveModel {
        name: Set("Baker Ada".to_owned()),
        contact_details: Set(serde_json::json!({
            "mobile": "+61424000000",
            "home": "0395555555",
            "address": "12 Test St, Testville, Vic, Australia"
        })),
        bakery_id: Set(Some(terres_bakery_res.last_insert_id)),
        ..Default::default()
    };
    let _baker_ada_res = Baker::insert(baker_ada).exec(&ctx.db).await?;

    // Stone Bakery, with no baker
    let stone_bakery = bakery::ActiveModel {
        name: Set("Stone Bakery".to_owned()),
        profit_margin: Set(13.5),
        ..Default::default()
    };
    let _stone_bakery_res = Bakery::insert(stone_bakery).exec(&ctx.db).await?;

    #[derive(Debug, FromQueryResult, PartialEq)]
    struct BakerLite {
        name: String,
    }

    #[derive(Debug, FromQueryResult, PartialEq)]
    struct BakeryLite {
        name: String,
    }

    // get all bakery and baker's name and put them into tuples
    let bakers_in_bakery: Vec<(BakeryLite, Option<BakerLite>)> = Bakery::find()
        .find_also_related(Baker)
        .select_only()
        .column_as(bakery::Column::Name, (SelectA, bakery::Column::Name))
        .column_as(baker::Column::Name, (SelectB, baker::Column::Name))
        .order_by_asc(bakery::Column::Id)
        .order_by_asc(baker::Column::Id)
        .into_model()
        .all(&ctx.db)
        .await?;

    assert_eq!(
        bakers_in_bakery,
        [
            (
                BakeryLite {
                    name: "SeaSide Bakery".to_owned(),
                },
                Some(BakerLite {
                    name: "Baker Bob".to_owned(),
                })
            ),
            (
                BakeryLite {
                    name: "SeaSide Bakery".to_owned(),
                },
                Some(BakerLite {
                    name: "Baker Bobby".to_owned(),
                })
            ),
            (
                BakeryLite {
                    name: "Terres Bakery".to_owned(),
                },
                Some(BakerLite {
                    name: "Baker Ada".to_owned(),
                })
            ),
            (
                BakeryLite {
                    name: "Stone Bakery".to_owned(),
                },
                None,
            ),
        ]
    );

    let seaside_bakery = Bakery::find()
        .filter(bakery::Column::Id.eq(1))
        .one(&ctx.db)
        .await?
        .unwrap();

    let bakers = seaside_bakery.find_related(Baker).all(&ctx.db).await?;

    assert_eq!(
        bakers,
        [
            baker::Model {
                id: 1,
                name: "Baker Bob".to_owned(),
                contact_details: serde_json::json!({
                    "mobile": "+61424000000",
                    "home": "0395555555",
                    "address": "12 Test St, Testville, Vic, Australia"
                }),
                bakery_id: Some(1),
            },
            baker::Model {
                id: 2,
                name: "Baker Bobby".to_owned(),
                contact_details: serde_json::json!({
                    "mobile": "+85212345678",
                }),
                bakery_id: Some(1),
            }
        ]
    );

    let select_bakery_with_baker = Bakery::find()
        .find_with_related(Baker)
        .order_by_asc(baker::Column::Id);

    assert_eq!(
        select_bakery_with_baker
            .build(sea_orm::DatabaseBackend::MySql)
            .to_string(),
        [
            "SELECT `bakery`.`id` AS `A_id`,",
            "`bakery`.`name` AS `A_name`,",
            "`bakery`.`profit_margin` AS `A_profit_margin`,",
            "`baker`.`id` AS `B_id`,",
            "`baker`.`name` AS `B_name`,",
            "`baker`.`contact_details` AS `B_contact_details`,",
            "`baker`.`bakery_id` AS `B_bakery_id`",
            "FROM `bakery`",
            "LEFT JOIN `baker` ON `bakery`.`id` = `baker`.`bakery_id`",
            "ORDER BY `bakery`.`id` ASC, `baker`.`id` ASC"
        ]
        .join(" ")
    );

    assert_eq!(
        select_bakery_with_baker.all(&ctx.db).await?,
        [
            (
                bakery::Model {
                    id: 1,
                    name: "SeaSide Bakery".to_owned(),
                    profit_margin: 10.4,
                },
                vec![
                    baker::Model {
                        id: 1,
                        name: "Baker Bob".to_owned(),
                        contact_details: serde_json::json!({
                            "mobile": "+61424000000",
                            "home": "0395555555",
                            "address": "12 Test St, Testville, Vic, Australia"
                        }),
                        bakery_id: Some(seaside_bakery_res.last_insert_id),
                    },
                    baker::Model {
                        id: 2,
                        name: "Baker Bobby".to_owned(),
                        contact_details: serde_json::json!({
                            "mobile": "+85212345678",
                        }),
                        bakery_id: Some(seaside_bakery_res.last_insert_id),
                    }
                ]
            ),
            (
                bakery::Model {
                    id: 2,
                    name: "Terres Bakery".to_owned(),
                    profit_margin: 13.5,
                },
                vec![baker::Model {
                    id: 3,
                    name: "Baker Ada".to_owned(),
                    contact_details: serde_json::json!({
                        "mobile": "+61424000000",
                        "home": "0395555555",
                        "address": "12 Test St, Testville, Vic, Australia"
                    }),
                    bakery_id: Some(terres_bakery_res.last_insert_id),
                }]
            ),
            (
                bakery::Model {
                    id: 3,
                    name: "Stone Bakery".to_owned(),
                    profit_margin: 13.5,
                },
                vec![]
            ),
        ]
    );

    ctx.delete().await;

    Ok(())
}

#[sea_orm_macros::test]
pub async fn linked() -> Result<(), DbErr> {
    use common::bakery_chain::Order;
    use sea_orm::{SelectA, SelectB};
    use sea_query::{Alias, Expr};

    let ctx = TestContext::new("test_linked").await;
    create_tables(&ctx.db).await?;

    // SeaSide Bakery
    let seaside_bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    };
    let seaside_bakery_res = Bakery::insert(seaside_bakery).exec(&ctx.db).await?;

    // Bob's Baker, Cake & Cake Baker
    let baker_bob = baker::ActiveModel {
        name: Set("Baker Bob".to_owned()),
        contact_details: Set(serde_json::json!({
            "mobile": "+61424000000",
            "home": "0395555555",
            "address": "12 Test St, Testville, Vic, Australia"
        })),
        bakery_id: Set(Some(seaside_bakery_res.last_insert_id)),
        ..Default::default()
    };
    let baker_bob_res = Baker::insert(baker_bob).exec(&ctx.db).await?;
    let mud_cake = cake::ActiveModel {
        name: Set("Mud Cake".to_owned()),
        price: Set(rust_dec(10.25)),
        gluten_free: Set(false),
        serial: Set(Uuid::new_v4()),
        bakery_id: Set(Some(seaside_bakery_res.last_insert_id)),
        ..Default::default()
    };
    let mud_cake_res = Cake::insert(mud_cake).exec(&ctx.db).await?;
    let bob_cakes_bakers = cakes_bakers::ActiveModel {
        cake_id: Set(mud_cake_res.last_insert_id),
        baker_id: Set(baker_bob_res.last_insert_id),
    };
    CakesBakers::insert(bob_cakes_bakers).exec(&ctx.db).await?;

    // Bobby's Baker, Cake & Cake Baker
    let baker_bobby = baker::ActiveModel {
        name: Set("Baker Bobby".to_owned()),
        contact_details: Set(serde_json::json!({
            "mobile": "+85212345678",
        })),
        bakery_id: Set(Some(seaside_bakery_res.last_insert_id)),
        ..Default::default()
    };
    let baker_bobby_res = Baker::insert(baker_bobby).exec(&ctx.db).await?;
    let cheese_cake = cake::ActiveModel {
        name: Set("Cheese Cake".to_owned()),
        price: Set(rust_dec(20.5)),
        gluten_free: Set(false),
        serial: Set(Uuid::new_v4()),
        bakery_id: Set(Some(seaside_bakery_res.last_insert_id)),
        ..Default::default()
    };
    let cheese_cake_res = Cake::insert(cheese_cake).exec(&ctx.db).await?;
    let bobby_cakes_bakers = cakes_bakers::ActiveModel {
        cake_id: Set(cheese_cake_res.last_insert_id),
        baker_id: Set(baker_bobby_res.last_insert_id),
    };
    CakesBakers::insert(bobby_cakes_bakers)
        .exec(&ctx.db)
        .await?;
    let chocolate_cake = cake::ActiveModel {
        name: Set("Chocolate Cake".to_owned()),
        price: Set(rust_dec(30.15)),
        gluten_free: Set(false),
        serial: Set(Uuid::new_v4()),
        bakery_id: Set(Some(seaside_bakery_res.last_insert_id)),
        ..Default::default()
    };
    let chocolate_cake_res = Cake::insert(chocolate_cake).exec(&ctx.db).await?;
    let bobby_cakes_bakers = cakes_bakers::ActiveModel {
        cake_id: Set(chocolate_cake_res.last_insert_id),
        baker_id: Set(baker_bobby_res.last_insert_id),
    };
    CakesBakers::insert(bobby_cakes_bakers)
        .exec(&ctx.db)
        .await?;

    // Freerider's Baker, no cake baked
    let baker_freerider = baker::ActiveModel {
        name: Set("Freerider".to_owned()),
        contact_details: Set(serde_json::json!({
            "mobile": "+85298765432",
        })),
        bakery_id: Set(Some(seaside_bakery_res.last_insert_id)),
        ..Default::default()
    };
    let _baker_freerider_res = Baker::insert(baker_freerider).exec(&ctx.db).await?;

    // Kate's Customer, Order & Line Item
    let customer_kate = customer::ActiveModel {
        name: Set("Kate".to_owned()),
        notes: Set(Some("Loves cheese cake".to_owned())),
        ..Default::default()
    };
    let customer_kate_res = Customer::insert(customer_kate).exec(&ctx.db).await?;
    let kate_order_1 = order::ActiveModel {
        bakery_id: Set(seaside_bakery_res.last_insert_id),
        customer_id: Set(customer_kate_res.last_insert_id),
        total: Set(rust_dec(15.10)),
        placed_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let kate_order_1_res = Order::insert(kate_order_1).exec(&ctx.db).await?;
    lineitem::ActiveModel {
        cake_id: Set(cheese_cake_res.last_insert_id),
        order_id: Set(kate_order_1_res.last_insert_id),
        price: Set(rust_dec(7.55)),
        quantity: Set(2),
        ..Default::default()
    }
    .save(&ctx.db)
    .await?;
    let kate_order_2 = order::ActiveModel {
        bakery_id: Set(seaside_bakery_res.last_insert_id),
        customer_id: Set(customer_kate_res.last_insert_id),
        total: Set(rust_dec(29.7)),
        placed_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let kate_order_2_res = Order::insert(kate_order_2).exec(&ctx.db).await?;
    lineitem::ActiveModel {
        cake_id: Set(chocolate_cake_res.last_insert_id),
        order_id: Set(kate_order_2_res.last_insert_id),
        price: Set(rust_dec(9.9)),
        quantity: Set(3),
        ..Default::default()
    }
    .save(&ctx.db)
    .await?;

    // Kara's Customer, Order & Line Item
    let customer_kara = customer::ActiveModel {
        name: Set("Kara".to_owned()),
        notes: Set(Some("Loves all cakes".to_owned())),
        ..Default::default()
    };
    let customer_kara_res = Customer::insert(customer_kara).exec(&ctx.db).await?;
    let kara_order_1 = order::ActiveModel {
        bakery_id: Set(seaside_bakery_res.last_insert_id),
        customer_id: Set(customer_kara_res.last_insert_id),
        total: Set(rust_dec(15.10)),
        placed_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let kara_order_1_res = Order::insert(kara_order_1).exec(&ctx.db).await?;
    lineitem::ActiveModel {
        cake_id: Set(mud_cake_res.last_insert_id),
        order_id: Set(kara_order_1_res.last_insert_id),
        price: Set(rust_dec(7.55)),
        quantity: Set(2),
        ..Default::default()
    }
    .save(&ctx.db)
    .await?;
    let kara_order_2 = order::ActiveModel {
        bakery_id: Set(seaside_bakery_res.last_insert_id),
        customer_id: Set(customer_kara_res.last_insert_id),
        total: Set(rust_dec(29.7)),
        placed_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let kara_order_2_res = Order::insert(kara_order_2).exec(&ctx.db).await?;
    lineitem::ActiveModel {
        cake_id: Set(cheese_cake_res.last_insert_id),
        order_id: Set(kara_order_2_res.last_insert_id),
        price: Set(rust_dec(9.9)),
        quantity: Set(3),
        ..Default::default()
    }
    .save(&ctx.db)
    .await?;

    #[derive(Debug, FromQueryResult, PartialEq)]
    struct BakerLite {
        name: String,
    }

    #[derive(Debug, FromQueryResult, PartialEq)]
    struct CustomerLite {
        name: String,
    }

    // filtered find
    let baked_for_customers: Vec<(BakerLite, Option<CustomerLite>)> = Baker::find()
        .find_also_linked(baker::BakedForCustomer)
        .select_only()
        .column_as(baker::Column::Name, (SelectA, baker::Column::Name))
        .column_as(
            Expr::col((Alias::new("r4"), customer::Column::Name)),
            (SelectB, customer::Column::Name),
        )
        .group_by(baker::Column::Id)
        .group_by(Expr::col((Alias::new("r4"), customer::Column::Id)))
        .group_by(baker::Column::Name)
        .group_by(Expr::col((Alias::new("r4"), customer::Column::Name)))
        .order_by_asc(baker::Column::Id)
        .order_by_asc(Expr::col((Alias::new("r4"), customer::Column::Id)))
        .into_model()
        .all(&ctx.db)
        .await?;

    assert_eq!(
        baked_for_customers,
        [
            (
                BakerLite {
                    name: "Baker Bob".to_owned(),
                },
                Some(CustomerLite {
                    name: "Kara".to_owned(),
                })
            ),
            (
                BakerLite {
                    name: "Baker Bobby".to_owned(),
                },
                Some(CustomerLite {
                    name: "Kate".to_owned(),
                })
            ),
            (
                BakerLite {
                    name: "Baker Bobby".to_owned(),
                },
                Some(CustomerLite {
                    name: "Kara".to_owned(),
                })
            ),
            (
                BakerLite {
                    name: "Freerider".to_owned(),
                },
                None,
            ),
        ]
    );

    // try to use find_linked instead
    let baker_bob = Baker::find()
        .filter(baker::Column::Id.eq(1))
        .one(&ctx.db)
        .await?
        .unwrap();

    let baker_bob_customers = baker_bob
        .find_linked(baker::BakedForCustomer)
        .all(&ctx.db)
        .await?;

    assert_eq!(
        baker_bob_customers,
        [customer::Model {
            id: 2,
            name: "Kara".to_owned(),
            notes: Some("Loves all cakes".to_owned()),
        }]
    );

    // find full model using with_linked
    let select_baker_with_customer = Baker::find()
        .find_with_linked(baker::BakedForCustomer)
        .order_by_asc(baker::Column::Id)
        .order_by_asc(Expr::col((Alias::new("r4"), customer::Column::Id)));

    assert_eq!(
        select_baker_with_customer
            .build(sea_orm::DatabaseBackend::MySql)
            .to_string(),
        [
            "SELECT `baker`.`id` AS `A_id`,",
            "`baker`.`name` AS `A_name`,",
            "`baker`.`contact_details` AS `A_contact_details`,",
            "`baker`.`bakery_id` AS `A_bakery_id`,",
            "`r4`.`id` AS `B_id`,",
            "`r4`.`name` AS `B_name`,",
            "`r4`.`notes` AS `B_notes`",
            "FROM `baker`",
            "LEFT JOIN `cakes_bakers` AS `r0` ON `baker`.`id` = `r0`.`baker_id`",
            "LEFT JOIN `cake` AS `r1` ON `r0`.`cake_id` = `r1`.`id`",
            "LEFT JOIN `lineitem` AS `r2` ON `r1`.`id` = `r2`.`cake_id`",
            "LEFT JOIN `order` AS `r3` ON `r2`.`order_id` = `r3`.`id`",
            "LEFT JOIN `customer` AS `r4` ON `r3`.`customer_id` = `r4`.`id`",
            "ORDER BY `baker`.`id` ASC, `r4`.`id` ASC"
        ]
        .join(" ")
    );

    assert_eq!(
        select_baker_with_customer.all(&ctx.db).await?,
        [
            (
                baker::Model {
                    id: 1,
                    name: "Baker Bob".into(),
                    contact_details: serde_json::json!({
                        "mobile": "+61424000000",
                        "home": "0395555555",
                        "address": "12 Test St, Testville, Vic, Australia",
                    }),
                    bakery_id: Some(1),
                },
                vec![customer::Model {
                    id: 2,
                    name: "Kara".into(),
                    notes: Some("Loves all cakes".into()),
                }]
            ),
            (
                baker::Model {
                    id: 2,
                    name: "Baker Bobby".into(),
                    contact_details: serde_json::json!({
                        "mobile": "+85212345678",
                    }),
                    bakery_id: Some(1),
                },
                vec![
                    customer::Model {
                        id: 1,
                        name: "Kate".into(),
                        notes: Some("Loves cheese cake".into()),
                    },
                    customer::Model {
                        id: 1,
                        name: "Kate".into(),
                        notes: Some("Loves cheese cake".into()),
                    },
                    customer::Model {
                        id: 2,
                        name: "Kara".into(),
                        notes: Some("Loves all cakes".into()),
                    },
                ]
            ),
            (
                baker::Model {
                    id: 3,
                    name: "Freerider".into(),
                    contact_details: serde_json::json!({
                        "mobile": "+85298765432",
                    }),
                    bakery_id: Some(1),
                },
                vec![]
            ),
        ]
    );

    ctx.delete().await;

    Ok(())
}

#[sea_orm_macros::test]
pub async fn select_three() -> Result<(), DbErr> {
    use common::bakery_chain::*;

    let ctx = TestContext::new("test_select_three").await;
    create_tables(&ctx.db).await?;

    seed_data::init_1(&ctx, true).await;

    let items: Vec<(order::Model, Option<customer::Model>)> = order::Entity::find()
        .find_also_related(customer::Entity)
        .order_by_asc(order::Column::Id)
        .all(&ctx.db)
        .await
        .unwrap();

    let order = order::Model {
        id: 101,
        total: Decimal::from(10),
        bakery_id: 42,
        customer_id: 11,
        placed_at: DateTime::UNIX_EPOCH,
    };

    let customer = customer::Model {
        id: 11,
        name: "Bob".to_owned(),
        notes: Some("Sweet tooth".to_owned()),
    };
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].0, order);
    assert_eq!(items[0].1.as_ref().unwrap(), &customer);

    let items: Vec<(
        order::Model,
        Option<customer::Model>,
        Option<lineitem::Model>,
    )> = order::Entity::find()
        .find_also_related(customer::Entity)
        .find_also_related(lineitem::Entity)
        .order_by_asc(order::Column::Id)
        .order_by_asc(lineitem::Column::Id)
        .all(&ctx.db)
        .await
        .unwrap();

    let line_1 = lineitem::Model {
        id: 1,
        price: 2.into(),
        quantity: 2,
        order_id: 101,
        cake_id: 13,
    };

    let line_2 = lineitem::Model {
        id: 2,
        price: 3.into(),
        quantity: 2,
        order_id: 101,
        cake_id: 15,
    };

    assert_eq!(items.len(), 2);
    assert_eq!(items[0].0, order);
    assert_eq!(items[0].1.as_ref().unwrap(), &customer);
    assert_eq!(items[1].1.as_ref().unwrap(), &customer);

    assert_eq!(items[1].0, order);
    assert_eq!(items[0].2.as_ref().unwrap(), &line_1);
    assert_eq!(items[1].2.as_ref().unwrap(), &line_2);

    let items: Vec<(order::Model, Option<lineitem::Model>, Option<cake::Model>)> =
        order::Entity::find()
            .find_also_related(lineitem::Entity)
            .and_also_related(cake::Entity)
            .order_by_asc(order::Column::Id)
            .order_by_asc(lineitem::Column::Id)
            .all(&ctx.db)
            .await
            .unwrap();

    assert_eq!(items.len(), 2);
    assert_eq!(items[0].0, order);
    assert_eq!(items[0].1.as_ref().unwrap(), &line_1);
    assert_eq!(items[0].2.as_ref().unwrap().name, "Cheesecake");

    assert_eq!(items[1].0, order);
    assert_eq!(items[1].1.as_ref().unwrap(), &line_2);
    assert_eq!(items[1].2.as_ref().unwrap().name, "Chocolate");

    ctx.delete().await;

    Ok(())
}
