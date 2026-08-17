/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_PC_RTP_TRANSCEIVER_H_
#define SDK_ANDROID_SRC_JNI_PC_RTP_TRANSCEIVER_H_

#include <jni.h>

#include "api/rtp_transceiver_interface.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"

namespace webrtc {
namespace jni {

RtpTransceiverInit JavaToNativeRtpTransceiverInit(
    JNIEnv* jni,
    const JavaRef<jobject>& j_init);

ScopedJavaLocalRef<jobject> NativeToJavaRtpTransceiver(
    JNIEnv* env,
    scoped_refptr<RtpTransceiverInterface> transceiver);

// This takes ownership of the of the `j_transceiver` and stores it as a global
// reference. This calls the Java Transceiver's dispose() method with the dtor.
class JavaRtpTransceiverGlobalOwner {
 public:
  JavaRtpTransceiverGlobalOwner(JNIEnv* env,
                                const JavaRef<jobject>& j_transceiver);
  JavaRtpTransceiverGlobalOwner(JavaRtpTransceiverGlobalOwner&& other);
  ~JavaRtpTransceiverGlobalOwner();

 private:
  ScopedJavaGlobalRef<jobject> j_transceiver_;
};

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_RTP_TRANSCEIVER_H_
