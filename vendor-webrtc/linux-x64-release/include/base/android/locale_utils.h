// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_LOCALE_UTILS_H_
#define BASE_ANDROID_LOCALE_UTILS_H_

#include <jni.h>

#include <string>

#include "base/base_export.h"

namespace base {
namespace android {

BASE_EXPORT std::string GetDefaultCountryCode();

// Return the current default locale of the device as string.
BASE_EXPORT std::string GetDefaultLocaleString();

// Returns a list of user-selected locales as a comma separated string, ordered
// by decreasing preference.
BASE_EXPORT std::string GetDefaultLocaleListString();

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_LOCALE_UTILS_H_
