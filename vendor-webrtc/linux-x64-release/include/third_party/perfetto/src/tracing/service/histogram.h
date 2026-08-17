/*
 * Copyright (C) 2023 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACING_SERVICE_HISTOGRAM_H_
#define SRC_TRACING_SERVICE_HISTOGRAM_H_

#include <stddef.h>
#include <stdint.h>

#include <limits>

#include "perfetto/base/logging.h"

namespace perfetto {

using HistValue = int64_t;

// Usage:
// Histogram<10, 100, 1000> h;  // A histogram with 3 + 1 (overflow) bucket.
// h.Add(value);
// h.GetBucketSum(0);  // Returns SUM(x) for 0 < x <= 10
// h.GetBucketSum(1);  // Returns SUM(x) for 10 < x <= 100
// h.GetBucketSum(2);  // Returns SUM(x) for 100 < x <= 1000
// h.GetBucketSum(3);  // Returns SUM(x) for x > 1000
// Likewise h.GetBucketCount(x) returns the COUNT(x).
template <HistValue... thresholds>
class Histogram {
 public:
  // 1+ is for the overflow bucket (anything > the last threshold).
  static constexpr size_t kNumBuckets = 1 + sizeof...(thresholds);

  void Add(HistValue value) {
    size_t bucket = BucketForValue(value);
    bucket_sum_[bucket] += value;
    ++bucket_count_[bucket];
  }

  static constexpr size_t num_buckets() { return kNumBuckets; }

  HistValue GetBucketThres(size_t n) const {
    PERFETTO_DCHECK(n < kNumBuckets);
    return bucket_thres_[n];
  }

  uint64_t GetBucketCount(size_t n) const {
    PERFETTO_DCHECK(n < kNumBuckets);
    return bucket_count_[n];
  }

  HistValue GetBucketSum(size_t n) const {
    PERFETTO_DCHECK(n < kNumBuckets);
    return bucket_sum_[n];
  }

  void Merge(const Histogram& other) {
    for (size_t i = 0; i < kNumBuckets; ++i) {
      bucket_sum_[i] += other.bucket_sum_[i];
      bucket_count_[i] += other.bucket_count_[i];
    }
  }

 private:
  static size_t BucketForValue(HistValue value) {
    for (size_t i = 0; i < kNumBuckets - 1; i++) {
      if (value <= bucket_thres_[i])
        return i;
    }
    return kNumBuckets - 1;
  }

  static constexpr HistValue bucket_thres_[kNumBuckets]{
      thresholds..., std::numeric_limits<HistValue>::max()};

  HistValue bucket_sum_[kNumBuckets]{};
  uint64_t bucket_count_[kNumBuckets]{};
};

}  // namespace perfetto

#endif  // SRC_TRACING_SERVICE_HISTOGRAM_H_
