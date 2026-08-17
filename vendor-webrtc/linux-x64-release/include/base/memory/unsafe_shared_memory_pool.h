// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_UNSAFE_SHARED_MEMORY_POOL_H_
#define BASE_MEMORY_UNSAFE_SHARED_MEMORY_POOL_H_

#include <memory>
#include <utility>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/memory/ref_counted.h"
#include "base/memory/unsafe_shared_memory_region.h"
#include "base/synchronization/lock.h"
#include "base/types/pass_key.h"

namespace base {

// UnsafeSharedMemoryPool manages allocation and pooling of
// UnsafeSharedMemoryRegions. Using pool saves cost of repeated shared memory
// allocations. Up-to 32 regions would be pooled. It is thread-safe. May return
// bigger regions than requested. If a requested size is increased, all stored
// regions are purged. Regions are returned to the buffer on destruction of
// |SharedMemoryHandle| if they are of a correct size.
class BASE_EXPORT UnsafeSharedMemoryPool
    : public RefCountedThreadSafe<UnsafeSharedMemoryPool> {
 public:
  // Used to store the allocation result.
  // This class returns memory to the pool upon destruction.
  class BASE_EXPORT Handle {
   public:
    Handle(PassKey<UnsafeSharedMemoryPool>,
           UnsafeSharedMemoryRegion region,
           WritableSharedMemoryMapping mapping,
           scoped_refptr<UnsafeSharedMemoryPool> pool);

    ~Handle();
    // Disallow copy and assign.
    Handle(const Handle&) = delete;
    Handle& operator=(const Handle&) = delete;

    const UnsafeSharedMemoryRegion& GetRegion() const LIFETIME_BOUND;

    WritableSharedMemoryMapping& GetMapping() LIFETIME_BOUND {
      return const_cast<WritableSharedMemoryMapping&>(
          std::as_const(*this).GetMapping());
    }
    const WritableSharedMemoryMapping& GetMapping() const LIFETIME_BOUND;

   private:
    UnsafeSharedMemoryRegion region_;
    WritableSharedMemoryMapping mapping_;
    scoped_refptr<UnsafeSharedMemoryPool> pool_;
  };

  UnsafeSharedMemoryPool();
  // Disallow copy and assign.
  UnsafeSharedMemoryPool(const UnsafeSharedMemoryPool&) = delete;
  UnsafeSharedMemoryPool& operator=(const UnsafeSharedMemoryPool&) = delete;

  // Allocates a region of the given |size| or reuses a previous allocation if
  // possible.
  std::unique_ptr<Handle> MaybeAllocateBuffer(size_t size);

  // Shuts down the pool, freeing all currently unused allocations and freeing
  // outstanding ones as they are returned.
  void Shutdown();

 private:
  friend class RefCountedThreadSafe<UnsafeSharedMemoryPool>;
  ~UnsafeSharedMemoryPool();

  void ReleaseBuffer(UnsafeSharedMemoryRegion region,
                     WritableSharedMemoryMapping mapping);

  Lock lock_;
  // All shared memory regions cached internally are guaranteed to be
  // at least `region_size_` bytes in size.
  size_t region_size_ GUARDED_BY(lock_) = 0u;
  // Cached unused regions and their mappings.
  std::vector<std::pair<UnsafeSharedMemoryRegion, WritableSharedMemoryMapping>>
      regions_ GUARDED_BY(lock_);
  bool is_shutdown_ GUARDED_BY(lock_) = false;
};

}  // namespace base

#endif  // BASE_MEMORY_UNSAFE_SHARED_MEMORY_POOL_H_
