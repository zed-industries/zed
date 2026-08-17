#![allow(unused_imports, dead_code)]

pub mod common;

use chrono::offset::Utc;
use common::{bakery_chain::*, setup::*, TestContext};
use rust_decimal::prelude::*;
use sea_orm::{entity::*, query::*, DatabaseConnection, FromQueryResult};
use uuid::Uuid;

// Run the test locally:
// DATABASE_URL="mysql://root:@localhost" cargo test --features sqlx-mysql,runtime-async-std --test sequential_op_tests
#[sea_orm_macros::test]
#[cfg(any(feature = "sqlx-mysql", feature = "sqlx-postgres"))]
pub async fn test_multiple_operations() {
    let ctx = TestContext::new("multiple_sequential_operations").await;

    create_tables(&ctx.db).await.unwrap();
    seed_data(&ctx.db).await;
    let baker_least_sales = find_baker_least_sales(&ctx.db).await.unwrap();
    assert_eq!(baker_least_sales.name, "Baker 2");

    let new_cake = create_cake(&ctx.db, baker_least_sales).await.unwrap();
    create_order(&ctx.db, new_cake).await;

    let baker_least_sales = find_baker_least_sales(&ctx.db).await.unwrap();
    assert_eq!(baker_least_sales.name, "Baker 1");

    ctx.delete().await;
}

async fn seed_data(db: &DatabaseConnection) {
    let bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert bakery");

    let baker_1 = baker::ActiveModel {
        name: Set("Baker 1".to_owned()),
        contact_details: Set(serde_json::json!({})),
        bakery_id: Set(Some(bakery.id.clone().unwrap())),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert baker");

    let _baker_2 = baker::ActiveModel {
        name: Set("Baker 2".to_owned()),
        contact_details: Set(serde_json::json!({})),
        bakery_id: Set(Some(bakery.id.clone().unwrap())),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert baker");

    let mud_cake = cake::ActiveModel {
        name: Set("Mud Cake".to_owned()),
        price: Set(rust_dec(10.25)),
        gluten_free: Set(false),
        serial: Set(Uuid::new_v4()),
        bakery_id: Set(Some(bakery.id.clone().unwrap())),
        ..Default::default()
    };

    let cake_insert_res = Cake::insert(mud_cake)
        .exec(db)
        .await
        .expect("could not insert cake");

    let cake_baker = cakes_bakers::ActiveModel {
        cake_id: Set(cake_insert_res.last_insert_id),
        baker_id: Set(baker_1.id.clone().unwrap()),
    };

    let cake_baker_res = CakesBakers::insert(cake_baker.clone())
        .exec(db)
        .await
        .expect("could not insert cake_baker");
    assert_eq!(
        cake_baker_res.last_insert_id,
        (cake_baker.cake_id.unwrap(), cake_baker.baker_id.unwrap())
    );

    let customer_kate = customer::ActiveModel {
        name: Set("Kate".to_owned()),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert customer");

    let kate_order_1 = order::ActiveModel {
        bakery_id: Set(bakery.id.clone().unwrap()),
        customer_id: Set(customer_kate.id.clone().unwrap()),
        total: Set(rust_dec(99.95)),
        placed_at: Set(Utc::now().naive_utc()),

        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert order");

    let _lineitem = lineitem::ActiveModel {
        cake_id: Set(cake_insert_res.last_insert_id),
        price: Set(rust_dec(10.00)),
        quantity: Set(12),
        order_id: Set(kate_order_1.id.clone().unwrap()),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert order");

    let _lineitem2 = lineitem::ActiveModel {
        cake_id: Set(cake_insert_res.last_insert_id),
        price: Set(rust_dec(50.00)),
        quantity: Set(2),
        order_id: Set(kate_order_1.id.clone().unwrap()),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert order");
}

async fn find_baker_least_sales(db: &DatabaseConnection) -> Option<baker::Model> {
    #[cfg(any(feature = "sqlx-postgres"))]
    type Type = i64;
    #[cfg(not(any(feature = "sqlx-postgres")))]
    type Type = Decimal;

    #[derive(Debug, FromQueryResult)]
    struct SelectResult {
        id: i32,
        cakes_sold_opt: Option<Type>,
    }

    #[derive(Debug)]
    struct LeastSalesBakerResult {
        id: i32,
        cakes_sold: Decimal,
    }

    let rel: RelationDef = cakes_bakers::Entity::belongs_to(baker::Entity)
        .from(cakes_bakers::Column::BakerId)
        .to(baker::Column::Id)
        .into();

    let rel2: RelationDef = cakes_bakers::Entity::belongs_to(cake::Entity)
        .from(cakes_bakers::Column::CakeId)
        .to(cake::Column::Id)
        .into();

    let rel3: RelationDef = cake::Entity::has_many(lineitem::Entity)
        .from(cake::Column::Id)
        .to(lineitem::Column::CakeId)
        .into();

    let select = cakes_bakers::Entity::find()
        .join(JoinType::RightJoin, rel)
        .join(JoinType::LeftJoin, rel2)
        .join(JoinType::LeftJoin, rel3)
        .select_only()
        .column(baker::Column::Id)
        .column_as(lineitem::Column::Quantity.sum(), "cakes_sold_opt")
        .group_by(baker::Column::Id);

    let mut results: Vec<LeastSalesBakerResult> = select
        .into_model::<SelectResult>()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|b| LeastSalesBakerResult {
            id: b.id,
            cakes_sold: b.cakes_sold_opt.unwrap_or_default().into(),
        })
        .collect();

    results.sort_by(|a, b| b.cakes_sold.cmp(&a.cakes_sold));

    Baker::find_by_id(results.last().unwrap().id)
        .one(db)
        .await
        .unwrap()
}

async fn create_cake(db: &DatabaseConnection, baker: baker::Model) -> Option<cake::Model> {
    let new_cake = cake::ActiveModel {
        name: Set("New Cake".to_owned()),
        price: Set(rust_dec(8.00)),
        gluten_free: Set(false),
        serial: Set(Uuid::new_v4()),
        bakery_id: Set(Some(baker.bakery_id.unwrap())),
        ..Default::default()
    };

    let cake_insert_res = Cake::insert(new_cake)
        .exec(db)
        .await
        .expect("could not insert cake");

    let cake_baker = cakes_bakers::ActiveModel {
        cake_id: Set(cake_insert_res.last_insert_id),
        baker_id: Set(baker.id),
    };

    let cake_baker_res = CakesBakers::insert(cake_baker.clone())
        .exec(db)
        .await
        .expect("could not insert cake_baker");
    assert_eq!(
        cake_baker_res.last_insert_id,
        (cake_baker.cake_id.unwrap(), cake_baker.baker_id.unwrap())
    );

    Cake::find_by_id(cake_insert_res.last_insert_id)
        .one(db)
        .await
        .unwrap()
}

async fn create_order(db: &DatabaseConnection, cake: cake::Model) {
    let another_customer = customer::ActiveModel {
        name: Set("John".to_owned()),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert customer");

    let order = order::ActiveModel {
        bakery_id: Set(cake.bakery_id.unwrap()),
        customer_id: Set(another_customer.id.clone().unwrap()),
        total: Set(rust_dec(200.00)),
        placed_at: Set(Utc::now().naive_utc()),

        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert order");

    let _lineitem = lineitem::ActiveModel {
        cake_id: Set(cake.id),
        price: Set(rust_dec(10.00)),
        quantity: Set(300),
        order_id: Set(order.id.clone().unwrap()),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert order");
}

pub async fn test_delete_bakery(db: &DatabaseConnection) {
    let initial_bakeries = Bakery::find().all(db).await.unwrap().len();

    let bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert bakery");

    assert_eq!(
        Bakery::find().all(db).await.unwrap().len(),
        initial_bakeries + 1
    );

    let _result = bakery.delete(db).await.expect("failed to delete bakery");

    assert_eq!(
        Bakery::find().all(db).await.unwrap().len(),
        initial_bakeries
    );
}
