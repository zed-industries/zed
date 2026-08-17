#![allow(unused_imports, dead_code)]

pub mod common;

pub use common::{bakery_chain::*, setup::*, TestContext};
use sea_orm::{entity::*, query::*, DbConn, DbErr, RuntimeErr};

#[sea_orm_macros::test]
async fn loader_load_one() -> Result<(), DbErr> {
    let ctx = TestContext::new("loader_test_load_one").await;
    create_tables(&ctx.db).await?;

    let bakery_0 = insert_bakery(&ctx.db, "SeaSide Bakery").await?;

    let baker_1 = insert_baker(&ctx.db, "Baker 1", bakery_0.id).await?;
    let baker_2 = insert_baker(&ctx.db, "Baker 2", bakery_0.id).await?;
    let baker_3 = baker::ActiveModel {
        name: Set("Baker 3".to_owned()),
        contact_details: Set(serde_json::json!({})),
        bakery_id: Set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await?;

    let bakers = baker::Entity::find().all(&ctx.db).await?;
    let bakeries = bakers.load_one(bakery::Entity, &ctx.db).await?;

    assert_eq!(bakers, [baker_1, baker_2, baker_3]);
    assert_eq!(bakeries, [Some(bakery_0.clone()), Some(bakery_0), None]);

    // has many find, should use load_many instead
    let bakeries = bakery::Entity::find().all(&ctx.db).await?;
    let bakers = bakeries.load_one(baker::Entity, &ctx.db).await;

    assert_eq!(
        bakers,
        Err(DbErr::Query(RuntimeErr::Internal(
            "Relation is HasMany instead of HasOne".to_string()
        )))
    );

    Ok(())
}

#[sea_orm_macros::test]
async fn loader_load_many() -> Result<(), DbErr> {
    let ctx = TestContext::new("loader_test_load_many").await;
    create_tables(&ctx.db).await?;

    let bakery_1 = insert_bakery(&ctx.db, "SeaSide Bakery").await?;
    let bakery_2 = insert_bakery(&ctx.db, "Offshore Bakery").await?;
    let bakery_3 = insert_bakery(&ctx.db, "Rocky Bakery").await?;

    let baker_1 = insert_baker(&ctx.db, "Baker 1", bakery_1.id).await?;
    let baker_2 = insert_baker(&ctx.db, "Baker 2", bakery_1.id).await?;

    let baker_3 = insert_baker(&ctx.db, "John", bakery_2.id).await?;
    let baker_4 = insert_baker(&ctx.db, "Baker 4", bakery_2.id).await?;

    let bakeries = bakery::Entity::find().all(&ctx.db).await?;
    let bakers = bakeries.load_many(baker::Entity, &ctx.db).await?;

    assert_eq!(
        bakeries,
        [bakery_1.clone(), bakery_2.clone(), bakery_3.clone()]
    );
    assert_eq!(
        bakers,
        [
            vec![baker_1.clone(), baker_2.clone()],
            vec![baker_3.clone(), baker_4.clone()],
            vec![]
        ]
    );

    // load bakers again but with additional condition

    let bakers = bakeries
        .load_many(
            baker::Entity::find().filter(baker::Column::Name.like("Baker%")),
            &ctx.db,
        )
        .await?;

    assert_eq!(
        bakers,
        [
            vec![baker_1.clone(), baker_2.clone()],
            vec![baker_4.clone()],
            vec![]
        ]
    );

    // now, start from baker

    let bakers = baker::Entity::find().all(&ctx.db).await?;
    let bakeries = bakers.load_one(bakery::Entity::find(), &ctx.db).await?;

    // note that two bakers share the same bakery
    assert_eq!(bakers, [baker_1, baker_2, baker_3, baker_4]);
    assert_eq!(
        bakeries,
        [
            Some(bakery_1.clone()),
            Some(bakery_1),
            Some(bakery_2.clone()),
            Some(bakery_2)
        ]
    );

    Ok(())
}

#[sea_orm_macros::test]
async fn loader_load_many_multi() -> Result<(), DbErr> {
    let ctx = TestContext::new("loader_test_load_many_multi").await;
    create_tables(&ctx.db).await?;

    let bakery_1 = insert_bakery(&ctx.db, "SeaSide Bakery").await?;
    let bakery_2 = insert_bakery(&ctx.db, "Offshore Bakery").await?;

    let baker_1 = insert_baker(&ctx.db, "John", bakery_1.id).await?;
    let baker_2 = insert_baker(&ctx.db, "Jane", bakery_1.id).await?;
    let baker_3 = insert_baker(&ctx.db, "Peter", bakery_2.id).await?;

    let cake_1 = insert_cake(&ctx.db, "Cheesecake", Some(bakery_1.id)).await?;
    let cake_2 = insert_cake(&ctx.db, "Chocolate", Some(bakery_2.id)).await?;
    let cake_3 = insert_cake(&ctx.db, "Chiffon", Some(bakery_2.id)).await?;
    let _cake_4 = insert_cake(&ctx.db, "Apple Pie", None).await?; // no one makes apple pie

    let bakeries = bakery::Entity::find().all(&ctx.db).await?;
    let bakers = bakeries.load_many(baker::Entity, &ctx.db).await?;
    let cakes = bakeries.load_many(cake::Entity, &ctx.db).await?;

    assert_eq!(bakeries, [bakery_1, bakery_2]);
    assert_eq!(bakers, [vec![baker_1, baker_2], vec![baker_3]]);
    assert_eq!(cakes, [vec![cake_1], vec![cake_2, cake_3]]);

    Ok(())
}

#[sea_orm_macros::test]
async fn loader_load_many_to_many() -> Result<(), DbErr> {
    let ctx = TestContext::new("loader_test_load_many_to_many").await;
    create_tables(&ctx.db).await?;

    let bakery_1 = insert_bakery(&ctx.db, "SeaSide Bakery").await?;

    let baker_1 = insert_baker(&ctx.db, "Jane", bakery_1.id).await?;
    let baker_2 = insert_baker(&ctx.db, "Peter", bakery_1.id).await?;
    let baker_3 = insert_baker(&ctx.db, "Fred", bakery_1.id).await?; // does not make cake

    let cake_1 = insert_cake(&ctx.db, "Cheesecake", None).await?;
    let cake_2 = insert_cake(&ctx.db, "Coffee", None).await?;
    let cake_3 = insert_cake(&ctx.db, "Chiffon", None).await?;
    let cake_4 = insert_cake(&ctx.db, "Apple Pie", None).await?; // no one makes apple pie

    insert_cake_baker(&ctx.db, baker_1.id, cake_1.id).await?;
    insert_cake_baker(&ctx.db, baker_1.id, cake_2.id).await?;
    insert_cake_baker(&ctx.db, baker_2.id, cake_2.id).await?;
    insert_cake_baker(&ctx.db, baker_2.id, cake_3.id).await?;

    let bakers = baker::Entity::find().all(&ctx.db).await?;
    let cakes = bakers
        .load_many_to_many(cake::Entity, cakes_bakers::Entity, &ctx.db)
        .await?;

    assert_eq!(bakers, [baker_1.clone(), baker_2.clone(), baker_3.clone()]);
    assert_eq!(
        cakes,
        [
            vec![cake_1.clone(), cake_2.clone()],
            vec![cake_2.clone(), cake_3.clone()],
            vec![]
        ]
    );

    // same, but apply restrictions on cakes

    let cakes = bakers
        .load_many_to_many(
            cake::Entity::find().filter(cake::Column::Name.like("Ch%")),
            cakes_bakers::Entity,
            &ctx.db,
        )
        .await?;
    assert_eq!(cakes, [vec![cake_1.clone()], vec![cake_3.clone()], vec![]]);

    // now, start again from cakes

    let cakes = cake::Entity::find().all(&ctx.db).await?;
    let bakers = cakes
        .load_many_to_many(baker::Entity, cakes_bakers::Entity, &ctx.db)
        .await?;

    assert_eq!(cakes, [cake_1, cake_2, cake_3, cake_4]);
    assert_eq!(
        bakers,
        [
            vec![baker_1.clone()],
            vec![baker_1.clone(), baker_2.clone()],
            vec![baker_2.clone()],
            vec![]
        ]
    );

    Ok(())
}

pub async fn insert_bakery(db: &DbConn, name: &str) -> Result<bakery::Model, DbErr> {
    bakery::ActiveModel {
        name: Set(name.to_owned()),
        profit_margin: Set(1.0),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn insert_baker(db: &DbConn, name: &str, bakery_id: i32) -> Result<baker::Model, DbErr> {
    baker::ActiveModel {
        name: Set(name.to_owned()),
        contact_details: Set(serde_json::json!({})),
        bakery_id: Set(Some(bakery_id)),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn insert_cake(
    db: &DbConn,
    name: &str,
    bakery_id: Option<i32>,
) -> Result<cake::Model, DbErr> {
    cake::ActiveModel {
        name: Set(name.to_owned()),
        price: Set(rust_decimal::Decimal::ONE),
        gluten_free: Set(false),
        bakery_id: Set(bakery_id),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn insert_cake_baker(
    db: &DbConn,
    baker_id: i32,
    cake_id: i32,
) -> Result<cakes_bakers::Model, DbErr> {
    cakes_bakers::ActiveModel {
        cake_id: Set(cake_id),
        baker_id: Set(baker_id),
    }
    .insert(db)
    .await
}
