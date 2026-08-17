// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_SAMPLE_MAP_ITERATOR_H_
#define BASE_METRICS_SAMPLE_MAP_ITERATOR_H_

#include <stdint.h>

#include <atomic>
#include <type_traits>
#include <utility>

#include "base/check.h"
#include "base/memory/raw_ptr.h"
#include "base/metrics/histogram_base.h"
#include "base/metrics/histogram_samples.h"
#include "base/types/to_address.h"

namespace base {

// An iterator for going through a SampleMap. `MapT` is the underlying map type
// that stores the counts. `support_extraction` should be true iff the caller
// wants this iterator to support extracting the values. If the counts are
// pointers, accesses to them will be atomic; see `kUseAtomicOps` below.
template <typename MapT, bool support_extraction>
class SampleMapIterator : public SampleCountIterator {
 private:
  using T = std::conditional_t<support_extraction, MapT, const MapT>;

 public:
  explicit SampleMapIterator(T& sample_counts)
      : iter_(sample_counts.begin()), end_(sample_counts.end()) {
    SkipEmptyBuckets();
  }

  ~SampleMapIterator() override {
    if constexpr (support_extraction) {
      // Ensure that the user has consumed all the samples in order to ensure no
      // samples are lost.
      DCHECK(Done());
    }
  }

  // SampleCountIterator:
  bool Done() const override { return iter_ == end_; }

  void Next() override {
    DCHECK(!Done());
    ++iter_;
    SkipEmptyBuckets();
  }

  void Get(HistogramBase::Sample32* min,
           int64_t* max,
           HistogramBase::Count32* count) override {
    DCHECK(!Done());
    *min = iter_->first;
    *max = int64_t{iter_->first} + 1;
    if constexpr (support_extraction) {
      *count = Exchange();
    } else {
      *count = Load();
    }
  }

 private:
  using I = std::conditional_t<support_extraction,
                               typename T::iterator,
                               typename T::const_iterator>;

  // If the counts are pointers, assume they may live in shared memory, which
  // means accesses to them must be atomic, since other processes may attempt to
  // concurrently modify their values. (Note that a lock wouldn't help here,
  // since said other processes would not be aware of our lock.) If they are
  // values, we don't bother with atomic ops; callers who want thread-safety can
  // use locking.
  static constexpr bool kUseAtomicOps =
      IsPointerOrRawPtr<typename T::mapped_type>;

  void SkipEmptyBuckets() {
    while (!Done() && Load() == 0) {
      ++iter_;
    }
  }

  HistogramBase::Count32 Load() const {
    if constexpr (kUseAtomicOps) {
      return iter_->second->load(std::memory_order_relaxed);
    } else {
      return iter_->second;
    }
  }

  HistogramBase::Count32 Exchange() const
    requires support_extraction
  {
    if constexpr (kUseAtomicOps) {
      return iter_->second->exchange(0, std::memory_order_relaxed);
    } else {
      return std::exchange(iter_->second, 0);
    }
  }

  I iter_;
  const I end_;
};

}  // namespace base

#endif  // BASE_METRICS_SAMPLE_MAP_ITERATOR_H_
