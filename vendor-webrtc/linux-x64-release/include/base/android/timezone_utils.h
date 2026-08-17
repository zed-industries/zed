// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_TIMEZONE_UTILS_H_
#define BASE_ANDROID_TIMEZONE_UTILS_H_

#include <jni.h>

#include <string>

#include "base/base_export.h"

namespace base {
namespace android {

// Return an ICU timezone created from the host timezone.
BASE_EXPORT std::u16string GetDefaultTimeZoneId();

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_TIMEZONE_UTILS_H_
