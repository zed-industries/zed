/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_NATIVE_CAPTURER_OBSERVER_H_
#define SDK_ANDROID_SRC_JNI_NATIVE_CAPTURER_OBSERVER_H_

#include <jni.h>

#include "sdk/android/native_api/jni/scoped_java_ref.h"
#include "sdk/android/src/jni/android_video_track_source.h"

namespace webrtc {
namespace jni {

ScopedJavaLocalRef<jobject> CreateJavaNativeCapturerObserver(
    JNIEnv* env,
    scoped_refptr<AndroidVideoTrackSource> native_source);

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_NATIVE_CAPTURER_OBSERVER_H_
