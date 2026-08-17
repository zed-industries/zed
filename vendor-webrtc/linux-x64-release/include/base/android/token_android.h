// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_TOKEN_ANDROID_H_
#define BASE_ANDROID_TOKEN_ANDROID_H_

#include <jni.h>

#include <optional>

#include "base/android/scoped_java_ref.h"
#include "base/base_export.h"
#include "base/containers/span.h"
#include "base/token.h"

namespace base::android {

class BASE_EXPORT TokenAndroid {
 public:
  // Create a Java Token with the same value as `token`.
  static ScopedJavaLocalRef<jobject> Create(JNIEnv* env,
                                            const base::Token& token);

  // Creates a Token from `j_token`.
  static base::Token FromJavaToken(JNIEnv* env,
                                   const JavaRef<jobject>& j_token);

  TokenAndroid() = delete;
  TokenAndroid(const TokenAndroid&) = delete;
  TokenAndroid& operator=(const TokenAndroid&) = delete;
};

}  // namespace base::android

namespace jni_zero {
template <>
inline base::Token FromJniType<base::Token>(JNIEnv* env,
                                            const JavaRef<jobject>& j_object) {
  return base::android::TokenAndroid::FromJavaToken(env, j_object);
}
template <>
inline ScopedJavaLocalRef<jobject> ToJniType<base::Token>(
    JNIEnv* env,
    const base::Token& token) {
  return base::android::TokenAndroid::Create(env, token);
}
}  // namespace jni_zero

#endif  // BASE_ANDROID_TOKEN_ANDROID_H_
