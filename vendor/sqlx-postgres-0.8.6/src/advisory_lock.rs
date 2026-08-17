use crate::error::Result;
use crate::Either;
use crate::PgConnection;
use hkdf::Hkdf;
use once_cell::sync::OnceCell;
use sha2::Sha256;
use std::ops::{Deref, DerefMut};

/// A mutex-like type utilizing [Postgres advisory locks].
///
/// Advisory locks are a mechanism provided by Postgres to have mutually exclusive or shared
/// locks tracked in the database with application-defined semantics, as opposed to the standard
/// row-level or table-level locks which may not fit all use-cases.
///
/// This API provides a convenient wrapper for generating and storing the integer keys that
/// advisory locks use, as well as RAII guards for releasing advisory locks when they fall out
/// of scope.
///
/// This API only handles session-scoped advisory locks (explicitly locked and unlocked, or
/// automatically released when a connection is closed).
///
/// It is also possible to use transaction-scoped locks but those can be used by beginning a
/// transaction and calling the appropriate lock functions (e.g. `SELECT pg_advisory_xact_lock()`)
/// manually, and cannot be explicitly released, but are automatically released when a transaction
/// ends (is committed or rolled back).
///
/// Session-level locks can be acquired either inside or outside a transaction and are not
/// tied to transaction semantics; a lock acquired inside a transaction is still held when that
/// transaction is committed or rolled back, until explicitly released or the connection is closed.
///
/// Locks can be acquired in either shared or exclusive modes, which can be thought of as read locks
/// and write locks, respectively. Multiple shared locks are allowed for the same key, but a single
/// exclusive lock prevents any other lock being taken for a given key until it is released.
///
/// [Postgres advisory locks]: https://www.postgresql.org/docs/current/explicit-locking.html#ADVISORY-LOCKS
#[derive(Debug, Clone)]
pub struct PgAdvisoryLock {
    key: PgAdvisoryLockKey,
    /// The query to execute to release this lock.
    release_query: OnceCell<String>,
}

/// A key type natively used by Postgres advisory locks.
///
/// Currently, Postgres advisory locks have two different key spaces: one keyed by a single
/// 64-bit integer, and one keyed by a pair of two 32-bit integers. The Postgres docs
/// specify that these key spaces "do not overlap":
///
/// <https://www.postgresql.org/docs/current/functions-admin.html#FUNCTIONS-ADVISORY-LOCKS>
///
/// The documentation for the `pg_locks` system view explains further how advisory locks
/// are treated in Postgres:
///
/// <https://www.postgresql.org/docs/current/view-pg-locks.html>
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PgAdvisoryLockKey {
    /// The keyspace designated by a single 64-bit integer.
    ///
    /// When [PgAdvisoryLock] is constructed with [::new()][PgAdvisoryLock::new()],
    /// this is the keyspace used.
    BigInt(i64),
    /// The keyspace designated by two 32-bit integers.
    IntPair(i32, i32),
}

/// A wrapper for `PgConnection` (or a similar type) that represents a held Postgres advisory lock.
///
/// Can be acquired by [`PgAdvisoryLock::acquire()`] or [`PgAdvisoryLock::try_acquire()`].
/// Released on-drop or via [`Self::release_now()`].
///
/// ### Note: Release-on-drop is not immediate!
/// On drop, this guard queues a `pg_advisory_unlock()` call on the connection which will be
/// flushed to the server the next time it is used, or when it is returned to
/// a [`PgPool`][crate::PgPool] in the case of
/// [`PoolConnection<Postgres>`][crate::pool::PoolConnection].
///
/// This means the lock is not actually released as soon as the guard is dropped. To ensure the
/// lock is eagerly released, you can call [`.release_now().await`][Self::release_now()].
pub struct PgAdvisoryLockGuard<'lock, C: AsMut<PgConnection>> {
    lock: &'lock PgAdvisoryLock,
    conn: Option<C>,
}

impl PgAdvisoryLock {
    /// Construct a `PgAdvisoryLock` using the given string as a key.
    ///
    /// This is intended to make it easier to use an advisory lock by using a human-readable string
    /// for a key as opposed to manually generating a unique integer key. The generated integer key
    /// is guaranteed to be stable and in the single 64-bit integer keyspace
    /// (see [`PgAdvisoryLockKey`] for details).
    ///
    /// This is done by applying the [Hash-based Key Derivation Function (HKDF; IETF RFC 5869)][hkdf]
    /// to the bytes of the input string, but in a way that the calculated integer is unlikely
    /// to collide with any similar implementations (although we don't currently know of any).
    /// See the source of this method for details.
    ///
    /// [hkdf]: https://datatracker.ietf.org/doc/html/rfc5869
    /// ### Example
    /// ```rust
    /// use sqlx::postgres::{PgAdvisoryLock, PgAdvisoryLockKey};
    ///
    /// let lock = PgAdvisoryLock::new("my first Postgres advisory lock!");
    /// // Negative values are fine because of how Postgres treats advisory lock keys.
    /// // See the documentation for the `pg_locks` system view for details.
    /// assert_eq!(lock.key(), &PgAdvisoryLockKey::BigInt(-5560419505042474287));
    /// ```
    pub fn new(key_string: impl AsRef<str>) -> Self {
        let input_key_material = key_string.as_ref();

        // HKDF was chosen because it is designed to concentrate the entropy in a variable-length
        // input key and produce a higher quality but reduced-length output key with a
        // well-specified and reproducible algorithm.
        //
        // Granted, the input key is usually meant to be pseudorandom and not human readable,
        // but we're not trying to produce an unguessable value by any means; just one that's as
        // unlikely to already be in use as possible, but still deterministic.
        //
        // SHA-256 was chosen as the hash function because it's already used in the Postgres driver,
        // which should save on codegen and optimization.

        // We don't supply a salt as that is intended to be random, but we want a deterministic key.
        let hkdf = Hkdf::<Sha256>::new(None, input_key_material.as_bytes());

        let mut output_key_material = [0u8; 8];

        // The first string is the "info" string of the HKDF which is intended to tie the output
        // exclusively to SQLx. This should avoid collisions with implementations using a similar
        // strategy. If you _want_ this to match some other implementation then you should get
        // the calculated integer key from it and use that directly.
        //
        // Do *not* change this string as it will affect the output!
        hkdf.expand(
            b"SQLx (Rust) Postgres advisory lock",
            &mut output_key_material,
        )
        // `Hkdf::expand()` only returns an error if you ask for more than 255 times the digest size.
        // This is specified by RFC 5869 but not elaborated upon:
        // https://datatracker.ietf.org/doc/html/rfc5869#section-2.3
        // Since we're only asking for 8 bytes, this error shouldn't be returned.
        .expect("BUG: `output_key_material` should be of acceptable length");

        // For ease of use, this method assumes the user doesn't care which keyspace is used.
        //
        // It doesn't seem likely that someone would care about using the `(int, int)` keyspace
        // specifically unless they already had keys to use, in which case they wouldn't
        // care about this method. That's why we also provide `with_key()`.
        //
        // The choice of `from_le_bytes()` is mostly due to x86 being the most popular
        // architecture for server software, so it should be a no-op there.
        let key = PgAdvisoryLockKey::BigInt(i64::from_le_bytes(output_key_material));

        tracing::trace!(
            ?key,
            key_string = ?input_key_material,
            "generated key from key string",
        );

        Self::with_key(key)
    }

    /// Construct a `PgAdvisoryLock` with a manually supplied key.
    pub fn with_key(key: PgAdvisoryLockKey) -> Self {
        Self {
            key,
            release_query: OnceCell::new(),
        }
    }

    /// Returns the current key.
    pub fn key(&self) -> &PgAdvisoryLockKey {
        &self.key
    }

    // Why doesn't this use `Acquire`? Well, I tried it and got really useless errors
    // about "cannot project lifetimes to parent scope".
    //
    // It has something to do with how lifetimes work on the `Acquire` trait, I couldn't
    // be bothered to figure it out. Probably another issue with a lack of `async fn` in traits
    // or lazy normalization.

    /// Acquires an exclusive lock using `pg_advisory_lock()`, waiting until the lock is acquired.
    ///
    /// For a version that returns immediately instead of waiting, see [`Self::try_acquire()`].
    ///
    /// A connection-like type is required to execute the call. Allowed types include `PgConnection`,
    /// `PoolConnection<Postgres>` and `Transaction<Postgres>`, as well as mutable references to
    /// any of these.
    ///
    /// The returned guard queues a `pg_advisory_unlock()` call on the connection when dropped,
    /// which will be executed the next time the connection is used, or when returned to a
    /// [`PgPool`][crate::PgPool] in the case of `PoolConnection<Postgres>`.
    ///
    /// Postgres allows a single connection to acquire a given lock more than once without releasing
    /// it first, so in that sense the lock is re-entrant. However, the number of unlock operations
    /// must match the number of lock operations for the lock to actually be released.
    ///
    /// See [Postgres' documentation for the Advisory Lock Functions][advisory-funcs] for details.
    ///
    /// [advisory-funcs]: https://www.postgresql.org/docs/current/functions-admin.html#FUNCTIONS-ADVISORY-LOCKS
    pub async fn acquire<C: AsMut<PgConnection>>(
        &self,
        mut conn: C,
    ) -> Result<PgAdvisoryLockGuard<'_, C>> {
        match &self.key {
            PgAdvisoryLockKey::BigInt(key) => {
                crate::query::query("SELECT pg_advisory_lock($1)")
                    .bind(key)
                    .execute(conn.as_mut())
                    .await?;
            }
            PgAdvisoryLockKey::IntPair(key1, key2) => {
                crate::query::query("SELECT pg_advisory_lock($1, $2)")
                    .bind(key1)
                    .bind(key2)
                    .execute(conn.as_mut())
                    .await?;
            }
        }

        Ok(PgAdvisoryLockGuard::new(self, conn))
    }

    /// Acquires an exclusive lock using `pg_try_advisory_lock()`, returning immediately
    /// if the lock could not be acquired.
    ///
    /// For a version that waits until the lock is acquired, see [`Self::acquire()`].
    ///
    /// A connection-like type is required to execute the call. Allowed types include `PgConnection`,
    /// `PoolConnection<Postgres>` and `Transaction<Postgres>`, as well as mutable references to
    /// any of these. The connection is returned if the lock could not be acquired.
    ///
    /// The returned guard queues a `pg_advisory_unlock()` call on the connection when dropped,
    /// which will be executed the next time the connection is used, or when returned to a
    /// [`PgPool`][crate::PgPool] in the case of `PoolConnection<Postgres>`.
    ///
    /// Postgres allows a single connection to acquire a given lock more than once without releasing
    /// it first, so in that sense the lock is re-entrant. However, the number of unlock operations
    /// must match the number of lock operations for the lock to actually be released.
    ///
    /// See [Postgres' documentation for the Advisory Lock Functions][advisory-funcs] for details.
    ///
    /// [advisory-funcs]: https://www.postgresql.org/docs/current/functions-admin.html#FUNCTIONS-ADVISORY-LOCKS
    pub async fn try_acquire<C: AsMut<PgConnection>>(
        &self,
        mut conn: C,
    ) -> Result<Either<PgAdvisoryLockGuard<'_, C>, C>> {
        let locked: bool = match &self.key {
            PgAdvisoryLockKey::BigInt(key) => {
                crate::query_scalar::query_scalar("SELECT pg_try_advisory_lock($1)")
                    .bind(key)
                    .fetch_one(conn.as_mut())
                    .await?
            }
            PgAdvisoryLockKey::IntPair(key1, key2) => {
                crate::query_scalar::query_scalar("SELECT pg_try_advisory_lock($1, $2)")
                    .bind(key1)
                    .bind(key2)
                    .fetch_one(conn.as_mut())
                    .await?
            }
        };

        if locked {
            Ok(Either::Left(PgAdvisoryLockGuard::new(self, conn)))
        } else {
            Ok(Either::Right(conn))
        }
    }

    /// Execute `pg_advisory_unlock()` for this lock's key on the given connection.
    ///
    /// This is used by [`PgAdvisoryLockGuard::release_now()`] and is also provided for manually
    /// releasing the lock from connections returned by [`PgAdvisoryLockGuard::leak()`].
    ///
    /// An error should only be returned if there is something wrong with the connection,
    /// in which case the lock will be automatically released by the connection closing anyway.
    ///
    /// The `boolean` value is that returned by `pg_advisory_lock()`. If it is `false`, it
    /// indicates that the lock was not actually held by the given connection and that a warning
    /// has been logged by the Postgres server.
    pub async fn force_release<C: AsMut<PgConnection>>(&self, mut conn: C) -> Result<(C, bool)> {
        let released: bool = match &self.key {
            PgAdvisoryLockKey::BigInt(key) => {
                crate::query_scalar::query_scalar("SELECT pg_advisory_unlock($1)")
                    .bind(key)
                    .fetch_one(conn.as_mut())
                    .await?
            }
            PgAdvisoryLockKey::IntPair(key1, key2) => {
                crate::query_scalar::query_scalar("SELECT pg_advisory_unlock($1, $2)")
                    .bind(key1)
                    .bind(key2)
                    .fetch_one(conn.as_mut())
                    .await?
            }
        };

        Ok((conn, released))
    }

    fn get_release_query(&self) -> &str {
        self.release_query.get_or_init(|| match &self.key {
            PgAdvisoryLockKey::BigInt(key) => format!("SELECT pg_advisory_unlock({key})"),
            PgAdvisoryLockKey::IntPair(key1, key2) => {
                format!("SELECT pg_advisory_unlock({key1}, {key2})")
            }
        })
    }
}

impl PgAdvisoryLockKey {
    /// Converts `Self::Bigint(bigint)` to `Some(bigint)` and all else to `None`.
    pub fn as_bigint(&self) -> Option<i64> {
        if let Self::BigInt(bigint) = self {
            Some(*bigint)
        } else {
            None
        }
    }
}

const NONE_ERR: &str = "BUG: PgAdvisoryLockGuard.conn taken";

impl<'lock, C: AsMut<PgConnection>> PgAdvisoryLockGuard<'lock, C> {
    fn new(lock: &'lock PgAdvisoryLock, conn: C) -> Self {
        PgAdvisoryLockGuard {
            lock,
            conn: Some(conn),
        }
    }

    /// Immediately release the held advisory lock instead of when the connection is next used.
    ///
    /// An error should only be returned if there is something wrong with the connection,
    /// in which case the lock will be automatically released by the connection closing anyway.
    ///
    /// If `pg_advisory_unlock()` returns `false`, a warning will be logged, both by SQLx as
    /// well as the Postgres server. This would only happen if the lock was released without
    /// using this guard, or the connection was swapped using [`std::mem::replace()`].
    pub async fn release_now(mut self) -> Result<C> {
        let (conn, released) = self
            .lock
            .force_release(self.conn.take().expect(NONE_ERR))
            .await?;

        if !released {
            tracing::warn!(
                lock = ?self.lock.key,
                "PgAdvisoryLockGuard: advisory lock was not held by the contained connection",
            );
        }

        Ok(conn)
    }

    /// Cancel the release of the advisory lock, keeping it held until the connection is closed.
    ///
    /// To manually release the lock later, see [`PgAdvisoryLock::force_release()`].
    pub fn leak(mut self) -> C {
        self.conn.take().expect(NONE_ERR)
    }
}

impl<'lock, C: AsMut<PgConnection> + AsRef<PgConnection>> Deref for PgAdvisoryLockGuard<'lock, C> {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        self.conn.as_ref().expect(NONE_ERR).as_ref()
    }
}

/// Mutable access to the underlying connection is provided so it can still be used like normal,
/// even allowing locks to be taken recursively.
///
/// However, replacing the connection with a different one using, e.g. [`std::mem::replace()`]
/// is a logic error and will cause a warning to be logged by the PostgreSQL server when this
/// guard attempts to release the lock.
impl<'lock, C: AsMut<PgConnection> + AsRef<PgConnection>> DerefMut
    for PgAdvisoryLockGuard<'lock, C>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn.as_mut().expect(NONE_ERR).as_mut()
    }
}

impl<'lock, C: AsMut<PgConnection> + AsRef<PgConnection>> AsRef<PgConnection>
    for PgAdvisoryLockGuard<'lock, C>
{
    fn as_ref(&self) -> &PgConnection {
        self.conn.as_ref().expect(NONE_ERR).as_ref()
    }
}

/// Mutable access to the underlying connection is provided so it can still be used like normal,
/// even allowing locks to be taken recursively.
///
/// However, replacing the connection with a different one using, e.g. [`std::mem::replace()`]
/// is a logic error and will cause a warning to be logged by the PostgreSQL server when this
/// guard attempts to release the lock.
impl<'lock, C: AsMut<PgConnection>> AsMut<PgConnection> for PgAdvisoryLockGuard<'lock, C> {
    fn as_mut(&mut self) -> &mut PgConnection {
        self.conn.as_mut().expect(NONE_ERR).as_mut()
    }
}

/// Queues a `pg_advisory_unlock()` call on the wrapped connection which will be flushed
/// to the server the next time it is used, or when it is returned to [`PgPool`][crate::PgPool]
/// in the case of [`PoolConnection<Postgres>`][crate::pool::PoolConnection].
impl<'lock, C: AsMut<PgConnection>> Drop for PgAdvisoryLockGuard<'lock, C> {
    fn drop(&mut self) {
        if let Some(mut conn) = self.conn.take() {
            // Queue a simple query message to execute next time the connection is used.
            // The `async fn` versions can safely use the prepared statement protocol,
            // but this is the safest way to queue a query to execute on the next opportunity.
            conn.as_mut()
                .queue_simple_query(self.lock.get_release_query())
                .expect("BUG: PgAdvisoryLock::get_release_query() somehow too long for protocol");
        }
    }
}
