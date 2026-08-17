// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_METRICS_HISTOGRAM_TESTER_H_
#define BASE_TEST_METRICS_HISTOGRAM_TESTER_H_

#include <functional>
#include <map>
#include <memory>
#include <ostream>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>
#include <vector>

#include "base/location.h"
#include "base/metrics/histogram.h"
#include "base/metrics/histogram_base.h"
#include "base/time/time.h"
#include "testing/gmock/include/gmock/gmock.h"

namespace base {

struct Bucket;
class HistogramSamples;

// HistogramTester provides a simple interface for examining histograms, UMA
// or otherwise. Tests can use this interface to verify that histogram data is
// getting logged as intended.
//
// Note: When using this class from a browser test, one might have to call
// SubprocessMetricsProvider::MergeHistogramDeltasForTesting() to sync the
// histogram data between the renderer and browser processes. If it is in a
// content browser test, then content::FetchHistogramsFromChildProcesses()
// should be used to achieve that.
// To test histograms in Java tests, use HistogramWatcher.
class HistogramTester {
 public:
  using CountsMap = std::map<std::string, HistogramBase::Count32, std::less<>>;

  // Takes a snapshot of all current histograms counts.
  HistogramTester();

  HistogramTester(const HistogramTester&) = delete;
  HistogramTester& operator=(const HistogramTester&) = delete;

  ~HistogramTester();

  // EXPECTs that the number of samples in bucket |sample| of histogram |name|
  // grew by |expected_bucket_count| since the HistogramTester was created and
  // that no other bucket of the histogram gained any extra samples.
  // If a bucket had samples before the HistogramTester was created, these
  // samples are completely ignored.
  void ExpectUniqueSample(std::string_view name,
                          HistogramBase::Sample32 sample,
                          HistogramBase::Count32 expected_bucket_count,
                          const Location& location = FROM_HERE) const;
  template <typename T>
  void ExpectUniqueSample(std::string_view name,
                          T sample,
                          HistogramBase::Count32 expected_bucket_count,
                          const Location& location = FROM_HERE) const {
    ExpectUniqueSample(name, static_cast<HistogramBase::Sample32>(sample),
                       expected_bucket_count, location);
  }
  void ExpectUniqueTimeSample(std::string_view name,
                              TimeDelta sample,
                              HistogramBase::Count32 expected_bucket_count,
                              const Location& location = FROM_HERE) const;

  // EXPECTs that the number of samples in bucket |sample| of histogram |name|
  // grew by |expected_count| since the HistogramTester was created. Samples in
  // other buckets are ignored.
  void ExpectBucketCount(std::string_view name,
                         HistogramBase::Sample32 sample,
                         HistogramBase::Count32 expected_count,
                         const Location& location = FROM_HERE) const;
  template <typename T>
  void ExpectBucketCount(std::string_view name,
                         T sample,
                         HistogramBase::Count32 expected_count,
                         const Location& location = FROM_HERE) const {
    ExpectBucketCount(name, static_cast<HistogramBase::Sample32>(sample),
                      expected_count, location);
  }
  void ExpectTimeBucketCount(std::string_view name,
                             TimeDelta sample,
                             HistogramBase::Count32 expected_count,
                             const Location& location = FROM_HERE) const;

  // EXPECTs that the total number of samples in histogram |name|
  // grew by |expected_count| since the HistogramTester was created.
  void ExpectTotalCount(std::string_view name,
                        HistogramBase::Count32 expected_count,
                        const Location& location = FROM_HERE) const;

  // Returns the sum of all samples recorded since the HistogramTester was
  // created.
  int64_t GetTotalSum(std::string_view name) const;

  // Returns a list of all of the buckets recorded since creation of this
  // object, as vector<Bucket>, where the Bucket represents the min boundary of
  // the bucket and the count of samples recorded to that bucket since creation.
  //
  // Note: The histogram defines the bucket boundaries. If you test a histogram
  // with exponential bucket sizes, this function may not be particularly useful
  // because you would need to guess the bucket boundaries.
  //
  // Example usage, using gMock:
  //   EXPECT_THAT(histogram_tester.GetAllSamples("HistogramName"),
  //               ElementsAre(Bucket(1, 5), Bucket(2, 10), Bucket(3, 5)));
  //
  // If you want make empty bucket explicit, use the BucketsAre() matcher
  // defined below:
  //   EXPECT_THAT(histogram_tester.GetAllSamples("HistogramName"),
  //               BucketsAre(Bucket(1, 0), Bucket(2, 10), Bucket(3, 5)));
  //
  // If you want to test a superset relation, prefer BucketsInclude() over
  // IsSupersetOf() because the former handles empty buckets as expected:
  //   EXPECT_THAT(histogram_tester.GetAllSamples("HistogramName"),
  //               BucketsInclude(Bucket(1, 0), Bucket(2, 10), Bucket(3, 5)));
  // With IsSupersetOf(), this expectation would always fail because
  // GetAllSamples() does not contain empty buckets.
  //
  // If you build the expected list programmatically, you can use the matchers
  // ElementsAreArray(), BucketsAreArray(), BucketsIncludeArray().
  //
  // If you prefer not to depend on gMock at the expense of a slightly less
  // helpful failure message, use EXPECT_EQ:
  //   EXPECT_EQ(expected_buckets,
  //             histogram_tester.GetAllSamples("HistogramName"));
  std::vector<Bucket> GetAllSamples(std::string_view name) const;

  // Returns the value of the |sample| bucket for ths histogram |name|.
  HistogramBase::Count32 GetBucketCount(std::string_view name,
                                      HistogramBase::Sample32 sample) const;
  template <typename T>
  HistogramBase::Count32 GetBucketCount(std::string_view name, T sample) const {
    return GetBucketCount(name, static_cast<HistogramBase::Sample32>(sample));
  }

  // Finds histograms whose names start with |prefix|, and returns them along
  // with the counts of any samples added since the creation of this object.
  // Histograms that are unchanged are omitted from the result. The return value
  // is a map whose keys are the histogram name, and whose values are the sample
  // count.
  //
  // This is useful for cases where the code under test is choosing among a
  // family of related histograms and incrementing one of them. Typically you
  // should pass the result of this function directly to EXPECT_THAT.
  //
  // Example usage, using gmock (which produces better failure messages):
  //   #include "testing/gmock/include/gmock/gmock.h"
  // ...
  //   base::HistogramTester::CountsMap expected_counts;
  //   expected_counts["MyMetric.A"] = 1;
  //   expected_counts["MyMetric.B"] = 1;
  //   EXPECT_THAT(histogram_tester.GetTotalCountsForPrefix("MyMetric."),
  //               testing::ContainerEq(expected_counts));
  CountsMap GetTotalCountsForPrefix(std::string_view prefix) const;

  // Returns the HistogramSamples recorded since the creation of the
  // HistogramTester.
  std::unique_ptr<HistogramSamples> GetHistogramSamplesSinceCreation(
      std::string_view histogram_name) const;

  // Dumps all histograms that have had new samples added to them into a string,
  // for debugging purposes. Note: this will dump the entire contents of any
  // modified histograms and not just the modified buckets.
  std::string GetAllHistogramsRecorded() const;

 private:
  // Returns the total number of values recorded for |histogram| since the
  // HistogramTester was created.
  int GetTotalCountForSamples(const HistogramBase& histogram) const;

  // Sets |*sample_count| to number of samples by which bucket |sample| bucket
  // grew since the HistogramTester was created. If |total_count| is non-null,
  // sets |*total_count| to the number of samples recorded for |histogram|
  // since the HistogramTester was created.
  void GetBucketCountForSamples(const HistogramBase& histogram,
                                HistogramBase::Sample32 sample,
                                HistogramBase::Count32* sample_count,
                                HistogramBase::Count32* total_count) const;

  // Returns the deltas for |histogram| since the HistogramTester was created
  // as an ASCII art histogram for debugging purposes.
  std::string SnapshotToString(const HistogramBase& histogram) const;

  // Snapshot of all histograms recorded before the HistogramTester was created.
  // Used to determine the histogram changes made during this instance's
  // lifecycle.
  std::map<std::string, std::unique_ptr<HistogramSamples>, std::less<>>
      histograms_snapshot_;
};

struct Bucket {
  Bucket(HistogramBase::Sample32 min, HistogramBase::Count32 count)
      : min(min), count(count) {}

  // A variant of the above constructor that accepts an `EnumType` for the `min`
  // value. Typically, this `EnumType` is the C++ enum (class) that is
  // associated with the metric this bucket is referring to.
  //
  // The constructor forwards to the above non-templated constructor. Therefore,
  // `EnumType` must be implicitly convertible to `HistogramBase::Sample32`.
  template <typename MetricEnum>
    requires(std::is_enum_v<MetricEnum>)
  Bucket(MetricEnum min, HistogramBase::Count32 count)
      : Bucket(static_cast<std::underlying_type_t<MetricEnum>>(min), count) {}

  friend bool operator==(const Bucket&, const Bucket&) = default;

  HistogramBase::Sample32 min;
  HistogramBase::Count32 count;
};

void PrintTo(const Bucket& value, std::ostream* os);

// The BucketsAre[Array]() and BucketsInclude[Array]() matchers are intended to
// match GetAllSamples().
//
// Unlike the standard matchers UnorderedElementsAreArray() and IsSupersetOf(),
// they explicitly support empty buckets (`Bucket::Count32 == 0`). Empty buckets
// need special handling because GetAllSamples() doesn't contain empty ones.

// BucketsAre() and BucketsAreArray() match a container that contains exactly
// the non-empty `buckets`.
//
// For example,
//   EXPECT_THAT(histogram_tester.GetAllSamples("HistogramName"),
//               BucketsAre(Bucket(Enum::A, 0),
//                          Bucket(Enum::B, 1),
//                          Bucket(Enum::C, 2)));
// - matches the actual value `{Bucket(B, 1), Bucket(C, 2)}`;
// - does not match `{Bucket(A, n), Bucket(B, 1), Bucket(C, 2)}` for any `n`
//   (including `n == 0`).
template <typename BucketArray>
auto BucketsAreArray(BucketArray buckets) {
  auto non_empty_buckets = buckets;
  std::erase_if(non_empty_buckets, [](Bucket b) { return b.count == 0; });
  return ::testing::UnorderedElementsAreArray(non_empty_buckets);
}

template <typename... BucketTypes>
auto BucketsAre(BucketTypes... buckets) {
  return BucketsAreArray(std::vector<Bucket>{buckets...});
}

// BucketsInclude() and BucketsIncludeArray() are empty-bucket-friendly
// replacements of IsSupersetOf[Array](): they match a container that contains
// all non-empty `buckets` and none of the empty `buckets`.
//
// For example,
//   EXPECT_THAT(histogram_tester.GetAllSamples("HistogramName"),
//               BucketsInclude(Bucket(Enum::A, 0),
//                              Bucket(Enum::B, 1),
//                              Bucket(Enum::C, 2)));
// - matches `{Bucket(B, 1), Bucket(C, 2), Bucket(D, 3)}`;
// - not match `{Bucket(A, n), Bucket(B, 1), Bucket(C, 2), Bucket(D, 3)}` for
//   any `n` (including `n == 0`).
template <typename BucketArray>
auto BucketsIncludeArray(const BucketArray& buckets) {
  // The `empty_buckets` have `count == 0`, so the `HistogramBase::Sample32`
  // suffices.
  std::vector<HistogramBase::Sample32> empty_buckets;
  std::vector<Bucket> non_empty_buckets;
  for (const Bucket& b : buckets) {
    if (b.count == 0) {
      empty_buckets.push_back(b.min);
    } else {
      non_empty_buckets.push_back(b);
    }
  }
  using ::testing::AllOf;
  using ::testing::AnyOfArray;
  using ::testing::Each;
  using ::testing::Field;
  using ::testing::IsSupersetOf;
  using ::testing::Not;
  return AllOf(
      IsSupersetOf(non_empty_buckets),
      Each(Field("Bucket::min", &Bucket::min, Not(AnyOfArray(empty_buckets)))));
}

template <typename... BucketTypes>
auto BucketsInclude(BucketTypes... buckets) {
  return BucketsIncludeArray(std::vector<Bucket>{buckets...});
}

}  // namespace base

#endif  // BASE_TEST_METRICS_HISTOGRAM_TESTER_H_
