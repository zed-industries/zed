use crate::{
    ConnectionTrait, DbErr, EntityTrait, FromQueryResult, Identity, IdentityOf, IntoIdentity,
    PartialModelTrait, PrimaryKeyToColumn, QueryOrder, QuerySelect, Select, SelectModel,
    SelectThree, SelectThreeModel, SelectTwo, SelectTwoModel, SelectorTrait,
};
use sea_query::{
    Condition, DynIden, Expr, IntoValueTuple, Order, SeaRc, SelectStatement, SimpleExpr, Value,
    ValueTuple,
};
use std::marker::PhantomData;
use strum::IntoEnumIterator as Iterable;

#[cfg(feature = "with-json")]
use crate::JsonValue;

/// Cursor pagination
#[derive(Debug, Clone)]
pub struct Cursor<S>
where
    S: SelectorTrait,
{
    query: SelectStatement,
    table: DynIden,
    order_columns: Identity,
    secondary_order_by: Vec<(DynIden, Identity)>,
    first: Option<u64>,
    last: Option<u64>,
    before: Option<ValueTuple>,
    after: Option<ValueTuple>,
    sort_asc: bool,
    is_result_reversed: bool,
    phantom: PhantomData<S>,
}

impl<S> Cursor<S>
where
    S: SelectorTrait,
{
    /// Create a new cursor
    pub fn new<C>(query: SelectStatement, table: DynIden, order_columns: C) -> Self
    where
        C: IntoIdentity,
    {
        Self {
            query,
            table,
            order_columns: order_columns.into_identity(),
            last: None,
            first: None,
            after: None,
            before: None,
            sort_asc: true,
            is_result_reversed: false,
            phantom: PhantomData,
            secondary_order_by: Default::default(),
        }
    }

    /// Filter paginated result with corresponding column less than the input value
    pub fn before<V>(&mut self, values: V) -> &mut Self
    where
        V: IntoValueTuple,
    {
        self.before = Some(values.into_value_tuple());
        self
    }

    /// Filter paginated result with corresponding column greater than the input value
    pub fn after<V>(&mut self, values: V) -> &mut Self
    where
        V: IntoValueTuple,
    {
        self.after = Some(values.into_value_tuple());
        self
    }

    fn apply_filters(&mut self) -> &mut Self {
        if let Some(values) = self.after.clone() {
            let condition = self.apply_filter(values, |c, v| {
                let exp = Expr::col((SeaRc::clone(&self.table), SeaRc::clone(c)));
                if self.sort_asc {
                    exp.gt(v)
                } else {
                    exp.lt(v)
                }
            });
            self.query.cond_where(condition);
        }

        if let Some(values) = self.before.clone() {
            let condition = self.apply_filter(values, |c, v| {
                let exp = Expr::col((SeaRc::clone(&self.table), SeaRc::clone(c)));
                if self.sort_asc {
                    exp.lt(v)
                } else {
                    exp.gt(v)
                }
            });
            self.query.cond_where(condition);
        }

        self
    }

    fn apply_filter<F>(&self, values: ValueTuple, f: F) -> Condition
    where
        F: Fn(&DynIden, Value) -> SimpleExpr,
    {
        match (&self.order_columns, values) {
            (Identity::Unary(c1), ValueTuple::One(v1)) => Condition::all().add(f(c1, v1)),
            (Identity::Binary(c1, c2), ValueTuple::Two(v1, v2)) => Condition::any()
                .add(
                    Condition::all()
                        .add(
                            Expr::col((SeaRc::clone(&self.table), SeaRc::clone(c1))).eq(v1.clone()),
                        )
                        .add(f(c2, v2)),
                )
                .add(f(c1, v1)),
            (Identity::Ternary(c1, c2, c3), ValueTuple::Three(v1, v2, v3)) => Condition::any()
                .add(
                    Condition::all()
                        .add(
                            Expr::col((SeaRc::clone(&self.table), SeaRc::clone(c1))).eq(v1.clone()),
                        )
                        .add(
                            Expr::col((SeaRc::clone(&self.table), SeaRc::clone(c2))).eq(v2.clone()),
                        )
                        .add(f(c3, v3)),
                )
                .add(
                    Condition::all()
                        .add(
                            Expr::col((SeaRc::clone(&self.table), SeaRc::clone(c1))).eq(v1.clone()),
                        )
                        .add(f(c2, v2)),
                )
                .add(f(c1, v1)),
            (Identity::Many(col_vec), ValueTuple::Many(val_vec))
                if col_vec.len() == val_vec.len() =>
            {
                // The length of `col_vec` and `val_vec` should be equal and is denoted by "n".
                //
                // The elements of `col_vec` and `val_vec` are denoted by:
                //   - `col_vec`: "col_1", "col_2", ..., "col_n-1", "col_n"
                //   - `val_vec`: "val_1", "val_2", ..., "val_n-1", "val_n"
                //
                // The general form of the where condition should have "n" number of inner-AND-condition chained by an outer-OR-condition.
                // The "n"-th inner-AND-condition should have exactly "n" number of column value expressions,
                // to construct the expression we take the first "n" number of column and value from the respected vector.
                //   - if it's not the last element, then we construct a "col_1 = val_1" equal expression
                //   - otherwise, for the last element, we should construct a "col_n > val_n" greater than or "col_n < val_n" less than expression.
                // i.e.
                // WHERE
                //   (col_1 = val_1 AND col_2 = val_2 AND ... AND col_n > val_n)
                //   OR (col_1 = val_1 AND col_2 = val_2 AND ... AND col_n-1 > val_n-1)
                //   OR (col_1 = val_1 AND col_2 = val_2 AND ... AND col_n-2 > val_n-2)
                //   OR ...
                //   OR (col_1 = val_1 AND col_2 > val_2)
                //   OR (col_1 > val_1)

                // Counting from 1 to "n" (inclusive) but in reverse, i.e. n, n-1, ..., 2, 1
                (1..=col_vec.len())
                    .rev()
                    .fold(Condition::any(), |cond_any, n| {
                        // Construct the inner-AND-condition
                        let inner_cond_all =
                            // Take the first "n" elements from the column and value vector respectively
                            col_vec.iter().zip(val_vec.iter()).enumerate().take(n).fold(
                                Condition::all(),
                                |inner_cond_all, (i, (col, val))| {
                                    let val = val.clone();
                                    // Construct a equal expression,
                                    // except for the last one being greater than or less than expression
                                    let expr = if i != (n - 1) {
                                        Expr::col((SeaRc::clone(&self.table), SeaRc::clone(col)))
                                            .eq(val)
                                    } else {
                                        f(col, val)
                                    };
                                    // Chain it with AND operator
                                    inner_cond_all.add(expr)
                                },
                            );
                        // Chain inner-AND-condition with OR operator
                        cond_any.add(inner_cond_all)
                    })
            }
            _ => panic!("column arity mismatch"),
        }
    }

    /// Use ascending sort order
    pub fn asc(&mut self) -> &mut Self {
        self.sort_asc = true;
        self
    }

    /// Use descending sort order
    pub fn desc(&mut self) -> &mut Self {
        self.sort_asc = false;
        self
    }

    /// Limit result set to only first N rows in ascending order of the order by column
    pub fn first(&mut self, num_rows: u64) -> &mut Self {
        self.last = None;
        self.first = Some(num_rows);
        self
    }

    /// Limit result set to only last N rows in ascending order of the order by column
    pub fn last(&mut self, num_rows: u64) -> &mut Self {
        self.first = None;
        self.last = Some(num_rows);
        self
    }

    fn resolve_sort_order(&mut self) -> Order {
        let should_reverse_order = self.last.is_some();
        self.is_result_reversed = should_reverse_order;

        if (self.sort_asc && !should_reverse_order) || (!self.sort_asc && should_reverse_order) {
            Order::Asc
        } else {
            Order::Desc
        }
    }

    fn apply_limit(&mut self) -> &mut Self {
        if let Some(num_rows) = self.first {
            self.query.limit(num_rows);
        } else if let Some(num_rows) = self.last {
            self.query.limit(num_rows);
        }

        self
    }

    fn apply_order_by(&mut self) -> &mut Self {
        self.query.clear_order_by();
        let ord = self.resolve_sort_order();

        let query = &mut self.query;
        let order = |query: &mut SelectStatement, col| {
            query.order_by((SeaRc::clone(&self.table), SeaRc::clone(col)), ord.clone());
        };
        match &self.order_columns {
            Identity::Unary(c1) => {
                order(query, c1);
            }
            Identity::Binary(c1, c2) => {
                order(query, c1);
                order(query, c2);
            }
            Identity::Ternary(c1, c2, c3) => {
                order(query, c1);
                order(query, c2);
                order(query, c3);
            }
            Identity::Many(vec) => {
                for col in vec.iter() {
                    order(query, col);
                }
            }
        }

        for (tbl, col) in self.secondary_order_by.iter().cloned() {
            if let Identity::Unary(c1) = col {
                query.order_by((tbl, c1), ord.clone());
            }
        }

        self
    }

    /// Fetch the paginated result
    pub async fn all<C>(&mut self, db: &C) -> Result<Vec<S::Item>, DbErr>
    where
        C: ConnectionTrait,
    {
        self.apply_limit();
        self.apply_order_by();
        self.apply_filters();

        let stmt = db.get_database_backend().build(&self.query);
        let rows = db.query_all(stmt).await?;
        let mut buffer = Vec::with_capacity(rows.len());
        for row in rows.into_iter() {
            buffer.push(S::from_raw_query_result(row)?);
        }
        if self.is_result_reversed {
            buffer.reverse()
        }
        Ok(buffer)
    }

    /// Construct a [Cursor] that fetch any custom struct
    pub fn into_model<M>(self) -> Cursor<SelectModel<M>>
    where
        M: FromQueryResult,
    {
        Cursor {
            query: self.query,
            table: self.table,
            order_columns: self.order_columns,
            last: self.last,
            first: self.first,
            after: self.after,
            before: self.before,
            sort_asc: self.sort_asc,
            is_result_reversed: self.is_result_reversed,
            phantom: PhantomData,
            secondary_order_by: self.secondary_order_by,
        }
    }

    /// Return a [Selector] from `Self` that wraps a [SelectModel] with a [PartialModel](PartialModelTrait)
    pub fn into_partial_model<M>(self) -> Cursor<SelectModel<M>>
    where
        M: PartialModelTrait,
    {
        M::select_cols(QuerySelect::select_only(self)).into_model::<M>()
    }

    /// Construct a [Cursor] that fetch JSON value
    #[cfg(feature = "with-json")]
    pub fn into_json(self) -> Cursor<SelectModel<JsonValue>> {
        Cursor {
            query: self.query,
            table: self.table,
            order_columns: self.order_columns,
            last: self.last,
            first: self.first,
            after: self.after,
            before: self.before,
            sort_asc: self.sort_asc,
            is_result_reversed: self.is_result_reversed,
            phantom: PhantomData,
            secondary_order_by: self.secondary_order_by,
        }
    }

    /// Set the cursor ordering for another table when dealing with SelectTwo
    pub fn set_secondary_order_by(&mut self, tbl_col: Vec<(DynIden, Identity)>) -> &mut Self {
        self.secondary_order_by = tbl_col;
        self
    }
}

impl<S> QuerySelect for Cursor<S>
where
    S: SelectorTrait,
{
    type QueryStatement = SelectStatement;

    fn query(&mut self) -> &mut SelectStatement {
        &mut self.query
    }
}

impl<S> QueryOrder for Cursor<S>
where
    S: SelectorTrait,
{
    type QueryStatement = SelectStatement;

    fn query(&mut self) -> &mut SelectStatement {
        &mut self.query
    }
}

/// A trait for any type that can be turn into a cursor
pub trait CursorTrait {
    /// Select operation
    type Selector: SelectorTrait + Send + Sync;
}

impl<E, M> CursorTrait for Select<E>
where
    E: EntityTrait<Model = M>,
    M: FromQueryResult + Sized + Send + Sync,
{
    type Selector = SelectModel<M>;
}

impl<E, M> Select<E>
where
    E: EntityTrait<Model = M>,
    M: FromQueryResult + Sized + Send + Sync,
{
    /// Convert into a cursor
    pub fn cursor_by<C>(self, order_columns: C) -> Cursor<SelectModel<M>>
    where
        C: IntoIdentity,
    {
        Cursor::new(self.query, SeaRc::new(E::default()), order_columns)
    }
}

impl<E, F, M, N> CursorTrait for SelectTwo<E, F>
where
    E: EntityTrait<Model = M>,
    F: EntityTrait<Model = N>,
    M: FromQueryResult + Sized + Send + Sync,
    N: FromQueryResult + Sized + Send + Sync,
{
    type Selector = SelectTwoModel<M, N>;
}

impl<E, F, G, M, N, O> CursorTrait for SelectThree<E, F, G>
where
    E: EntityTrait<Model = M>,
    F: EntityTrait<Model = N>,
    G: EntityTrait<Model = O>,
    M: FromQueryResult + Sized + Send + Sync,
    N: FromQueryResult + Sized + Send + Sync,
    O: FromQueryResult + Sized + Send + Sync,
{
    type Selector = SelectThreeModel<M, N, O>;
}

impl<E, F, M, N> SelectTwo<E, F>
where
    E: EntityTrait<Model = M>,
    F: EntityTrait<Model = N>,
    M: FromQueryResult + Sized + Send + Sync,
    N: FromQueryResult + Sized + Send + Sync,
{
    /// Convert into a cursor using column of first entity
    pub fn cursor_by<C>(self, order_columns: C) -> Cursor<SelectTwoModel<M, N>>
    where
        C: IdentityOf<E>,
    {
        let primary_keys: Vec<(DynIden, Identity)> = <F::PrimaryKey as Iterable>::iter()
            .map(|pk| {
                (
                    SeaRc::new(F::default()),
                    Identity::Unary(SeaRc::new(pk.into_column())),
                )
            })
            .collect();
        let mut cursor = Cursor::new(
            self.query,
            SeaRc::new(E::default()),
            order_columns.identity_of(),
        );
        cursor.set_secondary_order_by(primary_keys);
        cursor
    }

    /// Convert into a cursor using column of second entity
    pub fn cursor_by_other<C>(self, order_columns: C) -> Cursor<SelectTwoModel<M, N>>
    where
        C: IdentityOf<F>,
    {
        let primary_keys: Vec<(DynIden, Identity)> = <E::PrimaryKey as Iterable>::iter()
            .map(|pk| {
                (
                    SeaRc::new(E::default()),
                    Identity::Unary(SeaRc::new(pk.into_column())),
                )
            })
            .collect();
        let mut cursor = Cursor::new(
            self.query,
            SeaRc::new(F::default()),
            order_columns.identity_of(),
        );
        cursor.set_secondary_order_by(primary_keys);
        cursor
    }
}

impl<E, F, G, M, N, O> SelectThree<E, F, G>
where
    E: EntityTrait<Model = M>,
    F: EntityTrait<Model = N>,
    G: EntityTrait<Model = O>,
    M: FromQueryResult + Sized + Send + Sync,
    N: FromQueryResult + Sized + Send + Sync,
    O: FromQueryResult + Sized + Send + Sync,
{
    /// Convert into a cursor using column of first entity
    pub fn cursor_by<C>(self, order_columns: C) -> Cursor<SelectThreeModel<M, N, O>>
    where
        C: IdentityOf<E>,
    {
        let mut cursor = Cursor::new(
            self.query,
            SeaRc::new(E::default()),
            order_columns.identity_of(),
        );
        {
            let primary_keys: Vec<(DynIden, Identity)> = <F::PrimaryKey as Iterable>::iter()
                .map(|pk| {
                    (
                        SeaRc::new(F::default()),
                        Identity::Unary(SeaRc::new(pk.into_column())),
                    )
                })
                .collect();
            cursor.set_secondary_order_by(primary_keys);
        }
        {
            let primary_keys: Vec<(DynIden, Identity)> = <G::PrimaryKey as Iterable>::iter()
                .map(|pk| {
                    (
                        SeaRc::new(G::default()),
                        Identity::Unary(SeaRc::new(pk.into_column())),
                    )
                })
                .collect();
            cursor.set_secondary_order_by(primary_keys);
        }
        cursor
    }
}

#[cfg(test)]
#[cfg(feature = "mock")]
mod tests {
    use crate::entity::prelude::*;
    use crate::tests_cfg::*;
    use crate::{DbBackend, MockDatabase, Statement, Transaction};
    use pretty_assertions::assert_eq;

    #[smol_potat::test]
    async fn first_2_before_10() -> Result<(), DbErr> {
        use fruit::*;

        let models = [
            Model {
                id: 1,
                name: "Blueberry".into(),
                cake_id: Some(1),
            },
            Model {
                id: 2,
                name: "Raspberry".into(),
                cake_id: Some(1),
            },
        ];

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        assert_eq!(
            Entity::find()
                .cursor_by(Column::Id)
                .before(10)
                .first(2)
                .all(&db)
                .await?,
            models
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "fruit"."id", "fruit"."name", "fruit"."cake_id""#,
                    r#"FROM "fruit""#,
                    r#"WHERE "fruit"."id" < $1"#,
                    r#"ORDER BY "fruit"."id" ASC"#,
                    r#"LIMIT $2"#,
                ]
                .join(" ")
                .as_str(),
                [10_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn last_2_after_10_desc() -> Result<(), DbErr> {
        use fruit::*;

        let mut models = [
            Model {
                id: 1,
                name: "Blueberry".into(),
                cake_id: Some(1),
            },
            Model {
                id: 2,
                name: "Raspberry".into(),
                cake_id: Some(1),
            },
        ];

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        models.reverse();

        assert_eq!(
            Entity::find()
                .cursor_by(Column::Id)
                .after(10)
                .last(2)
                .desc()
                .all(&db)
                .await?,
            models
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "fruit"."id", "fruit"."name", "fruit"."cake_id""#,
                    r#"FROM "fruit""#,
                    r#"WHERE "fruit"."id" < $1"#,
                    r#"ORDER BY "fruit"."id" ASC"#,
                    r#"LIMIT $2"#,
                ]
                .join(" ")
                .as_str(),
                [10_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn first_2_before_10_also_related_select() -> Result<(), DbErr> {
        let models = [
            (
                cake::Model {
                    id: 1,
                    name: "Blueberry Cheese Cake".into(),
                },
                Some(fruit::Model {
                    id: 9,
                    name: "Blueberry".into(),
                    cake_id: Some(1),
                }),
            ),
            (
                cake::Model {
                    id: 2,
                    name: "Raspberry Cheese Cake".into(),
                },
                Some(fruit::Model {
                    id: 10,
                    name: "Raspberry".into(),
                    cake_id: Some(1),
                }),
            ),
        ];

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        assert_eq!(
            cake::Entity::find()
                .find_also_related(Fruit)
                .cursor_by(cake::Column::Id)
                .before(10)
                .first(2)
                .all(&db)
                .await?,
            models
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "cake"."id" AS "A_id", "cake"."name" AS "A_name","#,
                    r#""fruit"."id" AS "B_id", "fruit"."name" AS "B_name", "fruit"."cake_id" AS "B_cake_id""#,
                    r#"FROM "cake""#,
                    r#"LEFT JOIN "fruit" ON "cake"."id" = "fruit"."cake_id""#,
                    r#"WHERE "cake"."id" < $1"#,
                    r#"ORDER BY "cake"."id" ASC, "fruit"."id" ASC LIMIT $2"#,
                ]
                .join(" ")
                .as_str(),
                [10_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn last_2_after_10_also_related_select_desc() -> Result<(), DbErr> {
        let mut models = [
            (
                cake::Model {
                    id: 2,
                    name: "Raspberry Cheese Cake".into(),
                },
                Some(fruit::Model {
                    id: 10,
                    name: "Raspberry".into(),
                    cake_id: Some(1),
                }),
            ),
            (
                cake::Model {
                    id: 1,
                    name: "Blueberry Cheese Cake".into(),
                },
                Some(fruit::Model {
                    id: 9,
                    name: "Blueberry".into(),
                    cake_id: Some(1),
                }),
            ),
        ];

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        models.reverse();

        assert_eq!(
            cake::Entity::find()
                .find_also_related(Fruit)
                .cursor_by(cake::Column::Id)
                .after(10)
                .last(2)
                .desc()
                .all(&db)
                .await?,
            models
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "cake"."id" AS "A_id", "cake"."name" AS "A_name","#,
                    r#""fruit"."id" AS "B_id", "fruit"."name" AS "B_name", "fruit"."cake_id" AS "B_cake_id""#,
                    r#"FROM "cake""#,
                    r#"LEFT JOIN "fruit" ON "cake"."id" = "fruit"."cake_id""#,
                    r#"WHERE "cake"."id" < $1"#,
                    r#"ORDER BY "cake"."id" ASC, "fruit"."id" ASC LIMIT $2"#,
                ]
                .join(" ")
                .as_str(),
                [10_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn first_2_before_10_also_related_select_cursor_other() -> Result<(), DbErr> {
        let models = [(
            cake::Model {
                id: 1,
                name: "Blueberry Cheese Cake".into(),
            },
            Some(fruit::Model {
                id: 9,
                name: "Blueberry".into(),
                cake_id: Some(1),
            }),
        )];

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        assert_eq!(
            cake::Entity::find()
                .find_also_related(Fruit)
                .cursor_by_other(fruit::Column::Id)
                .before(10)
                .first(2)
                .all(&db)
                .await?,
            models
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "cake"."id" AS "A_id", "cake"."name" AS "A_name","#,
                    r#""fruit"."id" AS "B_id", "fruit"."name" AS "B_name", "fruit"."cake_id" AS "B_cake_id""#,
                    r#"FROM "cake""#,
                    r#"LEFT JOIN "fruit" ON "cake"."id" = "fruit"."cake_id""#,
                    r#"WHERE "fruit"."id" < $1"#,
                    r#"ORDER BY "fruit"."id" ASC, "cake"."id" ASC LIMIT $2"#,
                ]
                .join(" ")
                .as_str(),
                [10_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn last_2_after_10_also_related_select_cursor_other_desc() -> Result<(), DbErr> {
        let models = [(
            cake::Model {
                id: 1,
                name: "Blueberry Cheese Cake".into(),
            },
            Some(fruit::Model {
                id: 9,
                name: "Blueberry".into(),
                cake_id: Some(1),
            }),
        )];

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        assert_eq!(
            cake::Entity::find()
                .find_also_related(Fruit)
                .cursor_by_other(fruit::Column::Id)
                .after(10)
                .last(2)
                .desc()
                .all(&db)
                .await?,
            models
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "cake"."id" AS "A_id", "cake"."name" AS "A_name","#,
                    r#""fruit"."id" AS "B_id", "fruit"."name" AS "B_name", "fruit"."cake_id" AS "B_cake_id""#,
                    r#"FROM "cake""#,
                    r#"LEFT JOIN "fruit" ON "cake"."id" = "fruit"."cake_id""#,
                    r#"WHERE "fruit"."id" < $1"#,
                    r#"ORDER BY "fruit"."id" ASC, "cake"."id" ASC LIMIT $2"#,
                ]
                .join(" ")
                .as_str(),
                [10_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn first_2_before_10_also_linked_select() -> Result<(), DbErr> {
        let models = [
            (
                cake::Model {
                    id: 1,
                    name: "Blueberry Cheese Cake".into(),
                },
                Some(vendor::Model {
                    id: 9,
                    name: "Blueberry".into(),
                }),
            ),
            (
                cake::Model {
                    id: 2,
                    name: "Raspberry Cheese Cake".into(),
                },
                Some(vendor::Model {
                    id: 10,
                    name: "Raspberry".into(),
                }),
            ),
        ];

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        assert_eq!(
            cake::Entity::find()
                .find_also_linked(entity_linked::CakeToFillingVendor)
                .cursor_by(cake::Column::Id)
                .before(10)
                .first(2)
                .all(&db)
                .await?,
            models
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "cake"."id" AS "A_id", "cake"."name" AS "A_name","#,
                    r#""r2"."id" AS "B_id", "r2"."name" AS "B_name""#,
                    r#"FROM "cake""#,
                    r#"LEFT JOIN "cake_filling" AS "r0" ON "cake"."id" = "r0"."cake_id""#,
                    r#"LEFT JOIN "filling" AS "r1" ON "r0"."filling_id" = "r1"."id""#,
                    r#"LEFT JOIN "vendor" AS "r2" ON "r1"."vendor_id" = "r2"."id""#,
                    r#"WHERE "cake"."id" < $1 ORDER BY "cake"."id" ASC, "vendor"."id" ASC LIMIT $2"#,
                ]
                .join(" ")
                .as_str(),
                [10_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn last_2_after_10_also_linked_select_desc() -> Result<(), DbErr> {
        let mut models = [
            (
                cake::Model {
                    id: 2,
                    name: "Raspberry Cheese Cake".into(),
                },
                Some(vendor::Model {
                    id: 10,
                    name: "Raspberry".into(),
                }),
            ),
            (
                cake::Model {
                    id: 1,
                    name: "Blueberry Cheese Cake".into(),
                },
                Some(vendor::Model {
                    id: 9,
                    name: "Blueberry".into(),
                }),
            ),
        ];

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        models.reverse();

        assert_eq!(
            cake::Entity::find()
                .find_also_linked(entity_linked::CakeToFillingVendor)
                .cursor_by(cake::Column::Id)
                .after(10)
                .last(2)
                .desc()
                .all(&db)
                .await?,
            models
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "cake"."id" AS "A_id", "cake"."name" AS "A_name","#,
                    r#""r2"."id" AS "B_id", "r2"."name" AS "B_name""#,
                    r#"FROM "cake""#,
                    r#"LEFT JOIN "cake_filling" AS "r0" ON "cake"."id" = "r0"."cake_id""#,
                    r#"LEFT JOIN "filling" AS "r1" ON "r0"."filling_id" = "r1"."id""#,
                    r#"LEFT JOIN "vendor" AS "r2" ON "r1"."vendor_id" = "r2"."id""#,
                    r#"WHERE "cake"."id" < $1 ORDER BY "cake"."id" ASC, "vendor"."id" ASC LIMIT $2"#,
                ]
                .join(" ")
                .as_str(),
                [10_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn first_2_before_10_also_linked_select_cursor_other() -> Result<(), DbErr> {
        let models = [(
            cake::Model {
                id: 1,
                name: "Blueberry Cheese Cake".into(),
            },
            Some(vendor::Model {
                id: 9,
                name: "Blueberry".into(),
            }),
        )];

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        assert_eq!(
            cake::Entity::find()
                .find_also_linked(entity_linked::CakeToFillingVendor)
                .cursor_by_other(vendor::Column::Id)
                .before(10)
                .first(2)
                .all(&db)
                .await?,
            models
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "cake"."id" AS "A_id", "cake"."name" AS "A_name","#,
                    r#""r2"."id" AS "B_id", "r2"."name" AS "B_name""#,
                    r#"FROM "cake""#,
                    r#"LEFT JOIN "cake_filling" AS "r0" ON "cake"."id" = "r0"."cake_id""#,
                    r#"LEFT JOIN "filling" AS "r1" ON "r0"."filling_id" = "r1"."id""#,
                    r#"LEFT JOIN "vendor" AS "r2" ON "r1"."vendor_id" = "r2"."id""#,
                    r#"WHERE "vendor"."id" < $1 ORDER BY "vendor"."id" ASC, "cake"."id" ASC LIMIT $2"#,
                ]
                .join(" ")
                .as_str(),
                [10_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn last_2_after_10_also_linked_select_cursor_other_desc() -> Result<(), DbErr> {
        let mut models = [(
            cake::Model {
                id: 1,
                name: "Blueberry Cheese Cake".into(),
            },
            Some(vendor::Model {
                id: 9,
                name: "Blueberry".into(),
            }),
        )];

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        models.reverse();

        assert_eq!(
            cake::Entity::find()
                .find_also_linked(entity_linked::CakeToFillingVendor)
                .cursor_by_other(vendor::Column::Id)
                .after(10)
                .last(2)
                .desc()
                .all(&db)
                .await?,
            models
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "cake"."id" AS "A_id", "cake"."name" AS "A_name","#,
                    r#""r2"."id" AS "B_id", "r2"."name" AS "B_name""#,
                    r#"FROM "cake""#,
                    r#"LEFT JOIN "cake_filling" AS "r0" ON "cake"."id" = "r0"."cake_id""#,
                    r#"LEFT JOIN "filling" AS "r1" ON "r0"."filling_id" = "r1"."id""#,
                    r#"LEFT JOIN "vendor" AS "r2" ON "r1"."vendor_id" = "r2"."id""#,
                    r#"WHERE "vendor"."id" < $1 ORDER BY "vendor"."id" ASC, "cake"."id" ASC LIMIT $2"#,
                ]
                .join(" ")
                .as_str(),
                [10_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn last_2_after_10() -> Result<(), DbErr> {
        use fruit::*;

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[
                Model {
                    id: 22,
                    name: "Raspberry".into(),
                    cake_id: Some(1),
                },
                Model {
                    id: 21,
                    name: "Blueberry".into(),
                    cake_id: Some(1),
                },
            ]])
            .into_connection();

        assert_eq!(
            Entity::find()
                .cursor_by(Column::Id)
                .after(10)
                .last(2)
                .all(&db)
                .await?,
            [
                Model {
                    id: 21,
                    name: "Blueberry".into(),
                    cake_id: Some(1),
                },
                Model {
                    id: 22,
                    name: "Raspberry".into(),
                    cake_id: Some(1),
                },
            ]
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "fruit"."id", "fruit"."name", "fruit"."cake_id""#,
                    r#"FROM "fruit""#,
                    r#"WHERE "fruit"."id" > $1"#,
                    r#"ORDER BY "fruit"."id" DESC"#,
                    r#"LIMIT $2"#,
                ]
                .join(" ")
                .as_str(),
                [10_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn first_2_before_10_desc() -> Result<(), DbErr> {
        use fruit::*;

        let models = [
            Model {
                id: 22,
                name: "Raspberry".into(),
                cake_id: Some(1),
            },
            Model {
                id: 21,
                name: "Blueberry".into(),
                cake_id: Some(1),
            },
        ];

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        assert_eq!(
            Entity::find()
                .cursor_by(Column::Id)
                .before(10)
                .first(2)
                .desc()
                .all(&db)
                .await?,
            models
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "fruit"."id", "fruit"."name", "fruit"."cake_id""#,
                    r#"FROM "fruit""#,
                    r#"WHERE "fruit"."id" > $1"#,
                    r#"ORDER BY "fruit"."id" DESC"#,
                    r#"LIMIT $2"#,
                ]
                .join(" ")
                .as_str(),
                [10_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn last_2_after_25_before_30() -> Result<(), DbErr> {
        use fruit::*;

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[
                Model {
                    id: 27,
                    name: "Raspberry".into(),
                    cake_id: Some(1),
                },
                Model {
                    id: 26,
                    name: "Blueberry".into(),
                    cake_id: Some(1),
                },
            ]])
            .into_connection();

        assert_eq!(
            Entity::find()
                .cursor_by(Column::Id)
                .after(25)
                .before(30)
                .last(2)
                .all(&db)
                .await?,
            [
                Model {
                    id: 26,
                    name: "Blueberry".into(),
                    cake_id: Some(1),
                },
                Model {
                    id: 27,
                    name: "Raspberry".into(),
                    cake_id: Some(1),
                },
            ]
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "fruit"."id", "fruit"."name", "fruit"."cake_id""#,
                    r#"FROM "fruit""#,
                    r#"WHERE "fruit"."id" > $1"#,
                    r#"AND "fruit"."id" < $2"#,
                    r#"ORDER BY "fruit"."id" DESC"#,
                    r#"LIMIT $3"#,
                ]
                .join(" ")
                .as_str(),
                [25_i32.into(), 30_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn first_2_after_30_before_25_desc() -> Result<(), DbErr> {
        use fruit::*;

        let models = [
            Model {
                id: 27,
                name: "Raspberry".into(),
                cake_id: Some(1),
            },
            Model {
                id: 26,
                name: "Blueberry".into(),
                cake_id: Some(1),
            },
        ];

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        assert_eq!(
            Entity::find()
                .cursor_by(Column::Id)
                .after(30)
                .before(25)
                .first(2)
                .desc()
                .all(&db)
                .await?,
            models
        );

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "fruit"."id", "fruit"."name", "fruit"."cake_id""#,
                    r#"FROM "fruit""#,
                    r#"WHERE "fruit"."id" < $1"#,
                    r#"AND "fruit"."id" > $2"#,
                    r#"ORDER BY "fruit"."id" DESC"#,
                    r#"LIMIT $3"#,
                ]
                .join(" ")
                .as_str(),
                [30_i32.into(), 25_i32.into(), 2_u64.into()]
            ),])]
        );

        Ok(())
    }

    mod test_entity {
        use crate as sea_orm;
        use crate::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "example")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            #[sea_orm(primary_key)]
            pub category: String,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    mod xyz_entity {
        use crate as sea_orm;
        use crate::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "m")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub x: i32,
            #[sea_orm(primary_key)]
            pub y: String,
            #[sea_orm(primary_key)]
            pub z: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    #[smol_potat::test]
    async fn composite_keys_1() -> Result<(), DbErr> {
        use test_entity::*;

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[Model {
                id: 1,
                category: "CAT".into(),
            }]])
            .into_connection();

        assert!(!Entity::find()
            .cursor_by((Column::Category, Column::Id))
            .first(3)
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "example"."id", "example"."category""#,
                    r#"FROM "example""#,
                    r#"ORDER BY "example"."category" ASC, "example"."id" ASC"#,
                    r#"LIMIT $1"#,
                ]
                .join(" ")
                .as_str(),
                [3_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn composite_keys_1_desc() -> Result<(), DbErr> {
        use test_entity::*;

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[Model {
                id: 1,
                category: "CAT".into(),
            }]])
            .into_connection();

        assert!(!Entity::find()
            .cursor_by((Column::Category, Column::Id))
            .last(3)
            .desc()
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "example"."id", "example"."category""#,
                    r#"FROM "example""#,
                    r#"ORDER BY "example"."category" ASC, "example"."id" ASC"#,
                    r#"LIMIT $1"#,
                ]
                .join(" ")
                .as_str(),
                [3_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn composite_keys_2() -> Result<(), DbErr> {
        use test_entity::*;

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[Model {
                id: 1,
                category: "CAT".into(),
            }]])
            .into_connection();

        assert!(!Entity::find()
            .cursor_by((Column::Category, Column::Id))
            .after(("A".to_owned(), 2))
            .first(3)
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "example"."id", "example"."category""#,
                    r#"FROM "example""#,
                    r#"WHERE ("example"."category" = $1 AND "example"."id" > $2)"#,
                    r#"OR "example"."category" > $3"#,
                    r#"ORDER BY "example"."category" ASC, "example"."id" ASC"#,
                    r#"LIMIT $4"#,
                ]
                .join(" ")
                .as_str(),
                [
                    "A".to_string().into(),
                    2i32.into(),
                    "A".to_string().into(),
                    3_u64.into(),
                ]
            )])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn composite_keys_2_desc() -> Result<(), DbErr> {
        use test_entity::*;

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[Model {
                id: 1,
                category: "CAT".into(),
            }]])
            .into_connection();

        assert!(!Entity::find()
            .cursor_by((Column::Category, Column::Id))
            .before(("A".to_owned(), 2))
            .last(3)
            .desc()
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "example"."id", "example"."category""#,
                    r#"FROM "example""#,
                    r#"WHERE ("example"."category" = $1 AND "example"."id" > $2)"#,
                    r#"OR "example"."category" > $3"#,
                    r#"ORDER BY "example"."category" ASC, "example"."id" ASC"#,
                    r#"LIMIT $4"#,
                ]
                .join(" ")
                .as_str(),
                [
                    "A".to_string().into(),
                    2i32.into(),
                    "A".to_string().into(),
                    3_u64.into(),
                ]
            )])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn composite_keys_3() -> Result<(), DbErr> {
        use test_entity::*;

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[Model {
                id: 1,
                category: "CAT".into(),
            }]])
            .into_connection();

        assert!(!Entity::find()
            .cursor_by((Column::Category, Column::Id))
            .before(("A".to_owned(), 2))
            .last(3)
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "example"."id", "example"."category""#,
                    r#"FROM "example""#,
                    r#"WHERE ("example"."category" = $1 AND "example"."id" < $2)"#,
                    r#"OR "example"."category" < $3"#,
                    r#"ORDER BY "example"."category" DESC, "example"."id" DESC"#,
                    r#"LIMIT $4"#,
                ]
                .join(" ")
                .as_str(),
                [
                    "A".to_string().into(),
                    2i32.into(),
                    "A".to_string().into(),
                    3_u64.into(),
                ]
            )])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn composite_keys_3_desc() -> Result<(), DbErr> {
        use test_entity::*;

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[Model {
                id: 1,
                category: "CAT".into(),
            }]])
            .into_connection();

        assert!(!Entity::find()
            .cursor_by((Column::Category, Column::Id))
            .after(("A".to_owned(), 2))
            .first(3)
            .desc()
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "example"."id", "example"."category""#,
                    r#"FROM "example""#,
                    r#"WHERE ("example"."category" = $1 AND "example"."id" < $2)"#,
                    r#"OR "example"."category" < $3"#,
                    r#"ORDER BY "example"."category" DESC, "example"."id" DESC"#,
                    r#"LIMIT $4"#,
                ]
                .join(" ")
                .as_str(),
                [
                    "A".to_string().into(),
                    2i32.into(),
                    "A".to_string().into(),
                    3_u64.into(),
                ]
            )])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn composite_keys_4() -> Result<(), DbErr> {
        use xyz_entity::*;

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[Model {
                x: 'x' as i32,
                y: "y".into(),
                z: 'z' as i64,
            }]])
            .into_connection();

        assert!(!Entity::find()
            .cursor_by((Column::X, Column::Y, Column::Z))
            .first(4)
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "m"."x", "m"."y", "m"."z""#,
                    r#"FROM "m""#,
                    r#"ORDER BY "m"."x" ASC, "m"."y" ASC, "m"."z" ASC"#,
                    r#"LIMIT $1"#,
                ]
                .join(" ")
                .as_str(),
                [4_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn composite_keys_4_desc() -> Result<(), DbErr> {
        use xyz_entity::*;

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[Model {
                x: 'x' as i32,
                y: "y".into(),
                z: 'z' as i64,
            }]])
            .into_connection();

        assert!(!Entity::find()
            .cursor_by((Column::X, Column::Y, Column::Z))
            .last(4)
            .desc()
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "m"."x", "m"."y", "m"."z""#,
                    r#"FROM "m""#,
                    r#"ORDER BY "m"."x" ASC, "m"."y" ASC, "m"."z" ASC"#,
                    r#"LIMIT $1"#,
                ]
                .join(" ")
                .as_str(),
                [4_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn composite_keys_5() -> Result<(), DbErr> {
        use xyz_entity::*;

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[Model {
                x: 'x' as i32,
                y: "y".into(),
                z: 'z' as i64,
            }]])
            .into_connection();

        assert!(!Entity::find()
            .cursor_by((Column::X, Column::Y, Column::Z))
            .after(('x' as i32, "y".to_owned(), 'z' as i64))
            .first(4)
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "m"."x", "m"."y", "m"."z""#,
                    r#"FROM "m""#,
                    r#"WHERE ("m"."x" = $1 AND "m"."y" = $2 AND "m"."z" > $3)"#,
                    r#"OR ("m"."x" = $4 AND "m"."y" > $5)"#,
                    r#"OR "m"."x" > $6"#,
                    r#"ORDER BY "m"."x" ASC, "m"."y" ASC, "m"."z" ASC"#,
                    r#"LIMIT $7"#,
                ]
                .join(" ")
                .as_str(),
                [
                    ('x' as i32).into(),
                    "y".into(),
                    ('z' as i64).into(),
                    ('x' as i32).into(),
                    "y".into(),
                    ('x' as i32).into(),
                    4_u64.into(),
                ]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn composite_keys_5_desc() -> Result<(), DbErr> {
        use xyz_entity::*;

        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[Model {
                x: 'x' as i32,
                y: "y".into(),
                z: 'z' as i64,
            }]])
            .into_connection();

        assert!(!Entity::find()
            .cursor_by((Column::X, Column::Y, Column::Z))
            .before(('x' as i32, "y".to_owned(), 'z' as i64))
            .last(4)
            .desc()
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "m"."x", "m"."y", "m"."z""#,
                    r#"FROM "m""#,
                    r#"WHERE ("m"."x" = $1 AND "m"."y" = $2 AND "m"."z" > $3)"#,
                    r#"OR ("m"."x" = $4 AND "m"."y" > $5)"#,
                    r#"OR "m"."x" > $6"#,
                    r#"ORDER BY "m"."x" ASC, "m"."y" ASC, "m"."z" ASC"#,
                    r#"LIMIT $7"#,
                ]
                .join(" ")
                .as_str(),
                [
                    ('x' as i32).into(),
                    "y".into(),
                    ('z' as i64).into(),
                    ('x' as i32).into(),
                    "y".into(),
                    ('x' as i32).into(),
                    4_u64.into(),
                ]
            ),])]
        );

        Ok(())
    }

    mod composite_entity {
        use crate as sea_orm;
        use crate::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "t")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub col_1: String,
            #[sea_orm(primary_key)]
            pub col_2: String,
            #[sea_orm(primary_key)]
            pub col_3: String,
            #[sea_orm(primary_key)]
            pub col_4: String,
            #[sea_orm(primary_key)]
            pub col_5: String,
            #[sea_orm(primary_key)]
            pub col_6: String,
            #[sea_orm(primary_key)]
            pub col_7: String,
            #[sea_orm(primary_key)]
            pub col_8: String,
            #[sea_orm(primary_key)]
            pub col_9: String,
            #[sea_orm(primary_key)]
            pub col_10: String,
            #[sea_orm(primary_key)]
            pub col_11: String,
            #[sea_orm(primary_key)]
            pub col_12: String,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    #[smol_potat::test]
    async fn cursor_by_many() -> Result<(), DbErr> {
        use composite_entity::*;

        let base_sql = [
            r#"SELECT "t"."col_1", "t"."col_2", "t"."col_3", "t"."col_4", "t"."col_5", "t"."col_6", "t"."col_7", "t"."col_8", "t"."col_9", "t"."col_10", "t"."col_11", "t"."col_12""#,
            r#"FROM "t" WHERE"#,
        ].join(" ");

        assert_eq!(
            DbBackend::Postgres.build(&
                Entity::find()
                .cursor_by((Column::Col1, Column::Col2, Column::Col3, Column::Col4))
                .after(("val_1", "val_2", "val_3", "val_4")).apply_limit().apply_order_by().apply_filters().query
            ).to_string(),
            format!("{base_sql} {}", [
                r#"("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" > 'val_4')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" > 'val_3')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" > 'val_2')"#,
                r#"OR "t"."col_1" > 'val_1'"#,
                r#"ORDER BY "t"."col_1" ASC, "t"."col_2" ASC, "t"."col_3" ASC, "t"."col_4" ASC"#,
            ].join(" "))
        );

        assert_eq!(
            DbBackend::Postgres.build(&
                Entity::find()
                .cursor_by((Column::Col1, Column::Col2, Column::Col3, Column::Col4, Column::Col5))
                .after(("val_1", "val_2", "val_3", "val_4", "val_5")).apply_limit().apply_order_by().apply_filters()
                .query
            ).to_string(),
            format!("{base_sql} {}", [
                r#"("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" > 'val_5')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" > 'val_4')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" > 'val_3')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" > 'val_2')"#,
                r#"OR "t"."col_1" > 'val_1'"#,
                r#"ORDER BY "t"."col_1" ASC, "t"."col_2" ASC, "t"."col_3" ASC, "t"."col_4" ASC, "t"."col_5" ASC"#,
            ].join(" "))
        );

        assert_eq!(
            DbBackend::Postgres.build(&
                Entity::find()
                .cursor_by((Column::Col1, Column::Col2, Column::Col3, Column::Col4, Column::Col5, Column::Col6))
                .after(("val_1", "val_2", "val_3", "val_4", "val_5", "val_6")).apply_limit().apply_order_by().apply_filters()
                .query
            ).to_string(),
            format!("{base_sql} {}", [
                r#"("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" > 'val_6')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" > 'val_5')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" > 'val_4')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" > 'val_3')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" > 'val_2')"#,
                r#"OR "t"."col_1" > 'val_1'"#,
                r#"ORDER BY "t"."col_1" ASC, "t"."col_2" ASC, "t"."col_3" ASC, "t"."col_4" ASC, "t"."col_5" ASC, "t"."col_6" ASC"#,
            ].join(" "))
        );

        assert_eq!(
            DbBackend::Postgres.build(&
                Entity::find()
                .cursor_by((Column::Col1, Column::Col2, Column::Col3, Column::Col4, Column::Col5, Column::Col6, Column::Col7))
                .before(("val_1", "val_2", "val_3", "val_4", "val_5", "val_6", "val_7")).apply_limit().apply_order_by().apply_filters()
                .query
            ).to_string(),
            format!("{base_sql} {}", [
                r#"("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" = 'val_6' AND "t"."col_7" < 'val_7')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" < 'val_6')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" < 'val_5')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" < 'val_4')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" < 'val_3')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" < 'val_2')"#,
                r#"OR "t"."col_1" < 'val_1'"#,
                r#"ORDER BY "t"."col_1" ASC, "t"."col_2" ASC, "t"."col_3" ASC, "t"."col_4" ASC, "t"."col_5" ASC, "t"."col_6" ASC, "t"."col_7" ASC"#,
            ].join(" "))
        );

        assert_eq!(
            DbBackend::Postgres.build(&
                Entity::find()
                .cursor_by((Column::Col1, Column::Col2, Column::Col3, Column::Col4, Column::Col5, Column::Col6, Column::Col7, Column::Col8))
                .before(("val_1", "val_2", "val_3", "val_4", "val_5", "val_6", "val_7", "val_8")).apply_limit().apply_order_by().apply_filters()
                .query
            ).to_string(),
            format!("{base_sql} {}", [
                r#"("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" = 'val_6' AND "t"."col_7" = 'val_7' AND "t"."col_8" < 'val_8')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" = 'val_6' AND "t"."col_7" < 'val_7')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" < 'val_6')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" < 'val_5')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" < 'val_4')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" < 'val_3')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" < 'val_2')"#,
                r#"OR "t"."col_1" < 'val_1'"#,
                r#"ORDER BY "t"."col_1" ASC, "t"."col_2" ASC, "t"."col_3" ASC, "t"."col_4" ASC, "t"."col_5" ASC, "t"."col_6" ASC, "t"."col_7" ASC, "t"."col_8" ASC"#,
            ].join(" "))
        );

        assert_eq!(
            DbBackend::Postgres.build(&
                Entity::find()
                .cursor_by((Column::Col1, Column::Col2, Column::Col3, Column::Col4, Column::Col5, Column::Col6, Column::Col7, Column::Col8, Column::Col9))
                .before(("val_1", "val_2", "val_3", "val_4", "val_5", "val_6", "val_7", "val_8", "val_9")).apply_limit().apply_order_by().apply_filters()
                .query
            ).to_string(),
            format!("{base_sql} {}", [
                r#"("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" = 'val_6' AND "t"."col_7" = 'val_7' AND "t"."col_8" = 'val_8' AND "t"."col_9" < 'val_9')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" = 'val_6' AND "t"."col_7" = 'val_7' AND "t"."col_8" < 'val_8')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" = 'val_6' AND "t"."col_7" < 'val_7')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" < 'val_6')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" < 'val_5')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" < 'val_4')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" < 'val_3')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" < 'val_2')"#,
                r#"OR "t"."col_1" < 'val_1'"#,
                r#"ORDER BY "t"."col_1" ASC, "t"."col_2" ASC, "t"."col_3" ASC, "t"."col_4" ASC, "t"."col_5" ASC, "t"."col_6" ASC, "t"."col_7" ASC, "t"."col_8" ASC, "t"."col_9" ASC"#,
            ].join(" "))
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn cursor_by_many_desc() -> Result<(), DbErr> {
        use composite_entity::*;

        let base_sql = [
            r#"SELECT "t"."col_1", "t"."col_2", "t"."col_3", "t"."col_4", "t"."col_5", "t"."col_6", "t"."col_7", "t"."col_8", "t"."col_9", "t"."col_10", "t"."col_11", "t"."col_12""#,
            r#"FROM "t" WHERE"#,
        ].join(" ");

        assert_eq!(
            DbBackend::Postgres.build(&
                Entity::find()
                .cursor_by((Column::Col1, Column::Col2, Column::Col3, Column::Col4))
                .before(("val_1", "val_2", "val_3", "val_4")).desc().apply_limit().apply_order_by().apply_filters().query
            ).to_string(),
            format!("{base_sql} {}", [
                r#"("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" > 'val_4')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" > 'val_3')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" > 'val_2')"#,
                r#"OR "t"."col_1" > 'val_1'"#,
                r#"ORDER BY "t"."col_1" DESC, "t"."col_2" DESC, "t"."col_3" DESC, "t"."col_4" DESC"#,
            ].join(" "))
        );

        assert_eq!(
            DbBackend::Postgres.build(&
                Entity::find()
                .cursor_by((Column::Col1, Column::Col2, Column::Col3, Column::Col4, Column::Col5))
                .before(("val_1", "val_2", "val_3", "val_4", "val_5")).desc().apply_limit().apply_order_by().apply_filters()
                .query
            ).to_string(),
            format!("{base_sql} {}", [
                r#"("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" > 'val_5')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" > 'val_4')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" > 'val_3')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" > 'val_2')"#,
                r#"OR "t"."col_1" > 'val_1'"#,
                r#"ORDER BY "t"."col_1" DESC, "t"."col_2" DESC, "t"."col_3" DESC, "t"."col_4" DESC, "t"."col_5" DESC"#,
            ].join(" "))
        );

        assert_eq!(
            DbBackend::Postgres.build(&
                Entity::find()
                .cursor_by((Column::Col1, Column::Col2, Column::Col3, Column::Col4, Column::Col5, Column::Col6))
                .before(("val_1", "val_2", "val_3", "val_4", "val_5", "val_6")).desc().apply_limit().apply_order_by().apply_filters()
                .query
            ).to_string(),
            format!("{base_sql} {}", [
                r#"("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" > 'val_6')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" > 'val_5')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" > 'val_4')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" > 'val_3')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" > 'val_2')"#,
                r#"OR "t"."col_1" > 'val_1'"#,
                r#"ORDER BY "t"."col_1" DESC, "t"."col_2" DESC, "t"."col_3" DESC, "t"."col_4" DESC, "t"."col_5" DESC, "t"."col_6" DESC"#,
            ].join(" "))
        );

        assert_eq!(
            DbBackend::Postgres.build(&
                Entity::find()
                .cursor_by((Column::Col1, Column::Col2, Column::Col3, Column::Col4, Column::Col5, Column::Col6, Column::Col7))
                .after(("val_1", "val_2", "val_3", "val_4", "val_5", "val_6", "val_7")).desc().apply_limit().apply_order_by().apply_filters()
                .query
            ).to_string(),
            format!("{base_sql} {}", [
                r#"("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" = 'val_6' AND "t"."col_7" < 'val_7')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" < 'val_6')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" < 'val_5')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" < 'val_4')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" < 'val_3')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" < 'val_2')"#,
                r#"OR "t"."col_1" < 'val_1'"#,
                r#"ORDER BY "t"."col_1" DESC, "t"."col_2" DESC, "t"."col_3" DESC, "t"."col_4" DESC, "t"."col_5" DESC, "t"."col_6" DESC, "t"."col_7" DESC"#,
            ].join(" "))
        );

        assert_eq!(
            DbBackend::Postgres.build(&
                Entity::find()
                .cursor_by((Column::Col1, Column::Col2, Column::Col3, Column::Col4, Column::Col5, Column::Col6, Column::Col7, Column::Col8))
                .after(("val_1", "val_2", "val_3", "val_4", "val_5", "val_6", "val_7", "val_8")).desc().apply_limit().apply_order_by().apply_filters()
                .query
            ).to_string(),
            format!("{base_sql} {}", [
                r#"("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" = 'val_6' AND "t"."col_7" = 'val_7' AND "t"."col_8" < 'val_8')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" = 'val_6' AND "t"."col_7" < 'val_7')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" < 'val_6')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" < 'val_5')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" < 'val_4')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" < 'val_3')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" < 'val_2')"#,
                r#"OR "t"."col_1" < 'val_1'"#,
                r#"ORDER BY "t"."col_1" DESC, "t"."col_2" DESC, "t"."col_3" DESC, "t"."col_4" DESC, "t"."col_5" DESC, "t"."col_6" DESC, "t"."col_7" DESC, "t"."col_8" DESC"#,
            ].join(" "))
        );

        assert_eq!(
            DbBackend::Postgres.build(&
                Entity::find()
                .cursor_by((Column::Col1, Column::Col2, Column::Col3, Column::Col4, Column::Col5, Column::Col6, Column::Col7, Column::Col8, Column::Col9))
                .after(("val_1", "val_2", "val_3", "val_4", "val_5", "val_6", "val_7", "val_8", "val_9")).desc().apply_limit().apply_order_by().apply_filters()
                .query
            ).to_string(),
            format!("{base_sql} {}", [
                r#"("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" = 'val_6' AND "t"."col_7" = 'val_7' AND "t"."col_8" = 'val_8' AND "t"."col_9" < 'val_9')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" = 'val_6' AND "t"."col_7" = 'val_7' AND "t"."col_8" < 'val_8')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" = 'val_6' AND "t"."col_7" < 'val_7')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" = 'val_5' AND "t"."col_6" < 'val_6')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" = 'val_4' AND "t"."col_5" < 'val_5')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" = 'val_3' AND "t"."col_4" < 'val_4')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" = 'val_2' AND "t"."col_3" < 'val_3')"#,
                r#"OR ("t"."col_1" = 'val_1' AND "t"."col_2" < 'val_2')"#,
                r#"OR "t"."col_1" < 'val_1'"#,
                r#"ORDER BY "t"."col_1" DESC, "t"."col_2" DESC, "t"."col_3" DESC, "t"."col_4" DESC, "t"."col_5" DESC, "t"."col_6" DESC, "t"."col_7" DESC, "t"."col_8" DESC, "t"."col_9" DESC"#,
            ].join(" "))
        );

        Ok(())
    }

    mod test_base_entity {
        use crate as sea_orm;
        use crate::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "base")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            #[sea_orm(primary_key)]
            pub name: String,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {
            #[sea_orm(has_many = "super::test_related_entity::Entity")]
            TestRelatedEntity,
        }

        impl Related<super::test_related_entity::Entity> for Entity {
            fn to() -> RelationDef {
                Relation::TestRelatedEntity.def()
            }
        }

        impl ActiveModelBehavior for ActiveModel {}
    }

    mod test_related_entity {
        use super::test_base_entity;
        use crate as sea_orm;
        use crate::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "related")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            #[sea_orm(primary_key)]
            pub name: String,
            pub test_id: i32,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {
            #[sea_orm(
                belongs_to = "test_base_entity::Entity",
                from = "Column::TestId",
                to = "super::test_base_entity::Column::Id"
            )]
            TestBaseEntity,
        }

        impl Related<super::test_base_entity::Entity> for Entity {
            fn to() -> RelationDef {
                Relation::TestBaseEntity.def()
            }
        }

        impl ActiveModelBehavior for ActiveModel {}
    }

    #[smol_potat::test]
    async fn related_composite_keys_1() -> Result<(), DbErr> {
        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[(
                test_base_entity::Model {
                    id: 1,
                    name: "CAT".into(),
                },
                test_related_entity::Model {
                    id: 1,
                    name: "CATE".into(),
                    test_id: 1,
                },
            )]])
            .into_connection();

        assert!(!test_base_entity::Entity::find()
            .find_also_related(test_related_entity::Entity)
            .cursor_by((test_base_entity::Column::Id, test_base_entity::Column::Name))
            .first(1)
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "base"."id" AS "A_id", "base"."name" AS "A_name","#,
                    r#""related"."id" AS "B_id", "related"."name" AS "B_name", "related"."test_id" AS "B_test_id""#,
                    r#"FROM "base""#,
                    r#"LEFT JOIN "related" ON "base"."id" = "related"."test_id""#,
                    r#"ORDER BY "base"."id" ASC, "base"."name" ASC, "related"."id" ASC, "related"."name" ASC LIMIT $1"#,
                ]
                .join(" ")
                .as_str(),
                [1_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn related_composite_keys_1_desc() -> Result<(), DbErr> {
        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[(
                test_base_entity::Model {
                    id: 1,
                    name: "CAT".into(),
                },
                test_related_entity::Model {
                    id: 1,
                    name: "CATE".into(),
                    test_id: 1,
                },
            )]])
            .into_connection();

        assert!(!test_base_entity::Entity::find()
            .find_also_related(test_related_entity::Entity)
            .cursor_by((test_base_entity::Column::Id, test_base_entity::Column::Name))
            .last(1)
            .desc()
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "base"."id" AS "A_id", "base"."name" AS "A_name","#,
                    r#""related"."id" AS "B_id", "related"."name" AS "B_name", "related"."test_id" AS "B_test_id""#,
                    r#"FROM "base""#,
                    r#"LEFT JOIN "related" ON "base"."id" = "related"."test_id""#,
                    r#"ORDER BY "base"."id" ASC, "base"."name" ASC, "related"."id" ASC, "related"."name" ASC LIMIT $1"#,
                ]
                .join(" ")
                .as_str(),
                [1_u64.into()]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn related_composite_keys_2() -> Result<(), DbErr> {
        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[(
                test_base_entity::Model {
                    id: 1,
                    name: "CAT".into(),
                },
                test_related_entity::Model {
                    id: 1,
                    name: "CATE".into(),
                    test_id: 1,
                },
            )]])
            .into_connection();

        assert!(!test_base_entity::Entity::find()
            .find_also_related(test_related_entity::Entity)
            .cursor_by((test_base_entity::Column::Id, test_base_entity::Column::Name))
            .after((1, "C".to_string()))
            .first(2)
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "base"."id" AS "A_id", "base"."name" AS "A_name","#,
                    r#""related"."id" AS "B_id", "related"."name" AS "B_name", "related"."test_id" AS "B_test_id""#,
                    r#"FROM "base""#,
                    r#"LEFT JOIN "related" ON "base"."id" = "related"."test_id""#,
                    r#"WHERE ("base"."id" = $1 AND "base"."name" > $2) OR "base"."id" > $3"#,
                    r#"ORDER BY "base"."id" ASC, "base"."name" ASC, "related"."id" ASC, "related"."name" ASC LIMIT $4"#,
                ]
                .join(" ")
                .as_str(),
                [
                    1_i32.into(),
                    "C".into(),
                    1_i32.into(),
                    2_u64.into(),
                ]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn related_composite_keys_2_desc() -> Result<(), DbErr> {
        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[(
                test_base_entity::Model {
                    id: 1,
                    name: "CAT".into(),
                },
                test_related_entity::Model {
                    id: 1,
                    name: "CATE".into(),
                    test_id: 1,
                },
            )]])
            .into_connection();

        assert!(!test_base_entity::Entity::find()
            .find_also_related(test_related_entity::Entity)
            .cursor_by((test_base_entity::Column::Id, test_base_entity::Column::Name))
            .before((1, "C".to_string()))
            .last(2)
            .desc()
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "base"."id" AS "A_id", "base"."name" AS "A_name","#,
                    r#""related"."id" AS "B_id", "related"."name" AS "B_name", "related"."test_id" AS "B_test_id""#,
                    r#"FROM "base""#,
                    r#"LEFT JOIN "related" ON "base"."id" = "related"."test_id""#,
                    r#"WHERE ("base"."id" = $1 AND "base"."name" > $2) OR "base"."id" > $3"#,
                    r#"ORDER BY "base"."id" ASC, "base"."name" ASC, "related"."id" ASC, "related"."name" ASC LIMIT $4"#,
                ]
                .join(" ")
                .as_str(),
                [
                    1_i32.into(),
                    "C".into(),
                    1_i32.into(),
                    2_u64.into(),
                ]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn related_composite_keys_3() -> Result<(), DbErr> {
        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[(
                test_base_entity::Model {
                    id: 1,
                    name: "CAT".into(),
                },
                test_related_entity::Model {
                    id: 1,
                    name: "CATE".into(),
                    test_id: 1,
                },
            )]])
            .into_connection();

        assert!(!test_base_entity::Entity::find()
            .find_also_related(test_related_entity::Entity)
            .cursor_by_other((
                test_related_entity::Column::Id,
                test_related_entity::Column::Name
            ))
            .after((1, "CAT".to_string()))
            .first(2)
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "base"."id" AS "A_id", "base"."name" AS "A_name","#,
                    r#""related"."id" AS "B_id", "related"."name" AS "B_name", "related"."test_id" AS "B_test_id""#,
                    r#"FROM "base""#,
                    r#"LEFT JOIN "related" ON "base"."id" = "related"."test_id""#,
                    r#"WHERE ("related"."id" = $1 AND "related"."name" > $2) OR "related"."id" > $3"#,
                    r#"ORDER BY "related"."id" ASC, "related"."name" ASC, "base"."id" ASC, "base"."name" ASC LIMIT $4"#,
                ]
                .join(" ")
                .as_str(),
                [
                    1_i32.into(),
                    "CAT".into(),
                    1_i32.into(),
                    2_u64.into(),
                ]
            ),])]
        );

        Ok(())
    }

    #[smol_potat::test]
    async fn related_composite_keys_3_desc() -> Result<(), DbErr> {
        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[(
                test_base_entity::Model {
                    id: 1,
                    name: "CAT".into(),
                },
                test_related_entity::Model {
                    id: 1,
                    name: "CATE".into(),
                    test_id: 1,
                },
            )]])
            .into_connection();

        assert!(!test_base_entity::Entity::find()
            .find_also_related(test_related_entity::Entity)
            .cursor_by_other((
                test_related_entity::Column::Id,
                test_related_entity::Column::Name
            ))
            .before((1, "CAT".to_string()))
            .last(2)
            .desc()
            .all(&db)
            .await?
            .is_empty());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::many([Statement::from_sql_and_values(
                DbBackend::Postgres,
                [
                    r#"SELECT "base"."id" AS "A_id", "base"."name" AS "A_name","#,
                    r#""related"."id" AS "B_id", "related"."name" AS "B_name", "related"."test_id" AS "B_test_id""#,
                    r#"FROM "base""#,
                    r#"LEFT JOIN "related" ON "base"."id" = "related"."test_id""#,
                    r#"WHERE ("related"."id" = $1 AND "related"."name" > $2) OR "related"."id" > $3"#,
                    r#"ORDER BY "related"."id" ASC, "related"."name" ASC, "base"."id" ASC, "base"."name" ASC LIMIT $4"#,
                ]
                .join(" ")
                .as_str(),
                [
                    1_i32.into(),
                    "CAT".into(),
                    1_i32.into(),
                    2_u64.into(),
                ]
            ),])]
        );

        Ok(())
    }
}
