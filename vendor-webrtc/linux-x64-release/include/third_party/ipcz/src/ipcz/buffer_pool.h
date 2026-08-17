// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_BUFFER_POOL_H_
#define IPCZ_SRC_IPCZ_BUFFER_POOL_H_

#include <cstdint>
#include <map>
#include <memory>

#include "ipcz/block_allocator.h"
#include "ipcz/buffer_id.h"
#include "ipcz/driver_memory_mapping.h"
#include "ipcz/fragment.h"
#include "ipcz/fragment_descriptor.h"
#include "third_party/abseil-cpp/absl/container/flat_hash_map.h"
#include "third_party/abseil-cpp/absl/synchronization/mutex.h"
#include "third_party/abseil-cpp/absl/types/span.h"

namespace ipcz {

class BlockAllocatorPool;

// BufferPool maintains ownership of an extensible collection of mapped
// DriverMemory buffers, exposing access to them for dynamic memory allocation
// and arbitrary state sharing. Every buffer owned by a BufferPool is identified
// by a unique BufferId, and once a buffer is added to the pool it remains there
// indefinitely.
//
// BufferPool objects are thread-safe.
class BufferPool {
 public:
  BufferPool();
  ~BufferPool();

  // Resolves `descriptor` to a concrete Fragment. If the descriptor is null or
  // describes a region of memory which exceeds the bounds of the identified
  // buffer, this returns a null Fragment.
  //
  // If the descriptor's BufferId is not yet registered with this pool, this
  // returns a pending Fragment with the same BufferId and dimensions as
  // `descriptor`.
  //
  // Otherwise this returns a resolved Fragment which references an appropriate
  // span of mapped memory.
  Fragment GetFragment(const FragmentDescriptor& descriptor);

  // Registers `mapping` under `id` within this pool, along with a collection of
  // BlockAllocators that have already been initialized within the mapped
  // memory, to support block allocation by the pool.
  //
  // Returns true if the mapping and BlockAllocators were successfully added to
  // the pool, or false if the pool already had a buffer registered under the
  // given `id` or if any allocator within `allocators` is not contained by
  // `mapping` or is otherwise invalid.
  //
  // Note that every allocator in `block_allocators` must have a unique
  // power-of-2 block size, as each buffer only supports at most one allocator
  // per block size.
  bool AddBlockBuffer(BufferId id,
                      DriverMemoryMapping mapping,
                      absl::Span<const BlockAllocator> block_allocators);

  // Returns the total size in bytes of capacity available across all registered
  // BlockAllocators for the given `block_size`.
  size_t GetTotalBlockCapacity(size_t block_size);

  // Attempts to allocate an unused block of at least `block_size` bytes from
  // any available block allocation buffer in the pool, preferring the smaller
  // blocks over larger ones. If the BufferPool cannot accommodate the
  // allocation request, this returns a null Fragment.
  Fragment AllocateBlock(size_t block_size);

  // Similar to AllocateFragment(), but this may allocate less space than
  // requested if that's all that's available. May still return a null Fragment
  // if the BufferPool has trouble finding available memory.
  Fragment AllocateBlockBestEffort(size_t preferred_block_size);

  // Frees a block previously allocated from this pool via AllocateBlock() or
  // AllocateBlockBestEffort(). Returns true if successful, or false if
  // `fragment` was not allocated from one of this pool's block buffers.
  bool FreeBlock(const Fragment& fragment);

  // Runs `callback` as soon as the identified buffer is added to the underlying
  // BufferPool. If the buffer is already present here, `callback` is run
  // immediately. If this BufferPool's NodeLink is disconnected before the
  // buffer becomes available, `callback` is destroyed and never run.
  using WaitForBufferCallback = std::function<void()>;
  void WaitForBufferAsync(BufferId id, WaitForBufferCallback callback);

  // Purges all pending buffer callbacks enqueued by WaitForBufferAsync(),
  // without invoking them.
  void DropPendingBufferCallbacks();

 private:
  absl::Mutex mutex_;
  absl::flat_hash_map<BufferId, DriverMemoryMapping> mappings_
      ABSL_GUARDED_BY(mutex_);

  // Mapping from block size to a pool of BlockAllocators for that size. When
  // a new BlockAllocator is registered with this BufferPool, it's added to an
  // appropriate BlockAllocatorPool within this map.
  using BlockAllocatorPoolMap =
      std::map<size_t, std::unique_ptr<BlockAllocatorPool>>;
  BlockAllocatorPoolMap block_allocator_pools_ ABSL_GUARDED_BY(mutex_);

  // Callbacks to be invoked when an identified buffer becomes available.
  using BufferCallbackMap =
      absl::flat_hash_map<BufferId, std::vector<WaitForBufferCallback>>;
  BufferCallbackMap buffer_callbacks_ ABSL_GUARDED_BY(mutex_);
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_BUFFER_POOL_H_
