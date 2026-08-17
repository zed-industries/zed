/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Checksum caching functionality.

use http_1x::HeaderMap;
use std::sync::{Arc, Mutex};

/// A cache for storing previously calculated checksums.
#[derive(Debug, Clone)]
pub struct ChecksumCache {
    inner: Arc<Mutex<Option<HeaderMap>>>,
}

impl ChecksumCache {
    /// Create a new empty checksum cache.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(None)),
        }
    }

    /// Get a cached checksum if previously calculated
    pub fn get(&self) -> Option<HeaderMap> {
        self.inner.lock().unwrap().clone()
    }

    /// Store a checksum in the cache.
    pub fn set(&self, headers: HeaderMap) {
        let mut inner = self.inner.lock().unwrap();
        *inner = Some(headers);
    }
}

impl Default for ChecksumCache {
    fn default() -> Self {
        Self::new()
    }
}
