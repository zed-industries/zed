use crate::{error::Error, row::Row};

/// A record that can be built from a row returned by the database.
///
/// In order to use [`query_as`](crate::query_as) the output type must implement `FromRow`.
///
/// ## Derivable
///
/// This trait can be derived by SQLx for any struct. The generated implementation
/// will consist of a sequence of calls to [`Row::try_get`] using the name from each
/// struct field.
///
/// ```rust,ignore
/// #[derive(sqlx::FromRow)]
/// struct User {
///     id: i32,
///     name: String,
/// }
/// ```
///
/// ### Field attributes
///
/// Several attributes can be specified to customize how each column in a row is read:
///
/// #### `rename`
///
/// When the name of a field in Rust does not match the name of its corresponding column,
/// you can use the `rename` attribute to specify the name that the field has in the row.
/// For example:
///
/// ```rust,ignore
/// #[derive(sqlx::FromRow)]
/// struct User {
///     id: i32,
///     name: String,
///     #[sqlx(rename = "description")]
///     about_me: String
/// }
/// ```
///
/// Given a query such as:
///
/// ```sql
/// SELECT id, name, description FROM users;
/// ```
///
/// will read the content of the column `description` into the field `about_me`.
///
/// #### `rename_all`
/// By default, field names are expected verbatim (with the exception of the raw identifier prefix `r#`, if present).
/// Placed at the struct level, this attribute changes how the field name is mapped to its SQL column name:
///
/// ```rust,ignore
/// #[derive(sqlx::FromRow)]
/// #[sqlx(rename_all = "camelCase")]
/// struct UserPost {
///     id: i32,
///     // remapped to "userId"
///     user_id: i32,
///     contents: String
/// }
/// ```
///
/// The supported values are `snake_case` (available if you have non-snake-case field names for some
/// reason), `lowercase`, `UPPERCASE`, `camelCase`, `PascalCase`, `SCREAMING_SNAKE_CASE` and `kebab-case`.
/// The styling of each option is intended to be an example of its behavior.
///
/// Case conversion is handled by the `heck` crate.
/// See [its documentation](https://docs.rs/heck/0.5.0/heck/#definition-of-a-word-boundary)
/// for details.
///
/// Note that numbers are *not* considered separate words.
/// For example, `Foo1` to snake case would be `foo1`, *not* `foo_1`.
/// See [this issue](https://github.com/launchbadge/sqlx/issues/3864) for discussion.
///
/// #### `default`
///
/// When your struct contains a field that is not present in your query,
/// if the field type has an implementation for [`Default`],
/// you can use the `default` attribute to assign the default value to said field.
/// For example:
///
/// ```rust,ignore
/// #[derive(sqlx::FromRow)]
/// struct User {
///     id: i32,
///     name: String,
///     #[sqlx(default)]
///     location: Option<String>
/// }
/// ```
///
/// Given a query such as:
///
/// ```sql
/// SELECT id, name FROM users;
/// ```
///
/// will set the value of the field `location` to the default value of `Option<String>`,
/// which is `None`.
///
/// Moreover, if the struct has an implementation for [`Default`], you can use the `default`
/// attribute at the struct level rather than for each single field. If a field does not appear in the result,
/// its value is taken from the `Default` implementation for the struct.
/// For example:
///
/// ```rust, ignore
/// #[derive(Default, sqlx::FromRow)]
/// #[sqlx(default)]
/// struct Options {
///     option_a: Option<i32>,
///     option_b: Option<String>,
///     option_c: Option<bool>,
/// }
/// ```
///
/// For a derived `Default` implementation this effectively populates each missing field
/// with `Default::default()`, but a manual `Default` implementation can provide
/// different placeholder values, if applicable.
///
/// This is similar to how `#[serde(default)]` behaves.
///
/// #### `flatten`
///
/// If you want to handle a field that implements [`FromRow`],
/// you can use the `flatten` attribute to specify that you want
/// it to use [`FromRow`] for parsing rather than the usual method.
/// For example:
///
/// ```rust,ignore
/// #[derive(sqlx::FromRow)]
/// struct Address {
///     country: String,
///     city: String,
///     road: String,
/// }
///
/// #[derive(sqlx::FromRow)]
/// struct User {
///     id: i32,
///     name: String,
///     #[sqlx(flatten)]
///     address: Address,
/// }
/// ```
/// Given a query such as:
///
/// ```sql
/// SELECT id, name, country, city, road FROM users;
/// ```
///
/// This field is compatible with the `default` attribute.
///
/// #### `skip`
///
/// This is a variant of the `default` attribute which instead always takes the value from
/// the `Default` implementation for this field type ignoring any results in your query.
/// This can be useful, if some field does not satifisfy the trait bounds (i.e.
/// `sqlx::decode::Decode`, `sqlx::type::Type`), in particular in case of nested structures.
/// For example:
///
/// ```rust,ignore
/// #[derive(sqlx::FromRow)]
/// struct Address {
///     user_name: String,
///     street: String,
///     city: String,
/// }
///
/// #[derive(sqlx::FromRow)]
/// struct User {
///     name: String,
///     #[sqlx(skip)]
///     addresses: Vec<Address>,
/// }
/// ```
///
/// Then when querying into `User`, only `name` needs to be set:
///
/// ```rust,ignore
/// let user: User = sqlx::query_as("SELECT name FROM users")
///    .fetch_one(&mut some_connection)
///    .await?;
///
/// // `Default` for `Vec<Address>` is an empty vector.
/// assert!(user.addresses.is_empty());
/// ```
///
/// #### `try_from`
///
/// When your struct contains a field whose type is not matched with the database type,
/// if the field type has an implementation [`TryFrom`] for the database type,
/// you can use the `try_from` attribute to convert the database type to the field type.
/// For example:
///
/// ```rust,ignore
/// #[derive(sqlx::FromRow)]
/// struct User {
///     id: i32,
///     name: String,
///     #[sqlx(try_from = "i64")]
///     bigIntInMySql: u64
/// }
/// ```
///
/// Given a query such as:
///
/// ```sql
/// SELECT id, name, bigIntInMySql FROM users;
/// ```
///
/// In MySql, `BigInt` type matches `i64`, but you can convert it to `u64` by `try_from`.
///
/// #### `json`
///
/// If your database supports a JSON type, you can leverage `#[sqlx(json)]`
/// to automatically integrate JSON deserialization in your [`FromRow`] implementation using [`serde`](https://docs.rs/serde/latest/serde/).
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct Data {
///     field1: String,
///     field2: u64
/// }
///
/// #[derive(sqlx::FromRow)]
/// struct User {
///     id: i32,
///     name: String,
///     #[sqlx(json)]
///     metadata: Data
/// }
/// ```
///
/// Given a query like the following:
///
/// ```sql
/// SELECT
///     1 AS id,
///     'Name' AS name,
///     JSON_OBJECT('field1', 'value1', 'field2', 42) AS metadata
/// ```
///
/// The `metadata` field will be deserialized used its `serde::Deserialize` implementation:
///
/// ```rust,ignore
/// User {
///     id: 1,
///     name: "Name",
///     metadata: Data {
///         field1: "value1",
///         field2: 42
///     }
/// }
/// ```
///
/// By default the `#[sqlx(json)]` attribute will assume that the underlying database row is
/// _not_ NULL. This can cause issues when your field type is an `Option<T>` because this would be
/// represented as the _not_ NULL (in terms of DB) JSON value of `null`.
///
/// If you wish to describe a database row which _is_ NULLable but _cannot_ contain the JSON value `null`,
/// use the `#[sqlx(json(nullable))]` attribute.
///
/// For example
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// struct Data {
///     field1: String,
///     field2: u64
/// }
///
/// #[derive(sqlx::FromRow)]
/// struct User {
///     id: i32,
///     name: String,
///     #[sqlx(json(nullable))]
///     metadata: Option<Data>
/// }
/// ```
/// Would describe a database field which _is_ NULLable but if it exists it must be the JSON representation of `Data`
/// and cannot be the JSON value `null`
///
/// ## Manual implementation
///
/// You can also implement the [`FromRow`] trait by hand. This can be useful if you
/// have a struct with a field that needs manual decoding:
///
///
/// ```rust,ignore
/// use sqlx::{FromRow, sqlite::SqliteRow, sqlx::Row};
/// struct MyCustomType {
///     custom: String,
/// }
///
/// struct Foo {
///     bar: MyCustomType,
/// }
///
/// impl FromRow<'_, SqliteRow> for Foo {
///     fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
///         Ok(Self {
///             bar: MyCustomType {
///                 custom: row.try_get("custom")?
///             }
///         })
///     }
/// }
/// ```
pub trait FromRow<'r, R: Row>: Sized {
    fn from_row(row: &'r R) -> Result<Self, Error>;
}

impl<'r, R> FromRow<'r, R> for ()
where
    R: Row,
{
    #[inline]
    fn from_row(_: &'r R) -> Result<Self, Error> {
        Ok(())
    }
}

// implement FromRow for tuples of types that implement Decode
// up to tuples of 16 values

macro_rules! impl_from_row_for_tuple {
    ($( ($idx:tt) -> $T:ident );+;) => {
        impl<'r, R, $($T,)+> FromRow<'r, R> for ($($T,)+)
        where
            R: Row,
            usize: crate::column::ColumnIndex<R>,
            $($T: crate::decode::Decode<'r, R::Database> + crate::types::Type<R::Database>,)+
        {
            #[inline]
            fn from_row(row: &'r R) -> Result<Self, Error> {
                Ok(($(row.try_get($idx as usize)?,)+))
            }
        }
    };
}

impl_from_row_for_tuple!(
    (0) -> T1;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
    (10) -> T11;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
    (10) -> T11;
    (11) -> T12;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
    (10) -> T11;
    (11) -> T12;
    (12) -> T13;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
    (10) -> T11;
    (11) -> T12;
    (12) -> T13;
    (13) -> T14;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
    (10) -> T11;
    (11) -> T12;
    (12) -> T13;
    (13) -> T14;
    (14) -> T15;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
    (10) -> T11;
    (11) -> T12;
    (12) -> T13;
    (13) -> T14;
    (14) -> T15;
    (15) -> T16;
);
