// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// SampleMap implements HistogramSamples interface. It is used by the
// SparseHistogram class to store samples.

#ifndef BASE_METRICS_SAMPLE_MAP_H_
#define BASE_METRICS_SAMPLE_MAP_H_

#include <stdint.h>

#include <map>
#include <memory>

#include "base/base_export.h"
#include "base/metrics/histogram_base.h"
#include "base/metrics/histogram_samples.h"

namespace base {

// The logic here is similar to that of PersistentSampleMap but with different
// data structures. Changes here likely need to be duplicated there.
class BASE_EXPORT SampleMap : public HistogramSamples {
 public:
  using SampleToCountMap =
      std::map<HistogramBase::Sample32, HistogramBase::Count32>;

  explicit SampleMap(uint64_t id = 0);

  SampleMap(const SampleMap&) = delete;
  SampleMap& operator=(const SampleMap&) = delete;

  ~SampleMap() override;

  // HistogramSamples:
  void Accumulate(HistogramBase::Sample32 value,
                  HistogramBase::Count32 count) override;
  HistogramBase::Count32 GetCount(HistogramBase::Sample32 value) const override;
  HistogramBase::Count32 TotalCount() const override;
  std::unique_ptr<SampleCountIterator> Iterator() const override;
  std::unique_ptr<SampleCountIterator> ExtractingIterator() override;
  bool IsDefinitelyEmpty() const override;

 protected:
  // Performs arithmetic. |op| is ADD or SUBTRACT.
  bool AddSubtractImpl(SampleCountIterator* iter, Operator op) override;

 private:
  SampleToCountMap sample_counts_;
};

}  // namespace base

#endif  // BASE_METRICS_SAMPLE_MAP_H_
