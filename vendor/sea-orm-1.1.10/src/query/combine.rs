use crate::{
    ColumnTrait, EntityTrait, IdenStatic, Iterable, QueryTrait, Select, SelectThree, SelectTwo,
    SelectTwoMany,
};
use core::marker::PhantomData;
use sea_query::{Alias, ColumnRef, Iden, Order, SeaRc, SelectExpr, SelectStatement, SimpleExpr};

macro_rules! select_def {
    ( $ident: ident, $str: expr ) => {
        /// Implements the traits [Iden] and [IdenStatic] for a type
        #[derive(Debug, Clone, Copy)]
        pub struct $ident;

        impl Iden for $ident {
            fn unquoted(&self, s: &mut dyn std::fmt::Write) {
                write!(s, "{}", self.as_str()).unwrap();
            }
        }

        impl IdenStatic for $ident {
            fn as_str(&self) -> &str {
                $str
            }
        }
    };
}

select_def!(SelectA, "A_");
select_def!(SelectB, "B_");
select_def!(SelectC, "C_");

impl<E> Select<E>
where
    E: EntityTrait,
{
    pub(crate) fn apply_alias(mut self, pre: &str) -> Self {
        self.query().exprs_mut_for_each(|sel| {
            match &sel.alias {
                Some(alias) => {
                    let alias = format!("{}{}", pre, alias.to_string().as_str());
                    sel.alias = Some(SeaRc::new(Alias::new(alias)));
                }
                None => {
                    let col = match &sel.expr {
                        SimpleExpr::Column(col_ref) => match &col_ref {
                            ColumnRef::Column(col)
                            | ColumnRef::TableColumn(_, col)
                            | ColumnRef::SchemaTableColumn(_, _, col) => col,
                            ColumnRef::Asterisk | ColumnRef::TableAsterisk(_) => {
                                panic!("cannot apply alias for Column with asterisk")
                            }
                        },
                        SimpleExpr::AsEnum(_, simple_expr) => match simple_expr.as_ref() {
                            SimpleExpr::Column(col_ref) => match &col_ref {
                                ColumnRef::Column(col)
                                | ColumnRef::TableColumn(_, col)
                                | ColumnRef::SchemaTableColumn(_, _, col) => col,
                                ColumnRef::Asterisk | ColumnRef::TableAsterisk(_) => {
                                    panic!("cannot apply alias for AsEnum with asterisk")
                                }
                            },
                            _ => {
                                panic!("cannot apply alias for AsEnum with expr other than Column")
                            }
                        },
                        _ => panic!("cannot apply alias for expr other than Column or AsEnum"),
                    };
                    let alias = format!("{}{}", pre, col.to_string().as_str());
                    sel.alias = Some(SeaRc::new(Alias::new(alias)));
                }
            };
        });
        self
    }

    /// Selects extra Entity and returns it together with the Entity from `Self`
    pub fn select_also<F>(mut self, _: F) -> SelectTwo<E, F>
    where
        F: EntityTrait,
    {
        self = self.apply_alias(SelectA.as_str());
        SelectTwo::new(self.into_query())
    }

    /// Makes a SELECT operation in conjunction to another relation
    pub fn select_with<F>(mut self, _: F) -> SelectTwoMany<E, F>
    where
        F: EntityTrait,
    {
        self = self.apply_alias(SelectA.as_str());
        SelectTwoMany::new(self.into_query())
    }
}

impl<E, F> SelectTwo<E, F>
where
    E: EntityTrait,
    F: EntityTrait,
{
    /// Selects extra Entity and returns it together with the Entities from `Self`
    pub fn select_also<G>(self, _: G) -> SelectThree<E, F, G>
    where
        G: EntityTrait,
    {
        SelectThree::new(self.into_query())
    }

    pub(crate) fn new(query: SelectStatement) -> Self {
        Self::new_without_prepare(query).prepare_select()
    }

    pub(crate) fn new_without_prepare(query: SelectStatement) -> Self {
        Self {
            query,
            entity: PhantomData,
        }
    }

    fn prepare_select(mut self) -> Self {
        prepare_select_col::<F, _, _>(&mut self, SelectB);
        self
    }
}

impl<E, F> SelectTwoMany<E, F>
where
    E: EntityTrait,
    F: EntityTrait,
{
    pub(crate) fn new(query: SelectStatement) -> Self {
        Self::new_without_prepare(query)
            .prepare_select()
            .prepare_order_by()
    }

    pub(crate) fn new_without_prepare(query: SelectStatement) -> Self {
        Self {
            query,
            entity: PhantomData,
        }
    }

    fn prepare_select(mut self) -> Self {
        prepare_select_col::<F, _, _>(&mut self, SelectB);
        self
    }

    fn prepare_order_by(mut self) -> Self {
        for col in <E::PrimaryKey as Iterable>::iter() {
            self.query.order_by((E::default(), col), Order::Asc);
        }
        self
    }
}

impl<E, F, G> SelectThree<E, F, G>
where
    E: EntityTrait,
    F: EntityTrait,
    G: EntityTrait,
{
    pub(crate) fn new(query: SelectStatement) -> Self {
        Self::new_without_prepare(query).prepare_select()
    }

    pub(crate) fn new_without_prepare(query: SelectStatement) -> Self {
        Self {
            query,
            entity: PhantomData,
        }
    }

    fn prepare_select(mut self) -> Self {
        prepare_select_col::<G, _, _>(&mut self, SelectC);
        self
    }
}

fn prepare_select_col<F, S, A>(selector: &mut S, alias: A)
where
    F: EntityTrait,
    S: QueryTrait<QueryStatement = SelectStatement>,
    A: IdenStatic,
{
    for col in <F::Column as Iterable>::iter() {
        let alias = format!("{}{}", alias.as_str(), col.as_str());
        selector.query().expr(SelectExpr {
            expr: col.select_as(col.into_expr()),
            alias: Some(SeaRc::new(Alias::new(alias))),
            window: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_cfg::{cake, fruit};
    use crate::{ColumnTrait, DbBackend, EntityTrait, QueryFilter, QuerySelect, QueryTrait};

    #[test]
    fn alias_1() {
        assert_eq!(
            cake::Entity::find()
                .column_as(cake::Column::Id, "B")
                .apply_alias("A_")
                .build(DbBackend::MySql)
                .to_string(),
            "SELECT `cake`.`id` AS `A_id`, `cake`.`name` AS `A_name`, `cake`.`id` AS `A_B` FROM `cake`",
        );
    }

    #[test]
    fn select_also_1() {
        assert_eq!(
            cake::Entity::find()
                .left_join(fruit::Entity)
                .select_also(fruit::Entity)
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id` AS `A_id`, `cake`.`name` AS `A_name`,",
                "`fruit`.`id` AS `B_id`, `fruit`.`name` AS `B_name`, `fruit`.`cake_id` AS `B_cake_id`",
                "FROM `cake` LEFT JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id`",
            ].join(" ")
        );
    }

    #[test]
    fn select_with_1() {
        assert_eq!(
            cake::Entity::find()
                .left_join(fruit::Entity)
                .select_with(fruit::Entity)
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id` AS `A_id`, `cake`.`name` AS `A_name`,",
                "`fruit`.`id` AS `B_id`, `fruit`.`name` AS `B_name`, `fruit`.`cake_id` AS `B_cake_id`",
                "FROM `cake` LEFT JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id`",
                "ORDER BY `cake`.`id` ASC",
            ].join(" ")
        );
    }

    #[test]
    fn select_also_2() {
        assert_eq!(
            cake::Entity::find()
                .left_join(fruit::Entity)
                .select_also(fruit::Entity)
                .filter(cake::Column::Id.eq(1))
                .filter(fruit::Column::Id.eq(2))
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id` AS `A_id`, `cake`.`name` AS `A_name`,",
                "`fruit`.`id` AS `B_id`, `fruit`.`name` AS `B_name`, `fruit`.`cake_id` AS `B_cake_id`",
                "FROM `cake` LEFT JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id`",
                "WHERE `cake`.`id` = 1 AND `fruit`.`id` = 2",
            ].join(" ")
        );
    }

    #[test]
    fn select_with_2() {
        assert_eq!(
            cake::Entity::find()
                .left_join(fruit::Entity)
                .select_with(fruit::Entity)
                .filter(cake::Column::Id.eq(1))
                .filter(fruit::Column::Id.eq(2))
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id` AS `A_id`, `cake`.`name` AS `A_name`,",
                "`fruit`.`id` AS `B_id`, `fruit`.`name` AS `B_name`, `fruit`.`cake_id` AS `B_cake_id`",
                "FROM `cake` LEFT JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id`",
                "WHERE `cake`.`id` = 1 AND `fruit`.`id` = 2",
                "ORDER BY `cake`.`id` ASC",
            ].join(" ")
        );
    }
}
