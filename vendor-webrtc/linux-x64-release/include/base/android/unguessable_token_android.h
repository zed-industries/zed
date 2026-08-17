// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_UNGUESSABLE_TOKEN_ANDROID_H_
#define BASE_ANDROID_UNGUESSABLE_TOKEN_ANDROID_H_

#include "base/base_export.h"
#include "base/unguessable_token.h"
#include "third_party/jni_zero/jni_zero.h"

namespace base {
namespace android {

class BASE_EXPORT UnguessableTokenAndroid {
 public:
  // Create a Java UnguessableToken with the same value as |token|.
  static jni_zero::ScopedJavaLocalRef<jobject> Create(
      JNIEnv* env,
      const base::UnguessableToken& token);

  // Create a native UnguessableToken from Java UnguessableToken |token|.
  static base::UnguessableToken FromJavaUnguessableToken(
      JNIEnv* env,
      const jni_zero::JavaRef<jobject>& token);

  // Parcel UnguessableToken |token| and unparcel it, and return the result.
  // While this method is intended for facilitating unit tests, it results only
  // in a clone of |token|.
  static jni_zero::ScopedJavaLocalRef<jobject> ParcelAndUnparcelForTesting(
      JNIEnv* env,
      const jni_zero::JavaRef<jobject>& token);

  UnguessableTokenAndroid() = delete;
  UnguessableTokenAndroid(const UnguessableTokenAndroid&) = delete;
  UnguessableTokenAndroid& operator=(const UnguessableTokenAndroid&) = delete;
};

}  // namespace android
}  // namespace base

namespace jni_zero {
template <>
inline base::UnguessableToken FromJniType<base::UnguessableToken>(
    JNIEnv* env,
    const JavaRef<jobject>& j_object) {
  return base::android::UnguessableTokenAndroid::FromJavaUnguessableToken(
      env, j_object);
}

template <>
inline ScopedJavaLocalRef<jobject> ToJniType<base::UnguessableToken>(
    JNIEnv* env,
    const base::UnguessableToken& token) {
  return base::android::UnguessableTokenAndroid::Create(env, token);
}
}  // namespace jni_zero

#endif  // BASE_ANDROID_UNGUESSABLE_TOKEN_ANDROID_H_
