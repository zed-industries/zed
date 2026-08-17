//! The provided implementation for `ArchiveContext`.

use crate::{validation::ArchiveContext, Fallible};
use core::{
    alloc::{Layout, LayoutError},
    fmt,
    ops::Range,
};

/// Errors that can occur when checking archive memory.
#[derive(Debug)]
pub enum ArchiveError {
    /// Computing the target of a relative pointer overflowed
    Overflow {
        /// The base pointer
        base: *const u8,
        /// The offset
        offset: isize,
    },
    /// The archive is under-aligned for one of the types inside
    Underaligned {
        /// The expected alignment of the archive
        expected_align: usize,
        /// The actual alignment of the archive
        actual_align: usize,
    },
    /// A pointer pointed outside the bounds of the archive
    OutOfBounds {
        /// The base of the relative pointer
        base: *const u8,
        /// The offset of the relative pointer
        offset: isize,
        /// The pointer range of the archive
        range: Range<*const u8>,
    },
    /// There wasn't enough space for the desired type at the pointed location
    Overrun {
        /// The pointer to the type
        ptr: *const u8,
        /// The desired size of the type
        size: usize,
        /// The pointer range of the archive
        range: Range<*const u8>,
    },
    /// The pointer wasn't aligned properly for the desired type
    Unaligned {
        /// The pointer to the type
        ptr: *const u8,
        /// The required alignment of the type
        align: usize,
    },
    /// The pointer wasn't within the subtree range
    SubtreePointerOutOfBounds {
        /// The pointer to the subtree
        ptr: *const u8,
        /// The subtree range
        subtree_range: Range<*const u8>,
    },
    /// There wasn't enough space in the subtree range for the desired type at the pointed location
    SubtreePointerOverrun {
        /// The pointer to the subtree type,
        ptr: *const u8,
        /// The desired size of the type
        size: usize,
        /// The subtree range
        subtree_range: Range<*const u8>,
    },
    /// A subtree range was popped out of order.
    ///
    /// Subtree ranges must be popped in the reverse of the order they are pushed.
    RangePoppedOutOfOrder {
        /// The expected depth of the range
        expected_depth: usize,
        /// The actual depth of the range
        actual_depth: usize,
    },
    /// A subtree range was not popped before validation concluded.
    UnpoppedSubtreeRanges {
        /// The depth of the last subtree that was pushed
        last_range: usize,
    },
    /// The maximum subtree depth was reached or exceeded.
    ExceededMaximumSubtreeDepth {
        /// The maximum depth that subtrees may be validated down to
        max_subtree_depth: usize,
    },
    /// A layout error occurred
    LayoutError {
        /// A layout error
        layout_error: LayoutError,
    },
}

// SAFETY: ArchiveError is safe to send to another thread
// This trait is not automatically implemented because the enum contains a pointer
unsafe impl Send for ArchiveError {}

// SAFETY: ArchiveError is safe to share between threads
// This trait is not automatically implemented because the enum contains a pointer
unsafe impl Sync for ArchiveError {}

impl fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ArchiveError::Overflow { base, offset } => write!(
                f,
                "relative pointer overflowed: base {:p} offset {}",
                base, offset
            ),
            ArchiveError::Underaligned {
                expected_align,
                actual_align,
            } => write!(
                f,
                "archive underaligned: need alignment {} but have alignment {}",
                expected_align, actual_align
            ),
            ArchiveError::OutOfBounds {
                base,
                offset,
                ref range,
            } => write!(
                f,
                "pointer out of bounds: base {:p} offset {} not in range {:p}..{:p}",
                base, offset, range.start, range.end
            ),
            ArchiveError::Overrun {
                ptr,
                size,
                ref range,
            } => write!(
                f,
                "pointer overran buffer: ptr {:p} size {} in range {:p}..{:p}",
                ptr, size, range.start, range.end
            ),
            ArchiveError::Unaligned { ptr, align } => {
                write!(
                    f,
                    "unaligned pointer: ptr {:p} unaligned for alignment {}",
                    ptr, align
                )
            }
            ArchiveError::SubtreePointerOutOfBounds {
                ptr,
                ref subtree_range,
            } => write!(
                f,
                "subtree pointer out of bounds: ptr {:p} not in range {:p}..{:p}",
                ptr, subtree_range.start, subtree_range.end
            ),
            ArchiveError::SubtreePointerOverrun {
                ptr,
                size,
                ref subtree_range,
            } => write!(
                f,
                "subtree pointer overran range: ptr {:p} size {} in range {:p}..{:p}",
                ptr, size, subtree_range.start, subtree_range.end
            ),
            ArchiveError::RangePoppedOutOfOrder {
                expected_depth,
                actual_depth,
            } => write!(
                f,
                "subtree range popped out of order: expected depth {}, actual depth {}",
                expected_depth, actual_depth
            ),
            ArchiveError::UnpoppedSubtreeRanges { ref last_range } => {
                write!(f, "unpopped subtree ranges: last range {}", last_range)
            }
            ArchiveError::ExceededMaximumSubtreeDepth { max_subtree_depth } => write!(
                f,
                "pushed a subtree range that exceeded the maximum subtree depth of {}",
                max_subtree_depth
            ),
            ArchiveError::LayoutError { ref layout_error } => {
                write!(f, "a layout error occurred: {}", layout_error)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ArchiveError {}

/// A prefix range from an [`ArchiveValidator`].
#[derive(Debug)]
pub struct PrefixRange {
    range: Range<*const u8>,
    depth: usize,
}

// SAFETY: PrefixRange is safe to send to another thread
// This trait is not automatically implemented because the struct contains a pointer
unsafe impl Send for PrefixRange {}

// SAFETY: PrefixRange is safe to share between threads
// This trait is not automatically implemented because the struct contains a pointer
unsafe impl Sync for PrefixRange {}

/// A suffix range from an [`ArchiveValidator`].
#[derive(Debug)]
pub struct SuffixRange {
    start: *const u8,
    depth: usize,
}

// SAFETY: SuffixRange is safe to send to another thread
// This trait is not automatically implemented because the struct contains a pointer
unsafe impl Send for SuffixRange {}

// SAFETY: SuffixRange is safe to share between threads
// This trait is not automatically implemented because the struct contains a pointer
unsafe impl Sync for SuffixRange {}

/// A validator that can verify archives with nonlocal memory.
#[derive(Debug)]
pub struct ArchiveValidator<'a> {
    bytes: &'a [u8],
    subtree_range: Range<*const u8>,
    subtree_depth: usize,
    max_subtree_depth: usize,
}

// SAFETY: ArchiveValidator is safe to send to another thread
// This trait is not automatically implemented because the struct contains a pointer
unsafe impl<'a> Send for ArchiveValidator<'a> {}

// SAFETY: ArchiveValidator is safe to share between threads
// This trait is not automatically implemented because the struct contains a pointer
unsafe impl<'a> Sync for ArchiveValidator<'a> {}

impl<'a> ArchiveValidator<'a> {
    /// Creates a new bounds validator for the given bytes.
    #[inline]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self::with_max_depth(bytes, usize::MAX)
    }

    /// Crates a new bounds validator for the given bytes with a maximum validation depth.
    #[inline]
    pub fn with_max_depth(bytes: &'a [u8], max_subtree_depth: usize) -> Self {
        Self {
            bytes,
            subtree_range: bytes.as_ptr_range(),
            subtree_depth: 0,
            max_subtree_depth,
        }
    }

    /// Returns the log base 2 of the alignment of the archive.
    ///
    /// An archive that is 2-aligned will return 1, 4-aligned will return 2, 8-aligned will return 3
    /// and so on.
    #[inline]
    pub fn log_alignment(&self) -> usize {
        (self.bytes.as_ptr() as usize).trailing_zeros() as usize
    }

    /// Returns the alignment of the archive.
    #[inline]
    pub fn alignment(&self) -> usize {
        1 << self.log_alignment()
    }
}

impl<'a> Fallible for ArchiveValidator<'a> {
    type Error = ArchiveError;
}

impl<'a> ArchiveContext for ArchiveValidator<'a> {
    type PrefixRange = PrefixRange;
    type SuffixRange = SuffixRange;

    #[inline]
    unsafe fn bounds_check_ptr(
        &mut self,
        base: *const u8,
        offset: isize,
    ) -> Result<*const u8, Self::Error> {
        let base_pos = base.offset_from(self.bytes.as_ptr());
        let target_pos = base_pos
            .checked_add(offset)
            .ok_or(ArchiveError::Overflow { base, offset })?;
        if target_pos < 0 || target_pos as usize > self.bytes.len() {
            Err(ArchiveError::OutOfBounds {
                base,
                offset,
                range: self.bytes.as_ptr_range(),
            })
        } else {
            Ok(base.offset(offset))
        }
    }

    #[inline]
    unsafe fn bounds_check_layout(
        &mut self,
        data_address: *const u8,
        layout: &Layout,
    ) -> Result<(), Self::Error> {
        if self.alignment() < layout.align() {
            Err(ArchiveError::Underaligned {
                expected_align: layout.align(),
                actual_align: self.alignment(),
            })
        } else if (data_address as usize) & (layout.align() - 1) != 0 {
            Err(ArchiveError::Unaligned {
                ptr: data_address,
                align: layout.align(),
            })
        } else {
            let available_space = self.bytes.as_ptr_range().end.offset_from(data_address) as usize;
            if available_space < layout.size() {
                Err(ArchiveError::Overrun {
                    ptr: data_address,
                    size: layout.size(),
                    range: self.bytes.as_ptr_range(),
                })
            } else {
                Ok(())
            }
        }
    }

    #[inline]
    unsafe fn bounds_check_subtree_ptr_layout(
        &mut self,
        data_address: *const u8,
        layout: &Layout,
    ) -> Result<(), Self::Error> {
        if layout.size() == 0 {
            if data_address < self.subtree_range.start || data_address > self.subtree_range.end {
                Err(ArchiveError::SubtreePointerOutOfBounds {
                    ptr: data_address,
                    subtree_range: self.subtree_range.clone(),
                })
            } else {
                Ok(())
            }
        } else if !self.subtree_range.contains(&data_address) {
            Err(ArchiveError::SubtreePointerOutOfBounds {
                ptr: data_address,
                subtree_range: self.subtree_range.clone(),
            })
        } else {
            let available_space = self.subtree_range.end.offset_from(data_address) as usize;
            if available_space < layout.size() {
                Err(ArchiveError::SubtreePointerOverrun {
                    ptr: data_address,
                    size: layout.size(),
                    subtree_range: self.subtree_range.clone(),
                })
            } else {
                Ok(())
            }
        }
    }

    #[inline]
    unsafe fn push_prefix_subtree_range(
        &mut self,
        root: *const u8,
        end: *const u8,
    ) -> Result<PrefixRange, Self::Error> {
        if self.subtree_depth >= self.max_subtree_depth {
            Err(ArchiveError::ExceededMaximumSubtreeDepth {
                max_subtree_depth: self.max_subtree_depth,
            })
        } else {
            let result = PrefixRange {
                range: Range {
                    start: end,
                    end: self.subtree_range.end,
                },
                depth: self.subtree_depth,
            };
            self.subtree_depth += 1;
            self.subtree_range.end = root;
            Ok(result)
        }
    }

    #[inline]
    fn pop_prefix_range(&mut self, range: PrefixRange) -> Result<(), Self::Error> {
        if self.subtree_depth - 1 != range.depth {
            Err(ArchiveError::RangePoppedOutOfOrder {
                expected_depth: self.subtree_depth - 1,
                actual_depth: range.depth,
            })
        } else {
            self.subtree_range = range.range;
            self.subtree_depth = range.depth;
            Ok(())
        }
    }

    #[inline]
    unsafe fn push_suffix_subtree_range(
        &mut self,
        start: *const u8,
        root: *const u8,
    ) -> Result<SuffixRange, Self::Error> {
        let result = SuffixRange {
            start: self.subtree_range.start,
            depth: self.subtree_depth,
        };
        self.subtree_depth += 1;
        self.subtree_range.start = start;
        self.subtree_range.end = root;
        Ok(result)
    }

    #[inline]
    fn pop_suffix_range(&mut self, range: SuffixRange) -> Result<(), Self::Error> {
        if self.subtree_depth - 1 != range.depth {
            Err(ArchiveError::RangePoppedOutOfOrder {
                expected_depth: self.subtree_depth - 1,
                actual_depth: range.depth,
            })
        } else {
            self.subtree_range.end = self.subtree_range.start;
            self.subtree_range.start = range.start;
            self.subtree_depth = range.depth;
            Ok(())
        }
    }

    #[inline]
    fn finish(&mut self) -> Result<(), Self::Error> {
        if self.subtree_depth != 0 {
            Err(ArchiveError::UnpoppedSubtreeRanges {
                last_range: self.subtree_depth - 1,
            })
        } else {
            Ok(())
        }
    }

    fn wrap_layout_error(layout_error: core::alloc::LayoutError) -> Self::Error {
        ArchiveError::LayoutError { layout_error }
    }
}
