// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Histogram is an object that aggregates statistics, and can summarize them in
// various forms, including ASCII graphical, HTML, and numerically (as a
// vector of numbers corresponding to each of the aggregating buckets).

// It supports calls to accumulate either time intervals (which are processed
// as integral number of milliseconds), or arbitrary integral units.

// For Histogram (exponential histogram), LinearHistogram and CustomHistogram,
// the minimum for a declared range is 1 (instead of 0), while the maximum is
// (HistogramBase::kSampleType_MAX - 1). However, there will always be underflow
// and overflow buckets added automatically, so a 0 bucket will always exist
// even when a minimum value of 1 is specified.

// Each use of a histogram with the same name will reference the same underlying
// data, so it is safe to record to the same histogram from multiple locations
// in the code. It is a runtime error if all uses of the same histogram do not
// agree exactly in type, bucket size and range.

// For Histogram and LinearHistogram, the maximum for a declared range should
// always be larger (not equal) than minimal range. Zero and
// HistogramBase::kSampleType_MAX are implicitly added as first and last ranges,
// so the smallest legal bucket_count is 3. However CustomHistogram can have
// bucket count as 2 (when you give a custom ranges vector containing only 1
// range).
// For these 3 kinds of histograms, the max bucket count is always
// (Histogram::kBucketCount_MAX - 1).

// The buckets layout of class Histogram is exponential. For example, buckets
// might contain (sequentially) the count of values in the following intervals:
// [0,1), [1,2), [2,4), [4,8), [8,16), [16,32), [32,64), [64,infinity)
// That bucket allocation would actually result from construction of a histogram
// for values between 1 and 64, with 8 buckets, such as:
// Histogram count("some name", 1, 64, 8);
// Note that the underflow bucket [0,1) and the overflow bucket [64,infinity)
// are also counted by the constructor in the user supplied "bucket_count"
// argument.
// The above example has an exponential ratio of 2 (doubling the bucket width
// in each consecutive bucket).  The Histogram class automatically calculates
// the smallest ratio that it can use to construct the number of buckets
// selected in the constructor.  An another example, if you had 50 buckets,
// and millisecond time values from 1 to 10000, then the ratio between
// consecutive bucket widths will be approximately somewhere around the 50th
// root of 10000.  This approach provides very fine grain (narrow) buckets
// at the low end of the histogram scale, but allows the histogram to cover a
// gigantic range with the addition of very few buckets.

// Usually we use macros to define and use a histogram, which are defined in
// base/metrics/histogram_macros.h. Note: Callers should include that header
// directly if they only access the histogram APIs through macros.
//
// Macros use a pattern involving a function static variable, that is a pointer
// to a histogram.  This static is explicitly initialized on any thread
// that detects a uninitialized (NULL) pointer.  The potentially racy
// initialization is not a problem as it is always set to point to the same
// value (i.e., the FactoryGet always returns the same value).  FactoryGet
// is also completely thread safe, which results in a completely thread safe,
// and relatively fast, set of counters.  To avoid races at shutdown, the static
// pointer is NOT deleted, and we leak the histograms at process termination.

#ifndef BASE_METRICS_HISTOGRAM_H_
#define BASE_METRICS_HISTOGRAM_H_

#include <stddef.h>
#include <stdint.h>

#include <map>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "base/dcheck_is_on.h"
#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/metrics/bucket_ranges.h"
#include "base/metrics/histogram_base.h"
#include "base/metrics/histogram_samples.h"
#include "base/time/time.h"
#include "base/values.h"

namespace base {

class BooleanHistogram;
class CustomHistogram;
class DelayedPersistentAllocation;
class Histogram;
class HistogramTest;
class LinearHistogram;
class Pickle;
class PickleIterator;
class SampleVector;
class SampleVectorBase;

class BASE_EXPORT Histogram : public HistogramBase {
 public:
  // Initialize maximum number of buckets in histograms as 1000, plus over and
  // under.  This must be a value that fits in a uint32_t (since that's how we
  // serialize bucket counts) as well as a Sample32 (since samples can be up to
  // this value).
  static constexpr size_t kBucketCount_MAX = 1002;

  using Counts = std::vector<Count32>;

  Histogram(const Histogram&) = delete;
  Histogram& operator=(const Histogram&) = delete;

  ~Histogram() override;

  //----------------------------------------------------------------------------
  // For a valid histogram, input should follow these restrictions:
  // minimum > 0 (if a minimum below 1 is specified, it will implicitly be
  //              normalized up to 1)
  // maximum > minimum
  // buckets > 2 [minimum buckets needed: underflow, overflow and the range]
  // Additionally,
  // buckets <= (maximum - minimum + 2) - this is to ensure that we don't have
  // more buckets than the range of numbers; having more buckets than 1 per
  // value in the range would be nonsensical.
  static HistogramBase* FactoryGet(std::string_view name,
                                   Sample32 minimum,
                                   Sample32 maximum,
                                   size_t bucket_count,
                                   int32_t flags);
  static HistogramBase* FactoryTimeGet(std::string_view name,
                                       base::TimeDelta minimum,
                                       base::TimeDelta maximum,
                                       size_t bucket_count,
                                       int32_t flags);
  static HistogramBase* FactoryMicrosecondsTimeGet(std::string_view name,
                                                   base::TimeDelta minimum,
                                                   base::TimeDelta maximum,
                                                   size_t bucket_count,
                                                   int32_t flags);

  // Overloads of the above functions that take a const std::string& or const
  // char* |name| param, to avoid code bloat from the std::string constructor
  // being inlined into call sites.
  static HistogramBase* FactoryGet(const std::string& name,
                                   Sample32 minimum,
                                   Sample32 maximum,
                                   size_t bucket_count,
                                   int32_t flags);
  static HistogramBase* FactoryTimeGet(const std::string& name,
                                       base::TimeDelta minimum,
                                       base::TimeDelta maximum,
                                       size_t bucket_count,
                                       int32_t flags);
  static HistogramBase* FactoryMicrosecondsTimeGet(const std::string& name,
                                                   base::TimeDelta minimum,
                                                   base::TimeDelta maximum,
                                                   size_t bucket_count,
                                                   int32_t flags);

  static HistogramBase* FactoryGet(const char* name,
                                   Sample32 minimum,
                                   Sample32 maximum,
                                   size_t bucket_count,
                                   int32_t flags);
  static HistogramBase* FactoryTimeGet(const char* name,
                                       base::TimeDelta minimum,
                                       base::TimeDelta maximum,
                                       size_t bucket_count,
                                       int32_t flags);
  static HistogramBase* FactoryMicrosecondsTimeGet(const char* name,
                                                   base::TimeDelta minimum,
                                                   base::TimeDelta maximum,
                                                   size_t bucket_count,
                                                   int32_t flags);

  // Create a histogram using data in persistent storage.
  static std::unique_ptr<HistogramBase> PersistentCreate(
      DurableStringView durable_name,
      const BucketRanges* ranges,
      const DelayedPersistentAllocation& counts,
      const DelayedPersistentAllocation& logged_counts,
      HistogramSamples::Metadata* meta,
      HistogramSamples::Metadata* logged_meta);

  static void InitializeBucketRanges(Sample32 minimum,
                                     Sample32 maximum,
                                     BucketRanges* ranges);

  // This constant if for FindCorruption. Since snapshots of histograms are
  // taken asynchronously relative to sampling, and our counting code currently
  // does not prevent race conditions, it is pretty likely that we'll catch a
  // redundant count that doesn't match the sample count.  We allow for a
  // certain amount of slop before flagging this as an inconsistency. Even with
  // an inconsistency, we'll snapshot it again (for UMA in about a half hour),
  // so we'll eventually get the data, if it was not the result of a corruption.
  static const int kCommonRaceBasedCountMismatch;

  // Check to see if bucket ranges, counts and tallies in the snapshot are
  // consistent with the bucket ranges and checksums in our histogram.  This can
  // produce a false-alarm if a race occurred in the reading of the data during
  // a SnapShot process, but should otherwise be false at all times (unless we
  // have memory over-writes, or DRAM failures). Flag definitions are located
  // under "enum Inconsistency" in base/metrics/histogram_base.h.
  uint32_t FindCorruption(const HistogramSamples& samples) const override;

  //----------------------------------------------------------------------------
  // Accessors for factory construction, serialization and testing.
  //----------------------------------------------------------------------------
  const BucketRanges* bucket_ranges() const;
  Sample32 declared_min() const;
  Sample32 declared_max() const;
  virtual Sample32 ranges(size_t i) const;
  virtual size_t bucket_count() const;

  // This function validates histogram construction arguments. It returns false
  // if some of the arguments are bad but also corrects them so they should
  // function on non-dcheck builds without crashing.
  // Note. Currently it allow some bad input, e.g. 0 as minimum, but silently
  // converts it to good input: 1.
  static bool InspectConstructionArguments(std::string_view name,
                                           Sample32* minimum,
                                           Sample32* maximum,
                                           size_t* bucket_count);

  // HistogramBase implementation:
  uint64_t name_hash() const override;
  HistogramType GetHistogramType() const override;
  bool HasConstructionArguments(Sample32 expected_minimum,
                                Sample32 expected_maximum,
                                size_t expected_bucket_count) const override;
  void Add(Sample32 value) override;
  void AddCount(Sample32 value, int count) override;
  std::unique_ptr<HistogramSamples> SnapshotSamples() const override;
  std::unique_ptr<HistogramSamples> SnapshotUnloggedSamples() const override;
  void MarkSamplesAsLogged(const HistogramSamples& samples) final;
  std::unique_ptr<HistogramSamples> SnapshotDelta() override;
  std::unique_ptr<HistogramSamples> SnapshotFinalDelta() const override;
  bool AddSamples(const HistogramSamples& samples) override;
  bool AddSamplesFromPickle(base::PickleIterator* iter) override;
  base::Value::Dict ToGraphDict() const override;

 protected:
  // This class, defined entirely within the .cc file, contains all the
  // common logic for building a Histogram and can be overridden by more
  // specific types to alter details of how the creation is done. It is
  // defined as an embedded class (rather than an anonymous one) so it
  // can access the protected constructors.
  class Factory;

  // |ranges| should contain the underflow and overflow buckets. See top
  // comments for example.
  Histogram(DurableStringView durable_name, const BucketRanges* ranges);

  // Traditionally, histograms allocate their own memory for the bucket
  // vector but "shared" histograms use memory regions allocated from a
  // special memory segment that is passed in here.  It is assumed that
  // the life of this memory is managed externally and exceeds the lifetime
  // of this object. Practically, this memory is never released until the
  // process exits and the OS cleans it up.
  Histogram(DurableStringView durable_name,
            const BucketRanges* ranges,
            const DelayedPersistentAllocation& counts,
            const DelayedPersistentAllocation& logged_counts,
            HistogramSamples::Metadata* meta,
            HistogramSamples::Metadata* logged_meta);

  // HistogramBase implementation:
  void SerializeInfoImpl(base::Pickle* pickle) const override;

  // Return a string description of what goes in a given bucket.
  // Most commonly this is the numeric value, but in derived classes it may
  // be a name (or string description) given to the bucket.
  virtual std::string GetAsciiBucketRange(size_t it) const;

 private:
  // Allow tests to corrupt our innards for testing purposes.
  friend class HistogramTest;
  friend class HistogramThreadsafeTest;
  FRIEND_TEST_ALL_PREFIXES(HistogramTest, BoundsTest);
  FRIEND_TEST_ALL_PREFIXES(HistogramTest, BucketPlacementTest);
  FRIEND_TEST_ALL_PREFIXES(HistogramTest, CorruptSampleCounts);

  friend class StatisticsRecorder;  // To allow it to delete duplicates.
  friend class StatisticsRecorderTest;

  friend BASE_EXPORT HistogramBase* DeserializeHistogramInfo(
      base::PickleIterator* iter);
  static HistogramBase* DeserializeInfoImpl(base::PickleIterator* iter);

  static HistogramBase* FactoryGetInternal(std::string_view name,
                                           Sample32 minimum,
                                           Sample32 maximum,
                                           size_t bucket_count,
                                           int32_t flags);
  static HistogramBase* FactoryTimeGetInternal(std::string_view name,
                                               base::TimeDelta minimum,
                                               base::TimeDelta maximum,
                                               size_t bucket_count,
                                               int32_t flags);
  static HistogramBase* FactoryMicrosecondsTimeGetInternal(
      std::string_view name,
      base::TimeDelta minimum,
      base::TimeDelta maximum,
      size_t bucket_count,
      int32_t flags);

  // Create a snapshot containing all samples (both logged and unlogged).
  // Implementation of SnapshotSamples method with a more specific type for
  // internal use.
  std::unique_ptr<SampleVector> SnapshotAllSamples() const;

  // Returns a copy of unlogged samples as the underlying SampleVector class,
  // instead of the HistogramSamples base class. Used for tests and to avoid
  // virtual dispatch from some callsites.
  std::unique_ptr<SampleVector> SnapshotUnloggedSamplesImpl() const;

  // Writes the type, min, max, and bucket count information of the histogram in
  // |params|.
  Value::Dict GetParameters() const override;

  // Samples that have not yet been logged with SnapshotDelta().
  std::unique_ptr<SampleVectorBase> unlogged_samples_;

  // Accumulation of all samples that have been logged with SnapshotDelta().
  std::unique_ptr<SampleVectorBase> logged_samples_;

#if DCHECK_IS_ON()  // Don't waste memory if it won't be used.
  // Flag to indicate if PrepareFinalDelta has been previously called. It is
  // used to DCHECK that a final delta is not created multiple times.
  mutable bool final_delta_created_ = false;
#endif
};

//------------------------------------------------------------------------------

// LinearHistogram is a more traditional histogram, with evenly spaced
// buckets.
class BASE_EXPORT LinearHistogram : public Histogram {
 public:
  LinearHistogram(const LinearHistogram&) = delete;
  LinearHistogram& operator=(const LinearHistogram&) = delete;

  ~LinearHistogram() override;

  /* minimum should start from 1. 0 is as minimum is invalid. 0 is an implicit
     default underflow bucket. */
  static HistogramBase* FactoryGet(std::string_view name,
                                   Sample32 minimum,
                                   Sample32 maximum,
                                   size_t bucket_count,
                                   int32_t flags);
  static HistogramBase* FactoryTimeGet(std::string_view name,
                                       TimeDelta minimum,
                                       TimeDelta maximum,
                                       size_t bucket_count,
                                       int32_t flags);

  // Overloads of the above two functions that take a const std::string& or
  // const char* |name| param, to avoid code bloat from the std::string
  // constructor being inlined into call sites.
  static HistogramBase* FactoryGet(const std::string& name,
                                   Sample32 minimum,
                                   Sample32 maximum,
                                   size_t bucket_count,
                                   int32_t flags);
  static HistogramBase* FactoryTimeGet(const std::string& name,
                                       TimeDelta minimum,
                                       TimeDelta maximum,
                                       size_t bucket_count,
                                       int32_t flags);

  static HistogramBase* FactoryGet(const char* name,
                                   Sample32 minimum,
                                   Sample32 maximum,
                                   size_t bucket_count,
                                   int32_t flags);
  static HistogramBase* FactoryTimeGet(const char* name,
                                       TimeDelta minimum,
                                       TimeDelta maximum,
                                       size_t bucket_count,
                                       int32_t flags);

  // Create a histogram using data in persistent storage.
  static std::unique_ptr<HistogramBase> PersistentCreate(
      DurableStringView durable_name,
      const BucketRanges* ranges,
      const DelayedPersistentAllocation& counts,
      const DelayedPersistentAllocation& logged_counts,
      HistogramSamples::Metadata* meta,
      HistogramSamples::Metadata* logged_meta);

  struct DescriptionPair {
    Sample32 sample;
    const char* description;  // Null means end of a list of pairs.
  };

  // Create a LinearHistogram and store a list of number/text values for use in
  // writing the histogram graph.
  // |descriptions| can be NULL, which means no special descriptions to set. If
  // it's not NULL, the last element in the array must has a NULL in its
  // "description" field.
  static HistogramBase* FactoryGetWithRangeDescription(
      std::string_view name,
      Sample32 minimum,
      Sample32 maximum,
      size_t bucket_count,
      int32_t flags,
      const DescriptionPair descriptions[]);

  static void InitializeBucketRanges(Sample32 minimum,
                                     Sample32 maximum,
                                     BucketRanges* ranges);

  // Overridden from Histogram:
  HistogramType GetHistogramType() const override;

 protected:
  class Factory;

  LinearHistogram(DurableStringView durable_name, const BucketRanges* ranges);

  LinearHistogram(DurableStringView durable_name,
                  const BucketRanges* ranges,
                  const DelayedPersistentAllocation& counts,
                  const DelayedPersistentAllocation& logged_counts,
                  HistogramSamples::Metadata* meta,
                  HistogramSamples::Metadata* logged_meta);

  // If we have a description for a bucket, then return that.  Otherwise
  // let parent class provide a (numeric) description.
  std::string GetAsciiBucketRange(size_t i) const override;

 private:
  friend BASE_EXPORT HistogramBase* DeserializeHistogramInfo(
      base::PickleIterator* iter);
  static HistogramBase* DeserializeInfoImpl(base::PickleIterator* iter);

  static HistogramBase* FactoryGetInternal(std::string_view name,
                                           Sample32 minimum,
                                           Sample32 maximum,
                                           size_t bucket_count,
                                           int32_t flags);
  static HistogramBase* FactoryTimeGetInternal(std::string_view name,
                                               TimeDelta minimum,
                                               TimeDelta maximum,
                                               size_t bucket_count,
                                               int32_t flags);

  // For some ranges, we store a printable description of a bucket range.
  // If there is no description, then GetAsciiBucketRange() uses parent class
  // to provide a description.
  typedef std::map<Sample32, std::string> BucketDescriptionMap;
  BucketDescriptionMap bucket_description_;
};

//------------------------------------------------------------------------------

// ScaledLinearHistogram is a wrapper around a linear histogram that scales the
// counts down by some factor. Remainder values are kept locally but lost when
// uploaded or serialized. The integral counts are rounded up/down so should
// average to the correct value when many reports are added.
//
// This is most useful when adding many counts at once via AddCount() that can
// cause overflows of the 31-bit counters, usually with an enum as the value.
class BASE_EXPORT ScaledLinearHistogram {
  using AtomicCount = Histogram::AtomicCount;
  using Sample32 = Histogram::Sample32;

 public:
  // Currently only works with "exact" linear histograms: minimum=1, maximum=N,
  // and bucket_count=N+1.
  ScaledLinearHistogram(std::string_view name,
                        Sample32 minimum,
                        Sample32 maximum,
                        size_t bucket_count,
                        int32_t scale,
                        int32_t flags);

  // Overload of the above function that take a const std::string& or const
  // char* |name| param, to avoid code bloat from the std::string constructor
  // being inlined into call sites.
  ScaledLinearHistogram(const char* name,
                        Sample32 minimum,
                        Sample32 maximum,
                        size_t bucket_count,
                        int32_t scale,
                        int32_t flags);
  ScaledLinearHistogram(const std::string& name,
                        Sample32 minimum,
                        Sample32 maximum,
                        size_t bucket_count,
                        int32_t scale,
                        int32_t flags);

  ScaledLinearHistogram(const ScaledLinearHistogram&) = delete;
  ScaledLinearHistogram& operator=(const ScaledLinearHistogram&) = delete;

  ~ScaledLinearHistogram();

  // Like AddCount() but actually accumulates |count|/|scale| and increments
  // the accumulated remainder by |count|%|scale|. An additional increment
  // is done when the remainder has grown sufficiently large.
  // The value after scaling must fit into 32-bit signed integer.
  void AddScaledCount(Sample32 value, int64_t count);

  int32_t scale() const { return scale_; }
  HistogramBase* histogram() { return histogram_; }

 private:
  // Pointer to the underlying histogram. Ownership of it remains with
  // the statistics-recorder. This is typed as HistogramBase because it may be a
  // DummyHistogram if expired.
  const raw_ptr<HistogramBase> histogram_;

  // The scale factor of the sample counts.
  const int32_t scale_;

  // A vector of "remainder" counts indexed by bucket number. These values
  // may be negative as the scaled count is actually bumped once the
  // remainder is 1/2 way to the scale value (thus "rounding").
  std::vector<AtomicCount> remainders_;
};

//------------------------------------------------------------------------------

// BooleanHistogram is a histogram for booleans.
class BASE_EXPORT BooleanHistogram : public LinearHistogram {
 public:
  static HistogramBase* FactoryGet(std::string_view name, int32_t flags);

  // Overload of the above function that take a const std::string& or const
  // char* |name| param, to avoid code bloat from the std::string constructor
  // being inlined into call sites.
  static HistogramBase* FactoryGet(const std::string& name, int32_t flags);
  static HistogramBase* FactoryGet(const char* name, int32_t flags);

  BooleanHistogram(const BooleanHistogram&) = delete;
  BooleanHistogram& operator=(const BooleanHistogram&) = delete;

  // Create a histogram using data in persistent storage.
  static std::unique_ptr<HistogramBase> PersistentCreate(
      DurableStringView durable_name,
      const BucketRanges* ranges,
      const DelayedPersistentAllocation& counts,
      const DelayedPersistentAllocation& logged_counts,
      HistogramSamples::Metadata* meta,
      HistogramSamples::Metadata* logged_meta);

  HistogramType GetHistogramType() const override;

 protected:
  class Factory;

 private:
  static HistogramBase* FactoryGetInternal(std::string_view name,
                                           int32_t flags);

  BooleanHistogram(DurableStringView durable_name, const BucketRanges* ranges);
  BooleanHistogram(DurableStringView durable_name,
                   const BucketRanges* ranges,
                   const DelayedPersistentAllocation& counts,
                   const DelayedPersistentAllocation& logged_counts,
                   HistogramSamples::Metadata* meta,
                   HistogramSamples::Metadata* logged_meta);

  friend BASE_EXPORT HistogramBase* DeserializeHistogramInfo(
      base::PickleIterator* iter);
  static HistogramBase* DeserializeInfoImpl(base::PickleIterator* iter);
};

//------------------------------------------------------------------------------

// CustomHistogram is a histogram for a set of custom integers.
class BASE_EXPORT CustomHistogram : public Histogram {
 public:
  // |custom_ranges| contains a vector of limits on ranges. Each limit should be
  // > 0 and < kSampleType_MAX. (Currently 0 is still accepted for backward
  // compatibility). The limits can be unordered or contain duplication, but
  // client should not depend on this.
  static HistogramBase* FactoryGet(std::string_view name,
                                   const std::vector<Sample32>& custom_ranges,
                                   int32_t flags);

  // Overload of the above function that take a const std::string& or const
  // char* |name| param, to avoid code bloat from the std::string constructor
  // being inlined into call sites.
  static HistogramBase* FactoryGet(const std::string& name,
                                   const std::vector<Sample32>& custom_ranges,
                                   int32_t flags);
  static HistogramBase* FactoryGet(const char* name,
                                   const std::vector<Sample32>& custom_ranges,
                                   int32_t flags);

  CustomHistogram(const CustomHistogram&) = delete;
  CustomHistogram& operator=(const CustomHistogram&) = delete;

  // Create a histogram using data in persistent storage.
  static std::unique_ptr<HistogramBase> PersistentCreate(
      DurableStringView durable_name,
      const BucketRanges* ranges,
      const DelayedPersistentAllocation& counts,
      const DelayedPersistentAllocation& logged_counts,
      HistogramSamples::Metadata* meta,
      HistogramSamples::Metadata* logged_meta);

  // Overridden from Histogram:
  HistogramType GetHistogramType() const override;

  // Helper method for transforming an array of valid enumeration values
  // to the std::vector<int> expected by UMA_HISTOGRAM_CUSTOM_ENUMERATION.
  // This function ensures that a guard bucket exists right after any
  // valid sample value (unless the next higher sample is also a valid value),
  // so that invalid samples never fall into the same bucket as valid samples.
  static std::vector<Sample32> ArrayToCustomEnumRanges(
      base::span<const Sample32> values);

 protected:
  class Factory;

  CustomHistogram(DurableStringView durable_name, const BucketRanges* ranges);

  CustomHistogram(DurableStringView durable_name,
                  const BucketRanges* ranges,
                  const DelayedPersistentAllocation& counts,
                  const DelayedPersistentAllocation& logged_counts,
                  HistogramSamples::Metadata* meta,
                  HistogramSamples::Metadata* logged_meta);

  // HistogramBase implementation:
  void SerializeInfoImpl(base::Pickle* pickle) const override;

 private:
  friend BASE_EXPORT HistogramBase* DeserializeHistogramInfo(
      base::PickleIterator* iter);
  static HistogramBase* DeserializeInfoImpl(base::PickleIterator* iter);

  static HistogramBase* FactoryGetInternal(
      std::string_view name,
      const std::vector<Sample32>& custom_ranges,
      int32_t flags);

  static bool ValidateCustomRanges(const std::vector<Sample32>& custom_ranges);
};

namespace internal {

// Controls whether invocations of UMA_HISTOGRAM_SPLIT_BY_PROCESS_PRIORITY in
// this process log to their ".BestEffort" suffix or not. Timing metrics
// reported through UMA_HISTOGRAM_SPLIT_BY_PROCESS_PRIORITY which overlap a
// best-effort range will be suffixed with ".BestEffort".
BASE_EXPORT void SetSharedLastForegroundTimeForMetrics(
    const std::atomic<TimeTicks>* last_foreground_time_ref);

// Returns the pointer passed to SetSharedLastForegroundTimeForMetrics, or
// nullptr if it was never called.
BASE_EXPORT const std::atomic<TimeTicks>*
GetSharedLastForegroundTimeForMetricsForTesting();

// Reports whether the interval [`now - range`, `now`] overlaps with a period
// where this process was running at Process::Priority::kBestEffort. Defaults to
// false if `last_foreground_time_ref` was never set (e.g. in processes not
// affected by priorities) but otherwise defaults to true if there's ambiguity
// (might have overlapped a best-effort range; as the reported timing might have
// been affected and shouldn't be reported as "definitely measured in
// foreground").
// This method is atomic and suitable for performance critical histogram
// samples.
BASE_EXPORT bool OverlapsBestEffortRange(TimeTicks now, TimeDelta range);

}  // namespace internal

}  // namespace base

#endif  // BASE_METRICS_HISTOGRAM_H_
