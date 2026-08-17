//! Common test utilities for fork integration tests
//!
//! This module provides shared helper functions for integration tests,
//! reducing code duplication across test files.

#![allow(dead_code)]

use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::{Duration, Instant},
};

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Get a unique test directory with counter to avoid conflicts
pub fn get_unique_test_dir(test_name: &str) -> PathBuf {
    let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    env::temp_dir().join(format!("fork_test_{}_{}", test_name, counter))
}

/// Get a simple test directory without counter
pub fn get_test_dir(prefix: &str) -> PathBuf {
    env::temp_dir().join(format!("fork_test_{}", prefix))
}

/// Setup a test directory (creates and cleans if exists)
pub fn setup_test_dir(path: PathBuf) -> PathBuf {
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("Failed to create test directory");
    path
}

/// Wait for a file to exist with timeout
pub fn wait_for_file(path: &Path, timeout_ms: u64) -> bool {
    let start = Instant::now();
    while start.elapsed().as_millis() < timeout_ms as u128 {
        if path.exists() {
            return true;
        }
        thread::sleep(Duration::from_millis(10));
    }
    false
}

/// Cleanup a test directory
pub fn cleanup_test_dir(path: &Path) {
    let _ = fs::remove_dir_all(path);
}
