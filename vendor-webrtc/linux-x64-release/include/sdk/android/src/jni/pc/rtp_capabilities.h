/*
 *  Copyright 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_PC_RTP_CAPABILITIES_H_
#define SDK_ANDROID_SRC_JNI_PC_RTP_CAPABILITIES_H_

#include <jni.h>

#include "api/rtp_parameters.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"

namespace webrtc {
namespace jni {

RtpCapabilities JavaToNativeRtpCapabilities(
    JNIEnv* jni,
    const JavaRef<jobject>& j_capabilities);

ScopedJavaLocalRef<jobject> NativeToJavaRtpCapabilities(
    JNIEnv* jni,
    const RtpCapabilities& capabilities);

RtpCodecCapability JavaToNativeRtpCodecCapability(
    JNIEnv* jni,
    const JavaRef<jobject>& j_codec_capability);

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_RTP_CAPABILITIES_H_
