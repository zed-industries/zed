// https://github.com/moka-rs/moka/issues/131

use std::{collections::hash_map::DefaultHasher, hash::BuildHasher, sync::Arc};

use moka::future::Cache;

#[tokio::main]
async fn main() {
    f1_fail();
    f2_pass();
    f3_fail();
    f4_pass();
}

const CAP: u64 = 100;

fn f1_fail() {
    // This should fail because V is not Clone.
    let _cache: Cache<MyKey, MyValue> = Cache::new(CAP);
}

fn f2_pass() {
    let cache: Cache<MyKey, Arc<MyValue>> = Cache::new(CAP);
    let _ = cache.clone();
}

fn f3_fail() {
    // This should fail because S is not Clone.
    let _cache: Cache<MyKey, Arc<MyValue>, _> = Cache::builder().build_with_hasher(MyBuildHasher1);
}

fn f4_pass() {
    let cache: Cache<MyKey, Arc<MyValue>, _> = Cache::builder().build_with_hasher(MyBuildHasher2);
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
