// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_DISCARDABLE_MEMORY_ALLOCATOR_H_
#define BASE_TEST_TEST_DISCARDABLE_MEMORY_ALLOCATOR_H_

#include <stddef.h>

#include "base/memory/discardable_memory_allocator.h"

namespace base {

// TestDiscardableMemoryAllocator is a simple DiscardableMemoryAllocator
// implementation that can be used for testing. It allocates one-shot
// DiscardableMemory instances backed by heap memory.
class TestDiscardableMemoryAllocator : public DiscardableMemoryAllocator {
 public:
  TestDiscardableMemoryAllocator() = default;

  TestDiscardableMemoryAllocator(const TestDiscardableMemoryAllocator&) =
      delete;
  TestDiscardableMemoryAllocator& operator=(
      const TestDiscardableMemoryAllocator&) = delete;

  // Overridden from DiscardableMemoryAllocator:
  std::unique_ptr<DiscardableMemory> AllocateLockedDiscardableMemory(
      size_t size) override;

  size_t GetBytesAllocated() const override;

  void ReleaseFreeMemory() override {
    // Do nothing since it is backed by heap memory.
  }
};

}  // namespace base

#endif  // BASE_TEST_TEST_DISCARDABLE_MEMORY_ALLOCATOR_H_
