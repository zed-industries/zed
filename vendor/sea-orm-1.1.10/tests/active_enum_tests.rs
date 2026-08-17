#![allow(unused_imports, dead_code)]

pub mod common;

use active_enum::Entity as ActiveEnumEntity;
pub use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
#[cfg(feature = "sqlx-postgres")]
use sea_orm::QueryTrait;
use sea_orm::{
    entity::prelude::*,
    entity::*,
    sea_query::{BinOper, Expr},
    ActiveEnum as ActiveEnumTrait, DatabaseConnection, FromQueryResult, QuerySelect,
};

#[sea_orm_macros::test]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("active_enum_tests").await;
    create_tables(&ctx.db).await?;
    insert_active_enum(&ctx.db).await?;
    insert_active_enum_child(&ctx.db).await?;

    #[cfg(feature = "sqlx-postgres")]
    insert_active_enum_vec(&ctx.db).await?;

    find_related_active_enum(&ctx.db).await?;
    find_linked_active_enum(&ctx.db).await?;

    ctx.delete().await;

    Ok(())
}

pub async fn insert_active_enum(db: &DatabaseConnection) -> Result<(), DbErr> {
    use active_enum::*;

    let model = Model {
        id: 1,
        category: None,
        color: None,
        tea: None,
    };

    assert_eq!(
        model,
        ActiveModel {
            category: Set(None),
            color: Set(None),
            tea: Set(None),
            ..Default::default()
        }
        .insert(db)
        .await?
    );
    assert_eq!(model, Entity::find().one(db).await?.unwrap());
    assert_eq!(
        model,
        Entity::find()
            .filter(Column::Id.is_not_null())
            .filter(Column::Category.is_null())
            .filter(Column::Color.is_null())
            .filter(Column::Tea.is_null())
            .one(db)
            .await?
            .unwrap()
    );

    let _ = ActiveModel {
        category: Set(Some(Category::Big)),
        color: Set(Some(Color::Black)),
        tea: Set(Some(Tea::EverydayTea)),
        ..model.into_active_model()
    }
    .save(db)
    .await?;

    let model = Entity::find().one(db).await?.unwrap();
    assert_eq!(
        model,
        Model {
            id: 1,
            category: Some(Category::Big),
            color: Some(Color::Black),
            tea: Some(Tea::EverydayTea),
        }
    );
    assert_eq!(
        model,
        Entity::find()
            .filter(Column::Id.eq(1))
            .filter(Column::Category.eq(Category::Big))
            .filter(Column::Color.eq(Color::Black))
            .filter(Column::Tea.eq(Tea::EverydayTea))
            .one(db)
            .await?
            .unwrap()
    );

    #[derive(Debug, FromQueryResult, PartialEq)]
    struct SelectResult {
        tea_alias: Option<Tea>,
    }

    assert_eq!(
        SelectResult {
            tea_alias: Some(Tea::EverydayTea),
        },
        Entity::find()
            .select_only()
            .column_as(Column::Tea, "tea_alias")
            .into_model()
            .one(db)
            .await?
            .unwrap()
    );

    assert_eq!(
        serde_json::json!({
            "id": 1,
            "category": "B",
            "color": 0,
            "tea": "EverydayTea",
        }),
        Entity::find().into_json().one(db).await?.unwrap()
    );

    assert_eq!(
        serde_json::json!({
            "tea_alias": "EverydayTea",
        }),
        Entity::find()
            .select_only()
            .column_as(Column::Tea, "tea_alias")
            .into_json()
            .one(db)
            .await?
            .unwrap()
    );

    assert_eq!(
        model,
        Entity::find()
            .filter(
                Expr::col(Column::Tea)
                    .binary(BinOper::In, Expr::tuple([Tea::EverydayTea.as_enum()]))
            )
            .one(db)
            .await?
            .unwrap()
    );
    // Equivalent to the above.
    let select_with_tea_in =
        Entity::find().filter(Column::Tea.is_in([Tea::EverydayTea, Tea::BreakfastTea]));
    #[cfg(feature = "sqlx-postgres")]
    assert_eq!(
        select_with_tea_in
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string(),
        [
            r#"SELECT "active_enum"."id","#,
            r#""active_enum"."category","#,
            r#""active_enum"."color","#,
            r#"CAST("active_enum"."tea" AS text)"#,
            r#"FROM "public"."active_enum""#,
            r#"WHERE "active_enum"."tea" IN (CAST('EverydayTea' AS tea), CAST('BreakfastTea' AS tea))"#,
        ]
        .join(" ")
    );
    assert_eq!(model, select_with_tea_in.one(db).await?.unwrap());

    assert_eq!(
        model,
        Entity::find()
            .filter(Column::Tea.is_not_null())
            .filter(
                Expr::col(Column::Tea)
                    .binary(BinOper::NotIn, Expr::tuple([Tea::BreakfastTea.as_enum()]))
            )
            .one(db)
            .await?
            .unwrap()
    );
    // Equivalent to the above.
    let select_with_tea_not_in = Entity::find()
        .filter(Column::Tea.is_not_null())
        .filter(Column::Tea.is_not_in([Tea::BreakfastTea]));

    #[cfg(feature = "sqlx-postgres")]
    assert_eq!(
        select_with_tea_not_in
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string(),
        [
            r#"SELECT "active_enum"."id","#,
            r#""active_enum"."category","#,
            r#""active_enum"."color","#,
            r#"CAST("active_enum"."tea" AS text)"#,
            r#"FROM "public"."active_enum""#,
            r#"WHERE "active_enum"."tea" IS NOT NULL"#,
            r#"AND "active_enum"."tea" NOT IN (CAST('BreakfastTea' AS tea))"#,
        ]
        .join(" ")
    );

    assert_eq!(model, select_with_tea_not_in.one(db).await?.unwrap());

    // String enums should be compared alphabetically in all supported DBs.
    // 'B' < 'S', so Big is considered "smaller" than Small.
    assert_eq!(
        model,
        Entity::find()
            .filter(Column::Category.lt(Category::Small))
            .one(db)
            .await?
            .unwrap()
    );

    // Integer enums should be compared by value in all supported DBs.
    // 0 <= 1, so Black is considered "smaller or equal to" White.
    assert_eq!(
        model,
        Entity::find()
            .filter(Column::Color.lte(Color::White))
            .one(db)
            .await?
            .unwrap()
    );

    // Native enum comparisons are not portable.
    //
    // Postgres enums are compared by their definition order
    // (see https://www.postgresql.org/docs/current/datatype-enum.html#DATATYPE-ENUM-ORDERING).
    // Tea was defined as ('EverydayTea', 'BreakfastTea'), so EverydayTea is considered "smaller" than BreakfastTea.
    //
    // SQLite doesn't support enum types and SeaORM works around this limitation by storing them as strings.
    // When treated as strings, EverydayTea is not "smaller" than BreakfastTea!
    //
    // MySQL should be the same as Postgres (see https://dev.mysql.com/doc/refman/8.0/en/enum.html#enum-sorting),
    // but in practice this test case behaves like SQLite. I'm not sure why.
    #[cfg(feature = "sqlx-postgres")]
    assert_eq!(
        model,
        Entity::find()
            .filter(Column::Tea.lt(Tea::BreakfastTea))
            .one(db)
            .await?
            .unwrap()
    );
    #[cfg(any(feature = "sqlx-mysql", feature = "sqlx-sqlite"))]
    assert_eq!(
        None,
        Entity::find()
            .filter(Column::Tea.lt(Tea::BreakfastTea))
            .one(db)
            .await?
    );

    let res = model.delete(db).await?;

    assert_eq!(res.rows_affected, 1);
    assert_eq!(Entity::find().one(db).await?, None);

    Ok(())
}

pub async fn insert_active_enum_child(db: &DatabaseConnection) -> Result<(), DbErr> {
    use active_enum_child::*;

    active_enum::ActiveModel {
        category: Set(Some(Category::Small)),
        color: Set(Some(Color::White)),
        tea: Set(Some(Tea::BreakfastTea)),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let am = ActiveModel {
        parent_id: Set(2),
        category: Set(None),
        color: Set(None),
        tea: Set(None),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let model = Entity::find().one(db).await?.unwrap();
    assert_eq!(
        model,
        Model {
            id: 1,
            parent_id: 2,
            category: None,
            color: None,
            tea: None,
        }
    );
    assert_eq!(
        model,
        Entity::find()
            .filter(Column::Id.is_not_null())
            .filter(Column::Category.is_null())
            .filter(Column::Color.is_null())
            .filter(Column::Tea.is_null())
            .one(db)
            .await?
            .unwrap()
    );

    ActiveModel {
        category: Set(Some(Category::Big)),
        color: Set(Some(Color::Black)),
        tea: Set(Some(Tea::EverydayTea)),
        ..am.into_active_model()
    }
    .save(db)
    .await?;

    let model = Entity::find().one(db).await?.unwrap();
    assert_eq!(
        model,
        Model {
            id: 1,
            parent_id: 2,
            category: Some(Category::Big),
            color: Some(Color::Black),
            tea: Some(Tea::EverydayTea),
        }
    );
    assert_eq!(
        model,
        Entity::find()
            .filter(Column::Id.eq(1))
            .filter(Column::Category.eq(Category::Big))
            .filter(Column::Color.eq(Color::Black))
            .filter(Column::Tea.eq(Tea::EverydayTea))
            .one(db)
            .await?
            .unwrap()
    );

    Ok(())
}

pub async fn insert_active_enum_vec(db: &DatabaseConnection) -> Result<(), DbErr> {
    use categories::*;

    let model = Model {
        id: 1,
        categories: None,
    };

    assert_eq!(
        model,
        ActiveModel {
            id: Set(1),
            categories: Set(None),
            ..Default::default()
        }
        .insert(db)
        .await?
    );
    assert_eq!(model, Entity::find().one(db).await?.unwrap());
    assert_eq!(
        model,
        Entity::find()
            .filter(Column::Id.is_not_null())
            .filter(Column::Categories.is_null())
            .one(db)
            .await?
            .unwrap()
    );

    let _ = ActiveModel {
        id: Set(1),
        categories: Set(Some(vec![Category::Big, Category::Small])),
        ..model.into_active_model()
    }
    .save(db)
    .await?;

    let model = Entity::find().one(db).await?.unwrap();
    assert_eq!(
        model,
        Model {
            id: 1,
            categories: Some(vec![Category::Big, Category::Small]),
        }
    );
    assert_eq!(
        model,
        Entity::find()
            .filter(Column::Id.eq(1))
            .filter(Expr::cust_with_values(
                r#"$1 = ANY("categories")"#,
                vec![Category::Big]
            ))
            .one(db)
            .await?
            .unwrap()
    );

    let res = model.delete(db).await?;

    assert_eq!(res.rows_affected, 1);
    assert_eq!(Entity::find().one(db).await?, None);

    Ok(())
}

pub async fn find_related_active_enum(db: &DatabaseConnection) -> Result<(), DbErr> {
    assert_eq!(
        active_enum::Model {
            id: 2,
            category: None,
            color: None,
            tea: None,
        }
        .find_related(ActiveEnumChild)
        .all(db)
        .await?,
        [active_enum_child::Model {
            id: 1,
            parent_id: 2,
            category: Some(Category::Big),
            color: Some(Color::Black),
            tea: Some(Tea::EverydayTea),
        }]
    );
    assert_eq!(
        ActiveEnumEntity::find()
            .find_with_related(ActiveEnumChild)
            .all(db)
            .await?,
        [(
            active_enum::Model {
                id: 2,
                category: Some(Category::Small),
                color: Some(Color::White),
                tea: Some(Tea::BreakfastTea),
            },
            vec![active_enum_child::Model {
                id: 1,
                parent_id: 2,
                category: Some(Category::Big),
                color: Some(Color::Black),
                tea: Some(Tea::EverydayTea),
            }]
        )]
    );
    assert_eq!(
        ActiveEnumEntity::find()
            .find_also_related(ActiveEnumChild)
            .all(db)
            .await?,
        [(
            active_enum::Model {
                id: 2,
                category: Some(Category::Small),
                color: Some(Color::White),
                tea: Some(Tea::BreakfastTea),
            },
            Some(active_enum_child::Model {
                id: 1,
                parent_id: 2,
                category: Some(Category::Big),
                color: Some(Color::Black),
                tea: Some(Tea::EverydayTea),
            })
        )]
    );

    assert_eq!(
        active_enum_child::Model {
            id: 1,
            parent_id: 2,
            category: None,
            color: None,
            tea: None,
        }
        .find_related(ActiveEnum)
        .all(db)
        .await?,
        [active_enum::Model {
            id: 2,
            category: Some(Category::Small),
            color: Some(Color::White),
            tea: Some(Tea::BreakfastTea),
        }]
    );
    assert_eq!(
        ActiveEnumChild::find()
            .find_with_related(ActiveEnum)
            .all(db)
            .await?,
        [(
            active_enum_child::Model {
                id: 1,
                parent_id: 2,
                category: Some(Category::Big),
                color: Some(Color::Black),
                tea: Some(Tea::EverydayTea),
            },
            vec![active_enum::Model {
                id: 2,
                category: Some(Category::Small),
                color: Some(Color::White),
                tea: Some(Tea::BreakfastTea),
            }]
        )]
    );
    assert_eq!(
        ActiveEnumChild::find()
            .find_also_related(ActiveEnum)
            .all(db)
            .await?,
        [(
            active_enum_child::Model {
                id: 1,
                parent_id: 2,
                category: Some(Category::Big),
                color: Some(Color::Black),
                tea: Some(Tea::EverydayTea),
            },
            Some(active_enum::Model {
                id: 2,
                category: Some(Category::Small),
                color: Some(Color::White),
                tea: Some(Tea::BreakfastTea),
            })
        )]
    );

    Ok(())
}

pub async fn find_linked_active_enum(db: &DatabaseConnection) -> Result<(), DbErr> {
    assert_eq!(
        active_enum::Model {
            id: 2,
            category: None,
            color: None,
            tea: None,
        }
        .find_linked(active_enum::ActiveEnumChildLink)
        .all(db)
        .await?,
        [active_enum_child::Model {
            id: 1,
            parent_id: 2,
            category: Some(Category::Big),
            color: Some(Color::Black),
            tea: Some(Tea::EverydayTea),
        }]
    );
    assert_eq!(
        ActiveEnumEntity::find()
            .find_also_linked(active_enum::ActiveEnumChildLink)
            .all(db)
            .await?,
        [(
            active_enum::Model {
                id: 2,
                category: Some(Category::Small),
                color: Some(Color::White),
                tea: Some(Tea::BreakfastTea),
            },
            Some(active_enum_child::Model {
                id: 1,
                parent_id: 2,
                category: Some(Category::Big),
                color: Some(Color::Black),
                tea: Some(Tea::EverydayTea),
            })
        )]
    );
    assert_eq!(
        ActiveEnumEntity::find()
            .find_with_linked(active_enum::ActiveEnumChildLink)
            .all(db)
            .await?,
        [(
            active_enum::Model {
                id: 2,
                category: Some(Category::Small),
                color: Some(Color::White),
                tea: Some(Tea::BreakfastTea),
            },
            vec![active_enum_child::Model {
                id: 1,
                parent_id: 2,
                category: Some(Category::Big),
                color: Some(Color::Black),
                tea: Some(Tea::EverydayTea),
            }]
        )]
    );

    assert_eq!(
        active_enum_child::Model {
            id: 1,
            parent_id: 2,
            category: None,
            color: None,
            tea: None,
        }
        .find_linked(active_enum_child::ActiveEnumLink)
        .all(db)
        .await?,
        [active_enum::Model {
            id: 2,
            category: Some(Category::Small),
            color: Some(Color::White),
            tea: Some(Tea::BreakfastTea),
        }]
    );
    assert_eq!(
        ActiveEnumChild::find()
            .find_also_linked(active_enum_child::ActiveEnumLink)
            .all(db)
            .await?,
        [(
            active_enum_child::Model {
                id: 1,
                parent_id: 2,
                category: Some(Category::Big),
                color: Some(Color::Black),
                tea: Some(Tea::EverydayTea),
            },
            Some(active_enum::Model {
                id: 2,
                category: Some(Category::Small),
                color: Some(Color::White),
                tea: Some(Tea::BreakfastTea),
            })
        )]
    );
    assert_eq!(
        ActiveEnumChild::find()
            .find_with_linked(active_enum_child::ActiveEnumLink)
            .all(db)
            .await?,
        [(
            active_enum_child::Model {
                id: 1,
                parent_id: 2,
                category: Some(Category::Big),
                color: Some(Color::Black),
                tea: Some(Tea::EverydayTea),
            },
            vec![active_enum::Model {
                id: 2,
                category: Some(Category::Small),
                color: Some(Color::White),
                tea: Some(Tea::BreakfastTea),
            }]
        )]
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    pub use pretty_assertions::assert_eq;
    pub use sea_orm::{DbBackend, QueryTrait};

    #[test]
    fn active_enum_find_related() {
        let active_enum_model = active_enum::Model {
            id: 1,
            category: None,
            color: None,
            tea: None,
        };
        let _select = active_enum_model.find_related(ActiveEnumChild);
        #[cfg(any(feature = "sqlx-mysql", feature = "sqlx-sqlite"))]
        {
            assert_eq!(
                _select.build(DbBackend::Sqlite).to_string(),
                [
                    r#"SELECT "active_enum_child"."id", "active_enum_child"."parent_id", "active_enum_child"."category", "active_enum_child"."color", "active_enum_child"."tea""#,
                    r#"FROM "active_enum_child""#,
                    r#"INNER JOIN "active_enum" ON "active_enum"."id" = "active_enum_child"."parent_id""#,
                    r#"WHERE "active_enum"."id" = 1"#,
                ]
                .join(" ")
            );
            assert_eq!(
                _select.build(DbBackend::MySql).to_string(),
                [
                    "SELECT `active_enum_child`.`id`, `active_enum_child`.`parent_id`, `active_enum_child`.`category`, `active_enum_child`.`color`, `active_enum_child`.`tea`",
                    "FROM `active_enum_child`",
                    "INNER JOIN `active_enum` ON `active_enum`.`id` = `active_enum_child`.`parent_id`",
                    "WHERE `active_enum`.`id` = 1",
                ]
                .join(" ")
            );
        }
        #[cfg(feature = "sqlx-postgres")]
        assert_eq!(
            _select.build(DbBackend::Postgres).to_string(),
            [
                r#"SELECT "active_enum_child"."id", "active_enum_child"."parent_id", "active_enum_child"."category", "active_enum_child"."color", CAST("active_enum_child"."tea" AS text)"#,
                r#"FROM "public"."active_enum_child""#,
                r#"INNER JOIN "public"."active_enum" ON "active_enum"."id" = "active_enum_child"."parent_id""#,
                r#"WHERE "active_enum"."id" = 1"#,
            ]
            .join(" ")
        );

        let _select = ActiveEnumEntity::find().find_also_related(ActiveEnumChild);
        #[cfg(any(feature = "sqlx-mysql", feature = "sqlx-sqlite"))]
        {
            assert_eq!(
                _select
                    .build(DbBackend::Sqlite)
                    .to_string(),
                [
                    r#"SELECT "active_enum"."id" AS "A_id", "active_enum"."category" AS "A_category", "active_enum"."color" AS "A_color", "active_enum"."tea" AS "A_tea","#,
                    r#""active_enum_child"."id" AS "B_id", "active_enum_child"."parent_id" AS "B_parent_id", "active_enum_child"."category" AS "B_category", "active_enum_child"."color" AS "B_color", "active_enum_child"."tea" AS "B_tea""#,
                    r#"FROM "active_enum""#,
                    r#"LEFT JOIN "active_enum_child" ON "active_enum"."id" = "active_enum_child"."parent_id""#,
                ]
                .join(" ")
            );
            assert_eq!(
                _select
                    .build(DbBackend::MySql)
                    .to_string(),
                [
                    "SELECT `active_enum`.`id` AS `A_id`, `active_enum`.`category` AS `A_category`, `active_enum`.`color` AS `A_color`, `active_enum`.`tea` AS `A_tea`,",
                    "`active_enum_child`.`id` AS `B_id`, `active_enum_child`.`parent_id` AS `B_parent_id`, `active_enum_child`.`category` AS `B_category`, `active_enum_child`.`color` AS `B_color`, `active_enum_child`.`tea` AS `B_tea`",
                    "FROM `active_enum`",
                    "LEFT JOIN `active_enum_child` ON `active_enum`.`id` = `active_enum_child`.`parent_id`",
                ]
                .join(" ")
            );
        }
        #[cfg(feature = "sqlx-postgres")]
        assert_eq!(
            _select
                .build(DbBackend::Postgres)
                .to_string(),
            [
                r#"SELECT "active_enum"."id" AS "A_id", "active_enum"."category" AS "A_category", "active_enum"."color" AS "A_color", CAST("active_enum"."tea" AS text) AS "A_tea","#,
                r#""active_enum_child"."id" AS "B_id", "active_enum_child"."parent_id" AS "B_parent_id", "active_enum_child"."category" AS "B_category", "active_enum_child"."color" AS "B_color", CAST("active_enum_child"."tea" AS text) AS "B_tea""#,
                r#"FROM "public"."active_enum""#,
                r#"LEFT JOIN "public"."active_enum_child" ON "active_enum"."id" = "active_enum_child"."parent_id""#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn active_enum_find_linked() {
        let active_enum_model = active_enum::Model {
            id: 1,
            category: None,
            color: None,
            tea: None,
        };
        let _select = active_enum_model.find_linked(active_enum::ActiveEnumChildLink);
        #[cfg(any(feature = "sqlx-mysql", feature = "sqlx-sqlite"))]
        {
            assert_eq!(
                _select.build(DbBackend::Sqlite).to_string(),
                [
                    r#"SELECT "active_enum_child"."id", "active_enum_child"."parent_id", "active_enum_child"."category", "active_enum_child"."color", "active_enum_child"."tea""#,
                    r#"FROM "active_enum_child""#,
                    r#"INNER JOIN "active_enum" AS "r0" ON "r0"."id" = "active_enum_child"."parent_id""#,
                    r#"WHERE "r0"."id" = 1"#,
                ]
                .join(" ")
            );
            assert_eq!(
                _select.build(DbBackend::MySql).to_string(),
                [
                    "SELECT `active_enum_child`.`id`, `active_enum_child`.`parent_id`, `active_enum_child`.`category`, `active_enum_child`.`color`, `active_enum_child`.`tea`",
                    "FROM `active_enum_child`",
                    "INNER JOIN `active_enum` AS `r0` ON `r0`.`id` = `active_enum_child`.`parent_id`",
                    "WHERE `r0`.`id` = 1",
                ]
                .join(" ")
            );
        }
        #[cfg(feature = "sqlx-postgres")]
        assert_eq!(
            _select.build(DbBackend::Postgres).to_string(),
            [
                r#"SELECT "active_enum_child"."id", "active_enum_child"."parent_id", "active_enum_child"."category", "active_enum_child"."color", CAST("active_enum_child"."tea" AS text)"#,
                r#"FROM "public"."active_enum_child""#,
                r#"INNER JOIN "public"."active_enum" AS "r0" ON "r0"."id" = "active_enum_child"."parent_id""#,
                r#"WHERE "r0"."id" = 1"#,
            ]
            .join(" ")
        );

        let _select = ActiveEnumEntity::find().find_also_linked(active_enum::ActiveEnumChildLink);
        #[cfg(any(feature = "sqlx-mysql", feature = "sqlx-sqlite"))]
        {
            assert_eq!(
                _select
                    .build(DbBackend::Sqlite)
                    .to_string(),
                [
                    r#"SELECT "active_enum"."id" AS "A_id", "active_enum"."category" AS "A_category", "active_enum"."color" AS "A_color", "active_enum"."tea" AS "A_tea","#,
                    r#""r0"."id" AS "B_id", "r0"."parent_id" AS "B_parent_id", "r0"."category" AS "B_category", "r0"."color" AS "B_color", "r0"."tea" AS "B_tea""#,
                    r#"FROM "active_enum""#,
                    r#"LEFT JOIN "active_enum_child" AS "r0" ON "active_enum"."id" = "r0"."parent_id""#,
                ]
                .join(" ")
            );
            assert_eq!(
                _select
                    .build(DbBackend::MySql)
                    .to_string(),
                [
                    "SELECT `active_enum`.`id` AS `A_id`, `active_enum`.`category` AS `A_category`, `active_enum`.`color` AS `A_color`, `active_enum`.`tea` AS `A_tea`,",
                    "`r0`.`id` AS `B_id`, `r0`.`parent_id` AS `B_parent_id`, `r0`.`category` AS `B_category`, `r0`.`color` AS `B_color`, `r0`.`tea` AS `B_tea`",
                    "FROM `active_enum`",
                    "LEFT JOIN `active_enum_child` AS `r0` ON `active_enum`.`id` = `r0`.`parent_id`",
                ]
                .join(" ")
            );
        }
        #[cfg(feature = "sqlx-postgres")]
        assert_eq!(
            _select
                .build(DbBackend::Postgres)
                .to_string(),
            [
                r#"SELECT "active_enum"."id" AS "A_id", "active_enum"."category" AS "A_category", "active_enum"."color" AS "A_color", CAST("active_enum"."tea" AS text) AS "A_tea","#,
                r#""r0"."id" AS "B_id", "r0"."parent_id" AS "B_parent_id", "r0"."category" AS "B_category", "r0"."color" AS "B_color", CAST("r0"."tea" AS text) AS "B_tea""#,
                r#"FROM "public"."active_enum""#,
                r#"LEFT JOIN "public"."active_enum_child" AS "r0" ON "active_enum"."id" = "r0"."parent_id""#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn active_enum_child_find_related() {
        let active_enum_child_model = active_enum_child::Model {
            id: 1,
            parent_id: 2,
            category: None,
            color: None,
            tea: None,
        };
        let _select = active_enum_child_model.find_related(ActiveEnum);
        #[cfg(any(feature = "sqlx-mysql", feature = "sqlx-sqlite"))]
        {
            assert_eq!(
                _select.build(DbBackend::Sqlite).to_string(),
                [
                    r#"SELECT "active_enum"."id", "active_enum"."category", "active_enum"."color", "active_enum"."tea""#,
                    r#"FROM "active_enum""#,
                    r#"INNER JOIN "active_enum_child" ON "active_enum_child"."parent_id" = "active_enum"."id""#,
                    r#"WHERE "active_enum_child"."id" = 1"#,
                ]
                .join(" ")
            );
            assert_eq!(
                _select.build(DbBackend::MySql).to_string(),
                [
                    "SELECT `active_enum`.`id`, `active_enum`.`category`, `active_enum`.`color`, `active_enum`.`tea`",
                    "FROM `active_enum`",
                    "INNER JOIN `active_enum_child` ON `active_enum_child`.`parent_id` = `active_enum`.`id`",
                    "WHERE `active_enum_child`.`id` = 1",
                ]
                .join(" ")
            );
        }
        #[cfg(feature = "sqlx-postgres")]
        assert_eq!(
            _select.build(DbBackend::Postgres).to_string(),
            [
                r#"SELECT "active_enum"."id", "active_enum"."category", "active_enum"."color", CAST("active_enum"."tea" AS text)"#,
                r#"FROM "public"."active_enum""#,
                r#"INNER JOIN "public"."active_enum_child" ON "active_enum_child"."parent_id" = "active_enum"."id""#,
                r#"WHERE "active_enum_child"."id" = 1"#,
            ]
            .join(" ")
        );

        let _select = ActiveEnumChild::find().find_also_related(ActiveEnum);
        #[cfg(any(feature = "sqlx-mysql", feature = "sqlx-sqlite"))]
        {
            assert_eq!(
                _select
                    .build(DbBackend::Sqlite)
                    .to_string(),
                [
                    r#"SELECT "active_enum_child"."id" AS "A_id", "active_enum_child"."parent_id" AS "A_parent_id", "active_enum_child"."category" AS "A_category", "active_enum_child"."color" AS "A_color", "active_enum_child"."tea" AS "A_tea","#,
                    r#""active_enum"."id" AS "B_id", "active_enum"."category" AS "B_category", "active_enum"."color" AS "B_color", "active_enum"."tea" AS "B_tea""#,
                    r#"FROM "active_enum_child""#,
                    r#"LEFT JOIN "active_enum" ON "active_enum_child"."parent_id" = "active_enum"."id""#,
                ]
                .join(" ")
            );
            assert_eq!(
                _select
                    .build(DbBackend::MySql)
                    .to_string(),
                [
                    "SELECT `active_enum_child`.`id` AS `A_id`, `active_enum_child`.`parent_id` AS `A_parent_id`, `active_enum_child`.`category` AS `A_category`, `active_enum_child`.`color` AS `A_color`, `active_enum_child`.`tea` AS `A_tea`,",
                    "`active_enum`.`id` AS `B_id`, `active_enum`.`category` AS `B_category`, `active_enum`.`color` AS `B_color`, `active_enum`.`tea` AS `B_tea`",
                    "FROM `active_enum_child`",
                    "LEFT JOIN `active_enum` ON `active_enum_child`.`parent_id` = `active_enum`.`id`",
                ]
                .join(" ")
            );
        }
        #[cfg(feature = "sqlx-postgres")]
        assert_eq!(
            _select
                .build(DbBackend::Postgres)
                .to_string(),
            [
                r#"SELECT "active_enum_child"."id" AS "A_id", "active_enum_child"."parent_id" AS "A_parent_id", "active_enum_child"."category" AS "A_category", "active_enum_child"."color" AS "A_color", CAST("active_enum_child"."tea" AS text) AS "A_tea","#,
                r#""active_enum"."id" AS "B_id", "active_enum"."category" AS "B_category", "active_enum"."color" AS "B_color", CAST("active_enum"."tea" AS text) AS "B_tea""#,
                r#"FROM "public"."active_enum_child""#,
                r#"LEFT JOIN "public"."active_enum" ON "active_enum_child"."parent_id" = "active_enum"."id""#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn active_enum_child_find_linked() {
        let active_enum_child_model = active_enum_child::Model {
            id: 1,
            parent_id: 2,
            category: None,
            color: None,
            tea: None,
        };
        let _select = active_enum_child_model.find_linked(active_enum_child::ActiveEnumLink);
        #[cfg(any(feature = "sqlx-mysql", feature = "sqlx-sqlite"))]
        {
            assert_eq!(
                _select.build(DbBackend::Sqlite).to_string(),
                [
                    r#"SELECT "active_enum"."id", "active_enum"."category", "active_enum"."color", "active_enum"."tea""#,
                    r#"FROM "active_enum""#,
                    r#"INNER JOIN "active_enum_child" AS "r0" ON "r0"."parent_id" = "active_enum"."id""#,
                    r#"WHERE "r0"."id" = 1"#,
                ]
                .join(" ")
            );
            assert_eq!(
                _select.build(DbBackend::MySql).to_string(),
                [
                    "SELECT `active_enum`.`id`, `active_enum`.`category`, `active_enum`.`color`, `active_enum`.`tea`",
                    "FROM `active_enum`",
                    "INNER JOIN `active_enum_child` AS `r0` ON `r0`.`parent_id` = `active_enum`.`id`",
                    "WHERE `r0`.`id` = 1",
                ]
                .join(" ")
            );
        }
        #[cfg(feature = "sqlx-postgres")]
        assert_eq!(
            _select.build(DbBackend::Postgres).to_string(),
            [
                r#"SELECT "active_enum"."id", "active_enum"."category", "active_enum"."color", CAST("active_enum"."tea" AS text)"#,
                r#"FROM "public"."active_enum""#,
                r#"INNER JOIN "public"."active_enum_child" AS "r0" ON "r0"."parent_id" = "active_enum"."id""#,
                r#"WHERE "r0"."id" = 1"#,
            ]
            .join(" ")
        );

        let _select = ActiveEnumChild::find().find_also_linked(active_enum_child::ActiveEnumLink);
        #[cfg(any(feature = "sqlx-mysql", feature = "sqlx-sqlite"))]
        {
            assert_eq!(
                _select
                    .build(DbBackend::Sqlite)
                    .to_string(),
                [
                    r#"SELECT "active_enum_child"."id" AS "A_id", "active_enum_child"."parent_id" AS "A_parent_id", "active_enum_child"."category" AS "A_category", "active_enum_child"."color" AS "A_color", "active_enum_child"."tea" AS "A_tea","#,
                    r#""r0"."id" AS "B_id", "r0"."category" AS "B_category", "r0"."color" AS "B_color", "r0"."tea" AS "B_tea""#,
                    r#"FROM "active_enum_child""#,
                    r#"LEFT JOIN "active_enum" AS "r0" ON "active_enum_child"."parent_id" = "r0"."id""#,
                ]
                .join(" ")
            );
            assert_eq!(
                _select
                    .build(DbBackend::MySql)
                    .to_string(),
                [
                    "SELECT `active_enum_child`.`id` AS `A_id`, `active_enum_child`.`parent_id` AS `A_parent_id`, `active_enum_child`.`category` AS `A_category`, `active_enum_child`.`color` AS `A_color`, `active_enum_child`.`tea` AS `A_tea`,",
                    "`r0`.`id` AS `B_id`, `r0`.`category` AS `B_category`, `r0`.`color` AS `B_color`, `r0`.`tea` AS `B_tea`",
                    "FROM `active_enum_child`",
                    "LEFT JOIN `active_enum` AS `r0` ON `active_enum_child`.`parent_id` = `r0`.`id`",
                ]
                .join(" ")
            );
        }
        #[cfg(feature = "sqlx-postgres")]
        assert_eq!(
            _select
                .build(DbBackend::Postgres)
                .to_string(),
            [
                r#"SELECT "active_enum_child"."id" AS "A_id", "active_enum_child"."parent_id" AS "A_parent_id", "active_enum_child"."category" AS "A_category", "active_enum_child"."color" AS "A_color", CAST("active_enum_child"."tea" AS text) AS "A_tea","#,
                r#""r0"."id" AS "B_id", "r0"."category" AS "B_category", "r0"."color" AS "B_color", CAST("r0"."tea" AS text) AS "B_tea""#,
                r#"FROM "public"."active_enum_child""#,
                r#"LEFT JOIN "public"."active_enum" AS "r0" ON "active_enum_child"."parent_id" = "r0"."id""#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn create_enum_from() {
        use sea_orm::{Schema, Statement};

        let db_postgres = DbBackend::Postgres;
        let schema = Schema::new(db_postgres);

        assert_eq!(
            schema
                .create_enum_from_entity(active_enum::Entity)
                .iter()
                .map(|stmt| db_postgres.build(stmt))
                .collect::<Vec<_>>(),
            [Statement::from_string(
                db_postgres,
                r#"CREATE TYPE "tea" AS ENUM ('EverydayTea', 'BreakfastTea')"#.to_owned()
            ),]
        );

        assert_eq!(
            db_postgres.build(&schema.create_enum_from_active_enum::<Tea>()),
            Statement::from_string(
                db_postgres,
                r#"CREATE TYPE "tea" AS ENUM ('EverydayTea', 'BreakfastTea')"#.to_owned()
            )
        );
    }

    #[test]
    fn display_test() {
        assert_eq!(format!("{}", Tea::BreakfastTea), "BreakfastTea");
        assert_eq!(format!("{}", DisplayTea::BreakfastTea), "Breakfast");
        assert_eq!(format!("{}", Tea::EverydayTea), "EverydayTea");
        assert_eq!(format!("{}", DisplayTea::EverydayTea), "Everyday");
    }
}
