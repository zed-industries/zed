use std::borrow::Cow;
use std::marker;

use heed_traits::LexicographicComparator;
use types::LazyDecode;

use crate::cursor::MoveOperation;
use crate::envs::DefaultComparator;
use crate::iteration_method::{IterationMethod, MoveBetweenKeys, MoveThroughDuplicateValues};
use crate::*;

/// Advances `bytes` to the immediate lexicographic successor of equal length, as
/// defined by the `C` comparator. If no successor exists (i.e. `bytes` is the maximal
/// value), it remains unchanged and the function returns `false`. Otherwise, updates
/// `bytes` and returns `true`.
fn advance_prefix<C: LexicographicComparator>(bytes: &mut [u8]) -> bool {
    let mut idx = bytes.len();
    while idx > 0 && bytes[idx - 1] == C::max_elem() {
        idx -= 1;
    }
    if idx == 0 {
        return false;
    }
    bytes[idx - 1] = C::successor(bytes[idx - 1]).expect("Cannot advance byte; this is a bug.");
    for i in (idx + 1)..=bytes.len() {
        bytes[i - 1] = C::min_elem();
    }
    true
}

/// Retreats `bytes` to the immediate lexicographic predecessor of equal length, as
/// defined by the `C` comparator. If no predecessor exists (i.e. `bytes` is the minimum
/// value), it remains unchanged and the function returns `false`. Otherwise, updates
/// `bytes` and returns `true`.
fn retreat_prefix<C: LexicographicComparator>(bytes: &mut [u8]) -> bool {
    let mut idx = bytes.len();
    while idx > 0 && bytes[idx - 1] == C::min_elem() {
        idx -= 1;
    }
    if idx == 0 {
        return false;
    }
    bytes[idx - 1] = C::predecessor(bytes[idx - 1]).expect("Cannot retreat byte; this is a bug.");
    for i in (idx + 1)..=bytes.len() {
        bytes[i - 1] = C::max_elem();
    }
    true
}

fn move_on_prefix_end<'txn, C: LexicographicComparator>(
    cursor: &mut RoCursor<'txn>,
    prefix: &mut [u8],
) -> Result<Option<(&'txn [u8], &'txn [u8])>> {
    if advance_prefix::<C>(prefix) {
        let result = cursor
            .move_on_key_greater_than_or_equal_to(prefix)
            .and_then(|_| cursor.move_on_prev(MoveOperation::NoDup));
        retreat_prefix::<C>(prefix);
        result
    } else {
        // `prefix` is the maximum among all bytes sequence of the same length.
        cursor.move_on_last(MoveOperation::NoDup)
    }
}

/// A read-only prefix iterator structure.
pub struct RoPrefix<'txn, KC, DC, C = DefaultComparator, IM = MoveThroughDuplicateValues> {
    cursor: RoCursor<'txn>,
    prefix: Vec<u8>,
    move_on_first: bool,
    _phantom: marker::PhantomData<(KC, DC, C, IM)>,
}

impl<'txn, KC, DC, C, IM> RoPrefix<'txn, KC, DC, C, IM> {
    pub(crate) fn new(cursor: RoCursor<'txn>, prefix: Vec<u8>) -> RoPrefix<'txn, KC, DC, C, IM> {
        RoPrefix { cursor, prefix, move_on_first: true, _phantom: marker::PhantomData }
    }

    /// Move on the first value of keys, ignoring duplicate values.
    ///
    /// For more info, see [`RoIter::move_between_keys`].
    pub fn move_between_keys(self) -> RoPrefix<'txn, KC, DC, C, MoveBetweenKeys> {
        RoPrefix {
            cursor: self.cursor,
            prefix: self.prefix,
            move_on_first: self.move_on_first,
            _phantom: marker::PhantomData,
        }
    }

    /// Move through key/values entries and output duplicate values.
    ///
    /// For more info, see [`RoIter::move_through_duplicate_values`].
    pub fn move_through_duplicate_values(
        self,
    ) -> RoPrefix<'txn, KC, DC, C, MoveThroughDuplicateValues> {
        RoPrefix {
            cursor: self.cursor,
            prefix: self.prefix,
            move_on_first: self.move_on_first,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the codec types of this iterator, specifying the codecs.
    pub fn remap_types<KC2, DC2>(self) -> RoPrefix<'txn, KC2, DC2, C, IM> {
        RoPrefix {
            cursor: self.cursor,
            prefix: self.prefix,
            move_on_first: self.move_on_first,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the key codec type of this iterator, specifying the new codec.
    pub fn remap_key_type<KC2>(self) -> RoPrefix<'txn, KC2, DC, C, IM> {
        self.remap_types::<KC2, DC>()
    }

    /// Change the data codec type of this iterator, specifying the new codec.
    pub fn remap_data_type<DC2>(self) -> RoPrefix<'txn, KC, DC2, C, IM> {
        self.remap_types::<KC, DC2>()
    }

    /// Wrap the data bytes into a lazy decoder.
    pub fn lazily_decode_data(self) -> RoPrefix<'txn, KC, LazyDecode<DC>, C, IM> {
        self.remap_types::<KC, LazyDecode<DC>>()
    }
}

impl<'txn, KC, DC, C, IM> Iterator for RoPrefix<'txn, KC, DC, C, IM>
where
    KC: BytesDecode<'txn>,
    DC: BytesDecode<'txn>,
    C: LexicographicComparator,
    IM: IterationMethod,
{
    type Item = Result<(KC::DItem, DC::DItem)>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.move_on_first {
            self.move_on_first = false;
            self.cursor.move_on_key_greater_than_or_equal_to(&self.prefix)
        } else {
            self.cursor.move_on_next(IM::MOVE_OPERATION)
        };

        match result {
            Ok(Some((key, data))) => {
                if key.starts_with(&self.prefix) {
                    match (KC::bytes_decode(key), DC::bytes_decode(data)) {
                        (Ok(key), Ok(data)) => Some(Ok((key, data))),
                        (Err(e), _) | (_, Err(e)) => Some(Err(Error::Decoding(e))),
                    }
                } else {
                    None
                }
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }

    fn last(mut self) -> Option<Self::Item> {
        let result = if self.move_on_first {
            move_on_prefix_end::<C>(&mut self.cursor, &mut self.prefix)
        } else {
            match (
                self.cursor.current(),
                move_on_prefix_end::<C>(&mut self.cursor, &mut self.prefix),
            ) {
                (Ok(Some((ckey, _))), Ok(Some((key, data)))) if ckey != key => {
                    Ok(Some((key, data)))
                }
                (Ok(_), Ok(_)) => Ok(None),
                (Err(e), _) | (_, Err(e)) => Err(e),
            }
        };

        match result {
            Ok(Some((key, data))) => {
                if key.starts_with(&self.prefix) {
                    match (KC::bytes_decode(key), DC::bytes_decode(data)) {
                        (Ok(key), Ok(data)) => Some(Ok((key, data))),
                        (Err(e), _) | (_, Err(e)) => Some(Err(Error::Decoding(e))),
                    }
                } else {
                    None
                }
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

impl<KC, DC, C, IM> fmt::Debug for RoPrefix<'_, KC, DC, C, IM> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RoPrefix").finish()
    }
}

#[cfg(feature = "read-txn-no-tls")]
unsafe impl<KC, DC, IM> Send for RoPrefix<'_, KC, DC, IM> {}

/// A read-write prefix iterator structure.
pub struct RwPrefix<'txn, KC, DC, C = DefaultComparator, IM = MoveThroughDuplicateValues> {
    cursor: RwCursor<'txn>,
    prefix: Vec<u8>,
    move_on_first: bool,
    _phantom: marker::PhantomData<(KC, DC, C, IM)>,
}

impl<'txn, KC, DC, C, IM> RwPrefix<'txn, KC, DC, C, IM> {
    pub(crate) fn new(cursor: RwCursor<'txn>, prefix: Vec<u8>) -> RwPrefix<'txn, KC, DC, C, IM> {
        RwPrefix { cursor, prefix, move_on_first: true, _phantom: marker::PhantomData }
    }

    /// Delete the entry the cursor is currently pointing to.
    ///
    /// Returns `true` if the entry was successfully deleted.
    ///
    /// # Safety
    ///
    /// It is _[undefined behavior]_ to keep a reference of a value from this database
    /// while modifying it.
    ///
    /// > [Values returned from the database are valid only until a subsequent update operation,
    /// > or the end of the transaction.](http://www.lmdb.tech/doc/group__mdb.html#structMDB__val)
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn del_current(&mut self) -> Result<bool> {
        self.cursor.del_current()
    }

    /// Write a new value to the current entry.
    ///
    /// The given key **must** be equal to the one this cursor is pointing otherwise the database
    /// can be put into an inconsistent state.
    ///
    /// Returns `true` if the entry was successfully written.
    ///
    /// > This is intended to be used when the new data is the same size as the old.
    /// > Otherwise it will simply perform a delete of the old record followed by an insert.
    ///
    /// # Safety
    ///
    /// It is _[undefined behavior]_ to keep a reference of a value from this database while
    /// modifying it, so you can't use the key/value that comes from the cursor to feed
    /// this function.
    ///
    /// In other words: Transform the key and value that you borrow from this database into an owned
    /// version of them (e.g. `&str` into `String`).
    ///
    /// > [Values returned from the database are valid only until a subsequent update operation,
    /// > or the end of the transaction.](http://www.lmdb.tech/doc/group__mdb.html#structMDB__val)
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn put_current<'a>(
        &mut self,
        key: &'a KC::EItem,
        data: &'a DC::EItem,
    ) -> Result<bool>
    where
        KC: BytesEncode<'a>,
        DC: BytesEncode<'a>,
    {
        let key_bytes: Cow<[u8]> = KC::bytes_encode(key).map_err(Error::Encoding)?;
        let data_bytes: Cow<[u8]> = DC::bytes_encode(data).map_err(Error::Encoding)?;
        self.cursor.put_current(&key_bytes, &data_bytes)
    }

    /// Write a new value to the current entry. The entry is written with the specified flags.
    ///
    /// The given key **must** be equal to the one this cursor is pointing otherwise the database
    /// can be put into an inconsistent state.
    ///
    /// Returns `true` if the entry was successfully written.
    ///
    /// > This is intended to be used when the new data is the same size as the old.
    /// > Otherwise it will simply perform a delete of the old record followed by an insert.
    ///
    /// # Safety
    ///
    /// Please read the safety notes of the [`RwPrefix::put_current`] method.
    pub unsafe fn put_current_reserved_with_flags<'a, F>(
        &mut self,
        flags: PutFlags,
        key: &'a KC::EItem,
        data_size: usize,
        write_func: F,
    ) -> Result<bool>
    where
        KC: BytesEncode<'a>,
        F: FnOnce(&mut ReservedSpace) -> io::Result<()>,
    {
        let key_bytes: Cow<[u8]> = KC::bytes_encode(key).map_err(Error::Encoding)?;
        self.cursor.put_current_reserved_with_flags(flags, &key_bytes, data_size, write_func)
    }

    /// Insert a key-value pair in this database. The entry is written with the specified flags and data codec.
    ///
    /// For more info, see [`RwIter::put_current_with_options`].
    ///
    /// # Safety
    ///
    /// It is _[undefined behavior]_ to keep a reference of a value from this database while
    /// modifying it, so you can't use the key/value that comes from the cursor to feed
    /// this function.
    ///
    /// In other words: Transform the key and value that you borrow from this database into an owned
    /// version of them (e.g. `&str` into `String`).
    ///
    /// > [Values returned from the database are valid only until a subsequent update operation,
    /// > or the end of the transaction.](http://www.lmdb.tech/doc/group__mdb.html#structMDB__val)
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn put_current_with_options<'a, NDC>(
        &mut self,
        flags: PutFlags,
        key: &'a KC::EItem,
        data: &'a NDC::EItem,
    ) -> Result<()>
    where
        KC: BytesEncode<'a>,
        NDC: BytesEncode<'a>,
    {
        let key_bytes: Cow<[u8]> = KC::bytes_encode(key).map_err(Error::Encoding)?;
        let data_bytes: Cow<[u8]> = NDC::bytes_encode(data).map_err(Error::Encoding)?;
        self.cursor.put_current_with_flags(flags, &key_bytes, &data_bytes)
    }

    /// Move on the first value of keys, ignoring duplicate values.
    ///
    /// For more info, see [`RoIter::move_between_keys`].
    pub fn move_between_keys(self) -> RwPrefix<'txn, KC, DC, C, MoveBetweenKeys> {
        RwPrefix {
            cursor: self.cursor,
            prefix: self.prefix,
            move_on_first: self.move_on_first,
            _phantom: marker::PhantomData,
        }
    }

    /// Move through key/values entries and output duplicate values.
    ///
    /// For more info, see [`RoIter::move_through_duplicate_values`].
    pub fn move_through_duplicate_values(
        self,
    ) -> RwPrefix<'txn, KC, DC, C, MoveThroughDuplicateValues> {
        RwPrefix {
            cursor: self.cursor,
            prefix: self.prefix,
            move_on_first: self.move_on_first,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the codec types of this iterator, specifying the codecs.
    pub fn remap_types<KC2, DC2>(self) -> RwPrefix<'txn, KC2, DC2, C, IM> {
        RwPrefix {
            cursor: self.cursor,
            prefix: self.prefix,
            move_on_first: self.move_on_first,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the key codec type of this iterator, specifying the new codec.
    pub fn remap_key_type<KC2>(self) -> RwPrefix<'txn, KC2, DC, C, IM> {
        self.remap_types::<KC2, DC>()
    }

    /// Change the data codec type of this iterator, specifying the new codec.
    pub fn remap_data_type<DC2>(self) -> RwPrefix<'txn, KC, DC2, C, IM> {
        self.remap_types::<KC, DC2>()
    }

    /// Wrap the data bytes into a lazy decoder.
    pub fn lazily_decode_data(self) -> RwPrefix<'txn, KC, LazyDecode<DC>, C, IM> {
        self.remap_types::<KC, LazyDecode<DC>>()
    }
}

impl<'txn, KC, DC, C, IM> Iterator for RwPrefix<'txn, KC, DC, C, IM>
where
    KC: BytesDecode<'txn>,
    DC: BytesDecode<'txn>,
    C: LexicographicComparator,
    IM: IterationMethod,
{
    type Item = Result<(KC::DItem, DC::DItem)>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.move_on_first {
            self.move_on_first = false;
            self.cursor.move_on_key_greater_than_or_equal_to(&self.prefix)
        } else {
            self.cursor.move_on_next(IM::MOVE_OPERATION)
        };

        match result {
            Ok(Some((key, data))) => {
                if key.starts_with(&self.prefix) {
                    match (KC::bytes_decode(key), DC::bytes_decode(data)) {
                        (Ok(key), Ok(data)) => Some(Ok((key, data))),
                        (Err(e), _) | (_, Err(e)) => Some(Err(Error::Decoding(e))),
                    }
                } else {
                    None
                }
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }

    fn last(mut self) -> Option<Self::Item> {
        let result = if self.move_on_first {
            move_on_prefix_end::<C>(&mut self.cursor, &mut self.prefix)
        } else {
            match (
                self.cursor.current(),
                move_on_prefix_end::<C>(&mut self.cursor, &mut self.prefix),
            ) {
                (Ok(Some((ckey, _))), Ok(Some((key, data)))) if ckey != key => {
                    Ok(Some((key, data)))
                }
                (Ok(_), Ok(_)) => Ok(None),
                (Err(e), _) | (_, Err(e)) => Err(e),
            }
        };

        match result {
            Ok(Some((key, data))) => {
                if key.starts_with(&self.prefix) {
                    match (KC::bytes_decode(key), DC::bytes_decode(data)) {
                        (Ok(key), Ok(data)) => Some(Ok((key, data))),
                        (Err(e), _) | (_, Err(e)) => Some(Err(Error::Decoding(e))),
                    }
                } else {
                    None
                }
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

impl<KC, DC, C, IM> fmt::Debug for RwPrefix<'_, KC, DC, C, IM> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RwPrefix").finish()
    }
}

/// A reverse read-only prefix iterator structure.
pub struct RoRevPrefix<'txn, KC, DC, C = DefaultComparator, IM = MoveThroughDuplicateValues> {
    cursor: RoCursor<'txn>,
    prefix: Vec<u8>,
    move_on_last: bool,
    _phantom: marker::PhantomData<(KC, DC, C, IM)>,
}

impl<'txn, KC, DC, C, IM> RoRevPrefix<'txn, KC, DC, C, IM> {
    pub(crate) fn new(cursor: RoCursor<'txn>, prefix: Vec<u8>) -> RoRevPrefix<'txn, KC, DC, C, IM> {
        RoRevPrefix { cursor, prefix, move_on_last: true, _phantom: marker::PhantomData }
    }

    /// Move on the first value of keys, ignoring duplicate values.
    ///
    /// For more info, see [`RoIter::move_between_keys`].
    pub fn move_between_keys(self) -> RoRevPrefix<'txn, KC, DC, C, MoveBetweenKeys> {
        RoRevPrefix {
            cursor: self.cursor,
            prefix: self.prefix,
            move_on_last: self.move_on_last,
            _phantom: marker::PhantomData,
        }
    }

    /// Move through key/values entries and output duplicate values.
    ///
    /// For more info, see [`RoIter::move_through_duplicate_values`].
    pub fn move_through_duplicate_values(
        self,
    ) -> RoRevPrefix<'txn, KC, DC, C, MoveThroughDuplicateValues> {
        RoRevPrefix {
            cursor: self.cursor,
            prefix: self.prefix,
            move_on_last: self.move_on_last,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the codec types of this iterator, specifying the codecs.
    pub fn remap_types<KC2, DC2>(self) -> RoRevPrefix<'txn, KC2, DC2, C, IM> {
        RoRevPrefix {
            cursor: self.cursor,
            prefix: self.prefix,
            move_on_last: self.move_on_last,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the key codec type of this iterator, specifying the new codec.
    pub fn remap_key_type<KC2>(self) -> RoRevPrefix<'txn, KC2, DC, C, IM> {
        self.remap_types::<KC2, DC>()
    }

    /// Change the data codec type of this iterator, specifying the new codec.
    pub fn remap_data_type<DC2>(self) -> RoRevPrefix<'txn, KC, DC2, C, IM> {
        self.remap_types::<KC, DC2>()
    }

    /// Wrap the data bytes into a lazy decoder.
    pub fn lazily_decode_data(self) -> RoRevPrefix<'txn, KC, LazyDecode<DC>, C, IM> {
        self.remap_types::<KC, LazyDecode<DC>>()
    }
}

impl<'txn, KC, DC, C, IM> Iterator for RoRevPrefix<'txn, KC, DC, C, IM>
where
    KC: BytesDecode<'txn>,
    DC: BytesDecode<'txn>,
    C: LexicographicComparator,
    IM: IterationMethod,
{
    type Item = Result<(KC::DItem, DC::DItem)>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.move_on_last {
            self.move_on_last = false;
            move_on_prefix_end::<C>(&mut self.cursor, &mut self.prefix)
        } else {
            self.cursor.move_on_prev(IM::MOVE_OPERATION)
        };

        match result {
            Ok(Some((key, data))) => {
                if key.starts_with(&self.prefix) {
                    match (KC::bytes_decode(key), DC::bytes_decode(data)) {
                        (Ok(key), Ok(data)) => Some(Ok((key, data))),
                        (Err(e), _) | (_, Err(e)) => Some(Err(Error::Decoding(e))),
                    }
                } else {
                    None
                }
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }

    fn last(mut self) -> Option<Self::Item> {
        let result = if self.move_on_last {
            self.cursor.move_on_key_greater_than_or_equal_to(&self.prefix)
        } else {
            let current = self.cursor.current();
            let start = self.cursor.move_on_key_greater_than_or_equal_to(&self.prefix);
            match (current, start) {
                (Ok(Some((ckey, _))), Ok(Some((key, data)))) if ckey != key => {
                    Ok(Some((key, data)))
                }
                (Ok(_), Ok(_)) => Ok(None),
                (Err(e), _) | (_, Err(e)) => Err(e),
            }
        };

        match result {
            Ok(Some((key, data))) => {
                if key.starts_with(&self.prefix) {
                    match (KC::bytes_decode(key), DC::bytes_decode(data)) {
                        (Ok(key), Ok(data)) => Some(Ok((key, data))),
                        (Err(e), _) | (_, Err(e)) => Some(Err(Error::Decoding(e))),
                    }
                } else {
                    None
                }
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

impl<KC, DC, C, IM> fmt::Debug for RoRevPrefix<'_, KC, DC, C, IM> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RoRevPrefix").finish()
    }
}

#[cfg(feature = "read-txn-no-tls")]
unsafe impl<KC, DC, IM> Send for RoRevPrefix<'_, KC, DC, IM> {}

/// A reverse read-write prefix iterator structure.
pub struct RwRevPrefix<'txn, KC, DC, C = DefaultComparator, IM = MoveThroughDuplicateValues> {
    cursor: RwCursor<'txn>,
    prefix: Vec<u8>,
    move_on_last: bool,
    _phantom: marker::PhantomData<(KC, DC, C, IM)>,
}

impl<'txn, KC, DC, C, IM> RwRevPrefix<'txn, KC, DC, C, IM> {
    pub(crate) fn new(cursor: RwCursor<'txn>, prefix: Vec<u8>) -> RwRevPrefix<'txn, KC, DC, C, IM> {
        RwRevPrefix { cursor, prefix, move_on_last: true, _phantom: marker::PhantomData }
    }

    /// Delete the entry the cursor is currently pointing to.
    ///
    /// Returns `true` if the entry was successfully deleted.
    ///
    /// # Safety
    ///
    /// It is _[undefined behavior]_ to keep a reference of a value from this database
    /// while modifying it.
    ///
    /// > [Values returned from the database are valid only until a subsequent update operation,
    /// > or the end of the transaction.](http://www.lmdb.tech/doc/group__mdb.html#structMDB__val)
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn del_current(&mut self) -> Result<bool> {
        self.cursor.del_current()
    }

    /// Write a new value to the current entry.
    ///
    /// The given key **must** be equal to the one this cursor is pointing otherwise the database
    /// can be put into an inconsistent state.
    ///
    /// Returns `true` if the entry was successfully written.
    ///
    /// > This is intended to be used when the new data is the same size as the old.
    /// > Otherwise it will simply perform a delete of the old record followed by an insert.
    ///
    /// # Safety
    ///
    /// It is _[undefined behavior]_ to keep a reference of a value from this database while
    /// modifying it, so you can't use the key/value that comes from the cursor to feed
    /// this function.
    ///
    /// In other words: Transform the key and value that you borrow from this database into an owned
    /// version of them (e.g. `&str` into `String`).
    ///
    /// > [Values returned from the database are valid only until a subsequent update operation,
    /// > or the end of the transaction.](http://www.lmdb.tech/doc/group__mdb.html#structMDB__val)
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn put_current<'a>(
        &mut self,
        key: &'a KC::EItem,
        data: &'a DC::EItem,
    ) -> Result<bool>
    where
        KC: BytesEncode<'a>,
        DC: BytesEncode<'a>,
    {
        let key_bytes: Cow<[u8]> = KC::bytes_encode(key).map_err(Error::Encoding)?;
        let data_bytes: Cow<[u8]> = DC::bytes_encode(data).map_err(Error::Encoding)?;
        self.cursor.put_current(&key_bytes, &data_bytes)
    }

    /// Write a new value to the current entry. The entry is written with the specified flags.
    ///
    /// The given key **must** be equal to the one this cursor is pointing otherwise the database
    /// can be put into an inconsistent state.
    ///
    /// Returns `true` if the entry was successfully written.
    ///
    /// > This is intended to be used when the new data is the same size as the old.
    /// > Otherwise it will simply perform a delete of the old record followed by an insert.
    ///
    /// # Safety
    ///
    /// Please read the safety notes of the [`RwRevPrefix::put_current`] method.
    pub unsafe fn put_current_reserved_with_flags<'a, F>(
        &mut self,
        flags: PutFlags,
        key: &'a KC::EItem,
        data_size: usize,
        write_func: F,
    ) -> Result<bool>
    where
        KC: BytesEncode<'a>,
        F: FnOnce(&mut ReservedSpace) -> io::Result<()>,
    {
        let key_bytes: Cow<[u8]> = KC::bytes_encode(key).map_err(Error::Encoding)?;
        self.cursor.put_current_reserved_with_flags(flags, &key_bytes, data_size, write_func)
    }

    /// Insert a key-value pair in this database. The entry is written with the specified flags and data codec.
    ///
    /// For more info, see [`RwIter::put_current_with_options`].
    ///
    /// # Safety
    ///
    /// It is _[undefined behavior]_ to keep a reference of a value from this database while
    /// modifying it, so you can't use the key/value that comes from the cursor to feed
    /// this function.
    ///
    /// In other words: Transform the key and value that you borrow from this database into an owned
    /// version of them (e.g. `&str` into `String`).
    ///
    /// > [Values returned from the database are valid only until a subsequent update operation,
    /// > or the end of the transaction.](http://www.lmdb.tech/doc/group__mdb.html#structMDB__val)
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn put_current_with_options<'a, NDC>(
        &mut self,
        flags: PutFlags,
        key: &'a KC::EItem,
        data: &'a NDC::EItem,
    ) -> Result<()>
    where
        KC: BytesEncode<'a>,
        NDC: BytesEncode<'a>,
    {
        let key_bytes: Cow<[u8]> = KC::bytes_encode(key).map_err(Error::Encoding)?;
        let data_bytes: Cow<[u8]> = NDC::bytes_encode(data).map_err(Error::Encoding)?;
        self.cursor.put_current_with_flags(flags, &key_bytes, &data_bytes)
    }

    /// Move on the first value of keys, ignoring duplicate values.
    ///
    /// For more info, see [`RoIter::move_between_keys`].
    pub fn move_between_keys(self) -> RwRevPrefix<'txn, KC, DC, C, MoveBetweenKeys> {
        RwRevPrefix {
            cursor: self.cursor,
            prefix: self.prefix,
            move_on_last: self.move_on_last,
            _phantom: marker::PhantomData,
        }
    }

    /// Move through key/values entries and output duplicate values.
    ///
    /// For more info, see [`RoIter::move_through_duplicate_values`].
    pub fn move_through_duplicate_values(
        self,
    ) -> RwRevPrefix<'txn, KC, DC, C, MoveThroughDuplicateValues> {
        RwRevPrefix {
            cursor: self.cursor,
            prefix: self.prefix,
            move_on_last: self.move_on_last,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the codec types of this iterator, specifying the codecs.
    pub fn remap_types<KC2, DC2>(self) -> RwRevPrefix<'txn, KC2, DC2, C, IM> {
        RwRevPrefix {
            cursor: self.cursor,
            prefix: self.prefix,
            move_on_last: self.move_on_last,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the key codec type of this iterator, specifying the new codec.
    pub fn remap_key_type<KC2>(self) -> RwRevPrefix<'txn, KC2, DC, C, IM> {
        self.remap_types::<KC2, DC>()
    }

    /// Change the data codec type of this iterator, specifying the new codec.
    pub fn remap_data_type<DC2>(self) -> RwRevPrefix<'txn, KC, DC2, C, IM> {
        self.remap_types::<KC, DC2>()
    }

    /// Wrap the data bytes into a lazy decoder.
    pub fn lazily_decode_data(self) -> RwRevPrefix<'txn, KC, LazyDecode<DC>, C, IM> {
        self.remap_types::<KC, LazyDecode<DC>>()
    }
}

impl<'txn, KC, DC, C, IM> Iterator for RwRevPrefix<'txn, KC, DC, C, IM>
where
    KC: BytesDecode<'txn>,
    DC: BytesDecode<'txn>,
    C: LexicographicComparator,
    IM: IterationMethod,
{
    type Item = Result<(KC::DItem, DC::DItem)>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.move_on_last {
            self.move_on_last = false;
            move_on_prefix_end::<C>(&mut self.cursor, &mut self.prefix)
        } else {
            self.cursor.move_on_prev(IM::MOVE_OPERATION)
        };

        match result {
            Ok(Some((key, data))) => {
                if key.starts_with(&self.prefix) {
                    match (KC::bytes_decode(key), DC::bytes_decode(data)) {
                        (Ok(key), Ok(data)) => Some(Ok((key, data))),
                        (Err(e), _) | (_, Err(e)) => Some(Err(Error::Decoding(e))),
                    }
                } else {
                    None
                }
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }

    fn last(mut self) -> Option<Self::Item> {
        let result = if self.move_on_last {
            self.cursor.move_on_key_greater_than_or_equal_to(&self.prefix)
        } else {
            let current = self.cursor.current();
            let start = self.cursor.move_on_key_greater_than_or_equal_to(&self.prefix);
            match (current, start) {
                (Ok(Some((ckey, _))), Ok(Some((key, data)))) if ckey != key => {
                    Ok(Some((key, data)))
                }
                (Ok(_), Ok(_)) => Ok(None),
                (Err(e), _) | (_, Err(e)) => Err(e),
            }
        };

        match result {
            Ok(Some((key, data))) => {
                if key.starts_with(&self.prefix) {
                    match (KC::bytes_decode(key), DC::bytes_decode(data)) {
                        (Ok(key), Ok(data)) => Some(Ok((key, data))),
                        (Err(e), _) | (_, Err(e)) => Some(Err(Error::Decoding(e))),
                    }
                } else {
                    None
                }
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

impl<KC, DC, C, IM> fmt::Debug for RwRevPrefix<'_, KC, DC, C, IM> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RwRevPrefix").finish()
    }
}
