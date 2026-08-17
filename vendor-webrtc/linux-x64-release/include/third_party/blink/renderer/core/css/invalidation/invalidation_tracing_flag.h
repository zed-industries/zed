// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_INVALIDATION_TRACING_FLAG_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_INVALIDATION_TRACING_FLAG_H_

#include "base/compiler_specific.h"

namespace blink {
// Style invalidation is super sensitive to performance benchmarks.
// We can easily get 1% regression per additional if statement on recursive
// invalidate methods.
// To minimize performance impact, we wrap trace events with a lookup of
// cached flag.
class InvalidationTracingFlag {
 public:
  ALWAYS_INLINE static bool IsEnabled() {
    static const unsigned char* is_tracing_enabled = GetCategoryGroupEnabled();
    return *is_tracing_enabled;
  }

 private:
  static const unsigned char* GetCategoryGroupEnabled();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_INVALIDATION_TRACING_FLAG_H_
