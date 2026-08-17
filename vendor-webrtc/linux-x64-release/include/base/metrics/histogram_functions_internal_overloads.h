// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_HISTOGRAM_FUNCTIONS_INTERNAL_OVERLOADS_H_
#define BASE_METRICS_HISTOGRAM_FUNCTIONS_INTERNAL_OVERLOADS_H_

// IWYU pragma: private, include "base/metrics/histogram_functions.h"

#include <stdint.h>

#include <string>
#include <string_view>
#include <type_traits>

#include "base/base_export.h"
#include "base/check_op.h"
#include "base/metrics/histogram.h"
#include "base/metrics/histogram_base.h"
#include "base/time/time.h"

// Do not include this header directly.
//
// This file provides overloads for the functions defined in
// histogram_functions.h. These functions are duplicated to also support both
// std::string and char* for the name. This avoids extra ctors/dtors at call
// sites. As of Dec. 18, 2024, having all the overloads saves about 45KiB in
// binary size for 32-bit Android, at the cost of a total (across several files)
// of about 800 lines of boilerplate that doesn't need significant maintenance.
//
// These overloads are in a separate header for readability. See the main header
// for documentation:
// https://chromium.googlesource.com/chromium/src/+/HEAD/base/metrics/histogram_functions.h.

namespace base {
// LINT.IfChange(UmaHistogramExactLinear)
BASE_EXPORT void UmaHistogramExactLinear(const std::string& name,
                                         int sample,
                                         int exclusive_max);
BASE_EXPORT void UmaHistogramExactLinear(const char* name,
                                         int sample,
                                         int exclusive_max);
// LINT.ThenChange(/base/metrics/histogram_functions.h:UmaHistogramExactLinear)

// LINT.IfChange(UmaHistogramEnumeration)
template <typename T>
void UmaHistogramEnumeration(const std::string& name, T sample) {
  static_assert(std::is_enum_v<T>, "T is not an enum.");
  // This also ensures that an enumeration that doesn't define kMaxValue fails
  // with a semi-useful error ("no member named 'kMaxValue' in ...").
  static_assert(static_cast<uintmax_t>(T::kMaxValue) <=
                    static_cast<uintmax_t>(INT_MAX) - 1,
                "Enumeration's kMaxValue is out of range of INT_MAX!");
  DCHECK_LE(static_cast<uintmax_t>(sample),
            static_cast<uintmax_t>(T::kMaxValue));
  return UmaHistogramExactLinear(name, static_cast<int>(sample),
                                 static_cast<int>(T::kMaxValue) + 1);
}

template <typename T>
void UmaHistogramEnumeration(const char* name, T sample) {
  static_assert(std::is_enum_v<T>, "T is not an enum.");
  // This also ensures that an enumeration that doesn't define kMaxValue fails
  // with a semi-useful error ("no member named 'kMaxValue' in ...").
  static_assert(static_cast<uintmax_t>(T::kMaxValue) <=
                    static_cast<uintmax_t>(INT_MAX) - 1,
                "Enumeration's kMaxValue is out of range of INT_MAX!");
  DCHECK_LE(static_cast<uintmax_t>(sample),
            static_cast<uintmax_t>(T::kMaxValue));
  return UmaHistogramExactLinear(name, static_cast<int>(sample),
                                 static_cast<int>(T::kMaxValue) + 1);
}

template <typename T>
void UmaHistogramEnumeration(const std::string& name, T sample, T enum_size) {
  static_assert(std::is_enum_v<T>, "T is not an enum.");
  DCHECK_LE(static_cast<uintmax_t>(enum_size), static_cast<uintmax_t>(INT_MAX));
  DCHECK_LT(static_cast<uintmax_t>(sample), static_cast<uintmax_t>(enum_size));
  return UmaHistogramExactLinear(name, static_cast<int>(sample),
                                 static_cast<int>(enum_size));
}

template <typename T>
void UmaHistogramEnumeration(const char* name, T sample, T enum_size) {
  static_assert(std::is_enum_v<T>, "T is not an enum.");
  DCHECK_LE(static_cast<uintmax_t>(enum_size), static_cast<uintmax_t>(INT_MAX));
  DCHECK_LT(static_cast<uintmax_t>(sample), static_cast<uintmax_t>(enum_size));
  return UmaHistogramExactLinear(name, static_cast<int>(sample),
                                 static_cast<int>(enum_size));
}
// LINT.ThenChange(/base/metrics/histogram_functions.h:UmaHistogramEnumeration)

// LINT.IfChange(UmaHistogramBoolean)
BASE_EXPORT void UmaHistogramBoolean(const std::string& name, bool sample);
BASE_EXPORT void UmaHistogramBoolean(const char* name, bool sample);
// LINT.ThenChange(/base/metrics/histogram_functions.h:UmaHistogramBoolean)

// LINT.IfChange(UmaHistogramPercentage)
BASE_EXPORT void UmaHistogramPercentage(const std::string& name, int percent);
BASE_EXPORT void UmaHistogramPercentage(const char* name, int percent);

BASE_EXPORT void UmaHistogramPercentageObsoleteDoNotUse(const std::string& name,
                                                        int percent);
BASE_EXPORT void UmaHistogramPercentageObsoleteDoNotUse(const char* name,
                                                        int percent);
// LINT.ThenChange(/base/metrics/histogram_functions.h:UmaHistogramPercentage)

// LINT.IfChange(UmaHistogramCounts)
BASE_EXPORT void UmaHistogramCustomCounts(const std::string& name,
                                          int sample,
                                          int min,
                                          int exclusive_max,
                                          size_t buckets);
BASE_EXPORT void UmaHistogramCustomCounts(const char* name,
                                          int sample,
                                          int min,
                                          int exclusive_max,
                                          size_t buckets);

BASE_EXPORT void UmaHistogramCounts100(const std::string& name, int sample);
BASE_EXPORT void UmaHistogramCounts100(const char* name, int sample);
BASE_EXPORT void UmaHistogramCounts1000(const std::string& name, int sample);
BASE_EXPORT void UmaHistogramCounts1000(const char* name, int sample);
BASE_EXPORT void UmaHistogramCounts10000(const std::string& name, int sample);
BASE_EXPORT void UmaHistogramCounts10000(const char* name, int sample);
BASE_EXPORT void UmaHistogramCounts100000(const std::string& name, int sample);
BASE_EXPORT void UmaHistogramCounts100000(const char* name, int sample);
BASE_EXPORT void UmaHistogramCounts1M(const std::string& name, int sample);
BASE_EXPORT void UmaHistogramCounts1M(const char* name, int sample);
BASE_EXPORT void UmaHistogramCounts10M(const std::string& name, int sample);
BASE_EXPORT void UmaHistogramCounts10M(const char* name, int sample);
// LINT.ThenChange(/base/metrics/histogram_functions.h:UmaHistogramCounts)

// LINT.IfChange(UmaHistogramTimes)
BASE_EXPORT void UmaHistogramCustomTimes(const std::string& name,
                                         TimeDelta sample,
                                         TimeDelta min,
                                         TimeDelta max,
                                         size_t buckets);
BASE_EXPORT void UmaHistogramCustomTimes(const char* name,
                                         TimeDelta sample,
                                         TimeDelta min,
                                         TimeDelta max,
                                         size_t buckets);

BASE_EXPORT void UmaHistogramTimes(const std::string& name, TimeDelta sample);
BASE_EXPORT void UmaHistogramTimes(const char* name, TimeDelta sample);

BASE_EXPORT void UmaHistogramMediumTimes(const std::string& name,
                                         TimeDelta sample);
BASE_EXPORT void UmaHistogramMediumTimes(const char* name, TimeDelta sample);

BASE_EXPORT void UmaHistogramLongTimes(const std::string& name,
                                       TimeDelta sample);
BASE_EXPORT void UmaHistogramLongTimes(const char* name, TimeDelta sample);

BASE_EXPORT void UmaHistogramLongTimes100(const std::string& name,
                                          TimeDelta sample);
BASE_EXPORT void UmaHistogramLongTimes100(const char* name, TimeDelta sample);
// LINT.ThenChange(/base/metrics/histogram_functions.h:UmaHistogramTimes)

// LINT.IfChange(UmaHistogramMicrosecondsTimes)
BASE_EXPORT void UmaHistogramCustomMicrosecondsTimes(const std::string& name,
                                                     TimeDelta sample,
                                                     TimeDelta min,
                                                     TimeDelta max,
                                                     size_t buckets);
BASE_EXPORT void UmaHistogramCustomMicrosecondsTimes(const char* name,
                                                     TimeDelta sample,
                                                     TimeDelta min,
                                                     TimeDelta max,
                                                     size_t buckets);

BASE_EXPORT void UmaHistogramMicrosecondsTimes(const std::string& name,
                                               TimeDelta sample);
BASE_EXPORT void UmaHistogramMicrosecondsTimes(const char* name,
                                               TimeDelta sample);
// LINT.ThenChange(/base/metrics/histogram_functions.h:UmaHistogramMicrosecondsTimes)

// LINT.IfChange(UmaHistogramMemory)
BASE_EXPORT void UmaHistogramMemoryKB(const std::string& name, int sample);
BASE_EXPORT void UmaHistogramMemoryKB(const char* name, int sample);

BASE_EXPORT void UmaHistogramMemoryMB(const std::string& name, int sample);
BASE_EXPORT void UmaHistogramMemoryMB(const char* name, int sample);

BASE_EXPORT void UmaHistogramMemoryLargeMB(const std::string& name, int sample);
BASE_EXPORT void UmaHistogramMemoryLargeMB(const char* name, int sample);
// LINT.ThenChange(/base/metrics/histogram_functions.h:UmaHistogramMemory)

// LINT.IfChange(UmaHistogramSparse)
BASE_EXPORT void UmaHistogramSparse(const std::string& name, int sample);
BASE_EXPORT void UmaHistogramSparse(const char* name, int sample);
// LINT.ThenChange(/base/metrics/histogram_functions.h:UmaHistogramSparse)

}  // namespace base

#endif  // BASE_METRICS_HISTOGRAM_FUNCTIONS_INTERNAL_OVERLOADS_H_
