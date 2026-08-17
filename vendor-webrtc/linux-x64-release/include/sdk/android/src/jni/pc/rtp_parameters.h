/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_PC_RTP_PARAMETERS_H_
#define SDK_ANDROID_SRC_JNI_PC_RTP_PARAMETERS_H_

#include <jni.h>

#include "api/rtp_parameters.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"

namespace webrtc {
namespace jni {

RtpEncodingParameters JavaToNativeRtpEncodingParameters(
    JNIEnv* env,
    const JavaRef<jobject>& j_encoding_parameters);

RtpParameters JavaToNativeRtpParameters(JNIEnv* jni,
                                        const JavaRef<jobject>& j_parameters);
ScopedJavaLocalRef<jobject> NativeToJavaRtpParameters(
    JNIEnv* jni,
    const RtpParameters& parameters);

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_RTP_PARAMETERS_H_
