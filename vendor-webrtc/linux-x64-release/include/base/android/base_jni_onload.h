// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_BASE_JNI_ONLOAD_H_
#define BASE_ANDROID_BASE_JNI_ONLOAD_H_

#include <jni.h>

#include "base/base_export.h"
#include "base/functional/callback.h"

namespace base {
namespace android {

// Returns whether initialization succeeded.
BASE_EXPORT bool OnJNIOnLoadInit();

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_BASE_JNI_ONLOAD_H_
