/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_MEMORY_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_MEMORY_INFO_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace base {
class TickClock;
}

namespace blink {

struct HeapInfo {
  DISALLOW_NEW();

  size_t used_js_heap_size = 0;
  size_t total_js_heap_size = 0;
  size_t js_heap_size_limit = 0;
};

class CORE_EXPORT MemoryInfo final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Precision of the performance.memory() attribute. A kPrecise value means
  // that the numbers will not be bucketized and only cached for a small amount
  // of time (50 ms). A kBucketized value means that the numbers will be
  // bucketized and cached for a long period of time (20 minutes).
  enum class Precision { kPrecise, kBucketized };

  explicit MemoryInfo(Precision precision);

  uint64_t totalJSHeapSize() const { return info_.total_js_heap_size; }
  uint64_t usedJSHeapSize() const { return info_.used_js_heap_size; }
  uint64_t jsHeapSizeLimit() const { return info_.js_heap_size_limit; }

 private:
  FRIEND_TEST_ALL_PREFIXES(MemoryInfoTest, Bucketized);
  FRIEND_TEST_ALL_PREFIXES(MemoryInfoTest, Precise);
  friend struct MemoryInfoTestScopedMockTime;
  // The caller owns the |clock| which must outlive the MemoryInfo.
  static void SetTickClockForTestingForCurrentThread(
      const base::TickClock* clock);

  HeapInfo info_;
};

CORE_EXPORT size_t QuantizeMemorySize(size_t);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_MEMORY_INFO_H_
