// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_INT_STRING_CALLBACK_H_
#define BASE_ANDROID_INT_STRING_CALLBACK_H_

#include <string>

#include "base/android/scoped_java_ref.h"
#include "base/base_export.h"

namespace base {
namespace android {

// Runs the Java |callback| by calling its onResult method and passing the
// integer and string as its arguments.
void BASE_EXPORT RunIntStringCallbackAndroid(const JavaRef<jobject>& callback,
                                             int int_arg,
                                             const std::string& str_arg);

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_INT_STRING_CALLBACK_H_
