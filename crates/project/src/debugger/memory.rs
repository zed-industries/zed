//! This module defines the format in which memory of debuggee is represented.
//!
//! Each byte in memory can either be mapped or unmapped. We try to mimic that twofold:
//! - We assume that the memory is divided into pages of a fixed size.
//! - We assume that each page can be either mapped or unmapped.
//!
//! These two assumptions drive the shape of the memory representation.
//! In particular, we want the unmapped pages to be represented without allocating any memory, as *most*
//! of the memory in a program space is usually unmapped.
//! Note that per DAP we don't know what the address space layout is, so we can't optimize off of it.
//! Note that while we optimize for a paged layout, we also want to be able to represent memory that is not paged.
//! This use case is relevant to embedded folks. Furthermore, we cater to default 4k page size.
//! It is picked arbitrarily as a ubiquous default - other than that, the underlying format of Zed's memory storage should not be relevant
//! to the users of this module.

use std::{collections::BTreeMap, ops::RangeInclusive, sync::Arc};

use gpui::BackgroundExecutor;
use smallvec::SmallVec;

const PAGE_SIZE: u64 = 4096;

/// Represents the contents of a single page. We special-case unmapped pages to be allocation-free,
/// since they're going to make up the majority of the memory in a program space (even though the user might not even get to see them - ever).
#[derive(Clone, Debug)]
pub(super) enum PageContents {
    /// Whole page is unreadable.
    Unmapped,
    Mapped(Arc<MappedPageContents>),
}

impl PageContents {
    #[cfg(test)]
    fn mapped(contents: Vec<u8>) -> Self {
        PageContents::Mapped(Arc::new(MappedPageContents(
            vec![PageChunk::Mapped(contents.into())].into(),
        )))
    }
}

#[derive(Clone, Debug)]
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
        self.0.iter().map(|chunk| chunk.len()).sum()
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

#[derive(Default, Debug)]
pub(super) struct MappedPageContents(
    /// Most of the time there should be only one chunk (either mapped or unmapped),
    /// but we do leave the possibility open of having multiple regions of memory in a single page.
    SmallVec<[PageChunk; 1]>,
);

type MemoryAddress = u64;
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
#[repr(transparent)]
pub(super) struct PageAddress(u64);

impl PageAddress {
    pub(super) fn iter_range(
        range: RangeInclusive<PageAddress>,
    ) -> impl Iterator<Item = PageAddress> {
        let mut current = range.start().0;
        let end = range.end().0;

        std::iter::from_fn(move || {
            if current > end {
                None
            } else {
                let addr = PageAddress(current);
                current += PAGE_SIZE;
                Some(addr)
            }
        })
    }
}

pub(super) struct Memory {
    pages: BTreeMap<PageAddress, PageContents>,
}

/// Represents a single memory cell (or None if a given cell is unmapped/unknown).
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
#[repr(transparent)]
pub struct MemoryCell(pub Option<u8>);

impl Memory {
    pub(super) fn new() -> Self {
        Self {
            pages: Default::default(),
        }
    }

    pub(super) fn memory_range_to_page_range(
        range: RangeInclusive<MemoryAddress>,
    ) -> RangeInclusive<PageAddress> {
        let start_page = (range.start() / PAGE_SIZE) * PAGE_SIZE;
        let end_page = (range.end() / PAGE_SIZE) * PAGE_SIZE;
        PageAddress(start_page)..=PageAddress(end_page)
    }

    pub(super) fn build_page(&self, page_address: PageAddress) -> Option<MemoryPageBuilder> {
        if self.pages.contains_key(&page_address) {
            // We already know the state of this page.
            None
        } else {
            Some(MemoryPageBuilder::new(page_address))
        }
    }

    pub(super) fn insert_page(&mut self, address: PageAddress, page: PageContents) {
        self.pages.insert(address, page);
    }

    pub(super) fn memory_range(&self, range: RangeInclusive<MemoryAddress>) -> MemoryIterator {
        let pages = Self::memory_range_to_page_range(range.clone());
        let pages = self
            .pages
            .range(pages)
            .map(|(address, page)| (*address, page.clone()))
            .collect::<Vec<_>>();
        MemoryIterator::new(range, pages.into_iter())
    }

    pub(crate) fn clear(&mut self, background_executor: &BackgroundExecutor) {
        let memory = std::mem::take(&mut self.pages);
        background_executor
            .spawn(async move {
                drop(memory);
            })
            .detach();
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
///
/// However, we're still unsure about what's *after* the unreadable region.
/// This is where this builder comes in. It lets us track the state of figuring out contents of a single page.
pub(super) struct MemoryPageBuilder {
    chunks: MappedPageContents,
    base_address: PageAddress,
    left_to_read: u64,
}

/// Represents a chunk of memory of which we don't know if it's mapped or unmapped; thus we need
/// to issue a request to figure out it's state.
pub(super) struct UnknownMemory {
    pub(super) address: MemoryAddress,
    pub(super) size: u64,
}

impl MemoryPageBuilder {
    fn new(base_address: PageAddress) -> Self {
        Self {
            chunks: Default::default(),
            base_address,
            left_to_read: PAGE_SIZE,
        }
    }

    pub(super) fn build(self) -> (PageAddress, PageContents) {
        debug_assert_eq!(self.left_to_read, 0);
        debug_assert_eq!(
            self.chunks.len(),
            PAGE_SIZE,
            "Expected `build` to be called on a fully-fetched page"
        );
        let contents = if let Some(first) = self.chunks.0.first()
            && self.chunks.len() == 1
            && matches!(first, PageChunk::Unmapped(PAGE_SIZE))
        {
            PageContents::Unmapped
        } else {
            PageContents::Mapped(Arc::new(MappedPageContents(self.chunks.0)))
        };
        (self.base_address, contents)
    }
    /// Drives the fetching of memory, in an iterator-esque style.
    pub(super) fn next_request(&self) -> Option<UnknownMemory> {
        if self.left_to_read == 0 {
            None
        } else {
            let offset_in_current_page = PAGE_SIZE - self.left_to_read;
            Some(UnknownMemory {
                address: self.base_address.0 + offset_in_current_page,
                size: self.left_to_read,
            })
        }
    }
    pub(super) fn unknown(&mut self, bytes: u64) {
        if bytes == 0 {
            return;
        }
        self.left_to_read -= bytes;
        self.chunks.0.push(PageChunk::Unmapped(bytes));
    }
    pub(super) fn known(&mut self, data: Arc<[u8]>) {
        if data.is_empty() {
            return;
        }
        self.left_to_read -= data.len() as u64;
        self.chunks.0.push(PageChunk::Mapped(data));
    }
}

fn page_contents_into_iter(data: Arc<MappedPageContents>) -> Box<dyn Iterator<Item = MemoryCell>> {
    let mut data_range = 0..data.0.len();
    let iter = std::iter::from_fn(move || {
        let data = &data;
        let data_ref = data.clone();
        data_range.next().map(move |index| {
            let contents = &data_ref.0[index];
            match contents {
                PageChunk::Mapped(items) => {
                    let chunk_range = 0..items.len();
                    let items = items.clone();
                    Box::new(
                        chunk_range
                            .into_iter()
                            .map(move |ix| MemoryCell(Some(items[ix]))),
                    ) as Box<dyn Iterator<Item = MemoryCell>>
                }
                PageChunk::Unmapped(len) => {
                    Box::new(std::iter::repeat_n(MemoryCell(None), *len as usize))
                }
            }
        })
    })
    .flatten();

    Box::new(iter)
}
/// Defines an iteration over a range of memory. Some of this memory might be unmapped or straight up missing.
/// Thus, this iterator alternates between synthesizing values and yielding known memory.
pub struct MemoryIterator {
    start: MemoryAddress,
    end: MemoryAddress,
    current_known_page: Option<(PageAddress, Box<dyn Iterator<Item = MemoryCell>>)>,
    pages: std::vec::IntoIter<(PageAddress, PageContents)>,
}

impl MemoryIterator {
    fn new(
        range: RangeInclusive<MemoryAddress>,
        pages: std::vec::IntoIter<(PageAddress, PageContents)>,
    ) -> Self {
        Self {
            start: *range.start(),
            end: *range.end(),
            current_known_page: None,
            pages,
        }
    }
    fn fetch_next_page(&mut self) -> bool {
        if let Some((mut address, chunk)) = self.pages.next() {
            let mut contents = match chunk {
                PageContents::Unmapped => None,
                PageContents::Mapped(mapped_page_contents) => {
                    Some(page_contents_into_iter(mapped_page_contents))
                }
            };

            if address.0 < self.start {
                // Skip ahead till our iterator is at the start of the range

                //address: 20, start: 25
                //
                let to_skip = self.start - address.0;
                address.0 += to_skip;
                if let Some(contents) = &mut contents {
                    contents.nth(to_skip as usize - 1);
                }
            }
            self.current_known_page = contents.map(|contents| (address, contents));
            true
        } else {
            false
        }
    }
}
impl Iterator for MemoryIterator {
    type Item = MemoryCell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            return None;
        }
        if let Some((current_page_address, current_memory_chunk)) = self.current_known_page.as_mut()
            && current_page_address.0 <= self.start
        {
            if let Some(next_cell) = current_memory_chunk.next() {
                self.start += 1;
                return Some(next_cell);
            } else {
                self.current_known_page.take();
            }
        }
        if !self.fetch_next_page() {
            self.start += 1;
            Some(MemoryCell(None))
        } else {
            self.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::debugger::{
        MemoryCell,
        memory::{MemoryIterator, PageAddress, PageContents},
    };

    #[test]
    fn iterate_over_unmapped_memory() {
        let empty_iterator = MemoryIterator::new(0..=127, Default::default());
        let actual = empty_iterator.collect::<Vec<_>>();
        let expected = vec![MemoryCell(None); 128];
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);
    }

    #[test]
    fn iterate_over_partially_mapped_memory() {
        let it = MemoryIterator::new(
            0..=127,
            vec![(PageAddress(5), PageContents::mapped(vec![1]))].into_iter(),
        );
        let actual = it.collect::<Vec<_>>();
        let expected = std::iter::repeat_n(MemoryCell(None), 5)
            .chain(std::iter::once(MemoryCell(Some(1))))
            .chain(std::iter::repeat_n(MemoryCell(None), 122))
            .collect::<Vec<_>>();
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);
    }

    #[test]
    fn reads_from_the_middle_of_a_page() {
        let partial_iter = MemoryIterator::new(
            20..=30,
            vec![(PageAddress(0), PageContents::mapped((0..255).collect()))].into_iter(),
        );
        let actual = partial_iter.collect::<Vec<_>>();
        let expected = (20..=30)
            .map(|val| MemoryCell(Some(val)))
            .collect::<Vec<_>>();
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);
    }
}
