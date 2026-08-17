// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_BUILD_TIME_H_
#define BASE_BUILD_TIME_H_

#include "base/generated_build_date.h"
#include "base/time/time.h"

namespace base {

// GetBuildTime returns the time at which the current binary was built,
// rounded down to 5:00:00am at the start of the day in UTC.
//
// This uses a generated file, which doesn't trigger a rebuild when the time
// changes. It will, however, be updated whenever //build/util/LASTCHANGE
// changes.
//
// This value should only be considered accurate to within a day.
// It will always be in the past.
//
// Note: If the build is not official (i.e. is_official_build = false)
// this time will be set to 5:00:00am on the most recent first Sunday
// of a month.
constexpr Time GetBuildTime() {
  // BASE_GENERATED_BUILD_DATE_TIMESTAMP is a Unix timestamp value. See
  // //base/write_build_date_header.py and //build/compute_build_timestamp.py
  // for details. On non-official builds, this will be first Sunday of the month
  // at 5:00am UTC.
  return Time::FromTimeT(BASE_GENERATED_BUILD_DATE_TIMESTAMP);
}

}  // namespace base

#endif  // BASE_BUILD_TIME_H_
