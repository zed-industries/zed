/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_PC_RTC_STATS_COLLECTOR_CALLBACK_WRAPPER_H_
#define SDK_ANDROID_SRC_JNI_PC_RTC_STATS_COLLECTOR_CALLBACK_WRAPPER_H_

#include <jni.h>

#include "api/peer_connection_interface.h"
#include "sdk/android/src/jni/jni_helpers.h"

namespace webrtc {
namespace jni {

// Adapter for a Java RTCStatsCollectorCallback presenting a C++
// RTCStatsCollectorCallback and dispatching the callback from C++ back to
// Java.
class RTCStatsCollectorCallbackWrapper : public RTCStatsCollectorCallback {
 public:
  RTCStatsCollectorCallbackWrapper(JNIEnv* jni,
                                   const JavaRef<jobject>& j_callback);
  ~RTCStatsCollectorCallbackWrapper() override;

  void OnStatsDelivered(
      const scoped_refptr<const RTCStatsReport>& report) override;

 private:
  const ScopedJavaGlobalRef<jobject> j_callback_global_;
};

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_RTC_STATS_COLLECTOR_CALLBACK_WRAPPER_H_
