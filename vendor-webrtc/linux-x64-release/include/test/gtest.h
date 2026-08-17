/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_GTEST_H_
#define TEST_GTEST_H_

#include "rtc_base/ignore_wundef.h"

RTC_PUSH_IGNORING_WUNDEF()
// IWYU pragma: begin_exports
#include "testing/gtest/include/gtest/gtest-spi.h"
#include "testing/gtest/include/gtest/gtest.h"
// IWYU pragma: end_exports
RTC_POP_IGNORING_WUNDEF()

// GTEST_HAS_DEATH_TEST is set to 1 when death tests are supported, but appears
// to be left unset if they're not supported. Rather than depend on this, we
// set it to 0 ourselves here.
#ifndef GTEST_HAS_DEATH_TEST
#define GTEST_HAS_DEATH_TEST 0
#endif

#endif  // TEST_GTEST_H_
