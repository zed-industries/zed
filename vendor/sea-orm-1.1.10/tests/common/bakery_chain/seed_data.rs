use super::*;
use crate::common::TestContext;
use sea_orm::{prelude::*, NotSet, Set};

pub async fn init_1(ctx: &TestContext, link: bool) {
    bakery::Entity::insert(bakery::ActiveModel {
        id: Set(42),
        name: Set("cool little bakery".to_string()),
        profit_margin: Set(4.1),
    })
    .exec(&ctx.db)
    .await
    .expect("insert succeeds");

    cake::Entity::insert(cake::ActiveModel {
        id: Set(13),
        name: Set("Cheesecake".to_owned()),
        price: Set(2.into()),
        bakery_id: Set(if link { Some(42) } else { None }),
        gluten_free: Set(false),
        ..Default::default()
    })
    .exec(&ctx.db)
    .await
    .expect("insert succeeds");

    cake::Entity::insert(cake::ActiveModel {
        id: Set(15),
        name: Set("Chocolate".to_owned()),
        price: Set(3.into()),
        bakery_id: Set(if link { Some(42) } else { None }),
        gluten_free: Set(true),
        ..Default::default()
    })
    .exec(&ctx.db)
    .await
    .expect("insert succeeds");

    baker::Entity::insert(baker::ActiveModel {
        id: Set(22),
        name: Set("Master Baker".to_owned()),
        contact_details: Set(Json::Null),
        bakery_id: Set(if link { Some(42) } else { None }),
    })
    .exec(&ctx.db)
    .await
    .expect("insert succeeds");

    if link {
        cakes_bakers::Entity::insert(cakes_bakers::ActiveModel {
            cake_id: Set(13),
            baker_id: Set(22),
        })
        .exec(&ctx.db)
        .await
        .expect("insert succeeds");

        customer::Entity::insert(customer::ActiveModel {
            id: Set(11),
            name: Set("Bob".to_owned()),
            notes: Set(Some("Sweet tooth".to_owned())),
        })
        .exec(&ctx.db)
        .await
        .expect("insert succeeds");

        order::Entity::insert(order::ActiveModel {
            id: Set(101),
            total: Set(10.into()),
            bakery_id: Set(42),
            customer_id: Set(11),
            placed_at: Set(DateTime::UNIX_EPOCH),
        })
        .exec(&ctx.db)
        .await
        .expect("insert succeeds");

        lineitem::Entity::insert(lineitem::ActiveModel {
            id: NotSet,
            price: Set(2.into()),
            quantity: Set(2),
            order_id: Set(101),
            cake_id: Set(13),
        })
        .exec(&ctx.db)
        .await
        .expect("insert succeeds");

        lineitem::Entity::insert(lineitem::ActiveModel {
            id: NotSet,
            price: Set(3.into()),
            quantity: Set(2),
            order_id: Set(101),
            cake_id: Set(15),
        })
        .exec(&ctx.db)
        .await
        .expect("insert succeeds");
    }
}
