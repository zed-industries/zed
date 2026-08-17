// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_MADV_FREE_DISCARDABLE_MEMORY_POSIX_H_
#define BASE_MEMORY_MADV_FREE_DISCARDABLE_MEMORY_POSIX_H_

#include <stddef.h>
#include <stdint.h>

#include <atomic>
#include <vector>

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/memory/discardable_memory.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/sequence_checker.h"
#include "base/threading/thread_collision_warner.h"
#include "build/build_config.h"

namespace base {
// Discardable memory backed by the MADV_FREE advice value, available since
// Linux 4.5.
//
// When unlocked, this implementation of discardable memory will
// apply the MADV_FREE advice value to all pages within the allocated range,
// causing pages to be discarded instead of swapped upon memory pressure.
// When pages are discarded, they become zero-fill-on-demand pages.
// Attempting to unlock an already-unlocked instance is undefined behaviour.
//
// When locked, all pages will be checked for eviction. If any page has
// been discarded, the entire allocated range is unmapped and the lock fails.
// After a failed lock, the instance remains unlocked but any further attempts
// to lock will fail. Additionally, the discardable memory instance is
// invalidated and access to memory obtained via data() is undefined behaviour.
// Attempting to lock an already-locked instance is undefined behaviour. If no
// page in the allocated range has been discarded, then lock succeeds and the
// allocated range of memory is available for use without any page fault,
// additional allocations, or memory zeroing.
//
// If DCHECK_IS_ON(), additional checks are added to ensure that the discardable
// memory instance is being used correctly. These checks are not present by
// default, as some incur a significant performance penalty or do not warrant
// crashing the process. These checks are:
// -    Do not allow lock while already locked or unlock while already unlocked
// -    Do not allow memory access via data() if instance is deallocated after
//      Lock() (although invalid memory can still be accessed through existing
//      pointers)
// -    After Unlock(), disallow read or write of memory pointed to by data()
//      with PROT_NONE until next Lock()
//
// Caveats:
// [1]: The smallest allocation unit is the size of a page, so it is
//      unsuitable for small allocations.
//
// [2]: The size of a discardable memory instance must be greater than 0 bytes.
//
class BASE_EXPORT MadvFreeDiscardableMemoryPosix : public DiscardableMemory {
 public:
  MadvFreeDiscardableMemoryPosix(size_t size_in_pages,
                                 std::atomic<size_t>* allocator_byte_count);

  MadvFreeDiscardableMemoryPosix(const MadvFreeDiscardableMemoryPosix&) =
      delete;
  MadvFreeDiscardableMemoryPosix& operator=(
      const MadvFreeDiscardableMemoryPosix&) = delete;

  ~MadvFreeDiscardableMemoryPosix() override;

  bool Lock() override;
  void Unlock() override;
  void* data() const override;

  bool IsLockedForTesting() const;
  void DiscardForTesting() override;

  trace_event::MemoryAllocatorDump* CreateMemoryAllocatorDump(
      const char* name,
      trace_event::ProcessMemoryDump* pmd) const override;

 protected:
  size_t GetPageCount() const { return allocated_pages_; }

  bool IsValid() const;

  void SetKeepMemoryForTesting(bool keep_memory);

  // Force page discard by applying MADV_DONTNEED hint on a page.
  // Has the same effect as if the page was naturally discarded during
  // memory pressure due to MADV_FREE (i.e. zero-fill-on-demand pages for
  // anonymous private mappings).
  // Note that MADV_DONTNEED takes effect immediately for non-shared mappings.
  void DiscardPage(size_t page_index);

 private:
  bool LockPage(size_t page_index);
  void UnlockPage(size_t page_index);

  bool Deallocate();

  // Gets whether this instance has been discarded (but not yet unmapped).
  bool IsDiscarded() const;

  // Get whether all pages in this discardable memory instance are resident.
  bool IsResident() const;

  const size_t size_in_bytes_;
  const size_t allocated_pages_;

  // Pointer to allocator memory usage metric for updating upon allocation and
  // destruction.
  raw_ptr<std::atomic<size_t>> allocator_byte_count_;

  // Data comes from mmap() and we manage its poisioning.
  // RAW_PTR_EXCLUSION: Never allocated by PartitionAlloc (always mmap'ed), so
  // there is no benefit to using a raw_ptr, only cost.
  RAW_PTR_EXCLUSION void* data_;
  bool is_locked_ = true;

  // If true, MADV_FREE will not be set on Unlock().
  bool keep_memory_for_testing_ = false;

  // Stores the first word of a page for use during locking.
  std::vector<std::atomic<intptr_t>> page_first_word_;

  DFAKE_MUTEX(thread_collision_warner_);
};

enum class MadvFreeSupport { kUnsupported, kSupported };
BASE_EXPORT MadvFreeSupport GetMadvFreeSupport();

}  // namespace base

#endif  // BASE_MEMORY_MADV_FREE_DISCARDABLE_MEMORY_POSIX_H_
