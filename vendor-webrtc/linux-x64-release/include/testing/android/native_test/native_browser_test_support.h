// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_ANDROID_NATIVE_TEST_NATIVE_BROWSER_TEST_SUPPORT_H_
#define TESTING_ANDROID_NATIVE_TEST_NATIVE_BROWSER_TEST_SUPPORT_H_

#include <jni.h>

namespace testing {
namespace android {

// Android browser tests must wait for Java async initialization tasks to run
// before running the test. This function returns true in the browser process
// once they are done.
bool JavaAsyncStartupTasksCompleteForBrowserTests();

}  // namespace android
}  // namespace testing

#endif  // TESTING_ANDROID_NATIVE_TEST_NATIVE_BROWSER_TEST_SUPPORT_H_
