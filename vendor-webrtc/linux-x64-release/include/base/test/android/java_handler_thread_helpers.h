// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_ANDROID_JAVA_HANDLER_THREAD_HELPERS_H_
#define BASE_TEST_ANDROID_JAVA_HANDLER_THREAD_HELPERS_H_

#include <jni.h>

#include <memory>

#include "base/android/scoped_java_ref.h"

namespace base {

class WaitableEvent;

namespace android {

class JavaHandlerThread;

// Test-only helpers for working with JavaHandlerThread.
class JavaHandlerThreadHelpers {
 public:
  // Create the Java peer first and test that it works before connecting to the
  // native object.
  static std::unique_ptr<JavaHandlerThread> CreateJavaFirst();

  static void ThrowExceptionAndAbort(WaitableEvent* event);

  static bool IsExceptionTestException(
      ScopedJavaLocalRef<jthrowable> exception);

 private:
  JavaHandlerThreadHelpers() = default;
  ~JavaHandlerThreadHelpers() = default;
};

}  // namespace android
}  // namespace base

#endif  // BASE_TEST_ANDROID_JAVA_HANDLER_THREAD_HELPERS_H_
