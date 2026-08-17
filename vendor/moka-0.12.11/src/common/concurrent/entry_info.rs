use std::sync::atomic::{self, AtomicBool, AtomicU16, AtomicU32, Ordering};

use super::{AccessTime, KeyHash};
use crate::common::time::{AtomicInstant, Instant};

#[derive(Debug)]
pub(crate) struct EntryInfo<K> {
    key_hash: KeyHash<K>,
    /// `is_admitted` indicates that the entry has been admitted to the cache. When
    /// `false`, it means the entry is _temporary_ admitted to the cache or evicted
    /// from the cache (so it should not have LRU nodes).
    is_admitted: AtomicBool,
    /// `entry_gen` (entry generation) is incremented every time the entry is updated
    /// in the concurrent hash table.
    entry_gen: AtomicU16,
    /// `policy_gen` (policy generation) is incremented every time entry's `WriteOp`
    /// is applied to the cache policies including the access-order queue (the LRU
    /// deque).
    policy_gen: AtomicU16,
    last_accessed: AtomicInstant,
    last_modified: AtomicInstant,
    expiration_time: AtomicInstant,
    policy_weight: AtomicU32,
}

impl<K> EntryInfo<K> {
    #[inline]
    pub(crate) fn new(key_hash: KeyHash<K>, timestamp: Instant, policy_weight: u32) -> Self {
        #[cfg(feature = "unstable-debug-counters")]
        super::debug_counters::InternalGlobalDebugCounters::entry_info_created();

        Self {
            key_hash,
            is_admitted: AtomicBool::default(),
            // `entry_gen` starts at 1 and `policy_gen` start at 0.
            entry_gen: AtomicU16::new(1),
            policy_gen: AtomicU16::new(0),
            last_accessed: AtomicInstant::new(timestamp),
            last_modified: AtomicInstant::new(timestamp),
            expiration_time: AtomicInstant::default(),
            policy_weight: AtomicU32::new(policy_weight),
        }
    }

    #[inline]
    pub(crate) fn key_hash(&self) -> &KeyHash<K> {
        &self.key_hash
    }

    #[inline]
    pub(crate) fn is_admitted(&self) -> bool {
        self.is_admitted.load(Ordering::Acquire)
    }

    #[inline]
    pub(crate) fn set_admitted(&self, value: bool) {
        self.is_admitted.store(value, Ordering::Release);
    }

    /// Returns `true` if the `ValueEntry` having this `EntryInfo` is dirty.
    ///
    /// Dirty means that the entry has been updated in the concurrent hash table but
    /// not yet in the cache policies such as access-order queue.
    #[inline]
    pub(crate) fn is_dirty(&self) -> bool {
        let result =
            self.entry_gen.load(Ordering::Relaxed) != self.policy_gen.load(Ordering::Relaxed);
        atomic::fence(Ordering::Acquire);
        result
    }

    #[inline]
    pub(crate) fn entry_gen(&self) -> u16 {
        self.entry_gen.load(Ordering::Acquire)
    }

    /// Increments the entry generation and returns the new value.
    #[inline]
    pub(crate) fn incr_entry_gen(&self) -> u16 {
        // NOTE: This operation wraps around on overflow.
        let prev = self.entry_gen.fetch_add(1, Ordering::AcqRel);
        // Need to add `1` to the previous value to get the current value.
        prev.wrapping_add(1)
    }

    /// Sets the policy generation to the given value.
    #[inline]
    pub(crate) fn set_policy_gen(&self, value: u16) {
        let g = &self.policy_gen;
        loop {
            let current = g.load(Ordering::Acquire);

            // Do not set the given value if it is smaller than the current value of
            // `policy_gen`. Note that the current value may have been wrapped
            // around. If the value is much larger than the current value, it is
            // likely that the value of `policy_gen` has been wrapped around.
            if current >= value || value.wrapping_sub(current) > u16::MAX / 2 {
                break;
            }

            // Try to set the value.
            if g.compare_exchange_weak(current, value, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                break;
            }
        }
    }

    #[inline]
    pub(crate) fn policy_weight(&self) -> u32 {
        self.policy_weight.load(Ordering::Acquire)
    }

    pub(crate) fn set_policy_weight(&self, size: u32) {
        self.policy_weight.store(size, Ordering::Release);
    }

    #[inline]
    pub(crate) fn expiration_time(&self) -> Option<Instant> {
        self.expiration_time.instant()
    }

    pub(crate) fn set_expiration_time(&self, time: Option<Instant>) {
        if let Some(t) = time {
            self.expiration_time.set_instant(t);
        } else {
            self.expiration_time.clear();
        }
    }
}

#[cfg(feature = "unstable-debug-counters")]
impl<K> Drop for EntryInfo<K> {
    fn drop(&mut self) {
        super::debug_counters::InternalGlobalDebugCounters::entry_info_dropped();
    }
}

impl<K> AccessTime for EntryInfo<K> {
    #[inline]
    fn last_accessed(&self) -> Option<Instant> {
        self.last_accessed.instant()
    }

    #[inline]
    fn set_last_accessed(&self, timestamp: Instant) {
        self.last_accessed.set_instant(timestamp);
    }

    #[inline]
    fn last_modified(&self) -> Option<Instant> {
        self.last_modified.instant()
    }

    #[inline]
    fn set_last_modified(&self, timestamp: Instant) {
        self.last_modified.set_instant(timestamp);
    }
}

#[cfg(test)]
mod test {
    use super::EntryInfo;

    // Run with:
    //   RUSTFLAGS='--cfg rustver' cargo test --lib --features sync -- common::concurrent::entry_info::test --nocapture
    //   RUSTFLAGS='--cfg rustver' cargo test --lib --no-default-features --features sync -- common::concurrent::entry_info::test --nocapture
    //
    // Note: the size of the struct may change in a future version of Rust.
    #[cfg_attr(
        not(all(rustver, any(target_os = "linux", target_os = "macos"))),
        ignore
    )]
    #[test]
    fn check_struct_size() {
        use std::mem::size_of;

        #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
        enum TargetArch {
            Linux64,
            Linux32X86,
            Linux32Arm,
            Linux32Mips,
            MacOS64,
        }

        use TargetArch::*;

        #[allow(clippy::option_env_unwrap)]
        // e.g. "1.64"
        let ver =
            option_env!("RUSTC_SEMVER").expect("RUSTC_SEMVER env var was not set at compile time");
        let arch = if cfg!(target_os = "linux") {
            if cfg!(target_pointer_width = "64") {
                Linux64
            } else if cfg!(target_pointer_width = "32") {
                if cfg!(target_arch = "x86") {
                    Linux32X86
                } else if cfg!(target_arch = "arm") {
                    Linux32Arm
                } else if cfg!(target_arch = "mips") {
                    Linux32Mips
                } else {
                    unimplemented!();
                }
            } else {
                unimplemented!();
            }
        } else if cfg!(target_os = "macos") {
            MacOS64
        } else {
            panic!("Unsupported target architecture");
        };

        let expected_sizes = match arch {
            Linux64 | Linux32Arm | Linux32Mips => vec![("1.51", 56)],
            Linux32X86 => vec![("1.51", 48)],
            MacOS64 => vec![("1.62", 56)],
        };

        let mut expected = None;
        for (ver_str, size) in expected_sizes {
            expected = Some(size);
            if ver >= ver_str {
                break;
            }
        }

        if let Some(size) = expected {
            assert_eq!(size_of::<EntryInfo<()>>(), size);
        } else {
            panic!("No expected size for {arch:?} with Rust version {ver}");
        }
    }
}
