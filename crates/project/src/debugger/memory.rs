//! This module defines the format in which memory of debugee is represented.
//!
//! Each byte in memory can either be mapped or unmapped. We try to mimic that twofold:
//! - We assume that the memory is divided into pages of a fixed size.
//! - We assume that each page can be either mapped or unmapped.
//! These two assumptions drive the shape of the memory representation.
//! In particular, we want the unmapped pages to be represented without allocating any memory, as *most*
//! of the memory in a program space is usually unmapped.
//! Note that per DAP we don't know what the address space layout is, so we can't optimize off of it.
//! Note that while we optimize for a paged layout, we also want to be able to represent memory that is not paged.
//! This use case is relevant to embedded folks. Furthermore, we cater to default 4k page size.
//! It is picked arbitrarily as a ubiquous default - other than that, the underlying format of Zed's memory storage should not be relevant
//! to the users of this module.

use std::{
    collections::BTreeMap,
    ops::{Range, RangeInclusive},
    sync::Arc,
};

use smallvec::SmallVec;

const PAGE_SIZE: u64 = 4096;

/// Represents the contents of a single page. We special-case unmapped pages to be allocation-free,
/// since they're going to make up the majority of the memory in a program space (even though the user might not even get to see them - ever).
#[derive(Clone)]
enum PageContents {
    /// Whole page is unreadable.
    Unmapped,
    Mapped(Arc<MappedPageContents>),
}

impl PageContents {
    fn len(&self) -> u64 {
        match self {
            PageContents::Unmapped => PAGE_SIZE,
            PageContents::Mapped(contents) => contents.len(),
        }
    }
}

enum PageChunk {
    Mapped(Arc<[u8]>),
    Unmapped(u64),
}

impl PageChunk {
    fn len(&self) -> u64 {
        match self {
            PageChunk::Mapped(contents) => contents.len() as u64,
            PageChunk::Unmapped(size) => *size,
        }
    }
}

impl MappedPageContents {
    fn len(&self) -> u64 {
        self.chunks.iter().map(|chunk| chunk.len()).sum()
    }
}
/// We hope for the whole page to be mapped in a single chunk, but we do leave the possibility open
/// of having interleaved read permissions in a single page; debuggee's execution environment might either
/// have a different page size OR it might not have paged memory layout altogether
/// (which might be relevant to embedded systems).
///
/// As stated previously, the concept of a page in this module has to do more
/// with optimizing fetching of the memory and not with the underlying bits and pieces
/// of the memory of a debuggee.
struct MappedPageContents {
    /// Most of the time there should be only one chunk (either mapped or unmapped),
    /// but we do leave the possibility open of having multiple regions of memory in a single page.
    chunks: SmallVec<[PageChunk; 1]>,
}

type BaseMemoryAddress = u64;
pub(super) struct PageAddress {}
pub(super) struct Memory {
    pages: BTreeMap<BaseMemoryAddress, PageContents>,
}

impl Memory {
    pub(super) fn new() -> Self {
        Self {
            pages: Default::default(),
        }
    }

    pub(super) fn memory_range_to_pages(
        range: RangeInclusive<BaseMemoryAddress>,
    ) -> impl Iterator<Item = BaseMemoryAddress> {
        let start_page = range.start() / PAGE_SIZE;
        let end_page = (range.end() + PAGE_SIZE - 1) / PAGE_SIZE;
        (start_page..end_page).map(|page| page * PAGE_SIZE)
    }

    pub(super) fn build_page(&self, page_address: BaseMemoryAddress) -> Option<MemoryPageBuilder> {
        None
    }
    pub(super) fn pages(&self, range: Range<usize>) -> impl Iterator<Item = &[u8]> {
        None.into_iter()
    }
}

/// Builder for memory pages.
///
/// Memory reads in DAP are sequential (or at least we make them so).
/// ReadMemory response includes `unreadableBytes` property indicating the number of bytes
/// that could not be read after the last successfully read byte.
///
/// We use it as follows:
/// - We start off with a "large" 1-page ReadMemory request.
/// - If it succeeds/fails wholesale, cool; we have no unknown memory regions in this page.
/// - If it succeeds partially, we know # of mapped bytes.
///   We might also know the # of unmapped bytes.
/// However, we're still unsure about what's *after* the unreadable region.
///
/// This is where this builder comes in. It lets us track the state of figuring out contents of a single page.
pub(super) struct MemoryPageBuilder {
    chunks: SmallVec<[PageChunk; 1]>,
    base_address: BaseMemoryAddress,
    left_to_read: u64,
}

/// Represents a chunk of memory of which we don't know if it's mapped or unmapped; thus we need
/// to issue a request to figure out it's state.
pub(super) struct UnknownMemory {
    pub(super) address: BaseMemoryAddress,
    pub(super) size: u64,
}

impl MemoryPageBuilder {
    fn new(base_address: BaseMemoryAddress) -> Self {
        Self {
            chunks: Default::default(),
            base_address,
            left_to_read: PAGE_SIZE,
        }
    }

    pub(super) fn build(self) -> PageContents {
        debug_assert_eq!(
            self.chunks.iter().map(|chunk| chunk.len()).sum::<u64>(),
            PAGE_SIZE,
            "Expected `build` to be called on a fully-fetched page"
        );
        if let Some(first) = self.chunks.first()
            && self.chunks.len() == 1
            && matches!(first, PageChunk::Unmapped(PAGE_SIZE))
        {
            PageContents::Unmapped
        } else {
            PageContents::Mapped(Arc::new(MappedPageContents {
                chunks: self.chunks,
            }))
        }
    }
    /// Drives the fetching of memory, in an iterator-esque style.
    pub(super) fn next_request(&self) -> Option<UnknownMemory> {
        if self.left_to_read == 0 {
            None
        } else {
            let offset_in_current_page = PAGE_SIZE - self.left_to_read;
            Some(UnknownMemory {
                address: self.base_address + offset_in_current_page,
                size: self.left_to_read,
            })
        }
    }
    pub(super) fn unknown(&mut self, bytes: u64) {
        if bytes == 0 {
            return;
        }
        self.left_to_read -= bytes;
        self.chunks.push(PageChunk::Unmapped(bytes));
    }
    pub(super) fn known(&mut self, data: Arc<[u8]>) {
        if data.is_empty() {
            return;
        }
        self.left_to_read -= data.len() as u64;
        self.chunks.push(PageChunk::Mapped(data));
    }
}
