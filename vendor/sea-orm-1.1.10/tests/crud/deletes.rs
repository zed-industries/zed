pub use super::*;
use uuid::Uuid;

pub async fn test_delete_cake(db: &DbConn) {
    let initial_cakes = Cake::find().all(db).await.unwrap().len();

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

    let cake = mud_cake.save(db).await.expect("could not insert cake");

    let cakes = Cake::find().all(db).await.unwrap();
    assert_eq!(cakes.len(), initial_cakes + 1);

    let _result = cake.delete(db).await.expect("failed to delete cake");

    let cakes = Cake::find().all(db).await.unwrap();
    assert_eq!(cakes.len(), initial_cakes);
}

pub async fn test_delete_bakery(db: &DbConn) {
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
