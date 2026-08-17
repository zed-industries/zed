// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_JNI_STRING_H_
#define BASE_ANDROID_JNI_STRING_H_

#include <jni.h>

#include <string>
#include <string_view>

#include "base/android/scoped_java_ref.h"
#include "base/base_export.h"

namespace base {
namespace android {

// Convert a Java string to UTF8. Returns a std string.
BASE_EXPORT void ConvertJavaStringToUTF8(JNIEnv* env,
                                         jstring str,
                                         std::string* result);
BASE_EXPORT std::string ConvertJavaStringToUTF8(JNIEnv* env, jstring str);
BASE_EXPORT std::string ConvertJavaStringToUTF8(const JavaRef<jstring>& str);
BASE_EXPORT std::string ConvertJavaStringToUTF8(JNIEnv* env,
                                                const JavaRef<jstring>& str);

// Convert a std string to Java string.
BASE_EXPORT ScopedJavaLocalRef<jstring> ConvertUTF8ToJavaString(
    JNIEnv* env,
    std::string_view str);

// Convert a Java string to UTF16. Returns a std::u16string.
BASE_EXPORT void ConvertJavaStringToUTF16(JNIEnv* env,
                                          jstring str,
                                          std::u16string* result);
BASE_EXPORT std::u16string ConvertJavaStringToUTF16(JNIEnv* env, jstring str);
BASE_EXPORT std::u16string ConvertJavaStringToUTF16(
    const JavaRef<jstring>& str);
BASE_EXPORT std::u16string ConvertJavaStringToUTF16(
    JNIEnv* env,
    const JavaRef<jstring>& str);

// Convert a std::u16string to a Java string.
BASE_EXPORT ScopedJavaLocalRef<jstring> ConvertUTF16ToJavaString(
    JNIEnv* env,
    std::u16string_view str);

}  // namespace android
}  // namespace base

namespace jni_zero {
template <>
inline std::string FromJniType<std::string>(JNIEnv* env,
                                            const JavaRef<jobject>& input) {
  return base::android::ConvertJavaStringToUTF8(
      env, static_cast<jstring>(input.obj()));
}

template <>
inline ScopedJavaLocalRef<jobject> ToJniType<std::string>(
    JNIEnv* env,
    const std::string& input) {
  return base::android::ConvertUTF8ToJavaString(env, input);
}

template <>
inline ScopedJavaLocalRef<jobject> ToJniType<const char>(JNIEnv* env,
                                                         const char* input) {
  return base::android::ConvertUTF8ToJavaString(env, input);
}

template <>
inline std::u16string FromJniType<std::u16string>(
    JNIEnv* env,
    const JavaRef<jobject>& input) {
  return base::android::ConvertJavaStringToUTF16(
      env, static_cast<jstring>(input.obj()));
}

template <>
inline ScopedJavaLocalRef<jobject> ToJniType<std::u16string>(
    JNIEnv* env,
    const std::u16string& input) {
  return base::android::ConvertUTF16ToJavaString(env, input);
}

template <>
inline ScopedJavaLocalRef<jobject> ToJniType<std::u16string_view>(
    JNIEnv* env,
    const std::u16string_view& input) {
  return base::android::ConvertUTF16ToJavaString(env, input);
}
}  // namespace jni_zero

#endif  // BASE_ANDROID_JNI_STRING_H_
