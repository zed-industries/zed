use std::ops::{Deref, DerefMut};

/// Map a SQL text value to/from a Rust type using [`Display`] and [`FromStr`].
///
/// This can be useful for types that do not have a direct SQL equivalent, or are simply not
/// supported by SQLx for one reason or another.
///
/// For strongly typed databases like Postgres, this will report the value's type as `TEXT`.
/// Explicit conversion may be necessary on the SQL side depending on the desired type.
///
/// [`Display`]: std::fmt::Display
/// [`FromStr`]: std::str::FromStr
///
/// ### Panics
///
/// You should only use this adapter with `Display` implementations that are infallible,
/// otherwise you may encounter panics when attempting to bind a value.
///
/// This is because the design of the `Encode` trait assumes encoding is infallible, so there is no
/// way to bubble up the error.
///
/// Fortunately, most `Display` implementations are infallible by convention anyway
/// (the standard `ToString` trait also assumes this), but you may still want to audit
/// the source code for any types you intend to use with this adapter, just to be safe.
///
/// ### Example: `SocketAddr`
///
/// MySQL and SQLite do not have a native SQL equivalent for `SocketAddr`, so if you want to
/// store and retrieve instances of it, it makes sense to map it to `TEXT`:
///
/// ```rust,no_run
/// # use sqlx::types::{time, uuid};
///
/// use std::net::SocketAddr;
///
/// use sqlx::Connection;
/// use sqlx::mysql::MySqlConnection;
/// use sqlx::types::Text;
///
/// use uuid::Uuid;
/// use time::OffsetDateTime;
///
/// #[derive(sqlx::FromRow, Debug)]
/// struct Login {
///     user_id: Uuid,
///     socket_addr: Text<SocketAddr>,
///     login_at: OffsetDateTime
/// }
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
///
/// let mut conn: MySqlConnection = MySqlConnection::connect("<DATABASE URL>").await?;
///
/// let user_id: Uuid = "e9a72cdc-d907-48d6-a488-c64a91fd063c".parse().unwrap();
/// let socket_addr: SocketAddr = "198.51.100.47:31790".parse().unwrap();
///
/// // CREATE TABLE user_login(user_id VARCHAR(36), socket_addr TEXT, login_at TIMESTAMP);
/// sqlx::query("INSERT INTO user_login(user_id, socket_addr, login_at) VALUES (?, ?, NOW())")
///     .bind(user_id)
///     .bind(Text(socket_addr))
///     .execute(&mut conn)
///     .await?;
///
/// let logins: Vec<Login> = sqlx::query_as("SELECT * FROM user_login")
///     .fetch_all(&mut conn)
///     .await?;
///
/// println!("Logins for user ID {user_id}: {logins:?}");
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Text<T>(pub T);

impl<T> Text<T> {
    /// Extract the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Text<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Text<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/* We shouldn't use blanket impls so individual drivers can provide specialized ones.
impl<T, DB> Type<DB> for Text<T>
where
    String: Type<DB>,
    DB: Database,
{
    fn type_info() -> DB::TypeInfo {
        String::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        String::compatible(ty)
    }
}

impl<'q, T, DB> Encode<'q, DB> for Text<T>
where
    T: Display,
    String: Encode<'q, DB>,
    DB: Database,
{
    fn encode_by_ref(&self, buf: &mut <DB as Database>::ArgumentBuffer<'q>) -> Result<IsNull, BoxDynError> {
        self.0.to_string().encode(buf)
    }
}

impl<'r, T, DB> Decode<'r, DB> for Text<T>
where
    T: FromStr,
    BoxDynError: From<<T as FromStr>::Err>,
    &'r str: Decode<'r, DB>,
    DB: Database,
{
    fn decode(value: <DB as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(Text(<&'r str as Decode<'r, DB>>::decode(value)?.parse()?))
    }
}
*/
