use std::borrow::Cow;
use std::ops::Deref;
use std::ptr::{self, NonNull};

use crate::envs::Env;
use crate::mdb::error::mdb_result;
use crate::mdb::ffi;
use crate::Result;

/// A read-only transaction.
///
/// ## LMDB Limitations
///
/// It's a must to keep read transactions short-lived.
///
/// Active Read transactions prevent the reuse of pages freed
/// by newer write transactions, thus the database can grow quickly.
///
/// ## OSX/Darwin Limitation
///
/// At least 10 transactions can be active at the same time in the same process, since only 10 POSIX semaphores can
/// be active at the same time for a process. Threads are in the same process space.
///
/// If the process crashes in the POSIX semaphore locking section of the transaction, the semaphore will be kept locked.
///
/// Note: if your program already use POSIX semaphores, you will have less available for heed/LMDB!
///
/// You may increase the limit by editing it **at your own risk**: `/Library/LaunchDaemons/sysctl.plist`
///
/// ## This struct is covariant
///
/// ```rust
/// #[allow(dead_code)]
/// trait CovariantMarker<'a>: 'static {
///     type T: 'a;
///
///     fn is_covariant(&'a self) -> &'a Self::T;
/// }
///
/// impl<'a> CovariantMarker<'a> for heed::RoTxn<'static> {
///     type T = heed::RoTxn<'a>;
///
///     fn is_covariant(&'a self) -> &'a heed::RoTxn<'a> {
///         self
///     }
/// }
/// ```
pub struct RoTxn<'e> {
    /// Makes the struct covariant and !Sync
    pub(crate) txn: Option<NonNull<ffi::MDB_txn>>,
    env: Cow<'e, Env>,
}

impl<'e> RoTxn<'e> {
    pub(crate) fn new(env: &'e Env) -> Result<RoTxn<'e>> {
        let mut txn: *mut ffi::MDB_txn = ptr::null_mut();

        unsafe {
            mdb_result(ffi::mdb_txn_begin(
                env.env_mut_ptr().as_mut(),
                ptr::null_mut(),
                ffi::MDB_RDONLY,
                &mut txn,
            ))?
        };

        Ok(RoTxn { txn: NonNull::new(txn), env: Cow::Borrowed(env) })
    }

    pub(crate) fn static_read_txn(env: Env) -> Result<RoTxn<'static>> {
        let mut txn: *mut ffi::MDB_txn = ptr::null_mut();

        unsafe {
            mdb_result(ffi::mdb_txn_begin(
                env.env_mut_ptr().as_mut(),
                ptr::null_mut(),
                ffi::MDB_RDONLY,
                &mut txn,
            ))?
        };

        Ok(RoTxn { txn: NonNull::new(txn), env: Cow::Owned(env) })
    }

    pub(crate) fn env_mut_ptr(&self) -> NonNull<ffi::MDB_env> {
        self.env.env_mut_ptr()
    }

    /// Commit a read transaction.
    ///
    /// Synchronizing some [`Env`] metadata with the global handle.
    ///
    /// ## LMDB
    ///
    /// It's mandatory in a multi-process setup to call [`RoTxn::commit`] upon read-only database opening.
    /// After the transaction opening, the database is dropped. The next transaction might return
    /// `Io(Os { code: 22, kind: InvalidInput, message: "Invalid argument" })` known as `EINVAL`.
    pub fn commit(mut self) -> Result<()> {
        // Asserts that the transaction hasn't been already
        // committed/aborter and ensure we cannot use it twice.
        let mut txn = self.txn.take().unwrap();
        let result = unsafe { mdb_result(ffi::mdb_txn_commit(txn.as_mut())) };
        result.map_err(Into::into)
    }
}

impl Drop for RoTxn<'_> {
    fn drop(&mut self) {
        if let Some(mut txn) = self.txn.take() {
            // Asserts that the transaction hasn't been already
            // committed/aborter and ensure we cannot use it twice.
            unsafe { ffi::mdb_txn_abort(txn.as_mut()) }
        }
    }
}

#[cfg(feature = "read-txn-no-tls")]
unsafe impl Send for RoTxn<'_> {}

/// A read-write transaction.
///
/// ## LMDB Limitations
///
/// Only one [`RwTxn`] may exist in the same environment at the same time.
/// If two exist, the new one may wait on a mutex for [`RwTxn::commit`] or [`RwTxn::abort`] to
/// be called for the first one.
///
/// ## OSX/Darwin Limitation
///
/// At least 10 transactions can be active at the same time in the same process, since only 10 POSIX semaphores can
/// be active at the same time for a process. Threads are in the same process space.
///
/// If the process crashes in the POSIX semaphore locking section of the transaction, the semaphore will be kept locked.
///
/// Note: if your program already use POSIX semaphores, you will have less available for heed/LMDB!
///
/// You may increase the limit by editing it **at your own risk**: `/Library/LaunchDaemons/sysctl.plist`
///
/// ## This struct is covariant
///
/// ```rust
/// #[allow(dead_code)]
/// trait CovariantMarker<'a>: 'static {
///     type T: 'a;
///
///     fn is_covariant(&'a self) -> &'a Self::T;
/// }
///
/// impl<'a> CovariantMarker<'a> for heed::RwTxn<'static> {
///     type T = heed::RwTxn<'a>;
///
///     fn is_covariant(&'a self) -> &'a heed::RwTxn<'a> {
///         self
///     }
/// }
/// ```
pub struct RwTxn<'p> {
    pub(crate) txn: RoTxn<'p>,
}

impl<'p> RwTxn<'p> {
    pub(crate) fn new(env: &'p Env) -> Result<RwTxn<'p>> {
        let mut txn: *mut ffi::MDB_txn = ptr::null_mut();

        unsafe {
            mdb_result(ffi::mdb_txn_begin(
                env.env_mut_ptr().as_mut(),
                ptr::null_mut(),
                0,
                &mut txn,
            ))?
        };

        Ok(RwTxn { txn: RoTxn { txn: NonNull::new(txn), env: Cow::Borrowed(env) } })
    }

    pub(crate) fn nested(env: &'p Env, parent: &'p mut RwTxn) -> Result<RwTxn<'p>> {
        let mut txn: *mut ffi::MDB_txn = ptr::null_mut();
        let parent_ptr: *mut ffi::MDB_txn = unsafe { parent.txn.txn.unwrap().as_mut() };

        unsafe {
            mdb_result(ffi::mdb_txn_begin(env.env_mut_ptr().as_mut(), parent_ptr, 0, &mut txn))?
        };

        Ok(RwTxn { txn: RoTxn { txn: NonNull::new(txn), env: Cow::Borrowed(env) } })
    }

    pub(crate) fn env_mut_ptr(&self) -> NonNull<ffi::MDB_env> {
        self.txn.env.env_mut_ptr()
    }

    /// Commit all the operations of a transaction into the database.
    /// The transaction is reset.
    pub fn commit(mut self) -> Result<()> {
        // Asserts that the transaction hasn't been already
        // committed/aborter and ensure we cannot use it two times.
        let mut txn = self.txn.txn.take().unwrap();
        let result = unsafe { mdb_result(ffi::mdb_txn_commit(txn.as_mut())) };
        result.map_err(Into::into)
    }

    /// Abandon all the operations of the transaction instead of saving them.
    /// The transaction is reset.
    pub fn abort(mut self) {
        // Asserts that the transaction hasn't been already
        // committed/aborter and ensure we cannot use it twice.
        let mut txn = self.txn.txn.take().unwrap();
        unsafe { ffi::mdb_txn_abort(txn.as_mut()) }
    }
}

impl<'p> Deref for RwTxn<'p> {
    type Target = RoTxn<'p>;

    fn deref(&self) -> &Self::Target {
        &self.txn
    }
}

// TODO can't we just always implement it?
#[cfg(master3)]
impl std::ops::DerefMut for RwTxn<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.txn
    }
}
