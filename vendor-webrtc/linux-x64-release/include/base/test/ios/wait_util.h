// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_IOS_WAIT_UTIL_H_
#define BASE_TEST_IOS_WAIT_UTIL_H_

#import <Foundation/Foundation.h>

#include "base/ios/block_types.h"
#include "base/time/time.h"

namespace base::test::ios {

// Constant for UI wait loop.
constexpr TimeDelta kSpinDelaySeconds = Milliseconds(10);

// Constant for timeout while waiting for UI element.
constexpr TimeDelta kWaitForUIElementTimeout = Seconds(4);

// Constant for timeout while waiting for JavaScript completion.
constexpr TimeDelta kWaitForJSCompletionTimeout = Seconds(6);

// Constant for timeout while waiting for a download to complete.
constexpr TimeDelta kWaitForDownloadTimeout = Seconds(10);

// Constant for timeout while waiting for a pageload to complete.
constexpr TimeDelta kWaitForPageLoadTimeout = Seconds(10);

// Constant for timeout while waiting for a generic action to complete.
constexpr TimeDelta kWaitForActionTimeout = Seconds(10);

// Constant for timeout while waiting for clear browsing data. It seems this
// can take a very long time on the bots when running simulators in parallel.
// TODO(crbug.com/41475878): Investigate why this is sometimes very slow.
constexpr TimeDelta kWaitForClearBrowsingDataTimeout = Seconds(45);

// Constant for timeout while waiting for cookies operations to complete.
constexpr TimeDelta kWaitForCookiesTimeout = Seconds(4);

// Constant for timeout while waiting for a file operation to complete.
constexpr TimeDelta kWaitForFileOperationTimeout = Seconds(2);

// Returns true when condition() becomes true, otherwise returns false after
// |timeout|. Repetitively runs the current NSRunLoop and the current
// MessageLoop (if |run_message_loop| is true). Passing |run_message_loop| true
// only makes sense in unit tests.
[[nodiscard]] bool WaitUntilConditionOrTimeout(TimeDelta timeout,
                                               bool run_message_loop,
                                               ConditionBlock condition);

// Same as above but `run_message_loop` is false.
[[nodiscard]] bool WaitUntilConditionOrTimeout(TimeDelta timeout,
                                               ConditionBlock condition);

// Lets the run loop of the current thread process other messages
// within the given maximum delay. This method may return before max_delay
// elapsed.
void SpinRunLoopWithMaxDelay(TimeDelta max_delay);

// Lets the run loop of the current thread process other messages
// within the given minimum delay. This method returns after |min_delay|
// elapsed.
void SpinRunLoopWithMinDelay(TimeDelta min_delay);

}  // namespace base::test::ios

#endif  // BASE_TEST_IOS_WAIT_UTIL_H_
