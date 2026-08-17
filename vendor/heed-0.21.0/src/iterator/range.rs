use std::borrow::Cow;
use std::marker;
use std::ops::Bound;

use types::LazyDecode;

use crate::cursor::MoveOperation;
use crate::iteration_method::{IterationMethod, MoveBetweenKeys, MoveThroughDuplicateValues};
use crate::*;

fn move_on_range_end<'txn>(
    cursor: &mut RoCursor<'txn>,
    end_bound: &Bound<Vec<u8>>,
) -> Result<Option<(&'txn [u8], &'txn [u8])>> {
    match end_bound {
        Bound::Included(end) => match cursor.move_on_key_greater_than_or_equal_to(end) {
            Ok(Some((key, data))) if key == &end[..] => Ok(Some((key, data))),
            Ok(_) => cursor.move_on_prev(MoveOperation::NoDup),
            Err(e) => Err(e),
        },
        Bound::Excluded(end) => cursor
            .move_on_key_greater_than_or_equal_to(end)
            .and_then(|_| cursor.move_on_prev(MoveOperation::NoDup)),
        Bound::Unbounded => cursor.move_on_last(MoveOperation::NoDup),
    }
}

fn move_on_range_start<'txn>(
    cursor: &mut RoCursor<'txn>,
    start_bound: &mut Bound<Vec<u8>>,
) -> Result<Option<(&'txn [u8], &'txn [u8])>> {
    match start_bound {
        Bound::Included(start) => cursor.move_on_key_greater_than_or_equal_to(start),
        Bound::Excluded(start) => match cursor.move_on_key_greater_than_or_equal_to(start)? {
            Some((key, _)) if key == start => cursor.move_on_next(MoveOperation::NoDup),
            result => Ok(result),
        },
        Bound::Unbounded => cursor.move_on_first(MoveOperation::NoDup),
    }
}

/// A read-only range iterator structure.
pub struct RoRange<'txn, KC, DC, C = DefaultComparator, IM = MoveThroughDuplicateValues> {
    cursor: RoCursor<'txn>,
    move_on_start: bool,
    start_bound: Bound<Vec<u8>>,
    end_bound: Bound<Vec<u8>>,
    _phantom: marker::PhantomData<(KC, DC, C, IM)>,
}

impl<'txn, KC, DC, C, IM> RoRange<'txn, KC, DC, C, IM> {
    pub(crate) fn new(
        cursor: RoCursor<'txn>,
        start_bound: Bound<Vec<u8>>,
        end_bound: Bound<Vec<u8>>,
    ) -> RoRange<'txn, KC, DC, C, IM> {
        RoRange {
            cursor,
            move_on_start: true,
            start_bound,
            end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Move on the first value of keys, ignoring duplicate values.
    ///
    /// For more info, see [`RoIter::move_between_keys`].
    pub fn move_between_keys(self) -> RoRange<'txn, KC, DC, C, MoveBetweenKeys> {
        RoRange {
            cursor: self.cursor,
            move_on_start: self.move_on_start,
            start_bound: self.start_bound,
            end_bound: self.end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Move through key/values entries and output duplicate values.
    ///
    /// For more info, see [`RoIter::move_through_duplicate_values`].
    pub fn move_through_duplicate_values(
        self,
    ) -> RoRange<'txn, KC, DC, C, MoveThroughDuplicateValues> {
        RoRange {
            cursor: self.cursor,
            move_on_start: self.move_on_start,
            start_bound: self.start_bound,
            end_bound: self.end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the codec types of this iterator, specifying the codecs.
    pub fn remap_types<KC2, DC2>(self) -> RoRange<'txn, KC2, DC2, C, IM> {
        RoRange {
            cursor: self.cursor,
            move_on_start: self.move_on_start,
            start_bound: self.start_bound,
            end_bound: self.end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the key codec type of this iterator, specifying the new codec.
    pub fn remap_key_type<KC2>(self) -> RoRange<'txn, KC2, DC, C, IM> {
        self.remap_types::<KC2, DC>()
    }

    /// Change the data codec type of this iterator, specifying the new codec.
    pub fn remap_data_type<DC2>(self) -> RoRange<'txn, KC, DC2, C, IM> {
        self.remap_types::<KC, DC2>()
    }

    /// Wrap the data bytes into a lazy decoder.
    pub fn lazily_decode_data(self) -> RoRange<'txn, KC, LazyDecode<DC>, C, IM> {
        self.remap_types::<KC, LazyDecode<DC>>()
    }
}

impl<'txn, KC, DC, C, IM> Iterator for RoRange<'txn, KC, DC, C, IM>
where
    KC: BytesDecode<'txn>,
    DC: BytesDecode<'txn>,
    IM: IterationMethod,
    C: Comparator,
{
    type Item = Result<(KC::DItem, DC::DItem)>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.move_on_start {
            self.move_on_start = false;
            move_on_range_start(&mut self.cursor, &mut self.start_bound)
        } else {
            self.cursor.move_on_next(IM::MOVE_OPERATION)
        };

        match result {
            Ok(Some((key, data))) => {
                let must_be_returned = match &self.end_bound {
                    Bound::Included(end) => C::compare(key, end).is_le(),
                    Bound::Excluded(end) => C::compare(key, end).is_lt(),
                    Bound::Unbounded => true,
                };

                if must_be_returned {
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
        let result = if self.move_on_start {
            move_on_range_end(&mut self.cursor, &self.end_bound)
        } else {
            match (self.cursor.current(), move_on_range_end(&mut self.cursor, &self.end_bound)) {
                (Ok(Some((ckey, _))), Ok(Some((key, data)))) if C::compare(ckey, key).is_ne() => {
                    Ok(Some((key, data)))
                }
                (Ok(_), Ok(_)) => Ok(None),
                (Err(e), _) | (_, Err(e)) => Err(e),
            }
        };

        match result {
            Ok(Some((key, data))) => {
                let must_be_returned = match &self.start_bound {
                    Bound::Included(start) => C::compare(key, start).is_ge(),
                    Bound::Excluded(start) => C::compare(key, start).is_gt(),
                    Bound::Unbounded => true,
                };

                if must_be_returned {
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

impl<KC, DC, C, IM> fmt::Debug for RoRange<'_, KC, DC, C, IM> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RoRange").finish()
    }
}

#[cfg(feature = "read-txn-no-tls")]
unsafe impl<KC, DC, C, IM> Send for RoRange<'_, KC, DC, C, IM> {}

/// A read-write range iterator structure.
pub struct RwRange<'txn, KC, DC, C = DefaultComparator, IM = MoveThroughDuplicateValues> {
    cursor: RwCursor<'txn>,
    move_on_start: bool,
    start_bound: Bound<Vec<u8>>,
    end_bound: Bound<Vec<u8>>,
    _phantom: marker::PhantomData<(KC, DC, C, IM)>,
}

impl<'txn, KC, DC, C, IM> RwRange<'txn, KC, DC, C, IM> {
    pub(crate) fn new(
        cursor: RwCursor<'txn>,
        start_bound: Bound<Vec<u8>>,
        end_bound: Bound<Vec<u8>>,
    ) -> RwRange<'txn, KC, DC, C, IM> {
        RwRange {
            cursor,
            move_on_start: true,
            start_bound,
            end_bound,
            _phantom: marker::PhantomData,
        }
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
    /// Please read the safety notes of the [`RwRange::put_current`] method.
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
    pub fn move_between_keys(self) -> RwRange<'txn, KC, DC, C, MoveBetweenKeys> {
        RwRange {
            cursor: self.cursor,
            move_on_start: self.move_on_start,
            start_bound: self.start_bound,
            end_bound: self.end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Move through key/values entries and output duplicate values.
    ///
    /// For more info, see [`RoIter::move_through_duplicate_values`].
    pub fn move_through_duplicate_values(
        self,
    ) -> RwRange<'txn, KC, DC, C, MoveThroughDuplicateValues> {
        RwRange {
            cursor: self.cursor,
            move_on_start: self.move_on_start,
            start_bound: self.start_bound,
            end_bound: self.end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the codec types of this iterator, specifying the codecs.
    pub fn remap_types<KC2, DC2>(self) -> RwRange<'txn, KC2, DC2, C, IM> {
        RwRange {
            cursor: self.cursor,
            move_on_start: self.move_on_start,
            start_bound: self.start_bound,
            end_bound: self.end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the key codec type of this iterator, specifying the new codec.
    pub fn remap_key_type<KC2>(self) -> RwRange<'txn, KC2, DC, C, IM> {
        self.remap_types::<KC2, DC>()
    }

    /// Change the data codec type of this iterator, specifying the new codec.
    pub fn remap_data_type<DC2>(self) -> RwRange<'txn, KC, DC2, C, IM> {
        self.remap_types::<KC, DC2>()
    }

    /// Wrap the data bytes into a lazy decoder.
    pub fn lazily_decode_data(self) -> RwRange<'txn, KC, LazyDecode<DC>, C, IM> {
        self.remap_types::<KC, LazyDecode<DC>>()
    }
}

impl<'txn, KC, DC, C, IM> Iterator for RwRange<'txn, KC, DC, C, IM>
where
    KC: BytesDecode<'txn>,
    DC: BytesDecode<'txn>,
    IM: IterationMethod,
    C: Comparator,
{
    type Item = Result<(KC::DItem, DC::DItem)>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.move_on_start {
            self.move_on_start = false;
            move_on_range_start(&mut self.cursor, &mut self.start_bound)
        } else {
            self.cursor.move_on_next(IM::MOVE_OPERATION)
        };

        match result {
            Ok(Some((key, data))) => {
                let must_be_returned = match self.end_bound {
                    Bound::Included(ref end) => C::compare(key, end).is_le(),
                    Bound::Excluded(ref end) => C::compare(key, end).is_lt(),
                    Bound::Unbounded => true,
                };

                if must_be_returned {
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
        let result = if self.move_on_start {
            move_on_range_end(&mut self.cursor, &self.end_bound)
        } else {
            match (self.cursor.current(), move_on_range_end(&mut self.cursor, &self.end_bound)) {
                (Ok(Some((ckey, _))), Ok(Some((key, data)))) if C::compare(ckey, key).is_ne() => {
                    Ok(Some((key, data)))
                }
                (Ok(_), Ok(_)) => Ok(None),
                (Err(e), _) | (_, Err(e)) => Err(e),
            }
        };

        match result {
            Ok(Some((key, data))) => {
                let must_be_returned = match &self.start_bound {
                    Bound::Included(start) => C::compare(key, start).is_ge(),
                    Bound::Excluded(start) => C::compare(key, start).is_gt(),
                    Bound::Unbounded => true,
                };

                if must_be_returned {
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

impl<KC, DC, C, IM> fmt::Debug for RwRange<'_, KC, DC, C, IM> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RwRange").finish()
    }
}

/// A reverse read-only range iterator structure.
pub struct RoRevRange<'txn, KC, DC, C = DefaultComparator, IM = MoveThroughDuplicateValues> {
    cursor: RoCursor<'txn>,
    move_on_end: bool,
    start_bound: Bound<Vec<u8>>,
    end_bound: Bound<Vec<u8>>,
    _phantom: marker::PhantomData<(KC, DC, C, IM)>,
}

impl<'txn, KC, DC, C, IM> RoRevRange<'txn, KC, DC, C, IM> {
    pub(crate) fn new(
        cursor: RoCursor<'txn>,
        start_bound: Bound<Vec<u8>>,
        end_bound: Bound<Vec<u8>>,
    ) -> RoRevRange<'txn, KC, DC, C, IM> {
        RoRevRange {
            cursor,
            move_on_end: true,
            start_bound,
            end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Move on the first value of keys, ignoring duplicate values.
    ///
    /// For more info, see [`RoIter::move_between_keys`].
    pub fn move_between_keys(self) -> RoRevRange<'txn, KC, DC, C, MoveBetweenKeys> {
        RoRevRange {
            cursor: self.cursor,
            move_on_end: self.move_on_end,
            start_bound: self.start_bound,
            end_bound: self.end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Move through key/values entries and output duplicate values.
    ///
    /// For more info, see [`RoIter::move_through_duplicate_values`].
    pub fn move_through_duplicate_values(
        self,
    ) -> RoRevRange<'txn, KC, DC, C, MoveThroughDuplicateValues> {
        RoRevRange {
            cursor: self.cursor,
            move_on_end: self.move_on_end,
            start_bound: self.start_bound,
            end_bound: self.end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the codec types of this iterator, specifying the codecs.
    pub fn remap_types<KC2, DC2>(self) -> RoRevRange<'txn, KC2, DC2, C, IM> {
        RoRevRange {
            cursor: self.cursor,
            move_on_end: self.move_on_end,
            start_bound: self.start_bound,
            end_bound: self.end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the key codec type of this iterator, specifying the new codec.
    pub fn remap_key_type<KC2>(self) -> RoRevRange<'txn, KC2, DC, C, IM> {
        self.remap_types::<KC2, DC>()
    }

    /// Change the data codec type of this iterator, specifying the new codec.
    pub fn remap_data_type<DC2>(self) -> RoRevRange<'txn, KC, DC2, C, IM> {
        self.remap_types::<KC, DC2>()
    }

    /// Wrap the data bytes into a lazy decoder.
    pub fn lazily_decode_data(self) -> RoRevRange<'txn, KC, LazyDecode<DC>, C, IM> {
        self.remap_types::<KC, LazyDecode<DC>>()
    }
}

impl<'txn, KC, DC, C, IM> Iterator for RoRevRange<'txn, KC, DC, C, IM>
where
    KC: BytesDecode<'txn>,
    DC: BytesDecode<'txn>,
    IM: IterationMethod,
    C: Comparator,
{
    type Item = Result<(KC::DItem, DC::DItem)>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.move_on_end {
            self.move_on_end = false;
            move_on_range_end(&mut self.cursor, &self.end_bound)
        } else {
            self.cursor.move_on_prev(IM::MOVE_OPERATION)
        };

        match result {
            Ok(Some((key, data))) => {
                let must_be_returned = match &self.start_bound {
                    Bound::Included(start) => C::compare(key, start).is_ge(),
                    Bound::Excluded(start) => C::compare(key, start).is_gt(),
                    Bound::Unbounded => true,
                };

                if must_be_returned {
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
        let result = if self.move_on_end {
            move_on_range_start(&mut self.cursor, &mut self.start_bound)
        } else {
            let current = self.cursor.current();
            let start = move_on_range_start(&mut self.cursor, &mut self.start_bound);
            match (current, start) {
                (Ok(Some((ckey, _))), Ok(Some((key, data)))) if C::compare(ckey, key).is_ne() => {
                    Ok(Some((key, data)))
                }
                (Ok(_), Ok(_)) => Ok(None),
                (Err(e), _) | (_, Err(e)) => Err(e),
            }
        };

        match result {
            Ok(Some((key, data))) => {
                let must_be_returned = match &self.end_bound {
                    Bound::Included(end) => C::compare(key, end).is_le(),
                    Bound::Excluded(end) => C::compare(key, end).is_lt(),
                    Bound::Unbounded => true,
                };

                if must_be_returned {
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

impl<KC, DC, C, IM> fmt::Debug for RoRevRange<'_, KC, DC, C, IM> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RoRevRange").finish()
    }
}

#[cfg(feature = "read-txn-no-tls")]
unsafe impl<KC, DC, C, IM> Send for RoRevRange<'_, KC, DC, C, IM> {}

/// A reverse read-write range iterator structure.
pub struct RwRevRange<'txn, KC, DC, C = DefaultComparator, IM = MoveThroughDuplicateValues> {
    cursor: RwCursor<'txn>,
    move_on_end: bool,
    start_bound: Bound<Vec<u8>>,
    end_bound: Bound<Vec<u8>>,
    _phantom: marker::PhantomData<(KC, DC, C, IM)>,
}

impl<'txn, KC, DC, C, IM> RwRevRange<'txn, KC, DC, C, IM> {
    pub(crate) fn new(
        cursor: RwCursor<'txn>,
        start_bound: Bound<Vec<u8>>,
        end_bound: Bound<Vec<u8>>,
    ) -> RwRevRange<'txn, KC, DC, C, IM> {
        RwRevRange {
            cursor,
            move_on_end: true,
            start_bound,
            end_bound,
            _phantom: marker::PhantomData,
        }
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
    /// Please read the safety notes of the [`RwRevRange::put_current`] method.
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
    pub fn move_between_keys(self) -> RwRevRange<'txn, KC, DC, C, MoveBetweenKeys> {
        RwRevRange {
            cursor: self.cursor,
            move_on_end: self.move_on_end,
            start_bound: self.start_bound,
            end_bound: self.end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Move through key/values entries and output duplicate values.
    ///
    /// For more info, see [`RoIter::move_through_duplicate_values`].
    pub fn move_through_duplicate_values(
        self,
    ) -> RwRevRange<'txn, KC, DC, C, MoveThroughDuplicateValues> {
        RwRevRange {
            cursor: self.cursor,
            move_on_end: self.move_on_end,
            start_bound: self.start_bound,
            end_bound: self.end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the codec types of this iterator, specifying the codecs.
    pub fn remap_types<KC2, DC2>(self) -> RwRevRange<'txn, KC2, DC2, C, IM> {
        RwRevRange {
            cursor: self.cursor,
            move_on_end: self.move_on_end,
            start_bound: self.start_bound,
            end_bound: self.end_bound,
            _phantom: marker::PhantomData,
        }
    }

    /// Change the key codec type of this iterator, specifying the new codec.
    pub fn remap_key_type<KC2>(self) -> RwRevRange<'txn, KC2, DC, C, IM> {
        self.remap_types::<KC2, DC>()
    }

    /// Change the data codec type of this iterator, specifying the new codec.
    pub fn remap_data_type<DC2>(self) -> RwRevRange<'txn, KC, DC2, C, IM> {
        self.remap_types::<KC, DC2>()
    }

    /// Wrap the data bytes into a lazy decoder.
    pub fn lazily_decode_data(self) -> RwRevRange<'txn, KC, LazyDecode<DC>, C, IM> {
        self.remap_types::<KC, LazyDecode<DC>>()
    }
}

impl<'txn, KC, DC, C, IM> Iterator for RwRevRange<'txn, KC, DC, C, IM>
where
    KC: BytesDecode<'txn>,
    DC: BytesDecode<'txn>,
    IM: IterationMethod,
    C: Comparator,
{
    type Item = Result<(KC::DItem, DC::DItem)>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.move_on_end {
            self.move_on_end = false;
            move_on_range_end(&mut self.cursor, &self.end_bound)
        } else {
            self.cursor.move_on_prev(IM::MOVE_OPERATION)
        };

        match result {
            Ok(Some((key, data))) => {
                let must_be_returned = match &self.start_bound {
                    Bound::Included(start) => C::compare(key, start).is_ge(),
                    Bound::Excluded(start) => C::compare(key, start).is_gt(),
                    Bound::Unbounded => true,
                };

                if must_be_returned {
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
        let result = if self.move_on_end {
            move_on_range_start(&mut self.cursor, &mut self.start_bound)
        } else {
            let current = self.cursor.current();
            let start = move_on_range_start(&mut self.cursor, &mut self.start_bound);
            match (current, start) {
                (Ok(Some((ckey, _))), Ok(Some((key, data)))) if C::compare(ckey, key).is_ne() => {
                    Ok(Some((key, data)))
                }
                (Ok(_), Ok(_)) => Ok(None),
                (Err(e), _) | (_, Err(e)) => Err(e),
            }
        };

        match result {
            Ok(Some((key, data))) => {
                let must_be_returned = match &self.end_bound {
                    Bound::Included(end) => C::compare(key, end).is_le(),
                    Bound::Excluded(end) => C::compare(key, end).is_lt(),
                    Bound::Unbounded => true,
                };

                if must_be_returned {
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

impl<KC, DC, C, IM> fmt::Debug for RwRevRange<'_, KC, DC, C, IM> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RwRevRange").finish()
    }
}
