/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_IOS_COVERAGE_UTIL_IOS_H_
#define TEST_IOS_COVERAGE_UTIL_IOS_H_

namespace webrtc {
namespace test {

// In debug builds, if IOS_ENABLE_COVERAGE is defined, sets the filename of the
// coverage file. Otherwise, it does nothing.
void ConfigureCoverageReportPath();

}  // namespace test
}  // namespace webrtc

#endif  // TEST_IOS_COVERAGE_UTIL_IOS_H_
