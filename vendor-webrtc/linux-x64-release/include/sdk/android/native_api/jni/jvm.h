/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_NATIVE_API_JNI_JVM_H_
#define SDK_ANDROID_NATIVE_API_JNI_JVM_H_

#include <jni.h>

namespace webrtc {
// Returns a JNI environment usable on this thread.
JNIEnv* AttachCurrentThreadIfNeeded();
}  // namespace webrtc

#endif  // SDK_ANDROID_NATIVE_API_JNI_JVM_H_
