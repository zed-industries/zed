/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef SDK_ANDROID_NATIVE_API_JNI_APPLICATION_CONTEXT_PROVIDER_H_
#define SDK_ANDROID_NATIVE_API_JNI_APPLICATION_CONTEXT_PROVIDER_H_

#include "sdk/android/native_api/jni/scoped_java_ref.h"

namespace webrtc {

ScopedJavaLocalRef<jobject> GetAppContext(JNIEnv* jni);

}  // namespace webrtc

#endif  // SDK_ANDROID_NATIVE_API_JNI_APPLICATION_CONTEXT_PROVIDER_H_
