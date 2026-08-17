/*
 * Copyright (C) 2007 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_ANDROID_SDK_NATIVEHELPER_JNIHELP_H_
#define SRC_ANDROID_SDK_NATIVEHELPER_JNIHELP_H_

/*
 * JNI helper functions.
 *
 * This file may be included by C or C++ code, which is trouble because jni.h
 * uses different typedefs for JNIEnv in each language.
 */

// Copied from
// https://cs.android.com/android/platform/superproject/main/+/main:libnativehelper/include/nativehelper/JNIHelp.h;drc=88c826d2c0b20f8c29e5549333c49fb824055e6a

#include <sys/cdefs.h>

#include <errno.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <jni.h>

#include <android/log.h>

// Avoid formatting this as it must match webview's usage
// (webview/graphics_utils.cpp).
// clang-format off
#ifndef NELEM
#define NELEM(x) ((int) (sizeof(x) / sizeof((x)[0])))
#endif
// clang-format on

/*
 * For C++ code, we provide inlines that map to the C functions.  g++ always
 * inlines these, even on non-optimized builds.
 */
#if defined(__cplusplus)

namespace android::jnihelp {
struct [[maybe_unused]] ExpandableString {
  size_t dataSize;  // The length of the C string data (not including the
                    // null-terminator).
  char* data;       // The C string data.
};

[[maybe_unused]] static void ExpandableStringInitialize(
    struct ExpandableString* s) {
  memset(s, 0, sizeof(*s));
}

[[maybe_unused]] static void ExpandableStringRelease(
    struct ExpandableString* s) {
  free(s->data);
  memset(s, 0, sizeof(*s));
}

[[maybe_unused]] static bool ExpandableStringAppend(struct ExpandableString* s,
                                                    const char* text) {
  size_t textSize = strlen(text);
  size_t requiredSize = s->dataSize + textSize + 1;
  char* data = (char*)realloc(s->data, requiredSize);
  if (data == NULL) {
    return false;
  }
  s->data = data;
  memcpy(s->data + s->dataSize, text, textSize + 1);
  s->dataSize += textSize;
  return true;
}

[[maybe_unused]] static bool ExpandableStringAssign(struct ExpandableString* s,
                                                    const char* text) {
  ExpandableStringRelease(s);
  return ExpandableStringAppend(s, text);
}

[[maybe_unused]] inline char* safe_strerror(char* (*strerror_r_method)(int,
                                                                       char*,
                                                                       size_t),
                                            int errnum,
                                            char* buf,
                                            size_t buflen) {
  return strerror_r_method(errnum, buf, buflen);
}

[[maybe_unused]] inline char* safe_strerror(int (*strerror_r_method)(int,
                                                                     char*,
                                                                     size_t),
                                            int errnum,
                                            char* buf,
                                            size_t buflen) {
  int rc = strerror_r_method(errnum, buf, buflen);
  if (rc != 0) {
    snprintf(buf, buflen, "errno %d", errnum);
  }
  return buf;
}

[[maybe_unused]] static const char* platformStrError(int errnum,
                                                     char* buf,
                                                     size_t buflen) {
#ifdef _WIN32
  strerror_s(buf, buflen, errnum);
  return buf;
#else
  return safe_strerror(strerror_r, errnum, buf, buflen);
#endif
}

[[maybe_unused]] static jmethodID FindMethod(JNIEnv* env,
                                             const char* className,
                                             const char* methodName,
                                             const char* descriptor) {
  // This method is only valid for classes in the core library which are
  // not unloaded during the lifetime of managed code execution.
  jclass clazz = env->FindClass(className);
  jmethodID methodId = env->GetMethodID(clazz, methodName, descriptor);
  env->DeleteLocalRef(clazz);
  return methodId;
}

[[maybe_unused]] static bool AppendJString(JNIEnv* env,
                                           jstring text,
                                           struct ExpandableString* dst) {
  const char* utfText = env->GetStringUTFChars(text, NULL);
  if (utfText == NULL) {
    return false;
  }
  bool success = ExpandableStringAppend(dst, utfText);
  env->ReleaseStringUTFChars(text, utfText);
  return success;
}

/*
 * Returns a human-readable summary of an exception object.  The buffer will
 * be populated with the "binary" class name and, if present, the
 * exception message.
 */
[[maybe_unused]] static bool GetExceptionSummary(JNIEnv* env,
                                                 jthrowable thrown,
                                                 struct ExpandableString* dst) {
  // Summary is <exception_class_name> ": " <exception_message>
  jclass exceptionClass = env->GetObjectClass(thrown);  // Always succeeds
  jmethodID getName =
      FindMethod(env, "java/lang/Class", "getName", "()Ljava/lang/String;");
  jstring className = (jstring)env->CallObjectMethod(exceptionClass, getName);
  if (className == NULL) {
    ExpandableStringAssign(dst, "<error getting class name>");
    env->ExceptionClear();
    env->DeleteLocalRef(exceptionClass);
    return false;
  }
  env->DeleteLocalRef(exceptionClass);
  exceptionClass = NULL;

  if (!AppendJString(env, className, dst)) {
    ExpandableStringAssign(dst, "<error getting class name UTF-8>");
    env->ExceptionClear();
    env->DeleteLocalRef(className);
    return false;
  }
  env->DeleteLocalRef(className);
  className = NULL;

  jmethodID getMessage = FindMethod(env, "java/lang/Throwable", "getMessage",
                                    "()Ljava/lang/String;");
  jstring message = (jstring)env->CallObjectMethod(thrown, getMessage);
  if (message == NULL) {
    return true;
  }

  bool success =
      (ExpandableStringAppend(dst, ": ") && AppendJString(env, message, dst));
  if (!success) {
    // Two potential reasons for reaching here:
    //
    // 1. managed heap allocation failure (OOME).
    // 2. native heap allocation failure for the storage in |dst|.
    //
    // Attempt to append failure notification, okay to fail, |dst| contains the
    // class name of |thrown|.
    ExpandableStringAppend(dst, "<error getting message>");
    // Clear OOME if present.
    env->ExceptionClear();
  }
  env->DeleteLocalRef(message);
  message = NULL;
  return success;
}

[[maybe_unused]] static jobject NewStringWriter(JNIEnv* env) {
  jclass clazz = env->FindClass("java/io/StringWriter");
  jmethodID init = env->GetMethodID(clazz, "<init>", "()V");
  jobject instance = env->NewObject(clazz, init);
  env->DeleteLocalRef(clazz);
  return instance;
}

[[maybe_unused]] static jstring StringWriterToString(JNIEnv* env,
                                                     jobject stringWriter) {
  jmethodID toString = FindMethod(env, "java/io/StringWriter", "toString",
                                  "()Ljava/lang/String;");
  return (jstring)env->CallObjectMethod(stringWriter, toString);
}

[[maybe_unused]] static jobject NewPrintWriter(JNIEnv* env, jobject writer) {
  jclass clazz = env->FindClass("java/io/PrintWriter");
  jmethodID init = env->GetMethodID(clazz, "<init>", "(Ljava/io/Writer;)V");
  jobject instance = env->NewObject(clazz, init, writer);
  env->DeleteLocalRef(clazz);
  return instance;
}

[[maybe_unused]] static bool GetStackTrace(JNIEnv* env,
                                           jthrowable thrown,
                                           struct ExpandableString* dst) {
  // This function is equivalent to the following Java snippet:
  //   StringWriter sw = new StringWriter();
  //   PrintWriter pw = new PrintWriter(sw);
  //   thrown.printStackTrace(pw);
  //   String trace = sw.toString();
  //   return trace;
  jobject sw = NewStringWriter(env);
  if (sw == NULL) {
    return false;
  }

  jobject pw = NewPrintWriter(env, sw);
  if (pw == NULL) {
    env->DeleteLocalRef(sw);
    return false;
  }

  jmethodID printStackTrace =
      FindMethod(env, "java/lang/Throwable", "printStackTrace",
                 "(Ljava/io/PrintWriter;)V");
  env->CallVoidMethod(thrown, printStackTrace, pw);

  jstring trace = StringWriterToString(env, sw);

  env->DeleteLocalRef(pw);
  pw = NULL;
  env->DeleteLocalRef(sw);
  sw = NULL;

  if (trace == NULL) {
    return false;
  }

  bool success = AppendJString(env, trace, dst);
  env->DeleteLocalRef(trace);
  return success;
}

[[maybe_unused]] static void GetStackTraceOrSummary(
    JNIEnv* env,
    jthrowable thrown,
    struct ExpandableString* dst) {
  // This method attempts to get a stack trace or summary info for an exception.
  // The exception may be provided in the |thrown| argument to this function.
  // If |thrown| is NULL, then any pending exception is used if it exists.

  // Save pending exception, callees may raise other exceptions. Any pending
  // exception is rethrown when this function exits.
  jthrowable pendingException = env->ExceptionOccurred();
  if (pendingException != NULL) {
    env->ExceptionClear();
  }

  if (thrown == NULL) {
    if (pendingException == NULL) {
      ExpandableStringAssign(dst, "<no pending exception>");
      return;
    }
    thrown = pendingException;
  }

  if (!GetStackTrace(env, thrown, dst)) {
    // GetStackTrace may have raised an exception, clear it since it's not for
    // the caller.
    env->ExceptionClear();
    GetExceptionSummary(env, thrown, dst);
  }

  if (pendingException != NULL) {
    // Re-throw the pending exception present when this method was called.
    env->Throw(pendingException);
    env->DeleteLocalRef(pendingException);
  }
}

[[maybe_unused]] static void DiscardPendingException(JNIEnv* env,
                                                     const char* className) {
  jthrowable exception = env->ExceptionOccurred();
  env->ExceptionClear();
  if (exception == NULL) {
    return;
  }

  struct ExpandableString summary;
  ExpandableStringInitialize(&summary);
  GetExceptionSummary(env, exception, &summary);
  const char* details = (summary.data != NULL) ? summary.data : "Unknown";
  __android_log_print(ANDROID_LOG_WARN, "JNIHelp",
                      "Discarding pending exception (%s) to throw %s", details,
                      className);
  ExpandableStringRelease(&summary);
  env->DeleteLocalRef(exception);
}

[[maybe_unused]] static int ThrowException(JNIEnv* env,
                                           const char* className,
                                           const char* ctorSig,
                                           ...) {
  int status = -1;
  jclass exceptionClass = NULL;

  va_list args;
  va_start(args, ctorSig);

  DiscardPendingException(env, className);

  {
    /* We want to clean up local references before returning from this function,
     * so, regardless of return status, the end block must run. Have the work
     * done in a nested block to avoid using any uninitialized variables in the
     * end block. */
    exceptionClass = env->FindClass(className);
    if (exceptionClass == NULL) {
      __android_log_print(ANDROID_LOG_ERROR, "JNIHelp",
                          "Unable to find exception class %s", className);
      /* an exception, most likely ClassNotFoundException, will now be pending
       */
      goto end;
    }

    jmethodID init = env->GetMethodID(exceptionClass, "<init>", ctorSig);
    if (init == NULL) {
      __android_log_print(ANDROID_LOG_ERROR, "JNIHelp",
                          "Failed to find constructor for '%s' '%s'", className,
                          ctorSig);
      goto end;
    }

    jobject instance = env->NewObjectV(exceptionClass, init, args);
    if (instance == NULL) {
      __android_log_print(ANDROID_LOG_ERROR, "JNIHelp",
                          "Failed to construct '%s'", className);
      goto end;
    }

    if (env->Throw((jthrowable)instance) != JNI_OK) {
      __android_log_print(ANDROID_LOG_ERROR, "JNIHelp", "Failed to throw '%s'",
                          className);
      /* an exception, most likely OOM, will now be pending */
      goto end;
    }

    /* everything worked fine, just update status to success and clean up */
    status = 0;
  }

end:
  va_end(args);
  if (exceptionClass != NULL) {
    env->DeleteLocalRef(exceptionClass);
  }
  return status;
}

[[maybe_unused]] static jstring CreateExceptionMsg(JNIEnv* env,
                                                   const char* msg) {
  jstring detailMessage = env->NewStringUTF(msg);
  if (detailMessage == NULL) {
    /* Not really much we can do here. We're probably dead in the water,
    but let's try to stumble on... */
    env->ExceptionClear();
  }
  return detailMessage;
}
}  // namespace android::jnihelp

/*
 * Register one or more native methods with a particular class.  "className"
 * looks like "java/lang/String". Aborts on failure, returns 0 on success.
 */
[[maybe_unused]] static int jniRegisterNativeMethods(
    JNIEnv* env,
    const char* className,
    const JNINativeMethod* methods,
    int numMethods) {
  using namespace android::jnihelp;
  jclass clazz = env->FindClass(className);
  if (clazz == NULL) {
    __android_log_assert(
        "clazz == NULL", "JNIHelp",
        "Native registration unable to find class '%s'; aborting...",
        className);
  }
  int result = env->RegisterNatives(clazz, methods, numMethods);
  env->DeleteLocalRef(clazz);
  if (result == 0) {
    return 0;
  }

  // Failure to register natives is fatal. Try to report the corresponding
  // exception, otherwise abort with generic failure message.
  jthrowable thrown = env->ExceptionOccurred();
  if (thrown != NULL) {
    struct ExpandableString summary;
    ExpandableStringInitialize(&summary);
    if (GetExceptionSummary(env, thrown, &summary)) {
      __android_log_print(ANDROID_LOG_FATAL, "JNIHelp", "%s", summary.data);
    }
    ExpandableStringRelease(&summary);
    env->DeleteLocalRef(thrown);
  }
  __android_log_print(ANDROID_LOG_FATAL, "JNIHelp",
                      "RegisterNatives failed for '%s'; aborting...",
                      className);
  return result;
}

/*
 * Throw an exception with the specified class and an optional message.
 *
 * The "className" argument will be passed directly to FindClass, which
 * takes strings with slashes (e.g. "java/lang/Object").
 *
 * If an exception is currently pending, we log a warning message and
 * clear it.
 *
 * Returns 0 on success, nonzero if something failed (e.g. the exception
 * class couldn't be found, so *an* exception will still be pending).
 *
 * Currently aborts the VM if it can't throw the exception.
 */
[[maybe_unused]] static int jniThrowException(JNIEnv* env,
                                              const char* className,
                                              const char* msg) {
  using namespace android::jnihelp;
  jstring _detailMessage = CreateExceptionMsg(env, msg);
  int _status =
      ThrowException(env, className, "(Ljava/lang/String;)V", _detailMessage);
  if (_detailMessage != NULL) {
    env->DeleteLocalRef(_detailMessage);
  }
  return _status;
}

/*
 * Throw an android.system.ErrnoException, with the given function name and
 * errno value.
 */
[[maybe_unused]] static int jniThrowErrnoException(JNIEnv* env,
                                                   const char* functionName,
                                                   int errnum) {
  using namespace android::jnihelp;
  jstring _detailMessage = CreateExceptionMsg(env, functionName);
  int _status =
      ThrowException(env, "android/system/ErrnoException",
                     "(Ljava/lang/String;I)V", _detailMessage, errnum);
  if (_detailMessage != NULL) {
    env->DeleteLocalRef(_detailMessage);
  }
  return _status;
}

/*
 * Throw an exception with the specified class and formatted error message.
 *
 * The "className" argument will be passed directly to FindClass, which
 * takes strings with slashes (e.g. "java/lang/Object").
 *
 * If an exception is currently pending, we log a warning message and
 * clear it.
 *
 * Returns 0 on success, nonzero if something failed (e.g. the exception
 * class couldn't be found, so *an* exception will still be pending).
 *
 * Currently aborts the VM if it can't throw the exception.
 */
[[maybe_unused]] static int jniThrowExceptionFmt(JNIEnv* env,
                                                 const char* className,
                                                 const char* fmt,
                                                 ...) {
  va_list args;
  va_start(args, fmt);
  char msgBuf[512];
  vsnprintf(msgBuf, sizeof(msgBuf), fmt, args);
  va_end(args);
  return jniThrowException(env, className, msgBuf);
}

[[maybe_unused]] static int jniThrowNullPointerException(JNIEnv* env,
                                                         const char* msg) {
  return jniThrowException(env, "java/lang/NullPointerException", msg);
}

[[maybe_unused]] static int jniThrowRuntimeException(JNIEnv* env,
                                                     const char* msg) {
  return jniThrowException(env, "java/lang/RuntimeException", msg);
}

[[maybe_unused]] static int jniThrowIOException(JNIEnv* env, int errno_value) {
  using namespace android::jnihelp;
  char buffer[80];
  const char* message = platformStrError(errno_value, buffer, sizeof(buffer));
  return jniThrowException(env, "java/io/IOException", message);
}

/*
 * Returns a Java String object created from UTF-16 data either from jchar or,
 * if called from C++11, char16_t (a bitwise identical distinct type).
 */
[[maybe_unused]] static inline jstring
jniCreateString(JNIEnv* env, const jchar* unicodeChars, jsize len) {
  return env->NewString(unicodeChars, len);
}

[[maybe_unused]] static inline jstring
jniCreateString(JNIEnv* env, const char16_t* unicodeChars, jsize len) {
  return jniCreateString(env, reinterpret_cast<const jchar*>(unicodeChars),
                         len);
}

/*
 * Log a message and an exception.
 * If exception is NULL, logs the current exception in the JNI environment.
 */
[[maybe_unused]] static void jniLogException(JNIEnv* env,
                                             int priority,
                                             const char* tag,
                                             jthrowable exception = NULL) {
  using namespace android::jnihelp;
  struct ExpandableString summary;
  ExpandableStringInitialize(&summary);
  GetStackTraceOrSummary(env, exception, &summary);
  const char* details =
      (summary.data != NULL) ? summary.data : "No memory to report exception";
  __android_log_write(priority, tag, details);
  ExpandableStringRelease(&summary);
}

#else  // defined(__cplusplus)

// ART-internal only methods (not exported), exposed for legacy C users

int jniRegisterNativeMethods(JNIEnv* env,
                             const char* className,
                             const JNINativeMethod* gMethods,
                             int numMethods);

void jniLogException(JNIEnv* env,
                     int priority,
                     const char* tag,
                     jthrowable thrown);

int jniThrowException(JNIEnv* env, const char* className, const char* msg);

int jniThrowNullPointerException(JNIEnv* env, const char* msg);

#endif  // defined(__cplusplus)

#endif  // SRC_ANDROID_SDK_NATIVEHELPER_JNIHELP_H_
