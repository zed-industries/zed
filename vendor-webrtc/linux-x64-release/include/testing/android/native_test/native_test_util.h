// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_ANDROID_NATIVE_TEST_NATIVE_TEST_UTIL_H_
#define TESTING_ANDROID_NATIVE_TEST_NATIVE_TEST_UTIL_H_

#include <stdio.h>

#include <string>
#include <vector>

#include "base/compiler_specific.h"

// Helper methods for setting up environment for running gtest tests
// inside an APK.
namespace testing {
namespace android {

class ScopedMainEntryLogger {
 public:
  ScopedMainEntryLogger() {
    printf(">>ScopedMainEntryLogger\n");
  }

  ~ScopedMainEntryLogger() {
    printf("<<ScopedMainEntryLogger\n");
    // SAFETY: On Android with __ANDROID_API__ <= 22 (Lollipop MR1), `stdout` is
    // a macro that accesses a buffer of unknown length, triggering the
    // `-Wunsafe-buffer-usage` warning. This usage is sound, as it accesses a
    // fixed position in a system-provided array.
    // While Lollipop is no longer officially supported by Chromium, Cronet
    // currently maintains compatibility with this older Android version. Check
    // for optional bot `android-cronet-arm64-dbg` before removing this.
    fflush(UNSAFE_BUFFERS(stdout));
    fflush(UNSAFE_BUFFERS(stderr));
  }
};

void ParseArgsFromString(
    const std::string& command_line, std::vector<std::string>* args);
void ParseArgsFromCommandLineFile(
    const char* path, std::vector<std::string>* args);
int ArgsToArgv(const std::vector<std::string>& args, std::vector<char*>* argv);

}  // namespace android
}  // namespace testing

#endif  // TESTING_ANDROID_NATIVE_TEST_NATIVE_TEST_UTIL_H_
