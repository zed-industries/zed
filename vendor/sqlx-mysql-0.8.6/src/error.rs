use std::error::Error as StdError;
use std::fmt::{self, Debug, Display, Formatter};

use crate::protocol::response::ErrPacket;

use std::borrow::Cow;

pub(crate) use sqlx_core::error::*;

/// An error returned from the MySQL database.
pub struct MySqlDatabaseError(pub(super) ErrPacket);

impl MySqlDatabaseError {
    /// The [SQLSTATE](https://dev.mysql.com/doc/mysql-errors/8.0/en/server-error-reference.html) code for this error.
    pub fn code(&self) -> Option<&str> {
        self.0.sql_state.as_deref()
    }

    /// The [number](https://dev.mysql.com/doc/mysql-errors/8.0/en/server-error-reference.html)
    /// for this error.
    ///
    /// MySQL tends to use SQLSTATE as a general error category, and the error number as a more
    /// granular indication of the error.
    pub fn number(&self) -> u16 {
        self.0.error_code
    }

    /// The human-readable error message.
    pub fn message(&self) -> &str {
        &self.0.error_message
    }
}

impl Debug for MySqlDatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MySqlDatabaseError")
            .field("code", &self.code())
            .field("number", &self.number())
            .field("message", &self.message())
            .finish()
    }
}

impl Display for MySqlDatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(code) = &self.code() {
            write!(f, "{} ({}): {}", self.number(), code, self.message())
        } else {
            write!(f, "{}: {}", self.number(), self.message())
        }
    }
}

impl StdError for MySqlDatabaseError {}

impl DatabaseError for MySqlDatabaseError {
    #[inline]
    fn message(&self) -> &str {
        self.message()
    }

    #[inline]
    fn code(&self) -> Option<Cow<'_, str>> {
        self.code().map(Cow::Borrowed)
    }

    #[doc(hidden)]
    fn as_error(&self) -> &(dyn StdError + Send + Sync + 'static) {
        self
    }

    #[doc(hidden)]
    fn as_error_mut(&mut self) -> &mut (dyn StdError + Send + Sync + 'static) {
        self
    }

    #[doc(hidden)]
    fn into_error(self: Box<Self>) -> Box<dyn StdError + Send + Sync + 'static> {
        self
    }

    fn kind(&self) -> ErrorKind {
        match self.number() {
            error_codes::ER_DUP_KEY
            | error_codes::ER_DUP_ENTRY
            | error_codes::ER_DUP_UNIQUE
            | error_codes::ER_DUP_ENTRY_WITH_KEY_NAME
            | error_codes::ER_DUP_UNKNOWN_IN_INDEX => ErrorKind::UniqueViolation,

            error_codes::ER_NO_REFERENCED_ROW
            | error_codes::ER_NO_REFERENCED_ROW_2
            | error_codes::ER_ROW_IS_REFERENCED
            | error_codes::ER_ROW_IS_REFERENCED_2
            | error_codes::ER_FK_COLUMN_NOT_NULL
            | error_codes::ER_FK_CANNOT_DELETE_PARENT => ErrorKind::ForeignKeyViolation,

            error_codes::ER_BAD_NULL_ERROR | error_codes::ER_NO_DEFAULT_FOR_FIELD => {
                ErrorKind::NotNullViolation
            }

            error_codes::ER_CHECK_CONSTRAINT_VIOLATED => ErrorKind::CheckViolation,

            // https://mariadb.com/kb/en/e4025/
            error_codes::mariadb::ER_CONSTRAINT_FAILED
                // MySQL uses this code for a completely different error,
                // but we can differentiate by SQLSTATE:
                // <https://dev.mysql.com/doc/mysql-errors/8.4/en/server-error-reference.html#error_er_innodb_autoextend_size_out_of_range
                if self.0.sql_state.as_deref() == Some("23000") =>
            {
                ErrorKind::CheckViolation
            }

            _ => ErrorKind::Other,
        }
    }
}

/// The MySQL server uses SQLSTATEs as a generic error category,
/// and returns a `error_code` instead within the error packet.
///
/// For reference: <https://dev.mysql.com/doc/mysql-errors/8.0/en/server-error-reference.html>.
pub(crate) mod error_codes {
    /// Caused when a DDL operation creates duplicated keys.
    pub const ER_DUP_KEY: u16 = 1022;
    /// Caused when a DML operation tries create a duplicated entry for a key,
    /// be it a unique or primary one.
    pub const ER_DUP_ENTRY: u16 = 1062;
    /// Similar to `ER_DUP_ENTRY`, but only present in NDB clusters.
    ///
    /// See: <https://github.com/mysql/mysql-server/blob/fbdaa4def30d269bc4de5b85de61de34b11c0afc/mysql-test/suite/stress/include/ddl7.inc#L68>.
    pub const ER_DUP_UNIQUE: u16 = 1169;
    /// Similar to `ER_DUP_ENTRY`, but with a formatted string message.
    ///
    /// See: <https://bugs.mysql.com/bug.php?id=46976>.
    pub const ER_DUP_ENTRY_WITH_KEY_NAME: u16 = 1586;
    /// Caused when a DDL operation to add a unique index fails,
    /// because duplicate items were created by concurrent DML operations.
    /// When this happens, the key is unknown, so the server can't use `ER_DUP_KEY`.
    ///
    /// For example: an `INSERT` operation creates duplicate `name` fields when `ALTER`ing a table and making `name` unique.
    pub const ER_DUP_UNKNOWN_IN_INDEX: u16 = 1859;

    /// Caused when inserting an entry with a column with a value that does not reference a foreign row.
    pub const ER_NO_REFERENCED_ROW: u16 = 1216;
    /// Caused when deleting a row that is referenced in other tables.
    pub const ER_ROW_IS_REFERENCED: u16 = 1217;
    /// Caused when deleting a row that is referenced in other tables.
    /// This differs from `ER_ROW_IS_REFERENCED` in that the error message contains the affected constraint.
    pub const ER_ROW_IS_REFERENCED_2: u16 = 1451;
    /// Caused when inserting an entry with a column with a value that does not reference a foreign row.
    /// This differs from `ER_NO_REFERENCED_ROW` in that the error message contains the affected constraint.
    pub const ER_NO_REFERENCED_ROW_2: u16 = 1452;
    /// Caused when creating a FK with `ON DELETE SET NULL` or `ON UPDATE SET NULL` to a column that is `NOT NULL`, or vice-versa.
    pub const ER_FK_COLUMN_NOT_NULL: u16 = 1830;
    /// Removed in 5.7.3.
    pub const ER_FK_CANNOT_DELETE_PARENT: u16 = 1834;

    /// Caused when inserting a NULL value to a column marked as NOT NULL.
    pub const ER_BAD_NULL_ERROR: u16 = 1048;
    /// Caused when inserting a DEFAULT value to a column marked as NOT NULL, which also doesn't have a default value set.
    pub const ER_NO_DEFAULT_FOR_FIELD: u16 = 1364;

    /// Caused when a check constraint is violated.
    ///
    /// Only available after 8.0.16.
    pub const ER_CHECK_CONSTRAINT_VIOLATED: u16 = 3819;

    pub(crate) mod mariadb {
        /// Error code emitted by MariaDB for constraint errors: <https://mariadb.com/kb/en/e4025/>
        ///
        /// MySQL emits this code for a completely different error:
        /// <https://dev.mysql.com/doc/mysql-errors/8.4/en/server-error-reference.html#error_er_innodb_autoextend_size_out_of_range>
        ///
        /// You also check that SQLSTATE is `23000`.
        pub const ER_CONSTRAINT_FAILED: u16 = 4025;
    }
}
