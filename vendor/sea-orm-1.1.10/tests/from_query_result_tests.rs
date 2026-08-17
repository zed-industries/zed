#![allow(unused_imports, dead_code)]

use sea_orm::{
    prelude::*,
    query::{QueryOrder, QuerySelect},
    FromQueryResult, JoinType, Set,
};

use crate::common::TestContext;
use common::bakery_chain::*;
use serde_json::json;

mod common;

#[derive(FromQueryResult)]
struct Cake {
    id: i32,
    name: String,
    #[sea_orm(nested)]
    bakery: Option<CakeBakery>,
}

#[derive(FromQueryResult)]
struct CakeBakery {
    #[sea_orm(from_alias = "bakery_id")]
    id: i32,
    #[sea_orm(from_alias = "bakery_name")]
    title: String,
}

#[derive(FromQueryResult)]
struct BakeryDetails {
    #[sea_orm(nested)]
    basics: Bakery,
    profit: f64,
}

#[derive(FromQueryResult)]
struct Bakery {
    id: i32,
    title: String,
}

#[derive(FromQueryResult)]
struct BakeryFlat {
    id: i32,
    name: String,
    #[sea_orm(from_alias = "profit_margin")]
    profit: f64,
}

#[sea_orm_macros::test]
async fn from_query_result_left_join_does_not_exist() {
    let ctx = TestContext::new("from_query_result_left_join_does_not_exist").await;
    create_tables(&ctx.db).await.unwrap();

    seed_data::init_1(&ctx, false).await;

    let cake: Cake = cake::Entity::find()
        .select_only()
        .column(cake::Column::Id)
        .column(cake::Column::Name)
        .column_as(bakery::Column::Id, "bakery_id")
        .column_as(bakery::Column::Name, "bakery_name")
        .left_join(bakery::Entity)
        .order_by_asc(cake::Column::Id)
        .into_model()
        .one(&ctx.db)
        .await
        .expect("succeeds to get the result")
        .expect("exactly one model in DB");

    assert_eq!(cake.id, 13);
    assert_eq!(cake.name, "Cheesecake");
    assert!(cake.bakery.is_none());

    ctx.delete().await;
}

#[sea_orm_macros::test]
async fn from_query_result_left_join_exists() {
    let ctx = TestContext::new("from_query_result_left_join_exists").await;
    create_tables(&ctx.db).await.unwrap();

    seed_data::init_1(&ctx, true).await;

    let cake: Cake = cake::Entity::find()
        .select_only()
        .column(cake::Column::Id)
        .column(cake::Column::Name)
        .column_as(bakery::Column::Id, "bakery_id")
        .column_as(bakery::Column::Name, "bakery_name")
        .left_join(bakery::Entity)
        .order_by_asc(cake::Column::Id)
        .into_model()
        .one(&ctx.db)
        .await
        .expect("succeeds to get the result")
        .expect("exactly one model in DB");

    assert_eq!(cake.id, 13);
    assert_eq!(cake.name, "Cheesecake");
    let bakery = cake.bakery.unwrap();
    assert_eq!(bakery.id, 42);
    assert_eq!(bakery.title, "cool little bakery");

    ctx.delete().await;
}

#[sea_orm_macros::test]
async fn from_query_result_flat() {
    let ctx = TestContext::new("from_query_result_flat").await;
    create_tables(&ctx.db).await.unwrap();

    seed_data::init_1(&ctx, true).await;

    let bakery: BakeryFlat = bakery::Entity::find()
        .into_model()
        .one(&ctx.db)
        .await
        .expect("succeeds to get the result")
        .expect("exactly one model in DB");

    assert_eq!(bakery.id, 42);
    assert_eq!(bakery.name, "cool little bakery");
    assert_eq!(bakery.profit, 4.1);

    ctx.delete().await;
}

#[sea_orm_macros::test]
async fn from_query_result_nested() {
    let ctx = TestContext::new("from_query_result_nested").await;
    create_tables(&ctx.db).await.unwrap();

    seed_data::init_1(&ctx, true).await;

    let bakery: BakeryDetails = bakery::Entity::find()
        .select_only()
        .column(bakery::Column::Id)
        .column_as(bakery::Column::Name, "title")
        .column_as(bakery::Column::ProfitMargin, "profit")
        .into_model()
        .one(&ctx.db)
        .await
        .expect("succeeds to get the result")
        .expect("exactly one model in DB");

    assert_eq!(bakery.basics.id, 42);
    assert_eq!(bakery.basics.title, "cool little bakery");
    assert_eq!(bakery.profit, 4.1);

    ctx.delete().await;
}

#[derive(FromQueryResult)]
struct CakePlain {
    id: i32,
    name: String,
    price: Decimal,
    #[sea_orm(nested)]
    baker: Option<cakes_bakers::Model>,
    #[sea_orm(skip)]
    hidden: i32,
}

#[sea_orm_macros::test]
async fn from_query_result_plain_model() {
    let ctx = TestContext::new("from_query_result_plain_model").await;
    create_tables(&ctx.db).await.unwrap();

    seed_data::init_1(&ctx, true).await;

    let cake: CakePlain = cake::Entity::find()
        .column(cakes_bakers::Column::CakeId)
        .column(cakes_bakers::Column::BakerId)
        .join(JoinType::LeftJoin, cakes_bakers::Relation::Cake.def().rev())
        .order_by_asc(cake::Column::Id)
        .into_model()
        .one(&ctx.db)
        .await
        .expect("succeeds to get the result")
        .expect("exactly one model in DB");

    assert_eq!(cake.id, 13);
    assert_eq!(cake.name, "Cheesecake");
    assert_eq!(cake.price, Decimal::from(2));
    let baker = cake.baker.unwrap();
    assert_eq!(baker.cake_id, 13);
    assert_eq!(baker.baker_id, 22);

    ctx.delete().await;
}

#[derive(Debug, FromQueryResult)]
struct WrongBakery {
    id: String,
    title: String,
}

#[derive(Debug, FromQueryResult)]
struct WrongCake {
    id: i32,
    name: String,
    #[sea_orm(nested)]
    bakery: Option<WrongBakery>,
}

#[sea_orm_macros::test]
async fn from_query_result_optional_field_but_type_error() {
    let ctx = TestContext::new("from_query_result_nested_error").await;
    create_tables(&ctx.db).await.unwrap();

    seed_data::init_1(&ctx, false).await;

    let _: DbErr = cake::Entity::find()
        .select_only()
        .column(cake::Column::Id)
        .left_join(bakery::Entity)
        .into_model::<WrongCake>()
        .one(&ctx.db)
        .await
        .expect_err("should error instead of returning an empty Option");

    ctx.delete().await;
}
