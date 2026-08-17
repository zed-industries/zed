use siphasher::sip::SipHasher;
use siphasher::sip128::{Hasher128, SipHasher as SipHasher128};
use std::hash::{Hash, Hasher};
use crate::minhash::Sha1Hasher;

pub trait SimHasher: Sized {
    type T;
    fn hash<U>(&self, item: &U) -> Self::T
    where
        Self: Sized,
        U: Hash;
}

pub struct SimSipHasher64 {
    key1: u64,
    key2: u64,
}

impl SimSipHasher64 {
    pub fn new(key1: u64, key2: u64) -> Self {
        SimSipHasher64 {
            key1,
            key2,
        }
    }
}

impl SimHasher for SimSipHasher64 {
    type T = u64;

    fn hash<U>(&self, item: &U) -> Self::T
    where
        Self: Sized,
        U: Hash,
    {
        let mut sip = SipHasher::new_with_keys(self.key1, self.key2);
        item.hash(&mut sip);
        sip.finish()
    }
}

pub struct ShaHasher64 {}

impl ShaHasher64 {
    pub fn new() -> Self {
        ShaHasher64 {}
    }
}

impl SimHasher for ShaHasher64 {
    type T = u64;

    fn hash<U>(&self, item: &U) -> Self::T
    where
        Self: Sized,
        U: Hash,
    {
        let mut hasher = Sha1Hasher::new();
        item.hash(&mut hasher);
        hasher.finish()
    }
}

pub struct SimSipHasher128 {
    key1: u64,
    key2: u64,
}

impl SimSipHasher128 {
    pub fn new(key1: u64, key2: u64) -> Self {
        SimSipHasher128 {
            key1,
            key2,
        }
    }
}

impl SimHasher for SimSipHasher128 {
    type T = u128;

    fn hash<U>(&self, item: &U) -> Self::T
    where
        Self: Sized,
        U: Hash,
    {
        let mut sip = SipHasher128::new_with_keys(self.key1, self.key2);
        item.hash(&mut sip);
        sip.finish128().as_u128()
    }
}
