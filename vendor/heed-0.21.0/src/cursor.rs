use std::ops::{Deref, DerefMut};
use std::{marker, mem, ptr};

use crate::mdb::error::mdb_result;
use crate::mdb::ffi;
use crate::*;

pub struct RoCursor<'txn> {
    cursor: *mut ffi::MDB_cursor,
    _marker: marker::PhantomData<&'txn ()>,
}

impl<'txn> RoCursor<'txn> {
    // TODO should I ask for a &mut RoTxn, here?
    pub(crate) fn new(txn: &'txn RoTxn, dbi: ffi::MDB_dbi) -> Result<RoCursor<'txn>> {
        let mut cursor: *mut ffi::MDB_cursor = ptr::null_mut();
        let mut txn = txn.txn.unwrap();
        unsafe { mdb_result(ffi::mdb_cursor_open(txn.as_mut(), dbi, &mut cursor))? }
        Ok(RoCursor { cursor, _marker: marker::PhantomData })
    }

    pub fn current(&mut self) -> Result<Option<(&'txn [u8], &'txn [u8])>> {
        let mut key_val = mem::MaybeUninit::uninit();
        let mut data_val = mem::MaybeUninit::uninit();

        // Move the cursor on the first database key
        let result = unsafe {
            mdb_result(ffi::mdb_cursor_get(
                self.cursor,
                key_val.as_mut_ptr(),
                data_val.as_mut_ptr(),
                ffi::cursor_op::MDB_GET_CURRENT,
            ))
        };

        match result {
            Ok(()) => {
                let key = unsafe { crate::from_val(key_val.assume_init()) };
                let data = unsafe { crate::from_val(data_val.assume_init()) };
                Ok(Some((key, data)))
            }
            Err(e) if e.not_found() => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn move_on_first(&mut self, op: MoveOperation) -> Result<Option<(&'txn [u8], &'txn [u8])>> {
        let mut key_val = mem::MaybeUninit::uninit();
        let mut data_val = mem::MaybeUninit::uninit();

        let flag = match op {
            MoveOperation::Any => ffi::cursor_op::MDB_FIRST,
            MoveOperation::Dup => {
                unsafe {
                    mdb_result(ffi::mdb_cursor_get(
                        self.cursor,
                        ptr::null_mut(),
                        &mut ffi::MDB_val { mv_size: 0, mv_data: ptr::null_mut() },
                        ffi::cursor_op::MDB_FIRST_DUP,
                    ))?
                };
                ffi::cursor_op::MDB_GET_CURRENT
            }
            MoveOperation::NoDup => ffi::cursor_op::MDB_FIRST,
        };

        // Move the cursor on the first database key
        let result = unsafe {
            mdb_result(ffi::mdb_cursor_get(
                self.cursor,
                key_val.as_mut_ptr(),
                data_val.as_mut_ptr(),
                flag,
            ))
        };

        match result {
            Ok(()) => {
                let key = unsafe { crate::from_val(key_val.assume_init()) };
                let data = unsafe { crate::from_val(data_val.assume_init()) };
                Ok(Some((key, data)))
            }
            Err(e) if e.not_found() => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn move_on_last(&mut self, op: MoveOperation) -> Result<Option<(&'txn [u8], &'txn [u8])>> {
        let mut key_val = mem::MaybeUninit::uninit();
        let mut data_val = mem::MaybeUninit::uninit();

        let flag = match op {
            MoveOperation::Any => ffi::cursor_op::MDB_LAST,
            MoveOperation::Dup => {
                unsafe {
                    mdb_result(ffi::mdb_cursor_get(
                        self.cursor,
                        ptr::null_mut(),
                        &mut ffi::MDB_val { mv_size: 0, mv_data: ptr::null_mut() },
                        ffi::cursor_op::MDB_LAST_DUP,
                    ))?
                };
                ffi::cursor_op::MDB_GET_CURRENT
            }
            MoveOperation::NoDup => ffi::cursor_op::MDB_LAST,
        };

        // Move the cursor on the first database key
        let result = unsafe {
            mdb_result(ffi::mdb_cursor_get(
                self.cursor,
                key_val.as_mut_ptr(),
                data_val.as_mut_ptr(),
                flag,
            ))
        };

        match result {
            Ok(()) => {
                let key = unsafe { crate::from_val(key_val.assume_init()) };
                let data = unsafe { crate::from_val(data_val.assume_init()) };
                Ok(Some((key, data)))
            }
            Err(e) if e.not_found() => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn move_on_key(&mut self, key: &[u8]) -> Result<bool> {
        let mut key_val = unsafe { crate::into_val(key) };

        // Move the cursor to the specified key
        let result = unsafe {
            mdb_result(ffi::mdb_cursor_get(
                self.cursor,
                &mut key_val,
                &mut ffi::MDB_val { mv_size: 0, mv_data: ptr::null_mut() },
                ffi::cursor_op::MDB_SET,
            ))
        };

        match result {
            Ok(()) => Ok(true),
            Err(e) if e.not_found() => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    pub fn move_on_key_greater_than_or_equal_to(
        &mut self,
        key: &[u8],
    ) -> Result<Option<(&'txn [u8], &'txn [u8])>> {
        let mut key_val = unsafe { crate::into_val(key) };
        let mut data_val = mem::MaybeUninit::uninit();

        // Move the cursor to the specified key
        let result = unsafe {
            mdb_result(ffi::mdb_cursor_get(
                self.cursor,
                &mut key_val,
                data_val.as_mut_ptr(),
                ffi::cursor_op::MDB_SET_RANGE,
            ))
        };

        match result {
            Ok(()) => {
                let key = unsafe { crate::from_val(key_val) };
                let data = unsafe { crate::from_val(data_val.assume_init()) };
                Ok(Some((key, data)))
            }
            Err(e) if e.not_found() => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn move_on_prev(&mut self, op: MoveOperation) -> Result<Option<(&'txn [u8], &'txn [u8])>> {
        let mut key_val = mem::MaybeUninit::uninit();
        let mut data_val = mem::MaybeUninit::uninit();

        let flag = match op {
            MoveOperation::Any => ffi::cursor_op::MDB_PREV,
            MoveOperation::Dup => ffi::cursor_op::MDB_PREV_DUP,
            MoveOperation::NoDup => ffi::cursor_op::MDB_PREV_NODUP,
        };

        // Move the cursor to the previous non-dup key
        let result = unsafe {
            mdb_result(ffi::mdb_cursor_get(
                self.cursor,
                key_val.as_mut_ptr(),
                data_val.as_mut_ptr(),
                flag,
            ))
        };

        match result {
            Ok(()) => {
                let key = unsafe { crate::from_val(key_val.assume_init()) };
                let data = unsafe { crate::from_val(data_val.assume_init()) };
                Ok(Some((key, data)))
            }
            Err(e) if e.not_found() => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn move_on_next(&mut self, op: MoveOperation) -> Result<Option<(&'txn [u8], &'txn [u8])>> {
        let mut key_val = mem::MaybeUninit::uninit();
        let mut data_val = mem::MaybeUninit::uninit();

        let flag = match op {
            MoveOperation::Any => ffi::cursor_op::MDB_NEXT,
            MoveOperation::Dup => ffi::cursor_op::MDB_NEXT_DUP,
            MoveOperation::NoDup => ffi::cursor_op::MDB_NEXT_NODUP,
        };

        // Move the cursor to the next non-dup key
        let result = unsafe {
            mdb_result(ffi::mdb_cursor_get(
                self.cursor,
                key_val.as_mut_ptr(),
                data_val.as_mut_ptr(),
                flag,
            ))
        };

        match result {
            Ok(()) => {
                let key = unsafe { crate::from_val(key_val.assume_init()) };
                let data = unsafe { crate::from_val(data_val.assume_init()) };
                Ok(Some((key, data)))
            }
            Err(e) if e.not_found() => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

impl Drop for RoCursor<'_> {
    fn drop(&mut self) {
        unsafe { ffi::mdb_cursor_close(self.cursor) }
    }
}

pub struct RwCursor<'txn> {
    cursor: RoCursor<'txn>,
}

impl<'txn> RwCursor<'txn> {
    pub(crate) fn new(txn: &'txn RwTxn, dbi: ffi::MDB_dbi) -> Result<RwCursor<'txn>> {
        Ok(RwCursor { cursor: RoCursor::new(txn, dbi)? })
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
        // Delete the current entry
        let result = mdb_result(ffi::mdb_cursor_del(self.cursor.cursor, 0));

        match result {
            Ok(()) => Ok(true),
            Err(e) if e.not_found() => Ok(false),
            Err(e) => Err(e.into()),
        }
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
    pub unsafe fn put_current(&mut self, key: &[u8], data: &[u8]) -> Result<bool> {
        let mut key_val = crate::into_val(key);
        let mut data_val = crate::into_val(data);

        // Modify the pointed data
        let result = mdb_result(ffi::mdb_cursor_put(
            self.cursor.cursor,
            &mut key_val,
            &mut data_val,
            ffi::MDB_CURRENT,
        ));

        match result {
            Ok(()) => Ok(true),
            Err(e) if e.not_found() => Ok(false),
            Err(e) => Err(e.into()),
        }
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
    /// Please read the safety notes of the [`Self::put_current`] method.
    pub unsafe fn put_current_reserved_with_flags<F>(
        &mut self,
        flags: PutFlags,
        key: &[u8],
        data_size: usize,
        write_func: F,
    ) -> Result<bool>
    where
        F: FnOnce(&mut ReservedSpace) -> io::Result<()>,
    {
        let mut key_val = crate::into_val(key);
        let mut reserved = ffi::reserve_size_val(data_size);
        let flags = ffi::MDB_RESERVE | flags.bits();

        let result =
            mdb_result(ffi::mdb_cursor_put(self.cursor.cursor, &mut key_val, &mut reserved, flags));

        let found = match result {
            Ok(()) => true,
            Err(e) if e.not_found() => false,
            Err(e) => return Err(e.into()),
        };

        let mut reserved = ReservedSpace::from_val(reserved);
        write_func(&mut reserved)?;

        if reserved.remaining() == 0 {
            Ok(found)
        } else {
            Err(io::Error::from(io::ErrorKind::UnexpectedEof).into())
        }
    }

    /// Append the given key/value pair to the end of the database.
    ///
    /// If a key is inserted that is less than any previous key a `KeyExist` error
    /// is returned and the key is not inserted into the database.
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
    pub unsafe fn put_current_with_flags(
        &mut self,
        flags: PutFlags,
        key: &[u8],
        data: &[u8],
    ) -> Result<()> {
        let mut key_val = crate::into_val(key);
        let mut data_val = crate::into_val(data);

        // Modify the pointed data
        let result = mdb_result(ffi::mdb_cursor_put(
            self.cursor.cursor,
            &mut key_val,
            &mut data_val,
            flags.bits(),
        ));

        result.map_err(Into::into)
    }
}

impl<'txn> Deref for RwCursor<'txn> {
    type Target = RoCursor<'txn>;

    fn deref(&self) -> &Self::Target {
        &self.cursor
    }
}

impl DerefMut for RwCursor<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cursor
    }
}

/// The way the `Iterator::next/prev` method behaves towards DUP data.
#[derive(Debug, Clone, Copy)]
pub enum MoveOperation {
    /// Move on the next/prev entry, wether it's the same key or not.
    Any,
    /// Move on the next/prev data of the current key.
    Dup,
    /// Move on the next/prev entry which is the next/prev key.
    /// Skip the multiple values of the current key.
    NoDup,
}
