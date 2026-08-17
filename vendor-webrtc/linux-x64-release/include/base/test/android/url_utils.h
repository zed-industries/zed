// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_ANDROID_URL_UTILS_H_
#define BASE_TEST_ANDROID_URL_UTILS_H_

#include <jni.h>

#include "base/base_export.h"
#include "base/files/file_path.h"

namespace base {
namespace android {

// Returns the root of the test data directory. This function will call into
// Java class UrlUtils through JNI bridge.
BASE_EXPORT FilePath GetIsolatedTestRoot();

}  // namespace android
}  // namespace base

#endif  // BASE_TEST_ANDROID_URL_UTILS_H_
