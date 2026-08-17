// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_JAVA_EXCEPTION_REPORTER_H_
#define BASE_ANDROID_JAVA_EXCEPTION_REPORTER_H_

#include <jni.h>

#include "base/android/scoped_java_ref.h"
#include "base/base_export.h"
#include "base/functional/callback_forward.h"

namespace base {
namespace android {

using JavaExceptionCallback = void (*)(const char* exception);

// Install the exception handler. This should only be called once per process.
BASE_EXPORT void InitJavaExceptionReporter();

// Same as above except the handler ensures child process exists immediately
// after an unhandled exception. This is used for child processes because
// DumpWithoutCrashing does not work for child processes on Android.
BASE_EXPORT void InitJavaExceptionReporterForChildProcess();

// Sets a callback to be called with the contents of a Java exception, which may
// be nullptr.
BASE_EXPORT void SetJavaExceptionCallback(JavaExceptionCallback callback);

// Returns the value last passed to SetJavaException(), or nullptr if it has
// not been called.
BASE_EXPORT JavaExceptionCallback GetJavaExceptionCallback();

// Calls the Java exception callback, if any, with exception.
void SetJavaException(const char* exception);

// Sets a filter that determines whether a java exception should cause a crash
// report. |java_exception_filter| should return true if a crash report should
// be generated.
BASE_EXPORT void SetJavaExceptionFilter(
    base::RepeatingCallback<bool(const JavaRef<jthrowable>&)>
        java_exception_filter);

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_JAVA_EXCEPTION_REPORTER_H_
