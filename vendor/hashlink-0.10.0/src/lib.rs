#![no_std]
extern crate alloc;

pub mod linked_hash_map;
pub mod linked_hash_set;
pub mod lru_cache;
#[cfg(feature = "serde_impl")]
pub mod serde;

use core::hash::{BuildHasher, Hasher};

pub use linked_hash_map::LinkedHashMap;
pub use linked_hash_set::LinkedHashSet;
pub use lru_cache::LruCache;

/// Default hash builder, matches hashbrown's default hasher.
///
/// See [`DefaultHasher`] for more details.
#[derive(Clone, Copy, Default, Debug)]
pub struct DefaultHashBuilder(hashbrown::DefaultHashBuilder);

impl BuildHasher for DefaultHashBuilder {
    type Hasher = DefaultHasher;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        DefaultHasher(self.0.build_hasher())
    }
}

/// Default hasher, as selected by hashbrown.
#[derive(Clone)]
pub struct DefaultHasher(<hashbrown::DefaultHashBuilder as BuildHasher>::Hasher);

impl Hasher for DefaultHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.0.write_u8(i)
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.0.write_u16(i)
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.0.write_u32(i)
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0.write_u64(i)
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.0.write_u128(i)
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.0.write_usize(i)
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }

    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.0.write_i8(i)
    }

    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.0.write_i16(i)
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.0.write_i32(i)
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.0.write_i64(i)
    }

    #[inline]
    fn write_i128(&mut self, i: i128) {
        self.0.write_i128(i)
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.0.write_isize(i)
    }
}
