//! Building blocks of SQL statements.
//!
//! [`Expr`] representing the primitive building block in the expressions.
//!
//! [`SimpleExpr`] is the expression common among select fields, where clauses and many other places.

use crate::{func::*, query::*, types::*, value::*};

/// Helper to build a [`SimpleExpr`].
#[derive(Debug, Clone)]
pub struct Expr {
    pub(crate) left: SimpleExpr,
    pub(crate) right: Option<SimpleExpr>,
    pub(crate) uopr: Option<UnOper>,
    pub(crate) bopr: Option<BinOper>,
}

/// Represents a Simple Expression in SQL.
///
/// [`SimpleExpr`] is a node in the expression tree and can represent identifiers, function calls,
/// various operators and sub-queries.
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleExpr {
    Column(ColumnRef),
    Tuple(Vec<SimpleExpr>),
    Unary(UnOper, Box<SimpleExpr>),
    FunctionCall(FunctionCall),
    Binary(Box<SimpleExpr>, BinOper, Box<SimpleExpr>),
    SubQuery(Option<SubQueryOper>, Box<SubQueryStatement>),
    Value(Value),
    Values(Vec<Value>),
    Custom(String),
    CustomWithExpr(String, Vec<SimpleExpr>),
    Keyword(Keyword),
    AsEnum(DynIden, Box<SimpleExpr>),
    Case(Box<CaseStatement>),
    Constant(Value),
}

/// "Operator" methods for building expressions.
///
/// Before `sea_query` 0.32.0 (`sea_orm` 1.1.1),
/// these methods were awailable only on [`Expr`]/[`SimpleExpr`]
/// and you needed to manually construct these types first.
///
/// Now, you can call them directly on any expression type:
///
/// ```no_run
/// # use sea_query::*;
/// #
/// let expr = 1_i32.cast_as("REAL");
///
/// let expr = Func::char_length("abc").eq(3_i32);
///
/// let expr = Expr::current_date().cast_as("TEXT").like("2024%");
/// ```
pub trait ExprTrait: Sized {
    /// Express an arithmetic addition operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.add(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 + 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 + 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 + 1 = 2"#
    /// );
    /// ```
    fn add<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::Add, right)
    }

    /// Express a `AS enum` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Char::Table)
    ///     .columns([Char::FontSize])
    ///     .values_panic(["large".as_enum("FontSizeEnum")])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `character` (`font_size`) VALUES ('large')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "character" ("font_size") VALUES (CAST('large' AS "FontSizeEnum"))"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "character" ("font_size") VALUES ('large')"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn as_enum<N>(self, type_name: N) -> SimpleExpr
    where
        N: IntoIden;

    fn and<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::And, right)
    }

    /// Express a `BETWEEN` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().between(1, 10))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` BETWEEN 1 AND 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" BETWEEN 1 AND 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" BETWEEN 1 AND 10"#
    /// );
    /// ```
    fn between<A, B>(self, a: A, b: B) -> SimpleExpr
    where
        A: Into<SimpleExpr>,
        B: Into<SimpleExpr>,
    {
        self.binary(
            BinOper::Between,
            SimpleExpr::Binary(Box::new(a.into()), BinOper::And, Box::new(b.into())),
        )
    }

    /// Create any binary operation
    ///
    /// # Examples
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .cond_where(all![
    ///         Char::SizeW.into_column_ref().binary(BinOper::SmallerThan, 10),
    ///         Char::SizeW.into_column_ref().binary(BinOper::GreaterThan, Char::SizeH.into_column_ref())
    ///     ])
    ///     .to_owned();
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` < 10 AND `size_w` > `size_h`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" < 10 AND "size_w" > "size_h""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" < 10 AND "size_w" > "size_h""#
    /// );
    /// ```
    fn binary<O, R>(self, op: O, right: R) -> SimpleExpr
    where
        O: Into<BinOper>,
        R: Into<SimpleExpr>;

    /// Express a `CAST AS` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select().expr("1".cast_as("integer")).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT CAST('1' AS integer)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CAST('1' AS integer)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CAST('1' AS integer)"#
    /// );
    /// ```
    fn cast_as<N>(self, type_name: N) -> SimpleExpr
    where
        N: IntoIden;

    /// Express an arithmetic division operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.div(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 / 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 / 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 / 1 = 2"#
    /// );
    /// ```
    fn div<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::Div, right)
    }

    /// Express an equal (`=`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     // Sometimes, you'll have to qualify the call because of conflicting std traits.
    ///     .and_where(ExprTrait::eq("What!", "Nothing"))
    ///     .and_where(Char::Id.into_column_ref().eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 'What!' = 'Nothing' AND `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'What!' = 'Nothing' AND "id" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'What!' = 'Nothing' AND "id" = 1"#
    /// );
    /// ```
    fn eq<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::Equal, right)
    }

    /// Express a equal expression between two table columns,
    /// you will mainly use this to relate identical value between two table columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::FontId).into_column_ref().equals((Font::Table, Font::Id)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" = "font"."id""#
    /// );
    /// ```
    fn equals<C>(self, col: C) -> SimpleExpr
    where
        C: IntoColumnRef,
    {
        self.binary(BinOper::Equal, col.into_column_ref())
    }

    /// Express a greater than (`>`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().gt(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" > 2"#
    /// );
    /// ```
    fn gt<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::GreaterThan, right)
    }

    /// Express a greater than or equal (`>=`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().gte(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` >= 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" >= 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" >= 2"#
    /// );
    /// ```
    fn gte<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::GreaterThanOrEqual, right)
    }

    /// Express a `IN` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Char::SizeW.into_column_ref().in_subquery(
    ///         Query::select()
    ///             .expr(Expr::cust("3 + 2 * 2"))
    ///             .take()
    ///     ))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` IN (SELECT 3 + 2 * 2)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" IN (SELECT 3 + 2 * 2)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" IN (SELECT 3 + 2 * 2)"#
    /// );
    /// ```
    fn in_subquery(self, sel: SelectStatement) -> SimpleExpr {
        self.binary(
            BinOper::In,
            SimpleExpr::SubQuery(None, Box::new(sel.into_sub_query_statement())),
        )
    }

    /// Express a `IN` sub expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::FontId])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         ExprTrait::in_tuples(
    ///             Expr::tuple([
    ///                 Expr::col(Char::Character).into(),
    ///                 Expr::col(Char::FontId).into(),
    ///             ]),
    ///             [(1, String::from("1")), (2, String::from("2"))]
    ///         )
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font_id` FROM `character` WHERE (`character`, `font_id`) IN ((1, '1'), (2, '2'))"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font_id" FROM "character" WHERE ("character", "font_id") IN ((1, '1'), (2, '2'))"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font_id" FROM "character" WHERE ("character", "font_id") IN ((1, '1'), (2, '2'))"#
    /// );
    /// ```
    fn in_tuples<V, I>(self, v: I) -> SimpleExpr
    where
        V: IntoValueTuple,
        I: IntoIterator<Item = V>,
    {
        self.binary(
            BinOper::In,
            SimpleExpr::Tuple(
                v.into_iter()
                    .map(|m| SimpleExpr::Values(m.into_value_tuple().into_iter().collect()))
                    .collect(),
            ),
        )
    }

    /// Express a `IS` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::Ascii).into_column_ref().is(true))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`ascii` IS TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."ascii" IS TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."ascii" IS TRUE"#
    /// );
    /// ```
    fn is<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        self.binary(BinOper::Is, right)
    }

    /// Express a `IN` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Id])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         (Char::Table, Char::SizeW)
    ///             .into_column_ref()
    ///             .is_in([1, 2, 3]),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE `character`.`size_w` IN (1, 2, 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" IN (1, 2, 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" IN (1, 2, 3)"#
    /// );
    /// ```
    /// Empty value list
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Id])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         (Char::Table, Char::SizeW)
    ///             .into_column_ref()
    ///             .is_in(Vec::<u8>::new()),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE 1 = 2"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn is_in<V, I>(self, v: I) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
        I: IntoIterator<Item = V>,
    {
        self.binary(
            BinOper::In,
            SimpleExpr::Tuple(v.into_iter().map(|v| v.into()).collect()),
        )
    }

    /// Express a `IS NOT` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::Ascii).into_column_ref().is_not(true))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`ascii` IS NOT TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."ascii" IS NOT TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."ascii" IS NOT TRUE"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn is_not<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        self.binary(BinOper::IsNot, right)
    }

    /// Express a `NOT IN` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Id])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         (Char::Table, Char::SizeW)
    ///             .into_column_ref()
    ///             .is_not_in([1, 2, 3]),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE `character`.`size_w` NOT IN (1, 2, 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" NOT IN (1, 2, 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" NOT IN (1, 2, 3)"#
    /// );
    /// ```
    /// Empty value list
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Id])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         (Char::Table, Char::SizeW)
    ///             .into_column_ref()
    ///             .is_not_in(Vec::<u8>::new()),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE 1 = 1"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn is_not_in<V, I>(self, v: I) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
        I: IntoIterator<Item = V>,
    {
        self.binary(
            BinOper::NotIn,
            SimpleExpr::Tuple(v.into_iter().map(|v| v.into()).collect()),
        )
    }

    /// Express a `IS NOT NULL` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().is_not_null())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` IS NOT NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" IS NOT NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" IS NOT NULL"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn is_not_null(self) -> SimpleExpr {
        self.binary(BinOper::IsNot, Keyword::Null)
    }

    /// Express a `IS NULL` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().is_null())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" IS NULL"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn is_null(self) -> SimpleExpr {
        self.binary(BinOper::Is, Keyword::Null)
    }

    /// Express a bitwise left shift.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.left_shift(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 << 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 << 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 << 1 = 2"#
    /// );
    /// ```
    fn left_shift<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        self.binary(BinOper::LShift, right)
    }

    /// Express a `LIKE` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::Character).into_column_ref().like("Ours'%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`character` LIKE 'Ours\'%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" LIKE E'Ours\'%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" LIKE 'Ours''%'"#
    /// );
    /// ```
    ///
    /// Like with ESCAPE
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::Character).into_column_ref().like(LikeExpr::new(r"|_Our|_").escape('|')))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`character` LIKE '|_Our|_' ESCAPE '|'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" LIKE '|_Our|_' ESCAPE '|'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" LIKE '|_Our|_' ESCAPE '|'"#
    /// );
    /// ```
    fn like<L>(self, like: L) -> SimpleExpr
    where
        L: IntoLikeExpr,
    {
        ExprTrait::binary(self, BinOper::Like, like.into_like_expr())
    }

    /// Express a less than (`<`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().lt(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` < 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" < 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" < 2"#
    /// );
    /// ```
    fn lt<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::SmallerThan, right)
    }

    /// Express a less than or equal (`<=`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().lte(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` <= 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" <= 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" <= 2"#
    /// );
    /// ```
    fn lte<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::SmallerThanOrEqual, right)
    }

    /// Express an arithmetic modulo operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.modulo(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 % 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 % 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 % 1 = 2"#
    /// );
    /// ```
    fn modulo<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        self.binary(BinOper::Mod, right)
    }

    /// Express an arithmetic multiplication operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.mul(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 * 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 * 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 * 1 = 2"#
    /// );
    /// ```
    fn mul<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::Mul, right)
    }

    /// Express a not equal (`<>`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     // Sometimes, you'll have to qualify the call because of conflicting std traits.
    ///     .and_where(ExprTrait::ne("Morning", "Good"))
    ///     .and_where(Char::Id.into_column_ref().ne(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 'Morning' <> 'Good' AND `id` <> 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'Morning' <> 'Good' AND "id" <> 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'Morning' <> 'Good' AND "id" <> 1"#
    /// );
    /// ```
    fn ne<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::NotEqual, right)
    }

    /// Negates an expression with `NOT`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(ExprTrait::not(Expr::col((Char::Table, Char::SizeW)).is_null()))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE NOT `character`.`size_w` IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE NOT "character"."size_w" IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE NOT "character"."size_w" IS NULL"#
    /// );
    /// ```
    fn not(self) -> SimpleExpr {
        self.unary(UnOper::Not)
    }

    /// Express a `NOT BETWEEN` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::SizeW).into_column_ref().not_between(1, 10))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` NOT BETWEEN 1 AND 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" NOT BETWEEN 1 AND 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" NOT BETWEEN 1 AND 10"#
    /// );
    /// ```
    fn not_between<A, B>(self, a: A, b: B) -> SimpleExpr
    where
        A: Into<SimpleExpr>,
        B: Into<SimpleExpr>,
    {
        self.binary(
            BinOper::NotBetween,
            SimpleExpr::Binary(Box::new(a.into()), BinOper::And, Box::new(b.into())),
        )
    }

    /// Express a not equal expression between two table columns,
    /// you will mainly use this to relate identical value between two table columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::FontId).into_column_ref().not_equals((Font::Table, Font::Id)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`font_id` <> `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" <> "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" <> "font"."id""#
    /// );
    /// ```
    fn not_equals<C>(self, col: C) -> SimpleExpr
    where
        C: IntoColumnRef,
    {
        self.binary(BinOper::NotEqual, col.into_column_ref())
    }

    /// Express a `NOT IN` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Char::SizeW.into_column_ref().not_in_subquery(
    ///         Query::select()
    ///             .expr(Expr::cust("3 + 2 * 2"))
    ///             .take()
    ///     ))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` NOT IN (SELECT 3 + 2 * 2)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" NOT IN (SELECT 3 + 2 * 2)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" NOT IN (SELECT 3 + 2 * 2)"#
    /// );
    /// ```
    fn not_in_subquery(self, sel: SelectStatement) -> SimpleExpr {
        self.binary(
            BinOper::NotIn,
            SimpleExpr::SubQuery(None, Box::new(sel.into_sub_query_statement())),
        )
    }

    /// Express a `NOT LIKE` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where((Char::Table, Char::Character).into_column_ref().not_like("Ours'%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`character` NOT LIKE 'Ours\'%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" NOT LIKE E'Ours\'%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" NOT LIKE 'Ours''%'"#
    /// );
    /// ```
    fn not_like<L>(self, like: L) -> SimpleExpr
    where
        L: IntoLikeExpr,
    {
        ExprTrait::binary(self, BinOper::NotLike, like.into_like_expr())
    }

    /// Express a logical `OR` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(false.or(true))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE FALSE OR TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE FALSE OR TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE FALSE OR TRUE"#
    /// );
    /// ```
    fn or<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::Or, right)
    }

    /// Express a bitwise right shift.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.right_shift(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 >> 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 >> 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 >> 1 = 2"#
    /// );
    /// ```
    fn right_shift<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        self.binary(BinOper::RShift, right)
    }

    /// Express an arithmetic subtraction operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.sub(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 - 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 - 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 - 1 = 2"#
    /// );
    /// ```
    fn sub<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::Sub, right)
    }

    /// Apply any unary operator to the expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).is_null().unary(UnOper::Not))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE NOT `character`.`size_w` IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE NOT "character"."size_w" IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE NOT "character"."size_w" IS NULL"#
    /// );
    /// ```
    fn unary(self, o: UnOper) -> SimpleExpr;

    /// Express a bitwise AND operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select().expr(1.bit_and(2).eq(3)).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT (1 & 2) = 3"#
    /// );
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.bit_and(1).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (1 & 1) = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE (1 & 1) = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE (1 & 1) = 1"#
    /// );
    /// ```
    fn bit_and<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::BitAnd, right)
    }

    /// Express a bitwise OR operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(1.bit_or(1).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (1 | 1) = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE (1 | 1) = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE (1 | 1) = 1"#
    /// );
    /// ```
    fn bit_or<R>(self, right: R) -> SimpleExpr
    where
        R: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, BinOper::BitOr, right)
    }
}

/// This generic implementation covers all expression types,
/// including [ColumnRef], [Value], [FunctionCall], [SimpleExpr]...
impl<T> ExprTrait for T
where
    T: Into<SimpleExpr>,
{
    fn as_enum<N>(self, type_name: N) -> SimpleExpr
    where
        N: IntoIden,
    {
        SimpleExpr::AsEnum(type_name.into_iden(), Box::new(self.into()))
    }

    fn binary<O, R>(self, op: O, right: R) -> SimpleExpr
    where
        O: Into<BinOper>,
        R: Into<SimpleExpr>,
    {
        SimpleExpr::Binary(Box::new(self.into()), op.into(), Box::new(right.into()))
    }

    fn cast_as<N>(self, type_name: N) -> SimpleExpr
    where
        N: IntoIden,
    {
        SimpleExpr::FunctionCall(Func::cast_as(self, type_name))
    }

    fn unary(self, op: UnOper) -> SimpleExpr {
        SimpleExpr::Unary(op, Box::new(self.into()))
    }
}

impl Expr {
    fn new_with_left<T>(left: T) -> Self
    where
        T: Into<SimpleExpr>,
    {
        let left = left.into();
        Self {
            left,
            right: None,
            uopr: None,
            bopr: None,
        }
    }

    #[deprecated(since = "0.29.0", note = "Please use the [`Asterisk`]")]
    pub fn asterisk() -> Self {
        Self::col(Asterisk)
    }

    /// Express the target column without table prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::SizeW).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// ```
    pub fn col<T>(n: T) -> Self
    where
        T: IntoColumnRef,
    {
        Self::new_with_left(n.into_column_ref())
    }

    /// Express the target column without table prefix, returning a [`SimpleExpr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::column(Char::SizeW).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::column((Char::Table, Char::SizeW)).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// ```
    pub fn column<T>(n: T) -> SimpleExpr
    where
        T: IntoColumnRef,
    {
        SimpleExpr::Column(n.into_column_ref())
    }

    /// Wraps tuple of `SimpleExpr`, can be used for tuple comparison
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         Expr::tuple([Expr::col(Char::SizeW).into(), Expr::value(100)])
    ///             .lt(Expr::tuple([Expr::value(500), Expr::value(100)])))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (`size_w`, 100) < (500, 100)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w", 100) < (500, 100)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w", 100) < (500, 100)"#
    /// );
    /// ```
    pub fn tuple<I>(n: I) -> Self
    where
        I: IntoIterator<Item = SimpleExpr>,
    {
        Expr::expr(SimpleExpr::Tuple(
            n.into_iter().collect::<Vec<SimpleExpr>>(),
        ))
    }

    #[deprecated(since = "0.29.0", note = "Please use the [`Asterisk`]")]
    pub fn table_asterisk<T>(t: T) -> Self
    where
        T: IntoIden,
    {
        Self::col((t.into_iden(), Asterisk))
    }

    /// Express a [`Value`], returning a [`Expr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::val(1).into())
    ///     .and_where(Expr::val(2.5).into())
    ///     .and_where(Expr::val("3").into())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// ```
    pub fn val<V>(v: V) -> Self
    where
        V: Into<Value>,
    {
        Self::new_with_left(v)
    }

    /// Wrap an expression to perform some operation on it later.
    ///
    /// Since `sea_query` 0.32.0 (`sea_orm` 1.1.1), **this is not necessary** in most cases!
    ///
    /// Some SQL operations used to be defined only as inherent methods on [`Expr`].
    /// Thus, to use them, you needed to manually convert from other types to [`Expr`].
    /// But now these operations are also defined as [`ExprTrait`] methods
    /// that can be called directly on any expression type,
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     // This is the old style, when `Expr::expr` was necessary:
    ///     .and_where(Expr::expr(Expr::col(Char::SizeW).if_null(0)).gt(2))
    ///     .to_owned();
    ///
    /// // But since 0.32.0, this compiles too:
    /// let _ = Expr::col(Char::SizeW).if_null(0).gt(2);
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE IFNULL(`size_w`, 0) > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE COALESCE("size_w", 0) > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE IFNULL("size_w", 0) > 2"#
    /// );
    /// ```
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Character)
    ///     .from(Char::Table)
    ///     // This is the old style, when `Expr::expr` was necessary:
    ///     .and_where(Expr::expr(Func::lower(Expr::col(Char::Character))).is_in(["a", "b"]))
    ///     .to_owned();
    ///
    /// // But since 0.32.0, this compiles too:
    /// let _ = Func::lower(Expr::col(Char::Character)).is_in(["a", "b"]);
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` FROM `character` WHERE LOWER(`character`) IN ('a', 'b')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE LOWER("character") IN ('a', 'b')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" FROM "character" WHERE LOWER("character") IN ('a', 'b')"#
    /// );
    /// ```
    #[allow(clippy::self_named_constructors)]
    pub fn expr<T>(expr: T) -> Self
    where
        T: Into<SimpleExpr>,
    {
        Self::new_with_left(expr)
    }

    /// Express a [`Value`], returning a [`SimpleExpr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::value(1))
    ///     .and_where(Expr::value(2.5))
    ///     .and_where(Expr::value("3"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// ```
    pub fn value<V>(v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        v.into()
    }

    /// Express any custom expression in [`&str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::cust("1 = 1"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 = 1"#
    /// );
    /// ```
    pub fn cust<T>(s: T) -> SimpleExpr
    where
        T: Into<String>,
    {
        SimpleExpr::Custom(s.into())
    }

    /// Express any custom expression with [`Value`]. Use this if your expression needs variables.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::Id).eq(1))
    ///     .and_where(Expr::cust_with_values("6 = ? * ?", [2, 3]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `id` = 1 AND (6 = 2 * 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "id" = 1 AND (6 = 2 * 3)"#
    /// );
    /// ```
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::Id).eq(1))
    ///     .and_where(Expr::cust_with_values("6 = $2 * $1", [3, 2]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "id" = 1 AND (6 = 2 * 3)"#
    /// );
    /// ```
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_values("6 = ? * ?", [2, 3]))
    ///     .to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT 6 = 2 * 3"#);
    /// assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT 6 = 2 * 3"#);
    /// ```
    /// Postgres only: use `$$` to escape `$`
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_values("$1 $$ $2", ["a", "b"]))
    ///     .to_owned();
    ///
    /// assert_eq!(query.to_string(PostgresQueryBuilder), r#"SELECT 'a' $ 'b'"#);
    /// ```
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_values("data @? ($1::JSONPATH)", ["hello"]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT data @? ('hello'::JSONPATH)"#
    /// );
    /// ```
    pub fn cust_with_values<T, V, I>(s: T, v: I) -> SimpleExpr
    where
        T: Into<String>,
        V: Into<Value>,
        I: IntoIterator<Item = V>,
    {
        SimpleExpr::CustomWithExpr(
            s.into(),
            v.into_iter()
                .map(|v| Into::<Value>::into(v).into())
                .collect(),
        )
    }

    /// Express any custom expression with [`SimpleExpr`]. Use this if your expression needs other expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::val(1).add(2))
    ///     .expr(Expr::cust_with_expr("data @? ($1::JSONPATH)", "hello"))
    ///     .to_owned();
    /// let (sql, values) = query.build(PostgresQueryBuilder);
    ///
    /// assert_eq!(sql, r#"SELECT $1 + $2, data @? ($3::JSONPATH)"#);
    /// assert_eq!(
    ///     values,
    ///     Values(vec![1i32.into(), 2i32.into(), "hello".into()])
    /// );
    /// ```
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_expr(
    ///         "json_agg(DISTINCT $1)",
    ///         Expr::col(Char::Character),
    ///     ))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT json_agg(DISTINCT "character")"#
    /// );
    /// ```
    pub fn cust_with_expr<T, E>(s: T, expr: E) -> SimpleExpr
    where
        T: Into<String>,
        E: Into<SimpleExpr>,
    {
        SimpleExpr::CustomWithExpr(s.into(), vec![expr.into()])
    }

    /// Express any custom expression with [`SimpleExpr`]. Use this if your expression needs other expressions.
    pub fn cust_with_exprs<T, I>(s: T, v: I) -> SimpleExpr
    where
        T: Into<String>,
        I: IntoIterator<Item = SimpleExpr>,
    {
        SimpleExpr::CustomWithExpr(s.into(), v.into_iter().collect())
    }

    /// Express an equal (`=`) expression.
    ///
    /// This is equivalent to a newer [ExprTrait::eq] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::val("What!").eq("Nothing"))
    ///     .and_where(Expr::col(Char::Id).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 'What!' = 'Nothing' AND `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'What!' = 'Nothing' AND "id" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'What!' = 'Nothing' AND "id" = 1"#
    /// );
    /// ```
    pub fn eq<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::eq(self, v)
    }

    /// Express a not equal (`<>`) expression.
    ///
    /// This is equivalent to a newer [ExprTrait::ne] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::val("Morning").ne("Good"))
    ///     .and_where(Expr::col(Char::Id).ne(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 'Morning' <> 'Good' AND `id` <> 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'Morning' <> 'Good' AND "id" <> 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'Morning' <> 'Good' AND "id" <> 1"#
    /// );
    /// ```
    pub fn ne<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::ne(self, v)
    }

    /// Express a equal expression between two table columns,
    /// you will mainly use this to relate identical value between two table columns.
    ///
    /// This is equivalent to a newer [ExprTrait::equals] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" = "font"."id""#
    /// );
    /// ```
    pub fn equals<C>(self, col: C) -> SimpleExpr
    where
        C: IntoColumnRef,
    {
        ExprTrait::equals(self, col)
    }

    /// Express a not equal expression between two table columns,
    /// you will mainly use this to relate identical value between two table columns.
    ///
    /// This is equivalent to a newer [ExprTrait::not_equals] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::FontId)).not_equals((Font::Table, Font::Id)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`font_id` <> `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" <> "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" <> "font"."id""#
    /// );
    /// ```
    pub fn not_equals<C>(self, col: C) -> SimpleExpr
    where
        C: IntoColumnRef,
    {
        ExprTrait::not_equals(self, col)
    }

    /// Express a greater than (`>`) expression.
    ///
    /// This is equivalent to a newer [ExprTrait::gt] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).gt(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" > 2"#
    /// );
    /// ```
    pub fn gt<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::gt(self, v)
    }

    /// Express a greater than or equal (`>=`) expression.
    ///
    /// This is equivalent to a newer [ExprTrait::gte] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).gte(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` >= 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" >= 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" >= 2"#
    /// );
    /// ```
    pub fn gte<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::gte(self, v)
    }

    /// Express a less than (`<`) expression.
    ///
    /// This is equivalent to a newer [ExprTrait::lt] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).lt(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` < 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" < 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" < 2"#
    /// );
    /// ```
    pub fn lt<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::lt(self, v)
    }

    /// Express a less than or equal (`<=`) expression.
    ///
    /// This is equivalent to a newer [ExprTrait::lte] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).lte(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` <= 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" <= 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" <= 2"#
    /// );
    /// ```
    pub fn lte<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::lte(self, v)
    }

    /// Express an arithmetic addition operation.
    ///
    /// This is equivalent to a newer [ExprTrait::add] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::val(1).add(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 + 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 + 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 + 1 = 2"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn add<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::add(self, v)
    }

    /// Express an arithmetic subtraction operation.
    ///
    /// This is equivalent to a newer [ExprTrait::sub] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::val(1).sub(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 - 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 - 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 - 1 = 2"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn sub<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::sub(self, v)
    }

    /// Express an arithmetic multiplication operation.
    ///
    /// This is equivalent to a newer [ExprTrait::mul] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::val(1).mul(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 * 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 * 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 * 1 = 2"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn mul<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::mul(self, v)
    }

    /// Express an arithmetic division operation.
    ///
    /// This is equivalent to a newer [ExprTrait::div] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::val(1).div(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 / 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 / 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 / 1 = 2"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn div<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::div(self, v)
    }

    /// Express an arithmetic modulo operation.
    ///
    /// This is equivalent to a newer [ExprTrait::modulo] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::val(1).modulo(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 % 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 % 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 % 1 = 2"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn modulo<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::modulo(self, v)
    }

    /// Express a bitwise left shift.
    ///
    /// This is equivalent to a newer [ExprTrait::left_shift] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::val(1).left_shift(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 << 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 << 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 << 1 = 2"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn left_shift<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::left_shift(self, v)
    }

    /// Express a bitwise right shift.
    ///
    /// This is equivalent to a newer [ExprTrait::right_shift] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::val(1).right_shift(1).eq(2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 >> 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 >> 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 >> 1 = 2"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn right_shift<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::right_shift(self, v)
    }

    /// Express a `BETWEEN` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::between] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).between(1, 10))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` BETWEEN 1 AND 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" BETWEEN 1 AND 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" BETWEEN 1 AND 10"#
    /// );
    /// ```
    pub fn between<V>(self, a: V, b: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::between(self, a, b)
    }

    /// Express a `NOT BETWEEN` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::not_between] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).not_between(1, 10))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` NOT BETWEEN 1 AND 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" NOT BETWEEN 1 AND 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" NOT BETWEEN 1 AND 10"#
    /// );
    /// ```
    pub fn not_between<V>(self, a: V, b: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::not_between(self, a, b)
    }

    /// Express a `LIKE` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::like] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::Character)).like("Ours'%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`character` LIKE 'Ours\'%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" LIKE E'Ours\'%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" LIKE 'Ours''%'"#
    /// );
    /// ```
    ///
    /// Like with ESCAPE
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::Character)).like(LikeExpr::new(r"|_Our|_").escape('|')))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`character` LIKE '|_Our|_' ESCAPE '|'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" LIKE '|_Our|_' ESCAPE '|'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" LIKE '|_Our|_' ESCAPE '|'"#
    /// );
    /// ```
    pub fn like<L: IntoLikeExpr>(self, like: L) -> SimpleExpr {
        ExprTrait::like(self, like)
    }

    /// Express a `NOT LIKE` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::not_like] and may require more some wrapping beforehand.
    pub fn not_like<L: IntoLikeExpr>(self, like: L) -> SimpleExpr {
        ExprTrait::not_like(self, like)
    }

    /// Express a `IS NULL` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::is_null] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).is_null())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" IS NULL"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn is_null(self) -> SimpleExpr {
        ExprTrait::is_null(self)
    }

    /// Express a `IS` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::is] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::Ascii)).is(true))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`ascii` IS TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."ascii" IS TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."ascii" IS TRUE"#
    /// );
    /// ```
    pub fn is<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::is(self, v)
    }

    /// Express a `IS NOT NULL` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::is_not_null] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).is_not_null())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` IS NOT NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" IS NOT NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" IS NOT NULL"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn is_not_null(self) -> SimpleExpr {
        ExprTrait::is_not_null(self)
    }

    /// Express a `IS NOT` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::is_not] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::Ascii)).is_not(true))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`ascii` IS NOT TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."ascii" IS NOT TRUE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."ascii" IS NOT TRUE"#
    /// );
    /// ```
    pub fn is_not<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::is_not(self, v)
    }

    /// Create any binary operation
    ///
    /// This is equivalent to a newer [ExprTrait::binary] and may require more some wrapping beforehand.
    ///
    /// # Examples
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .cond_where(all![
    ///         Expr::col(Char::SizeW).binary(BinOper::SmallerThan, 10),
    ///         Expr::col(Char::SizeW).binary(BinOper::GreaterThan, Expr::col(Char::SizeH))
    ///     ])
    ///     .to_owned();
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` < 10 AND `size_w` > `size_h`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" < 10 AND "size_w" > "size_h""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" < 10 AND "size_w" > "size_h""#
    /// );
    /// ```
    pub fn binary<O, T>(self, op: O, right: T) -> SimpleExpr
    where
        O: Into<BinOper>,
        T: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, op, right)
    }

    /// Negates an expression with `NOT`.
    ///
    /// This is equivalent to a newer [ExprTrait::not] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     // Before 0.32.0, you had call `not` on an `Expr`, which had to be constructed using `Expr::expr`:
    ///     .and_where(Expr::expr(Expr::col((Char::Table, Char::SizeW)).is_null()).not())
    ///     .to_owned();
    ///
    /// // But since 0.32.0, this compiles too:
    /// let _ = Expr::col((Char::Table, Char::SizeW)).is_null().not();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE NOT `character`.`size_w` IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE NOT "character"."size_w" IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE NOT "character"."size_w" IS NULL"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> SimpleExpr {
        ExprTrait::not(self)
    }

    /// Express a `MAX` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).max())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MAX(`character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MAX("character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MAX("character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn max(self) -> SimpleExpr {
        Func::max(self.left).into()
    }

    /// Express a `MIN` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).min())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MIN(`character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MIN("character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MIN("character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn min(self) -> SimpleExpr {
        Func::min(self.left).into()
    }

    /// Express a `SUM` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).sum())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT SUM(`character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT SUM("character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT SUM("character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn sum(self) -> SimpleExpr {
        Func::sum(self.left).into()
    }

    /// Express a `COUNT` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).count())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT COUNT(`character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COUNT("character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT COUNT("character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn count(self) -> SimpleExpr {
        Func::count(self.left).into()
    }

    /// Express a `COUNT` function with the DISTINCT modifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).count_distinct())
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT COUNT(DISTINCT `character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COUNT(DISTINCT "character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT COUNT(DISTINCT "character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn count_distinct(self) -> SimpleExpr {
        Func::count_distinct(self.left).into()
    }

    /// Express a `IF NULL` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col((Char::Table, Char::SizeW)).if_null(0))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT IFNULL(`character`.`size_w`, 0) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT COALESCE("character"."size_w", 0) FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT IFNULL("character"."size_w", 0) FROM "character""#
    /// );
    /// ```
    pub fn if_null<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        Func::if_null(self.left, v).into()
    }

    /// Express a `IN` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::is_in] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Id])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).is_in([1, 2, 3]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE `character`.`size_w` IN (1, 2, 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" IN (1, 2, 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" IN (1, 2, 3)"#
    /// );
    /// ```
    /// Empty value list
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Id])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).is_in(Vec::<u8>::new()))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE 1 = 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE 1 = 2"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn is_in<V, I>(self, v: I) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
        I: IntoIterator<Item = V>,
    {
        ExprTrait::is_in(self, v)
    }

    /// Express a `IN` sub expression.
    ///
    /// This is equivalent to a newer [ExprTrait::in_tuples] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::FontId])
    ///     .from(Char::Table)
    ///     .and_where(
    ///         Expr::tuple([
    ///             Expr::col(Char::Character).into(),
    ///             Expr::col(Char::FontId).into(),
    ///         ])
    ///         .in_tuples([(1, String::from("1")), (2, String::from("2"))])
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `font_id` FROM `character` WHERE (`character`, `font_id`) IN ((1, '1'), (2, '2'))"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "font_id" FROM "character" WHERE ("character", "font_id") IN ((1, '1'), (2, '2'))"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "font_id" FROM "character" WHERE ("character", "font_id") IN ((1, '1'), (2, '2'))"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn in_tuples<V, I>(self, v: I) -> SimpleExpr
    where
        V: IntoValueTuple,
        I: IntoIterator<Item = V>,
    {
        ExprTrait::in_tuples(self, v)
    }

    /// Express a `NOT IN` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::is_not_in] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Id])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).is_not_in([1, 2, 3]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE `character`.`size_w` NOT IN (1, 2, 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" NOT IN (1, 2, 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" NOT IN (1, 2, 3)"#
    /// );
    /// ```
    /// Empty value list
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Id])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::SizeW)).is_not_in(Vec::<u8>::new()))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE 1 = 1"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn is_not_in<V, I>(self, v: I) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
        I: IntoIterator<Item = V>,
    {
        ExprTrait::is_not_in(self, v)
    }

    /// Express a `IN` sub-query expression.
    ///
    /// This is equivalent to a newer [ExprTrait::in_subquery] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::SizeW).in_subquery(
    ///         Query::select()
    ///             .expr(Expr::cust("3 + 2 * 2"))
    ///             .take()
    ///     ))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` IN (SELECT 3 + 2 * 2)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" IN (SELECT 3 + 2 * 2)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" IN (SELECT 3 + 2 * 2)"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn in_subquery(self, sel: SelectStatement) -> SimpleExpr {
        ExprTrait::in_subquery(self, sel)
    }

    /// Express a `NOT IN` sub-query expression.
    ///
    /// This is equivalent to a newer [ExprTrait::not_in_subquery] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::SizeW).not_in_subquery(
    ///         Query::select()
    ///             .expr(Expr::cust("3 + 2 * 2"))
    ///             .take()
    ///     ))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` NOT IN (SELECT 3 + 2 * 2)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" NOT IN (SELECT 3 + 2 * 2)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" NOT IN (SELECT 3 + 2 * 2)"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn not_in_subquery(self, sel: SelectStatement) -> SimpleExpr {
        ExprTrait::not_in_subquery(self, sel)
    }

    /// Express a `EXISTS` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr_as(Expr::exists(Query::select().column(Char::Id).from(Char::Table).take()), "character_exists")
    ///     .expr_as(Expr::exists(Query::select().column(Glyph::Id).from(Glyph::Table).take()), "glyph_exists")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT EXISTS(SELECT `id` FROM `character`) AS `character_exists`, EXISTS(SELECT `id` FROM `glyph`) AS `glyph_exists`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT EXISTS(SELECT "id" FROM "character") AS "character_exists", EXISTS(SELECT "id" FROM "glyph") AS "glyph_exists""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT EXISTS(SELECT "id" FROM "character") AS "character_exists", EXISTS(SELECT "id" FROM "glyph") AS "glyph_exists""#
    /// );
    /// ```
    pub fn exists(sel: SelectStatement) -> SimpleExpr {
        SimpleExpr::SubQuery(
            Some(SubQueryOper::Exists),
            Box::new(sel.into_sub_query_statement()),
        )
    }

    /// Express a `ANY` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Id)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::Id).eq(Expr::any(
    ///         Query::select().column(Char::Id).from(Char::Table).take(),
    ///     )))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE `id` = ANY(SELECT `id` FROM `character`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "id" = ANY(SELECT "id" FROM "character")"#
    /// );
    /// ```
    pub fn any(sel: SelectStatement) -> SimpleExpr {
        SimpleExpr::SubQuery(
            Some(SubQueryOper::Any),
            Box::new(sel.into_sub_query_statement()),
        )
    }

    /// Express a `SOME` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::Id)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::Id).ne(Expr::some(
    ///         Query::select().column(Char::Id).from(Char::Table).take(),
    ///     )))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` FROM `character` WHERE `id` <> SOME(SELECT `id` FROM `character`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "character" WHERE "id" <> SOME(SELECT "id" FROM "character")"#
    /// );
    /// ```
    pub fn some(sel: SelectStatement) -> SimpleExpr {
        SimpleExpr::SubQuery(
            Some(SubQueryOper::Some),
            Box::new(sel.into_sub_query_statement()),
        )
    }

    /// Express a `ALL` sub-query expression.
    pub fn all(sel: SelectStatement) -> SimpleExpr {
        SimpleExpr::SubQuery(
            Some(SubQueryOper::All),
            Box::new(sel.into_sub_query_statement()),
        )
    }

    /// Express a `AS enum` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::as_enum] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col(Char::FontSize).as_enum("text"))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `font_size` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CAST("font_size" AS "text") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "font_size" FROM "character""#
    /// );
    ///
    /// struct TextArray;
    ///
    /// impl Iden for TextArray {
    ///     fn unquoted(&self, s: &mut dyn std::fmt::Write) {
    ///         write!(s, "text[]").unwrap();
    ///     }
    /// }
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col(Char::FontSize).as_enum(TextArray))
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `font_size` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CAST("font_size" AS "text"[]) FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "font_size" FROM "character""#
    /// );
    ///
    /// let query = Query::insert()
    ///     .into_table(Char::Table)
    ///     .columns([Char::FontSize])
    ///     .values_panic([Expr::val("large").as_enum("FontSizeEnum")])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `character` (`font_size`) VALUES ('large')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "character" ("font_size") VALUES (CAST('large' AS "FontSizeEnum"))"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "character" ("font_size") VALUES ('large')"#
    /// );
    /// ```
    pub fn as_enum<T>(self, type_name: T) -> SimpleExpr
    where
        T: IntoIden,
    {
        ExprTrait::as_enum(self, type_name)
    }

    /// Adds new `CASE WHEN` to existing case statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///         Expr::case(
    ///                 Expr::col((Glyph::Table, Glyph::Aspect)).is_in([2, 4]),
    ///                 true
    ///              )
    ///             .finally(false),
    ///          "is_even"
    ///     )
    ///     .from(Glyph::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT (CASE WHEN ("glyph"."aspect" IN (2, 4)) THEN TRUE ELSE FALSE END) AS "is_even" FROM "glyph""#
    /// );
    /// ```
    pub fn case<C, T>(cond: C, then: T) -> CaseStatement
    where
        C: IntoCondition,
        T: Into<SimpleExpr>,
    {
        CaseStatement::new().case(cond, then)
    }

    /// Express a `CAST AS` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::cast_as] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::val("1").cast_as("integer"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT CAST('1' AS integer)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CAST('1' AS integer)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CAST('1' AS integer)"#
    /// );
    /// ```
    pub fn cast_as<T>(self, type_name: T) -> SimpleExpr
    where
        T: IntoIden,
    {
        ExprTrait::cast_as(self, type_name)
    }

    /// Keyword `CURRENT_DATE`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::*;
    ///
    /// let query = Query::select().expr(Expr::current_date()).to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT CURRENT_DATE"#);
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CURRENT_DATE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CURRENT_DATE"#
    /// );
    /// ```
    pub fn current_date() -> Expr {
        Expr::new_with_left(Keyword::CurrentDate)
    }

    /// Keyword `CURRENT_TIMESTAMP`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::*;
    ///
    /// let query = Query::select().expr(Expr::current_time()).to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT CURRENT_TIME"#);
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CURRENT_TIME"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CURRENT_TIME"#
    /// );
    /// ```
    pub fn current_time() -> Expr {
        Expr::new_with_left(Keyword::CurrentTime)
    }

    /// Keyword `CURRENT_TIMESTAMP`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{Expr, MysqlQueryBuilder, PostgresQueryBuilder, Query, SqliteQueryBuilder};
    ///
    /// let query = Query::select().expr(Expr::current_timestamp()).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT CURRENT_TIMESTAMP"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CURRENT_TIMESTAMP"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CURRENT_TIMESTAMP"#
    /// );
    /// ```
    pub fn current_timestamp() -> Expr {
        Expr::new_with_left(Keyword::CurrentTimestamp)
    }

    /// Custom keyword.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::*;
    ///
    /// let query = Query::select()
    ///     .expr(Expr::custom_keyword("test"))
    ///     .to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT test"#);
    /// assert_eq!(query.to_string(PostgresQueryBuilder), r#"SELECT test"#);
    /// assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT test"#);
    /// ```
    pub fn custom_keyword<T>(i: T) -> Expr
    where
        T: IntoIden,
    {
        Expr::new_with_left(Keyword::Custom(i.into_iden()))
    }
}

impl From<Expr> for SimpleExpr {
    /// Convert into SimpleExpr
    fn from(src: Expr) -> Self {
        if let Some(uopr) = src.uopr {
            SimpleExpr::Unary(uopr, Box::new(src.left))
        } else if let Some(bopr) = src.bopr {
            SimpleExpr::Binary(Box::new(src.left), bopr, Box::new(src.right.unwrap()))
        } else {
            src.left
        }
    }
}

impl<T> From<T> for SimpleExpr
where
    T: Into<Value>,
{
    fn from(v: T) -> Self {
        SimpleExpr::Value(v.into())
    }
}

impl From<FunctionCall> for SimpleExpr {
    fn from(func: FunctionCall) -> Self {
        SimpleExpr::FunctionCall(func)
    }
}

impl From<ColumnRef> for SimpleExpr {
    fn from(col: ColumnRef) -> Self {
        SimpleExpr::Column(col)
    }
}

impl From<Keyword> for SimpleExpr {
    fn from(k: Keyword) -> Self {
        SimpleExpr::Keyword(k)
    }
}

impl From<LikeExpr> for SimpleExpr {
    fn from(like: LikeExpr) -> Self {
        match like.escape {
            Some(escape) => SimpleExpr::Binary(
                Box::new(like.pattern.into()),
                BinOper::Escape,
                Box::new(SimpleExpr::Constant(escape.into())),
            ),
            None => like.pattern.into(),
        }
    }
}

impl SimpleExpr {
    /// Negates an expression with `NOT`.
    ///
    /// This is equivalent to a newer [ExprTrait::not] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Char::SizeW)
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::SizeW).eq(1).not())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `size_w` FROM `character` WHERE NOT `size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "size_w" FROM "character" WHERE NOT "size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "size_w" FROM "character" WHERE NOT "size_w" = 1"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> SimpleExpr {
        ExprTrait::not(self)
    }

    /// Express a logical `AND` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .cond_where(any![
    ///         Expr::col(Char::SizeW).eq(1).and(Expr::col(Char::SizeH).eq(2)),
    ///         Expr::col(Char::SizeW).eq(3).and(Expr::col(Char::SizeH).eq(4)),
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (`size_w` = 1 AND `size_h` = 2) OR (`size_w` = 3 AND `size_h` = 4)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w" = 1 AND "size_h" = 2) OR ("size_w" = 3 AND "size_h" = 4)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w" = 1 AND "size_h" = 2) OR ("size_w" = 3 AND "size_h" = 4)"#
    /// );
    /// ```
    pub fn and(self, right: SimpleExpr) -> Self {
        ExprTrait::and(self, right)
    }

    /// Express a logical `OR` operation.
    ///
    /// This is equivalent to a newer [ExprTrait::or] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col(Char::SizeW).eq(1).or(Expr::col(Char::SizeH).eq(2)))
    ///     .and_where(Expr::col(Char::SizeW).eq(3).or(Expr::col(Char::SizeH).eq(4)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (`size_w` = 1 OR `size_h` = 2) AND (`size_w` = 3 OR `size_h` = 4)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w" = 1 OR "size_h" = 2) AND ("size_w" = 3 OR "size_h" = 4)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w" = 1 OR "size_h" = 2) AND ("size_w" = 3 OR "size_h" = 4)"#
    /// );
    /// ```
    pub fn or(self, right: SimpleExpr) -> Self {
        ExprTrait::or(self, right)
    }

    /// Express an equal (`=`) expression.
    ///
    /// This is equivalent to a newer [ExprTrait::eq] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::value("What!").eq("Nothing"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 'What!' = 'Nothing'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'What!' = 'Nothing'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'What!' = 'Nothing'"#
    /// );
    /// ```
    pub fn eq<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::eq(self, v)
    }

    /// Express a not equal (`<>`) expression.
    ///
    /// This is equivalent to a newer [ExprTrait::ne] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::value("Morning").ne("Good"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 'Morning' <> 'Good'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'Morning' <> 'Good'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'Morning' <> 'Good'"#
    /// );
    /// ```
    pub fn ne<V>(self, v: V) -> SimpleExpr
    where
        V: Into<SimpleExpr>,
    {
        ExprTrait::ne(self, v)
    }

    /// Perform addition with another [`SimpleExpr`].
    ///
    /// This is equivalent to a newer [ExprTrait::add] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(
    ///         Expr::col(Char::SizeW)
    ///             .max()
    ///             .add(Expr::col(Char::SizeH).max()),
    ///     )
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MAX(`size_w`) + MAX(`size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MAX("size_w") + MAX("size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MAX("size_w") + MAX("size_h") FROM "character""#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn add<T>(self, right: T) -> Self
    where
        T: Into<SimpleExpr>,
    {
        ExprTrait::add(self, right)
    }

    /// Perform multiplication with another [`SimpleExpr`].
    ///
    /// This is equivalent to a newer [ExprTrait::mul] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(
    ///         Expr::col(Char::SizeW)
    ///             .max()
    ///             .mul(Expr::col(Char::SizeH).max()),
    ///     )
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MAX(`size_w`) * MAX(`size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MAX("size_w") * MAX("size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MAX("size_w") * MAX("size_h") FROM "character""#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn mul<T>(self, right: T) -> Self
    where
        T: Into<SimpleExpr>,
    {
        ExprTrait::mul(self, right)
    }

    /// Perform division with another [`SimpleExpr`].
    ///
    /// This is equivalent to a newer [ExprTrait::div] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(
    ///         Expr::col(Char::SizeW)
    ///             .max()
    ///             .div(Expr::col(Char::SizeH).max()),
    ///     )
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MAX(`size_w`) / MAX(`size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MAX("size_w") / MAX("size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MAX("size_w") / MAX("size_h") FROM "character""#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn div<T>(self, right: T) -> Self
    where
        T: Into<SimpleExpr>,
    {
        ExprTrait::div(self, right)
    }

    /// Perform subtraction with another [`SimpleExpr`].
    ///
    /// This is equivalent to a newer [ExprTrait::sub] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(
    ///         Expr::col(Char::SizeW)
    ///             .max()
    ///             .sub(Expr::col(Char::SizeW).min()),
    ///     )
    ///     .from(Char::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT MAX(`size_w`) - MIN(`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT MAX("size_w") - MIN("size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT MAX("size_w") - MIN("size_w") FROM "character""#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn sub<T>(self, right: T) -> Self
    where
        T: Into<SimpleExpr>,
    {
        ExprTrait::sub(self, right)
    }

    /// Express a `CAST AS` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::cast_as] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .expr(Expr::value("1").cast_as("integer"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT CAST('1' AS integer)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT CAST('1' AS integer)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT CAST('1' AS integer)"#
    /// );
    /// ```
    pub fn cast_as<T>(self, type_name: T) -> Self
    where
        T: IntoIden,
    {
        ExprTrait::cast_as(self, type_name)
    }

    /// Soft deprecated. This is not meant to be in the public API.
    #[doc(hidden)]
    pub fn cast_as_quoted<T>(self, type_name: T, q: Quote) -> Self
    where
        T: IntoIden,
    {
        let func = Func::cast_as_quoted(self, type_name, q);
        Self::FunctionCall(func)
    }

    /// Create any binary operation
    ///
    /// This is equivalent to a newer [ExprTrait::add] and may require more some wrapping beforehand.
    ///
    /// # Examples
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .cond_where(all![
    ///         Expr::value(10).binary(BinOper::SmallerThan, Expr::col(Char::SizeW)),
    ///         Expr::value(20).binary(BinOper::GreaterThan, Expr::col(Char::SizeH))
    ///     ])
    ///     .to_owned();
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 10 < `size_w` AND 20 > `size_h`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 10 < "size_w" AND 20 > "size_h""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 10 < "size_w" AND 20 > "size_h""#
    /// );
    pub fn binary<O, T>(self, op: O, right: T) -> Self
    where
        O: Into<BinOper>,
        T: Into<SimpleExpr>,
    {
        ExprTrait::binary(self, op, right)
    }

    /// Express a `LIKE` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::like] and may require more some wrapping beforehand.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .columns([Char::Character, Char::SizeW, Char::SizeH])
    ///     .from(Char::Table)
    ///     .and_where(Expr::col((Char::Table, Char::FontId)).cast_as("TEXT").like("a%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE CAST(`character`.`font_id` AS TEXT) LIKE 'a%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE CAST("character"."font_id" AS TEXT) LIKE 'a%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE CAST("character"."font_id" AS TEXT) LIKE 'a%'"#
    /// );
    /// ```
    pub fn like<L: IntoLikeExpr>(self, like: L) -> Self {
        ExprTrait::like(self, like)
    }

    /// Express a `NOT LIKE` expression.
    ///
    /// This is equivalent to a newer [ExprTrait::not_like] and may require more some wrapping beforehand.
    pub fn not_like<L: IntoLikeExpr>(self, like: L) -> Self {
        ExprTrait::not_like(self, like)
    }

    pub(crate) fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_, _, _))
    }

    pub(crate) fn get_bin_oper(&self) -> Option<&BinOper> {
        match self {
            Self::Binary(_, oper, _) => Some(oper),
            _ => None,
        }
    }
}
