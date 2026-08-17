use crate::{database::*, QueryTrait, Statement};

/// This structure provides debug capabilities
#[derive(Debug)]
pub struct DebugQuery<'a, Q, T> {
    /// The query to debug
    pub query: &'a Q,
    /// The value of the query
    pub value: T,
}

macro_rules! debug_query_build {
    ($impl_obj:ty, $db_expr:expr) => {
        impl<'a, Q> DebugQuery<'a, Q, $impl_obj>
        where
            Q: QueryTrait,
        {
            /// This macro builds a [Statement] when invoked
            pub fn build(&self) -> Statement {
                let func = $db_expr;
                let db_backend = func(self);
                self.query.build(db_backend)
            }
        }
    };
}

debug_query_build!(DbBackend, |x: &DebugQuery<_, DbBackend>| x.value);
debug_query_build!(&DbBackend, |x: &DebugQuery<_, &DbBackend>| *x.value);
debug_query_build!(DatabaseConnection, |x: &DebugQuery<
    _,
    DatabaseConnection,
>| x.value.get_database_backend());
debug_query_build!(&DatabaseConnection, |x: &DebugQuery<
    _,
    &DatabaseConnection,
>| x.value.get_database_backend());

/// Helper to get a `Statement` from an object that impl `QueryTrait`.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "mock")]
/// # use sea_orm::{error::*, tests_cfg::*, MockDatabase, MockExecResult, DbBackend};
/// #
/// # let conn = MockDatabase::new(DbBackend::Postgres)
/// #     .into_connection();
/// #
/// use sea_orm::{debug_query_stmt, entity::*, query::*, tests_cfg::cake};
///
/// let c = cake::Entity::insert(cake::ActiveModel {
///     id: ActiveValue::set(1),
///     name: ActiveValue::set("Apple Pie".to_owned()),
/// });
///
/// let raw_sql = debug_query_stmt!(&c, &conn).to_string();
/// assert_eq!(
///     raw_sql,
///     r#"INSERT INTO "cake" ("id", "name") VALUES (1, 'Apple Pie')"#
/// );
///
/// let raw_sql = debug_query_stmt!(&c, conn).to_string();
/// assert_eq!(
///     raw_sql,
///     r#"INSERT INTO "cake" ("id", "name") VALUES (1, 'Apple Pie')"#
/// );
///
/// let raw_sql = debug_query_stmt!(&c, DbBackend::MySql).to_string();
/// assert_eq!(
///     raw_sql,
///     r#"INSERT INTO `cake` (`id`, `name`) VALUES (1, 'Apple Pie')"#
/// );
///
/// let raw_sql = debug_query_stmt!(&c, &DbBackend::MySql).to_string();
/// assert_eq!(
///     raw_sql,
///     r#"INSERT INTO `cake` (`id`, `name`) VALUES (1, 'Apple Pie')"#
/// );
/// ```
#[macro_export]
macro_rules! debug_query_stmt {
    ($query:expr,$value:expr) => {
        $crate::DebugQuery {
            query: $query,
            value: $value,
        }
        .build();
    };
}

/// Helper to get a raw SQL string from an object that impl `QueryTrait`.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "mock")]
/// # use sea_orm::{error::*, tests_cfg::*, MockDatabase, MockExecResult, DbBackend};
/// #
/// # let conn = MockDatabase::new(DbBackend::Postgres)
/// #     .into_connection();
/// #
/// use sea_orm::{debug_query, entity::*, query::*, tests_cfg::cake};
///
/// let c = cake::Entity::insert(cake::ActiveModel {
///     id: ActiveValue::set(1),
///     name: ActiveValue::set("Apple Pie".to_owned()),
/// });
///
/// let raw_sql = debug_query!(&c, &conn);
/// assert_eq!(
///     raw_sql,
///     r#"INSERT INTO "cake" ("id", "name") VALUES (1, 'Apple Pie')"#
/// );
///
/// let raw_sql = debug_query!(&c, conn);
/// assert_eq!(
///     raw_sql,
///     r#"INSERT INTO "cake" ("id", "name") VALUES (1, 'Apple Pie')"#
/// );
///
/// let raw_sql = debug_query!(&c, DbBackend::Sqlite);
/// assert_eq!(
///     raw_sql,
///     r#"INSERT INTO "cake" ("id", "name") VALUES (1, 'Apple Pie')"#
/// );
/// ```
#[macro_export]
macro_rules! debug_query {
    ($query:expr,$value:expr) => {
        $crate::debug_query_stmt!($query, $value).to_string();
    };
}
