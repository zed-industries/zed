/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_GTEST_PROD_UTIL_H_
#define RTC_BASE_GTEST_PROD_UTIL_H_

// Define our own version of FRIEND_TEST here rather than including
// gtest_prod.h to avoid depending on any part of GTest in production code.
#define FRIEND_TEST_WEBRTC(test_case_name, test_name) \
  friend class test_case_name##_##test_name##_Test

// This file is a plain copy of Chromium's base/gtest_prod_util.h.
//
// This is a wrapper for gtest's FRIEND_TEST macro that friends
// test with all possible prefixes. This is very helpful when changing the test
// prefix, because the friend declarations don't need to be updated.
//
// Example usage:
//
// class MyClass {
//  private:
//   void MyMethod();
//   FRIEND_TEST_ALL_PREFIXES(MyClassTest, MyMethod);
// };
#define FRIEND_TEST_ALL_PREFIXES(test_case_name, test_name) \
  FRIEND_TEST_WEBRTC(test_case_name, test_name);            \
  FRIEND_TEST_WEBRTC(test_case_name, DISABLED_##test_name); \
  FRIEND_TEST_WEBRTC(test_case_name, FLAKY_##test_name);    \
  FRIEND_TEST_WEBRTC(test_case_name, FAILS_##test_name)

#endif  // RTC_BASE_GTEST_PROD_UTIL_H_
