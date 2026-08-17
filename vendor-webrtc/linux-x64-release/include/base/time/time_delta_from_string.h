// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TIME_TIME_DELTA_FROM_STRING_H_
#define BASE_TIME_TIME_DELTA_FROM_STRING_H_

#include <optional>
#include <string_view>

#include "base/base_export.h"

namespace base {

class TimeDelta;

// Helper function for TimeDelta.
// This is not part of TimeDelta to avoid dragging the includes above into
// base/time/time.h.
//
// Adapted from Go's doc at https://golang.org/pkg/time/#ParseDuration
//   [ParseDuration] parses a duration string. A duration string is
//   a possibly signed sequence of decimal numbers, each with optional
//   fraction and a unit suffix, such as "300ms", "-1.5h" or "2h45m".
//   Valid time units are "ns", "us" "ms", "s", "m", "h", "d".
//
// Special values that are allowed without specifying units:
//  "0", "+0", "-0" -> TimeDelta()
//  "inf", "+inf"   -> TimeDelta::Max()
//  "-inf"          -> TimeDelta::Min()
// Returns `std::nullopt` when parsing fails. Numbers larger than 2^63-1
// will fail parsing. Overflowing `number * unit` will return +/-inf, as
// appropriate.
BASE_EXPORT std::optional<TimeDelta> TimeDeltaFromString(
    std::string_view duration_string);

}  // namespace base

#endif  // BASE_TIME_TIME_DELTA_FROM_STRING_H_
