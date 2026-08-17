// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_DISCARDABLE_MEMORY_ALLOCATOR_H_
#define BASE_MEMORY_DISCARDABLE_MEMORY_ALLOCATOR_H_

#include <stddef.h>

#include <memory>

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/memory/discardable_memory.h"

namespace base {
class DiscardableMemory;

// An allocator which creates and manages DiscardableMemory. The allocator
// itself should be created via CreateDiscardableMemoryAllocator, which
// selects an appropriate implementation depending on platform support.
class BASE_EXPORT DiscardableMemoryAllocator {
 public:
  DiscardableMemoryAllocator() = default;

  DiscardableMemoryAllocator(const DiscardableMemoryAllocator&) = delete;
  DiscardableMemoryAllocator& operator=(const DiscardableMemoryAllocator&) =
      delete;

  virtual ~DiscardableMemoryAllocator() = default;

  // Returns the allocator instance. Asserts if not already set.
  static DiscardableMemoryAllocator* GetInstance();

  // Returns true if the instance has been set.
  static bool HasInstance();

  // Sets the allocator instance. Can only be called once, e.g. on startup.
  // Ownership of |instance| remains with the caller.
  static void SetInstance(DiscardableMemoryAllocator* allocator);

  // Creates an initially-locked instance of discardable memory.
  // If the platform supports Android ashmem or madvise(MADV_FREE),
  // platform-specific techniques will be used to discard memory under pressure.
  // Otherwise, discardable memory is emulated and manually discarded
  // heuristicly (via memory pressure notifications).
  virtual std::unique_ptr<DiscardableMemory> AllocateLockedDiscardableMemory(
      size_t size) = 0;

  // Allocates discardable memory the same way |AllocateLockedDiscardableMemory|
  // does. In case of failure, calls |on_no_memory| and retries once. As a
  // consequence, |on_no_memory| should free some memory, and importantly,
  // address space as well.
  //
  // In case of allocation failure after retry, terminates the process with
  // an Out Of Memory status (for triage in crash reports).
  //
  // As a consequence, does *not* return nullptr.
  std::unique_ptr<DiscardableMemory>
  AllocateLockedDiscardableMemoryWithRetryOrDie(size_t size,
                                                OnceClosure on_no_memory);

  // Gets the total number of bytes allocated by this allocator which have not
  // been discarded.
  virtual size_t GetBytesAllocated() const = 0;

  // Release any memory used in the implementation of discardable memory that is
  // not immediately being used.
  virtual void ReleaseFreeMemory() = 0;
};

}  // namespace base

#endif  // BASE_MEMORY_DISCARDABLE_MEMORY_ALLOCATOR_H_
