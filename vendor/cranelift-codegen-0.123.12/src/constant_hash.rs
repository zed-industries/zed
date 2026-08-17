//! Runtime support for precomputed constant hash tables.
//!
//! The shared module with the same name can generate constant hash tables using open addressing
//! and quadratic probing.
//!
//! The hash tables are arrays that are guaranteed to:
//!
//! - Have a power-of-two size.
//! - Contain at least one empty slot.
//!
//! This module provides runtime support for lookups in these tables.

// Re-export entities from constant_hash for simplicity of use.
pub use cranelift_codegen_shared::constant_hash::*;

/// Trait that must be implemented by the entries in a constant hash table.
pub trait Table<K: Copy + Eq> {
    /// Get the number of entries in this table which must be a power of two.
    fn len(&self) -> usize;

    /// Get the key corresponding to the entry at `idx`, or `None` if the entry is empty.
    /// The `idx` must be in range.
    fn key(&self, idx: usize) -> Option<K>;
}

/// Look for `key` in `table`.
///
/// The provided `hash` value must have been computed from `key` using the same hash function that
/// was used to construct the table.
///
/// Returns `Ok(idx)` with the table index containing the found entry, or `Err(idx)` with the empty
/// sentinel entry if no entry could be found.
pub fn probe<K: Copy + Eq, T: Table<K> + ?Sized>(
    table: &T,
    key: K,
    hash: usize,
) -> Result<usize, usize> {
    debug_assert!(table.len().is_power_of_two());
    let mask = table.len() - 1;

    let mut idx = hash;
    let mut step = 0;

    loop {
        idx &= mask;

        match table.key(idx) {
            None => return Err(idx),
            Some(k) if k == key => return Ok(idx),
            _ => {}
        }

        // Quadratic probing.
        step += 1;

        // When `table.len()` is a power of two, it can be proven that `idx` will visit all
        // entries. This means that this loop will always terminate if the hash table has even
        // one unused entry.
        debug_assert!(step < table.len());
        idx += step;
    }
}
