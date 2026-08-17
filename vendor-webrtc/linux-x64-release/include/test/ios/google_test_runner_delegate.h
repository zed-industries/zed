/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_IOS_GOOGLE_TEST_RUNNER_DELEGATE_H_
#define TEST_IOS_GOOGLE_TEST_RUNNER_DELEGATE_H_

// Copied from Chromium base/test/ios/google_test_runner_delegate.h
// to avoid the //base dependency. This protocol is required
// to run iOS Unittest.
@protocol GoogleTestRunnerDelegate

// Returns YES if this delegate supports running GoogleTests via a call to
// `runGoogleTests`.
@property(nonatomic, readonly, assign) BOOL supportsRunningGoogleTests;

// Runs GoogleTests and returns the final exit code.
- (int)runGoogleTests;

@end

#endif  // TEST_IOS_GOOGLE_TEST_RUNNER_DELEGATE_H_
