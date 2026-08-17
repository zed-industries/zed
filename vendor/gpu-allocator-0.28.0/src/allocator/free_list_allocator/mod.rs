#![deny(unsafe_code, clippy::unwrap_used)]
#[cfg(feature = "std")]
use alloc::sync::Arc;
use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec::Vec,
};
#[cfg(feature = "std")]
use std::backtrace::Backtrace;
#[cfg(all(feature = "std", not(feature = "hashbrown")))]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "hashbrown")]
use hashbrown::{HashMap, HashSet};
use log::{log, Level};

#[cfg(feature = "visualizer")]
pub(crate) mod visualizer;

use super::{AllocationReport, AllocationType, SubAllocator, SubAllocatorBase};
use crate::{AllocationError, Result};

const USE_BEST_FIT: bool = true;

fn align_down(val: u64, alignment: u64) -> u64 {
    val & !(alignment - 1u64)
}

fn align_up(val: u64, alignment: u64) -> u64 {
    align_down(val + alignment - 1u64, alignment)
}

#[derive(Debug)]
pub(crate) struct MemoryChunk {
    pub(crate) chunk_id: core::num::NonZeroU64,
    pub(crate) size: u64,
    pub(crate) offset: u64,
    pub(crate) allocation_type: AllocationType,
    pub(crate) name: Option<String>,
    /// Only used if [`crate::AllocatorDebugSettings::store_stack_traces`] is [`true`]
    #[cfg(feature = "std")]
    pub(crate) backtrace: Arc<Backtrace>,
    next: Option<core::num::NonZeroU64>,
    prev: Option<core::num::NonZeroU64>,
}

#[derive(Debug)]
pub(crate) struct FreeListAllocator {
    size: u64,
    allocated: u64,
    pub(crate) chunk_id_counter: u64,
    pub(crate) chunks: HashMap<core::num::NonZeroU64, MemoryChunk>,
    free_chunks: HashSet<core::num::NonZeroU64>,
}

/// Test if two suballocations will overlap the same page.
fn is_on_same_page(offset_a: u64, size_a: u64, offset_b: u64, page_size: u64) -> bool {
    let end_a = offset_a + size_a - 1;
    let end_page_a = align_down(end_a, page_size);
    let start_b = offset_b;
    let start_page_b = align_down(start_b, page_size);

    end_page_a == start_page_b
}

/// Test if two allocation types will be conflicting or not.
fn has_granularity_conflict(type0: AllocationType, type1: AllocationType) -> bool {
    if type0 == AllocationType::Free || type1 == AllocationType::Free {
        return false;
    }

    type0 != type1
}

impl FreeListAllocator {
    pub(crate) fn new(size: u64) -> Self {
        #[allow(clippy::unwrap_used)]
        let initial_chunk_id = core::num::NonZeroU64::new(1).unwrap();

        let mut chunks = HashMap::default();
        chunks.insert(
            initial_chunk_id,
            MemoryChunk {
                chunk_id: initial_chunk_id,
                size,
                offset: 0,
                allocation_type: AllocationType::Free,
                name: None,
                #[cfg(feature = "std")]
                backtrace: Arc::new(Backtrace::disabled()),
                prev: None,
                next: None,
            },
        );

        let mut free_chunks = HashSet::default();
        free_chunks.insert(initial_chunk_id);

        Self {
            size,
            allocated: 0,
            // 0 is not allowed as a chunk ID, 1 is used by the initial chunk, next chunk is going to be 2.
            // The system well take the counter as the ID, and the increment the counter.
            chunk_id_counter: 2,
            chunks,
            free_chunks,
        }
    }

    /// Generates a new unique chunk ID
    fn get_new_chunk_id(&mut self) -> Result<core::num::NonZeroU64> {
        if self.chunk_id_counter == u64::MAX {
            // End of chunk id counter reached, no more allocations are possible.
            return Err(AllocationError::OutOfMemory);
        }

        let id = self.chunk_id_counter;
        self.chunk_id_counter += 1;
        core::num::NonZeroU64::new(id).ok_or_else(|| {
            AllocationError::Internal("New chunk id was 0, which is not allowed.".into())
        })
    }
    /// Finds the specified `chunk_id` in the list of free chunks and removes if from the list
    fn remove_id_from_free_list(&mut self, chunk_id: core::num::NonZeroU64) {
        self.free_chunks.remove(&chunk_id);
    }
    /// Merges two adjacent chunks. Right chunk will be merged into the left chunk
    fn merge_free_chunks(
        &mut self,
        chunk_left: core::num::NonZeroU64,
        chunk_right: core::num::NonZeroU64,
    ) -> Result<()> {
        // Gather data from right chunk and remove it
        let (right_size, right_next) = {
            let chunk = self.chunks.remove(&chunk_right).ok_or_else(|| {
                AllocationError::Internal("Chunk ID not present in chunk list.".into())
            })?;
            self.remove_id_from_free_list(chunk.chunk_id);

            (chunk.size, chunk.next)
        };

        // Merge into left chunk
        {
            let chunk = self.chunks.get_mut(&chunk_left).ok_or_else(|| {
                AllocationError::Internal("Chunk ID not present in chunk list.".into())
            })?;
            chunk.next = right_next;
            chunk.size += right_size;
        }

        // Patch pointers
        if let Some(right_next) = right_next {
            let chunk = self.chunks.get_mut(&right_next).ok_or_else(|| {
                AllocationError::Internal("Chunk ID not present in chunk list.".into())
            })?;
            chunk.prev = Some(chunk_left);
        }

        Ok(())
    }
}

impl SubAllocatorBase for FreeListAllocator {}
impl SubAllocator for FreeListAllocator {
    fn allocate(
        &mut self,
        size: u64,
        alignment: u64,
        allocation_type: AllocationType,
        granularity: u64,
        name: &str,
        #[cfg(feature = "std")] backtrace: Arc<Backtrace>,
    ) -> Result<(u64, core::num::NonZeroU64)> {
        let free_size = self.size - self.allocated;
        if size > free_size {
            return Err(AllocationError::OutOfMemory);
        }

        let mut best_fit_id: Option<core::num::NonZeroU64> = None;
        let mut best_offset = 0u64;
        let mut best_aligned_size = 0u64;
        let mut best_chunk_size = 0u64;

        for current_chunk_id in self.free_chunks.iter() {
            let current_chunk = self.chunks.get(current_chunk_id).ok_or_else(|| {
                AllocationError::Internal(
                    "Chunk ID in free list is not present in chunk list.".into(),
                )
            })?;

            if current_chunk.size < size {
                continue;
            }

            let mut offset = align_up(current_chunk.offset, alignment);

            if let Some(prev_idx) = current_chunk.prev {
                let previous = self.chunks.get(&prev_idx).ok_or_else(|| {
                    AllocationError::Internal("Invalid previous chunk reference.".into())
                })?;
                if is_on_same_page(previous.offset, previous.size, offset, granularity)
                    && has_granularity_conflict(previous.allocation_type, allocation_type)
                {
                    offset = align_up(offset, granularity);
                }
            }

            let padding = offset - current_chunk.offset;
            let aligned_size = padding + size;

            if aligned_size > current_chunk.size {
                continue;
            }

            if let Some(next_idx) = current_chunk.next {
                let next = self.chunks.get(&next_idx).ok_or_else(|| {
                    AllocationError::Internal("Invalid next chunk reference.".into())
                })?;
                if is_on_same_page(offset, size, next.offset, granularity)
                    && has_granularity_conflict(allocation_type, next.allocation_type)
                {
                    continue;
                }
            }

            if USE_BEST_FIT {
                if best_fit_id.is_none() || current_chunk.size < best_chunk_size {
                    best_fit_id = Some(*current_chunk_id);
                    best_aligned_size = aligned_size;
                    best_offset = offset;

                    best_chunk_size = current_chunk.size;
                };
            } else {
                best_fit_id = Some(*current_chunk_id);
                best_aligned_size = aligned_size;
                best_offset = offset;

                best_chunk_size = current_chunk.size;
                break;
            }
        }

        let first_fit_id = best_fit_id.ok_or(AllocationError::OutOfMemory)?;

        let chunk_id = if best_chunk_size > best_aligned_size {
            let new_chunk_id = self.get_new_chunk_id()?;

            let new_chunk = {
                let free_chunk = self.chunks.get_mut(&first_fit_id).ok_or_else(|| {
                    AllocationError::Internal("Chunk ID must be in chunk list.".into())
                })?;
                let new_chunk = MemoryChunk {
                    chunk_id: new_chunk_id,
                    size: best_aligned_size,
                    offset: free_chunk.offset,
                    allocation_type,
                    name: Some(name.to_string()),
                    #[cfg(feature = "std")]
                    backtrace,
                    prev: free_chunk.prev,
                    next: Some(first_fit_id),
                };

                free_chunk.prev = Some(new_chunk.chunk_id);
                free_chunk.offset += best_aligned_size;
                free_chunk.size -= best_aligned_size;
                new_chunk
            };

            if let Some(prev_id) = new_chunk.prev {
                let prev_chunk = self.chunks.get_mut(&prev_id).ok_or_else(|| {
                    AllocationError::Internal("Invalid previous chunk reference.".into())
                })?;
                prev_chunk.next = Some(new_chunk.chunk_id);
            }

            self.chunks.insert(new_chunk_id, new_chunk);

            new_chunk_id
        } else {
            let chunk = self
                .chunks
                .get_mut(&first_fit_id)
                .ok_or_else(|| AllocationError::Internal("Invalid chunk reference.".into()))?;

            chunk.allocation_type = allocation_type;
            chunk.name = Some(name.to_string());
            #[cfg(feature = "std")]
            {
                chunk.backtrace = backtrace;
            }

            self.remove_id_from_free_list(first_fit_id);

            first_fit_id
        };

        self.allocated += best_aligned_size;

        Ok((best_offset, chunk_id))
    }

    fn free(&mut self, chunk_id: Option<core::num::NonZeroU64>) -> Result<()> {
        let chunk_id = chunk_id
            .ok_or_else(|| AllocationError::Internal("Chunk ID must be a valid value.".into()))?;

        let (next_id, prev_id) = {
            let chunk = self.chunks.get_mut(&chunk_id).ok_or_else(|| {
                AllocationError::Internal(
                    "Attempting to free chunk that is not in chunk list.".into(),
                )
            })?;
            chunk.allocation_type = AllocationType::Free;
            chunk.name = None;
            #[cfg(feature = "std")]
            {
                chunk.backtrace = Arc::new(Backtrace::disabled());
            }

            self.allocated -= chunk.size;

            self.free_chunks.insert(chunk.chunk_id);

            (chunk.next, chunk.prev)
        };

        if let Some(next_id) = next_id {
            if self.chunks[&next_id].allocation_type == AllocationType::Free {
                self.merge_free_chunks(chunk_id, next_id)?;
            }
        }

        if let Some(prev_id) = prev_id {
            if self.chunks[&prev_id].allocation_type == AllocationType::Free {
                self.merge_free_chunks(prev_id, chunk_id)?;
            }
        }
        Ok(())
    }

    fn rename_allocation(
        &mut self,
        chunk_id: Option<core::num::NonZeroU64>,
        name: &str,
    ) -> Result<()> {
        let chunk_id = chunk_id
            .ok_or_else(|| AllocationError::Internal("Chunk ID must be a valid value.".into()))?;

        let chunk = self.chunks.get_mut(&chunk_id).ok_or_else(|| {
            AllocationError::Internal(
                "Attempting to rename chunk that is not in chunk list.".into(),
            )
        })?;

        if chunk.allocation_type == AllocationType::Free {
            return Err(AllocationError::Internal(
                "Attempting to rename a freed allocation.".into(),
            ));
        }

        chunk.name = Some(name.into());

        Ok(())
    }

    fn report_memory_leaks(
        &self,
        log_level: Level,
        memory_type_index: usize,
        memory_block_index: usize,
    ) {
        for (chunk_id, chunk) in self.chunks.iter() {
            if chunk.allocation_type == AllocationType::Free {
                continue;
            }
            let empty = "".to_string();
            let name = chunk.name.as_ref().unwrap_or(&empty);
            let backtrace_info;
            #[cfg(feature = "std")]
            {
                // TODO: Allocation could be avoided here if https://github.com/rust-lang/rust/pull/139135 is merged and stabilized.
                backtrace_info = format!(
                    ",
        backtrace: {}",
                    chunk.backtrace
                )
            }
            #[cfg(not(feature = "std"))]
            {
                backtrace_info = ""
            }
            log!(
                log_level,
                r#"leak detected: {{
    memory type: {}
    memory block: {}
    chunk: {{
        chunk_id: {},
        size: 0x{:x},
        offset: 0x{:x},
        allocation_type: {:?},
        name: {}{backtrace_info}
    }}
}}"#,
                memory_type_index,
                memory_block_index,
                chunk_id,
                chunk.size,
                chunk.offset,
                chunk.allocation_type,
                name,
            );
        }
    }

    fn report_allocations(&self) -> Vec<AllocationReport> {
        self.chunks
            .iter()
            .filter(|(_key, chunk)| chunk.allocation_type != AllocationType::Free)
            .map(|(_key, chunk)| AllocationReport {
                name: chunk
                    .name
                    .clone()
                    .unwrap_or_else(|| "<Unnamed FreeList allocation>".to_owned()),
                offset: chunk.offset,
                size: chunk.size,
                #[cfg(feature = "visualizer")]
                backtrace: chunk.backtrace.clone(),
            })
            .collect::<Vec<_>>()
    }

    fn allocated(&self) -> u64 {
        self.allocated
    }

    fn supports_general_allocations(&self) -> bool {
        true
    }
}
