// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_CALLBACK_ANDROID_H_
#define BASE_ANDROID_CALLBACK_ANDROID_H_

#include <jni.h>

#include <string>
#include <vector>

#include "base/android/scoped_java_ref.h"
#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/time/time.h"
#include "base/types/optional_ref.h"

// Provides helper utility methods that run the given callback with the
// specified argument.
namespace base {
namespace android {

void BASE_EXPORT RunObjectCallbackAndroid(const JavaRef<jobject>& callback,
                                          const JavaRef<jobject>& arg);

void BASE_EXPORT RunBooleanCallbackAndroid(const JavaRef<jobject>& callback,
                                           bool arg);

void BASE_EXPORT RunIntCallbackAndroid(const JavaRef<jobject>& callback,
                                       int32_t arg);

void BASE_EXPORT RunLongCallbackAndroid(const JavaRef<jobject>& callback,
                                        int64_t arg);

void BASE_EXPORT RunTimeCallbackAndroid(const JavaRef<jobject>& callback,
                                        base::Time time);

void BASE_EXPORT RunStringCallbackAndroid(const JavaRef<jobject>& callback,
                                          const std::string& arg);

void BASE_EXPORT RunOptionalStringCallbackAndroid(
    const JavaRef<jobject>& callback,
    base::optional_ref<const std::string> optional_string_arg);

void BASE_EXPORT RunByteArrayCallbackAndroid(const JavaRef<jobject>& callback,
                                             const std::vector<uint8_t>& arg);

void BASE_EXPORT RunRunnableAndroid(const JavaRef<jobject>& runnable);

}  // namespace android
}  // namespace base

namespace jni_zero {

template <>
inline base::RepeatingClosure FromJniType<base::RepeatingClosure>(
    JNIEnv* env,
    const JavaRef<jobject>& obj) {
  return base::BindRepeating(&base::android::RunRunnableAndroid,
                             base::android::ScopedJavaGlobalRef<jobject>(obj));
}

}  // namespace jni_zero

#endif  // BASE_ANDROID_CALLBACK_ANDROID_H_
