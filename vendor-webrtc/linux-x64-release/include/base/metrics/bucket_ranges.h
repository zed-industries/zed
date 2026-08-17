// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// BucketRanges stores the vector of ranges that delimit what samples are
// tallied in the corresponding buckets of a histogram. Histograms that have
// same ranges for all their corresponding buckets should share the same
// BucketRanges object.
//
// E.g. A 5 buckets LinearHistogram with 1 as minimal value and 4 as maximal
// value will need a BucketRanges with 6 ranges:
// 0, 1, 2, 3, 4, INT_MAX
//
// TODO(kaiwang): Currently we keep all negative values in 0~1 bucket. Consider
// changing 0 to INT_MIN.

#ifndef BASE_METRICS_BUCKET_RANGES_H_
#define BASE_METRICS_BUCKET_RANGES_H_

#include <limits.h>
#include <stddef.h>
#include <stdint.h>

#include <atomic>
#include <vector>

#include "base/base_export.h"
#include "base/check_op.h"
#include "base/containers/span.h"
#include "base/metrics/histogram_base.h"

namespace base {

class BASE_EXPORT BucketRanges {
 public:
  typedef std::vector<HistogramBase::Sample32> Ranges;

  explicit BucketRanges(size_t num_ranges);
  explicit BucketRanges(base::span<const HistogramBase::Sample32> data);

  BucketRanges(const BucketRanges&) = delete;
  BucketRanges& operator=(const BucketRanges&) = delete;

  ~BucketRanges();

  size_t size() const { return ranges_.size(); }
  HistogramBase::Sample32 range(size_t i) const { return ranges_[i]; }
  void set_range(size_t i, HistogramBase::Sample32 value) {
    DCHECK_LT(i, ranges_.size());
    DCHECK_GE(value, 0);
    ranges_[i] = value;
  }
  uint32_t checksum() const { return checksum_; }
  void set_checksum(uint32_t checksum) { checksum_ = checksum; }

  // A bucket is defined by a consecutive pair of entries in |ranges|, so there
  // is one fewer bucket than there are ranges.  For example, if |ranges| is
  // [0, 1, 3, 7, INT_MAX], then the buckets in this histogram are
  // [0, 1), [1, 3), [3, 7), and [7, INT_MAX).
  size_t bucket_count() const { return ranges_.size() - 1; }

  // Checksum methods to verify whether the ranges are corrupted (e.g. bad
  // memory access).
  uint32_t CalculateChecksum() const;
  bool HasValidChecksum() const;
  void ResetChecksum();

  // Return true iff |other| object has same ranges_ as |this| object's ranges_.
  bool Equals(const BucketRanges* other) const;

  // Set and get a reference into persistent memory where this bucket data
  // can be found (and re-used). These calls are internally atomic with no
  // safety against overwriting an existing value since though it is wasteful
  // to have multiple identical persistent records, it is still safe.
  void set_persistent_reference(uint32_t ref) const {
    persistent_reference_.store(ref, std::memory_order_release);
  }
  uint32_t persistent_reference() const {
    return persistent_reference_.load(std::memory_order_acquire);
  }

 private:
  // A monotonically increasing list of values which determine which bucket to
  // put a sample into.  For each index, show the smallest sample that can be
  // added to the corresponding bucket.
  Ranges ranges_;

  // Checksum for the conntents of ranges_.  Used to detect random over-writes
  // of our data, and to quickly see if some other BucketRanges instance is
  // possibly Equal() to this instance.
  // TODO(kaiwang): Consider change this to uint64_t. Because we see a lot of
  // noise on UMA dashboard.
  uint32_t checksum_;

  // A reference into a global PersistentMemoryAllocator where the ranges
  // information is stored. This allows for the record to be created once and
  // re-used simply by having all histograms with the same ranges use the
  // same reference.
  mutable std::atomic<uint32_t> persistent_reference_{0};
};

}  // namespace base

#endif  // BASE_METRICS_BUCKET_RANGES_H_
