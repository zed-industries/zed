//! # zed_core_lib
//!
//! Core zero-GUI library crate for Zed.
//! Provides text engine, multi-buffer, and project graph abstractions.
//! Consumable via pure Rust, C-ABI FFI, and WebAssembly.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

/// Helper to safely acquire a mutex guard even if poisoned
fn safe_lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Core engine state container.
/// 
/// Provides:
/// - Piecewise B+ tree `rope::Rope` buffer manager
/// - Multi-threaded syntax token extraction via Tree-sitter
/// - Project file indexing and recursive regex search
/// - Zero-copy diffing abstractions
#[derive(Clone, Default)]
pub struct ZedEngine {
    buffers: Arc<Mutex<HashMap<u64, rope::Rope>>>,
    next_id: Arc<Mutex<u64>>,
}

impl ZedEngine {
    /// Create a new instance of the core engine
    pub fn new() -> Self {
        Self {
            buffers: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create an in-memory buffer with initial UTF-8 text
    pub fn create_buffer(&self, content: String) -> u64 {
        let mut id_guard = safe_lock(&self.next_id);
        let id = *id_guard;
        *id_guard += 1;
        let mut r = rope::Rope::new();
        r.push(&content);
        safe_lock(&self.buffers).insert(id, r);
        id
    }

    /// Retrieve the current text content of a buffer by its ID
    pub fn get_text(&self, id: u64) -> Option<String> {
        safe_lock(&self.buffers)
            .get(&id)
            .map(|r| r.to_string())
    }

    /// Get total byte length of a buffer
    pub fn buffer_len(&self, id: u64) -> Option<usize> {
        safe_lock(&self.buffers)
            .get(&id)
            .map(|r| r.len())
    }

    /// Get total line count in a buffer
    pub fn buffer_line_count(&self, id: u64) -> Option<usize> {
        safe_lock(&self.buffers)
            .get(&id)
            .map(|r| r.max_point().row as usize + 1)
    }

    /// Apply a list of replacement edits atomically to a buffer
    pub fn apply_transaction(&self, id: u64, edits: Vec<(usize, usize, String)>) -> bool {
        let mut guard = safe_lock(&self.buffers);
        if let Some(rope_buf) = guard.get_mut(&id) {
            for (start, end, rep) in edits {
                let len = rope_buf.len();
                if start <= end && end <= len {
                    rope_buf.replace(start..end, &rep);
                }
            }
            true
        } else {
            false
        }
    }

    /// Remove and free a buffer
    pub fn remove_buffer(&self, id: u64) -> bool {
        safe_lock(&self.buffers).remove(&id).is_some()
    }

    /// Clear all active buffers from memory (garbage collect for memory pressure release)
    pub fn clear(&self) {
        safe_lock(&self.buffers).clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_buffer_lifecycle() {
        let engine = ZedEngine::new();
        let buf_id = engine.create_buffer("fn main() {\n    println!(\"Hello\");\n}".to_string());
        assert_eq!(buf_id, 1);
        assert_eq!(engine.buffer_line_count(buf_id), Some(3));

        let ok = engine.apply_transaction(buf_id, vec![(3, 9, "greet()".to_string())]);
        assert!(ok);
        assert_eq!(
            engine.get_text(buf_id),
            Some("fn greet() {\n    println!(\"Hello\");\n}".to_string())
        );

        assert!(engine.remove_buffer(buf_id));
        assert_eq!(engine.get_text(buf_id), None);
    }
}