//! Base types used throughout sea-query.

use crate::{expr::*, query::*, FunctionCall, ValueTuple, Values};
use std::{fmt, mem, ops};

#[cfg(feature = "backend-postgres")]
use crate::extension::postgres::PgBinOper;
#[cfg(feature = "backend-sqlite")]
use crate::extension::sqlite::SqliteBinOper;

/// A reference counted pointer: either [`Rc`][std::rc::Rc] or [`Arc`][std::sync::Arc],
/// depending on the feature flags.
///
/// [`Arc`][std::sync::Arc] is used when `thread-safe` feature is activated.
#[cfg(not(feature = "thread-safe"))]
pub type RcOrArc<T> = std::rc::Rc<T>;
/// A reference counted pointer: either [`Rc`][std::rc::Rc] or [`Arc`][std::sync::Arc],
/// depending on the feature flags.
///
/// [`Arc`][std::sync::Arc] is used when `thread-safe` feature is activated.
#[cfg(feature = "thread-safe")]
pub type RcOrArc<T> = std::sync::Arc<T>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quote(pub(crate) u8, pub(crate) u8);

macro_rules! iden_trait {
    ($($bounds:ident),*) => {
        /// Identifier
        pub trait Iden where $(Self: $bounds),* {
            fn prepare(&self, s: &mut dyn fmt::Write, q: Quote) {
                write!(s, "{}{}{}", q.left(), self.quoted(q), q.right()).unwrap();
            }

            fn quoted(&self, q: Quote) -> String {
                let byte = [q.1];
                let qq: &str = std::str::from_utf8(&byte).unwrap();
                self.to_string().replace(qq, qq.repeat(2).as_str())
            }

            /// A shortcut for writing an [`unquoted`][Iden::unquoted]
            /// identifier into a [`String`].
            ///
            /// We can't reuse [`ToString`] for this, because [`ToString`] uses
            /// the [`Display`][std::fmt::Display] representation. Bnd [`Iden`]
            /// representation is distinct from [`Display`][std::fmt::Display]
            /// and can be different.
            fn to_string(&self) -> String {
                let mut s = String::new();
                self.unquoted(&mut s);
                s
            }

            /// Write a raw identifier string without quotes.
            ///
            /// We indentionally don't reuse [`Display`][std::fmt::Display] for
            /// this, because we want to allow it to have a different logic.
            fn unquoted(&self, s: &mut dyn fmt::Write);
        }

        /// Identifier
        pub trait IdenStatic: Iden + Copy + 'static {
            fn as_str(&self) -> &'static str;
        }
    };
}

#[cfg(feature = "thread-safe")]
iden_trait!(Send, Sync);
#[cfg(not(feature = "thread-safe"))]
iden_trait!();

pub type DynIden = SeaRc<dyn Iden>;

#[derive(Debug)]
#[repr(transparent)]
pub struct SeaRc<I>(pub(crate) RcOrArc<I>)
where
    I: ?Sized;

impl ops::Deref for SeaRc<dyn Iden> {
    type Target = dyn Iden;

    fn deref(&self) -> &Self::Target {
        ops::Deref::deref(&self.0)
    }
}

impl Clone for SeaRc<dyn Iden> {
    fn clone(&self) -> SeaRc<dyn Iden> {
        SeaRc(RcOrArc::clone(&self.0))
    }
}

impl PartialEq for SeaRc<dyn Iden> {
    fn eq(&self, other: &Self) -> bool {
        let (self_vtable, other_vtable) = unsafe {
            let (_, self_vtable) = mem::transmute::<&dyn Iden, (usize, usize)>(&*self.0);
            let (_, other_vtable) = mem::transmute::<&dyn Iden, (usize, usize)>(&*other.0);
            (self_vtable, other_vtable)
        };
        self_vtable == other_vtable && self.to_string() == other.to_string()
    }
}

impl SeaRc<dyn Iden> {
    pub fn new<I>(i: I) -> SeaRc<dyn Iden>
    where
        I: Iden + 'static,
    {
        SeaRc(RcOrArc::new(i))
    }
}

pub trait IntoIden {
    fn into_iden(self) -> DynIden;
}

pub trait IdenList {
    type IntoIter: Iterator<Item = DynIden>;

    fn into_iter(self) -> Self::IntoIter;
}

impl fmt::Debug for dyn Iden {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.unquoted(formatter);
        Ok(())
    }
}

/// Column references
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnRef {
    Column(DynIden),
    TableColumn(DynIden, DynIden),
    SchemaTableColumn(DynIden, DynIden, DynIden),
    Asterisk,
    TableAsterisk(DynIden),
}

pub trait IntoColumnRef {
    fn into_column_ref(self) -> ColumnRef;
}

/// Table references
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum TableRef {
    /// Table identifier without any schema / database prefix
    Table(DynIden),
    /// Table identifier with schema prefix
    SchemaTable(DynIden, DynIden),
    /// Table identifier with database and schema prefix
    DatabaseSchemaTable(DynIden, DynIden, DynIden),
    /// Table identifier with alias
    TableAlias(DynIden, DynIden),
    /// Table identifier with schema prefix and alias
    SchemaTableAlias(DynIden, DynIden, DynIden),
    /// Table identifier with database and schema prefix and alias
    DatabaseSchemaTableAlias(DynIden, DynIden, DynIden, DynIden),
    /// Subquery with alias
    SubQuery(SelectStatement, DynIden),
    /// Values list with alias
    ValuesList(Vec<ValueTuple>, DynIden),
    /// Function call with alias
    FunctionCall(FunctionCall, DynIden),
}

pub trait IntoTableRef {
    fn into_table_ref(self) -> TableRef;
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOper {
    Not,
}

/// Binary operators.
///
/// If something is not supported here, you can use [`BinOper::Custom`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOper {
    And,
    Or,
    Like,
    NotLike,
    Is,
    IsNot,
    In,
    NotIn,
    Between,
    NotBetween,
    Equal,
    NotEqual,
    SmallerThan,
    GreaterThan,
    SmallerThanOrEqual,
    GreaterThanOrEqual,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    LShift,
    RShift,
    As,
    Escape,
    Custom(&'static str),
    #[cfg(feature = "backend-postgres")]
    PgOperator(PgBinOper),
    #[cfg(feature = "backend-sqlite")]
    SqliteOperator(SqliteBinOper),
}

/// Logical chain operator: conjunction or disjunction.
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalChainOper {
    And(SimpleExpr),
    Or(SimpleExpr),
}

/// Join types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Join,
    CrossJoin,
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullOuterJoin,
}

/// Nulls order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullOrdering {
    First,
    Last,
}

/// Order expression
#[derive(Debug, Clone, PartialEq)]
pub struct OrderExpr {
    pub(crate) expr: SimpleExpr,
    pub(crate) order: Order,
    pub(crate) nulls: Option<NullOrdering>,
}

/// Join on types
#[derive(Debug, Clone, PartialEq)]
pub enum JoinOn {
    Condition(Box<ConditionHolder>),
    Columns(Vec<SimpleExpr>),
}

/// Ordering options
#[derive(Debug, Clone, PartialEq)]
pub enum Order {
    Asc,
    Desc,
    Field(Values),
}

/// An explicit wrapper for [`Iden`]s which are dynamic user-provided strings.
///
/// Nowadays, `&str` implements [`Iden`] and can be used directly.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Alias(String);

/// Null Alias
#[derive(Default, Debug, Copy, Clone)]
pub struct NullAlias;

/// Asterisk ("*")
///
/// Express the asterisk without table prefix.
///
/// # Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let query = Query::select()
///     .column(Asterisk)
///     .from(Char::Table)
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"SELECT * FROM `character`"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder),
///     r#"SELECT * FROM "character""#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"SELECT * FROM "character""#
/// );
/// ```
///
/// Express the asterisk with table prefix.
///
/// Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let query = Query::select()
///     .column((Char::Table, Asterisk))
///     .from(Char::Table)
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"SELECT `character`.* FROM `character`"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder),
///     r#"SELECT "character".* FROM "character""#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"SELECT "character".* FROM "character""#
/// );
/// ```
#[derive(Default, Debug, Clone, Copy)]
pub struct Asterisk;

/// Known SQL keywords that can be used as expressions.
///
/// If something is not supported here, you can use [`Keyword::Custom`].
#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Null,
    CurrentDate,
    CurrentTime,
    CurrentTimestamp,
    Custom(DynIden),
}

/// Like Expression
#[derive(Debug, Clone)]
pub struct LikeExpr {
    pub(crate) pattern: String,
    pub(crate) escape: Option<char>,
}

pub trait IntoLikeExpr {
    fn into_like_expr(self) -> LikeExpr;
}

/// SubQuery operators
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SubQueryOper {
    Exists,
    Any,
    Some,
    All,
}

// Impl begins

impl Quote {
    pub fn new(c: u8) -> Self {
        Self(c, c)
    }

    pub fn left(&self) -> char {
        char::from(self.0)
    }

    pub fn right(&self) -> char {
        char::from(self.1)
    }
}

impl From<char> for Quote {
    fn from(c: char) -> Self {
        (c as u8).into()
    }
}

impl From<(char, char)> for Quote {
    fn from((l, r): (char, char)) -> Self {
        (l as u8, r as u8).into()
    }
}

impl From<u8> for Quote {
    fn from(u8: u8) -> Self {
        Quote::new(u8)
    }
}

impl From<(u8, u8)> for Quote {
    fn from((l, r): (u8, u8)) -> Self {
        Quote(l, r)
    }
}

impl<T: 'static> IntoIden for T
where
    T: Iden,
{
    fn into_iden(self) -> DynIden {
        SeaRc::new(self)
    }
}

impl IntoIden for DynIden {
    fn into_iden(self) -> DynIden {
        self
    }
}

impl<I> IdenList for I
where
    I: IntoIden,
{
    type IntoIter = std::iter::Once<DynIden>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.into_iden())
    }
}

impl<A, B> IdenList for (A, B)
where
    A: IntoIden,
    B: IntoIden,
{
    type IntoIter = std::array::IntoIter<DynIden, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0.into_iden(), self.1.into_iden()].into_iter()
    }
}

impl<A, B, C> IdenList for (A, B, C)
where
    A: IntoIden,
    B: IntoIden,
    C: IntoIden,
{
    type IntoIter = std::array::IntoIter<DynIden, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0.into_iden(), self.1.into_iden(), self.2.into_iden()].into_iter()
    }
}

impl IntoColumnRef for ColumnRef {
    fn into_column_ref(self) -> ColumnRef {
        self
    }
}

impl<T: 'static> IntoColumnRef for T
where
    T: IntoIden,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::Column(self.into_iden())
    }
}

impl IntoColumnRef for Asterisk {
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::Asterisk
    }
}

impl<S: 'static, T: 'static> IntoColumnRef for (S, T)
where
    S: IntoIden,
    T: IntoIden,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::TableColumn(self.0.into_iden(), self.1.into_iden())
    }
}

impl<T: 'static> IntoColumnRef for (T, Asterisk)
where
    T: IntoIden,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::TableAsterisk(self.0.into_iden())
    }
}

impl<S: 'static, T: 'static, U: 'static> IntoColumnRef for (S, T, U)
where
    S: IntoIden,
    T: IntoIden,
    U: IntoIden,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::SchemaTableColumn(self.0.into_iden(), self.1.into_iden(), self.2.into_iden())
    }
}

impl IntoTableRef for TableRef {
    fn into_table_ref(self) -> TableRef {
        self
    }
}

impl<T: 'static> IntoTableRef for T
where
    T: IntoIden,
{
    fn into_table_ref(self) -> TableRef {
        TableRef::Table(self.into_iden())
    }
}

impl<S: 'static, T: 'static> IntoTableRef for (S, T)
where
    S: IntoIden,
    T: IntoIden,
{
    fn into_table_ref(self) -> TableRef {
        TableRef::SchemaTable(self.0.into_iden(), self.1.into_iden())
    }
}

impl<S: 'static, T: 'static, U: 'static> IntoTableRef for (S, T, U)
where
    S: IntoIden,
    T: IntoIden,
    U: IntoIden,
{
    fn into_table_ref(self) -> TableRef {
        TableRef::DatabaseSchemaTable(self.0.into_iden(), self.1.into_iden(), self.2.into_iden())
    }
}

impl TableRef {
    /// Add or replace the current alias
    pub fn alias<A>(self, alias: A) -> Self
    where
        A: IntoIden,
    {
        match self {
            Self::Table(table) => Self::TableAlias(table, alias.into_iden()),
            Self::TableAlias(table, _) => Self::TableAlias(table, alias.into_iden()),
            Self::SchemaTable(schema, table) => {
                Self::SchemaTableAlias(schema, table, alias.into_iden())
            }
            Self::DatabaseSchemaTable(database, schema, table) => {
                Self::DatabaseSchemaTableAlias(database, schema, table, alias.into_iden())
            }
            Self::SchemaTableAlias(schema, table, _) => {
                Self::SchemaTableAlias(schema, table, alias.into_iden())
            }
            Self::DatabaseSchemaTableAlias(database, schema, table, _) => {
                Self::DatabaseSchemaTableAlias(database, schema, table, alias.into_iden())
            }
            Self::SubQuery(statement, _) => Self::SubQuery(statement, alias.into_iden()),
            Self::ValuesList(values, _) => Self::ValuesList(values, alias.into_iden()),
            Self::FunctionCall(func, _) => Self::FunctionCall(func, alias.into_iden()),
        }
    }
}

impl Alias {
    pub fn new<T>(n: T) -> Self
    where
        T: Into<String>,
    {
        Self(n.into())
    }
}

// Regaring potential `impl for String` and the need for `Alias`,
// see discussions on https://github.com/SeaQL/sea-query/pull/882

/// Reuses the `impl` for the underlying [str].
impl Iden for Alias {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        self.0.as_str().unquoted(s);
    }
}

/// The "base" `impl` for writing arbitrary "raw" strings as identifiers.
///
/// Reused for other string-like types.
impl Iden for &str {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        s.write_str(self).unwrap();
    }
}

impl NullAlias {
    pub fn new() -> Self {
        Self
    }
}

impl Iden for NullAlias {
    fn unquoted(&self, _s: &mut dyn fmt::Write) {}
}

impl LikeExpr {
    pub fn new<T>(pattern: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            pattern: pattern.into(),
            escape: None,
        }
    }

    #[deprecated(since = "0.29.0", note = "Please use the [`LikeExpr::new`] method")]
    pub fn str<T>(pattern: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            pattern: pattern.into(),
            escape: None,
        }
    }

    pub fn escape(self, c: char) -> Self {
        Self {
            pattern: self.pattern,
            escape: Some(c),
        }
    }
}

impl IntoLikeExpr for LikeExpr {
    fn into_like_expr(self) -> LikeExpr {
        self
    }
}

impl<T> IntoLikeExpr for T
where
    T: Into<String>,
{
    fn into_like_expr(self) -> LikeExpr {
        LikeExpr::new(self)
    }
}

#[cfg(test)]
mod tests {
    pub use crate::{tests_cfg::*, *};
    use pretty_assertions::assert_eq;
    pub use Character as CharReexport;

    #[test]
    fn test_identifier() {
        let query = Query::select().column("hello-World_").to_owned();

        #[cfg(feature = "backend-mysql")]
        assert_eq!(query.to_string(MysqlQueryBuilder), r"SELECT `hello-World_`");
        #[cfg(feature = "backend-postgres")]
        assert_eq!(
            query.to_string(PostgresQueryBuilder),
            r#"SELECT "hello-World_""#
        );
        #[cfg(feature = "backend-sqlite")]
        assert_eq!(
            query.to_string(SqliteQueryBuilder),
            r#"SELECT "hello-World_""#
        );
    }

    #[test]
    fn test_quoted_identifier_1() {
        let query = Query::select().column("hel`lo").to_owned();

        #[cfg(feature = "backend-mysql")]
        assert_eq!(query.to_string(MysqlQueryBuilder), r"SELECT `hel``lo`");
        #[cfg(feature = "backend-sqlite")]
        assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT "hel`lo""#);

        let query = Query::select().column("hel\"lo").to_owned();

        #[cfg(feature = "backend-postgres")]
        assert_eq!(query.to_string(PostgresQueryBuilder), r#"SELECT "hel""lo""#);
    }

    #[test]
    fn test_quoted_identifier_2() {
        let query = Query::select().column("hel``lo").to_owned();

        #[cfg(feature = "backend-mysql")]
        assert_eq!(query.to_string(MysqlQueryBuilder), r"SELECT `hel````lo`");
        #[cfg(feature = "backend-sqlite")]
        assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT "hel``lo""#);

        let query = Query::select().column("hel\"\"lo").to_owned();

        #[cfg(feature = "backend-postgres")]
        assert_eq!(
            query.to_string(PostgresQueryBuilder),
            r#"SELECT "hel""""lo""#
        );
    }

    #[test]
    fn test_cmp_identifier() {
        type CharLocal = Character;

        assert_eq!(
            ColumnRef::Column(Character::Id.into_iden()),
            ColumnRef::Column(Character::Id.into_iden())
        );
        assert_eq!(
            ColumnRef::Column(Character::Id.into_iden()),
            ColumnRef::Column(Char::Id.into_iden())
        );
        assert_eq!(
            ColumnRef::Column(Character::Id.into_iden()),
            ColumnRef::Column(CharLocal::Id.into_iden())
        );
        assert_eq!(
            ColumnRef::Column(Character::Id.into_iden()),
            ColumnRef::Column(CharReexport::Id.into_iden())
        );
        assert_eq!(
            ColumnRef::Column("id".into_iden()),
            ColumnRef::Column("id".into_iden())
        );
        assert_ne!(
            ColumnRef::Column("id".into_iden()),
            ColumnRef::Column("id_".into_iden())
        );
        assert_ne!(
            ColumnRef::Column(Character::Id.into_iden()),
            ColumnRef::Column("id".into_iden())
        );
        assert_ne!(
            ColumnRef::Column(Character::Id.into_iden()),
            ColumnRef::Column(Character::Table.into_iden())
        );
        assert_ne!(
            ColumnRef::Column(Character::Id.into_iden()),
            ColumnRef::Column(Font::Id.into_iden())
        );
    }
}
