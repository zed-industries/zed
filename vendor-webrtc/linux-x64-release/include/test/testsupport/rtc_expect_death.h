/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_TESTSUPPORT_RTC_EXPECT_DEATH_H_
#define TEST_TESTSUPPORT_RTC_EXPECT_DEATH_H_

#include "test/gtest.h"

#if RTC_CHECK_MSG_ENABLED
#define RTC_EXPECT_DEATH(statement, regex) EXPECT_DEATH(statement, regex)
#else
// If RTC_CHECKs messages are disabled we can't validate failure message
#define RTC_EXPECT_DEATH(statement, regex) EXPECT_DEATH(statement, "")
#endif

#endif  // TEST_TESTSUPPORT_RTC_EXPECT_DEATH_H_
