//! This example demonstrates how to append a `char` to a cached `Vec<String>` value.
//! It uses the `and_upsert_with` method of `Cache`.

use std::{
    io::{self, Cursor, Read},
    sync::{Arc, RwLock},
};

use moka::{
    ops::compute::{CompResult, Op},
    sync::Cache,
};

/// The type of the cache key.
type Key = i32;

/// The type of the cache value.
///
/// We want to store a raw value `String` for each `i32` key. We are going to append
/// a `char` to the `String` value in the cache.
///
/// Note that we have to wrap the `String` in an `Arc<RwLock<_>>`. We need the `Arc`,
/// an atomic reference counted shared pointer, because `and_try_compute_with` method
/// of `Cache` passes a _clone_ of the value to our closure, instead of passing a
/// `&mut` reference. We do not want to clone the `String` every time we append a
/// `char` to it, so we wrap it in an `Arc`. Then we need the `RwLock` because we
/// mutate the `String` when we append a value to it.
///
/// The reason that `and_try_compute_with` cannot pass a `&mut String` to the closure
/// is because the internal concurrent hash table of `Cache` is a lock free data
/// structure and does not use any mutexes. So it cannot guarantee: (1) the
/// `&mut String` is unique, and (2) it is not accessed concurrently by other
/// threads.
type Value = Arc<RwLock<String>>;

fn main() -> Result<(), tokio::io::Error> {
    let cache: Cache<Key, Value> = Cache::new(100);

    let key = 0;

    // We are going read a byte at a time from a byte string (`[u8; 3]`).
    let mut reader = Cursor::new(b"abc");

    // Read the first char 'a' from the reader, and insert a string "a" to the cache.
    let result = append_to_cached_string(&cache, key, &mut reader)?;
    let CompResult::Inserted(entry) = result else {
        panic!("`Inserted` should be returned: {result:?}");
    };
    assert_eq!(*entry.into_value().read().unwrap(), "a");

    // Read next char 'b' from the reader, and append it the cached string.
    let result = append_to_cached_string(&cache, key, &mut reader)?;
    let CompResult::ReplacedWith(entry) = result else {
        panic!("`ReplacedWith` should be returned: {result:?}");
    };
    assert_eq!(*entry.into_value().read().unwrap(), "ab");

    // Read next char 'c' from the reader, and append it the cached string.
    let result = append_to_cached_string(&cache, key, &mut reader)?;
    let CompResult::ReplacedWith(entry) = result else {
        panic!("`ReplacedWith` should be returned: {result:?}");
    };
    assert_eq!(*entry.into_value().read().unwrap(), "abc");

    // Reading should fail as no more char left.
    let err = append_to_cached_string(&cache, key, &mut reader);
    assert_eq!(
        err.expect_err("An error should be returned").kind(),
        io::ErrorKind::UnexpectedEof
    );

    Ok(())
}

/// Reads a byte from the `reader``, convert it into a `char`, append it to the
/// cached `String` for the given `key`, and returns the resulting cached entry.
///
/// If reading from the `reader` fails with an IO error, it returns the error.
///
/// This method uses cache's `and_try_compute_with` method.
fn append_to_cached_string(
    cache: &Cache<Key, Value>,
    key: Key,
    reader: &mut impl Read,
) -> io::Result<CompResult<Key, Value>> {
    cache.entry(key).and_try_compute_with(|maybe_entry| {
        // Read a char from the reader.
        let mut buf = [0u8];
        let len = reader.read(&mut buf)?;
        if len == 0 {
            // No more char left.
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "No more char left",
            ));
        }
        let char =
            char::from_u32(buf[0] as u32).expect("An ASCII byte should be converted into a char");

        // Check if the entry already exists.
        if let Some(entry) = maybe_entry {
            // The entry exists, append the char to the Vec.
            let v = entry.into_value();
            v.write().unwrap().push(char);
            Ok(Op::Put(v))
        } else {
            // The entry does not exist, insert a new Vec containing
            // the char.
            let v = RwLock::new(String::from(char));
            Ok(Op::Put(Arc::new(v)))
        }
    })
}
