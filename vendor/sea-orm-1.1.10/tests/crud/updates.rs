pub use super::*;
use sea_orm::{query::*, DbErr};
use uuid::Uuid;

pub async fn test_update_cake(db: &DbConn) {
    let seaside_bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    };
    let bakery_insert_res = Bakery::insert(seaside_bakery)
        .exec(db)
        .await
        .expect("could not insert bakery");

    let mud_cake = cake::ActiveModel {
        name: Set("Mud Cake".to_owned()),
        price: Set(rust_dec(10.25)),
        gluten_free: Set(false),
        serial: Set(Uuid::new_v4()),
        bakery_id: Set(Some(bakery_insert_res.last_insert_id)),
        ..Default::default()
    };

    let cake_insert_res = Cake::insert(mud_cake)
        .exec(db)
        .await
        .expect("could not insert cake");

    let cake: Option<cake::Model> = Cake::find_by_id(cake_insert_res.last_insert_id)
        .one(db)
        .await
        .expect("could not find cake");

    assert!(cake.is_some());
    let cake_model = cake.unwrap();
    assert_eq!(cake_model.name, "Mud Cake");
    assert_eq!(cake_model.price, rust_dec(10.25));
    assert!(!cake_model.gluten_free);

    let large_number = "1234_5678_9012.3456".parse().unwrap();

    let mut cake_am: cake::ActiveModel = cake_model.into();
    cake_am.name = Set("Extra chocolate mud cake".to_owned());
    cake_am.price = Set(large_number);

    let _cake_update_res: cake::Model = cake_am.update(db).await.expect("could not update cake");

    let cake: Option<cake::Model> = Cake::find_by_id(cake_insert_res.last_insert_id)
        .one(db)
        .await
        .expect("could not find cake");
    let cake_model = cake.unwrap();
    assert_eq!(cake_model.name, "Extra chocolate mud cake");
    assert_eq!(cake_model.price, large_number);
    assert!(!cake_model.gluten_free);
}

pub async fn test_update_bakery(db: &DbConn) {
    let seaside_bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    };
    let bakery_insert_res = Bakery::insert(seaside_bakery)
        .exec(db)
        .await
        .expect("could not insert bakery");

    let bakery: Option<bakery::Model> = Bakery::find_by_id(bakery_insert_res.last_insert_id)
        .one(db)
        .await
        .expect("could not find bakery");

    assert!(bakery.is_some());
    let bakery_model = bakery.unwrap();
    assert_eq!(bakery_model.name, "SeaSide Bakery");
    assert!((bakery_model.profit_margin - 10.40).abs() < f64::EPSILON);

    let mut bakery_am: bakery::ActiveModel = bakery_model.into();
    bakery_am.name = Set("SeaBreeze Bakery".to_owned());
    bakery_am.profit_margin = Set(12.00);

    let _bakery_update_res: bakery::Model =
        bakery_am.update(db).await.expect("could not update bakery");

    let bakery: Option<bakery::Model> = Bakery::find_by_id(bakery_insert_res.last_insert_id)
        .one(db)
        .await
        .expect("could not find bakery");
    let bakery_model = bakery.unwrap();
    assert_eq!(bakery_model.name, "SeaBreeze Bakery");
    assert!((bakery_model.profit_margin - 12.00).abs() < f64::EPSILON);
}

pub async fn test_update_deleted_customer(db: &DbConn) {
    let init_n_customers = Customer::find().count(db).await.unwrap();

    let customer = customer::ActiveModel {
        name: Set("John".to_owned()),
        notes: Set(None),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert customer");

    assert_eq!(
        Customer::find().count(db).await.unwrap(),
        init_n_customers + 1
    );

    let customer_id = customer.id.clone().unwrap();

    let _ = customer.delete(db).await;
    assert_eq!(Customer::find().count(db).await.unwrap(), init_n_customers);

    let customer = customer::ActiveModel {
        id: Set(customer_id),
        name: Set("John 2".to_owned()),
        ..Default::default()
    };

    let customer_update_res = customer.update(db).await;

    assert_eq!(customer_update_res, Err(DbErr::RecordNotUpdated));

    assert_eq!(Customer::find().count(db).await.unwrap(), init_n_customers);

    let customer: Option<customer::Model> = Customer::find_by_id(customer_id)
        .one(db)
        .await
        .expect("could not find customer");

    assert_eq!(customer, None);
}
