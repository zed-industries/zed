use core::convert::TryInto;
use seahash::SeaHasher;
use sha1::digest::Reset;
use sha1::{Digest, Sha1};
use siphasher::sip::{SipHasher, SipHasher24};
use std::error::Error;
use std::fmt;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};
use std::io::Write;
use std::str::FromStr;
use fnv::FnvHasher;


pub type SipHasher24BuildHasher = BuildHasherDefault<SipHasher24>;


pub struct Sha1Hasher {
    bytes: Vec<u8>,
}

impl Sha1Hasher {
    pub fn new() -> Self {
        Sha1Hasher { bytes: Vec::new() }
    }
}

impl Hasher for Sha1Hasher {
    fn finish(&self) -> u64 {
        let mut sha = Sha1::new();
        sha.write(self.bytes.as_slice());
        let result = sha.finalize();
        u64::from_be_bytes(result[0..8].try_into().unwrap())
    }

    fn write(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }
}


impl Default for Sha1Hasher {
    fn default() -> Self {
        Self::new()
    }
}