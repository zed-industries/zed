// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_BLOCK_ALLOCATOR_POOL_H_
#define IPCZ_SRC_IPCZ_BLOCK_ALLOCATOR_POOL_H_

#include <atomic>
#include <cstdint>
#include <list>

#include "ipcz/block_allocator.h"
#include "ipcz/buffer_id.h"
#include "ipcz/fragment.h"
#include "third_party/abseil-cpp/absl/container/flat_hash_map.h"
#include "third_party/abseil-cpp/absl/synchronization/mutex.h"
#include "third_party/abseil-cpp/absl/types/span.h"

namespace ipcz {

// Manages a pool of BlockAllocators for a specific block size. This maintains
// the allocators in a list along with a cached reference to the allocator most
// recently used for a successful allocation.
//
// The pool cycles through its allocators as needed to satisfy new requests,
// failing only once a complete pass over all allocators fails to yield a
// successful allocation.
//
// BlockAllocatorPool is is thread-safe.
class BlockAllocatorPool {
 public:
  BlockAllocatorPool();
  ~BlockAllocatorPool();

  // Returns the total allocable capacity (in bytes) within this pool. Note that
  // this counts all blocks, including ones which are currently allocated.
  size_t GetCapacity();

  // Registers a new allocator. `buffer_memory` is the entire mapped region
  // associated with `buffer_id`, not just the subspan managed by `allocator`.
  //
  // Note that each BlockAllocatorPool allows only one allocator per unique
  // `buffer_id`. This returns false if an allocator was already registered for
  // `buffer_id`, and true otherwise to indicate success.
  bool Add(BufferId buffer_id,
           absl::Span<uint8_t> buffer_memory,
           const BlockAllocator& allocator);

  // Allocates a block from the pool and returns a reference to it as a
  // Fragment. Returns a null Fragment if a block could not be allocated.
  Fragment Allocate();

  // Frees a fragment previously allocated from one of this pool's allocators.
  // Returns true if and only if `fragment` was a valid fragment to free.
  bool Free(const Fragment& fragment);

 private:
  struct Entry {
    Entry(BufferId buffer_id,
          absl::Span<uint8_t> buffer_memory,
          const BlockAllocator& allocator);
    ~Entry();

    const BufferId buffer_id;
    const absl::Span<uint8_t> buffer_memory;
    const BlockAllocator allocator;

    // Cached pointer to the next entry.
    Entry* next = nullptr;
  };

  absl::Mutex mutex_;

  // List of all allocators added to this pool. Once added, elements are never
  // removed from this list. Note that std::list is chosen so that Entry
  // references are stable over time.
  std::list<Entry> entries_ ABSL_GUARDED_BY(mutex_);

  // Maps BufferId to a specific Entry in the pool, so individual Fragments can
  // be freed efficiently by the pool.
  absl::flat_hash_map<BufferId, Entry*> entry_map_ ABSL_GUARDED_BY(mutex_);

  // The total capacity of all BlockAllocators added to the pool so far.
  size_t capacity_ ABSL_GUARDED_BY(mutex_) = 0;

  // An atomic cache of the most recently used Entry, for fast unsynchronized
  // access in the common case.
  std::atomic<Entry*> active_entry_{nullptr};
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_BLOCK_ALLOCATOR_POOL_H_
