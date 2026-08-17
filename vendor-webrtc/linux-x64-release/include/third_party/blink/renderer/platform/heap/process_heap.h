// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_PROCESS_HEAP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_PROCESS_HEAP_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/cppgc/process-heap-statistics.h"

namespace blink {

class PLATFORM_EXPORT ProcessHeap final {
  STATIC_ONLY(ProcessHeap);

 public:
  static void Init();

  static size_t TotalAllocatedObjectSize() {
    return cppgc::ProcessHeapStatistics::TotalAllocatedObjectSize();
  }

  static size_t TotalAllocatedSpace() {
    return cppgc::ProcessHeapStatistics::TotalAllocatedSpace();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_PROCESS_HEAP_H_
