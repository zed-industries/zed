// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_SUPPORT_IOS_H_
#define BASE_TEST_TEST_SUPPORT_IOS_H_

#include "base/test/test_suite.h"

namespace base {

using RunTestSuiteCallback = OnceCallback<int(void)>;

// Inits the message loop for tests on iOS.
void InitIOSTestMessageLoop();

// Inits the run test suite hook.
void InitIOSRunHook(RunTestSuiteCallback callback);

// Inits the initial args for tests on iOS.
void InitIOSArgs(int argc, char* argv[]);

// Launches an iOS app that runs the tests in the suite passed to
// InitIOSRunHook.
int RunTestsFromIOSApp();

// Returns true if unittests should be run by the XCTest runner.
bool ShouldRunIOSUnittestsWithXCTest();

}  // namespace base

#endif  // BASE_TEST_TEST_SUPPORT_IOS_H_
