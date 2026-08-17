use bitflags::bitflags;
#[cfg(master3)]
use lmdb_master3_sys as ffi;
#[cfg(not(master3))]
use lmdb_master_sys as ffi;

#[allow(unused)] // for cargo auto doc links
use crate::{Database, IntegerComparator};

bitflags! {
    /// LMDB environment flags (see <http://www.lmdb.tech/doc/group__mdb__env.html> for more details).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)]
    pub struct EnvFlags: u32 {
        /// mmap at a fixed address (experimental).
        const FIXEDMAP = ffi::MDB_FIXEDMAP;
        /// No environment directory.
        const NO_SUB_DIR = ffi::MDB_NOSUBDIR;
        /// Don't fsync after commit.
        const NO_SYNC = ffi::MDB_NOSYNC;
        /// Read only.
        const READ_ONLY = ffi::MDB_RDONLY;
        /// Don't fsync metapage after commit.
        const NO_META_SYNC = ffi::MDB_NOMETASYNC;
        /// Use writable mmap.
        const WRITE_MAP = ffi::MDB_WRITEMAP;
        /// Use asynchronous msync when MDB_WRITEMAP is used.
        const MAP_ASYNC = ffi::MDB_MAPASYNC;
        /// Tie reader locktable slots to MDB_txn objects instead of to threads.
        const NO_TLS = ffi::MDB_NOTLS;
        /// Don't do any locking, caller must manage their own locks.
        const NO_LOCK = ffi::MDB_NOLOCK;
        /// Don't do readahead (no effect on Windows).
        const NO_READ_AHEAD = ffi::MDB_NORDAHEAD;
        /// Don't initialize malloc'd memory before writing to datafile.
        const NO_MEM_INIT = ffi::MDB_NOMEMINIT;
    }
}

bitflags! {
    /// LMDB database flags (see <http://www.lmdb.tech/doc/group__mdb__dbi__open.html> for more details).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct AllDatabaseFlags: u32 {
        /// Use reverse string keys.
        const REVERSE_KEY = ffi::MDB_REVERSEKEY;
        /// Use sorted duplicates.
        const DUP_SORT = ffi::MDB_DUPSORT;
        /// Numeric keys in native byte order: either `u32` or `usize`.
        /// The keys must all be of the same size.
        ///
        /// It is recommended to set the comparator to [`IntegerComparator`](crate::IntegerComparator),
        /// rather than setting this flag manually.
        const INTEGER_KEY = ffi::MDB_INTEGERKEY;
        /// With [`DatabaseFlags::DUP_SORT`], sorted dup items have fixed size.
        const DUP_FIXED = ffi::MDB_DUPFIXED;
        /// With [`DatabaseFlags::DUP_SORT`], dups are [`DatabaseFlags::INTEGER_KEY`]-style integers.
        const INTEGER_DUP = ffi::MDB_INTEGERDUP;
        /// With [`DatabaseFlags::DUP_SORT`], use reverse string dups.
        const REVERSE_DUP = ffi::MDB_REVERSEDUP;
        /// Create DB if not already existing.
        const CREATE = ffi::MDB_CREATE;
    }
}

bitflags! {
    /// LMDB database flags (see <http://www.lmdb.tech/doc/group__mdb__dbi__open.html> for more details).
    // It is a subset of the whole list of possible flags LMDB exposes but
    // we only want users to be able to specify these with the DUP flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct DatabaseFlags: u32 {
        /// Use reverse string keys.
        ///
        /// ```
        /// # use std::fs;
        /// # use std::path::Path;
        /// # use heed::{DatabaseFlags, EnvOpenOptions};
        /// use heed::types::*;
        /// use heed::byteorder::BigEndian;
        ///
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// # let dir = tempfile::tempdir()?;
        /// # let env = unsafe { EnvOpenOptions::new()
        /// #     .map_size(10 * 1024 * 1024) // 10MB
        /// #     .max_dbs(3000)
        /// #     .open(dir.path())?
        /// # };
        ///
        /// let mut wtxn = env.write_txn()?;
        /// let db = env.database_options()
        ///     .types::<Str, Unit>()
        ///     .flags(DatabaseFlags::REVERSE_KEY)
        ///     .name("reverse-key")
        ///     .create(&mut wtxn)?;
        ///
        /// # db.clear(&mut wtxn)?;
        /// db.put(&mut wtxn, &"bonjour", &())?;
        /// db.put(&mut wtxn, &"hello", &())?;
        /// db.put(&mut wtxn, &"holla", &())?;
        ///
        /// let mut iter = db.iter(&wtxn)?;
        /// assert_eq!(iter.next().transpose()?, Some(("holla", ())));
        /// assert_eq!(iter.next().transpose()?, Some(("hello", ())));
        /// assert_eq!(iter.next().transpose()?, Some(("bonjour", ())));
        /// assert_eq!(iter.next().transpose()?, None);
        /// drop(iter);
        ///
        /// let mut iter = db.rev_iter(&wtxn)?;
        /// assert_eq!(iter.next().transpose()?, Some(("bonjour", ())));
        /// assert_eq!(iter.next().transpose()?, Some(("hello", ())));
        /// assert_eq!(iter.next().transpose()?, Some(("holla", ())));
        /// assert_eq!(iter.next().transpose()?, None);
        /// drop(iter);
        ///
        /// wtxn.commit()?;
        /// # Ok(()) }
        /// ```
        const REVERSE_KEY = ffi::MDB_REVERSEKEY;
        /// Use sorted duplicates.
        ///
        /// ```
        /// # use std::fs;
        /// # use std::path::Path;
        /// # use heed::{DatabaseFlags, EnvOpenOptions};
        /// use heed::types::*;
        /// use heed::byteorder::BigEndian;
        ///
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// # let dir = tempfile::tempdir()?;
        /// # let env = unsafe { EnvOpenOptions::new()
        /// #     .map_size(10 * 1024 * 1024) // 10MB
        /// #     .max_dbs(3000)
        /// #     .open(dir.path())?
        /// # };
        /// type BEI64 = I64<BigEndian>;
        ///
        /// let mut wtxn = env.write_txn()?;
        /// let db = env.database_options()
        ///     .types::<BEI64, BEI64>()
        ///     .flags(DatabaseFlags::DUP_SORT)
        ///     .name("dup-sort")
        ///     .create(&mut wtxn)?;
        ///
        /// # db.clear(&mut wtxn)?;
        /// db.put(&mut wtxn, &68, &120)?;
        /// db.put(&mut wtxn, &68, &121)?;
        /// db.put(&mut wtxn, &68, &122)?;
        /// db.put(&mut wtxn, &68, &123)?;
        /// db.put(&mut wtxn, &92, &32)?;
        /// db.put(&mut wtxn, &35, &120)?;
        /// db.put(&mut wtxn, &0, &120)?;
        /// db.put(&mut wtxn, &42, &120)?;
        ///
        /// let mut iter = db.get_duplicates(&wtxn, &68)?.expect("the key exists");
        /// assert_eq!(iter.next().transpose()?, Some((68, 120)));
        /// assert_eq!(iter.next().transpose()?, Some((68, 121)));
        /// assert_eq!(iter.next().transpose()?, Some((68, 122)));
        /// assert_eq!(iter.next().transpose()?, Some((68, 123)));
        /// assert_eq!(iter.next().transpose()?, None);
        /// drop(iter);
        ///
        /// let mut iter = db.get_duplicates(&wtxn, &68)?.expect("the key exists");
        /// assert_eq!(iter.last().transpose()?, Some((68, 123)));
        ///
        /// assert!(db.delete_one_duplicate(&mut wtxn, &68, &121)?, "The entry must exist");
        ///
        /// let mut iter = db.get_duplicates(&wtxn, &68)?.expect("the key exists");
        /// assert_eq!(iter.next().transpose()?, Some((68, 120)));
        /// // No more (68, 121) returned here!
        /// assert_eq!(iter.next().transpose()?, Some((68, 122)));
        /// assert_eq!(iter.next().transpose()?, Some((68, 123)));
        /// assert_eq!(iter.next().transpose()?, None);
        /// drop(iter);
        ///
        /// wtxn.commit()?;
        /// # Ok(()) }
        /// ```
        const DUP_SORT = ffi::MDB_DUPSORT;
        /// Numeric keys in native byte order: either `u32` or `usize`.
        /// The keys must all be of the same size.
        ///
        /// It is recommended to use the [`IntegerComparator`] when
        /// opening the [`Database`] instead. This comparator provides
        /// better support for ranges and does not allow for prefix
        /// iteration, as it is not applicable in this context.
        ///
        /// ```
        /// # use std::fs;
        /// # use std::path::Path;
        /// # use heed::{DatabaseFlags, EnvOpenOptions};
        /// use heed::types::*;
        /// use heed::byteorder::BigEndian;
        ///
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// # let dir = tempfile::tempdir()?;
        /// # let env = unsafe { EnvOpenOptions::new()
        /// #     .map_size(10 * 1024 * 1024) // 10MB
        /// #     .max_dbs(3000)
        /// #     .open(dir.path())?
        /// # };
        /// type BEI32 = I32<BigEndian>;
        ///
        /// let mut wtxn = env.write_txn()?;
        /// let db = env.database_options()
        ///     .types::<BEI32, BEI32>()
        ///     .flags(DatabaseFlags::INTEGER_KEY)
        ///     .name("integer-key")
        ///     .create(&mut wtxn)?;
        ///
        /// # db.clear(&mut wtxn)?;
        /// db.put(&mut wtxn, &68, &120)?;
        /// db.put(&mut wtxn, &92, &32)?;
        /// db.put(&mut wtxn, &35, &120)?;
        /// db.put(&mut wtxn, &0, &120)?;
        /// db.put(&mut wtxn, &42, &120)?;
        ///
        /// let mut iter = db.iter(&wtxn)?;
        /// assert_eq!(iter.next().transpose()?, Some((0, 120)));
        /// assert_eq!(iter.next().transpose()?, Some((35, 120)));
        /// assert_eq!(iter.next().transpose()?, Some((42, 120)));
        /// assert_eq!(iter.next().transpose()?, Some((68, 120)));
        /// assert_eq!(iter.next().transpose()?, Some((92, 32)));
        /// assert_eq!(iter.next().transpose()?, None);
        /// drop(iter);
        ///
        /// wtxn.commit()?;
        /// # Ok(()) }
        /// ```
        #[deprecated(since="0.21.0", note="please use `IntegerComparator` instead")]
        const INTEGER_KEY = ffi::MDB_INTEGERKEY;
        /// With [`DatabaseFlags::DUP_SORT`], sorted dup items have fixed size.
        ///
        /// ```
        /// # use std::fs;
        /// # use std::path::Path;
        /// # use heed::{DatabaseFlags, EnvOpenOptions};
        /// use heed::types::*;
        /// use heed::byteorder::BigEndian;
        ///
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// # let dir = tempfile::tempdir()?;
        /// # let env = unsafe { EnvOpenOptions::new()
        /// #     .map_size(10 * 1024 * 1024) // 10MB
        /// #     .max_dbs(3000)
        /// #     .open(dir.path())?
        /// # };
        /// type BEI64 = I64<BigEndian>;
        ///
        /// let mut wtxn = env.write_txn()?;
        /// let db = env.database_options()
        ///     .types::<BEI64, BEI64>()
        ///     .flags(DatabaseFlags::DUP_SORT | DatabaseFlags::DUP_FIXED)
        ///     .name("dup-sort-fixed")
        ///     .create(&mut wtxn)?;
        ///
        /// # db.clear(&mut wtxn)?;
        /// db.put(&mut wtxn, &68, &120)?;
        /// db.put(&mut wtxn, &68, &121)?;
        /// db.put(&mut wtxn, &68, &122)?;
        /// db.put(&mut wtxn, &68, &123)?;
        /// db.put(&mut wtxn, &92, &32)?;
        /// db.put(&mut wtxn, &35, &120)?;
        /// db.put(&mut wtxn, &0, &120)?;
        /// db.put(&mut wtxn, &42, &120)?;
        ///
        /// let mut iter = db.get_duplicates(&wtxn, &68)?.expect("the key exists");
        /// assert_eq!(iter.next().transpose()?, Some((68, 120)));
        /// assert_eq!(iter.next().transpose()?, Some((68, 121)));
        /// assert_eq!(iter.next().transpose()?, Some((68, 122)));
        /// assert_eq!(iter.next().transpose()?, Some((68, 123)));
        /// assert_eq!(iter.next().transpose()?, None);
        /// drop(iter);
        ///
        /// let mut iter = db.get_duplicates(&wtxn, &68)?.expect("the key exists");
        /// assert_eq!(iter.last().transpose()?, Some((68, 123)));
        ///
        /// assert!(db.delete_one_duplicate(&mut wtxn, &68, &121)?, "The entry must exist");
        ///
        /// let mut iter = db.get_duplicates(&wtxn, &68)?.expect("the key exists");
        /// assert_eq!(iter.next().transpose()?, Some((68, 120)));
        /// // No more (68, 121) returned here!
        /// assert_eq!(iter.next().transpose()?, Some((68, 122)));
        /// assert_eq!(iter.next().transpose()?, Some((68, 123)));
        /// assert_eq!(iter.next().transpose()?, None);
        /// drop(iter);
        ///
        /// wtxn.commit()?;
        /// # Ok(()) }
        /// ```
        const DUP_FIXED = ffi::MDB_DUPFIXED;
        /// With [`DatabaseFlags::DUP_SORT`], dups are [`DatabaseFlags::INTEGER_KEY`]-style integers.
        ///
        /// ```
        /// # use std::fs;
        /// # use std::path::Path;
        /// # use heed::{DatabaseFlags, EnvOpenOptions};
        /// use heed::types::*;
        /// use heed::byteorder::BigEndian;
        ///
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// # let dir = tempfile::tempdir()?;
        /// # let env = unsafe { EnvOpenOptions::new()
        /// #     .map_size(10 * 1024 * 1024) // 10MB
        /// #     .max_dbs(3000)
        /// #     .open(dir.path())?
        /// # };
        /// type BEI32 = I32<BigEndian>;
        ///
        /// let mut wtxn = env.write_txn()?;
        /// let db = env.database_options()
        ///     .types::<BEI32, BEI32>()
        ///     .flags(DatabaseFlags::DUP_SORT | DatabaseFlags::INTEGER_DUP)
        ///     .name("dup-sort-integer-dup")
        ///     .create(&mut wtxn)?;
        ///
        /// # db.clear(&mut wtxn)?;
        /// db.put(&mut wtxn, &68, &120)?;
        /// db.put(&mut wtxn, &68, &121)?;
        /// db.put(&mut wtxn, &68, &122)?;
        /// db.put(&mut wtxn, &68, &123)?;
        /// db.put(&mut wtxn, &92, &32)?;
        /// db.put(&mut wtxn, &35, &120)?;
        /// db.put(&mut wtxn, &0, &120)?;
        /// db.put(&mut wtxn, &42, &120)?;
        ///
        /// let mut iter = db.get_duplicates(&wtxn, &68)?.expect("the key exists");
        /// assert_eq!(iter.next().transpose()?, Some((68, 120)));
        /// assert_eq!(iter.next().transpose()?, Some((68, 121)));
        /// assert_eq!(iter.next().transpose()?, Some((68, 122)));
        /// assert_eq!(iter.next().transpose()?, Some((68, 123)));
        /// assert_eq!(iter.next().transpose()?, None);
        /// drop(iter);
        ///
        /// let mut iter = db.get_duplicates(&wtxn, &68)?.expect("the key exists");
        /// assert_eq!(iter.last().transpose()?, Some((68, 123)));
        ///
        /// assert!(db.delete_one_duplicate(&mut wtxn, &68, &121)?, "The entry must exist");
        ///
        /// let mut iter = db.get_duplicates(&wtxn, &68)?.expect("the key exists");
        /// assert_eq!(iter.next().transpose()?, Some((68, 120)));
        /// // No more (68, 121) returned here!
        /// assert_eq!(iter.next().transpose()?, Some((68, 122)));
        /// assert_eq!(iter.next().transpose()?, Some((68, 123)));
        /// assert_eq!(iter.next().transpose()?, None);
        /// drop(iter);
        ///
        /// wtxn.commit()?;
        /// # Ok(()) }
        /// ```
        const INTEGER_DUP = ffi::MDB_INTEGERDUP;
        /// With [`DatabaseFlags::DUP_SORT`], use reverse string dups.
        ///
        /// ```
        /// # use std::fs;
        /// # use std::path::Path;
        /// # use heed::{DatabaseFlags, EnvOpenOptions};
        /// use heed::types::*;
        /// use heed::byteorder::BigEndian;
        ///
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// # let dir = tempfile::tempdir()?;
        /// # let env = unsafe { EnvOpenOptions::new()
        /// #     .map_size(10 * 1024 * 1024) // 10MB
        /// #     .max_dbs(3000)
        /// #     .open(dir.path())?
        /// # };
        /// type BEI64 = I64<BigEndian>;
        ///
        /// let mut wtxn = env.write_txn()?;
        /// let db = env.database_options()
        ///     .types::<BEI64, Str>()
        ///     .flags(DatabaseFlags::DUP_SORT | DatabaseFlags::REVERSE_DUP)
        ///     .name("dup-sort")
        ///     .create(&mut wtxn)?;
        ///
        /// # db.clear(&mut wtxn)?;
        /// db.put(&mut wtxn, &68, &"bonjour")?;
        /// db.put(&mut wtxn, &68, &"holla")?;
        /// db.put(&mut wtxn, &68, &"hello")?;
        /// db.put(&mut wtxn, &92, &"hallo")?;
        ///
        /// let mut iter = db.get_duplicates(&wtxn, &68)?.expect("the key exists");
        /// assert_eq!(iter.next().transpose()?, Some((68, "holla")));
        /// assert_eq!(iter.next().transpose()?, Some((68, "hello")));
        /// assert_eq!(iter.next().transpose()?, Some((68, "bonjour")));
        /// assert_eq!(iter.next().transpose()?, None);
        /// drop(iter);
        ///
        /// wtxn.commit()?;
        /// # Ok(()) }
        /// ```
        const REVERSE_DUP = ffi::MDB_REVERSEDUP;
    }
}

bitflags! {
    /// LMDB put flags (see <http://www.lmdb.tech/doc/group__mdb.html#ga4fa8573d9236d54687c61827ebf8cac0>
    /// or <http://www.lmdb.tech/doc/group__mdb.html#ga1f83ccb40011837ff37cc32be01ad91e> for more details).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct PutFlags: u32 {
        /// Enter the new key/data pair only if it does not already appear in the database.
        ///
        /// This flag may only be specified if the database was opened with MDB_DUPSORT.
        /// The function will return MDB_KEYEXIST if the key/data pair already appears in the database.
        const NO_DUP_DATA = ffi::MDB_NODUPDATA;
        /// Enter the new key/data pair only if the key does not already appear in the database.
        ///
        /// The function will return MDB_KEYEXIST if the key already appears in the database,
        /// even if the database supports duplicates (MDB_DUPSORT).
        /// The data parameter will be set to point to the existing item.
        const NO_OVERWRITE = ffi::MDB_NOOVERWRITE;
        /// Append the given key/data pair to the end of the database.
        ///
        /// This option allows fast bulk loading when keys are already known to be in the correct order.
        /// Loading unsorted keys with this flag will cause a MDB_KEYEXIST error.
        const APPEND = ffi::MDB_APPEND;
        /// Append the given key/data pair to the end of the database but for sorted dup data.
        ///
        /// This option allows fast bulk loading when keys and dup data are already known to be in the correct order.
        /// Loading unsorted key/values with this flag will cause a MDB_KEYEXIST error.
        const APPEND_DUP = ffi::MDB_APPENDDUP;
    }
}
