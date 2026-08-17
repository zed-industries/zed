// https://github.com/moka-rs/moka/issues/131

use std::{collections::hash_map::DefaultHasher, hash::BuildHasher, sync::Arc};

use moka::sync::SegmentedCache;

fn main() {
    f1_fail();
    f2_pass();
    f3_fail();
    f4_pass();
}

const CAP: u64 = 100;
const SEG: usize = 4;

fn f1_fail() {
    // This should fail because V is not Clone.
    let _cache: SegmentedCache<MyKey, MyValue> = SegmentedCache::new(CAP, SEG);
}

fn f2_pass() {
    let cache: SegmentedCache<MyKey, Arc<MyValue>> = SegmentedCache::new(CAP, SEG);
    let _ = cache.clone();
}

fn f3_fail() {
    // This should fail because S is not Clone.
    let _cache: SegmentedCache<MyKey, Arc<MyValue>, _> =
        SegmentedCache::builder(SEG).build_with_hasher(MyBuildHasher1);
}

fn f4_pass() {
    let cache: SegmentedCache<MyKey, Arc<MyValue>, _> =
        SegmentedCache::builder(SEG).build_with_hasher(MyBuildHasher2);
    let _ = cache.clone();
}

// MyKey is not Clone.
#[derive(Hash, PartialEq, Eq)]
pub struct MyKey(i32);

// MyValue is not Clone.
pub struct MyValue(i32);

// MyBuildHasher1 is not Clone.
pub struct MyBuildHasher1;

impl BuildHasher for MyBuildHasher1 {
    type Hasher = DefaultHasher;

    fn build_hasher(&self) -> Self::Hasher {
        unimplemented!()
    }
}

// MyBuildHasher1 is Clone.
#[derive(Clone)]
pub struct MyBuildHasher2;

impl BuildHasher for MyBuildHasher2 {
    type Hasher = DefaultHasher;

    fn build_hasher(&self) -> Self::Hasher {
        unimplemented!()
    }
}
