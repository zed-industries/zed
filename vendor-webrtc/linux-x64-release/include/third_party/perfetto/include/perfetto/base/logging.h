/*
 * Copyright (C) 2017 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_BASE_LOGGING_H_
#define INCLUDE_PERFETTO_BASE_LOGGING_H_

#include <errno.h>
#include <string.h>  // For strerror.

#include "perfetto/base/build_config.h"
#include "perfetto/base/compiler.h"
#include "perfetto/base/export.h"

#if defined(__GNUC__) || defined(__clang__)
#if defined(__clang__)
#pragma clang diagnostic push
// Fix 'error: #pragma system_header ignored in main file' for clang in Google3.
#pragma clang diagnostic ignored "-Wpragma-system-header-outside-header"
#endif

// Ignore GCC warning about a missing argument for a variadic macro parameter.
#pragma GCC system_header

#if defined(__clang__)
#pragma clang diagnostic pop
#endif
#endif

#if PERFETTO_BUILDFLAG(PERFETTO_FORCE_DCHECK_ON)
#define PERFETTO_DCHECK_IS_ON() 1
#elif PERFETTO_BUILDFLAG(PERFETTO_FORCE_DCHECK_OFF)
#define PERFETTO_DCHECK_IS_ON() 0
#elif defined(DCHECK_ALWAYS_ON) ||                                         \
    (!defined(NDEBUG) && (PERFETTO_BUILDFLAG(PERFETTO_STANDALONE_BUILD) || \
                          PERFETTO_BUILDFLAG(PERFETTO_CHROMIUM_BUILD) ||   \
                          PERFETTO_BUILDFLAG(PERFETTO_ANDROID_BUILD)))
#define PERFETTO_DCHECK_IS_ON() 1
#else
#define PERFETTO_DCHECK_IS_ON() 0
#endif

#if PERFETTO_BUILDFLAG(PERFETTO_FORCE_DLOG_ON)
#define PERFETTO_DLOG_IS_ON() 1
#elif PERFETTO_BUILDFLAG(PERFETTO_FORCE_DLOG_OFF)
#define PERFETTO_DLOG_IS_ON() 0
#else
#define PERFETTO_DLOG_IS_ON() PERFETTO_DCHECK_IS_ON()
#endif

#if defined(PERFETTO_ANDROID_ASYNC_SAFE_LOG)
#if !PERFETTO_BUILDFLAG(PERFETTO_OS_ANDROID) || \
    !PERFETTO_BUILDFLAG(PERFETTO_ANDROID_BUILD)
#error "Async-safe logging is limited to Android tree builds"
#endif
// For binaries which need a very lightweight logging implementation.
// Note that this header is incompatible with android/log.h.
#include <async_safe/log.h>
#elif PERFETTO_BUILDFLAG(PERFETTO_OS_ANDROID)
// Normal android logging.
#include <android/log.h>
#endif

// Enable the "Print the most recent PERFETTO_LOG(s) before crashing" feature
// on Android in-tree builds and on standalone builds (mainly for testing).
// This is deliberately no PERFETTO_OS_ANDROID because we don't want this
// feature when perfetto is embedded in other Android projects (e.g. SDK).
// TODO(b/203795298): TFLite is using the client library in blaze builds and is
// targeting API 19. For now disable the feature based on API level.
#if defined(PERFETTO_ANDROID_ASYNC_SAFE_LOG)
#define PERFETTO_ENABLE_LOG_RING_BUFFER() 0
#elif PERFETTO_BUILDFLAG(PERFETTO_ANDROID_BUILD)
#define PERFETTO_ENABLE_LOG_RING_BUFFER() 1
#elif PERFETTO_BUILDFLAG(PERFETTO_STANDALONE_BUILD) && \
    (!PERFETTO_BUILDFLAG(PERFETTO_OS_ANDROID) ||       \
     (defined(__ANDROID_API__) && __ANDROID_API__ >= 21))
#define PERFETTO_ENABLE_LOG_RING_BUFFER() 1
#else
#define PERFETTO_ENABLE_LOG_RING_BUFFER() 0
#endif

namespace perfetto {
namespace base {

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

enum LogLev { kLogDebug = 0, kLogInfo, kLogImportant, kLogError };

struct LogMessageCallbackArgs {
  LogLev level;
  int line;
  const char* filename;
  const char* message;
};

using LogMessageCallback = void (*)(LogMessageCallbackArgs);

// This is not thread safe and must be called before using tracing from other
// threads.
PERFETTO_EXPORT_COMPONENT void SetLogMessageCallback(
    LogMessageCallback callback);

PERFETTO_EXPORT_COMPONENT void LogMessage(LogLev,
                                          const char* fname,
                                          int line,
                                          const char* fmt,
                                          ...) PERFETTO_PRINTF_FORMAT(4, 5);

// This is defined in debug_crash_stack_trace.cc, but that is only linked in
// standalone && debug builds, see enable_perfetto_stderr_crash_dump in
// perfetto.gni.
PERFETTO_EXPORT_COMPONENT void EnableStacktraceOnCrashForDebug();

#if PERFETTO_ENABLE_LOG_RING_BUFFER()
// Gets a snapshot of the logs from the internal log ring buffer and:
// - On Android in-tree builds: Passes that to android_set_abort_message().
//   That will attach the logs to the crash report.
// - On standalone builds (all otther OSes) prints that on stderr.
// This function must called only once, right before inducing a crash (This is
// because android_set_abort_message() can only be called once).
PERFETTO_EXPORT_COMPONENT void MaybeSerializeLastLogsForCrashReporting();
#else
inline void MaybeSerializeLastLogsForCrashReporting() {}
#endif

#if defined(PERFETTO_ANDROID_ASYNC_SAFE_LOG)
#define PERFETTO_XLOG(level, fmt, ...)                                        \
  do {                                                                        \
    async_safe_format_log((ANDROID_LOG_DEBUG + level), "perfetto",            \
                          "%s:%d " fmt, ::perfetto::base::Basename(__FILE__), \
                          __LINE__, ##__VA_ARGS__);                           \
  } while (0)
#elif defined(PERFETTO_DISABLE_LOG)
#define PERFETTO_XLOG(level, fmt, ...) \
  ::perfetto::base::ignore_result(level, fmt, ##__VA_ARGS__)
#else
#define PERFETTO_XLOG(level, fmt, ...)                                      \
  ::perfetto::base::LogMessage(level, ::perfetto::base::Basename(__FILE__), \
                               __LINE__, fmt, ##__VA_ARGS__)
#endif

#if defined(_MSC_VER)
#define PERFETTO_IMMEDIATE_CRASH()                               \
  do {                                                           \
    ::perfetto::base::MaybeSerializeLastLogsForCrashReporting(); \
    __debugbreak();                                              \
    __assume(0);                                                 \
  } while (0)
#else
#define PERFETTO_IMMEDIATE_CRASH()                               \
  do {                                                           \
    ::perfetto::base::MaybeSerializeLastLogsForCrashReporting(); \
    __builtin_trap();                                            \
    __builtin_unreachable();                                     \
  } while (0)
#endif

#if PERFETTO_BUILDFLAG(PERFETTO_VERBOSE_LOGS)
#define PERFETTO_LOG(fmt, ...) \
  PERFETTO_XLOG(::perfetto::base::kLogInfo, fmt, ##__VA_ARGS__)
#else  // PERFETTO_BUILDFLAG(PERFETTO_VERBOSE_LOGS)
#define PERFETTO_LOG(...) ::perfetto::base::ignore_result(__VA_ARGS__)
#endif  // PERFETTO_BUILDFLAG(PERFETTO_VERBOSE_LOGS)

#define PERFETTO_ILOG(fmt, ...) \
  PERFETTO_XLOG(::perfetto::base::kLogImportant, fmt, ##__VA_ARGS__)
#define PERFETTO_ELOG(fmt, ...) \
  PERFETTO_XLOG(::perfetto::base::kLogError, fmt, ##__VA_ARGS__)
#define PERFETTO_FATAL(fmt, ...)       \
  do {                                 \
    PERFETTO_PLOG(fmt, ##__VA_ARGS__); \
    PERFETTO_IMMEDIATE_CRASH();        \
  } while (0)

#if defined(__GNUC__) || defined(__clang__)
#define PERFETTO_PLOG(x, ...) \
  PERFETTO_ELOG(x " (errno: %d, %s)", ##__VA_ARGS__, errno, strerror(errno))
#else
// MSVC expands __VA_ARGS__ in a different order. Give up, not worth it.
#define PERFETTO_PLOG PERFETTO_ELOG
#endif

#define PERFETTO_CHECK(x)                            \
  do {                                               \
    if (PERFETTO_UNLIKELY(!(x))) {                   \
      PERFETTO_PLOG("%s", "PERFETTO_CHECK(" #x ")"); \
      PERFETTO_IMMEDIATE_CRASH();                    \
    }                                                \
  } while (0)

#if PERFETTO_DLOG_IS_ON()

#define PERFETTO_DLOG(fmt, ...) \
  PERFETTO_XLOG(::perfetto::base::kLogDebug, fmt, ##__VA_ARGS__)

#if defined(__GNUC__) || defined(__clang__)
#define PERFETTO_DPLOG(x, ...) \
  PERFETTO_DLOG(x " (errno: %d, %s)", ##__VA_ARGS__, errno, strerror(errno))
#else
// MSVC expands __VA_ARGS__ in a different order. Give up, not worth it.
#define PERFETTO_DPLOG PERFETTO_DLOG
#endif

#else  // PERFETTO_DLOG_IS_ON()

#define PERFETTO_DLOG(...) ::perfetto::base::ignore_result(__VA_ARGS__)
#define PERFETTO_DPLOG(...) ::perfetto::base::ignore_result(__VA_ARGS__)

#endif  // PERFETTO_DLOG_IS_ON()

#if PERFETTO_DCHECK_IS_ON()

#define PERFETTO_DCHECK(x) PERFETTO_CHECK(x)
#define PERFETTO_DFATAL(...) PERFETTO_FATAL(__VA_ARGS__)
#define PERFETTO_DFATAL_OR_ELOG(...) PERFETTO_DFATAL(__VA_ARGS__)

#else  // PERFETTO_DCHECK_IS_ON()

#define PERFETTO_DCHECK(x) \
  do {                     \
  } while (false && (x))

#define PERFETTO_DFATAL(...) ::perfetto::base::ignore_result(__VA_ARGS__)
#define PERFETTO_DFATAL_OR_ELOG(...) PERFETTO_ELOG(__VA_ARGS__)

#endif  // PERFETTO_DCHECK_IS_ON()

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_BASE_LOGGING_H_
