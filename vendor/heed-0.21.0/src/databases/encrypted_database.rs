use std::ops::RangeBounds;
use std::{any, fmt};

use heed_traits::{Comparator, LexicographicComparator};
use types::LazyDecode;

use crate::envs::DefaultComparator;
use crate::iteration_method::MoveOnCurrentKeyDuplicates;
#[allow(unused)] // for cargo auto doc links
use crate::mdb::ffi;
use crate::mdb::lmdb_flags::DatabaseFlags;
use crate::*;

/// Options and flags which can be used to configure how a [`Database`] is opened.
///
/// # Examples
///
/// Opening a file to read:
///
/// ```
/// # use std::fs;
/// # use std::path::Path;
/// # use heed::EnvOpenOptions;
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
/// // Imagine you have an optional name
/// let conditional_name = Some("big-endian-iter");
///
/// let mut wtxn = env.write_txn()?;
/// let mut options = env.database_options().types::<BEI64, Unit>();
/// if let Some(name) = conditional_name {
///    options.name(name);
/// }
/// let db = options.create(&mut wtxn)?;
///
/// # db.clear(&mut wtxn)?;
/// db.put(&mut wtxn, &68, &())?;
/// db.put(&mut wtxn, &35, &())?;
/// db.put(&mut wtxn, &0, &())?;
/// db.put(&mut wtxn, &42, &())?;
///
/// wtxn.commit()?;
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct EncryptedDatabaseOpenOptions<'e, 'n, KC, DC, C = DefaultComparator> {
    inner: DatabaseOpenOptions<'e, 'n, KC, DC, C>,
}

impl<'e> EncryptedDatabaseOpenOptions<'e, 'static, Unspecified, Unspecified> {
    /// Create an options struct to open/create a database with specific flags.
    pub fn new(env: &'e EncryptedEnv) -> Self {
        EncryptedDatabaseOpenOptions { inner: DatabaseOpenOptions::new(&env.inner) }
    }
}

impl<'e, 'n, KC, DC, C> EncryptedDatabaseOpenOptions<'e, 'n, KC, DC, C> {
    /// Change the type of the database.
    ///
    /// The default types are [`Unspecified`] and require a call to [`Database::remap_types`]
    /// to use the [`Database`].
    pub fn types<NKC, NDC>(self) -> EncryptedDatabaseOpenOptions<'e, 'n, NKC, NDC> {
        EncryptedDatabaseOpenOptions { inner: self.inner.types() }
    }
    /// Change the customized key compare function of the database.
    ///
    /// By default no customized compare function will be set when opening a database.
    pub fn key_comparator<NC>(self) -> EncryptedDatabaseOpenOptions<'e, 'n, KC, DC, NC> {
        EncryptedDatabaseOpenOptions { inner: self.inner.key_comparator() }
    }

    /// Change the name of the database.
    ///
    /// By default the database is unnamed and there only is a single unnamed database.
    pub fn name(&mut self, name: &'n str) -> &mut Self {
        self.inner.name(name);
        self
    }

    /// Specify the set of flags used to open the database.
    pub fn flags(&mut self, flags: DatabaseFlags) -> &mut Self {
        self.inner.flags(flags);
        self
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
    pub fn open(&self, rtxn: &RoTxn) -> Result<Option<EncryptedDatabase<KC, DC, C>>>
    where
        KC: 'static,
        DC: 'static,
        C: Comparator + 'static,
    {
        self.inner.open(rtxn).map(|opt| opt.map(EncryptedDatabase::new))
    }

    /// Creates a typed database that can already exist in this environment.
    ///
    /// If the database was previously opened in this program run, types will be checked.
    ///
    /// ## Important Information
    ///
    /// LMDB has an important restriction on the unnamed database when named ones are opened.
    /// The names of the named databases are stored as keys in the unnamed one and are immutable,
    /// and these keys can only be read and not written.
    pub fn create(&self, wtxn: &mut RwTxn) -> Result<EncryptedDatabase<KC, DC, C>>
    where
        KC: 'static,
        DC: 'static,
        C: Comparator + 'static,
    {
        self.inner.create(wtxn).map(EncryptedDatabase::new)
    }
}

impl<KC, DC, C> Clone for EncryptedDatabaseOpenOptions<'_, '_, KC, DC, C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<KC, DC, C> Copy for EncryptedDatabaseOpenOptions<'_, '_, KC, DC, C> {}

/// A typed database that accepts only the types it was created with.
///
/// # Example: Iterate over databases entries
///
/// In this example we store numbers in big endian this way those are ordered.
/// Thanks to their bytes representation, heed is able to iterate over them
/// from the lowest to the highest.
///
/// ```
/// # use std::fs;
/// # use std::path::Path;
/// # use heed::EnvOpenOptions;
/// use heed::Database;
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
/// let db: Database<BEI64, Unit> = env.create_database(&mut wtxn, Some("big-endian-iter"))?;
///
/// # db.clear(&mut wtxn)?;
/// db.put(&mut wtxn, &68, &())?;
/// db.put(&mut wtxn, &35, &())?;
/// db.put(&mut wtxn, &0, &())?;
/// db.put(&mut wtxn, &42, &())?;
///
/// // you can iterate over database entries in order
/// let rets: Result<_, _> = db.iter(&wtxn)?.collect();
/// let rets: Vec<(i64, _)> = rets?;
///
/// let expected = vec![
///     (0, ()),
///     (35, ()),
///     (42, ()),
///     (68, ()),
/// ];
///
/// assert_eq!(rets, expected);
/// wtxn.commit()?;
/// # Ok(()) }
/// ```
///
/// # Example: Iterate over and delete ranges of entries
///
/// Discern also support ranges and ranges deletions.
/// Same configuration as above, numbers are ordered, therefore it is safe to specify
/// a range and be able to iterate over and/or delete it.
///
/// ```
/// # use std::fs;
/// # use std::path::Path;
/// # use heed::EnvOpenOptions;
/// use heed::Database;
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
/// let db: Database<BEI64, Unit> = env.create_database(&mut wtxn, Some("big-endian-iter"))?;
///
/// # db.clear(&mut wtxn)?;
/// db.put(&mut wtxn, &0, &())?;
/// db.put(&mut wtxn, &68, &())?;
/// db.put(&mut wtxn, &35, &())?;
/// db.put(&mut wtxn, &42, &())?;
///
/// // you can iterate over ranges too!!!
/// let range = 35..=42;
/// let rets: Result<_, _> = db.range(&wtxn, &range)?.collect();
/// let rets: Vec<(i64, _)> = rets?;
///
/// let expected = vec![
///     (35, ()),
///     (42, ()),
/// ];
///
/// assert_eq!(rets, expected);
///
/// // even delete a range of keys
/// let range = 35..=42;
/// let deleted: usize = db.delete_range(&mut wtxn, &range)?;
///
/// let rets: Result<_, _> = db.iter(&wtxn)?.collect();
/// let rets: Vec<(i64, _)> = rets?;
///
/// let expected = vec![
///     (0, ()),
///     (68, ()),
/// ];
///
/// assert_eq!(deleted, 2);
/// assert_eq!(rets, expected);
///
/// wtxn.commit()?;
/// # Ok(()) }
/// ```
pub struct EncryptedDatabase<KC, DC, C = DefaultComparator> {
    inner: Database<KC, DC, C>,
}

impl<KC, DC, C> EncryptedDatabase<KC, DC, C> {
    pub(crate) fn new(inner: Database<KC, DC, C>) -> EncryptedDatabase<KC, DC, C> {
        EncryptedDatabase { inner }
    }

    /// Retrieves the value associated with a key.
    ///
    /// If the key does not exist, then `None` is returned.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// type BEI32= U32<BigEndian>;
    ///
    /// let mut wtxn = env.write_txn()?;
    /// let db: Database<Str, BEI32> = env.create_database(&mut wtxn, Some("get-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, "i-am-forty-two", &42)?;
    /// db.put(&mut wtxn, "i-am-twenty-seven", &27)?;
    ///
    /// let ret = db.get(&wtxn, "i-am-forty-two")?;
    /// assert_eq!(ret, Some(42));
    ///
    /// let ret = db.get(&wtxn, "i-am-twenty-one")?;
    /// assert_eq!(ret, None);
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn get<'a, 'txn>(
        &self,
        txn: &'txn mut RoTxn,
        key: &'a KC::EItem,
    ) -> Result<Option<DC::DItem>>
    where
        KC: BytesEncode<'a>,
        DC: BytesDecode<'txn>,
    {
        self.inner.get(txn, key)
    }

    /// Returns an iterator over all of the values of a single key.
    ///
    /// You can make this iterator `Send`able between threads by
    /// using the `read-txn-no-tls` crate feature.
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
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn get_duplicates<'a, 'txn>(
        &self,
        txn: &'txn mut RoTxn,
        key: &'a KC::EItem,
    ) -> Result<Option<RoIter<'txn, KC, DC, MoveOnCurrentKeyDuplicates>>>
    where
        KC: BytesEncode<'a>,
    {
        self.inner.get_duplicates(txn, key)
    }

    /// Retrieves the key/value pair lower than the given one in this database.
    ///
    /// If the database if empty or there is no key lower than the given one,
    /// then `None` is returned.
    ///
    /// Comparisons are made by using the bytes representation of the key.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// type BEU32 = U32<BigEndian>;
    ///
    /// let mut wtxn = env.write_txn()?;
    /// let db = env.create_database::<BEU32, Unit>(&mut wtxn, Some("get-lt-u32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &27, &())?;
    /// db.put(&mut wtxn, &42, &())?;
    /// db.put(&mut wtxn, &43, &())?;
    ///
    /// let ret = db.get_lower_than(&wtxn, &4404)?;
    /// assert_eq!(ret, Some((43, ())));
    ///
    /// let ret = db.get_lower_than(&wtxn, &43)?;
    /// assert_eq!(ret, Some((42, ())));
    ///
    /// let ret = db.get_lower_than(&wtxn, &27)?;
    /// assert_eq!(ret, None);
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn get_lower_than<'a, 'txn>(
        &self,
        txn: &'txn mut RoTxn,
        key: &'a KC::EItem,
    ) -> Result<Option<(KC::DItem, DC::DItem)>>
    where
        KC: BytesEncode<'a> + BytesDecode<'txn>,
        DC: BytesDecode<'txn>,
    {
        self.inner.get_lower_than(txn, key)
    }

    /// Retrieves the key/value pair lower than or equal to the given one in this database.
    ///
    /// If the database if empty or there is no key lower than or equal to the given one,
    /// then `None` is returned.
    ///
    /// Comparisons are made by using the bytes representation of the key.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// type BEU32 = U32<BigEndian>;
    ///
    /// let mut wtxn = env.write_txn()?;
    /// let db = env.create_database::<BEU32, Unit>(&mut wtxn, Some("get-lt-u32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &27, &())?;
    /// db.put(&mut wtxn, &42, &())?;
    /// db.put(&mut wtxn, &43, &())?;
    ///
    /// let ret = db.get_lower_than_or_equal_to(&wtxn, &4404)?;
    /// assert_eq!(ret, Some((43, ())));
    ///
    /// let ret = db.get_lower_than_or_equal_to(&wtxn, &43)?;
    /// assert_eq!(ret, Some((43, ())));
    ///
    /// let ret = db.get_lower_than_or_equal_to(&wtxn, &26)?;
    /// assert_eq!(ret, None);
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn get_lower_than_or_equal_to<'a, 'txn>(
        &self,
        txn: &'txn mut RoTxn,
        key: &'a KC::EItem,
    ) -> Result<Option<(KC::DItem, DC::DItem)>>
    where
        KC: BytesEncode<'a> + BytesDecode<'txn>,
        DC: BytesDecode<'txn>,
    {
        self.inner.get_lower_than_or_equal_to(txn, key)
    }

    /// Retrieves the key/value pair greater than the given one in this database.
    ///
    /// If the database if empty or there is no key greater than the given one,
    /// then `None` is returned.
    ///
    /// Comparisons are made by using the bytes representation of the key.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// type BEU32 = U32<BigEndian>;
    ///
    /// let mut wtxn = env.write_txn()?;
    /// let db = env.create_database::<BEU32, Unit>(&mut wtxn, Some("get-lt-u32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &27, &())?;
    /// db.put(&mut wtxn, &42, &())?;
    /// db.put(&mut wtxn, &43, &())?;
    ///
    /// let ret = db.get_greater_than(&wtxn, &0)?;
    /// assert_eq!(ret, Some((27, ())));
    ///
    /// let ret = db.get_greater_than(&wtxn, &42)?;
    /// assert_eq!(ret, Some((43, ())));
    ///
    /// let ret = db.get_greater_than(&wtxn, &43)?;
    /// assert_eq!(ret, None);
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn get_greater_than<'a, 'txn>(
        &self,
        txn: &'txn mut RoTxn,
        key: &'a KC::EItem,
    ) -> Result<Option<(KC::DItem, DC::DItem)>>
    where
        KC: BytesEncode<'a> + BytesDecode<'txn>,
        DC: BytesDecode<'txn>,
    {
        self.inner.get_greater_than(txn, key)
    }

    /// Retrieves the key/value pair greater than or equal to the given one in this database.
    ///
    /// If the database if empty or there is no key greater than or equal to the given one,
    /// then `None` is returned.
    ///
    /// Comparisons are made by using the bytes representation of the key.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// type BEU32 = U32<BigEndian>;
    ///
    /// let mut wtxn = env.write_txn()?;
    /// let db = env.create_database::<BEU32, Unit>(&mut wtxn, Some("get-lt-u32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &27, &())?;
    /// db.put(&mut wtxn, &42, &())?;
    /// db.put(&mut wtxn, &43, &())?;
    ///
    /// let ret = db.get_greater_than_or_equal_to(&wtxn, &0)?;
    /// assert_eq!(ret, Some((27, ())));
    ///
    /// let ret = db.get_greater_than_or_equal_to(&wtxn, &42)?;
    /// assert_eq!(ret, Some((42, ())));
    ///
    /// let ret = db.get_greater_than_or_equal_to(&wtxn, &44)?;
    /// assert_eq!(ret, None);
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn get_greater_than_or_equal_to<'a, 'txn>(
        &self,
        txn: &'txn mut RoTxn,
        key: &'a KC::EItem,
    ) -> Result<Option<(KC::DItem, DC::DItem)>>
    where
        KC: BytesEncode<'a> + BytesDecode<'txn>,
        DC: BytesDecode<'txn>,
    {
        self.inner.get_greater_than_or_equal_to(txn, key)
    }

    /// Retrieves the first key/value pair of this database.
    ///
    /// If the database if empty, then `None` is returned.
    ///
    /// Comparisons are made by using the bytes representation of the key.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("first-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    ///
    /// let ret = db.first(&wtxn)?;
    /// assert_eq!(ret, Some((27, "i-am-twenty-seven")));
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn first<'txn>(&self, txn: &'txn mut RoTxn) -> Result<Option<(KC::DItem, DC::DItem)>>
    where
        KC: BytesDecode<'txn>,
        DC: BytesDecode<'txn>,
    {
        self.inner.first(txn)
    }

    /// Retrieves the last key/value pair of this database.
    ///
    /// If the database if empty, then `None` is returned.
    ///
    /// Comparisons are made by using the bytes representation of the key.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("last-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    ///
    /// let ret = db.last(&wtxn)?;
    /// assert_eq!(ret, Some((42, "i-am-forty-two")));
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn last<'txn>(&self, txn: &'txn mut RoTxn) -> Result<Option<(KC::DItem, DC::DItem)>>
    where
        KC: BytesDecode<'txn>,
        DC: BytesDecode<'txn>,
    {
        self.inner.last(txn)
    }

    /// Returns the number of elements in this database.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    /// db.put(&mut wtxn, &521, "i-am-five-hundred-and-twenty-one")?;
    ///
    /// let ret = db.len(&wtxn)?;
    /// assert_eq!(ret, 4);
    ///
    /// db.delete(&mut wtxn, &27)?;
    ///
    /// let ret = db.len(&wtxn)?;
    /// assert_eq!(ret, 3);
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn len(&self, txn: &RoTxn) -> Result<u64> {
        self.inner.len(txn)
    }

    /// Returns `true` if and only if this database is empty.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    /// db.put(&mut wtxn, &521, "i-am-five-hundred-and-twenty-one")?;
    ///
    /// let ret = db.is_empty(&wtxn)?;
    /// assert_eq!(ret, false);
    ///
    /// db.clear(&mut wtxn)?;
    ///
    /// let ret = db.is_empty(&wtxn)?;
    /// assert_eq!(ret, true);
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn is_empty(&self, txn: &RoTxn) -> Result<bool> {
        self.inner.is_empty(txn)
    }

    /// Returns some statistics for this database.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    /// db.put(&mut wtxn, &521, "i-am-five-hundred-and-twenty-one")?;
    ///
    /// let stat = db.stat(&wtxn)?;
    /// assert_eq!(stat.depth, 1);
    /// assert_eq!(stat.branch_pages, 0);
    /// assert_eq!(stat.leaf_pages, 1);
    /// assert_eq!(stat.overflow_pages, 0);
    /// assert_eq!(stat.entries, 4);
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn stat(&self, txn: &RoTxn) -> Result<DatabaseStat> {
        self.inner.stat(txn)
    }

    /// Return a lexicographically ordered iterator of all key-value pairs in this database.
    ///
    /// You can make this iterator `Send`able between threads by
    /// using the `read-txn-no-tls` crate feature.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    ///
    /// let mut iter = db.iter(&wtxn)?;
    /// assert_eq!(iter.next().transpose()?, Some((13, "i-am-thirteen")));
    /// assert_eq!(iter.next().transpose()?, Some((27, "i-am-twenty-seven")));
    /// assert_eq!(iter.next().transpose()?, Some((42, "i-am-forty-two")));
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn iter<'txn>(&self, txn: &'txn mut RoTxn) -> Result<RoIter<'txn, KC, DC>> {
        self.inner.iter(txn)
    }

    /// Return a mutable lexicographically ordered iterator of all key-value pairs in this database.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    ///
    /// let mut iter = db.iter_mut(&mut wtxn)?;
    /// assert_eq!(iter.next().transpose()?, Some((13, "i-am-thirteen")));
    /// let ret = unsafe { iter.del_current()? };
    /// assert!(ret);
    ///
    /// assert_eq!(iter.next().transpose()?, Some((27, "i-am-twenty-seven")));
    /// assert_eq!(iter.next().transpose()?, Some((42, "i-am-forty-two")));
    /// let ret = unsafe { iter.put_current(&42, "i-am-the-new-forty-two")? };
    /// assert!(ret);
    ///
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    ///
    /// let ret = db.get(&wtxn, &13)?;
    /// assert_eq!(ret, None);
    ///
    /// let ret = db.get(&wtxn, &42)?;
    /// assert_eq!(ret, Some("i-am-the-new-forty-two"));
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn iter_mut<'txn>(&self, txn: &'txn mut RwTxn) -> Result<RwIter<'txn, KC, DC>> {
        self.inner.iter_mut(txn)
    }

    /// Return a reversed lexicographically ordered iterator of all key-value pairs in this database.
    ///
    /// You can make this iterator `Send`able between threads by
    /// using the `read-txn-no-tls` crate feature.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    ///
    /// let mut iter = db.rev_iter(&wtxn)?;
    /// assert_eq!(iter.next().transpose()?, Some((42, "i-am-forty-two")));
    /// assert_eq!(iter.next().transpose()?, Some((27, "i-am-twenty-seven")));
    /// assert_eq!(iter.next().transpose()?, Some((13, "i-am-thirteen")));
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn rev_iter<'txn>(&self, txn: &'txn mut RoTxn) -> Result<RoRevIter<'txn, KC, DC>> {
        self.inner.rev_iter(txn)
    }

    /// Return a mutable reversed lexicographically ordered iterator of all key-value\
    /// pairs in this database.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    ///
    /// let mut iter = db.rev_iter_mut(&mut wtxn)?;
    /// assert_eq!(iter.next().transpose()?, Some((42, "i-am-forty-two")));
    /// let ret = unsafe { iter.del_current()? };
    /// assert!(ret);
    ///
    /// assert_eq!(iter.next().transpose()?, Some((27, "i-am-twenty-seven")));
    /// assert_eq!(iter.next().transpose()?, Some((13, "i-am-thirteen")));
    /// let ret = unsafe { iter.put_current(&13, "i-am-the-new-thirteen")? };
    /// assert!(ret);
    ///
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    ///
    /// let ret = db.get(&wtxn, &42)?;
    /// assert_eq!(ret, None);
    ///
    /// let ret = db.get(&wtxn, &13)?;
    /// assert_eq!(ret, Some("i-am-the-new-thirteen"));
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn rev_iter_mut<'txn>(&self, txn: &'txn mut RwTxn) -> Result<RwRevIter<'txn, KC, DC>> {
        self.inner.rev_iter_mut(txn)
    }

    /// Return an ordered iterator of a range of key-value pairs in this database.
    ///
    /// Comparisons are made by using the comparator `C`.
    ///
    /// You can make this iterator `Send`able between threads by
    /// using the `read-txn-no-tls` crate feature.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    /// db.put(&mut wtxn, &521, "i-am-five-hundred-and-twenty-one")?;
    ///
    /// let range = 27..=42;
    /// let mut iter = db.range(&wtxn, &range)?;
    /// assert_eq!(iter.next().transpose()?, Some((27, "i-am-twenty-seven")));
    /// assert_eq!(iter.next().transpose()?, Some((42, "i-am-forty-two")));
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn range<'a, 'txn, R>(
        &self,
        txn: &'txn mut RoTxn,
        range: &'a R,
    ) -> Result<RoRange<'txn, KC, DC, C>>
    where
        KC: BytesEncode<'a>,
        R: RangeBounds<KC::EItem>,
    {
        self.inner.range(txn, range)
    }

    /// Return a mutable ordered iterator of a range of
    /// key-value pairs in this database.
    ///
    /// Comparisons are made by using the comparator `C`.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    /// db.put(&mut wtxn, &521, "i-am-five-hundred-and-twenty-one")?;
    ///
    /// let range = 27..=42;
    /// let mut range = db.range_mut(&mut wtxn, &range)?;
    /// assert_eq!(range.next().transpose()?, Some((27, "i-am-twenty-seven")));
    /// let ret = unsafe { range.del_current()? };
    /// assert!(ret);
    /// assert_eq!(range.next().transpose()?, Some((42, "i-am-forty-two")));
    /// let ret = unsafe { range.put_current(&42, "i-am-the-new-forty-two")? };
    /// assert!(ret);
    ///
    /// assert_eq!(range.next().transpose()?, None);
    /// drop(range);
    ///
    ///
    /// let mut iter = db.iter(&wtxn)?;
    /// assert_eq!(iter.next().transpose()?, Some((13, "i-am-thirteen")));
    /// assert_eq!(iter.next().transpose()?, Some((42, "i-am-the-new-forty-two")));
    /// assert_eq!(iter.next().transpose()?, Some((521, "i-am-five-hundred-and-twenty-one")));
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn range_mut<'a, 'txn, R>(
        &self,
        txn: &'txn mut RwTxn,
        range: &'a R,
    ) -> Result<RwRange<'txn, KC, DC, C>>
    where
        KC: BytesEncode<'a>,
        R: RangeBounds<KC::EItem>,
    {
        self.inner.range_mut(txn, range)
    }

    /// Return a reverse ordered iterator of a range of key-value
    /// pairs in this database.
    ///
    /// Comparisons are made by using the comparator `C`.
    ///
    /// You can make this iterator `Send`able between threads by
    /// using the `read-txn-no-tls` crate feature.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    /// db.put(&mut wtxn, &521, "i-am-five-hundred-and-twenty-one")?;
    ///
    /// let range = 27..=43;
    /// let mut iter = db.rev_range(&wtxn, &range)?;
    /// assert_eq!(iter.next().transpose()?, Some((42, "i-am-forty-two")));
    /// assert_eq!(iter.next().transpose()?, Some((27, "i-am-twenty-seven")));
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn rev_range<'a, 'txn, R>(
        &self,
        txn: &'txn mut RoTxn,
        range: &'a R,
    ) -> Result<RoRevRange<'txn, KC, DC, C>>
    where
        KC: BytesEncode<'a>,
        R: RangeBounds<KC::EItem>,
    {
        self.inner.rev_range(txn, range)
    }

    /// Return a mutable reverse ordered iterator of a range of
    /// key-value pairs in this database.
    ///
    /// Comparisons are made by using the comparator `C`.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    /// db.put(&mut wtxn, &521, "i-am-five-hundred-and-twenty-one")?;
    ///
    /// let range = 27..=42;
    /// let mut range = db.rev_range_mut(&mut wtxn, &range)?;
    /// assert_eq!(range.next().transpose()?, Some((42, "i-am-forty-two")));
    /// let ret = unsafe { range.del_current()? };
    /// assert!(ret);
    /// assert_eq!(range.next().transpose()?, Some((27, "i-am-twenty-seven")));
    /// let ret = unsafe { range.put_current(&27, "i-am-the-new-twenty-seven")? };
    /// assert!(ret);
    ///
    /// assert_eq!(range.next().transpose()?, None);
    /// drop(range);
    ///
    ///
    /// let mut iter = db.iter(&wtxn)?;
    /// assert_eq!(iter.next().transpose()?, Some((13, "i-am-thirteen")));
    /// assert_eq!(iter.next().transpose()?, Some((27, "i-am-the-new-twenty-seven")));
    /// assert_eq!(iter.next().transpose()?, Some((521, "i-am-five-hundred-and-twenty-one")));
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn rev_range_mut<'a, 'txn, R>(
        &self,
        txn: &'txn mut RwTxn,
        range: &'a R,
    ) -> Result<RwRevRange<'txn, KC, DC, C>>
    where
        KC: BytesEncode<'a>,
        R: RangeBounds<KC::EItem>,
    {
        self.inner.rev_range_mut(txn, range)
    }

    /// Return a lexicographically ordered iterator of all key-value pairs
    /// in this database that starts with the given prefix.
    ///
    /// Comparisons are made by using the bytes representation of the key.
    ///
    /// You can make this iterator `Send`able between threads by
    /// using the `read-txn-no-tls` crate feature.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<Str, BEI32> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, "i-am-twenty-eight", &28)?;
    /// db.put(&mut wtxn, "i-am-twenty-seven", &27)?;
    /// db.put(&mut wtxn, "i-am-twenty-nine",  &29)?;
    /// db.put(&mut wtxn, "i-am-forty-one",    &41)?;
    /// db.put(&mut wtxn, "i-am-forty-two",    &42)?;
    ///
    /// let mut iter = db.prefix_iter(&mut wtxn, "i-am-twenty")?;
    /// assert_eq!(iter.next().transpose()?, Some(("i-am-twenty-eight", 28)));
    /// assert_eq!(iter.next().transpose()?, Some(("i-am-twenty-nine", 29)));
    /// assert_eq!(iter.next().transpose()?, Some(("i-am-twenty-seven", 27)));
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn prefix_iter<'a, 'txn>(
        &self,
        txn: &'txn mut RoTxn,
        prefix: &'a KC::EItem,
    ) -> Result<RoPrefix<'txn, KC, DC, C>>
    where
        KC: BytesEncode<'a>,
        C: LexicographicComparator,
    {
        self.inner.prefix_iter(txn, prefix)
    }

    /// Return a mutable lexicographically ordered iterator of all key-value pairs
    /// in this database that starts with the given prefix.
    ///
    /// Comparisons are made by using the bytes representation of the key.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<Str, BEI32> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, "i-am-twenty-eight", &28)?;
    /// db.put(&mut wtxn, "i-am-twenty-seven", &27)?;
    /// db.put(&mut wtxn, "i-am-twenty-nine",  &29)?;
    /// db.put(&mut wtxn, "i-am-forty-one",    &41)?;
    /// db.put(&mut wtxn, "i-am-forty-two",    &42)?;
    ///
    /// let mut iter = db.prefix_iter_mut(&mut wtxn, "i-am-twenty")?;
    /// assert_eq!(iter.next().transpose()?, Some(("i-am-twenty-eight", 28)));
    /// let ret = unsafe { iter.del_current()? };
    /// assert!(ret);
    ///
    /// assert_eq!(iter.next().transpose()?, Some(("i-am-twenty-nine", 29)));
    /// assert_eq!(iter.next().transpose()?, Some(("i-am-twenty-seven", 27)));
    /// let ret = unsafe { iter.put_current("i-am-twenty-seven", &27000)? };
    /// assert!(ret);
    ///
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    ///
    /// let ret = db.get(&wtxn, "i-am-twenty-eight")?;
    /// assert_eq!(ret, None);
    ///
    /// let ret = db.get(&wtxn, "i-am-twenty-seven")?;
    /// assert_eq!(ret, Some(27000));
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn prefix_iter_mut<'a, 'txn>(
        &self,
        txn: &'txn mut RwTxn,
        prefix: &'a KC::EItem,
    ) -> Result<RwPrefix<'txn, KC, DC, C>>
    where
        KC: BytesEncode<'a>,
        C: LexicographicComparator,
    {
        self.inner.prefix_iter_mut(txn, prefix)
    }

    /// Return a reversed lexicographically ordered iterator of all key-value pairs
    /// in this database that starts with the given prefix.
    ///
    /// Comparisons are made by using the bytes representation of the key.
    ///
    /// You can make this iterator `Send`able between threads by
    /// using the `read-txn-no-tls` crate feature.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<Str, BEI32> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, "i-am-twenty-eight", &28)?;
    /// db.put(&mut wtxn, "i-am-twenty-seven", &27)?;
    /// db.put(&mut wtxn, "i-am-twenty-nine",  &29)?;
    /// db.put(&mut wtxn, "i-am-forty-one",    &41)?;
    /// db.put(&mut wtxn, "i-am-forty-two",    &42)?;
    ///
    /// let mut iter = db.rev_prefix_iter(&mut wtxn, "i-am-twenty")?;
    /// assert_eq!(iter.next().transpose()?, Some(("i-am-twenty-seven", 27)));
    /// assert_eq!(iter.next().transpose()?, Some(("i-am-twenty-nine", 29)));
    /// assert_eq!(iter.next().transpose()?, Some(("i-am-twenty-eight", 28)));
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn rev_prefix_iter<'a, 'txn>(
        &self,
        txn: &'txn mut RoTxn,
        prefix: &'a KC::EItem,
    ) -> Result<RoRevPrefix<'txn, KC, DC, C>>
    where
        KC: BytesEncode<'a>,
        C: LexicographicComparator,
    {
        self.inner.rev_prefix_iter(txn, prefix)
    }

    /// Return a mutable reversed lexicographically ordered iterator of all key-value pairs
    /// in this database that starts with the given prefix.
    ///
    /// Comparisons are made by using the bytes representation of the key.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<Str, BEI32> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, "i-am-twenty-eight", &28)?;
    /// db.put(&mut wtxn, "i-am-twenty-seven", &27)?;
    /// db.put(&mut wtxn, "i-am-twenty-nine",  &29)?;
    /// db.put(&mut wtxn, "i-am-forty-one",    &41)?;
    /// db.put(&mut wtxn, "i-am-forty-two",    &42)?;
    ///
    /// let mut iter = db.rev_prefix_iter_mut(&mut wtxn, "i-am-twenty")?;
    /// assert_eq!(iter.next().transpose()?, Some(("i-am-twenty-seven", 27)));
    /// let ret = unsafe { iter.del_current()? };
    /// assert!(ret);
    ///
    /// assert_eq!(iter.next().transpose()?, Some(("i-am-twenty-nine", 29)));
    /// assert_eq!(iter.next().transpose()?, Some(("i-am-twenty-eight", 28)));
    /// let ret = unsafe { iter.put_current("i-am-twenty-eight", &28000)? };
    /// assert!(ret);
    ///
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    ///
    /// let ret = db.get(&wtxn, "i-am-twenty-seven")?;
    /// assert_eq!(ret, None);
    ///
    /// let ret = db.get(&wtxn, "i-am-twenty-eight")?;
    /// assert_eq!(ret, Some(28000));
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn rev_prefix_iter_mut<'a, 'txn>(
        &self,
        txn: &'txn mut RwTxn,
        prefix: &'a KC::EItem,
    ) -> Result<RwRevPrefix<'txn, KC, DC, C>>
    where
        KC: BytesEncode<'a>,
        C: LexicographicComparator,
    {
        self.inner.rev_prefix_iter_mut(txn, prefix)
    }

    /// Insert a key-value pair in this database, replacing any previous value. The entry is
    /// written with no specific flag.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    /// db.put(&mut wtxn, &521, "i-am-five-hundred-and-twenty-one")?;
    ///
    /// let ret = db.get(&mut wtxn, &27)?;
    /// assert_eq!(ret, Some("i-am-twenty-seven"));
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn put<'a>(&self, txn: &mut RwTxn, key: &'a KC::EItem, data: &'a DC::EItem) -> Result<()>
    where
        KC: BytesEncode<'a>,
        DC: BytesEncode<'a>,
    {
        self.inner.put(txn, key, data)
    }

    /// Insert a key-value pair where the value can directly be written to disk, replacing any
    /// previous value.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use std::io::Write;
    /// use heed::Database;
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
    /// let db = env.create_database::<BEI32, Str>(&mut wtxn, Some("number-string"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// let value = "I am a long long long value";
    /// db.put_reserved(&mut wtxn, &42, value.len(), |reserved| {
    ///     reserved.write_all(value.as_bytes())
    /// })?;
    ///
    /// let ret = db.get(&mut wtxn, &42)?;
    /// assert_eq!(ret, Some(value));
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn put_reserved<'a, F>(
        &self,
        txn: &mut RwTxn,
        key: &'a KC::EItem,
        data_size: usize,
        write_func: F,
    ) -> Result<()>
    where
        KC: BytesEncode<'a>,
        F: FnOnce(&mut ReservedSpace) -> io::Result<()>,
    {
        self.inner.put_reserved(txn, key, data_size, write_func)
    }

    /// Insert a key-value pair in this database, replacing any previous value. The entry is
    /// written with the specified flags.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::{Database, PutFlags, DatabaseFlags, Error, MdbError};
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
    ///     .types::<BEI32, Str>()
    ///     .name("dup-i32")
    ///     .flags(DatabaseFlags::DUP_SORT)
    ///     .create(&mut wtxn)?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &42, "i-am-so-cool")?;
    /// db.put(&mut wtxn, &42, "i-am-the-king")?;
    /// db.put(&mut wtxn, &42, "i-am-fun")?;
    /// db.put_with_flags(&mut wtxn, PutFlags::APPEND, &54, "i-am-older-than-you")?;
    /// db.put_with_flags(&mut wtxn, PutFlags::APPEND_DUP, &54, "ok-but-i-am-better-than-you")?;
    /// // You can compose flags by OR'ing them
    /// db.put_with_flags(&mut wtxn, PutFlags::APPEND_DUP | PutFlags::NO_OVERWRITE, &55, "welcome")?;
    ///
    /// // The NO_DUP_DATA flag will return KeyExist if we try to insert the exact same key/value pair.
    /// let ret = db.put_with_flags(&mut wtxn, PutFlags::NO_DUP_DATA, &54, "ok-but-i-am-better-than-you");
    /// assert!(matches!(ret, Err(Error::Mdb(MdbError::KeyExist))));
    ///
    /// // The NO_OVERWRITE flag will return KeyExist if we try to insert something with an already existing key.
    /// let ret = db.put_with_flags(&mut wtxn, PutFlags::NO_OVERWRITE, &54, "there-can-be-only-one-data");
    /// assert!(matches!(ret, Err(Error::Mdb(MdbError::KeyExist))));
    ///
    /// let mut iter = db.iter(&wtxn)?;
    /// assert_eq!(iter.next().transpose()?, Some((42, "i-am-forty-two")));
    /// assert_eq!(iter.next().transpose()?, Some((42, "i-am-fun")));
    /// assert_eq!(iter.next().transpose()?, Some((42, "i-am-so-cool")));
    /// assert_eq!(iter.next().transpose()?, Some((42, "i-am-the-king")));
    /// assert_eq!(iter.next().transpose()?, Some((54, "i-am-older-than-you")));
    /// assert_eq!(iter.next().transpose()?, Some((54, "ok-but-i-am-better-than-you")));
    /// assert_eq!(iter.next().transpose()?, Some((55, "welcome")));
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn put_with_flags<'a>(
        &self,
        txn: &mut RwTxn,
        flags: PutFlags,
        key: &'a KC::EItem,
        data: &'a DC::EItem,
    ) -> Result<()>
    where
        KC: BytesEncode<'a>,
        DC: BytesEncode<'a>,
    {
        self.inner.put_with_flags(txn, flags, key, data)
    }

    /// Attempt to insert a key-value pair in this database, or if a value already exists for the
    /// key, returns the previous value.
    ///
    /// The entry is always written with the [`NO_OVERWRITE`](PutFlags::NO_OVERWRITE) flag.
    ///
    /// ```
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// assert_eq!(db.get_or_put(&mut wtxn, &42, "i-am-forty-two")?, None);
    /// assert_eq!(db.get_or_put(&mut wtxn, &42, "the meaning of life")?, Some("i-am-forty-two"));
    ///
    /// let ret = db.get(&mut wtxn, &42)?;
    /// assert_eq!(ret, Some("i-am-forty-two"));
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn get_or_put<'a, 'txn>(
        &'txn self,
        txn: &mut RwTxn,
        key: &'a KC::EItem,
        data: &'a DC::EItem,
    ) -> Result<Option<DC::DItem>>
    where
        KC: BytesEncode<'a>,
        DC: BytesEncode<'a> + BytesDecode<'a>,
    {
        self.inner.get_or_put(txn, key, data)
    }

    /// Attempt to insert a key-value pair in this database, or if a value already exists for the
    /// key, returns the previous value.
    ///
    /// The entry is written with the specified flags, in addition to
    /// [`NO_OVERWRITE`](PutFlags::NO_OVERWRITE) which is always used.
    ///
    /// ```
    /// # use heed::EnvOpenOptions;
    /// use heed::{Database, PutFlags};
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// assert_eq!(db.get_or_put_with_flags(&mut wtxn, PutFlags::empty(), &42, "i-am-forty-two")?, None);
    /// assert_eq!(db.get_or_put_with_flags(&mut wtxn, PutFlags::empty(), &42, "the meaning of life")?, Some("i-am-forty-two"));
    ///
    /// let ret = db.get(&mut wtxn, &42)?;
    /// assert_eq!(ret, Some("i-am-forty-two"));
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn get_or_put_with_flags<'a, 'txn>(
        &'txn self,
        txn: &mut RwTxn,
        flags: PutFlags,
        key: &'a KC::EItem,
        data: &'a DC::EItem,
    ) -> Result<Option<DC::DItem>>
    where
        KC: BytesEncode<'a>,
        DC: BytesEncode<'a> + BytesDecode<'a>,
    {
        self.inner.get_or_put_with_flags(txn, flags, key, data)
    }

    /// Attempt to insert a key-value pair in this database, where the value can be directly
    /// written to disk, or if a value already exists for the key, returns the previous value.
    ///
    /// The entry is always written with the [`NO_OVERWRITE`](PutFlags::NO_OVERWRITE) and
    /// [`MDB_RESERVE`](ffi::MDB_RESERVE) flags.
    ///
    /// ```
    /// # use heed::EnvOpenOptions;
    /// use std::io::Write;
    /// use heed::{Database, PutFlags};
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
    /// let db = env.create_database::<BEI32, Str>(&mut wtxn, Some("number-string"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// let long = "I am a long long long value";
    /// assert_eq!(
    ///     db.get_or_put_reserved(&mut wtxn, &42, long.len(), |reserved| {
    ///         reserved.write_all(long.as_bytes())
    ///     })?,
    ///     None
    /// );
    ///
    /// let longer = "I am an even longer long long long value";
    /// assert_eq!(
    ///     db.get_or_put_reserved(&mut wtxn, &42, longer.len(), |reserved| {
    ///         unreachable!()
    ///     })?,
    ///     Some(long)
    /// );
    ///
    /// let ret = db.get(&mut wtxn, &42)?;
    /// assert_eq!(ret, Some(long));
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn get_or_put_reserved<'a, 'txn, F>(
        &'txn self,
        txn: &mut RwTxn,
        key: &'a KC::EItem,
        data_size: usize,
        write_func: F,
    ) -> Result<Option<DC::DItem>>
    where
        KC: BytesEncode<'a>,
        F: FnOnce(&mut ReservedSpace) -> io::Result<()>,
        DC: BytesDecode<'a>,
    {
        self.inner.get_or_put_reserved(txn, key, data_size, write_func)
    }

    /// Attempt to insert a key-value pair in this database, where the value can be directly
    /// written to disk, or if a value already exists for the key, returns the previous value.
    ///
    /// The entry is written with the specified flags, in addition to
    /// [`NO_OVERWRITE`](PutFlags::NO_OVERWRITE) and [`MDB_RESERVE`](ffi::MDB_RESERVE)
    /// which are always used.
    ///
    /// ```
    /// # use heed::EnvOpenOptions;
    /// use std::io::Write;
    /// use heed::{Database, PutFlags};
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
    /// let db = env.create_database::<BEI32, Str>(&mut wtxn, Some("number-string"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// let long = "I am a long long long value";
    /// assert_eq!(
    ///     db.get_or_put_reserved_with_flags(&mut wtxn, PutFlags::empty(), &42, long.len(), |reserved| {
    ///         reserved.write_all(long.as_bytes())
    ///     })?,
    ///     None
    /// );
    ///
    /// let longer = "I am an even longer long long long value";
    /// assert_eq!(
    ///     db.get_or_put_reserved_with_flags(&mut wtxn, PutFlags::empty(), &42, longer.len(), |reserved| {
    ///         unreachable!()
    ///     })?,
    ///     Some(long)
    /// );
    ///
    /// let ret = db.get(&mut wtxn, &42)?;
    /// assert_eq!(ret, Some(long));
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn get_or_put_reserved_with_flags<'a, 'txn, F>(
        &'txn self,
        txn: &mut RwTxn,
        flags: PutFlags,
        key: &'a KC::EItem,
        data_size: usize,
        write_func: F,
    ) -> Result<Option<DC::DItem>>
    where
        KC: BytesEncode<'a>,
        F: FnOnce(&mut ReservedSpace) -> io::Result<()>,
        DC: BytesDecode<'a>,
    {
        self.inner.get_or_put_reserved_with_flags(txn, flags, key, data_size, write_func)
    }

    /// Deletes an entry or every duplicate data items of a key
    /// if the database supports duplicate data items.
    ///
    /// If the entry does not exist, then `false` is returned.
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    /// db.put(&mut wtxn, &521, "i-am-five-hundred-and-twenty-one")?;
    ///
    /// let ret = db.delete(&mut wtxn, &27)?;
    /// assert_eq!(ret, true);
    ///
    /// let ret = db.get(&mut wtxn, &27)?;
    /// assert_eq!(ret, None);
    ///
    /// let ret = db.delete(&mut wtxn, &467)?;
    /// assert_eq!(ret, false);
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn delete<'a>(&self, txn: &mut RwTxn, key: &'a KC::EItem) -> Result<bool>
    where
        KC: BytesEncode<'a>,
    {
        self.inner.delete(txn, key)
    }

    /// Deletes a single key-value pair in this database.
    ///
    /// If the database doesn't support duplicate data items the data is ignored.
    /// If the key does not exist, then `false` is returned.
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
    pub fn delete_one_duplicate<'a>(
        &self,
        txn: &mut RwTxn,
        key: &'a KC::EItem,
        data: &'a DC::EItem,
    ) -> Result<bool>
    where
        KC: BytesEncode<'a>,
        DC: BytesEncode<'a>,
    {
        self.inner.delete_one_duplicate(txn, key, data)
    }

    /// Deletes a range of key-value pairs in this database.
    ///
    /// Prefer using [`clear`] instead of a call to this method with a full range ([`..`]).
    ///
    /// Comparisons are made by using the comparator `C`.
    ///
    /// [`clear`]: crate::Database::clear
    /// [`..`]: std::ops::RangeFull
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    /// db.put(&mut wtxn, &521, "i-am-five-hundred-and-twenty-one")?;
    ///
    /// let range = 27..=42;
    /// let ret = db.delete_range(&mut wtxn, &range)?;
    /// assert_eq!(ret, 2);
    ///
    ///
    /// let mut iter = db.iter(&wtxn)?;
    /// assert_eq!(iter.next().transpose()?, Some((13, "i-am-thirteen")));
    /// assert_eq!(iter.next().transpose()?, Some((521, "i-am-five-hundred-and-twenty-one")));
    /// assert_eq!(iter.next().transpose()?, None);
    ///
    /// drop(iter);
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn delete_range<'a, 'txn, R>(&self, txn: &'txn mut RwTxn, range: &'a R) -> Result<usize>
    where
        KC: BytesEncode<'a> + BytesDecode<'txn>,
        C: Comparator,
        R: RangeBounds<KC::EItem>,
    {
        self.inner.delete_range(txn, range)
    }

    /// Deletes all key/value pairs in this database.
    ///
    /// Prefer using this method instead of a call to [`delete_range`] with a full range ([`..`]).
    ///
    /// [`delete_range`]: crate::Database::delete_range
    /// [`..`]: std::ops::RangeFull
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<BEI32, Str> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    /// db.put(&mut wtxn, &521, "i-am-five-hundred-and-twenty-one")?;
    ///
    /// db.clear(&mut wtxn)?;
    ///
    /// let ret = db.is_empty(&wtxn)?;
    /// assert!(ret);
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn clear(&self, txn: &mut RwTxn) -> Result<()> {
        self.inner.clear(txn)
    }

    /// Change the codec types of this database, specifying the codecs.
    ///
    /// # Safety
    ///
    /// It is up to you to ensure that the data read and written using the polymorphic
    /// handle correspond to the the typed, uniform one. If an invalid write is made,
    /// it can corrupt the database from the eyes of heed.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::fs;
    /// # use std::path::Path;
    /// # use heed::EnvOpenOptions;
    /// use heed::Database;
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
    /// let db: Database<Unit, Unit> = env.create_database(&mut wtxn, Some("iter-i32"))?;
    ///
    /// # db.clear(&mut wtxn)?;
    /// // We remap the types for ease of use.
    /// let db = db.remap_types::<BEI32, Str>();
    /// db.put(&mut wtxn, &42, "i-am-forty-two")?;
    /// db.put(&mut wtxn, &27, "i-am-twenty-seven")?;
    /// db.put(&mut wtxn, &13, "i-am-thirteen")?;
    /// db.put(&mut wtxn, &521, "i-am-five-hundred-and-twenty-one")?;
    ///
    /// wtxn.commit()?;
    /// # Ok(()) }
    /// ```
    pub fn remap_types<KC2, DC2>(&self) -> EncryptedDatabase<KC2, DC2, C> {
        EncryptedDatabase::new(self.inner.remap_types::<KC2, DC2>())
    }

    /// Change the key codec type of this database, specifying the new codec.
    pub fn remap_key_type<KC2>(&self) -> EncryptedDatabase<KC2, DC, C> {
        self.remap_types::<KC2, DC>()
    }

    /// Change the data codec type of this database, specifying the new codec.
    pub fn remap_data_type<DC2>(&self) -> EncryptedDatabase<KC, DC2, C> {
        self.remap_types::<KC, DC2>()
    }

    /// Wrap the data bytes into a lazy decoder.
    pub fn lazily_decode_data(&self) -> EncryptedDatabase<KC, LazyDecode<DC>, C> {
        self.remap_types::<KC, LazyDecode<DC>>()
    }
}

impl<KC, DC, C> Clone for EncryptedDatabase<KC, DC, C> {
    fn clone(&self) -> EncryptedDatabase<KC, DC, C> {
        *self
    }
}

impl<KC, DC, C> Copy for EncryptedDatabase<KC, DC, C> {}

impl<KC, DC, C> fmt::Debug for EncryptedDatabase<KC, DC, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EncryptedDatabase")
            .field("key_codec", &any::type_name::<KC>())
            .field("data_codec", &any::type_name::<DC>())
            .field("comparator", &any::type_name::<C>())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use heed_types::*;

    use super::*;

    #[test]
    fn put_overwrite() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let env = unsafe { EnvOpenOptions::new().open(dir.path())? };
        let mut txn = env.write_txn()?;
        let db = env.create_database::<Bytes, Bytes>(&mut txn, None)?;

        assert_eq!(db.get(&txn, b"hello").unwrap(), None);

        db.put(&mut txn, b"hello", b"hi").unwrap();
        assert_eq!(db.get(&txn, b"hello").unwrap(), Some(&b"hi"[..]));

        db.put(&mut txn, b"hello", b"bye").unwrap();
        assert_eq!(db.get(&txn, b"hello").unwrap(), Some(&b"bye"[..]));

        Ok(())
    }

    #[test]
    #[cfg(feature = "longer-keys")]
    fn longer_keys() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let env = unsafe { EnvOpenOptions::new().open(dir.path())? };
        let mut txn = env.write_txn()?;
        let db = env.create_database::<Bytes, Bytes>(&mut txn, None)?;

        // Try storing a key larger than 511 bytes (the default if MDB_MAXKEYSIZE is not set)
        let long_key = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut pharetra sit amet aliquam. Sit amet nisl purus in mollis nunc. Eget egestas purus viverra accumsan in nisl nisi scelerisque. Duis ultricies lacus sed turpis tincidunt. Sem nulla pharetra diam sit. Leo vel orci porta non pulvinar. Erat pellentesque adipiscing commodo elit at imperdiet dui. Suspendisse ultrices gravida dictum fusce ut placerat orci nulla. Diam donec adipiscing tristique risus nec feugiat. In fermentum et sollicitudin ac orci. Ut sem nulla pharetra diam sit amet. Aliquam purus sit amet luctus venenatis lectus. Erat pellentesque adipiscing commodo elit at imperdiet dui accumsan. Urna duis convallis convallis tellus id interdum velit laoreet id. Ac feugiat sed lectus vestibulum mattis ullamcorper velit sed. Tincidunt arcu non sodales neque. Habitant morbi tristique senectus et netus et malesuada fames.";

        assert_eq!(db.get(&txn, long_key).unwrap(), None);

        db.put(&mut txn, long_key, b"hi").unwrap();
        assert_eq!(db.get(&txn, long_key).unwrap(), Some(&b"hi"[..]));

        db.put(&mut txn, long_key, b"bye").unwrap();
        assert_eq!(db.get(&txn, long_key).unwrap(), Some(&b"bye"[..]));

        Ok(())
    }
}
