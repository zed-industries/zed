// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_BLOCK_ALLOCATOR_H_
#define IPCZ_SRC_IPCZ_BLOCK_ALLOCATOR_H_

#include <atomic>
#include <cstddef>
#include <cstdint>

#include "ipcz/ipcz.h"
#include "third_party/abseil-cpp/absl/base/macros.h"
#include "third_party/abseil-cpp/absl/types/span.h"

namespace ipcz {

// BlockAllocator manages a region of memory, dividing it into dynamically
// allocable blocks of a smaller fixed size. Allocation prioritizes reuse of the
// most recently freed blocks.
//
// This is a thread-safe, lock-free implementation which doesn't store heap
// pointers within the managed region. Multiple BlockAllocators may therefore
// cooperatively manage the same region of memory for the same block size across
// different threads and processes.
class BlockAllocator {
 public:
  // Constructs an empty BlockAllocator with no memory to manage and an
  // unspecified block size. This cannot be used to allocate blocks.
  BlockAllocator();

  // Constructs a BlockAllocator to manage the memory within `region`,
  // allocating blocks of `block_size` bytes. Note that this DOES NOT initialize
  // the region. Before any BlockAllocators can allocate blocks from `region`,
  // InitializeRegion() must be called once by a single BlockAllocator managing
  // this region for the same block size.
  BlockAllocator(absl::Span<uint8_t> region, uint32_t block_size);

  BlockAllocator(const BlockAllocator&);
  BlockAllocator& operator=(const BlockAllocator&);
  ~BlockAllocator();

  const absl::Span<uint8_t>& region() const { return region_; }

  size_t block_size() const { return block_size_; }

  size_t capacity() const {
    // Note that the first block cannot be allocated, so real capacity is one
    // less than the total number of blocks which fit within the region.
    ABSL_ASSERT(num_blocks_ > 0);
    return num_blocks_ - 1;
  }

  // Performs a one-time initialization of the memory region managed by this
  // allocator. Many allocators may operate on the same region, but only one
  // must initialize that region, and it must do so before any of them can
  // allocate blocks.
  void InitializeRegion() const;

  // Allocates a block from the allocator's managed region of memory and returns
  // a pointer to its base address, where `block_size()` contiguous bytes are
  // then owned by the caller. May return null if out of blocks.
  void* Allocate() const;

  // Frees a block back to the allocator, given its base address as returned by
  // a prior call to Allocate(). Returns true on success, or false on failure.
  // Failure implies that `ptr` was not a valid block to free.
  bool Free(void* ptr) const;

 private:
  // For a region of N bytes with a block size B, BlockAllocator divides the
  // region into `N/B` contiguous blocks with this BlockHeader structure at the
  // beginning of each. These headers form a singly-linked free-list of
  // available blocks. Block 0 can never be allocated and is used exclusively
  // for its header to point to the first free block. Each free block
  // references the next free block in the list, and the last free block
  // references back to block 0 to terminate the list.
  //
  // Once a block is allocated, its entire span of B bytes -- including the
  // header space -- will remain untouched by BlockAllocator and is available
  // for application use until freed. When a block is freed, its header is
  // restored.
  struct IPCZ_ALIGN(4) BlockHeader {
    // Only meaningful within the first block.
    //
    // This field is incremented any time the block's header changes, and it's
    // used to resolve races between concurrent Allocate() or Free() operations
    // modifying the head of the free-list.
    //
    // For blocks other than the first block, this is always zero.
    uint16_t version;

    // A relative index to the next free block in the list. Note that this is
    // not relative to the index of the header's own block, but rather to the
    // index of the block which physically follows it within the region (this
    // block's "successor".) For example if this header belongs to block 3,
    // the value of `next` is relative to index 4.
    //
    // This scheme is chosen so that a region can be initialized efficiently by
    // zeroing it out and then updating only the last block's header to
    // terminate the list. That way block 0 begins by pointing to block 1 as
    // the first free block, block 1 points to block 2 as the next free block,
    // and so on.
    int16_t next;
  };
  static_assert(sizeof(BlockHeader) == 4, "Invalid BlockHeader size");

  using AtomicBlockHeader = std::atomic<BlockHeader>;
  static_assert(AtomicBlockHeader::is_always_lock_free,
                "ipcz requires lock-free atomics");

  // Helper class which tracks an absolute block index, as well as a reference
  // to that block's atomic header within the managed region.
  class FreeBlock {
   public:
    FreeBlock(int16_t index, AtomicBlockHeader& header);

    AtomicBlockHeader& header() { return header_; }

    // Returns the address of the start of this block.
    void* address() const { return &header_; }

    // Atomically updates the header for this free block, implying that this
    // block will become the new head of the allocator's free-list.
    //
    // `next_free_block` is the absolute index of the previous head of the
    // free-list, which this block will now reference as the next free block.
    void SetNextFreeBlock(int16_t next_free_block);

   private:
    const int16_t index_;
    AtomicBlockHeader& header_;
  };

  // Attempts to update the front block's header to point to `first_free_block`
  // as the free-list's new head block. `last_known_header` is a reference to
  // a copy of the most recently known header value from the first block. This
  // call can only succeed if that value can be atomically swapped out for a
  // new value.
  //
  // On failure, `last_known_header` is updated to reflect the value of the
  // current front block's header.
  bool TryUpdateFrontHeader(BlockHeader& last_known_header,
                            int16_t first_free_block) const;

  int16_t last_block_index() const { return num_blocks_ - 1; }

  bool is_index_valid(int16_t index) const {
    return index >= 0 && index <= last_block_index();
  }

  // Returns a reference to the AtomicBlockHeader for the block at `index`.
  AtomicBlockHeader& block_header_at(int16_t index) const {
    ABSL_ASSERT(is_index_valid(index));
    return *reinterpret_cast<AtomicBlockHeader*>(&region_[block_size_ * index]);
  }

  // Returns a FreeBlock corresponding to the block at `index`.
  FreeBlock free_block_at(int16_t index) const {
    return FreeBlock(index, block_header_at(index));
  }

  absl::Span<uint8_t> region_;
  uint32_t block_size_ = 0;
  int16_t num_blocks_ = 0;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_BLOCK_ALLOCATOR_H_
