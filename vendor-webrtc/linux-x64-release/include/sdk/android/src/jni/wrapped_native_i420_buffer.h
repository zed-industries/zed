/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_WRAPPED_NATIVE_I420_BUFFER_H_
#define SDK_ANDROID_SRC_JNI_WRAPPED_NATIVE_I420_BUFFER_H_

#include <jni.h>

#include "api/video/video_frame_buffer.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"

namespace webrtc {
namespace jni {

// This function wraps the C++ I420 buffer and returns a Java
// VideoFrame.I420Buffer as a jobject.
ScopedJavaLocalRef<jobject> WrapI420Buffer(
    JNIEnv* jni,
    const scoped_refptr<I420BufferInterface>& i420_buffer);

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_WRAPPED_NATIVE_I420_BUFFER_H_
