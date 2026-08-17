// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef JNI_ZERO_LOGGING_H_
#define JNI_ZERO_LOGGING_H_

#include "third_party/jni_zero/jni_export.h"

#if defined(NDEBUG) && !defined(DCHECK_ALWAYS_ON)
#define JNI_ZERO_DCHECK_IS_ON() false
#else
#define JNI_ZERO_DCHECK_IS_ON() true
#endif

// Simplified version of Google's logging. Adapted from perfetto's
// implementation.
namespace jni_zero {

// Constexpr functions to extract basename(__FILE__), e.g.: ../foo/f.c -> f.c .
constexpr const char* StrEnd(const char* s) {
  return *s ? StrEnd(s + 1) : s;
}

constexpr const char* BasenameRecursive(const char* s,
                                        const char* begin,
                                        const char* end) {
  return (*s == '/' && s < end)
             ? (s + 1)
             : ((s > begin) ? BasenameRecursive(s - 1, begin, end) : s);
}

constexpr const char* Basename(const char* str) {
  return BasenameRecursive(StrEnd(str), str, StrEnd(str));
}

enum LogLev { kLogInfo = 0, kLogError, kLogFatal };

struct LogMessageCallbackArgs {
  LogLev level;
  int line;
  const char* filename;
  const char* message;
};

using LogMessageCallback = void (*)(LogMessageCallbackArgs);

// This is not thread safe and must be called before using tracing from other
// threads.
JNI_ZERO_COMPONENT_BUILD_EXPORT void SetLogMessageCallback(
    LogMessageCallback callback);

JNI_ZERO_COMPONENT_BUILD_EXPORT void LogMessage(LogLev,
                                                const char* fname,
                                                int line,
                                                const char* fmt,
                                                ...)
    __attribute__((__format__(__printf__, 4, 5)));

#define JNI_ZERO_IMMEDIATE_CRASH() \
  do {                             \
    __builtin_trap();              \
    __builtin_unreachable();       \
  } while (0)
#define JNI_ZERO_XLOG(level, fmt, ...)                                         \
  ::jni_zero::LogMessage(level, ::jni_zero::Basename(__FILE__), __LINE__, fmt, \
                         ##__VA_ARGS__)
#define JNI_ZERO_ILOG(fmt, ...) \
  JNI_ZERO_XLOG(::jni_zero::kLogInfo, fmt, ##__VA_ARGS__)
#define JNI_ZERO_ELOG(fmt, ...) \
  JNI_ZERO_XLOG(::jni_zero::kLogError, fmt, ##__VA_ARGS__)
#define JNI_ZERO_FLOG(fmt, ...) \
  JNI_ZERO_XLOG(::jni_zero::kLogFatal, fmt, ##__VA_ARGS__)

#define JNI_ZERO_CHECK(x)                            \
  do {                                               \
    if (__builtin_expect(!(x), 0)) {                 \
      JNI_ZERO_FLOG("%s", "JNI_ZERO_CHECK(" #x ")"); \
    }                                                \
  } while (0)
#if JNI_ZERO_DCHECK_IS_ON()
#define JNI_ZERO_DCHECK(x) JNI_ZERO_CHECK(x)
#else
#define JNI_ZERO_DCHECK(x) \
  do {                     \
  } while (false && (x))
#endif
}  // namespace jni_zero

#endif  // JNI_ZERO_LOGGING_H_
