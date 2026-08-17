use std::mem::MaybeUninit;

use libc::c_char;

#[repr(C)]
pub struct HashTable<K, V, const IS_MAP: bool> {
    num_buckets: usize,
    capacity: usize,
    occupied: *mut u8,
    keys: *mut MaybeUninit<K>,
    vals: *mut MaybeUninit<V>,
}

type Str = *const c_char;
pub type HashMap<K, V> = HashTable<K, V, true>;
pub type HashSet<K> = HashTable<K, u8, false>;

impl<K, V, const IS_MAP: bool> HashTable<K, V, IS_MAP>
{
    pub fn new() -> Self {
        HashTable {
            num_buckets: 0,
            capacity: 0,
            occupied: std::ptr::null_mut(),
            keys: std::ptr::null_mut(),
            vals: std::ptr::null_mut(),
        }
    }
}

// with alias
type MySet = HashTable<Str, c_char, false>;

#[no_mangle]
pub extern "C" fn new_set() -> *mut MySet {
    Box::into_raw(Box::new(HashSet::new()))
}

type SetCallback = unsafe extern "C" fn(key: Str);

#[no_mangle]
pub unsafe extern "C" fn set_for_each(set: *const MySet, callback: SetCallback) {
    todo!();
}

// without alias
#[no_mangle]
pub extern "C" fn new_map() -> *mut HashTable<Str, u64, true> {
    Box::into_raw(Box::new(HashMap::new()))
}

type MapCallback = unsafe extern "C" fn(key: Str, val: u64);

#[no_mangle]
pub unsafe extern "C" fn map_for_each(map: *const HashTable<Str, u64, true>, callback: MapCallback) {
    todo!();
}
