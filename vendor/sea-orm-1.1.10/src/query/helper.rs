use crate::{
    ColumnAsExpr, ColumnTrait, EntityTrait, Identity, IntoIdentity, IntoSimpleExpr, Iterable,
    ModelTrait, PrimaryKeyToColumn, RelationDef,
};
use sea_query::{
    Alias, ConditionType, Expr, Iden, IntoCondition, IntoIden, LockBehavior, LockType,
    NullOrdering, SeaRc, SelectExpr, SelectStatement, SimpleExpr, TableRef,
};
pub use sea_query::{Condition, ConditionalStatement, DynIden, JoinType, Order, OrderedStatement};

use sea_query::IntoColumnRef;

// LINT: when the column does not appear in tables selected from
// LINT: when there is a group by clause, but some columns don't have aggregate functions
// LINT: when the join table or column does not exists
/// Abstract API for performing queries
pub trait QuerySelect: Sized {
    #[allow(missing_docs)]
    type QueryStatement;

    /// Add the select SQL statement
    fn query(&mut self) -> &mut SelectStatement;

    /// Clear the selection list
    fn select_only(mut self) -> Self {
        self.query().clear_selects();
        self
    }

    /// Add a select column
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .column(cake::Column::Name)
    ///         .build(DbBackend::Postgres)
    ///         .to_string(),
    ///     r#"SELECT "cake"."name" FROM "cake""#
    /// );
    /// ```
    ///
    /// Enum column will be casted into text (PostgreSQL only)
    ///
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::lunch_set, DbBackend};
    ///
    /// assert_eq!(
    ///     lunch_set::Entity::find()
    ///         .select_only()
    ///         .column(lunch_set::Column::Tea)
    ///         .build(DbBackend::Postgres)
    ///         .to_string(),
    ///     r#"SELECT CAST("lunch_set"."tea" AS text) FROM "lunch_set""#
    /// );
    /// assert_eq!(
    ///     lunch_set::Entity::find()
    ///         .select_only()
    ///         .column(lunch_set::Column::Tea)
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     r#"SELECT `lunch_set`.`tea` FROM `lunch_set`"#
    /// );
    /// ```
    fn column<C>(mut self, col: C) -> Self
    where
        C: ColumnTrait,
    {
        self.query().expr(col.select_as(col.into_expr()));
        self
    }

    /// Add a select column with alias
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .column_as(cake::Column::Id.count(), "count")
    ///         .build(DbBackend::Postgres)
    ///         .to_string(),
    ///     r#"SELECT COUNT("cake"."id") AS "count" FROM "cake""#
    /// );
    /// ```
    fn column_as<C, I>(mut self, col: C, alias: I) -> Self
    where
        C: ColumnAsExpr,
        I: IntoIdentity,
    {
        self.query().expr(SelectExpr {
            expr: col.into_column_as_expr(),
            alias: Some(SeaRc::new(alias.into_identity())),
            window: None,
        });
        self
    }

    /// Select columns
    ///
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .columns([cake::Column::Id, cake::Column::Name])
    ///         .build(DbBackend::Postgres)
    ///         .to_string(),
    ///     r#"SELECT "cake"."id", "cake"."name" FROM "cake""#
    /// );
    /// ```
    ///
    /// Conditionally select all columns expect a specific column
    ///
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .columns(cake::Column::iter().filter(|col| match col {
    ///             cake::Column::Id => false,
    ///             _ => true,
    ///         }))
    ///         .build(DbBackend::Postgres)
    ///         .to_string(),
    ///     r#"SELECT "cake"."name" FROM "cake""#
    /// );
    /// ```
    ///
    /// Enum column will be casted into text (PostgreSQL only)
    ///
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::lunch_set, DbBackend};
    ///
    /// assert_eq!(
    ///     lunch_set::Entity::find()
    ///         .select_only()
    ///         .columns([lunch_set::Column::Name, lunch_set::Column::Tea])
    ///         .build(DbBackend::Postgres)
    ///         .to_string(),
    ///     r#"SELECT "lunch_set"."name", CAST("lunch_set"."tea" AS text) FROM "lunch_set""#
    /// );
    /// assert_eq!(
    ///     lunch_set::Entity::find()
    ///         .select_only()
    ///         .columns([lunch_set::Column::Name, lunch_set::Column::Tea])
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     r#"SELECT `lunch_set`.`name`, `lunch_set`.`tea` FROM `lunch_set`"#
    /// );
    /// ```
    fn columns<C, I>(mut self, cols: I) -> Self
    where
        C: ColumnTrait,
        I: IntoIterator<Item = C>,
    {
        for col in cols.into_iter() {
            self = self.column(col);
        }
        self
    }

    /// Add an offset expression. Passing in None would remove the offset.
    ///
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .offset(10)
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` OFFSET 10"
    /// );
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .offset(Some(10))
    ///         .offset(Some(20))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` OFFSET 20"
    /// );
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .offset(10)
    ///         .offset(None)
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake`"
    /// );
    /// ```
    fn offset<T>(mut self, offset: T) -> Self
    where
        T: Into<Option<u64>>,
    {
        if let Some(offset) = offset.into() {
            self.query().offset(offset);
        } else {
            self.query().reset_offset();
        }
        self
    }

    /// Add a limit expression. Passing in None would remove the limit.
    ///
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .limit(10)
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` LIMIT 10"
    /// );
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .limit(Some(10))
    ///         .limit(Some(20))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` LIMIT 20"
    /// );
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .limit(10)
    ///         .limit(None)
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake`"
    /// );
    /// ```
    fn limit<T>(mut self, limit: T) -> Self
    where
        T: Into<Option<u64>>,
    {
        if let Some(limit) = limit.into() {
            self.query().limit(limit);
        } else {
            self.query().reset_limit();
        }
        self
    }

    /// Add a group by column
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .column(cake::Column::Name)
    ///         .group_by(cake::Column::Name)
    ///         .build(DbBackend::Postgres)
    ///         .to_string(),
    ///     r#"SELECT "cake"."name" FROM "cake" GROUP BY "cake"."name""#
    /// );
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .column_as(cake::Column::Id.count(), "count")
    ///         .column_as(cake::Column::Id.sum(), "sum_of_id")
    ///         .group_by(cake::Column::Name)
    ///         .build(DbBackend::Postgres)
    ///         .to_string(),
    ///     r#"SELECT COUNT("cake"."id") AS "count", SUM("cake"."id") AS "sum_of_id" FROM "cake" GROUP BY "cake"."name""#
    /// );
    /// ```
    fn group_by<C>(mut self, col: C) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query().add_group_by([col.into_simple_expr()]);
        self
    }

    /// Add an AND HAVING expression
    /// ```
    /// use sea_orm::{sea_query::{Alias, Expr}, entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .having(cake::Column::Id.eq(4))
    ///         .having(cake::Column::Id.eq(5))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` HAVING `cake`.`id` = 4 AND `cake`.`id` = 5"
    /// );
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .column_as(cake::Column::Id.count(), "count")
    ///         .column_as(cake::Column::Id.sum(), "sum_of_id")
    ///         .group_by(cake::Column::Name)
    ///         .having(Expr::col(Alias::new("count")).gt(6))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT COUNT(`cake`.`id`) AS `count`, SUM(`cake`.`id`) AS `sum_of_id` FROM `cake` GROUP BY `cake`.`name` HAVING `count` > 6"
    /// );
    /// ```
    fn having<F>(mut self, filter: F) -> Self
    where
        F: IntoCondition,
    {
        self.query().cond_having(filter.into_condition());
        self
    }

    /// Add a DISTINCT expression
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    /// struct Input {
    ///     name: Option<String>,
    /// }
    /// let input = Input {
    ///     name: Some("cheese".to_owned()),
    /// };
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(
    ///             Condition::all().add_option(input.name.map(|n| cake::Column::Name.contains(&n)))
    ///         )
    ///         .distinct()
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT DISTINCT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`name` LIKE '%cheese%'"
    /// );
    /// ```
    fn distinct(mut self) -> Self {
        self.query().distinct();
        self
    }

    /// Add a DISTINCT ON expression
    /// NOTE: this function is only supported by `sqlx-postgres`
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    /// struct Input {
    ///     name: Option<String>,
    /// }
    /// let input = Input {
    ///     name: Some("cheese".to_owned()),
    /// };
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(
    ///             Condition::all().add_option(input.name.map(|n| cake::Column::Name.contains(&n)))
    ///         )
    ///         .distinct_on([(cake::Entity, cake::Column::Name)])
    ///         .build(DbBackend::Postgres)
    ///         .to_string(),
    ///     r#"SELECT DISTINCT ON ("cake"."name") "cake"."id", "cake"."name" FROM "cake" WHERE "cake"."name" LIKE '%cheese%'"#
    /// );
    /// ```
    fn distinct_on<T, I>(mut self, cols: I) -> Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = T>,
    {
        self.query().distinct_on(cols);
        self
    }

    #[doc(hidden)]
    fn join_join(mut self, join: JoinType, rel: RelationDef, via: Option<RelationDef>) -> Self {
        if let Some(via) = via {
            self = self.join(join, via)
        }
        self.join(join, rel)
    }

    #[doc(hidden)]
    fn join_join_rev(mut self, join: JoinType, rel: RelationDef, via: Option<RelationDef>) -> Self {
        self = self.join_rev(join, rel);
        if let Some(via) = via {
            self = self.join_rev(join, via)
        }
        self
    }

    /// Join via [`RelationDef`].
    fn join(mut self, join: JoinType, rel: RelationDef) -> Self {
        self.query()
            .join(join, rel.to_tbl.clone(), join_condition(rel));
        self
    }

    /// Join via [`RelationDef`] but in reverse direction.
    /// Assume when there exist a relation A to B.
    /// You can reverse join B from A.
    fn join_rev(mut self, join: JoinType, rel: RelationDef) -> Self {
        self.query()
            .join(join, rel.from_tbl.clone(), join_condition(rel));
        self
    }

    /// Join via [`RelationDef`] with table alias.
    fn join_as<I>(mut self, join: JoinType, mut rel: RelationDef, alias: I) -> Self
    where
        I: IntoIden,
    {
        let alias = alias.into_iden();
        rel.to_tbl = rel.to_tbl.alias(SeaRc::clone(&alias));
        self.query()
            .join(join, rel.to_tbl.clone(), join_condition(rel));
        self
    }

    /// Join via [`RelationDef`] with table alias but in reverse direction.
    /// Assume when there exist a relation A to B.
    /// You can reverse join B from A.
    fn join_as_rev<I>(mut self, join: JoinType, mut rel: RelationDef, alias: I) -> Self
    where
        I: IntoIden,
    {
        let alias = alias.into_iden();
        rel.from_tbl = rel.from_tbl.alias(SeaRc::clone(&alias));
        self.query()
            .join(join, rel.from_tbl.clone(), join_condition(rel));
        self
    }

    /// Select lock
    fn lock(mut self, lock_type: LockType) -> Self {
        self.query().lock(lock_type);
        self
    }

    /// Select lock shared
    fn lock_shared(mut self) -> Self {
        self.query().lock_shared();
        self
    }

    /// Select lock exclusive
    fn lock_exclusive(mut self) -> Self {
        self.query().lock_exclusive();
        self
    }

    /// Row locking with behavior (if supported).
    ///
    /// See [`SelectStatement::lock_with_behavior`](https://docs.rs/sea-query/*/sea_query/query/struct.SelectStatement.html#method.lock_with_behavior).
    fn lock_with_behavior(mut self, r#type: LockType, behavior: LockBehavior) -> Self {
        self.query().lock_with_behavior(r#type, behavior);
        self
    }

    /// Add an expression to the select expression list.
    /// ```
    /// use sea_orm::sea_query::Expr;
    /// use sea_orm::{entity::*, tests_cfg::cake, DbBackend, QuerySelect, QueryTrait};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .expr(Expr::col((cake::Entity, cake::Column::Id)))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id` FROM `cake`"
    /// );
    /// ```
    fn expr<T>(mut self, expr: T) -> Self
    where
        T: Into<SelectExpr>,
    {
        self.query().expr(expr);
        self
    }

    /// Add select expressions from vector of [`SelectExpr`].
    /// ```
    /// use sea_orm::sea_query::Expr;
    /// use sea_orm::{entity::*, tests_cfg::cake, DbBackend, QuerySelect, QueryTrait};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .exprs([
    ///             Expr::col((cake::Entity, cake::Column::Id)),
    ///             Expr::col((cake::Entity, cake::Column::Name)),
    ///         ])
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake`"
    /// );
    /// ```
    fn exprs<T, I>(mut self, exprs: I) -> Self
    where
        T: Into<SelectExpr>,
        I: IntoIterator<Item = T>,
    {
        self.query().exprs(exprs);
        self
    }

    /// Select column.
    /// ```
    /// use sea_orm::sea_query::{Alias, Expr, Func};
    /// use sea_orm::{entity::*, tests_cfg::cake, DbBackend, QuerySelect, QueryTrait};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .expr_as(
    ///             Func::upper(Expr::col((cake::Entity, cake::Column::Name))),
    ///             "name_upper"
    ///         )
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name`, UPPER(`cake`.`name`) AS `name_upper` FROM `cake`"
    /// );
    /// ```
    fn expr_as<T, A>(mut self, expr: T, alias: A) -> Self
    where
        T: Into<SimpleExpr>,
        A: IntoIdentity,
    {
        self.query().expr_as(expr, alias.into_identity());
        self
    }

    /// Same as `expr_as`. Here for legacy reasons.
    ///
    /// Select column.
    ///
    /// ```
    /// use sea_orm::sea_query::{Alias, Expr, Func};
    /// use sea_orm::{entity::*, tests_cfg::cake, DbBackend, QuerySelect, QueryTrait};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .expr_as(
    ///             Func::upper(Expr::col((cake::Entity, cake::Column::Name))),
    ///             "name_upper"
    ///         )
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name`, UPPER(`cake`.`name`) AS `name_upper` FROM `cake`"
    /// );
    /// ```
    fn expr_as_<T, A>(mut self, expr: T, alias: A) -> Self
    where
        T: Into<SimpleExpr>,
        A: IntoIdentity,
    {
        self.query().expr_as(expr, alias.into_identity());
        self
    }

    /// Shorthand of `expr_as(Expr::col((T, C)), A)`.
    ///
    /// ```
    /// use sea_orm::sea_query::{Alias, Expr, Func};
    /// use sea_orm::{entity::*, tests_cfg::cake, DbBackend, QuerySelect, QueryTrait};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .tbl_col_as((cake::Entity, cake::Column::Name), "cake_name")
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`name` AS `cake_name` FROM `cake`"
    /// );
    /// ```
    fn tbl_col_as<T, C, A>(mut self, (tbl, col): (T, C), alias: A) -> Self
    where
        T: IntoIden + 'static,
        C: IntoIden + 'static,
        A: IntoIdentity,
    {
        self.query()
            .expr_as(Expr::col((tbl, col)), alias.into_identity());
        self
    }
}

// LINT: when the column does not appear in tables selected from
/// Performs ORDER BY operations
pub trait QueryOrder: Sized {
    #[allow(missing_docs)]
    type QueryStatement: OrderedStatement;

    /// Add the query to perform an ORDER BY operation
    fn query(&mut self) -> &mut SelectStatement;

    /// Add an order_by expression
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .order_by(cake::Column::Id, Order::Asc)
    ///         .order_by(cake::Column::Name, Order::Desc)
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` ORDER BY `cake`.`id` ASC, `cake`.`name` DESC"
    /// );
    /// ```
    fn order_by<C>(mut self, col: C, ord: Order) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query().order_by_expr(col.into_simple_expr(), ord);
        self
    }

    /// Add an order_by expression (ascending)
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .order_by_asc(cake::Column::Id)
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` ORDER BY `cake`.`id` ASC"
    /// );
    /// ```
    fn order_by_asc<C>(mut self, col: C) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query()
            .order_by_expr(col.into_simple_expr(), Order::Asc);
        self
    }

    /// Add an order_by expression (descending)
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .order_by_desc(cake::Column::Id)
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` ORDER BY `cake`.`id` DESC"
    /// );
    /// ```
    fn order_by_desc<C>(mut self, col: C) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query()
            .order_by_expr(col.into_simple_expr(), Order::Desc);
        self
    }

    /// Add an order_by expression with nulls ordering option
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    /// use sea_query::NullOrdering;
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .order_by_with_nulls(cake::Column::Id, Order::Asc, NullOrdering::First)
    ///         .build(DbBackend::Postgres)
    ///         .to_string(),
    ///     r#"SELECT "cake"."id", "cake"."name" FROM "cake" ORDER BY "cake"."id" ASC NULLS FIRST"#
    /// );
    /// ```
    fn order_by_with_nulls<C>(mut self, col: C, ord: Order, nulls: NullOrdering) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query()
            .order_by_expr_with_nulls(col.into_simple_expr(), ord, nulls);
        self
    }
}

// LINT: when the column does not appear in tables selected from
/// Perform a FILTER opertation on a statement
pub trait QueryFilter: Sized {
    #[allow(missing_docs)]
    type QueryStatement: ConditionalStatement;

    /// Add the query to perform a FILTER on
    fn query(&mut self) -> &mut Self::QueryStatement;

    /// Add an AND WHERE expression
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Id.eq(4))
    ///         .filter(cake::Column::Id.eq(5))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`id` = 4 AND `cake`.`id` = 5"
    /// );
    /// ```
    ///
    /// Add a condition tree.
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(
    ///             Condition::any()
    ///                 .add(cake::Column::Id.eq(4))
    ///                 .add(cake::Column::Id.eq(5))
    ///         )
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`id` = 4 OR `cake`.`id` = 5"
    /// );
    /// ```
    ///
    /// Like above, but using the `IN` operator.
    ///
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Id.is_in([4, 5]))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`id` IN (4, 5)"
    /// );
    /// ```
    ///
    /// Like above, but using the `ANY` operator. Postgres only.
    ///
    /// ```
    /// use sea_orm::sea_query::{extension::postgres::PgFunc, Expr};
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(Expr::col((cake::Entity, cake::Column::Id)).eq(PgFunc::any(vec![4, 5])))
    ///         .build(DbBackend::Postgres)
    ///         .to_string(),
    ///     r#"SELECT "cake"."id", "cake"."name" FROM "cake" WHERE "cake"."id" = ANY(ARRAY [4,5])"#
    /// );
    /// ```
    ///
    /// Add a runtime-built condition tree.
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    /// struct Input {
    ///     name: Option<String>,
    /// }
    /// let input = Input {
    ///     name: Some("cheese".to_owned()),
    /// };
    ///
    /// let mut conditions = Condition::all();
    /// if let Some(name) = input.name {
    ///     conditions = conditions.add(cake::Column::Name.contains(&name));
    /// }
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(conditions)
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`name` LIKE '%cheese%'"
    /// );
    /// ```
    ///
    /// Add a runtime-built condition tree, functional-way.
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    /// struct Input {
    ///     name: Option<String>,
    /// }
    /// let input = Input {
    ///     name: Some("cheese".to_owned()),
    /// };
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(
    ///             Condition::all().add_option(input.name.map(|n| cake::Column::Name.contains(&n)))
    ///         )
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`name` LIKE '%cheese%'"
    /// );
    /// ```
    ///
    /// A slightly more complex example.
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, sea_query::Expr, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(
    ///             Condition::all()
    ///                 .add(
    ///                     Condition::all()
    ///                         .not()
    ///                         .add(Expr::val(1).eq(1))
    ///                         .add(Expr::val(2).eq(2))
    ///                 )
    ///                 .add(
    ///                     Condition::any()
    ///                         .add(Expr::val(3).eq(3))
    ///                         .add(Expr::val(4).eq(4))
    ///                 )
    ///         )
    ///         .build(DbBackend::Postgres)
    ///         .to_string(),
    ///     r#"SELECT "cake"."id", "cake"."name" FROM "cake" WHERE (NOT (1 = 1 AND 2 = 2)) AND (3 = 3 OR 4 = 4)"#
    /// );
    /// ```
    /// Use a sea_query expression
    /// ```
    /// use sea_orm::{entity::*, query::*, sea_query::Expr, tests_cfg::fruit, DbBackend};
    ///
    /// assert_eq!(
    ///     fruit::Entity::find()
    ///         .filter(Expr::col(fruit::Column::CakeId).is_null())
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `fruit`.`id`, `fruit`.`name`, `fruit`.`cake_id` FROM `fruit` WHERE `cake_id` IS NULL"
    /// );
    /// ```
    fn filter<F>(mut self, filter: F) -> Self
    where
        F: IntoCondition,
    {
        self.query().cond_where(filter.into_condition());
        self
    }

    /// Apply a where condition using the model's primary key
    fn belongs_to<M>(mut self, model: &M) -> Self
    where
        M: ModelTrait,
    {
        for key in <M::Entity as EntityTrait>::PrimaryKey::iter() {
            let col = key.into_column();
            self = self.filter(col.eq(model.get(col)));
        }
        self
    }

    /// Perform a check to determine table belongs to a Model through it's name alias
    fn belongs_to_tbl_alias<M>(mut self, model: &M, tbl_alias: &str) -> Self
    where
        M: ModelTrait,
    {
        for key in <M::Entity as EntityTrait>::PrimaryKey::iter() {
            let col = key.into_column();
            let expr = Expr::col((Alias::new(tbl_alias), col)).eq(model.get(col));
            self = self.filter(expr);
        }
        self
    }
}

pub(crate) fn join_condition(mut rel: RelationDef) -> Condition {
    // Use table alias (if any) to construct the join condition
    let from_tbl = match unpack_table_alias(&rel.from_tbl) {
        Some(alias) => alias,
        None => unpack_table_ref(&rel.from_tbl),
    };
    let to_tbl = match unpack_table_alias(&rel.to_tbl) {
        Some(alias) => alias,
        None => unpack_table_ref(&rel.to_tbl),
    };
    let owner_keys = rel.from_col;
    let foreign_keys = rel.to_col;

    let mut condition = match rel.condition_type {
        ConditionType::All => Condition::all(),
        ConditionType::Any => Condition::any(),
    };

    condition = condition.add(join_tbl_on_condition(
        SeaRc::clone(&from_tbl),
        SeaRc::clone(&to_tbl),
        owner_keys,
        foreign_keys,
    ));
    if let Some(f) = rel.on_condition.take() {
        condition = condition.add(f(from_tbl, to_tbl));
    }

    condition
}

pub(crate) fn join_tbl_on_condition(
    from_tbl: SeaRc<dyn Iden>,
    to_tbl: SeaRc<dyn Iden>,
    owner_keys: Identity,
    foreign_keys: Identity,
) -> Condition {
    let mut cond = Condition::all();
    for (owner_key, foreign_key) in owner_keys.into_iter().zip(foreign_keys.into_iter()) {
        cond = cond.add(
            Expr::col((SeaRc::clone(&from_tbl), owner_key))
                .equals((SeaRc::clone(&to_tbl), foreign_key)),
        );
    }
    cond
}

pub(crate) fn unpack_table_ref(table_ref: &TableRef) -> DynIden {
    match table_ref {
        TableRef::Table(tbl)
        | TableRef::SchemaTable(_, tbl)
        | TableRef::DatabaseSchemaTable(_, _, tbl)
        | TableRef::TableAlias(tbl, _)
        | TableRef::SchemaTableAlias(_, tbl, _)
        | TableRef::DatabaseSchemaTableAlias(_, _, tbl, _)
        | TableRef::SubQuery(_, tbl)
        | TableRef::ValuesList(_, tbl)
        | TableRef::FunctionCall(_, tbl) => SeaRc::clone(tbl),
    }
}

pub(crate) fn unpack_table_alias(table_ref: &TableRef) -> Option<DynIden> {
    match table_ref {
        TableRef::Table(_)
        | TableRef::SchemaTable(_, _)
        | TableRef::DatabaseSchemaTable(_, _, _)
        | TableRef::SubQuery(_, _)
        | TableRef::ValuesList(_, _) => None,
        TableRef::TableAlias(_, alias)
        | TableRef::SchemaTableAlias(_, _, alias)
        | TableRef::DatabaseSchemaTableAlias(_, _, _, alias)
        | TableRef::FunctionCall(_, alias) => Some(SeaRc::clone(alias)),
    }
}
