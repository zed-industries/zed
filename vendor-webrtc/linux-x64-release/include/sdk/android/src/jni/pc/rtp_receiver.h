/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_PC_RTP_RECEIVER_H_
#define SDK_ANDROID_SRC_JNI_PC_RTP_RECEIVER_H_

#include <jni.h>

#include "api/rtp_receiver_interface.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"

namespace webrtc {
namespace jni {

ScopedJavaLocalRef<jobject> NativeToJavaRtpReceiver(
    JNIEnv* env,
    scoped_refptr<RtpReceiverInterface> receiver);

// Takes ownership of the passed `j_receiver` and stores it as a global
// reference. Will call dispose() in the dtor.
class JavaRtpReceiverGlobalOwner {
 public:
  JavaRtpReceiverGlobalOwner(JNIEnv* env, const JavaRef<jobject>& j_receiver);
  JavaRtpReceiverGlobalOwner(JavaRtpReceiverGlobalOwner&& other);
  ~JavaRtpReceiverGlobalOwner();

 private:
  ScopedJavaGlobalRef<jobject> j_receiver_;
};

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_RTP_RECEIVER_H_
