/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Originally these classes are from Chromium.
// https://cs.chromium.org/chromium/src/base/android/scoped_java_ref.h.

#ifndef SDK_ANDROID_NATIVE_API_JNI_SCOPED_JAVA_REF_H_
#define SDK_ANDROID_NATIVE_API_JNI_SCOPED_JAVA_REF_H_

#include <jni.h>

#include <utility>

#include "sdk/android/native_api/jni/jvm.h"
#include "third_party/jni_zero/jni_zero.h"

namespace webrtc {
using jni_zero::JavaParamRef;
using jni_zero::JavaRef;
using jni_zero::ScopedJavaGlobalRef;
using jni_zero::ScopedJavaLocalRef;

template <typename T>
inline jni_zero::ScopedJavaLocalRef<T> static_java_ref_cast(
    JNIEnv* env,
    jni_zero::JavaRef<jobject> const& ref) {
  jni_zero::ScopedJavaLocalRef<jobject> owned_ref(env, ref);
  return jni_zero::ScopedJavaLocalRef<T>(env,
                                         static_cast<T>(owned_ref.Release()));
}

}  // namespace webrtc

#endif  // SDK_ANDROID_NATIVE_API_JNI_SCOPED_JAVA_REF_H_
