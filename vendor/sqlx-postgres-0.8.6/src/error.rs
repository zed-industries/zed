use std::error::Error as StdError;
use std::fmt::{self, Debug, Display, Formatter};

use atoi::atoi;
use smallvec::alloc::borrow::Cow;
use sqlx_core::bytes::Bytes;
pub(crate) use sqlx_core::error::*;

use crate::message::{BackendMessage, BackendMessageFormat, Notice, PgSeverity};

/// An error returned from the PostgreSQL database.
pub struct PgDatabaseError(pub(crate) Notice);

// Error message fields are documented:
// https://www.postgresql.org/docs/current/protocol-error-fields.html

impl PgDatabaseError {
    #[inline]
    pub fn severity(&self) -> PgSeverity {
        self.0.severity()
    }

    /// The [SQLSTATE](https://www.postgresql.org/docs/current/errcodes-appendix.html) code for
    /// this error.
    #[inline]
    pub fn code(&self) -> &str {
        self.0.code()
    }

    /// The primary human-readable error message. This should be accurate but
    /// terse (typically one line).
    #[inline]
    pub fn message(&self) -> &str {
        self.0.message()
    }

    /// An optional secondary error message carrying more detail about the problem.
    /// Might run to multiple lines.
    #[inline]
    pub fn detail(&self) -> Option<&str> {
        self.0.get(b'D')
    }

    /// An optional suggestion what to do about the problem. This is intended to differ from
    /// `detail` in that it offers advice (potentially inappropriate) rather than hard facts.
    /// Might run to multiple lines.
    #[inline]
    pub fn hint(&self) -> Option<&str> {
        self.0.get(b'H')
    }

    /// Indicates an error cursor position as an index into the original query string; or,
    /// a position into an internally generated query.
    #[inline]
    pub fn position(&self) -> Option<PgErrorPosition<'_>> {
        self.0
            .get_raw(b'P')
            .and_then(atoi)
            .map(PgErrorPosition::Original)
            .or_else(|| {
                let position = self.0.get_raw(b'p').and_then(atoi)?;
                let query = self.0.get(b'q')?;

                Some(PgErrorPosition::Internal { position, query })
            })
    }

    /// An indication of the context in which the error occurred. Presently this includes a call
    /// stack traceback of active procedural language functions and internally-generated queries.
    /// The trace is one entry per line, most recent first.
    pub fn r#where(&self) -> Option<&str> {
        self.0.get(b'W')
    }

    /// If this error is with a specific database object, the
    /// name of the schema containing that object, if any.
    pub fn schema(&self) -> Option<&str> {
        self.0.get(b's')
    }

    /// If this error is with a specific table, the name of the table.
    pub fn table(&self) -> Option<&str> {
        self.0.get(b't')
    }

    /// If the error is with a specific table column, the name of the column.
    pub fn column(&self) -> Option<&str> {
        self.0.get(b'c')
    }

    /// If the error is with a specific data type, the name of the data type.
    pub fn data_type(&self) -> Option<&str> {
        self.0.get(b'd')
    }

    /// If the error is with a specific constraint, the name of the constraint.
    /// For this purpose, indexes are constraints, even if they weren't created
    /// with constraint syntax.
    pub fn constraint(&self) -> Option<&str> {
        self.0.get(b'n')
    }

    /// The file name of the source-code location where this error was reported.
    pub fn file(&self) -> Option<&str> {
        self.0.get(b'F')
    }

    /// The line number of the source-code location where this error was reported.
    pub fn line(&self) -> Option<usize> {
        self.0.get_raw(b'L').and_then(atoi)
    }

    /// The name of the source-code routine reporting this error.
    pub fn routine(&self) -> Option<&str> {
        self.0.get(b'R')
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum PgErrorPosition<'a> {
    /// A position (in characters) into the original query.
    Original(usize),

    /// A position into the internally-generated query.
    Internal {
        /// The position in characters.
        position: usize,

        /// The text of a failed internally-generated command. This could be, for example,
        /// the SQL query issued by a PL/pgSQL function.
        query: &'a str,
    },
}

impl Debug for PgDatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PgDatabaseError")
            .field("severity", &self.severity())
            .field("code", &self.code())
            .field("message", &self.message())
            .field("detail", &self.detail())
            .field("hint", &self.hint())
            .field("position", &self.position())
            .field("where", &self.r#where())
            .field("schema", &self.schema())
            .field("table", &self.table())
            .field("column", &self.column())
            .field("data_type", &self.data_type())
            .field("constraint", &self.constraint())
            .field("file", &self.file())
            .field("line", &self.line())
            .field("routine", &self.routine())
            .finish()
    }
}

impl Display for PgDatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}

impl StdError for PgDatabaseError {}

impl DatabaseError for PgDatabaseError {
    fn message(&self) -> &str {
        self.message()
    }

    fn code(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(self.code()))
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
    fn into_error(self: Box<Self>) -> BoxDynError {
        self
    }

    fn is_transient_in_connect_phase(&self) -> bool {
        // https://www.postgresql.org/docs/current/errcodes-appendix.html
        [
            // too_many_connections
            // This may be returned if we just un-gracefully closed a connection,
            // give the database a chance to notice it and clean it up.
            "53300",
            // cannot_connect_now
            // Returned if the database is still starting up.
            "57P03",
        ]
        .contains(&self.code())
    }

    fn constraint(&self) -> Option<&str> {
        self.constraint()
    }

    fn table(&self) -> Option<&str> {
        self.table()
    }

    fn kind(&self) -> ErrorKind {
        match self.code() {
            error_codes::UNIQUE_VIOLATION => ErrorKind::UniqueViolation,
            error_codes::FOREIGN_KEY_VIOLATION => ErrorKind::ForeignKeyViolation,
            error_codes::NOT_NULL_VIOLATION => ErrorKind::NotNullViolation,
            error_codes::CHECK_VIOLATION => ErrorKind::CheckViolation,
            _ => ErrorKind::Other,
        }
    }
}

// ErrorResponse is the same structure as NoticeResponse but a different format code.
impl BackendMessage for PgDatabaseError {
    const FORMAT: BackendMessageFormat = BackendMessageFormat::ErrorResponse;

    #[inline(always)]
    fn decode_body(buf: Bytes) -> std::result::Result<Self, Error> {
        Ok(Self(Notice::decode_body(buf)?))
    }
}

/// For reference: <https://www.postgresql.org/docs/current/errcodes-appendix.html>
pub(crate) mod error_codes {
    /// Caused when a unique or primary key is violated.
    pub const UNIQUE_VIOLATION: &str = "23505";
    /// Caused when a foreign key is violated.
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
    /// Caused when a column marked as NOT NULL received a null value.
    pub const NOT_NULL_VIOLATION: &str = "23502";
    /// Caused when a check constraint is violated.
    pub const CHECK_VIOLATION: &str = "23514";
}
