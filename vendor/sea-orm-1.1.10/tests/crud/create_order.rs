pub use super::*;
use chrono::offset::Utc;
use uuid::Uuid;

pub async fn test_create_order(db: &DbConn) {
    // Bakery
    let seaside_bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    };
    let bakery_insert_res = Bakery::insert(seaside_bakery)
        .exec(db)
        .await
        .expect("could not insert bakery");

    // Baker
    let baker_bob = baker::ActiveModel {
        name: Set("Baker Bob".to_owned()),
        contact_details: Set(serde_json::json!({
            "mobile": "+61424000000",
            "home": "0395555555",
            "address": "12 Test St, Testville, Vic, Australia"
        })),
        bakery_id: Set(Some(bakery_insert_res.last_insert_id)),
        ..Default::default()
    };
    let baker_insert_res = Baker::insert(baker_bob)
        .exec(db)
        .await
        .expect("could not insert baker");

    // Cake
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

    // Cake_Baker
    let cake_baker = cakes_bakers::ActiveModel {
        cake_id: Set(cake_insert_res.last_insert_id),
        baker_id: Set(baker_insert_res.last_insert_id),
    };
    let cake_baker_res = CakesBakers::insert(cake_baker.clone())
        .exec(db)
        .await
        .expect("could not insert cake_baker");
    assert_eq!(
        cake_baker_res.last_insert_id,
        (cake_baker.cake_id.unwrap(), cake_baker.baker_id.unwrap())
    );

    // Customer
    let customer_kate = customer::ActiveModel {
        name: Set("Kate".to_owned()),
        notes: Set(Some("Loves cheese cake".to_owned())),
        ..Default::default()
    };
    let customer_insert_res = Customer::insert(customer_kate)
        .exec(db)
        .await
        .expect("could not insert customer");

    // Order
    let order_1 = order::ActiveModel {
        bakery_id: Set(bakery_insert_res.last_insert_id),
        customer_id: Set(customer_insert_res.last_insert_id),
        total: Set(rust_dec(15.10)),
        placed_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let order_insert_res = Order::insert(order_1)
        .exec(db)
        .await
        .expect("could not insert order");

    // Lineitem
    let lineitem_1 = lineitem::ActiveModel {
        cake_id: Set(cake_insert_res.last_insert_id),
        order_id: Set(order_insert_res.last_insert_id),
        price: Set(rust_dec(7.55)),
        quantity: Set(2),
        ..Default::default()
    };
    let _lineitem_insert_res = Lineitem::insert(lineitem_1)
        .exec(db)
        .await
        .expect("could not insert lineitem");

    let order: Option<order::Model> = Order::find_by_id(order_insert_res.last_insert_id)
        .one(db)
        .await
        .expect("could not find order");

    assert!(order.is_some());
    let order_model = order.unwrap();
    assert_eq!(order_model.total, rust_dec(15.10));

    let customer: Option<customer::Model> = Customer::find_by_id(order_model.customer_id)
        .one(db)
        .await
        .expect("could not find customer");

    let customer_model = customer.unwrap();
    assert_eq!(customer_model.name, "Kate");

    let bakery: Option<bakery::Model> = Bakery::find_by_id(order_model.bakery_id)
        .one(db)
        .await
        .expect("could not find bakery");

    let bakery_model = bakery.unwrap();
    assert_eq!(bakery_model.name, "SeaSide Bakery");

    let related_lineitems: Vec<lineitem::Model> = order_model
        .find_related(Lineitem)
        .all(db)
        .await
        .expect("could not find related lineitems");
    assert_eq!(related_lineitems.len(), 1);
    assert_eq!(related_lineitems[0].price, rust_dec(7.55));
    assert_eq!(related_lineitems[0].quantity, 2);
}
