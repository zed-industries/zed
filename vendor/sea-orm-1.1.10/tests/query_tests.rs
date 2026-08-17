#![allow(unused_imports, dead_code)]

pub mod common;

pub use common::{bakery_chain::*, setup::*, TestContext};
pub use sea_orm::entity::*;
pub use sea_orm::{ConnectionTrait, QueryFilter, QuerySelect};

// Run the test locally:
// DATABASE_URL="mysql://root:@localhost" cargo test --features sqlx-mysql,runtime-async-std --test query_tests
#[sea_orm_macros::test]
pub async fn find_one_with_no_result() {
    let ctx = TestContext::new("find_one_with_no_result").await;
    create_tables(&ctx.db).await.unwrap();

    let bakery = Bakery::find().one(&ctx.db).await.unwrap();
    assert_eq!(bakery, None);

    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn find_one_with_result() {
    let ctx = TestContext::new("find_one_with_result").await;
    create_tables(&ctx.db).await.unwrap();

    let bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    }
    .save(&ctx.db)
    .await
    .expect("could not insert bakery");

    let result = Bakery::find().one(&ctx.db).await.unwrap().unwrap();

    assert_eq!(result.id, bakery.id.unwrap());

    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn find_by_id_with_no_result() {
    let ctx = TestContext::new("find_by_id_with_no_result").await;
    create_tables(&ctx.db).await.unwrap();

    let bakery = Bakery::find_by_id(999).one(&ctx.db).await.unwrap();
    assert_eq!(bakery, None);

    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn find_by_id_with_result() {
    let ctx = TestContext::new("find_by_id_with_result").await;
    create_tables(&ctx.db).await.unwrap();

    let bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    }
    .save(&ctx.db)
    .await
    .expect("could not insert bakery");

    let result = Bakery::find_by_id(bakery.id.clone().unwrap())
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(result.id, bakery.id.unwrap());

    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn find_all_with_no_result() {
    let ctx = TestContext::new("find_all_with_no_result").await;
    create_tables(&ctx.db).await.unwrap();

    let bakeries = Bakery::find().all(&ctx.db).await.unwrap();
    assert_eq!(bakeries.len(), 0);

    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn find_all_with_result() {
    let ctx = TestContext::new("find_all_with_result").await;
    create_tables(&ctx.db).await.unwrap();

    let _ = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    }
    .save(&ctx.db)
    .await
    .expect("could not insert bakery");

    let _ = bakery::ActiveModel {
        name: Set("Top Bakery".to_owned()),
        profit_margin: Set(15.0),
        ..Default::default()
    }
    .save(&ctx.db)
    .await
    .expect("could not insert bakery");

    let bakeries = Bakery::find().all(&ctx.db).await.unwrap();

    assert_eq!(bakeries.len(), 2);

    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn find_all_filter_no_result() {
    let ctx = TestContext::new("find_all_filter_no_result").await;
    create_tables(&ctx.db).await.unwrap();

    let _ = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    }
    .save(&ctx.db)
    .await
    .expect("could not insert bakery");

    let _ = bakery::ActiveModel {
        name: Set("Top Bakery".to_owned()),
        profit_margin: Set(15.0),
        ..Default::default()
    }
    .save(&ctx.db)
    .await
    .expect("could not insert bakery");

    let bakeries = Bakery::find()
        .filter(bakery::Column::Name.contains("Good"))
        .all(&ctx.db)
        .await
        .unwrap();

    assert_eq!(bakeries.len(), 0);

    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn find_all_filter_with_results() {
    let ctx = TestContext::new("find_all_filter_with_results").await;
    create_tables(&ctx.db).await.unwrap();

    let _ = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    }
    .save(&ctx.db)
    .await
    .expect("could not insert bakery");

    let _ = bakery::ActiveModel {
        name: Set("Top Bakery".to_owned()),
        profit_margin: Set(15.0),
        ..Default::default()
    }
    .save(&ctx.db)
    .await
    .expect("could not insert bakery");

    let bakeries = Bakery::find()
        .filter(bakery::Column::Name.contains("Bakery"))
        .all(&ctx.db)
        .await
        .unwrap();

    assert_eq!(bakeries.len(), 2);

    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn select_only_exclude_option_fields() {
    let ctx = TestContext::new("select_only_exclude_option_fields").await;
    create_tables(&ctx.db).await.unwrap();

    let _ = customer::ActiveModel {
        name: Set("Alice".to_owned()),
        notes: Set(Some("Want to communicate with Bob".to_owned())),
        ..Default::default()
    }
    .save(&ctx.db)
    .await
    .expect("could not insert customer");

    let _ = customer::ActiveModel {
        name: Set("Bob".to_owned()),
        notes: Set(Some("Just listening".to_owned())),
        ..Default::default()
    }
    .save(&ctx.db)
    .await
    .expect("could not insert customer");

    let customers = Customer::find()
        .select_only()
        .column(customer::Column::Id)
        .column(customer::Column::Name)
        .all(&ctx.db)
        .await
        .unwrap();

    assert_eq!(customers.len(), 2);
    assert_eq!(customers[0].notes, None);
    assert_eq!(customers[1].notes, None);

    ctx.delete().await;
}
