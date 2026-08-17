use std::ptr;

pub use ffi::{
    mdb_cursor_close, mdb_cursor_del, mdb_cursor_get, mdb_cursor_open, mdb_cursor_put,
    mdb_dbi_open, mdb_del, mdb_drop, mdb_env_close, mdb_env_copyfd2, mdb_env_create,
    mdb_env_get_fd, mdb_env_get_flags, mdb_env_get_maxkeysize, mdb_env_info, mdb_env_open,
    mdb_env_set_flags, mdb_env_set_mapsize, mdb_env_set_maxdbs, mdb_env_set_maxreaders,
    mdb_env_stat, mdb_env_sync, mdb_filehandle_t, mdb_get, mdb_put, mdb_reader_check,
    mdb_set_compare, mdb_stat, mdb_txn_abort, mdb_txn_begin, mdb_txn_commit, mdb_version,
    MDB_cursor, MDB_dbi, MDB_env, MDB_stat, MDB_txn, MDB_val, MDB_CP_COMPACT, MDB_CURRENT,
    MDB_RDONLY, MDB_RESERVE,
};
#[cfg(master3)]
pub use ffi::{mdb_env_set_encrypt, MDB_enc_func};
#[cfg(master3)]
use lmdb_master3_sys as ffi;
#[cfg(not(master3))]
use lmdb_master_sys as ffi;

pub mod cursor_op {
    use super::ffi::{self, MDB_cursor_op};

    pub const MDB_FIRST: MDB_cursor_op = ffi::MDB_FIRST;
    pub const MDB_FIRST_DUP: MDB_cursor_op = ffi::MDB_FIRST_DUP;
    pub const MDB_LAST: MDB_cursor_op = ffi::MDB_LAST;
    pub const MDB_LAST_DUP: MDB_cursor_op = ffi::MDB_LAST_DUP;
    pub const MDB_SET_RANGE: MDB_cursor_op = ffi::MDB_SET_RANGE;
    pub const MDB_SET: MDB_cursor_op = ffi::MDB_SET;
    pub const MDB_PREV: MDB_cursor_op = ffi::MDB_PREV;
    pub const MDB_PREV_NODUP: MDB_cursor_op = ffi::MDB_PREV_NODUP;
    pub const MDB_PREV_DUP: MDB_cursor_op = ffi::MDB_PREV_DUP;
    pub const MDB_NEXT: MDB_cursor_op = ffi::MDB_NEXT;
    pub const MDB_NEXT_NODUP: MDB_cursor_op = ffi::MDB_NEXT_NODUP;
    pub const MDB_NEXT_DUP: MDB_cursor_op = ffi::MDB_NEXT_DUP;
    pub const MDB_GET_CURRENT: MDB_cursor_op = ffi::MDB_GET_CURRENT;
}

pub fn reserve_size_val(size: usize) -> ffi::MDB_val {
    ffi::MDB_val { mv_size: size, mv_data: ptr::null_mut() }
}

pub unsafe fn into_val(value: &[u8]) -> ffi::MDB_val {
    ffi::MDB_val { mv_data: value.as_ptr() as *mut libc::c_void, mv_size: value.len() }
}

pub unsafe fn from_val<'a>(value: ffi::MDB_val) -> &'a [u8] {
    std::slice::from_raw_parts(value.mv_data as *const u8, value.mv_size)
}
