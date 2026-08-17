// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_COVERAGE_UTIL_IOS_H_
#define TESTING_COVERAGE_UTIL_IOS_H_

namespace coverage_util {

// In debug builds, if IOS_ENABLE_COVERAGE is defined, sets the filename of the
// coverage file. Otherwise, it does nothing.
void ConfigureCoverageReportPath();

}  // namespace coverage_util

#endif  // TESTING_COVERAGE_UTIL_IOS_H_
