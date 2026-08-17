/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_PC_RTC_CERTIFICATE_H_
#define SDK_ANDROID_SRC_JNI_PC_RTC_CERTIFICATE_H_

#include "rtc_base/ref_count.h"
#include "rtc_base/rtc_certificate.h"
#include "sdk/android/native_api/jni/java_types.h"
#include "sdk/android/src/jni/jni_helpers.h"

namespace webrtc {
namespace jni {

RTCCertificatePEM JavaToNativeRTCCertificatePEM(
    JNIEnv* jni,
    const JavaRef<jobject>& j_rtc_certificate);

ScopedJavaLocalRef<jobject> NativeToJavaRTCCertificatePEM(
    JNIEnv* env,
    const RTCCertificatePEM& certificate);

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_RTC_CERTIFICATE_H_
