/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef SDK_ANDROID_SRC_JNI_SCOPED_JAVA_REF_COUNTED_H_
#define SDK_ANDROID_SRC_JNI_SCOPED_JAVA_REF_COUNTED_H_

#include "sdk/android/native_api/jni/scoped_java_ref.h"

namespace webrtc {
namespace jni {

// Holds a reference to a java object implementing the RefCounted interface, and
// calls its release() method from the destructor.
class ScopedJavaRefCounted {
 public:
  // Takes over the caller's reference.
  static ScopedJavaRefCounted Adopt(JNIEnv* jni,
                                    const JavaRef<jobject>& j_object) {
    return ScopedJavaRefCounted(jni, j_object);
  }

  // Retains the java object for the live time of this object.
  static ScopedJavaRefCounted Retain(JNIEnv* jni,
                                     const JavaRef<jobject>& j_object);
  ScopedJavaRefCounted(ScopedJavaRefCounted&& other) = default;

  ScopedJavaRefCounted(const ScopedJavaRefCounted& other) = delete;
  ScopedJavaRefCounted& operator=(const ScopedJavaRefCounted&) = delete;

  ~ScopedJavaRefCounted();

 private:
  // Adopts reference.
  ScopedJavaRefCounted(JNIEnv* jni, const JavaRef<jobject>& j_object)
      : j_object_(jni, j_object) {}

  ScopedJavaGlobalRef<jobject> j_object_;
};

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_SCOPED_JAVA_REF_COUNTED_H_
