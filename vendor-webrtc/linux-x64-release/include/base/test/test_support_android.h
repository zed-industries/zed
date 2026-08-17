// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_SUPPORT_ANDROID_H_
#define BASE_TEST_TEST_SUPPORT_ANDROID_H_

#include "base/base_export.h"

namespace base {

class FilePath;

// Init path providers for tests on Android.
BASE_EXPORT void InitAndroidTestPaths(const FilePath& test_data_dir);

// Init the message loop for tests on Android.
BASE_EXPORT void InitAndroidTestMessageLoop();

// Counts how many times MessagePumpAndroid::DoNonDelayedLooperWork() has been
// entered.
BASE_EXPORT uint32_t GetAndroidNonDelayedWorkEnterCount();

}  // namespace base

#endif  // BASE_TEST_TEST_SUPPORT_ANDROID_H_
