use crate::{
    backend::QueryBuilder,
    expr::*,
    prepare::*,
    query::{condition::*, OrderedStatement},
    types::*,
    value::*,
    FunctionCall, QueryStatementBuilder, QueryStatementWriter, SubQueryStatement, WindowStatement,
    WithClause, WithQuery,
};
use inherent::inherent;

/// Select rows from an existing table
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let query = Query::select()
///     .column(Char::Character)
///     .column((Font::Table, Font::Name))
///     .from(Char::Table)
///     .left_join(Font::Table, Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
///     .and_where(Expr::col(Char::SizeW).is_in([3, 4]))
///     .and_where(Expr::col(Char::Character).like("A%"))
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"SELECT `character`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id` WHERE `size_w` IN (3, 4) AND `character` LIKE 'A%'"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder),
///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" WHERE "size_w" IN (3, 4) AND "character" LIKE 'A%'"#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" WHERE "size_w" IN (3, 4) AND "character" LIKE 'A%'"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectStatement {
    pub(crate) distinct: Option<SelectDistinct>,
    pub(crate) selects: Vec<SelectExpr>,
    pub(crate) from: Vec<TableRef>,
    pub(crate) join: Vec<JoinExpr>,
    pub(crate) r#where: ConditionHolder,
    pub(crate) groups: Vec<SimpleExpr>,
    pub(crate) having: ConditionHolder,
    pub(crate) unions: Vec<(UnionType, SelectStatement)>,
    pub(crate) orders: Vec<OrderExpr>,
    pub(crate) limit: Option<Value>,
    pub(crate) offset: Option<Value>,
    pub(crate) lock: Option<LockClause>,
    pub(crate) window: Option<(DynIden, WindowStatement)>,
    pub(crate) with: Option<WithClause>,
    #[cfg(feature = "backend-postgres")]
    pub(crate) table_sample: Option<crate::extension::postgres::TableSample>,
    #[cfg(feature = "backend-mysql")]
    pub(crate) index_hints: Vec<crate::extension::mysql::IndexHint>,
}

/// List of distinct keywords that can be used in select statement
#[derive(Debug, Clone, PartialEq)]
pub enum SelectDistinct {
    All,
    Distinct,
    DistinctRow,
    DistinctOn(Vec<ColumnRef>),
}

/// Window type in [`SelectExpr`]
#[derive(Debug, Clone, PartialEq)]
pub enum WindowSelectType {
    /// Name in [`SelectStatement`]
    Name(DynIden),
    /// Inline query in [`SelectExpr`]
    Query(WindowStatement),
}

/// Select expression used in select statement
#[derive(Debug, Clone, PartialEq)]
pub struct SelectExpr {
    pub expr: SimpleExpr,
    pub alias: Option<DynIden>,
    pub window: Option<WindowSelectType>,
}

/// Join expression used in select statement
#[derive(Debug, Clone, PartialEq)]
pub struct JoinExpr {
    pub join: JoinType,
    pub table: Box<TableRef>,
    pub on: Option<JoinOn>,
    pub lateral: bool,
}

/// List of lock types that can be used in select statement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockType {
    /// Exclusive lock
    Update,
    NoKeyUpdate,
    /// Shared lock
    Share,
    KeyShare,
}

/// List of lock behavior can be used in select statement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockBehavior {
    Nowait,
    SkipLocked,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LockClause {
    pub(crate) r#type: LockType,
    pub(crate) tables: Vec<TableRef>,
    pub(crate) behavior: Option<LockBehavior>,
}

/// List of union types that can be used in union clause
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnionType {
    Intersect,
    Distinct,
    Except,
    All,
}

impl<T> From<T> for SelectExpr
where
    T: Into<SimpleExpr>,
{
    fn from(expr: T) -> Self {
        SelectExpr {
            expr: expr.into(),
            alias: None,
            window: None,
        }
    }
}

impl SelectStatement {
    /// Construct a new [`SelectStatement`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Take the ownership of data in the current [`SelectStatement`]
    pub fn take(&mut self) -> Self {
        Self {
            distinct: self.distinct.take(),
            selects: std::mem::take(&mut self.selects),
            from: std::mem::take(&mut self.from),
            join: std::mem::take(&mut self.join),
            r#where: std::mem::replace(&mut self.r#where, ConditionHolder::new()),
            groups: std::mem::take(&mut self.groups),
            having: std::mem::replace(&mut self.having, ConditionHolder::new()),
            unions: std::mem::take(&mut self.unions),
            orders: std::mem::take(&mut self.orders),
            limit: self.limit.take(),
            offset: self.offset.take(),
            lock: self.lock.take(),
            window: self.window.take(),
            with: self.with.take(),
            #[cfg(feature = "backend-postgres")]
            table_sample: std::mem::take(&mut self.table_sample),
            #[cfg(feature = "backend-mysql")]
            index_hints: std::mem::take(&mut self.index_hints),
        }
    }

    /// A shorthand to express if ... else ... when constructing the select statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     .conditions(
    ///         true,
    ///         |x| {
    ///             x.and_where(Expr::col(Char::FontId).eq(5));
    ///         },
    ///         |x| {
    ///             x.and_where(Expr::col(Char::FontId).eq(10));
    ///         },
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// ```
    pub fn conditions<T, F>(&mut self, b: bool, if_true: T, if_false: F) -> &mut Self
    where
        T: FnOnce(&mut Self),
        F: FnOnce(&mut Self),
    {
        if b {
            if_true(self)
        } else {
            if_false(self)
        }
        self
    }

    /// A shorthand to express if ... else ... when constructing the select statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     .apply_if(Some(5), |q, v| {
    ///         q.and_where(Expr::col(Char::FontId).eq(v));
    ///     })
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5"#
    /// );
    /// ```
    pub fn apply_if<T, F>(&mut self, val: Option<T>, if_some: F) -> &mut Self
    where
        Self: Sized,
        F: FnOnce(&mut Self, T),
    {
        if let Some(val) = val {
            if_some(self, val);
        }
        self
    }

    /// Construct part of the select statement in another function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let common_expr = |q: &mut SelectStatement| {
    ///     q.and_where(Expr::col(Char::FontId).eq(5));
    /// };
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     .apply(common_expr)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// ```
    pub fn apply<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(&mut Self),
    {
        func(self);
        self
    }

    /// Clear the select list
    pub fn clear_selects(&mut self) -> &mut Self {
        self.selects = Vec::new();
        self
    }

    /// Add an expression to the select expression list.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr(Expr::val(42))
    ///     .expr(Expr::col(Char::Id).max())
    ///     .expr((1..10_i32).fold(Expr::value(0), |expr, i| expr.add(i)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT 42, MAX(`id`), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT 42, MAX("id"), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT 42, MAX("id"), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM "character""#
    /// );
    /// ```
    pub fn expr<T>(&mut self, expr: T) -> &mut Self
    where
        T: Into<SelectExpr>,
    {
        self.selects.push(expr.into());
        self
    }

    /// Add select expressions from vector of [`SelectExpr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .exprs([
    ///         Expr::col(Char::Id).max(),
    ///         (1..10_i32).fold(Expr::value(0), |expr, i| expr.add(i)),
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MAX(`id`), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MAX("id"), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MAX("id"), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM "character""#
    /// );
    /// ```
    pub fn exprs<T, I>(&mut self, exprs: I) -> &mut Self
    where
        T: Into<SelectExpr>,
        I: IntoIterator<Item = T>,
    {
        self.selects
            .append(&mut exprs.into_iter().map(|c| c.into()).collect());
        self
    }

    pub fn exprs_mut_for_each<F>(&mut self, func: F)
    where
        F: FnMut(&mut SelectExpr),
    {
        self.selects.iter_mut().for_each(func);
    }

    /// Select distinct
    pub fn distinct(&mut self) -> &mut Self {
        self.distinct = Some(SelectDistinct::Distinct);
        self
    }

    /// Select distinct on for *POSTGRES ONLY*
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .distinct_on([Char::Character])
    ///     .column(Char::Character)
    ///     .column(Char::SizeW)
    ///     .column(Char::SizeH)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT DISTINCT ON ("character") "character", "size_w", "size_h" FROM "character""#
    /// )
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .distinct_on(vec![(Char::Table, Char::Character)])
    ///     .column(Char::Character)
    ///     .column(Char::SizeW)
    ///     .column(Char::SizeH)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT DISTINCT ON ("character"."character") "character", "size_w", "size_h" FROM "character""#
    /// )
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let distinct_cols: Vec<Character> = vec![];
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .distinct_on(distinct_cols)
    ///     .column(Char::Character)
    ///     .column(Char::SizeW)
    ///     .column(Char::SizeH)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character""#
    /// )
    /// ```
    pub fn distinct_on<T, I>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = T>,
    {
        let cols = cols
            .into_iter()
            .map(|col| col.into_column_ref())
            .collect::<Vec<ColumnRef>>();
        self.distinct = if !cols.is_empty() {
            Some(SelectDistinct::DistinctOn(cols))
        } else {
            None
        };
        self
    }

    /// Add a column to the select expression list.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .column(Char::Character)
    ///     .column(Char::SizeW)
    ///     .column(Char::SizeH)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character""#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .column((Char::Table, Char::Character))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`.`character` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character"."character" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character"."character" FROM "character""#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .column(("schema", Char::Table, Char::Character))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `schema`.`character`.`character` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "schema"."character"."character" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "schema"."character"."character" FROM "character""#
    /// );
    /// ```
    pub fn column<C>(&mut self, col: C) -> &mut Self
    where
        C: IntoColumnRef,
    {
        self.expr(SimpleExpr::Column(col.into_column_ref()))
    }

    /// Select columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character""#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .columns([
    ///         (Char::Table, Char::Character),
    ///         (Char::Table, Char::SizeW),
    ///         (Char::Table, Char::SizeH),
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`.`character`, `character`.`size_w`, `character`.`size_h` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character"."character", "character"."size_w", "character"."size_h" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character"."character", "character"."size_w", "character"."size_h" FROM "character""#
    /// );
    /// ```
    pub fn columns<T, I>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = T>,
    {
        self.exprs(
            cols.into_iter()
                .map(|c| SimpleExpr::Column(c.into_column_ref()))
                .collect::<Vec<SimpleExpr>>(),
        )
    }

    /// Select column.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr_as(Expr::col(Char::Character), "C")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` AS `C` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" AS "C" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" AS "C" FROM "character""#
    /// );
    /// ```
    pub fn expr_as<T, A>(&mut self, expr: T, alias: A) -> &mut Self
    where
        T: Into<SimpleExpr>,
        A: IntoIden,
    {
        self.expr(SelectExpr {
            expr: expr.into(),
            alias: Some(alias.into_iden()),
            window: None,
        });
        self
    }

    /// Select column with window function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr_window(
    ///         Expr::col(Char::Character),
    ///         WindowStatement::partition_by(Char::FontSize),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` OVER ( PARTITION BY `font_size` ) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" OVER ( PARTITION BY "font_size" ) FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" OVER ( PARTITION BY "font_size" ) FROM "character""#
    /// );
    /// ```
    pub fn expr_window<T>(&mut self, expr: T, window: WindowStatement) -> &mut Self
    where
        T: Into<SimpleExpr>,
    {
        self.expr(SelectExpr {
            expr: expr.into(),
            alias: None,
            window: Some(WindowSelectType::Query(window)),
        });
        self
    }

    /// Select column with window function and label.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr_window_as(
    ///         Expr::col(Char::Character),
    ///         WindowStatement::partition_by(Char::FontSize),
    ///         "C",
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` OVER ( PARTITION BY `font_size` ) AS `C` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" OVER ( PARTITION BY "font_size" ) AS "C" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" OVER ( PARTITION BY "font_size" ) AS "C" FROM "character""#
    /// );
    /// ```
    pub fn expr_window_as<T, A>(&mut self, expr: T, window: WindowStatement, alias: A) -> &mut Self
    where
        T: Into<SimpleExpr>,
        A: IntoIden,
    {
        self.expr(SelectExpr {
            expr: expr.into(),
            alias: Some(alias.into_iden()),
            window: Some(WindowSelectType::Query(window)),
        });
        self
    }

    /// Select column with window name.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr_window_name(Expr::col(Char::Character), "w")
    ///     .window("w", WindowStatement::partition_by(Char::FontSize))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` OVER `w` FROM `character` WINDOW `w` AS (PARTITION BY `font_size`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" OVER "w" FROM "character" WINDOW "w" AS (PARTITION BY "font_size")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" OVER "w" FROM "character" WINDOW "w" AS (PARTITION BY "font_size")"#
    /// );
    /// ```
    pub fn expr_window_name<T, W>(&mut self, expr: T, window: W) -> &mut Self
    where
        T: Into<SimpleExpr>,
        W: IntoIden,
    {
        self.expr(SelectExpr {
            expr: expr.into(),
            alias: None,
            window: Some(WindowSelectType::Name(window.into_iden())),
        });
        self
    }

    /// Select column with window name and label.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr_window_name_as(Expr::col(Char::Character), "w", "C")
    ///     .window("w", WindowStatement::partition_by(Char::FontSize))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` OVER `w` AS `C` FROM `character` WINDOW `w` AS (PARTITION BY `font_size`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" OVER "w" AS "C" FROM "character" WINDOW "w" AS (PARTITION BY "font_size")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" OVER "w" AS "C" FROM "character" WINDOW "w" AS (PARTITION BY "font_size")"#
    /// );
    /// ```
    pub fn expr_window_name_as<T, W, A>(&mut self, expr: T, window: W, alias: A) -> &mut Self
    where
        T: Into<SimpleExpr>,
        A: IntoIden,
        W: IntoIden,
    {
        self.expr(SelectExpr {
            expr: expr.into(),
            alias: Some(alias.into_iden()),
            window: Some(WindowSelectType::Name(window.into_iden())),
        });
        self
    }

    /// From table.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::FontSize)
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `font_size` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "font_size" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "font_size" FROM "character""#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::FontSize)
    ///     .from((Char::Table, Glyph::Table))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `font_size` FROM `character`.`glyph`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "font_size" FROM "character"."glyph""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "font_size" FROM "character"."glyph""#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::FontSize)
    ///     .from(("database", Char::Table, Glyph::Table))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `font_size` FROM `database`.`character`.`glyph`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "font_size" FROM "database"."character"."glyph""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "font_size" FROM "database"."character"."glyph""#
    /// );
    /// ```
    ///
    /// If you specify `from` multiple times, the resulting query will have multiple from clauses.
    /// You can perform an 'old-school' join this way.
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::asterisk())
    ///     .from(Char::Table)
    ///     .from(Font::Table)
    ///     .and_where(Expr::col((Font::Table, Font::Id)).equals((Char::Table, Char::FontId)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT * FROM `character`, `font` WHERE `font`.`id` = `character`.`font_id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT * FROM "character", "font" WHERE "font"."id" = "character"."font_id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT * FROM "character", "font" WHERE "font"."id" = "character"."font_id""#
    /// );
    /// ```
    pub fn from<R>(&mut self, tbl_ref: R) -> &mut Self
    where
        R: IntoTableRef,
    {
        self.from_from(tbl_ref.into_table_ref())
    }

    /// Shorthand for selecting from a constant value list.
    /// Panics on an empty values list.
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::asterisk())
    ///     .from_values([(1, "hello"), (2, "world")], "x")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT * FROM (VALUES ROW(1, 'hello'), ROW(2, 'world')) AS `x`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT * FROM (VALUES (1, 'hello'), (2, 'world')) AS "x""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT * FROM (VALUES (1, 'hello'), (2, 'world')) AS "x""#
    /// );
    /// ```
    pub fn from_values<I, V, A>(&mut self, value_tuples: I, alias: A) -> &mut Self
    where
        I: IntoIterator<Item = V>,
        V: IntoValueTuple,
        A: IntoIden,
    {
        let value_tuples: Vec<ValueTuple> = value_tuples
            .into_iter()
            .map(|vt| vt.into_value_tuple())
            .collect();
        assert!(!value_tuples.is_empty());
        self.from_from(TableRef::ValuesList(value_tuples, alias.into_iden()))
    }

    /// From table with alias.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table_as: DynIden = SeaRc::new("char");
    ///
    /// let query = Query::select()
    ///     .from_as(Char::Table, table_as.clone())
    ///     .column((table_as.clone(), Char::Character))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `char`.`character` FROM `character` AS `char`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "char"."character" FROM "character" AS "char""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "char"."character" FROM "character" AS "char""#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table_as = "alias";
    ///
    /// let query = Query::select()
    ///     .from_as((Font::Table, Char::Table), table_as.clone())
    ///     .column((table_as, Char::Character))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `alias`.`character` FROM `font`.`character` AS `alias`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "alias"."character" FROM "font"."character" AS "alias""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "alias"."character" FROM "font"."character" AS "alias""#
    /// );
    /// ```
    pub fn from_as<R, A>(&mut self, tbl_ref: R, alias: A) -> &mut Self
    where
        R: IntoTableRef,
        A: IntoIden,
    {
        self.from_from(tbl_ref.into_table_ref().alias(alias.into_iden()))
    }

    /// From sub-query.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Glyph::Image])
    ///     .from_subquery(
    ///         Query::select()
    ///             .columns([Glyph::Image, Glyph::Aspect])
    ///             .from(Glyph::Table)
    ///             .take(),
    ///         "subglyph",
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `image` FROM (SELECT `image`, `aspect` FROM `glyph`) AS `subglyph`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "image" FROM (SELECT "image", "aspect" FROM "glyph") AS "subglyph""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "image" FROM (SELECT "image", "aspect" FROM "glyph") AS "subglyph""#
    /// );
    /// ```
    pub fn from_subquery<T>(&mut self, query: SelectStatement, alias: T) -> &mut Self
    where
        T: IntoIden,
    {
        self.from_from(TableRef::SubQuery(query, alias.into_iden()))
    }

    /// From function call.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(ColumnRef::Asterisk)
    ///     .from_function(Func::random(), "func")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT * FROM RAND() AS `func`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT * FROM RANDOM() AS "func""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT * FROM RANDOM() AS "func""#
    /// );
    /// ```
    pub fn from_function<T>(&mut self, func: FunctionCall, alias: T) -> &mut Self
    where
        T: IntoIden,
    {
        self.from_from(TableRef::FunctionCall(func, alias.into_iden()))
    }

    /// Clears all current from clauses.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(ColumnRef::Asterisk)
    ///     .from(Char::Table)
    ///     .from_clear()
    ///     .from(Font::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT * FROM `font`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT * FROM "font""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT * FROM "font""#
    /// );
    /// ```
    pub fn from_clear(&mut self) -> &mut Self {
        self.from.clear();
        self
    }

    #[allow(clippy::wrong_self_convention)]
    fn from_from(&mut self, select: TableRef) -> &mut Self {
        self.from.push(select);
        self
    }

    /// Cross join.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .column((Font::Table, Font::Name))
    ///     .from(Char::Table)
    ///     .cross_join(Font::Table, Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` CROSS JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" CROSS JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" CROSS JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// let query = Query::select()
    ///         .column(Char::Character)
    ///         .column((Font::Table, Font::Name))
    ///         .from(Char::Table)
    ///         .cross_join(
    ///             Font::Table,
    ///             all![
    ///                 Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
    ///                 Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
    ///             ]
    ///         )
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` CROSS JOIN `font` ON `character`.`font_id` = `font`.`id` AND `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" CROSS JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" CROSS JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// ```
    pub fn cross_join<R, C>(&mut self, tbl_ref: R, condition: C) -> &mut Self
    where
        R: IntoTableRef,
        C: IntoCondition,
    {
        self.join(JoinType::CrossJoin, tbl_ref, condition)
    }

    /// Left join.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .column((Font::Table, Font::Name))
    ///     .from(Char::Table)
    ///     .left_join(Font::Table, Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// let query = Query::select()
    ///         .column(Char::Character)
    ///         .column((Font::Table, Font::Name))
    ///         .from(Char::Table)
    ///         .left_join(
    ///             Font::Table,
    ///             all![
    ///                 Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
    ///                 Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
    ///             ]
    ///         )
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id` AND `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// ```
    pub fn left_join<R, C>(&mut self, tbl_ref: R, condition: C) -> &mut Self
    where
        R: IntoTableRef,
        C: IntoCondition,
    {
        self.join(JoinType::LeftJoin, tbl_ref, condition)
    }

    /// Right join.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .column((Font::Table, Font::Name))
    ///     .from(Char::Table)
    ///     .right_join(Font::Table, Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// let query = Query::select()
    ///         .column(Char::Character)
    ///         .column((Font::Table, Font::Name))
    ///         .from(Char::Table)
    ///         .right_join(
    ///             Font::Table,
    ///             all![
    ///                 Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
    ///                 Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
    ///             ]
    ///         )
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id` AND `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// ```
    pub fn right_join<R, C>(&mut self, tbl_ref: R, condition: C) -> &mut Self
    where
        R: IntoTableRef,
        C: IntoCondition,
    {
        self.join(JoinType::RightJoin, tbl_ref, condition)
    }

    /// Inner join.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .column((Font::Table, Font::Name))
    ///     .from(Char::Table)
    ///     .inner_join(Font::Table, Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` INNER JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// let query = Query::select()
    ///         .column(Char::Character)
    ///         .column((Font::Table, Font::Name))
    ///         .from(Char::Table)
    ///         .inner_join(
    ///             Font::Table,
    ///             all![
    ///                 Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
    ///                 Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
    ///             ]
    ///         )
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` INNER JOIN `font` ON `character`.`font_id` = `font`.`id` AND `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// ```
    pub fn inner_join<R, C>(&mut self, tbl_ref: R, condition: C) -> &mut Self
    where
        R: IntoTableRef,
        C: IntoCondition,
    {
        self.join(JoinType::InnerJoin, tbl_ref, condition)
    }

    /// Full outer join.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .column((Font::Table, Font::Name))
    ///     .from(Char::Table)
    ///     .full_outer_join(Font::Table, Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" FULL OUTER JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" FULL OUTER JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// let query = Query::select()
    ///         .column(Char::Character)
    ///         .column((Font::Table, Font::Name))
    ///         .from(Char::Table)
    ///         .full_outer_join(
    ///             Font::Table,
    ///             all![
    ///                 Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
    ///                 Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
    ///             ]
    ///         )
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" FULL OUTER JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" FULL OUTER JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// ```
    pub fn full_outer_join<R, C>(&mut self, tbl_ref: R, condition: C) -> &mut Self
    where
        R: IntoTableRef,
        C: IntoCondition,
    {
        self.join(JoinType::FullOuterJoin, tbl_ref, condition)
    }

    /// Join with other table by [`JoinType`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .column((Font::Table, Font::Name))
    ///     .from(Char::Table)
    ///     .join(JoinType::RightJoin, Font::Table, Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// let query = Query::select()
    ///         .column(Char::Character)
    ///         .column((Font::Table, Font::Name))
    ///         .from(Char::Table)
    ///         .join(
    ///             JoinType::RightJoin,
    ///             Font::Table,
    ///             all![
    ///                 Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
    ///                 Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
    ///             ]
    ///         )
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id` AND `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// ```
    pub fn join<R, C>(&mut self, join: JoinType, tbl_ref: R, condition: C) -> &mut Self
    where
        R: IntoTableRef,
        C: IntoCondition,
    {
        self.join_join(
            join,
            tbl_ref.into_table_ref(),
            JoinOn::Condition(Box::new(ConditionHolder::new_with_condition(
                condition.into_condition(),
            ))),
            false,
        )
    }

    /// Join with other table by [`JoinType`], assigning an alias to the joined table.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .column((Font::Table, Font::Name))
    ///     .from(Char::Table)
    ///     .join_as(
    ///         JoinType::RightJoin,
    ///         Font::Table,
    ///         "f",
    ///         Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id))
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` AS `f` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" AS "f" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" AS "f" ON "character"."font_id" = "font"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// assert_eq!(
    ///     Query::select()
    ///         .column(Char::Character)
    ///         .column((Font::Table, Font::Name))
    ///         .from(Char::Table)
    ///         .join_as(
    ///             JoinType::RightJoin,
    ///             Font::Table,
    ///             "f",
    ///             Condition::all()
    ///                 .add(Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
    ///                 .add(Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
    ///         )
    ///         .to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` AS `f` ON `character`.`font_id` = `font`.`id` AND `character`.`font_id` = `font`.`id`"#
    /// );
    /// ```
    pub fn join_as<R, A, C>(
        &mut self,
        join: JoinType,
        tbl_ref: R,
        alias: A,
        condition: C,
    ) -> &mut Self
    where
        R: IntoTableRef,
        A: IntoIden,
        C: IntoCondition,
    {
        self.join_join(
            join,
            tbl_ref.into_table_ref().alias(alias.into_iden()),
            JoinOn::Condition(Box::new(ConditionHolder::new_with_condition(
                condition.into_condition(),
            ))),
            false,
        )
    }

    /// Join with sub-query.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let sub_glyph: DynIden = SeaRc::new("sub_glyph");
    /// let query = Query::select()
    ///     .column(Font::Name)
    ///     .from(Font::Table)
    ///     .join_subquery(
    ///         JoinType::LeftJoin,
    ///         Query::select().column(Glyph::Id).from(Glyph::Table).take(),
    ///         sub_glyph.clone(),
    ///         Expr::col((Font::Table, Font::Id)).equals((sub_glyph.clone(), Glyph::Id))
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `name` FROM `font` LEFT JOIN (SELECT `id` FROM `glyph`) AS `sub_glyph` ON `font`.`id` = `sub_glyph`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "name" FROM "font" LEFT JOIN (SELECT "id" FROM "glyph") AS "sub_glyph" ON "font"."id" = "sub_glyph"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "name" FROM "font" LEFT JOIN (SELECT "id" FROM "glyph") AS "sub_glyph" ON "font"."id" = "sub_glyph"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// assert_eq!(
    ///     Query::select()
    ///         .column(Font::Name)
    ///         .from(Font::Table)
    ///         .join_subquery(
    ///             JoinType::LeftJoin,
    ///             Query::select().column(Glyph::Id).from(Glyph::Table).take(),
    ///             sub_glyph.clone(),
    ///             Condition::all()
    ///                 .add(Expr::col((Font::Table, Font::Id)).equals((sub_glyph.clone(), Glyph::Id)))
    ///                 .add(Expr::col((Font::Table, Font::Id)).equals((sub_glyph.clone(), Glyph::Id)))
    ///         )
    ///         .to_string(MysqlQueryBuilder),
    ///     r#"SELECT `name` FROM `font` LEFT JOIN (SELECT `id` FROM `glyph`) AS `sub_glyph` ON `font`.`id` = `sub_glyph`.`id` AND `font`.`id` = `sub_glyph`.`id`"#
    /// );
    /// ```
    pub fn join_subquery<T, C>(
        &mut self,
        join: JoinType,
        query: SelectStatement,
        alias: T,
        condition: C,
    ) -> &mut Self
    where
        T: IntoIden,
        C: IntoCondition,
    {
        self.join_join(
            join,
            TableRef::SubQuery(query, alias.into_iden()),
            JoinOn::Condition(Box::new(ConditionHolder::new_with_condition(
                condition.into_condition(),
            ))),
            false,
        )
    }

    /// Join Lateral with sub-query. Not supported by SQLite.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let sub_glyph: DynIden = SeaRc::new("sub_glyph");
    /// let query = Query::select()
    ///     .column(Font::Name)
    ///     .from(Font::Table)
    ///     .join_lateral(
    ///         JoinType::LeftJoin,
    ///         Query::select().column(Glyph::Id).from(Glyph::Table).take(),
    ///         sub_glyph.clone(),
    ///         Expr::col((Font::Table, Font::Id)).equals((sub_glyph.clone(), Glyph::Id))
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `name` FROM `font` LEFT JOIN LATERAL (SELECT `id` FROM `glyph`) AS `sub_glyph` ON `font`.`id` = `sub_glyph`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "name" FROM "font" LEFT JOIN LATERAL (SELECT "id" FROM "glyph") AS "sub_glyph" ON "font"."id" = "sub_glyph"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// assert_eq!(
    ///     Query::select()
    ///         .column(Font::Name)
    ///         .from(Font::Table)
    ///         .join_lateral(
    ///             JoinType::LeftJoin,
    ///             Query::select().column(Glyph::Id).from(Glyph::Table).take(),
    ///             sub_glyph.clone(),
    ///             Condition::all()
    ///                 .add(Expr::col((Font::Table, Font::Id)).equals((sub_glyph.clone(), Glyph::Id)))
    ///                 .add(Expr::col((Font::Table, Font::Id)).equals((sub_glyph.clone(), Glyph::Id)))
    ///         )
    ///         .to_string(MysqlQueryBuilder),
    ///     r#"SELECT `name` FROM `font` LEFT JOIN LATERAL (SELECT `id` FROM `glyph`) AS `sub_glyph` ON `font`.`id` = `sub_glyph`.`id` AND `font`.`id` = `sub_glyph`.`id`"#
    /// );
    /// ```
    pub fn join_lateral<T, C>(
        &mut self,
        join: JoinType,
        query: SelectStatement,
        alias: T,
        condition: C,
    ) -> &mut Self
    where
        T: IntoIden,
        C: IntoCondition,
    {
        self.join_join(
            join,
            TableRef::SubQuery(query, alias.into_iden()),
            JoinOn::Condition(Box::new(ConditionHolder::new_with_condition(
                condition.into_condition(),
            ))),
            true,
        )
    }

    fn join_join(
        &mut self,
        join: JoinType,
        table: TableRef,
        on: JoinOn,
        lateral: bool,
    ) -> &mut Self {
        self.join.push(JoinExpr {
            join,
            table: Box::new(table),
            on: Some(on),
            lateral,
        });
        self
    }

    /// Group by columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .column((Font::Table, Font::Name))
    ///     .from(Char::Table)
    ///     .join(JoinType::RightJoin, Font::Table, Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
    ///     .group_by_columns([
    ///         Char::Character,
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id` GROUP BY `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character""#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .column((Font::Table, Font::Name))
    ///     .from(Char::Table)
    ///     .join(JoinType::RightJoin, Font::Table, Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
    ///     .group_by_columns([
    ///         (Char::Table, Char::Character),
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id` GROUP BY `character`.`character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character"."character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character"."character""#
    /// );
    /// ```
    pub fn group_by_columns<T, I>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = T>,
    {
        self.add_group_by(
            cols.into_iter()
                .map(|c| SimpleExpr::Column(c.into_column_ref()))
                .collect::<Vec<_>>(),
        )
    }

    /// Add a group by column.
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .column((Font::Table, Font::Name))
    ///     .from(Char::Table)
    ///     .join(JoinType::RightJoin, Font::Table, Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
    ///     .group_by_col((Char::Table, Char::Character))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id` GROUP BY `character`.`character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character"."character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character"."character""#
    /// );
    /// ```
    pub fn group_by_col<T>(&mut self, col: T) -> &mut Self
    where
        T: IntoColumnRef,
    {
        self.group_by_columns([col])
    }

    /// Add group by expressions from vector of [`SelectExpr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .column(Char::Character)
    ///     .add_group_by([Expr::col(Char::SizeW).into(), Expr::col(Char::SizeH).into()])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` GROUP BY `size_w`, `size_h`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" GROUP BY "size_w", "size_h""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" GROUP BY "size_w", "size_h""#
    /// );
    /// ```
    pub fn add_group_by<I>(&mut self, expr: I) -> &mut Self
    where
        I: IntoIterator<Item = SimpleExpr>,
    {
        self.groups.append(&mut expr.into_iter().collect());
        self
    }

    /// Having condition, expressed with [`any!`](crate::any) and [`all!`](crate::all).
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .expr(Expr::col(Glyph::Image).max())
    ///     .from(Glyph::Table)
    ///     .group_by_columns([
    ///         Glyph::Aspect,
    ///     ])
    ///     .cond_having(
    ///         all![
    ///             Expr::col((Glyph::Table, Glyph::Aspect)).is_in([3, 4]),
    ///             any![
    ///                 Expr::col((Glyph::Table, Glyph::Image)).like("A%"),
    ///                 Expr::col((Glyph::Table, Glyph::Image)).like("B%")
    ///             ]
    ///         ]
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect`, MAX(`image`) FROM `glyph` GROUP BY `aspect` HAVING `glyph`.`aspect` IN (3, 4) AND (`glyph`.`image` LIKE 'A%' OR `glyph`.`image` LIKE 'B%')"#
    /// );
    /// ```
    pub fn cond_having<C>(&mut self, condition: C) -> &mut Self
    where
        C: IntoCondition,
    {
        self.having.add_condition(condition.into_condition());
        self
    }

    /// And having condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .expr(Expr::col(Glyph::Image).max())
    ///     .from(Glyph::Table)
    ///     .group_by_columns([
    ///         Glyph::Aspect,
    ///     ])
    ///     .and_having(Expr::col(Glyph::Aspect).gt(2))
    ///     .cond_having(Expr::col(Glyph::Aspect).lt(8))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect`, MAX(`image`) FROM `glyph` GROUP BY `aspect` HAVING `aspect` > 2 AND `aspect` < 8"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect" HAVING "aspect" > 2 AND "aspect" < 8"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect" HAVING "aspect" > 2 AND "aspect" < 8"#
    /// );
    /// ```
    pub fn and_having(&mut self, other: SimpleExpr) -> &mut Self {
        self.cond_having(other)
    }

    /// Limit the number of returned rows.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .limit(10)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect` FROM `glyph` LIMIT 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "aspect" FROM "glyph" LIMIT 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "aspect" FROM "glyph" LIMIT 10"#
    /// );
    /// ```
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit.into());
        self
    }

    /// Reset limit
    pub fn reset_limit(&mut self) -> &mut Self {
        self.limit = None;
        self
    }

    /// Offset number of returned rows.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .limit(10)
    ///     .offset(10)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect` FROM `glyph` LIMIT 10 OFFSET 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "aspect" FROM "glyph" LIMIT 10 OFFSET 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "aspect" FROM "glyph" LIMIT 10 OFFSET 10"#
    /// );
    /// ```
    pub fn offset(&mut self, offset: u64) -> &mut Self {
        self.offset = Some(offset.into());
        self
    }

    /// Reset offset
    pub fn reset_offset(&mut self) -> &mut Self {
        self.offset = None;
        self
    }

    /// Row locking (if supported).
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::FontId).eq(5))
    ///     .lock(LockType::Update)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 FOR UPDATE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 FOR UPDATE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 "#
    /// );
    /// ```
    pub fn lock(&mut self, r#type: LockType) -> &mut Self {
        self.lock = Some(LockClause {
            r#type,
            tables: Vec::new(),
            behavior: None,
        });
        self
    }

    /// Row locking with tables (if supported).
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::FontId).eq(5))
    ///     .lock_with_tables(LockType::Update, [Glyph::Table])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 FOR UPDATE OF `glyph`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 FOR UPDATE OF "glyph""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 "#
    /// );
    /// ```
    pub fn lock_with_tables<T, I>(&mut self, r#type: LockType, tables: I) -> &mut Self
    where
        T: IntoTableRef,
        I: IntoIterator<Item = T>,
    {
        self.lock = Some(LockClause {
            r#type,
            tables: tables.into_iter().map(|t| t.into_table_ref()).collect(),
            behavior: None,
        });
        self
    }

    /// Row locking with behavior (if supported).
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::FontId).eq(5))
    ///     .lock_with_behavior(LockType::Update, LockBehavior::Nowait)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 FOR UPDATE NOWAIT"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 FOR UPDATE NOWAIT"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 "#
    /// );
    /// ```
    pub fn lock_with_behavior(&mut self, r#type: LockType, behavior: LockBehavior) -> &mut Self {
        self.lock = Some(LockClause {
            r#type,
            tables: Vec::new(),
            behavior: Some(behavior),
        });
        self
    }

    /// Row locking with tables and behavior (if supported).
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::FontId).eq(5))
    ///     .lock_with_tables_behavior(LockType::Update, [Glyph::Table], LockBehavior::Nowait)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 FOR UPDATE OF `glyph` NOWAIT"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 FOR UPDATE OF "glyph" NOWAIT"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 "#
    /// );
    /// ```
    pub fn lock_with_tables_behavior<T, I>(
        &mut self,
        r#type: LockType,
        tables: I,
        behavior: LockBehavior,
    ) -> &mut Self
    where
        T: IntoTableRef,
        I: IntoIterator<Item = T>,
    {
        self.lock = Some(LockClause {
            r#type,
            tables: tables.into_iter().map(|t| t.into_table_ref()).collect(),
            behavior: Some(behavior),
        });
        self
    }

    /// Shared row locking (if supported).
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::FontId).eq(5))
    ///     .lock_shared()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 FOR SHARE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 FOR SHARE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 "#
    /// );
    /// ```
    pub fn lock_shared(&mut self) -> &mut Self {
        self.lock(LockType::Share)
    }

    /// Exclusive row locking (if supported).
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::FontId).eq(5))
    ///     .lock_exclusive()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 FOR UPDATE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 FOR UPDATE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 "#
    /// );
    /// ```
    pub fn lock_exclusive(&mut self) -> &mut Self {
        self.lock(LockType::Update)
    }

    /// Union with another SelectStatement that must have the same selected fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::FontId).eq(5))
    ///     .union(UnionType::All, Query::select()
    ///         .column(Char::Character)
    ///         .from(Char::Table)
    ///         .and_where(Expr::col(Char::FontId).eq(4))
    ///         .to_owned()
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 UNION ALL (SELECT `character` FROM `character` WHERE `font_id` = 4)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 UNION ALL (SELECT "character" FROM "character" WHERE "font_id" = 4)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 UNION ALL SELECT "character" FROM "character" WHERE "font_id" = 4"#
    /// );
    /// ```
    pub fn union(&mut self, union_type: UnionType, query: SelectStatement) -> &mut Self {
        self.unions.push((union_type, query));
        self
    }

    /// Union with multiple SelectStatement that must have the same selected fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::FontId).eq(5))
    ///     .unions([
    ///         (UnionType::All, Query::select()
    ///             .column(Char::Character)
    ///             .from(Char::Table)
    ///             .and_where(Expr::col(Char::FontId).eq(4))
    ///             .to_owned()),
    ///         (UnionType::Distinct, Query::select()
    ///             .column(Char::Character)
    ///             .from(Char::Table)
    ///             .and_where(Expr::col(Char::FontId).eq(3))
    ///             .to_owned()),
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 UNION ALL (SELECT `character` FROM `character` WHERE `font_id` = 4) UNION (SELECT `character` FROM `character` WHERE `font_id` = 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 UNION ALL (SELECT "character" FROM "character" WHERE "font_id" = 4) UNION (SELECT "character" FROM "character" WHERE "font_id" = 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 UNION ALL SELECT "character" FROM "character" WHERE "font_id" = 4 UNION SELECT "character" FROM "character" WHERE "font_id" = 3"#
    /// );
    /// ```
    pub fn unions<T: IntoIterator<Item = (UnionType, SelectStatement)>>(
        &mut self,
        unions: T,
    ) -> &mut Self {
        self.unions.extend(unions);
        self
    }

    /// Create a [WithQuery] by specifying a [WithClause] to execute this query with.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, IntoCondition, IntoIden, tests_cfg::*};
    ///
    /// let base_query = SelectStatement::new()
    ///                     .column("id")
    ///                     .expr(1i32)
    ///                     .column("next")
    ///                     .column("value")
    ///                     .from("table")
    ///                     .to_owned();
    ///
    /// let cte_referencing = SelectStatement::new()
    ///                             .column("id")
    ///                             .expr(Expr::col("depth").add(1i32))
    ///                             .column("next")
    ///                             .column("value")
    ///                             .from("table")
    ///                             .join(
    ///                                 JoinType::InnerJoin,
    ///                                 "cte_traversal",
    ///                                 Expr::col(("cte_traversal", "next")).equals(("table", "id"))
    ///                             )
    ///                             .to_owned();
    ///
    /// let common_table_expression = CommonTableExpression::new()
    ///             .query(
    ///                 base_query.clone().union(UnionType::All, cte_referencing).to_owned()
    ///             )
    ///             .columns(["id", "depth", "next", "value"])
    ///             .table_name("cte_traversal")
    ///             .to_owned();
    ///
    /// let select = SelectStatement::new()
    ///         .column(ColumnRef::Asterisk)
    ///         .from("cte_traversal")
    ///         .to_owned();
    ///
    /// let with_clause = WithClause::new()
    ///         .recursive(true)
    ///         .cte(common_table_expression)
    ///         .cycle(Cycle::new_from_expr_set_using(SimpleExpr::Column(ColumnRef::Column("id".into_iden())), "looped", "traversal_path"))
    ///         .to_owned();
    ///
    /// let query = select.with(with_clause).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"WITH RECURSIVE `cte_traversal` (`id`, `depth`, `next`, `value`) AS (SELECT `id`, 1, `next`, `value` FROM `table` UNION ALL (SELECT `id`, `depth` + 1, `next`, `value` FROM `table` INNER JOIN `cte_traversal` ON `cte_traversal`.`next` = `table`.`id`)) SELECT * FROM `cte_traversal`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"WITH RECURSIVE "cte_traversal" ("id", "depth", "next", "value") AS (SELECT "id", 1, "next", "value" FROM "table" UNION ALL (SELECT "id", "depth" + 1, "next", "value" FROM "table" INNER JOIN "cte_traversal" ON "cte_traversal"."next" = "table"."id")) CYCLE "id" SET "looped" USING "traversal_path" SELECT * FROM "cte_traversal""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"WITH RECURSIVE "cte_traversal" ("id", "depth", "next", "value") AS (SELECT "id", 1, "next", "value" FROM "table" UNION ALL SELECT "id", "depth" + 1, "next", "value" FROM "table" INNER JOIN "cte_traversal" ON "cte_traversal"."next" = "table"."id") SELECT * FROM "cte_traversal""#
    /// );
    /// ```
    pub fn with(self, clause: WithClause) -> WithQuery {
        clause.query(self)
    }

    /// Create a Common Table Expression by specifying a [CommonTableExpression] or [WithClause] to execute this query with.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, IntoCondition, IntoIden, tests_cfg::*};
    ///
    /// let base_query = SelectStatement::new()
    ///                     .column("id")
    ///                     .expr(1i32)
    ///                     .column("next")
    ///                     .column("value")
    ///                     .from("table")
    ///                     .to_owned();
    ///
    /// let cte_referencing = SelectStatement::new()
    ///                             .column("id")
    ///                             .expr(Expr::col("depth").add(1i32))
    ///                             .column("next")
    ///                             .column("value")
    ///                             .from("table")
    ///                             .join(
    ///                                 JoinType::InnerJoin,
    ///                                 "cte_traversal",
    ///                                 Expr::col(("cte_traversal", "next")).equals(("table", "id"))
    ///                             )
    ///                             .to_owned();
    ///
    /// let common_table_expression = CommonTableExpression::new()
    ///             .query(
    ///                 base_query.clone().union(UnionType::All, cte_referencing).to_owned()
    ///             )
    ///             .columns(["id", "depth", "next", "value"])
    ///             .table_name("cte_traversal")
    ///             .to_owned();
    ///
    /// let with_clause = WithClause::new()
    ///         .recursive(true)
    ///         .cte(common_table_expression)
    ///         .cycle(Cycle::new_from_expr_set_using(SimpleExpr::Column(ColumnRef::Column("id".into_iden())), "looped", "traversal_path"))
    ///         .to_owned();
    ///
    /// let query = SelectStatement::new()
    ///         .column(ColumnRef::Asterisk)
    ///         .from("cte_traversal")
    ///         .with_cte(with_clause)
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"WITH RECURSIVE `cte_traversal` (`id`, `depth`, `next`, `value`) AS (SELECT `id`, 1, `next`, `value` FROM `table` UNION ALL (SELECT `id`, `depth` + 1, `next`, `value` FROM `table` INNER JOIN `cte_traversal` ON `cte_traversal`.`next` = `table`.`id`)) SELECT * FROM `cte_traversal`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"WITH RECURSIVE "cte_traversal" ("id", "depth", "next", "value") AS (SELECT "id", 1, "next", "value" FROM "table" UNION ALL (SELECT "id", "depth" + 1, "next", "value" FROM "table" INNER JOIN "cte_traversal" ON "cte_traversal"."next" = "table"."id")) CYCLE "id" SET "looped" USING "traversal_path" SELECT * FROM "cte_traversal""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"WITH RECURSIVE "cte_traversal" ("id", "depth", "next", "value") AS (SELECT "id", 1, "next", "value" FROM "table" UNION ALL SELECT "id", "depth" + 1, "next", "value" FROM "table" INNER JOIN "cte_traversal" ON "cte_traversal"."next" = "table"."id") SELECT * FROM "cte_traversal""#
    /// );
    /// ```
    pub fn with_cte<C: Into<WithClause>>(&mut self, clause: C) -> &mut Self {
        self.with = Some(clause.into());
        self
    }

    /// WINDOW
    ///
    /// # Examples:
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr_window_name_as(Expr::col(Char::Character), "w", "C")
    ///     .window("w", WindowStatement::partition_by(Char::FontSize))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` OVER `w` AS `C` FROM `character` WINDOW `w` AS (PARTITION BY `font_size`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" OVER "w" AS "C" FROM "character" WINDOW "w" AS (PARTITION BY "font_size")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" OVER "w" AS "C" FROM "character" WINDOW "w" AS (PARTITION BY "font_size")"#
    /// );
    /// ```
    pub fn window<A>(&mut self, name: A, window: WindowStatement) -> &mut Self
    where
        A: IntoIden,
    {
        self.window = Some((name.into_iden(), window));
        self
    }
}

#[inherent]
impl QueryStatementBuilder for SelectStatement {
    pub fn build_collect_any_into(
        &self,
        query_builder: &dyn QueryBuilder,
        sql: &mut dyn SqlWriter,
    ) {
        query_builder.prepare_select_statement(self, sql);
    }

    pub fn into_sub_query_statement(self) -> SubQueryStatement {
        SubQueryStatement::SelectStatement(self)
    }

    pub fn build_any(&self, query_builder: &dyn QueryBuilder) -> (String, Values);
    pub fn build_collect_any(
        &self,
        query_builder: &dyn QueryBuilder,
        sql: &mut dyn SqlWriter,
    ) -> String;
}

#[inherent]
impl QueryStatementWriter for SelectStatement {
    pub fn build_collect_into<T: QueryBuilder>(&self, query_builder: T, sql: &mut dyn SqlWriter) {
        query_builder.prepare_select_statement(self, sql);
    }

    pub fn build_collect<T: QueryBuilder>(
        &self,
        query_builder: T,
        sql: &mut dyn SqlWriter,
    ) -> String;
    pub fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Values);
    pub fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String;
}

#[inherent]
impl OrderedStatement for SelectStatement {
    pub fn add_order_by(&mut self, order: OrderExpr) -> &mut Self {
        self.orders.push(order);
        self
    }

    pub fn clear_order_by(&mut self) -> &mut Self {
        self.orders = Vec::new();
        self
    }

    pub fn order_by<T>(&mut self, col: T, order: Order) -> &mut Self
    where
        T: IntoColumnRef;

    pub fn order_by_expr(&mut self, expr: SimpleExpr, order: Order) -> &mut Self;
    pub fn order_by_customs<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: ToString,
        I: IntoIterator<Item = (T, Order)>;
    pub fn order_by_columns<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = (T, Order)>;
    pub fn order_by_with_nulls<T>(
        &mut self,
        col: T,
        order: Order,
        nulls: NullOrdering,
    ) -> &mut Self
    where
        T: IntoColumnRef;
    pub fn order_by_expr_with_nulls(
        &mut self,
        expr: SimpleExpr,
        order: Order,
        nulls: NullOrdering,
    ) -> &mut Self;
    pub fn order_by_customs_with_nulls<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: ToString,
        I: IntoIterator<Item = (T, Order, NullOrdering)>;
    pub fn order_by_columns_with_nulls<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = (T, Order, NullOrdering)>;
}

#[inherent]
impl ConditionalStatement for SelectStatement {
    pub fn and_or_where(&mut self, condition: LogicalChainOper) -> &mut Self {
        self.r#where.add_and_or(condition);
        self
    }

    pub fn cond_where<C>(&mut self, condition: C) -> &mut Self
    where
        C: IntoCondition,
    {
        self.r#where.add_condition(condition.into_condition());
        self
    }

    pub fn and_where_option(&mut self, other: Option<SimpleExpr>) -> &mut Self;
    pub fn and_where(&mut self, other: SimpleExpr) -> &mut Self;
}
