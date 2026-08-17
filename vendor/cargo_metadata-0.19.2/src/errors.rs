use std::{io, str::Utf8Error, string::FromUtf8Error};

/// Custom result type for `cargo_metadata::Error`
pub type Result<T> = ::std::result::Result<T, Error>;

/// Error returned when executing/parsing `cargo metadata` fails.
///
/// # Note about Backtraces
///
/// This error type does not contain backtraces, but each error variant
/// comes from _one_ specific place, so it's not really needed for the
/// inside of this crate. If you need a backtrace down to, but not inside
/// of, a failed call of `cargo_metadata` you can do one of multiple thinks:
///
/// 1. Convert it to a `failure::Error` (possible using the `?` operator),
///    which is similar to a `Box<::std::error::Error + 'static + Send  + Sync>`.
/// 2. Have appropriate variants in your own error type. E.g. you could wrap
///    a `failure::Context<Error>` or add a `failure::Backtrace` field (which
///    is empty if `RUST_BACKTRACE` is not set, so it's simple to use).
/// 3. You still can place a failure based error into a `error_chain` if you
///    really want to. (Either through foreign_links or by making it a field
///    value of a `ErrorKind` variant).
///
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error during execution of `cargo metadata`
    #[error("`cargo metadata` exited with an error: {stderr}")]
    CargoMetadata {
        /// stderr returned by the `cargo metadata` command
        stderr: String,
    },

    /// IO Error during execution of `cargo metadata`
    #[error("failed to start `cargo metadata`: {0}")]
    Io(#[from] io::Error),

    /// Output of `cargo metadata` was not valid utf8
    #[error("cannot convert the stdout of `cargo metadata`: {0}")]
    Utf8(#[from] Utf8Error),

    /// Error output of `cargo metadata` was not valid utf8
    #[error("cannot convert the stderr of `cargo metadata`: {0}")]
    ErrUtf8(#[from] FromUtf8Error),

    /// Deserialization error (structure of json did not match expected structure)
    #[error("failed to interpret `cargo metadata`'s json: {0}")]
    Json(#[from] ::serde_json::Error),

    /// The output did not contain any json
    #[error("could not find any json in the output of `cargo metadata`")]
    NoJson,
}
