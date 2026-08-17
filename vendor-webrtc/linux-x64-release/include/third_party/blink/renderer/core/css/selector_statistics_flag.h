// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SELECTOR_STATISTICS_FLAG_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SELECTOR_STATISTICS_FLAG_H_

#include "base/compiler_specific.h"

namespace blink {
// Caches a pointer to the trace-enabled state for selector statistics
// gathering. This state is global to the process and comes from the tracing
// subsystem. For performance reasons, we only grab the pointer once - the value
// will be updated as tracing is enabled/disabled, which we read by
// dereferencing the static variable. See comment in the definition of
// `TRACE_EVENT_API_GET_CATEGORY_GROUP_ENABLED` for more details.
class SelectorStatisticsFlag {
 public:
  ALWAYS_INLINE static bool IsEnabled() {
    static const unsigned char* is_tracing_enabled = GetCategoryGroupEnabled();
    return *is_tracing_enabled;
  }

 private:
  static const unsigned char* GetCategoryGroupEnabled();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SELECTOR_STATISTICS_FLAG_H_
