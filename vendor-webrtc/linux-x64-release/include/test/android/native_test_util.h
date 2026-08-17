/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_ANDROID_NATIVE_TEST_UTIL_H_
#define TEST_ANDROID_NATIVE_TEST_UTIL_H_

#include <android/log.h>

#include <string>
#include <vector>

#include "third_party/jni_zero/jni_zero.h"

// Helper methods for setting up environment for running gtest tests
// inside an APK.
namespace webrtc {
namespace test {
namespace android {

void AndroidLog(int priority, const char* format, ...);

std::string ASCIIJavaStringToUTF8(JNIEnv* env, jstring str);

void ParseArgsFromString(const std::string& command_line,
                         std::vector<std::string>* args);
void ParseArgsFromCommandLineFile(const char* path,
                                  std::vector<std::string>* args);
int ArgsToArgv(const std::vector<std::string>& args, std::vector<char*>* argv);

class ScopedMainEntryLogger {
 public:
  ScopedMainEntryLogger() {
    AndroidLog(ANDROID_LOG_INFO, ">>ScopedMainEntryLogger\n");
  }

  ~ScopedMainEntryLogger() {
    AndroidLog(ANDROID_LOG_INFO, "<<ScopedMainEntryLogger\n");
    fflush(stdout);
    fflush(stderr);
  }
};

}  // namespace android
}  // namespace test
}  // namespace webrtc

namespace jni_zero {
template <>
inline std::string FromJniType<std::string>(JNIEnv* env,
                                            const JavaRef<jobject>& input) {
  return webrtc::test::android::ASCIIJavaStringToUTF8(
      env, static_cast<jstring>(input.obj()));
}
}  // namespace jni_zero

#endif  // TEST_ANDROID_NATIVE_TEST_UTIL_H_
