// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_HISTOGRAM_SCOPE_H_
#define BASE_TRACE_EVENT_HISTOGRAM_SCOPE_H_

#include <stdint.h>

#include <optional>
#include <string>

#include "base/base_export.h"

namespace base::trace_event {

// Associates histograms in this scope with a `flow_id` in traces.
class BASE_EXPORT HistogramScope {
 public:
  explicit HistogramScope(uint64_t flow_id);
  HistogramScope(const HistogramScope&) = delete;
  HistogramScope& operator=(const HistogramScope&) = delete;
  ~HistogramScope();

  static std::optional<uint64_t> GetFlowId();

 private:
  uint64_t flow_id_;
};

}  // namespace base::trace_event

#endif  // BASE_TRACE_EVENT_HISTOGRAM_SCOPE_H_
