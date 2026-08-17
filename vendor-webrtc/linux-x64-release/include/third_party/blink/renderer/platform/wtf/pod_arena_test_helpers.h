/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_ARENA_TEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_ARENA_TEST_HELPERS_H_

#include "third_party/blink/renderer/platform/wtf/pod_arena.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

#include <gtest/gtest.h>

namespace WTF {
namespace arena_test_helpers {

// An allocator for the PODArena which tracks the regions which have
// been allocated.
class TrackedAllocator final : public PODArena::FastMallocAllocator {
 public:
  static scoped_refptr<TrackedAllocator> Create() {
    return base::AdoptRef(new TrackedAllocator);
  }

  void* Allocate(size_t size) override {
    void* result = PODArena::FastMallocAllocator::Allocate(size);
    allocated_regions_.push_back(result);
    return result;
  }

  void Free(void* ptr) override {
    size_t slot = allocated_regions_.Find(ptr);
    ASSERT_NE(slot, kNotFound);
    allocated_regions_.EraseAt(slot);
    PODArena::FastMallocAllocator::Free(ptr);
  }

  bool IsEmpty() const { return !NumRegions(); }

  int NumRegions() const { return allocated_regions_.size(); }

 private:
  TrackedAllocator() = default;
  Vector<void*> allocated_regions_;
};

}  // namespace arena_test_helpers
}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_ARENA_TEST_HELPERS_H_
