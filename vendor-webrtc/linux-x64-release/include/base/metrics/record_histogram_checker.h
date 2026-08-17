// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_RECORD_HISTOGRAM_CHECKER_H_
#define BASE_METRICS_RECORD_HISTOGRAM_CHECKER_H_

#include <stdint.h>

#include "base/base_export.h"

namespace base {

// RecordHistogramChecker provides an interface for checking whether
// the given histogram should be recorded.
class BASE_EXPORT RecordHistogramChecker {
 public:
  virtual ~RecordHistogramChecker() = default;

  // Returns true iff the given histogram should be recorded.
  // This method may be called on any thread, so it should not mutate any state.
  // |histogram_hash| corresponds to the result of HashMetricNameAs32Bits().
  virtual bool ShouldRecord(uint32_t histogram_hash) const = 0;
};

}  // namespace base

#endif  // BASE_METRICS_RECORD_HISTOGRAM_CHECKER_H_
