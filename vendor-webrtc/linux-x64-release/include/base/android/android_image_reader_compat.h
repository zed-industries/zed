// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_ANDROID_IMAGE_READER_COMPAT_H_
#define BASE_ANDROID_ANDROID_IMAGE_READER_COMPAT_H_

#include "base/base_export.h"

namespace base {
namespace android {

// Check if the image reader usage is supported.
BASE_EXPORT bool EnableAndroidImageReader();

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_ANDROID_IMAGE_READER_COMPAT_H_
