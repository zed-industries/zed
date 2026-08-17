use std::any::TypeId;
use std::ffi::CString;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::ptr::{self, NonNull};
use std::sync::Arc;
use std::{fmt, io, mem};

use heed_traits::Comparator;
use synchronoise::SignalEvent;

use super::{
    custom_key_cmp_wrapper, get_file_fd, metadata_from_fd, DefaultComparator, EnvClosingEvent,
    EnvInfo, FlagSetMode, IntegerComparator, OPENED_ENV,
};
use crate::cursor::{MoveOperation, RoCursor};
use crate::mdb::ffi::{self, MDB_env};
use crate::mdb::lmdb_error::mdb_result;
use crate::mdb::lmdb_flags::AllDatabaseFlags;
#[allow(unused)] // for cargo auto doc links
use crate::EnvOpenOptions;
use crate::{
    CompactionOption, Database, DatabaseOpenOptions, EnvFlags, Error, Result, RoTxn, RwTxn,
    Unspecified,
};

/// An environment handle constructed by using [`EnvOpenOptions::open`].
#[derive(Clone)]
pub struct Env {
    inner: Arc<EnvInner>,
}

impl Env {
    pub(crate) fn new(
        env_ptr: NonNull<MDB_env>,
        path: PathBuf,
        signal_event: Arc<SignalEvent>,
    ) -> Env {
        Env { inner: Arc::new(EnvInner { env_ptr, path, signal_event }) }
    }

    pub(crate) fn env_mut_ptr(&self) -> NonNull<ffi::MDB_env> {
        self.inner.env_ptr
    }

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
        let mut fd = mem::MaybeUninit::uninit();
        unsafe { mdb_result(ffi::mdb_env_get_fd(self.env_mut_ptr().as_mut(), fd.as_mut_ptr()))? };
        let fd = unsafe { fd.assume_init() };
        let metadata = unsafe { metadata_from_fd(fd)? };
        Ok(metadata.len())
    }

    /// Return the raw flags the environment was opened with.
    ///
    /// Returns `None` if the environment flags are different from the [`EnvFlags`] set.
    pub fn flags(&self) -> Result<Option<EnvFlags>> {
        self.get_flags().map(EnvFlags::from_bits)
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
        // safety: caller must ensure no other thread is calling this function.
        // <http://www.lmdb.tech/doc/group__mdb.html#ga83f66cf02bfd42119451e9468dc58445>
        mdb_result(unsafe {
            ffi::mdb_env_set_flags(
                self.env_mut_ptr().as_mut(),
                flags.bits(),
                mode.as_mdb_env_set_flags_input(),
            )
        })
        .map_err(Into::into)
    }

    /// Return the raw flags the environment is currently set with.
    pub fn get_flags(&self) -> Result<u32> {
        let mut flags = mem::MaybeUninit::uninit();
        unsafe {
            mdb_result(ffi::mdb_env_get_flags(self.env_mut_ptr().as_mut(), flags.as_mut_ptr()))?
        };
        let flags = unsafe { flags.assume_init() };
        Ok(flags)
    }

    /// Returns some basic informations about this environment.
    pub fn info(&self) -> EnvInfo {
        let mut raw_info = mem::MaybeUninit::uninit();
        unsafe { ffi::mdb_env_info(self.inner.env_ptr.as_ptr(), raw_info.as_mut_ptr()) };
        let raw_info = unsafe { raw_info.assume_init() };

        EnvInfo {
            map_addr: raw_info.me_mapaddr,
            map_size: raw_info.me_mapsize,
            last_page_number: raw_info.me_last_pgno,
            last_txn_id: raw_info.me_last_txnid,
            maximum_number_of_readers: raw_info.me_maxreaders,
            number_of_readers: raw_info.me_numreaders,
        }
    }

    /// Returns the size used by all the databases in the environment without the free pages.
    ///
    /// It is crucial to configure [`EnvOpenOptions::max_dbs`] with a sufficiently large value
    /// before invoking this function. All databases within the environment will be opened
    /// and remain so.
    pub fn non_free_pages_size(&self) -> Result<u64> {
        let compute_size = |stat: ffi::MDB_stat| {
            (stat.ms_leaf_pages + stat.ms_branch_pages + stat.ms_overflow_pages) as u64
                * stat.ms_psize as u64
        };

        let mut size = 0;

        let mut stat = mem::MaybeUninit::uninit();
        unsafe { mdb_result(ffi::mdb_env_stat(self.env_mut_ptr().as_mut(), stat.as_mut_ptr()))? };
        let stat = unsafe { stat.assume_init() };
        size += compute_size(stat);

        let rtxn = self.read_txn()?;
        // Open the main database
        let dbi = self.raw_open_dbi::<DefaultComparator>(rtxn.txn.unwrap(), None, 0)?;

        // We're going to iterate on the unnamed database
        let mut cursor = RoCursor::new(&rtxn, dbi)?;

        while let Some((key, _value)) = cursor.move_on_next(MoveOperation::NoDup)? {
            if key.contains(&0) {
                continue;
            }

            let key = String::from_utf8(key.to_vec()).unwrap();
            // Calling `ffi::db_stat` on a database instance does not involve key comparison
            // in LMDB, so it's safe to specify a noop key compare function for it.
            if let Ok(dbi) =
                self.raw_open_dbi::<DefaultComparator>(rtxn.txn.unwrap(), Some(&key), 0)
            {
                let mut stat = mem::MaybeUninit::uninit();
                unsafe {
                    mdb_result(ffi::mdb_stat(rtxn.txn.unwrap().as_mut(), dbi, stat.as_mut_ptr()))?
                };
                let stat = unsafe { stat.assume_init() };
                size += compute_size(stat);
            }
        }

        Ok(size)
    }

    /// Options and flags which can be used to configure how a [`Database`] is opened.
    pub fn database_options(&self) -> DatabaseOpenOptions<Unspecified, Unspecified> {
        DatabaseOpenOptions::new(self)
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
    ) -> Result<Option<Database<KC, DC>>>
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
    ) -> Result<Database<KC, DC>>
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

    pub(crate) fn raw_init_database<C: Comparator + 'static>(
        &self,
        raw_txn: NonNull<ffi::MDB_txn>,
        name: Option<&str>,
        mut flags: AllDatabaseFlags,
    ) -> Result<u32> {
        if TypeId::of::<C>() == TypeId::of::<IntegerComparator>() {
            flags.insert(AllDatabaseFlags::INTEGER_KEY);
        }

        match self.raw_open_dbi::<C>(raw_txn, name, flags.bits()) {
            Ok(dbi) => Ok(dbi),
            Err(e) => Err(e.into()),
        }
    }

    fn raw_open_dbi<C: Comparator + 'static>(
        &self,
        mut raw_txn: NonNull<ffi::MDB_txn>,
        name: Option<&str>,
        flags: u32,
    ) -> std::result::Result<u32, crate::mdb::lmdb_error::Error> {
        let mut dbi = 0;
        let name = name.map(|n| CString::new(n).unwrap());
        let name_ptr = match name {
            Some(ref name) => name.as_bytes_with_nul().as_ptr() as *const _,
            None => ptr::null(),
        };

        // safety: The name cstring is cloned by LMDB, we can drop it after.
        //         If a read-only is used with the MDB_CREATE flag, LMDB will throw an error.
        unsafe {
            mdb_result(ffi::mdb_dbi_open(raw_txn.as_mut(), name_ptr, flags, &mut dbi))?;
            let cmp_type_id = TypeId::of::<C>();

            if cmp_type_id != TypeId::of::<DefaultComparator>()
                && cmp_type_id != TypeId::of::<IntegerComparator>()
            {
                mdb_result(ffi::mdb_set_compare(
                    raw_txn.as_mut(),
                    dbi,
                    Some(custom_key_cmp_wrapper::<C>),
                ))?;
            }
        };

        Ok(dbi)
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
        RwTxn::new(self)
    }

    /// Create a nested transaction with read and write access for use with the environment.
    ///
    /// The new transaction will be a nested transaction, with the transaction indicated by parent
    /// as its parent. Transactions may be nested to any level.
    ///
    /// A parent transaction and its cursors may not issue any other operations than _commit_ and
    /// _abort_ while it has active child transactions.
    pub fn nested_write_txn<'p>(&'p self, parent: &'p mut RwTxn) -> Result<RwTxn<'p>> {
        RwTxn::nested(self, parent)
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
        RoTxn::new(self)
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
        RoTxn::static_read_txn(self)
    }

    /// Copy an LMDB environment to the specified path, with options.
    ///
    /// This function may be used to make a backup of an existing environment.
    /// No lockfile is created, since it gets recreated at need.
    pub fn copy_to_file<P: AsRef<Path>>(&self, path: P, option: CompactionOption) -> Result<File> {
        let file = File::options().create_new(true).write(true).open(&path)?;
        let fd = get_file_fd(&file);

        unsafe { self.copy_to_fd(fd, option)? };

        // We reopen the file to make sure the cursor is at the start,
        // even a seek to start doesn't work properly.
        let file = File::open(path)?;

        Ok(file)
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
        let flags = if let CompactionOption::Enabled = option { ffi::MDB_CP_COMPACT } else { 0 };
        mdb_result(ffi::mdb_env_copyfd2(self.inner.env_ptr.as_ptr(), fd, flags))?;
        Ok(())
    }

    /// Flush the data buffers to disk.
    pub fn force_sync(&self) -> Result<()> {
        unsafe { mdb_result(ffi::mdb_env_sync(self.inner.env_ptr.as_ptr(), 1))? }
        Ok(())
    }

    /// Returns the canonicalized path where this env lives.
    pub fn path(&self) -> &Path {
        &self.inner.path
    }

    /// Returns an `EnvClosingEvent` that can be used to wait for the closing event,
    /// multiple threads can wait on this event.
    ///
    /// Make sure that you drop all the copies of `Env`s you have, env closing are triggered
    /// when all references are dropped, the last one will eventually close the environment.
    pub fn prepare_for_closing(self) -> EnvClosingEvent {
        EnvClosingEvent(self.inner.signal_event.clone())
    }

    /// Check for stale entries in the reader lock table and clear them.
    ///
    /// Returns the number of stale readers cleared.
    pub fn clear_stale_readers(&self) -> Result<usize> {
        let mut dead: i32 = 0;
        unsafe { mdb_result(ffi::mdb_reader_check(self.inner.env_ptr.as_ptr(), &mut dead))? }
        // safety: The reader_check function asks for an i32, initialize it to zero
        //         and never decrements it. It is safe to use either an u32 or u64 (usize).
        Ok(dead as usize)
    }

    /// Resize the memory map to a new size.
    ///
    /// # Safety
    ///
    /// According to the [LMDB documentation](http://www.lmdb.tech/doc/group__mdb.html#gaa2506ec8dab3d969b0e609cd82e619e5),
    /// it is okay to call `mdb_env_set_mapsize` for an open environment as long as no transactions are active,
    /// but the library does not check for this condition, so the caller must ensure it explicitly.
    pub unsafe fn resize(&self, new_size: usize) -> Result<()> {
        if new_size % page_size::get() != 0 {
            let msg = format!(
                "map size ({}) must be a multiple of the system page size ({})",
                new_size,
                page_size::get()
            );
            return Err(Error::Io(io::Error::new(io::ErrorKind::InvalidInput, msg)));
        }
        mdb_result(unsafe { ffi::mdb_env_set_mapsize(self.env_mut_ptr().as_mut(), new_size) })
            .map_err(Into::into)
    }

    /// Get the maximum size of keys and MDB_DUPSORT data we can write.
    ///
    /// Depends on the compile-time constant MDB_MAXKEYSIZE. Default 511
    pub fn max_key_size(&self) -> usize {
        let maxsize: i32 = unsafe { ffi::mdb_env_get_maxkeysize(self.env_mut_ptr().as_mut()) };
        maxsize as usize
    }
}

unsafe impl Send for Env {}

unsafe impl Sync for Env {}

impl fmt::Debug for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Env").field("path", &self.inner.path.display()).finish_non_exhaustive()
    }
}

pub(crate) struct EnvInner {
    env_ptr: NonNull<MDB_env>,
    signal_event: Arc<SignalEvent>,
    pub(crate) path: PathBuf,
}

unsafe impl Send for EnvInner {}
unsafe impl Sync for EnvInner {}

impl Drop for EnvInner {
    fn drop(&mut self) {
        let mut lock = OPENED_ENV.write().unwrap();
        let removed = lock.remove(&self.path);
        debug_assert!(removed.is_some());
        unsafe { ffi::mdb_env_close(self.env_ptr.as_mut()) };
        self.signal_event.signal();
    }
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    use std::time::Duration;
    use std::{fs, thread};

    use crate::types::*;
    use crate::{env_closing_event, EnvOpenOptions, Error};

    #[test]
    fn close_env() {
        let dir = tempfile::tempdir().unwrap();
        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(10 * 1024 * 1024) // 10MB
                .max_dbs(30)
                .open(dir.path())
                .unwrap()
        };

        // Force a thread to keep the env for 1 second.
        let env_cloned = env.clone();
        thread::spawn(move || {
            let _env = env_cloned;
            thread::sleep(Duration::from_secs(1));
        });

        let mut wtxn = env.write_txn().unwrap();
        let db = env.create_database::<Str, Str>(&mut wtxn, None).unwrap();
        wtxn.commit().unwrap();

        // Create an ordered list of keys...
        let mut wtxn = env.write_txn().unwrap();
        db.put(&mut wtxn, "hello", "hello").unwrap();
        db.put(&mut wtxn, "world", "world").unwrap();

        let mut iter = db.iter(&wtxn).unwrap();
        assert_eq!(iter.next().transpose().unwrap(), Some(("hello", "hello")));
        assert_eq!(iter.next().transpose().unwrap(), Some(("world", "world")));
        assert_eq!(iter.next().transpose().unwrap(), None);
        drop(iter);

        wtxn.commit().unwrap();

        let signal_event = env.prepare_for_closing();

        eprintln!("waiting for the env to be closed");
        signal_event.wait();
        eprintln!("env closed successfully");

        // Make sure we don't have a reference to the env
        assert!(env_closing_event(dir.path()).is_none());
    }

    #[test]
    fn reopen_env_with_different_options_is_err() {
        let dir = tempfile::tempdir().unwrap();
        let _env = unsafe {
            EnvOpenOptions::new()
                .map_size(10 * 1024 * 1024) // 10MB
                .open(dir.path())
                .unwrap()
        };

        let result = unsafe {
            EnvOpenOptions::new()
                .map_size(12 * 1024 * 1024) // 12MB
                .open(dir.path())
        };

        assert!(matches!(result, Err(Error::EnvAlreadyOpened)));
    }

    #[test]
    fn open_env_with_named_path() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("babar.mdb")).unwrap();
        let _env = unsafe {
            EnvOpenOptions::new()
                .map_size(10 * 1024 * 1024) // 10MB
                .open(dir.path().join("babar.mdb"))
                .unwrap()
        };

        let error = unsafe {
            EnvOpenOptions::new()
                .map_size(10 * 1024 * 1024) // 10MB
                .open(dir.path().join("babar.mdb"))
                .unwrap_err()
        };

        assert!(matches!(error, Error::EnvAlreadyOpened));
    }

    #[test]
    #[cfg(not(windows))]
    fn open_database_with_writemap_flag() {
        let dir = tempfile::tempdir().unwrap();
        let mut envbuilder = EnvOpenOptions::new();
        envbuilder.map_size(10 * 1024 * 1024); // 10MB
        envbuilder.max_dbs(10);
        unsafe { envbuilder.flags(crate::EnvFlags::WRITE_MAP) };
        let env = unsafe { envbuilder.open(dir.path()).unwrap() };

        let mut wtxn = env.write_txn().unwrap();
        let _db = env.create_database::<Str, Str>(&mut wtxn, Some("my-super-db")).unwrap();
        wtxn.commit().unwrap();
    }

    #[test]
    fn open_database_with_nosubdir() {
        let dir = tempfile::tempdir().unwrap();
        let mut envbuilder = EnvOpenOptions::new();
        unsafe { envbuilder.flags(crate::EnvFlags::NO_SUB_DIR) };
        let _env = unsafe { envbuilder.open(dir.path().join("data.mdb")).unwrap() };
    }

    #[test]
    fn create_database_without_commit() {
        let dir = tempfile::tempdir().unwrap();
        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(10 * 1024 * 1024) // 10MB
                .max_dbs(10)
                .open(dir.path())
                .unwrap()
        };

        let mut wtxn = env.write_txn().unwrap();
        let _db = env.create_database::<Str, Str>(&mut wtxn, Some("my-super-db")).unwrap();
        wtxn.abort();

        let rtxn = env.read_txn().unwrap();
        let option = env.open_database::<Str, Str>(&rtxn, Some("my-super-db")).unwrap();
        assert!(option.is_none());
    }

    #[test]
    fn open_already_existing_database() {
        let dir = tempfile::tempdir().unwrap();
        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(10 * 1024 * 1024) // 10MB
                .max_dbs(10)
                .open(dir.path())
                .unwrap()
        };

        // we first create a database
        let mut wtxn = env.write_txn().unwrap();
        let _db = env.create_database::<Str, Str>(&mut wtxn, Some("my-super-db")).unwrap();
        wtxn.commit().unwrap();

        // Close the environement and reopen it, databases must not be loaded in memory.
        env.prepare_for_closing().wait();
        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(10 * 1024 * 1024) // 10MB
                .max_dbs(10)
                .open(dir.path())
                .unwrap()
        };

        let rtxn = env.read_txn().unwrap();
        let option = env.open_database::<Str, Str>(&rtxn, Some("my-super-db")).unwrap();
        assert!(option.is_some());
    }

    #[test]
    fn resize_database() {
        let dir = tempfile::tempdir().unwrap();
        let page_size = page_size::get();
        let env = unsafe {
            EnvOpenOptions::new().map_size(9 * page_size).max_dbs(1).open(dir.path()).unwrap()
        };

        let mut wtxn = env.write_txn().unwrap();
        let db = env.create_database::<Str, Str>(&mut wtxn, Some("my-super-db")).unwrap();
        wtxn.commit().unwrap();

        let mut wtxn = env.write_txn().unwrap();
        for i in 0..64 {
            db.put(&mut wtxn, &i.to_string(), "world").unwrap();
        }
        wtxn.commit().unwrap();

        let mut wtxn = env.write_txn().unwrap();
        for i in 64..128 {
            db.put(&mut wtxn, &i.to_string(), "world").unwrap();
        }
        wtxn.commit().expect_err("cannot commit a transaction that would reach the map size limit");

        unsafe {
            env.resize(10 * page_size).unwrap();
        }
        let mut wtxn = env.write_txn().unwrap();
        for i in 64..128 {
            db.put(&mut wtxn, &i.to_string(), "world").unwrap();
        }
        wtxn.commit().expect("transaction should commit after resizing the map size");

        assert_eq!(10 * page_size, env.info().map_size);
    }

    /// Non-regression test for
    /// <https://github.com/meilisearch/heed/issues/183>
    ///
    /// We should be able to open database Read-Only Env with
    /// no prior Read-Write Env opening. And query data.
    #[test]
    fn open_read_only_without_no_env_opened_before() {
        let expected_data0 = "Data Expected db0";
        let dir = tempfile::tempdir().unwrap();

        {
            // We really need this env to be dropped before the read-only access.
            let env = unsafe {
                EnvOpenOptions::new()
                    .map_size(16 * 1024 * 1024 * 1024) // 10MB
                    .max_dbs(32)
                    .open(dir.path())
                    .unwrap()
            };
            let mut wtxn = env.write_txn().unwrap();
            let database0 = env.create_database::<Str, Str>(&mut wtxn, Some("shared0")).unwrap();

            wtxn.commit().unwrap();
            let mut wtxn = env.write_txn().unwrap();
            database0.put(&mut wtxn, "shared0", expected_data0).unwrap();
            wtxn.commit().unwrap();
            // We also really need that no other env reside in memory in other thread doing tests.
            env.prepare_for_closing().wait();
        }

        {
            // Open now we do a read-only opening
            let env = unsafe {
                EnvOpenOptions::new()
                    .map_size(16 * 1024 * 1024 * 1024) // 10MB
                    .max_dbs(32)
                    .open(dir.path())
                    .unwrap()
            };
            let database0 = {
                let rtxn = env.read_txn().unwrap();
                let database0 =
                    env.open_database::<Str, Str>(&rtxn, Some("shared0")).unwrap().unwrap();
                // This commit is mandatory if not committed you might get
                // Io(Os { code: 22, kind: InvalidInput, message: "Invalid argument" })
                rtxn.commit().unwrap();
                database0
            };

            {
                // If we didn't committed the opening it might fail with EINVAL.
                let rtxn = env.read_txn().unwrap();
                let value = database0.get(&rtxn, "shared0").unwrap().unwrap();
                assert_eq!(value, expected_data0);
            }

            env.prepare_for_closing().wait();
        }

        // To avoid reintroducing the bug let's try to open again but without the commit
        {
            // Open now we do a read-only opening
            let env = unsafe {
                EnvOpenOptions::new()
                    .map_size(16 * 1024 * 1024 * 1024) // 10MB
                    .max_dbs(32)
                    .open(dir.path())
                    .unwrap()
            };
            let database0 = {
                let rtxn = env.read_txn().unwrap();
                let database0 =
                    env.open_database::<Str, Str>(&rtxn, Some("shared0")).unwrap().unwrap();
                // No commit it's important, dropping explicitly
                drop(rtxn);
                database0
            };

            {
                // We didn't committed the opening we will get EINVAL.
                let rtxn = env.read_txn().unwrap();
                // The dbg!() is intentional in case of a change in rust-std or in lmdb related
                // to the windows error.
                let err = dbg!(database0.get(&rtxn, "shared0"));

                // The error kind is still ErrorKind Uncategorized on windows.
                // Behind it's a ERROR_BAD_COMMAND code 22 like EINVAL.
                if cfg!(windows) {
                    assert!(err.is_err());
                } else {
                    assert!(
                        matches!(err, Err(Error::Io(ref e)) if e.kind() == ErrorKind::InvalidInput)
                    );
                }
            }

            env.prepare_for_closing().wait();
        }
    }

    #[test]
    fn max_key_size() {
        let dir = tempfile::tempdir().unwrap();
        let env = unsafe { EnvOpenOptions::new().open(dir.path().join(dir.path())).unwrap() };
        let maxkeysize = env.max_key_size();

        eprintln!("maxkeysize: {}", maxkeysize);

        if cfg!(feature = "longer-keys") {
            // Should be larger than the default of 511
            assert!(maxkeysize > 511);
        } else {
            // Should be the default of 511
            assert_eq!(maxkeysize, 511);
        }
    }
}
