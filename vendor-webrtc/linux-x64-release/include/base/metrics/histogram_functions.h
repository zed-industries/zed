// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_HISTOGRAM_FUNCTIONS_H_
#define BASE_METRICS_HISTOGRAM_FUNCTIONS_H_

#include <stdint.h>

#include <string>
#include <string_view>
#include <type_traits>

#include "base/base_export.h"
#include "base/check_op.h"
#include "base/metrics/histogram.h"
#include "base/metrics/histogram_base.h"
#include "base/metrics/histogram_functions_internal_overloads.h"  // IWYU pragma: export
#include "base/time/time.h"

// TODO(crbug.com/40801421): Update this file's function comments to provide
// more detail, like histogram_macros.h.
//
// Functions for recording metrics.
//
// For best practices on deciding when to emit to a histogram and what form
// the histogram should take, see
// https://chromium.googlesource.com/chromium/src.git/+/HEAD/tools/metrics/histograms/README.md
//
// For deciding whether to use the function or macro APIs, see
// https://chromium.googlesource.com/chromium/src/+/HEAD/tools/metrics/histograms/README.md#coding-emitting-to-histograms"
//
// Every function is duplicated to also support both std::string and char* for
// the name for improved binary size. These declarations are moved to a separate
// header for readability, see
// https://chromium.googlesource.com/chromium/src/+/HEAD/base/metrics/histogram_functions_internal_overloads.h.
namespace base {

// For numeric measurements where you want exact integer values up to
// |exclusive_max|. |exclusive_max| itself is included in the overflow bucket.
// Therefore, if you want an accurate measure up to kMax, then |exclusive_max|
// should be set to kMax + 1.
//
// |exclusive_max| should be 101 or less. If you need to capture a larger range,
// we recommend the use of the COUNT histograms below.
//
// Sample usage:
//   base::UmaHistogramExactLinear("Histogram.Linear", sample, kMax + 1);
// In this case, buckets are 1, 2, .., kMax, kMax+1, where the kMax+1 bucket
// captures everything kMax+1 and above.
// LINT.IfChange(UmaHistogramExactLinear)
BASE_EXPORT void UmaHistogramExactLinear(std::string_view name,
                                         int sample,
                                         int exclusive_max);
// LINT.ThenChange(/base/metrics/histogram_functions_internal_overloads.h:UmaHistogramExactLinear)

// For adding a sample to an enumerated histogram.
// Sample usage:
//   // These values are persisted to logs. Entries should not be renumbered and
//   // numeric values should never be reused.
//   enum class NewTabPageAction {
//     kUseOmnibox = 0,
//     kClickTitle = 1,
//     // kUseSearchbox = 2,  // no longer used, combined into omnibox
//     kOpenBookmark = 3,
//     kMaxValue = kOpenBookmark,
//   };
//   base::UmaHistogramEnumeration("My.Enumeration",
//                                 NewTabPageAction::kClickTitle);
//
// `kMaxValue` should be 1000 or less.
// Note that there is code that refers to implementation details of this
// function. Keep it synchronized.
// LINT.IfChange(UmaHistogramEnumeration)
template <typename T>
void UmaHistogramEnumeration(std::string_view name, T sample) {
  static_assert(std::is_enum_v<T>, "T is not an enum.");
  // This also ensures that an enumeration that doesn't define kMaxValue fails
  // with a semi-useful error ("no member named 'kMaxValue' in ...").
  static_assert(static_cast<uintmax_t>(T::kMaxValue) <=
                    static_cast<uintmax_t>(INT_MAX) - 1,
                "Enumeration's kMaxValue is out of range of INT_MAX!");
  DCHECK_LE(static_cast<uintmax_t>(sample),
            static_cast<uintmax_t>(T::kMaxValue));
  // While UmaHistogramExactLinear’s documentation states that the third
  // argument should be 101 or less, a value up to 1001 is actually accepted.
  return UmaHistogramExactLinear(name, static_cast<int>(sample),
                                 static_cast<int>(T::kMaxValue) + 1);
}

// Some legacy histograms may manually specify the enum size, with a kCount,
// COUNT, kMaxValue, or MAX_VALUE sentinel like so:
//   // These values are persisted to logs. Entries should not be renumbered and
//   // numeric values should never be reused.
//   enum class NewTabPageAction {
//     kUseOmnibox = 0,
//     kClickTitle = 1,
//     // kUseSearchbox = 2,  // no longer used, combined into omnibox
//     kOpenBookmark = 3,
//     kCount,
//   };
//   base::UmaHistogramEnumeration("My.Enumeration",
//                                 NewTabPageAction::kClickTitle,
//                                 kCount);
// Note: The value in |sample| must be strictly less than |enum_size|. This is
// otherwise functionally equivalent to the above.
// `enum_size` must be less than or equal to 1001.
template <typename T>
void UmaHistogramEnumeration(std::string_view name, T sample, T enum_size) {
  static_assert(std::is_enum_v<T>, "T is not an enum.");
  DCHECK_LE(static_cast<uintmax_t>(enum_size), static_cast<uintmax_t>(INT_MAX));
  DCHECK_LT(static_cast<uintmax_t>(sample), static_cast<uintmax_t>(enum_size));
  // While UmaHistogramExactLinear’s documentation states that the third
  // argument should be 101 or less, a value up to 1001 is actually accepted.
  return UmaHistogramExactLinear(name, static_cast<int>(sample),
                                 static_cast<int>(enum_size));
}
// LINT.ThenChange(/base/metrics/histogram_functions_internal_overloads.h:UmaHistogramEnumeration)

// For adding a boolean sample to histogram.
// Sample usage:
//   base::UmaHistogramBoolean("My.Boolean", true)
// LINT.IfChange(UmaHistogramBoolean)
BASE_EXPORT void UmaHistogramBoolean(std::string_view name, bool sample);
// LINT.ThenChange(/base/metrics/histogram_functions_internal_overloads.h:UmaHistogramBoolean)

// For adding a histogram sample denoting a percentage.
// Percents are integers between 1 and 100, inclusively.
// Sample usage:
//   base::UmaHistogramPercentage("My.Percent", 69)
// LINT.IfChange(UmaHistogramPercentage)
BASE_EXPORT void UmaHistogramPercentage(std::string_view name, int percent);

// Obsolete. Use |UmaHistogramPercentage| instead. See crbug/1121318.
BASE_EXPORT void UmaHistogramPercentageObsoleteDoNotUse(std::string_view name,
                                                        int percent);
// LINT.ThenChange(/base/metrics/histogram_functions_internal_overloads.h:UmaHistogramPercentage)

// For adding a counts histogram.
// Sample usage:
//   base::UmaHistogramCustomCounts("My.Counts", some_value, 1, 600, 30)
// LINT.IfChange(UmaHistogramCounts)
BASE_EXPORT void UmaHistogramCustomCounts(std::string_view name,
                                          int sample,
                                          int min,
                                          int exclusive_max,
                                          size_t buckets);

// Counts specialization for maximum counts 100, 1000, 10k, 100k, 1M and 10M.
BASE_EXPORT void UmaHistogramCounts100(std::string_view name, int sample);
BASE_EXPORT void UmaHistogramCounts1000(std::string_view name, int sample);
BASE_EXPORT void UmaHistogramCounts10000(std::string_view name, int sample);
BASE_EXPORT void UmaHistogramCounts100000(std::string_view name, int sample);
BASE_EXPORT void UmaHistogramCounts1M(std::string_view name, int sample);
BASE_EXPORT void UmaHistogramCounts10M(std::string_view name, int sample);
// LINT.ThenChange(/base/metrics/histogram_functions_internal_overloads.h:UmaHistogramCounts)

// For histograms storing times. Uses milliseconds granularity.
// LINT.IfChange(UmaHistogramTimes)
BASE_EXPORT void UmaHistogramCustomTimes(std::string_view name,
                                         TimeDelta sample,
                                         TimeDelta min,
                                         TimeDelta max,
                                         size_t buckets);

// Reference ScopedUmaHistogramTimer::ScopedHistogramTiming for timing.
// For short timings from 1 ms up to 10 seconds (50 buckets).
BASE_EXPORT void UmaHistogramTimes(std::string_view name, TimeDelta sample);

// For medium timings up to 3 minutes (50 buckets).
// TODO(crbug.com/353712922): rename and disambiguate this function/macro
BASE_EXPORT void UmaHistogramMediumTimes(std::string_view name,
                                         TimeDelta sample);
// For time intervals up to 1 hr (50 buckets).
BASE_EXPORT void UmaHistogramLongTimes(std::string_view name, TimeDelta sample);

// For time intervals up to 1 hr (100 buckets).
BASE_EXPORT void UmaHistogramLongTimes100(std::string_view name,
                                          TimeDelta sample);
// LINT.ThenChange(/base/metrics/histogram_functions_internal_overloads.h:UmaHistogramTimes)

// For histograms storing times with microseconds granularity.
// LINT.IfChange(UmaHistogramMicrosecondsTimes)
BASE_EXPORT void UmaHistogramCustomMicrosecondsTimes(std::string_view name,
                                                     TimeDelta sample,
                                                     TimeDelta min,
                                                     TimeDelta max,
                                                     size_t buckets);

// For microseconds timings from 1 microsecond up to 10 seconds (50 buckets).
BASE_EXPORT void UmaHistogramMicrosecondsTimes(std::string_view name,
                                               TimeDelta sample);
// LINT.ThenChange(/base/metrics/histogram_functions_internal_overloads.h:UmaHistogramMicrosecondsTimes)

// For recording memory-related histograms.
// LINT.IfChange(UmaHistogramMemory)
//
// Used to measure common KB-granularity memory stats. Sample is in KB. Range is
// from 1000KB (see crbug.com/40526504) to 500M. For measuring sizes less than
// 1000K, use `UmaHistogramCounts`.
BASE_EXPORT void UmaHistogramMemoryKB(std::string_view name, int sample);
// Used to measure common MB-granularity memory stats. Sample is in MB. Range is
// 1MB to ~1G.
BASE_EXPORT void UmaHistogramMemoryMB(std::string_view name, int sample);
// Used to measure common MB-granularity memory stats. Range is 1G to ~64G.
BASE_EXPORT void UmaHistogramMemoryLargeMB(std::string_view name, int sample);
// LINT.ThenChange(/base/metrics/histogram_functions_internal_overloads.h:UmaHistogramMemory)

// For recording sparse histograms.
// The |sample| can be a negative or non-negative number.
//
// Sparse histograms are well-suited for recording counts of exact sample values
// that are sparsely distributed over a relatively large range, in cases where
// ultra-fast performance is not critical. For instance, Sqlite.Version.* are
// sparse because for any given database, there's going to be exactly one
// version logged.
//
// Performance:
// ------------
// Sparse histograms are typically more memory-efficient but less time-efficient
// than other histograms. Essentially, they sparse histograms use a map rather
// than a vector for their backing storage; they also require lock acquisition
// to increment a sample, whereas other histogram do not. Hence, each increment
// operation is a bit slower than for other histograms. But, if the data is
// sparse, then they use less memory client-side, because they allocate buckets
// on demand rather than preallocating.
//
// Data size:
// ----------
// Note that server-side, we still need to load all buckets, across all users,
// at once. Thus, please avoid exploding such histograms, i.e. uploading many
// many distinct values to the server (across all users). Concretely, keep the
// number of distinct values <= 100 ideally, definitely <= 1000. If you have no
// guarantees on the range of your data, use clamping, e.g.:
//   UmaHistogramSparse("My.Histogram", std::clamp(value, 0, 200));
// LINT.IfChange(UmaHistogramSparse)
BASE_EXPORT void UmaHistogramSparse(std::string_view name, int sample);
// LINT.ThenChange(/base/metrics/histogram_functions_internal_overloads.h:UmaHistogramSparse)

// Scoped class which logs its time on this earth in milliseconds as an UMA
// histogram. This is recommended for when you want a histogram which measures
// the time it takes for a method to execute. It uses UmaHistogramTimes() and
// its variations under the hood.
//
// This is equivalent to SCOPED_UMA_HISTOGRAM_TIMER.
//
// Sample usages:
//   void Function() {
//     ScopedUmaHistogramTimer("Component.FunctionTime");
//     // useful stuff here
//     ...
//   }
//
//   void Function() {
//     ScopedUmaHistogramTimer("Component.FunctionTime",
//       ScopedUmaHistogramTimer::kMicroSecondTimes);
//     // useful stuff here
//     ...
//   }
class BASE_EXPORT ScopedUmaHistogramTimer {
 public:
  // Reference UmaHistogramTiming() function declarations for timing details
  // below.
  enum class ScopedHistogramTiming {
    // For microseconds timings from 1 microsecond up to 10 seconds (50
    // buckets).
    kMicrosecondTimes,
    // For short timings from 1 ms up to 10 seconds (50 buckets).
    kShortTimes,
    // For medium timings up to 3 minutes (50 buckets).
    kMediumTimes,
    // For time intervals up to 1 hr (50 buckets).
    kLongTimes
  };

  // Constructs the scoped timer with the given histogram name.
  explicit ScopedUmaHistogramTimer(
      std::string_view name,
      ScopedHistogramTiming timing = ScopedHistogramTiming::kShortTimes);

  ScopedUmaHistogramTimer(const ScopedUmaHistogramTimer&) = delete;
  ScopedUmaHistogramTimer& operator=(const ScopedUmaHistogramTimer&) = delete;

  ~ScopedUmaHistogramTimer();

 private:
  const base::TimeTicks constructed_;
  const ScopedHistogramTiming timing_;
  const std::string name_;
};

}  // namespace base

#endif  // BASE_METRICS_HISTOGRAM_FUNCTIONS_H_
