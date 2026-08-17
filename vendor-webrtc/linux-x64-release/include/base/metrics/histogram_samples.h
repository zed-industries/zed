// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_HISTOGRAM_SAMPLES_H_
#define BASE_METRICS_HISTOGRAM_SAMPLES_H_

#include <stddef.h>
#include <stdint.h>

#include <limits>
#include <memory>
#include <string>
#include <string_view>

#include "base/atomicops.h"
#include "base/base_export.h"
#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/metrics/histogram_base.h"

namespace base {

class Pickle;
class PickleIterator;
class SampleCountIterator;

// HistogramSamples is a container storing all samples of a histogram. All
// elements must be of a fixed width to ensure 32/64-bit interoperability.
// If this structure changes, bump the version number for kTypeIdHistogram
// in persistent_histogram_allocator.cc.
//
// Note that though these samples are individually consistent (through the use
// of atomic operations on the counts), there is only "eventual consistency"
// overall when multiple threads are accessing this data. That means that the
// sum, redundant-count, etc. could be momentarily out-of-sync with the stored
// counts but will settle to a consistent "steady state" once all threads have
// exited this code.
class BASE_EXPORT HistogramSamples {
 public:
  // A single bucket and count. To fit within a single atomic on 32-bit build
  // architectures, both |bucket| and |count| are limited in size to 16 bits.
  // This limits the functionality somewhat but if an entry can't fit then
  // the full array of samples can be allocated and used.
  struct SingleSample {
    uint16_t bucket;
    uint16_t count;
  };

  // A structure for managing an atomic single sample. Because this is generally
  // used in association with other atomic values, the defined methods use
  // acquire/release operations to guarantee ordering with outside values.
  union BASE_EXPORT AtomicSingleSample {
    AtomicSingleSample() : as_atomic(0) {}
    explicit AtomicSingleSample(subtle::Atomic32 rhs) : as_atomic(rhs) {}

    // Returns the single sample in an atomic manner. This in an "acquire"
    // load. The returned sample isn't shared and thus its fields can be safely
    // accessed. If this object is disabled, this will return an empty sample
    // (bucket count set to 0).
    SingleSample Load() const;

    // Extracts and returns the single sample and changes it to |new_value| in
    // an atomic manner. If this object is disabled, this will return an empty
    // sample (bucket count set to 0).
    SingleSample Extract(AtomicSingleSample new_value = AtomicSingleSample(0));

    // Like Extract() above, but also disables this object so that it will
    // never accumulate another value. If this object is already disabled, this
    // will return an empty sample (bucket count set to 0).
    SingleSample ExtractAndDisable();

    // Adds a given count to the held bucket. If not possible, it returns false
    // and leaves the parts unchanged. Once extracted/disabled, this always
    // returns false. This in an "acquire/release" operation.
    bool Accumulate(size_t bucket, HistogramBase::Count32 count);

    // Returns if the sample has been "disabled" (via Extract) and thus not
    // allowed to accept further accumulation.
    bool IsDisabled() const;

   private:
    // union field: The actual sample bucket and count.
    SingleSample as_parts;

    // union field: The sample as an atomic value. Atomic64 would provide
    // more flexibility but isn't available on all builds. This can hold a
    // special, internal "disabled" value indicating that it must not accept
    // further accumulation.
    subtle::Atomic32 as_atomic;
  };

  // A structure of information about the data, common to all sample containers.
  // Because of how this is used in persistent memory, it must be a POD object
  // that makes sense when initialized to all zeros.
  struct Metadata {
    // Expected size for 32/64-bit check.
    static constexpr size_t kExpectedInstanceSize = 24;

    // Initialized when the sample-set is first created with a value provided
    // by the caller. It is generally used to identify the sample-set across
    // threads and processes, though not necessarily uniquely as it is possible
    // to have multiple sample-sets representing subsets of the data.
    uint64_t id;

    // The sum of all the entries, effectively the sum(sample * count) for
    // all samples. Despite being atomic, no guarantees are made on the
    // accuracy of this value; there may be races during histogram
    // accumulation and snapshotting that we choose to accept. It should
    // be treated as approximate.
#ifdef ARCH_CPU_64_BITS
    subtle::Atomic64 sum;
#else
    // 32-bit systems don't have atomic 64-bit operations. Use a basic type
    // and don't worry about "shearing".
    int64_t sum;
#endif

    // A "redundant" count helps identify memory corruption. It redundantly
    // stores the total number of samples accumulated in the histogram. We
    // can compare this count to the sum of the counts (TotalCount() function),
    // and detect problems. Note, depending on the implementation of different
    // histogram types, there might be races during histogram accumulation
    // and snapshotting that we choose to accept. In this case, the tallies
    // might mismatch even when no memory corruption has happened.
    HistogramBase::AtomicCount redundant_count{0};

    // A single histogram value and associated count. This allows histograms
    // that typically report only a single value to not require full storage
    // to be allocated.
    AtomicSingleSample single_sample;  // 32 bits
  };

  // Because structures held in persistent memory must be POD, there can be no
  // default constructor to clear the fields. This derived class exists just
  // to clear them when being allocated on the heap.
  struct BASE_EXPORT LocalMetadata : Metadata {
    LocalMetadata();
  };

  HistogramSamples(const HistogramSamples&) = delete;
  HistogramSamples& operator=(const HistogramSamples&) = delete;
  virtual ~HistogramSamples();

  virtual void Accumulate(HistogramBase::Sample32 value,
                          HistogramBase::Count32 count) = 0;
  virtual HistogramBase::Count32 GetCount(
      HistogramBase::Sample32 value) const = 0;
  virtual HistogramBase::Count32 TotalCount() const = 0;

  bool Add(const HistogramSamples& other);

  // Add from serialized samples.
  bool AddFromPickle(PickleIterator* iter);

  bool Subtract(const HistogramSamples& other);

  // Adds the samples from |other| while also resetting |other|'s sample counts
  // to 0.
  bool Extract(HistogramSamples& other);

  // Returns an iterator to read the sample counts.
  virtual std::unique_ptr<SampleCountIterator> Iterator() const = 0;

  // Returns a special kind of iterator that resets the underlying sample count
  // to 0 when Get() is called. The returned iterator must be consumed
  // completely before being destroyed, otherwise samples may be lost (this is
  // enforced by a DCHECK in the destructor).
  virtual std::unique_ptr<SampleCountIterator> ExtractingIterator() = 0;

  // Returns true if |this| is empty (has no samples, has a |sum| of zero, and
  // has a |redundant_count| of zero), which is indicative that the caller does
  // not need to process |this|.
  // - Note 1: This should only be called when |this| is only manipulated on one
  // thread at a time (e.g., the underlying data does not change on another
  // thread). If this is not the case, then the returned value cannot be trusted
  // at all.
  // - Note 2: For performance reasons, this is not guaranteed to return the
  // correct value. If false is returned, |this| may or may not be empty.
  // However, if true is returned, then |this| is guaranteed to be empty (no
  // false positives). Of course, this assumes that "Note 1" is respected.
  //  - Note 3: The base implementation of this method checks for |sum| and
  // |redundant_count|, but the child implementations should also check for
  // samples.
  virtual bool IsDefinitelyEmpty() const;

  void Serialize(Pickle* pickle) const;

  // Returns ASCII representation of histograms data for histogram samples.
  // The dictionary returned will be of the form
  // {"name":<string>, "header":<string>, "body": <string>}
  base::Value::Dict ToGraphDict(std::string_view histogram_name,
                                int32_t flags) const;

  // Accessor functions.
  uint64_t id() const { return meta_->id; }
  int64_t sum() const {
#ifdef ARCH_CPU_64_BITS
    return subtle::NoBarrier_Load(&meta_->sum);
#else
    return meta_->sum;
#endif
  }
  HistogramBase::Count32 redundant_count() const {
    return subtle::NoBarrier_Load(&meta_->redundant_count);
  }

 protected:
  enum NegativeSampleReason {
    SAMPLES_HAVE_LOGGED_BUT_NOT_SAMPLE,
    SAMPLES_SAMPLE_LESS_THAN_LOGGED,
    SAMPLES_ADDED_NEGATIVE_COUNT,
    SAMPLES_ADD_WENT_NEGATIVE,
    SAMPLES_ADD_OVERFLOW,
    SAMPLES_ACCUMULATE_NEGATIVE_COUNT,
    SAMPLES_ACCUMULATE_WENT_NEGATIVE,
    DEPRECATED_SAMPLES_ACCUMULATE_OVERFLOW,
    SAMPLES_ACCUMULATE_OVERFLOW,
    MAX_NEGATIVE_SAMPLE_REASONS
  };

  HistogramSamples(uint64_t id, Metadata* meta);
  HistogramSamples(uint64_t id, std::unique_ptr<Metadata> meta);

  // Based on |op| type, add or subtract sample counts data from the iterator.
  enum Operator { ADD, SUBTRACT };
  virtual bool AddSubtractImpl(SampleCountIterator* iter, Operator op) = 0;

  // Accumulates to the embedded single-sample field if possible. Returns true
  // on success, false otherwise. Sum and redundant-count are also updated in
  // the success case.
  bool AccumulateSingleSample(HistogramBase::Sample32 value,
                              HistogramBase::Count32 count,
                              size_t bucket);

  // Atomically adjust the sum and redundant-count.
  void IncreaseSumAndCount(int64_t sum, HistogramBase::Count32 count);

  // Record a negative-sample observation and the reason why.
  void RecordNegativeSample(NegativeSampleReason reason,
                            HistogramBase::Count32 increment);

  AtomicSingleSample& single_sample() { return meta_->single_sample; }
  const AtomicSingleSample& single_sample() const {
    return meta_->single_sample;
  }

  // Produces an actual graph (set of blank vs non blank char's) for a bucket.
  static void WriteAsciiBucketGraph(double x_count,
                                    int line_length,
                                    std::string* output);

  // Writes textual description of the bucket contents (relative to histogram).
  // Output is the count in the buckets, as well as the percentage.
  void WriteAsciiBucketValue(HistogramBase::Count32 current,
                             double scaled_sum,
                             std::string* output) const;

  // Gets a body for this histogram samples.
  virtual std::string GetAsciiBody() const;

  // Gets a header message describing this histogram samples.
  virtual std::string GetAsciiHeader(std::string_view histogram_name,
                                     int32_t flags) const;

  // Returns a string description of what goes in a given bucket.
  const std::string GetSimpleAsciiBucketRange(
      HistogramBase::Sample32 sample) const;

  Metadata* meta() { return meta_; }

 private:
  FRIEND_TEST_ALL_PREFIXES(HistogramSamplesTest, WriteAsciiBucketGraph);

  // Depending on derived class `meta_` can come from:
  // - Local storage: Then `meta_owned_` is set and meta_ points to it.
  // - External storage: Then `meta_owned_` is null, and `meta_` point toward an
  //   external object. The callers guarantees the value will outlive this
  //   instance.
  std::unique_ptr<Metadata> meta_owned_;
  raw_ptr<Metadata> meta_;
};

class BASE_EXPORT SampleCountIterator {
 public:
  virtual ~SampleCountIterator();

  virtual bool Done() const = 0;
  virtual void Next() = 0;

  // Get the sample and count at current position.
  // Note: |max| is int64_t because histograms support logged values in the
  // full int32_t range and bucket max is exclusive, so it needs to support
  // values up to MAXINT32+1.
  // Requires: !Done();
  virtual void Get(HistogramBase::Sample32* min,
                   int64_t* max,
                   HistogramBase::Count32* count) = 0;
  static_assert(std::numeric_limits<HistogramBase::Sample32>::max() <
                    std::numeric_limits<int64_t>::max(),
                "Get() |max| must be able to hold Histogram::Sample32 max + 1");

  // Get the index of current histogram bucket.
  // For histograms that don't use predefined buckets, it returns false.
  // Requires: !Done();
  virtual bool GetBucketIndex(size_t* index) const;
};

class BASE_EXPORT SingleSampleIterator : public SampleCountIterator {
 public:
  SingleSampleIterator(HistogramBase::Sample32 min,
                       int64_t max,
                       HistogramBase::Count32 count,
                       size_t bucket_index,
                       bool value_was_extracted);
  ~SingleSampleIterator() override;

  // SampleCountIterator:
  bool Done() const override;
  void Next() override;
  void Get(HistogramBase::Sample32* min,
           int64_t* max,
           HistogramBase::Count32* count) override;

  // SampleVector uses predefined buckets so iterator can return bucket index.
  bool GetBucketIndex(size_t* index) const override;

 private:
  // Information about the single value to return.
  const HistogramBase::Sample32 min_;
  const int64_t max_;
  const size_t bucket_index_;
  HistogramBase::Count32 count_;

  // Whether the value that this iterator holds was extracted from the
  // underlying data (i.e., reset to 0).
  const bool value_was_extracted_;
};

}  // namespace base

#endif  // BASE_METRICS_HISTOGRAM_SAMPLES_H_
