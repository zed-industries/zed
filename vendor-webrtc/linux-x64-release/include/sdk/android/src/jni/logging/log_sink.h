/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef SDK_ANDROID_SRC_JNI_LOGGING_LOG_SINK_H_
#define SDK_ANDROID_SRC_JNI_LOGGING_LOG_SINK_H_

#include <string>

#include "absl/strings/string_view.h"
#include "rtc_base/logging.h"
#include "sdk/android/native_api/jni/java_types.h"
#include "sdk/android/src/jni/jni_helpers.h"

namespace webrtc {
namespace jni {

class JNILogSink : public LogSink {
 public:
  JNILogSink(JNIEnv* env, const JavaRef<jobject>& j_logging);
  ~JNILogSink() override;

  void OnLogMessage(const std::string& msg) override;
  void OnLogMessage(const std::string& msg,
                    LoggingSeverity severity,
                    const char* tag) override;
  void OnLogMessage(absl::string_view msg,
                    LoggingSeverity severity,
                    const char* tag) override;

 private:
  const ScopedJavaGlobalRef<jobject> j_logging_;
};

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_LOGGING_LOG_SINK_H_
