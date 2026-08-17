//-
// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::std_facade::fmt;
use crate::std_facade::Box;
#[cfg(feature = "std")]
use std::collections::HashMap;

use crate::test_runner::errors::TestCaseResult;

/// A key used for the result cache.
///
/// The capabilities of this structure are currently quite limited; all one can
/// do with safe code is get the `&dyn Debug` of the test input value. This may
/// improve in the future, particularly at such a time that specialisation
/// becomes stable.
#[derive(Debug)]
pub struct ResultCacheKey<'a> {
    value: &'a dyn fmt::Debug,
}

impl<'a> ResultCacheKey<'a> {
    pub(crate) fn new(value: &'a dyn fmt::Debug) -> Self {
        Self { value }
    }

    /// Return the test input value as an `&dyn Debug`.
    pub fn value_debug(&self) -> &dyn fmt::Debug {
        self.value
    }
}

/// An object which can cache the outcomes of tests.
pub trait ResultCache {
    /// Convert the given cache key into a `u64` representing that value. The
    /// u64 is used as the key below.
    ///
    /// This is a separate step so that ownership of the key value can be
    /// handed off to user code without needing to be able to clone it.
    fn key(&self, key: &ResultCacheKey) -> u64;
    /// Save `result` as the outcome associated with the test input in `key`.
    ///
    /// `result` is passed as a reference so that the decision to clone depends
    /// on whether the cache actually plans on storing it.
    fn put(&mut self, key: u64, result: &TestCaseResult);
    /// If `put()` has been called with a semantically equivalent `key`, return
    /// the saved result. Otherwise, return `None`.
    fn get(&self, key: u64) -> Option<&TestCaseResult>;
}

#[cfg(feature = "std")]
#[derive(Debug, Default, Clone)]
struct BasicResultCache {
    entries: HashMap<u64, TestCaseResult>,
}

#[cfg(feature = "std")]
impl ResultCache for BasicResultCache {
    fn key(&self, val: &ResultCacheKey) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;
        use std::io::{self, Write};

        struct HashWriter(DefaultHasher);
        impl io::Write for HashWriter {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.0.write(buf);
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let mut hash = HashWriter(DefaultHasher::default());
        write!(hash, "{:?}", val).expect("Debug format returned Err");
        hash.0.finish()
    }

    fn put(&mut self, key: u64, result: &TestCaseResult) {
        self.entries.insert(key, result.clone());
    }

    fn get(&self, key: u64) -> Option<&TestCaseResult> {
        self.entries.get(&key)
    }
}

/// A basic result cache.
///
/// Values are identified by their `Debug` string representation.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn basic_result_cache() -> Box<dyn ResultCache> {
    Box::new(BasicResultCache::default())
}

pub(crate) struct NoOpResultCache;
impl ResultCache for NoOpResultCache {
    fn key(&self, _: &ResultCacheKey) -> u64 {
        0
    }
    fn put(&mut self, _: u64, _: &TestCaseResult) {}
    fn get(&self, _: u64) -> Option<&TestCaseResult> {
        None
    }
}

/// A result cache that does nothing.
///
/// This is the default value of `ProptestConfig.result_cache`.
pub fn noop_result_cache() -> Box<dyn ResultCache> {
    Box::new(NoOpResultCache)
}
