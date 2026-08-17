use std::error::Error as StdError;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::{fmt, str};

use libc::c_int;
#[cfg(master3)]
use lmdb_master3_sys as ffi;
#[cfg(not(master3))]
use lmdb_master_sys as ffi;

/// An LMDB error kind.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    /// A key/data pair already exists.
    ///
    /// May be also returned by append functions. Data passed
    /// doesn't respect the database ordering.
    KeyExist,
    /// A key/data pair was not found (EOF).
    NotFound,
    /// Requested page not found - this usually indicates corruption.
    PageNotFound,
    /// Located page was wrong type.
    Corrupted,
    /// Update of meta page failed or environment had fatal error.
    Panic,
    /// Environment version mismatch.
    VersionMismatch,
    /// File is not a valid LMDB file.
    Invalid,
    /// Environment mapsize reached.
    MapFull,
    /// Environment maxdbs reached.
    DbsFull,
    /// Environment maxreaders reached.
    ReadersFull,
    /// Too many TLS keys in use - Windows only.
    TlsFull,
    /// Txn has too many dirty pages.
    TxnFull,
    /// Cursor stack too deep - internal error.
    CursorFull,
    /// Page has not enough space - internal error.
    PageFull,
    /// Database contents grew beyond environment mapsize.
    MapResized,
    /// Operation and DB incompatible, or DB type changed. This can mean:
    ///   - The operation expects an MDB_DUPSORT / MDB_DUPFIXED database.
    ///   - Opening a named DB when the unnamed DB has MDB_DUPSORT / MDB_INTEGERKEY.
    ///   - Accessing a data record as a database, or vice versa.
    ///   - The database was dropped and recreated with different flags.
    Incompatible,
    /// Invalid reuse of reader locktable slot.
    BadRslot,
    /// Transaction cannot recover - it must be aborted.
    BadTxn,
    /// Unsupported size of key/DB name/data, or wrong DUP_FIXED size.
    ///
    /// Common causes of this error:
    ///   - You tried to store a zero-length key
    ///   - You tried to store a key longer than the max allowed key (511 bytes by default)
    ///   - You are using [DUP_SORT](crate::DatabaseFlags::DUP_SORT) and trying to store a
    ///     value longer than the max allowed key size (511 bytes by default)
    ///
    /// In the last two cases you can enable the `longer-keys` feature to increase the max allowed key size.
    BadValSize,
    /// The specified DBI was changed unexpectedly.
    BadDbi,
    /// Unexpected problem - transaction should abort.
    Problem,
    /// Page checksum incorrect.
    #[cfg(master3)]
    BadChecksum,
    /// Encryption/decryption failed.
    #[cfg(master3)]
    CryptoFail,
    /// Environment encryption mismatch.
    #[cfg(master3)]
    EnvEncryption,
    /// Other error.
    Other(c_int),
}

impl Error {
    /// Returns `true` if the given error is [`Error::NotFound`].
    pub fn not_found(&self) -> bool {
        *self == Error::NotFound
    }

    /// Converts a raw error code to an `Error`.
    pub fn from_err_code(err_code: c_int) -> Error {
        match err_code {
            ffi::MDB_KEYEXIST => Error::KeyExist,
            ffi::MDB_NOTFOUND => Error::NotFound,
            ffi::MDB_PAGE_NOTFOUND => Error::PageNotFound,
            ffi::MDB_CORRUPTED => Error::Corrupted,
            ffi::MDB_PANIC => Error::Panic,
            ffi::MDB_VERSION_MISMATCH => Error::VersionMismatch,
            ffi::MDB_INVALID => Error::Invalid,
            ffi::MDB_MAP_FULL => Error::MapFull,
            ffi::MDB_DBS_FULL => Error::DbsFull,
            ffi::MDB_READERS_FULL => Error::ReadersFull,
            ffi::MDB_TLS_FULL => Error::TlsFull,
            ffi::MDB_TXN_FULL => Error::TxnFull,
            ffi::MDB_CURSOR_FULL => Error::CursorFull,
            ffi::MDB_PAGE_FULL => Error::PageFull,
            ffi::MDB_MAP_RESIZED => Error::MapResized,
            ffi::MDB_INCOMPATIBLE => Error::Incompatible,
            ffi::MDB_BAD_RSLOT => Error::BadRslot,
            ffi::MDB_BAD_TXN => Error::BadTxn,
            ffi::MDB_BAD_VALSIZE => Error::BadValSize,
            ffi::MDB_BAD_DBI => Error::BadDbi,
            ffi::MDB_PROBLEM => Error::Problem,
            #[cfg(master3)]
            ffi::MDB_BAD_CHECKSUM => Error::BadChecksum,
            #[cfg(master3)]
            ffi::MDB_CRYPTO_FAIL => Error::CryptoFail,
            #[cfg(master3)]
            ffi::MDB_ENV_ENCRYPTION => Error::EnvEncryption,
            other => Error::Other(other),
        }
    }

    /// Converts an `Error` to the raw error code.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn to_err_code(&self) -> c_int {
        match *self {
            Error::KeyExist => ffi::MDB_KEYEXIST,
            Error::NotFound => ffi::MDB_NOTFOUND,
            Error::PageNotFound => ffi::MDB_PAGE_NOTFOUND,
            Error::Corrupted => ffi::MDB_CORRUPTED,
            Error::Panic => ffi::MDB_PANIC,
            Error::VersionMismatch => ffi::MDB_VERSION_MISMATCH,
            Error::Invalid => ffi::MDB_INVALID,
            Error::MapFull => ffi::MDB_MAP_FULL,
            Error::DbsFull => ffi::MDB_DBS_FULL,
            Error::ReadersFull => ffi::MDB_READERS_FULL,
            Error::TlsFull => ffi::MDB_TLS_FULL,
            Error::TxnFull => ffi::MDB_TXN_FULL,
            Error::CursorFull => ffi::MDB_CURSOR_FULL,
            Error::PageFull => ffi::MDB_PAGE_FULL,
            Error::MapResized => ffi::MDB_MAP_RESIZED,
            Error::Incompatible => ffi::MDB_INCOMPATIBLE,
            Error::BadRslot => ffi::MDB_BAD_RSLOT,
            Error::BadTxn => ffi::MDB_BAD_TXN,
            Error::BadValSize => ffi::MDB_BAD_VALSIZE,
            Error::BadDbi => ffi::MDB_BAD_DBI,
            Error::Problem => ffi::MDB_PROBLEM,
            #[cfg(master3)]
            Error::BadChecksum => ffi::MDB_BAD_CHECKSUM,
            #[cfg(master3)]
            Error::CryptoFail => ffi::MDB_CRYPTO_FAIL,
            #[cfg(master3)]
            Error::EnvEncryption => ffi::MDB_ENV_ENCRYPTION,
            Error::Other(err_code) => err_code,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let description = unsafe {
            // This is safe since the error messages returned from mdb_strerror are static.
            let err: *const c_char = ffi::mdb_strerror(self.to_err_code()) as *const c_char;
            str::from_utf8_unchecked(CStr::from_ptr(err).to_bytes())
        };

        fmt.write_str(description)
    }
}

impl StdError for Error {}

pub fn mdb_result(err_code: c_int) -> Result<(), Error> {
    if err_code == ffi::MDB_SUCCESS {
        Ok(())
    } else {
        Err(Error::from_err_code(err_code))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_description() {
        assert_eq!("Permission denied", Error::from_err_code(13).to_string());
        assert_eq!("MDB_NOTFOUND: No matching key/data pair found", Error::NotFound.to_string());
    }
}
