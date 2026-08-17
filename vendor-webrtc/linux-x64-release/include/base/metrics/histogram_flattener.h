// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_HISTOGRAM_FLATTENER_H_
#define BASE_METRICS_HISTOGRAM_FLATTENER_H_

#include "base/base_export.h"
#include "base/metrics/histogram.h"

namespace base {

class HistogramSamples;

// HistogramFlattener is an interface used by HistogramSnapshotManager, which
// handles the logistics of gathering up available histograms for recording.
class BASE_EXPORT HistogramFlattener {
 public:
  HistogramFlattener(const HistogramFlattener&) = delete;
  HistogramFlattener& operator=(const HistogramFlattener&) = delete;
  virtual ~HistogramFlattener() = default;

  virtual void RecordDelta(const HistogramBase& histogram,
                           const HistogramSamples& snapshot) = 0;

 protected:
  HistogramFlattener() = default;
};

}  // namespace base

#endif  // BASE_METRICS_HISTOGRAM_FLATTENER_H_
