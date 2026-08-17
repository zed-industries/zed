// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// SampleVector implements HistogramSamples interface. It is used by all
// Histogram based classes to store samples.

#ifndef BASE_METRICS_SAMPLE_VECTOR_H_
#define BASE_METRICS_SAMPLE_VECTOR_H_

#include <stddef.h>
#include <stdint.h>

#include <atomic>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/metrics/bucket_ranges.h"
#include "base/metrics/histogram_base.h"
#include "base/metrics/histogram_samples.h"
#include "base/metrics/persistent_memory_allocator.h"

namespace base {

class BucketRanges;

class BASE_EXPORT SampleVectorBase : public HistogramSamples {
 public:
  SampleVectorBase(const SampleVectorBase&) = delete;
  SampleVectorBase& operator=(const SampleVectorBase&) = delete;
  ~SampleVectorBase() override;

  // HistogramSamples:
  void Accumulate(HistogramBase::Sample32 value,
                  HistogramBase::Count32 count) override;
  HistogramBase::Count32 GetCount(HistogramBase::Sample32 value) const override;
  HistogramBase::Count32 TotalCount() const override;
  std::unique_ptr<SampleCountIterator> Iterator() const override;
  std::unique_ptr<SampleCountIterator> ExtractingIterator() override;

  // Get count of a specific bucket.
  HistogramBase::Count32 GetCountAtIndex(size_t bucket_index) const;

  // Access the bucket ranges held externally.
  const BucketRanges* bucket_ranges() const { return bucket_ranges_; }

  AtomicSingleSample* SingleSampleForTesting() { return &single_sample(); }

 protected:
  SampleVectorBase(uint64_t id,
                   Metadata* meta,
                   const BucketRanges* bucket_ranges);
  SampleVectorBase(uint64_t id,
                   std::unique_ptr<Metadata> meta,
                   const BucketRanges* bucket_ranges);

  bool AddSubtractImpl(
      SampleCountIterator* iter,
      HistogramSamples::Operator op) override;  // |op| is ADD or SUBTRACT.

  virtual size_t GetBucketIndex(HistogramBase::Sample32 value) const;

  // Gets the destination bucket corresponding to `iter` and its `count` value.
  // Validates that the destination bucket matches the min/max from the iterator
  // and returns SIZE_MAX on a mismatch.
  size_t GetDestinationBucketIndexAndCount(SampleCountIterator& iter,
                                           HistogramBase::Count32* count);

  // Moves the single-sample value to a mounted "counts" array.
  void MoveSingleSampleToCounts();

  // Mounts (creating if necessary) an array of "counts" for multi-value
  // storage.
  void MountCountsStorageAndMoveSingleSample();

  // Mounts "counts" storage that already exists. This does not attempt to move
  // any single-sample information to that storage as that would violate the
  // "const" restriction that is often used to indicate read-only memory.
  virtual bool MountExistingCountsStorage() const = 0;

  // Creates "counts" storage and returns a span to it. The span's size must
  // be the number of counts required by the histogram. Ownership of the
  // array remains with the called method but will never change. This must be
  // called while some sort of lock is held to prevent reentry.
  virtual span<HistogramBase::Count32> CreateCountsStorageWhileLocked() = 0;

  std::optional<span<HistogramBase::AtomicCount>> counts() {
    HistogramBase::AtomicCount* data =
        counts_data_.load(std::memory_order_acquire);
    if (data == nullptr) {
      return std::nullopt;
    }
    return UNSAFE_TODO(span(data, counts_size_));
  }

  std::optional<span<const HistogramBase::AtomicCount>> counts() const {
    const HistogramBase::AtomicCount* data =
        counts_data_.load(std::memory_order_acquire);
    if (data == nullptr) {
      return std::nullopt;
    }
    return UNSAFE_TODO(span(data, counts_size_));
  }

  void set_counts(span<HistogramBase::AtomicCount> counts) const {
    CHECK_EQ(counts.size(), counts_size_);
    counts_data_.store(counts.data(), std::memory_order_release);
  }

  size_t counts_size() const { return counts_size_; }

 private:
  friend class SampleVectorTest;
  FRIEND_TEST_ALL_PREFIXES(HistogramTest, CorruptSampleCounts);
  FRIEND_TEST_ALL_PREFIXES(SharedHistogramTest, CorruptSampleCounts);

  // Returns a reference into the `counts()` array. As `counts()` may be an
  // empty optional until the array is populated, `counts()` must be checked for
  // having a value before calling `counts_at()`, or this method may CHECK-fail.
  const HistogramBase::AtomicCount& counts_at(size_t index) const {
    return (counts().value())[index];
  }
  HistogramBase::AtomicCount& counts_at(size_t index) {
    return (counts().value())[index];
  }

  // Shares the same BucketRanges with Histogram object.
  const raw_ptr<const BucketRanges> bucket_ranges_;

  // The number of counts in the histogram. Once `counts_data_` becomes
  // non-null, this is the number of values in the `counts_data_` array that
  // are usable by the SampleVector.
  const size_t counts_size_;

  // `counts_data_` is a pointer to a `HistogramBase::AtomicCount` array that is
  // held as an atomic pointer for concurrency reasons. When combined with the
  // single_sample held in the metadata, there are four possible states:
  //   1) single_sample == zero, counts_ == null
  //   2) single_sample != zero, counts_ == null
  //   3) single_sample != zero, counts_ != null BUT IS EMPTY
  //   4) single_sample == zero, counts_ != null and may have data
  // Once `counts_data_` is set to a value, it can never be changed and any
  // existing single-sample must be moved to this storage. It is mutable because
  // changing it doesn't change the (const) data but must adapt if a non-const
  // object causes the storage to be allocated and updated.
  //
  // Held as raw pointer in atomic, instead of as a span, to avoid locks. The
  // `counts_size_` is the size of the would-be span, which is CHECKd when
  // setting the pointer, and used to recreate a span on the way out.
  mutable std::atomic<HistogramBase::AtomicCount*> counts_data_;
};

// A sample vector that uses local memory for the counts array.
class BASE_EXPORT SampleVector : public SampleVectorBase {
 public:
  explicit SampleVector(const BucketRanges* bucket_ranges);
  SampleVector(uint64_t id, const BucketRanges* bucket_ranges);
  SampleVector(const SampleVector&) = delete;
  SampleVector& operator=(const SampleVector&) = delete;
  ~SampleVector() override;

  // HistogramSamples:
  bool IsDefinitelyEmpty() const override;

 private:
  FRIEND_TEST_ALL_PREFIXES(SampleVectorTest, GetPeakBucketSize);

  // HistogramSamples:
  std::string GetAsciiBody() const override;
  std::string GetAsciiHeader(std::string_view histogram_name,
                             int32_t flags) const override;

  // SampleVectorBase:
  bool MountExistingCountsStorage() const override;
  span<HistogramBase::Count32> CreateCountsStorageWhileLocked() override;

  // Writes cumulative percentage information based on the number
  // of past, current, and remaining bucket samples.
  void WriteAsciiBucketContext(int64_t past,
                               HistogramBase::Count32 current,
                               int64_t remaining,
                               uint32_t current_bucket_index,
                               std::string* output) const;

  // Finds out how large (graphically) the largest bucket will appear to be.
  double GetPeakBucketSize() const;

  size_t bucket_count() const { return bucket_ranges()->bucket_count(); }

  // Simple local storage for counts.
  mutable std::vector<HistogramBase::AtomicCount> local_counts_;
};

// A sample vector that uses persistent memory for the counts array.
class BASE_EXPORT PersistentSampleVector : public SampleVectorBase {
 public:
  PersistentSampleVector(std::string_view name,
                         uint64_t id,
                         const BucketRanges* bucket_ranges,
                         Metadata* meta,
                         const DelayedPersistentAllocation& counts);
  PersistentSampleVector(const PersistentSampleVector&) = delete;
  PersistentSampleVector& operator=(const PersistentSampleVector&) = delete;
  ~PersistentSampleVector() override;

  // HistogramSamples:
  bool IsDefinitelyEmpty() const override;

 private:
  // SampleVectorBase:
  bool MountExistingCountsStorage() const override;
  span<HistogramBase::Count32> CreateCountsStorageWhileLocked() override;

  // Persistent storage for counts.
  DelayedPersistentAllocation persistent_counts_;
};

}  // namespace base

#endif  // BASE_METRICS_SAMPLE_VECTOR_H_
