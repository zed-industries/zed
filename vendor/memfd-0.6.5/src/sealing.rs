use rustix::fs::SealFlags;
use std::collections::HashSet;

/// An `HashSet` specialized on `FileSeal`.
pub type SealsHashSet = HashSet<FileSeal>;

/// Seal that can be applied to a [`Memfd`].
///
/// [`Memfd`]: crate::Memfd
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FileSeal {
    /// File cannot be reduced in size.
    ///
    /// Corresponds to `F_SEAL_SHRINK`.
    SealShrink,
    /// File cannot be grown in size.
    ///
    /// Corresponds to `F_SEAL_GROW`.
    SealGrow,
    /// File cannot be written.
    ///
    /// Corresponds to `F_SEAL_WRITE`.
    SealWrite,
    /// File sealing cannot be further manipulated.
    ///
    /// Corresponds to `F_SEAL_SEAL`.
    SealSeal,
    /// Like `F_SEAL_WRITE`, but may still be written to through pre-existing
    /// writeable mappings. Introduced in Linux 5.1.
    ///
    /// Corresponds to `F_SEAL_FUTURE_WRITE`.
    #[cfg(any(target_os = "android", target_os = "linux"))]
    SealFutureWrite,
}

impl FileSeal {
    /// Return the bit-wise flag value of this seal.
    pub(crate) const fn bitflags(self) -> SealFlags {
        match self {
            Self::SealSeal => SealFlags::SEAL,
            Self::SealShrink => SealFlags::SHRINK,
            Self::SealGrow => SealFlags::GROW,
            Self::SealWrite => SealFlags::WRITE,
            #[cfg(any(target_os = "android", target_os = "linux"))]
            Self::SealFutureWrite => SealFlags::FUTURE_WRITE,
        }
    }
}

/// Convert a set of seals into a bitflags value.
pub(crate) fn seals_to_bitflags<'a>(seals: impl IntoIterator<Item = &'a FileSeal>) -> SealFlags {
    let mut bits = SealFlags::empty();
    for seal in seals {
        bits |= seal.bitflags();
    }
    bits
}

/// Convert a bitflags value to a set of seals.
pub(crate) fn bitflags_to_seals(bitflags: SealFlags) -> SealsHashSet {
    let mut sset = SealsHashSet::new();
    if bitflags.contains(SealFlags::SEAL) {
        sset.insert(FileSeal::SealSeal);
    }
    if bitflags.contains(SealFlags::SHRINK) {
        sset.insert(FileSeal::SealShrink);
    }
    if bitflags.contains(SealFlags::GROW) {
        sset.insert(FileSeal::SealGrow);
    }
    if bitflags.contains(SealFlags::WRITE) {
        sset.insert(FileSeal::SealWrite);
    }
    #[cfg(any(target_os = "android", target_os = "linux"))]
    if bitflags.contains(SealFlags::FUTURE_WRITE) {
        sset.insert(FileSeal::SealFutureWrite);
    }
    sset
}
