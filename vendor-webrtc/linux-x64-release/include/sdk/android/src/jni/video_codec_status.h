/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_VIDEO_CODEC_STATUS_H_
#define SDK_ANDROID_SRC_JNI_VIDEO_CODEC_STATUS_H_

#include <jni.h>
#include <stdint.h>

#include "sdk/android/native_api/jni/scoped_java_ref.h"

namespace webrtc {
namespace jni {
int32_t JavaToNativeVideoCodecStatus(
    JNIEnv* env,
    const JavaRef<jobject>& j_video_codec_status);
}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_VIDEO_CODEC_STATUS_H_
