use std::fmt;
use std::fs::File;
use std::panic::catch_unwind;
use std::path::Path;

use aead::generic_array::typenum::Unsigned;
use aead::{AeadMutInPlace, Key, KeyInit, Nonce, Tag};

use super::{Env, EnvClosingEvent, EnvInfo, FlagSetMode};
use crate::databases::{EncryptedDatabase, EncryptedDatabaseOpenOptions};
use crate::mdb::ffi::{self};
use crate::{CompactionOption, EnvFlags, Result, RoTxn, RwTxn, Unspecified};
#[allow(unused)] // fro cargo auto doc links
use crate::{Database, EnvOpenOptions};

/// An environment handle constructed by using [`EnvOpenOptions::open_encrypted`].
#[derive(Clone)]
pub struct EncryptedEnv {
    pub(crate) inner: Env,
}

impl EncryptedEnv {
    /// The size of the data file on disk.
    ///
    /// # Example
    ///
    /// ```
    /// use heed::EnvOpenOptions;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dir = tempfile::tempdir()?;
    /// let size_in_bytes = 1024 * 1024;
    /// let env = unsafe { EnvOpenOptions::new().map_size(size_in_bytes).open(dir.path())? };
    ///
    /// let actual_size = env.real_disk_size()? as usize;
    /// assert!(actual_size < size_in_bytes);
    /// # Ok(()) }
    /// ```
    pub fn real_disk_size(&self) -> Result<u64> {
        self.inner.real_disk_size()
    }

    /// Return the raw flags the environment was opened with.
    ///
    /// Returns `None` if the environment flags are different from the [`EnvFlags`] set.
    pub fn flags(&self) -> Result<Option<EnvFlags>> {
        self.inner.flags()
    }

    /// Enable or disable the environment's currently active [`EnvFlags`].
    ///
    /// ```
    /// use std::fs;
    /// use std::path::Path;
    /// use heed::{EnvOpenOptions, Database, EnvFlags, FlagSetMode};
    /// use heed::types::*;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut env_builder = EnvOpenOptions::new();
    /// let dir = tempfile::tempdir().unwrap();
    /// let env = unsafe { env_builder.open(dir.path())? };
    ///
    /// // Env was opened without flags.
    /// assert_eq!(env.get_flags().unwrap(), EnvFlags::empty().bits());
    ///
    /// // Enable a flag after opening.
    /// unsafe { env.set_flags(EnvFlags::NO_SYNC, FlagSetMode::Enable).unwrap(); }
    /// assert_eq!(env.get_flags().unwrap(), EnvFlags::NO_SYNC.bits());
    ///
    /// // Disable a flag after opening.
    /// unsafe { env.set_flags(EnvFlags::NO_SYNC, FlagSetMode::Disable).unwrap(); }
    /// assert_eq!(env.get_flags().unwrap(), EnvFlags::empty().bits());
    /// # Ok(()) }
    /// ```
    ///
    /// # Safety
    ///
    /// It is unsafe to use unsafe LMDB flags such as `NO_SYNC`, `NO_META_SYNC`, or `NO_LOCK`.
    ///
    /// LMDB also requires that only 1 thread calls this function at any given moment.
    /// Neither `heed` or LMDB check for this condition, so the caller must ensure it explicitly.
    pub unsafe fn set_flags(&self, flags: EnvFlags, mode: FlagSetMode) -> Result<()> {
        self.inner.set_flags(flags, mode)
    }

    /// Return the raw flags the environment is currently set with.
    pub fn get_flags(&self) -> Result<u32> {
        self.inner.get_flags()
    }

    /// Returns some basic informations about this environment.
    pub fn info(&self) -> EnvInfo {
        self.inner.info()
    }

    /// Returns the size used by all the databases in the environment without the free pages.
    ///
    /// It is crucial to configure [`EnvOpenOptions::max_dbs`] with a sufficiently large value
    /// before invoking this function. All databases within the environment will be opened
    /// and remain so.
    pub fn non_free_pages_size(&self) -> Result<u64> {
        self.inner.non_free_pages_size()
    }

    /// Options and flags which can be used to configure how a [`Database`] is opened.
    pub fn database_options(&self) -> EncryptedDatabaseOpenOptions<Unspecified, Unspecified> {
        EncryptedDatabaseOpenOptions::new(self)
    }

    /// Opens a typed database that already exists in this environment.
    ///
    /// If the database was previously opened in this program run, types will be checked.
    ///
    /// ## Important Information
    ///
    /// LMDB has an important restriction on the unnamed database when named ones are opened.
    /// The names of the named databases are stored as keys in the unnamed one and are immutable,
    /// and these keys can only be read and not written.
    ///
    /// ## LMDB read-only access of existing database
    ///
    /// In the case of accessing a database in a read-only manner from another process
    /// where you wrote, you might need to manually call [`RoTxn::commit`] to get metadata
    /// and the database handles opened and shared with the global [`Env`] handle.
    ///
    /// If not done, you might raise `Io(Os { code: 22, kind: InvalidInput, message: "Invalid argument" })`
    /// known as `EINVAL`.
    pub fn open_database<KC, DC>(
        &self,
        rtxn: &RoTxn,
        name: Option<&str>,
    ) -> Result<Option<EncryptedDatabase<KC, DC>>>
    where
        KC: 'static,
        DC: 'static,
    {
        let mut options = self.database_options().types::<KC, DC>();
        if let Some(name) = name {
            options.name(name);
        }
        options.open(rtxn)
    }

    /// Creates a typed database that can already exist in this environment.
    ///
    /// If the database was previously opened during this program run, types will be checked.
    ///
    /// ## Important Information
    ///
    /// LMDB has an important restriction on the unnamed database when named ones are opened.
    /// The names of the named databases are stored as keys in the unnamed one and are immutable,
    /// and these keys can only be read and not written.
    pub fn create_database<KC, DC>(
        &self,
        wtxn: &mut RwTxn,
        name: Option<&str>,
    ) -> Result<EncryptedDatabase<KC, DC>>
    where
        KC: 'static,
        DC: 'static,
    {
        let mut options = self.database_options().types::<KC, DC>();
        if let Some(name) = name {
            options.name(name);
        }
        options.create(wtxn)
    }

    /// Create a transaction with read and write access for use with the environment.
    ///
    /// ## LMDB Limitations
    ///
    /// Only one [`RwTxn`] may exist simultaneously in the current environment.
    /// If another write transaction is initiated, while another write transaction exists
    /// the thread initiating the new one will wait on a mutex upon completion of the previous
    /// transaction.
    pub fn write_txn(&self) -> Result<RwTxn> {
        self.inner.write_txn()
    }

    /// Create a nested transaction with read and write access for use with the environment.
    ///
    /// The new transaction will be a nested transaction, with the transaction indicated by parent
    /// as its parent. Transactions may be nested to any level.
    ///
    /// A parent transaction and its cursors may not issue any other operations than _commit_ and
    /// _abort_ while it has active child transactions.
    pub fn nested_write_txn<'p>(&'p self, parent: &'p mut RwTxn) -> Result<RwTxn<'p>> {
        self.inner.nested_write_txn(parent)
    }

    /// Create a transaction with read-only access for use with the environment.
    ///
    /// You can make this transaction `Send`able between threads by
    /// using the `read-txn-no-tls` crate feature.
    /// See [`Self::static_read_txn`] if you want the txn to own the environment.
    ///
    /// ## LMDB Limitations
    ///
    /// It's possible to have multiple read transactions in the same environment
    /// while there is a write transaction ongoing.
    ///
    /// But read transactions prevent reuse of pages freed by newer write transactions,
    /// thus the database can grow quickly. Write transactions prevent other write transactions,
    /// since writes are serialized.
    ///
    /// So avoid long-lived read transactions.
    ///
    /// ## Errors
    ///
    /// * [`crate::MdbError::Panic`]: A fatal error occurred earlier, and the environment must be shut down
    /// * [`crate::MdbError::MapResized`]: Another process wrote data beyond this [`Env`] mapsize and this env
    ///   map must be resized
    /// * [`crate::MdbError::ReadersFull`]: a read-only transaction was requested, and the reader lock table is
    ///   full
    pub fn read_txn(&self) -> Result<RoTxn> {
        self.inner.read_txn()
    }

    /// Create a transaction with read-only access for use with the environment.
    /// Contrary to [`Self::read_txn`], this version **owns** the environment, which
    /// means you won't be able to close the environment while this transaction is alive.
    ///
    /// You can make this transaction `Send`able between threads by
    /// using the `read-txn-no-tls` crate feature.
    ///
    /// ## LMDB Limitations
    ///
    /// It's possible to have multiple read transactions in the same environment
    /// while there is a write transaction ongoing.
    ///
    /// But read transactions prevent reuse of pages freed by newer write transactions,
    /// thus the database can grow quickly. Write transactions prevent other write transactions,
    /// since writes are serialized.
    ///
    /// So avoid long-lived read transactions.
    ///
    /// ## Errors
    ///
    /// * [`crate::MdbError::Panic`]: A fatal error occurred earlier, and the environment must be shut down
    /// * [`crate::MdbError::MapResized`]: Another process wrote data beyond this [`Env`] mapsize and this env
    ///   map must be resized
    /// * [`crate::MdbError::ReadersFull`]: a read-only transaction was requested, and the reader lock table is
    ///   full
    pub fn static_read_txn(self) -> Result<RoTxn<'static>> {
        self.inner.static_read_txn()
    }

    /// Copy an LMDB environment to the specified path, with options.
    ///
    /// This function may be used to make a backup of an existing environment.
    /// No lockfile is created, since it gets recreated at need.
    pub fn copy_to_file<P: AsRef<Path>>(&self, path: P, option: CompactionOption) -> Result<File> {
        self.inner.copy_to_file(path, option)
    }

    /// Copy an LMDB environment to the specified file descriptor, with compaction option.
    ///
    /// This function may be used to make a backup of an existing environment.
    /// No lockfile is created, since it gets recreated at need.
    ///
    /// # Safety
    ///
    /// The [`ffi::mdb_filehandle_t`] must have already been opened for Write access.
    pub unsafe fn copy_to_fd(
        &self,
        fd: ffi::mdb_filehandle_t,
        option: CompactionOption,
    ) -> Result<()> {
        self.inner.copy_to_fd(fd, option)
    }

    /// Flush the data buffers to disk.
    pub fn force_sync(&self) -> Result<()> {
        self.inner.force_sync()
    }

    /// Returns the canonicalized path where this env lives.
    pub fn path(&self) -> &Path {
        self.inner.path()
    }

    /// Returns an `EnvClosingEvent` that can be used to wait for the closing event,
    /// multiple threads can wait on this event.
    ///
    /// Make sure that you drop all the copies of `Env`s you have, env closing are triggered
    /// when all references are dropped, the last one will eventually close the environment.
    pub fn prepare_for_closing(self) -> EnvClosingEvent {
        self.inner.prepare_for_closing()
    }

    /// Check for stale entries in the reader lock table and clear them.
    ///
    /// Returns the number of stale readers cleared.
    pub fn clear_stale_readers(&self) -> Result<usize> {
        self.inner.clear_stale_readers()
    }

    /// Resize the memory map to a new size.
    ///
    /// # Safety
    ///
    /// According to the [LMDB documentation](http://www.lmdb.tech/doc/group__mdb.html#gaa2506ec8dab3d969b0e609cd82e619e5),
    /// it is okay to call `mdb_env_set_mapsize` for an open environment as long as no transactions are active,
    /// but the library does not check for this condition, so the caller must ensure it explicitly.
    pub unsafe fn resize(&self, new_size: usize) -> Result<()> {
        self.inner.resize(new_size)
    }

    /// Get the maximum size of keys and MDB_DUPSORT data we can write.
    ///
    /// Depends on the compile-time constant MDB_MAXKEYSIZE. Default 511
    pub fn max_key_size(&self) -> usize {
        self.inner.max_key_size()
    }
}

unsafe impl Send for EncryptedEnv {}

unsafe impl Sync for EncryptedEnv {}

impl fmt::Debug for EncryptedEnv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EncryptedEnv")
            .field("path", &self.inner.path().display())
            .finish_non_exhaustive()
    }
}

fn encrypt<A: AeadMutInPlace + KeyInit>(
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    plaintext: &[u8],
    chipertext_out: &mut [u8],
    auth_out: &mut [u8],
) -> aead::Result<()> {
    chipertext_out.copy_from_slice(plaintext);
    let key: &Key<A> = key.into();
    let nonce: &Nonce<A> = if nonce.len() >= A::NonceSize::USIZE {
        nonce[..A::NonceSize::USIZE].into()
    } else {
        return Err(aead::Error);
    };
    let mut aead = A::new(key);
    let tag = aead.encrypt_in_place_detached(nonce, aad, chipertext_out)?;
    auth_out.copy_from_slice(&tag);
    Ok(())
}

fn decrypt<A: AeadMutInPlace + KeyInit>(
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    chipher_text: &[u8],
    output: &mut [u8],
    auth_in: &[u8],
) -> aead::Result<()> {
    output.copy_from_slice(chipher_text);
    let key: &Key<A> = key.into();
    let nonce: &Nonce<A> = if nonce.len() >= A::NonceSize::USIZE {
        nonce[..A::NonceSize::USIZE].into()
    } else {
        return Err(aead::Error);
    };
    let tag: &Tag<A> = auth_in.into();
    let mut aead = A::new(key);
    aead.decrypt_in_place_detached(nonce, aad, output, tag)
}

/// The wrapper function that is called by LMDB that directly calls
/// the Rust idiomatic function internally.
pub(crate) unsafe extern "C" fn encrypt_func_wrapper<E: AeadMutInPlace + KeyInit>(
    src: *const ffi::MDB_val,
    dst: *mut ffi::MDB_val,
    key_ptr: *const ffi::MDB_val,
    encdec: i32,
) -> i32 {
    let result = catch_unwind(|| {
        let input = std::slice::from_raw_parts((*src).mv_data as *const u8, (*src).mv_size);
        let output = std::slice::from_raw_parts_mut((*dst).mv_data as *mut u8, (*dst).mv_size);
        let key = std::slice::from_raw_parts((*key_ptr).mv_data as *const u8, (*key_ptr).mv_size);
        let iv = std::slice::from_raw_parts(
            (*key_ptr.offset(1)).mv_data as *const u8,
            (*key_ptr.offset(1)).mv_size,
        );
        let auth = std::slice::from_raw_parts_mut(
            (*key_ptr.offset(2)).mv_data as *mut u8,
            (*key_ptr.offset(2)).mv_size,
        );

        let aad = [];
        let nonce = iv;
        let result = if encdec == 1 {
            encrypt::<E>(key, nonce, &aad, input, output, auth)
        } else {
            decrypt::<E>(key, nonce, &aad, input, output, auth)
        };

        result.is_err() as i32
    });

    result.unwrap_or(1)
}
