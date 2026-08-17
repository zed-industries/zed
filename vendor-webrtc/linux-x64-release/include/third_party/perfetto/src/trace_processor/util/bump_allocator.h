/*
 * Copyright (C) 2023 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_UTIL_BUMP_ALLOCATOR_H_
#define SRC_TRACE_PROCESSOR_UTIL_BUMP_ALLOCATOR_H_

#include <cstdint>
#include <cstring>
#include <optional>
#include <tuple>

#include "perfetto/base/logging.h"
#include "perfetto/ext/base/circular_queue.h"
#include "perfetto/ext/base/utils.h"

namespace perfetto::trace_processor {

// A simple memory allocator which "bumps" a pointer to service allocations.
// See [1] for more details for an overview of bump allocators.
//
// This implementation works by obtaining a large chunk of memory from the
// system allocator (i.e. from malloc). Every allocation uses that chunk as long
// as there is free space inside. Once an allocation is requested which does not
// fit in that chunk, a new chunk is requested from the system.
//
// IMPORTANT: all allocations returned from this allocator are 8-aligned and
// all allocation sizes must be a multiple of 8.
//
// IMPORTANT: this allocator can allocate a total of 4GB of memory (2^32). Once
// this is exhausted, any further allocation will cause a CHECK.
//
// IMPORTANT: all allocations *must* be explicitly freed before destroying this
// object. The destructor will CHECK if it detects any allocation which is
// unfreed.
//
// [1] https://rust-hosted-langs.github.io/book/chapter-simple-bump.html
class BumpAllocator {
 public:
  // The limit on the total number of bits which can be used to represent
  // the chunk id.
  static constexpr uint64_t kMaxIdBits = 58;

  // The limit on the total amount of memory which can be allocated.
  static constexpr uint64_t kAllocLimit = 1ull << kMaxIdBits;

  // The size of the "large chunk" requested from the system allocator.
  // The size of this value trades-off between unused memory use vs CPU cost
  // of going to the system allocator. 64KB feels a good trade-off there.
  static constexpr uint64_t kChunkSize = 64ull * 1024;  // 64KB

  // The maximum number of chunks which this allocator can have.
  static constexpr uint64_t kMaxChunkCount = kAllocLimit / kChunkSize;

  // The number of bits used to represent the offset the chunk in AllocId.
  //
  // This is simply log2(kChunkSize): we have a separate constant as log2 is
  // not a constexpr function: the static assets below verify this stays in
  // sync.
  static constexpr uint64_t kChunkOffsetAllocIdBits = 16u;

  // The number of bits used to represent the chunk index in AllocId.
  static constexpr uint64_t kChunkIndexAllocIdBits =
      kMaxIdBits - kChunkOffsetAllocIdBits;

  // Represents an allocation returned from the allocator. We return this
  // instead of just returning a pointer to allow looking up a chunk an
  // allocation belongs to without needing having to scan chunks.
  struct AllocId {
    uint64_t chunk_index : kChunkIndexAllocIdBits;
    uint64_t chunk_offset : kChunkOffsetAllocIdBits;

    // Comparison operators mainly for sorting.
    bool operator<(const AllocId& other) const {
      return std::tie(chunk_index, chunk_offset) <
             std::tie(other.chunk_index, other.chunk_offset);
    }
    bool operator>=(const AllocId& other) const { return !(*this < other); }
    bool operator>(const AllocId& other) const { return other < *this; }
  };
  static_assert(sizeof(AllocId) == sizeof(uint64_t),
                "AllocId should be 64-bit in size to allow serialization");
  static_assert(
      kMaxChunkCount == (1ull << kChunkIndexAllocIdBits),
      "Max chunk count must match the number of bits used for chunk indices");
  static_assert(
      kChunkSize == (1 << kChunkOffsetAllocIdBits),
      "Chunk size must match the number of bits used for offset within chunk");

  BumpAllocator();

  // Verifies that all calls to |Alloc| were paired with matching calls to
  // |Free|.
  ~BumpAllocator();

  BumpAllocator(BumpAllocator&&) noexcept = default;
  BumpAllocator& operator=(BumpAllocator&&) noexcept = default;

  // Allocates |size| bytes of memory. |size| must be a multiple of 8 and less
  // than or equal to |kChunkSize|.
  //
  // Returns an |AllocId| which can be converted to a pointer using
  // |GetPointer|.
  AllocId Alloc(uint32_t size);

  // Frees an allocation previously allocated by |Alloc|. This function is *not*
  // idempotent.
  //
  // Once this function returns, |id| is no longer valid for any use. Trying
  // to use it further (e.g. to passing to other methods including Free itself)
  // will cause undefined behaviour.
  void Free(AllocId id);

  // Given an AllocId, returns a pointer which can be read from/written to.
  //
  // The caller is only allowed to access up to |size| bytes, where |size| ==
  // the |size| argument to Alloc.
  void* GetPointer(AllocId);

  // Removes chunks from the start of this allocator where all the allocations
  // in the chunks have been freed. This releases the memory back to the system.
  //
  // Returns the number of chunks freed.
  uint64_t EraseFrontFreeChunks();

  // Returns a "past the end" serialized AllocId i.e. a serialized value
  // greater than all previously returned AllocIds.
  AllocId PastTheEndId();

  // Returns the number of erased chunks from the start of this allocator.
  //
  // This value may change any time |EraseFrontFreeChunks| is called but is
  // constant otherwise.
  uint64_t erased_front_chunks_count() const {
    return erased_front_chunks_count_;
  }

 private:
  struct Chunk {
    // The allocation from the system for this chunk. Because all allocations
    // need to be 8 byte aligned, the chunk also needs to be 8-byte aligned.
    // base::AlignedUniquePtr ensures this is the case.
    base::AlignedUniquePtr<uint8_t[]> allocation;

    // The bump offset relative to |allocation.data|. Incremented to service
    // Alloc requests.
    uint32_t bump_offset = 0;

    // The number of unfreed allocations in this chunk.
    uint32_t unfreed_allocations = 0;
  };

  // Tries to allocate |size| bytes in the final chunk in |chunks_|. Returns
  // an AllocId if this was successful or std::nullopt otherwise.
  std::optional<AllocId> TryAllocInLastChunk(uint32_t size);

  uint64_t ChunkIndexToQueueIndex(uint64_t chunk_index) const {
    return chunk_index - erased_front_chunks_count_;
  }
  uint64_t QueueIndexToChunkIndex(uint64_t index_in_chunks_vec) const {
    return erased_front_chunks_count_ + index_in_chunks_vec;
  }
  uint64_t LastChunkIndex() const {
    PERFETTO_DCHECK(!chunks_.empty());
    return QueueIndexToChunkIndex(static_cast<uint64_t>(chunks_.size() - 1));
  }

  base::CircularQueue<Chunk> chunks_;
  uint64_t erased_front_chunks_count_ = 0;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_UTIL_BUMP_ALLOCATOR_H_
