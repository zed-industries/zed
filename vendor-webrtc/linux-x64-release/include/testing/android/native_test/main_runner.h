// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_ANDROID_NATIVE_TEST_MAIN_RUNNER_H_
#define TESTING_ANDROID_NATIVE_TEST_MAIN_RUNNER_H_

#include <jni.h>

namespace testing {
namespace android {

bool RegisterMainRunnerJni(JNIEnv* env);

}  // namespace android
}  // namespace testing

#endif  // TESTING_ANDROID_NATIVE_TEST_MAIN_RUNNER_H_
