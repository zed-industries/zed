use crate::{
    join_tbl_on_condition, unpack_table_ref, ColumnTrait, EntityTrait, IdenStatic, Iterable,
    Linked, QuerySelect, Related, Select, SelectA, SelectB, SelectThree, SelectTwo, SelectTwoMany,
};
pub use sea_query::JoinType;
use sea_query::{Alias, Condition, Expr, IntoIden, SeaRc, SelectExpr};

impl<E> Select<E>
where
    E: EntityTrait,
{
    /// Left Join with a Related Entity.
    pub fn left_join<R>(self, _: R) -> Self
    where
        R: EntityTrait,
        E: Related<R>,
    {
        self.join_join(JoinType::LeftJoin, E::to(), E::via())
    }

    /// Right Join with a Related Entity.
    pub fn right_join<R>(self, _: R) -> Self
    where
        R: EntityTrait,
        E: Related<R>,
    {
        self.join_join(JoinType::RightJoin, E::to(), E::via())
    }

    /// Inner Join with a Related Entity.
    pub fn inner_join<R>(self, _: R) -> Self
    where
        R: EntityTrait,
        E: Related<R>,
    {
        self.join_join(JoinType::InnerJoin, E::to(), E::via())
    }

    /// Join with an Entity Related to me.
    pub fn reverse_join<R>(self, _: R) -> Self
    where
        R: EntityTrait + Related<E>,
    {
        self.join_rev(JoinType::InnerJoin, R::to())
    }

    /// Left Join with a Related Entity and select both Entity.
    pub fn find_also_related<R>(self, r: R) -> SelectTwo<E, R>
    where
        R: EntityTrait,
        E: Related<R>,
    {
        self.left_join(r).select_also(r)
    }

    /// Left Join with a Related Entity and select the related Entity as a `Vec`
    pub fn find_with_related<R>(self, r: R) -> SelectTwoMany<E, R>
    where
        R: EntityTrait,
        E: Related<R>,
    {
        self.left_join(r).select_with(r)
    }

    /// Left Join with a Linked Entity and select both Entity.
    pub fn find_also_linked<L, T>(self, l: L) -> SelectTwo<E, T>
    where
        L: Linked<FromEntity = E, ToEntity = T>,
        T: EntityTrait,
    {
        let mut slf = self;
        for (i, mut rel) in l.link().into_iter().enumerate() {
            let to_tbl = Alias::new(format!("r{i}")).into_iden();
            let from_tbl = if i > 0 {
                Alias::new(format!("r{}", i - 1)).into_iden()
            } else {
                unpack_table_ref(&rel.from_tbl)
            };
            let table_ref = rel.to_tbl;

            let mut condition = Condition::all().add(join_tbl_on_condition(
                SeaRc::clone(&from_tbl),
                SeaRc::clone(&to_tbl),
                rel.from_col,
                rel.to_col,
            ));
            if let Some(f) = rel.on_condition.take() {
                condition = condition.add(f(SeaRc::clone(&from_tbl), SeaRc::clone(&to_tbl)));
            }

            slf.query()
                .join_as(JoinType::LeftJoin, table_ref, to_tbl, condition);
        }
        slf = slf.apply_alias(SelectA.as_str());
        let mut select_two = SelectTwo::new_without_prepare(slf.query);
        for col in <T::Column as Iterable>::iter() {
            let alias = format!("{}{}", SelectB.as_str(), col.as_str());
            let expr = Expr::col((
                Alias::new(format!("r{}", l.link().len() - 1)).into_iden(),
                col.into_iden(),
            ));
            select_two.query().expr(SelectExpr {
                expr: col.select_as(expr),
                alias: Some(SeaRc::new(Alias::new(alias))),
                window: None,
            });
        }
        select_two
    }

    /// Left Join with a Linked Entity and select Entity as a `Vec`.
    pub fn find_with_linked<L, T>(self, l: L) -> SelectTwoMany<E, T>
    where
        L: Linked<FromEntity = E, ToEntity = T>,
        T: EntityTrait,
    {
        let mut slf = self;
        for (i, mut rel) in l.link().into_iter().enumerate() {
            let to_tbl = Alias::new(format!("r{i}")).into_iden();
            let from_tbl = if i > 0 {
                Alias::new(format!("r{}", i - 1)).into_iden()
            } else {
                unpack_table_ref(&rel.from_tbl)
            };
            let table_ref = rel.to_tbl;

            let mut condition = Condition::all().add(join_tbl_on_condition(
                SeaRc::clone(&from_tbl),
                SeaRc::clone(&to_tbl),
                rel.from_col,
                rel.to_col,
            ));
            if let Some(f) = rel.on_condition.take() {
                condition = condition.add(f(SeaRc::clone(&from_tbl), SeaRc::clone(&to_tbl)));
            }

            slf.query()
                .join_as(JoinType::LeftJoin, table_ref, to_tbl, condition);
        }
        slf = slf.apply_alias(SelectA.as_str());
        let mut select_two_many = SelectTwoMany::new_without_prepare(slf.query);
        for col in <T::Column as Iterable>::iter() {
            let alias = format!("{}{}", SelectB.as_str(), col.as_str());
            let expr = Expr::col((
                Alias::new(format!("r{}", l.link().len() - 1)).into_iden(),
                col.into_iden(),
            ));
            select_two_many.query().expr(SelectExpr {
                expr: col.select_as(expr),
                alias: Some(SeaRc::new(Alias::new(alias))),
                window: None,
            });
        }
        select_two_many
    }
}

impl<E, F> SelectTwo<E, F>
where
    E: EntityTrait,
    F: EntityTrait,
{
    /// Left Join with an Entity Related to the first Entity
    pub fn find_also_related<R>(self, r: R) -> SelectThree<E, F, R>
    where
        R: EntityTrait,
        E: Related<R>,
    {
        self.join_join(JoinType::LeftJoin, E::to(), E::via())
            .select_also(r)
    }

    /// Left Join with an Entity Related to the second Entity
    pub fn and_also_related<R>(self, r: R) -> SelectThree<E, F, R>
    where
        R: EntityTrait,
        F: Related<R>,
    {
        self.join_join(JoinType::LeftJoin, F::to(), F::via())
            .select_also(r)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_cfg::{cake, cake_filling, cake_filling_price, entity_linked, filling, fruit};
    use crate::{
        ColumnTrait, DbBackend, EntityTrait, ModelTrait, QueryFilter, QuerySelect, QueryTrait,
        RelationTrait,
    };
    use pretty_assertions::assert_eq;
    use sea_query::{Alias, ConditionType, Expr, IntoCondition, JoinType};

    #[test]
    fn join_1() {
        assert_eq!(
            cake::Entity::find()
                .left_join(fruit::Entity)
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id`, `cake`.`name` FROM `cake`",
                "LEFT JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id`",
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_2() {
        assert_eq!(
            cake::Entity::find()
                .inner_join(fruit::Entity)
                .filter(fruit::Column::Name.contains("cherry"))
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id`, `cake`.`name` FROM `cake`",
                "INNER JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id`",
                "WHERE `fruit`.`name` LIKE \'%cherry%\'"
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_3() {
        assert_eq!(
            fruit::Entity::find()
                .reverse_join(cake::Entity)
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `fruit`.`id`, `fruit`.`name`, `fruit`.`cake_id` FROM `fruit`",
                "INNER JOIN `cake` ON `cake`.`id` = `fruit`.`cake_id`",
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_4() {
        use crate::{Related, Select};

        let find_fruit: Select<fruit::Entity> = cake::Entity::find_related();
        assert_eq!(
            find_fruit
                .filter(cake::Column::Id.eq(11))
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `fruit`.`id`, `fruit`.`name`, `fruit`.`cake_id` FROM `fruit`",
                "INNER JOIN `cake` ON `cake`.`id` = `fruit`.`cake_id`",
                "WHERE `cake`.`id` = 11",
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_5() {
        let cake_model = cake::Model {
            id: 12,
            name: "".to_owned(),
        };

        assert_eq!(
            cake_model
                .find_related(fruit::Entity)
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `fruit`.`id`, `fruit`.`name`, `fruit`.`cake_id` FROM `fruit`",
                "INNER JOIN `cake` ON `cake`.`id` = `fruit`.`cake_id`",
                "WHERE `cake`.`id` = 12",
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_6() {
        assert_eq!(
            cake::Entity::find()
                .left_join(filling::Entity)
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id`, `cake`.`name` FROM `cake`",
                "LEFT JOIN `cake_filling` ON `cake`.`id` = `cake_filling`.`cake_id`",
                "LEFT JOIN `filling` ON `cake_filling`.`filling_id` = `filling`.`id`",
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_7() {
        use crate::{Related, Select};

        let find_filling: Select<filling::Entity> = cake::Entity::find_related();
        assert_eq!(
            find_filling.build(DbBackend::MySql).to_string(),
            [
                "SELECT `filling`.`id`, `filling`.`name`, `filling`.`vendor_id` FROM `filling`",
                "INNER JOIN `cake_filling` ON `cake_filling`.`filling_id` = `filling`.`id`",
                "INNER JOIN `cake` ON `cake`.`id` = `cake_filling`.`cake_id`",
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_8() {
        use crate::{Related, Select};

        let find_cake_filling_price: Select<cake_filling_price::Entity> =
            cake_filling::Entity::find_related();
        assert_eq!(
            find_cake_filling_price.build(DbBackend::Postgres).to_string(),
            [
                r#"SELECT "cake_filling_price"."cake_id", "cake_filling_price"."filling_id", "cake_filling_price"."price""#,
                r#"FROM "public"."cake_filling_price""#,
                r#"INNER JOIN "cake_filling" ON"#,
                r#""cake_filling"."cake_id" = "cake_filling_price"."cake_id" AND"#,
                r#""cake_filling"."filling_id" = "cake_filling_price"."filling_id""#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_9() {
        use crate::{Related, Select};

        let find_cake_filling: Select<cake_filling::Entity> =
            cake_filling_price::Entity::find_related();
        assert_eq!(
            find_cake_filling.build(DbBackend::Postgres).to_string(),
            [
                r#"SELECT "cake_filling"."cake_id", "cake_filling"."filling_id""#,
                r#"FROM "cake_filling""#,
                r#"INNER JOIN "public"."cake_filling_price" ON"#,
                r#""cake_filling_price"."cake_id" = "cake_filling"."cake_id" AND"#,
                r#""cake_filling_price"."filling_id" = "cake_filling"."filling_id""#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_10() {
        let cake_model = cake::Model {
            id: 12,
            name: "".to_owned(),
        };

        assert_eq!(
            cake_model
                .find_linked(entity_linked::CakeToFilling)
                .build(DbBackend::MySql)
                .to_string(),
            [
                r#"SELECT `filling`.`id`, `filling`.`name`, `filling`.`vendor_id`"#,
                r#"FROM `filling`"#,
                r#"INNER JOIN `cake_filling` AS `r0` ON `r0`.`filling_id` = `filling`.`id`"#,
                r#"INNER JOIN `cake` AS `r1` ON `r1`.`id` = `r0`.`cake_id`"#,
                r#"WHERE `r1`.`id` = 12"#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_11() {
        let cake_model = cake::Model {
            id: 18,
            name: "".to_owned(),
        };

        assert_eq!(
            cake_model
                .find_linked(entity_linked::CakeToFillingVendor)
                .build(DbBackend::MySql)
                .to_string(),
            [
                r#"SELECT `vendor`.`id`, `vendor`.`name`"#,
                r#"FROM `vendor`"#,
                r#"INNER JOIN `filling` AS `r0` ON `r0`.`vendor_id` = `vendor`.`id`"#,
                r#"INNER JOIN `cake_filling` AS `r1` ON `r1`.`filling_id` = `r0`.`id`"#,
                r#"INNER JOIN `cake` AS `r2` ON `r2`.`id` = `r1`.`cake_id`"#,
                r#"WHERE `r2`.`id` = 18"#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_12() {
        assert_eq!(
            cake::Entity::find()
                .find_also_linked(entity_linked::CakeToFilling)
                .build(DbBackend::MySql)
                .to_string(),
            [
                r#"SELECT `cake`.`id` AS `A_id`, `cake`.`name` AS `A_name`,"#,
                r#"`r1`.`id` AS `B_id`, `r1`.`name` AS `B_name`, `r1`.`vendor_id` AS `B_vendor_id`"#,
                r#"FROM `cake`"#,
                r#"LEFT JOIN `cake_filling` AS `r0` ON `cake`.`id` = `r0`.`cake_id`"#,
                r#"LEFT JOIN `filling` AS `r1` ON `r0`.`filling_id` = `r1`.`id`"#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_13() {
        assert_eq!(
            cake::Entity::find()
                .find_also_linked(entity_linked::CakeToFillingVendor)
                .build(DbBackend::MySql)
                .to_string(),
            [
                r#"SELECT `cake`.`id` AS `A_id`, `cake`.`name` AS `A_name`,"#,
                r#"`r2`.`id` AS `B_id`, `r2`.`name` AS `B_name`"#,
                r#"FROM `cake`"#,
                r#"LEFT JOIN `cake_filling` AS `r0` ON `cake`.`id` = `r0`.`cake_id`"#,
                r#"LEFT JOIN `filling` AS `r1` ON `r0`.`filling_id` = `r1`.`id`"#,
                r#"LEFT JOIN `vendor` AS `r2` ON `r1`.`vendor_id` = `r2`.`id`"#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_14() {
        assert_eq!(
            cake::Entity::find()
                .join(JoinType::LeftJoin, cake::Relation::TropicalFruit.def())
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id`, `cake`.`name` FROM `cake`",
                "LEFT JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id` AND `fruit`.`name` LIKE '%tropical%'",
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_15() {
        let cake_model = cake::Model {
            id: 18,
            name: "".to_owned(),
        };

        assert_eq!(
            cake_model
                .find_linked(entity_linked::CheeseCakeToFillingVendor)
                .build(DbBackend::MySql)
                .to_string(),
            [
                r#"SELECT `vendor`.`id`, `vendor`.`name`"#,
                r#"FROM `vendor`"#,
                r#"INNER JOIN `filling` AS `r0` ON `r0`.`vendor_id` = `vendor`.`id`"#,
                r#"INNER JOIN `cake_filling` AS `r1` ON `r1`.`filling_id` = `r0`.`id`"#,
                r#"INNER JOIN `cake` AS `r2` ON `r2`.`id` = `r1`.`cake_id` AND `r2`.`name` LIKE '%cheese%'"#,
                r#"WHERE `r2`.`id` = 18"#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_16() {
        let cake_model = cake::Model {
            id: 18,
            name: "".to_owned(),
        };
        assert_eq!(
            cake_model
                .find_linked(entity_linked::JoinWithoutReverse)
                .build(DbBackend::MySql)
                .to_string(),
            [
                r#"SELECT `vendor`.`id`, `vendor`.`name`"#,
                r#"FROM `vendor`"#,
                r#"INNER JOIN `filling` AS `r0` ON `r0`.`vendor_id` = `vendor`.`id`"#,
                r#"INNER JOIN `cake_filling` AS `r1` ON `r1`.`filling_id` = `r0`.`id`"#,
                r#"INNER JOIN `cake_filling` AS `r2` ON `r2`.`cake_id` = `r1`.`id` AND `r2`.`name` LIKE '%cheese%'"#,
                r#"WHERE `r2`.`id` = 18"#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_17() {
        assert_eq!(
            cake::Entity::find()
                .find_also_linked(entity_linked::CheeseCakeToFillingVendor)
                .build(DbBackend::MySql)
                .to_string(),
            [
                r#"SELECT `cake`.`id` AS `A_id`, `cake`.`name` AS `A_name`,"#,
                r#"`r2`.`id` AS `B_id`, `r2`.`name` AS `B_name`"#,
                r#"FROM `cake`"#,
                r#"LEFT JOIN `cake_filling` AS `r0` ON `cake`.`id` = `r0`.`cake_id` AND `cake`.`name` LIKE '%cheese%'"#,
                r#"LEFT JOIN `filling` AS `r1` ON `r0`.`filling_id` = `r1`.`id`"#,
                r#"LEFT JOIN `vendor` AS `r2` ON `r1`.`vendor_id` = `r2`.`id`"#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_18() {
        assert_eq!(
            cake::Entity::find()
                .find_also_linked(entity_linked::JoinWithoutReverse)
                .build(DbBackend::MySql)
                .to_string(),
                [
                    r#"SELECT `cake`.`id` AS `A_id`, `cake`.`name` AS `A_name`,"#,
                    r#"`r2`.`id` AS `B_id`, `r2`.`name` AS `B_name`"#,
                    r#"FROM `cake`"#,
                    r#"LEFT JOIN `cake` AS `r0` ON `cake_filling`.`cake_id` = `r0`.`id` AND `cake_filling`.`name` LIKE '%cheese%'"#,
                    r#"LEFT JOIN `filling` AS `r1` ON `r0`.`filling_id` = `r1`.`id`"#,
                    r#"LEFT JOIN `vendor` AS `r2` ON `r1`.`vendor_id` = `r2`.`id`"#,
                ]
                .join(" ")
        );
    }

    #[test]
    fn join_19() {
        assert_eq!(
            cake::Entity::find()
                .join(JoinType::LeftJoin, cake::Relation::TropicalFruit.def())
                .join(
                    JoinType::LeftJoin,
                    cake_filling::Relation::Cake
                        .def()
                        .rev()
                        .on_condition(|_left, right| {
                            Expr::col((right, cake_filling::Column::CakeId))
                                .gt(10)
                                .into_condition()
                        })
                )
                .join(
                    JoinType::LeftJoin,
                    cake_filling::Relation::Filling
                        .def()
                        .on_condition(|_left, right| {
                            Expr::col((right, filling::Column::Name))
                                .like("%lemon%")
                                .into_condition()
                        })
                )
                .join(JoinType::LeftJoin, filling::Relation::Vendor.def())
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id`, `cake`.`name` FROM `cake`",
                "LEFT JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id` AND `fruit`.`name` LIKE '%tropical%'",
                "LEFT JOIN `cake_filling` ON `cake`.`id` = `cake_filling`.`cake_id` AND `cake_filling`.`cake_id` > 10",
                "LEFT JOIN `filling` ON `cake_filling`.`filling_id` = `filling`.`id` AND `filling`.`name` LIKE '%lemon%'",
                "LEFT JOIN `vendor` ON `filling`.`vendor_id` = `vendor`.`id`",
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_20() {
        assert_eq!(
            cake::Entity::find()
                .column_as(
                    Expr::col((Alias::new("fruit_alias"), fruit::Column::Name)),
                    "fruit_name"
                )
                .join_as(
                    JoinType::LeftJoin,
                    cake::Relation::Fruit
                        .def()
                        .on_condition(|_left, right| {
                            Expr::col((right, fruit::Column::Name))
                                .like("%tropical%")
                                .into_condition()
                        }),
                    Alias::new("fruit_alias")
                )
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id`, `cake`.`name`, `fruit_alias`.`name` AS `fruit_name` FROM `cake`",
                "LEFT JOIN `fruit` AS `fruit_alias` ON `cake`.`id` = `fruit_alias`.`cake_id` AND `fruit_alias`.`name` LIKE '%tropical%'",
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_21() {
        assert_eq!(
            cake::Entity::find()
                .column_as(
                    Expr::col((Alias::new("cake_filling_alias"), cake_filling::Column::CakeId)),
                    "cake_filling_cake_id"
                )
                .join(JoinType::LeftJoin, cake::Relation::TropicalFruit.def())
                .join_as_rev(
                    JoinType::LeftJoin,
                    cake_filling::Relation::Cake
                        .def()
                        .on_condition(|left, _right| {
                            Expr::col((left, cake_filling::Column::CakeId))
                                .gt(10)
                                .into_condition()
                        }),
                    Alias::new("cake_filling_alias")
                )
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id`, `cake`.`name`, `cake_filling_alias`.`cake_id` AS `cake_filling_cake_id` FROM `cake`",
                "LEFT JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id` AND `fruit`.`name` LIKE '%tropical%'",
                "LEFT JOIN `cake_filling` AS `cake_filling_alias` ON `cake_filling_alias`.`cake_id` = `cake`.`id` AND `cake_filling_alias`.`cake_id` > 10",
            ]
            .join(" ")
        );
    }

    #[test]
    fn join_22() {
        assert_eq!(
            cake::Entity::find()
                .column_as(
                    Expr::col((Alias::new("cake_filling_alias"), cake_filling::Column::CakeId)),
                    "cake_filling_cake_id"
                )
                .join(JoinType::LeftJoin, cake::Relation::OrTropicalFruit.def())
                .join_as_rev(
                    JoinType::LeftJoin,
                    cake_filling::Relation::Cake
                        .def()
                        .condition_type(ConditionType::Any)
                        .on_condition(|left, _right| {
                            Expr::col((left, cake_filling::Column::CakeId))
                                .gt(10)
                                .into_condition()
                        }),
                    Alias::new("cake_filling_alias")
                )
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id`, `cake`.`name`, `cake_filling_alias`.`cake_id` AS `cake_filling_cake_id` FROM `cake`",
                "LEFT JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id` OR `fruit`.`name` LIKE '%tropical%'",
                "LEFT JOIN `cake_filling` AS `cake_filling_alias` ON `cake_filling_alias`.`cake_id` = `cake`.`id` OR `cake_filling_alias`.`cake_id` > 10",
            ]
            .join(" ")
        );
    }
}
