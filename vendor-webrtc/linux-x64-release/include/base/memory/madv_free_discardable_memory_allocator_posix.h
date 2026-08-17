// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_MADV_FREE_DISCARDABLE_MEMORY_ALLOCATOR_POSIX_H_
#define BASE_MEMORY_MADV_FREE_DISCARDABLE_MEMORY_ALLOCATOR_POSIX_H_

#include <stddef.h>

#include <atomic>
#include <memory>

#include "base/base_export.h"
#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/memory/discardable_memory.h"
#include "base/memory/discardable_memory_allocator.h"
#include "base/memory/madv_free_discardable_memory_posix.h"
#include "base/trace_event/base_tracing.h"
#include "build/build_config.h"

namespace base {
class BASE_EXPORT MadvFreeDiscardableMemoryAllocatorPosix
    : public DiscardableMemoryAllocator,
      public base::trace_event::MemoryDumpProvider {
 public:
  MadvFreeDiscardableMemoryAllocatorPosix();

  MadvFreeDiscardableMemoryAllocatorPosix(
      const MadvFreeDiscardableMemoryAllocatorPosix&) = delete;
  MadvFreeDiscardableMemoryAllocatorPosix& operator=(
      const MadvFreeDiscardableMemoryAllocatorPosix&) = delete;

  ~MadvFreeDiscardableMemoryAllocatorPosix() override;

  std::unique_ptr<DiscardableMemory> AllocateLockedDiscardableMemory(
      size_t size) override;

  size_t GetBytesAllocated() const override;

  void ReleaseFreeMemory() override {
    // Do nothing, since MADV_FREE discardable memory does not keep any memory
    // overhead that can be released.
  }

  bool OnMemoryDump(const trace_event::MemoryDumpArgs& args,
                    trace_event::ProcessMemoryDump* pmd) override;

 private:
  std::atomic<size_t> bytes_allocated_{0};
};
}  // namespace base

#endif  // BASE_MEMORY_MADV_FREE_DISCARDABLE_MEMORY_ALLOCATOR_POSIX_H_
