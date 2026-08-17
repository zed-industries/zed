// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_CHECK_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_CHECK_H_

#include <iosfwd>

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/immediate_crash.h"
#include "partition_alloc/partition_alloc_base/log_message.h"
#include "partition_alloc/partition_alloc_base/strings/cstring_builder.h"

#if PA_BUILDFLAG(IS_WIN) && defined(COMPONENT_BUILD)
#include "partition_alloc/partition_alloc_base/strings/safe_sprintf.h"
#endif  // PA_BUILDFLAG(IS_WIN) && defined(COMPONENT_BUILD)

#define PA_STRINGIFY_IMPL(s) #s
#define PA_STRINGIFY(s) PA_STRINGIFY_IMPL(s)

// This header defines the CHECK, DCHECK, and DPCHECK macros.
//
// CHECK dies with a fatal error if its condition is not true. It is not
// controlled by PA_BUILDFLAG(IS_DEBUG), so the check will be executed
// regardless of compilation mode.
//
// DCHECK, the "debug mode" check, is enabled depending on
// PA_BUILDFLAG(IS_DEBUG) and PA_BUILDFLAG(DCHECK_ALWAYS_ON), and its severity
// depends on PA_BUILDFLAG(DCHECK_IS_CONFIGURABLE).
//
// (D)PCHECK is like (D)CHECK, but includes the system error code (c.f.
// perror(3)).
//
// Additional information can be streamed to these macros and will be included
// in the log output if the condition doesn't hold (you may need to include
// <ostream>):
//
//   CHECK(condition) << "Additional info.";
//
// The condition is evaluated exactly once. Even in build modes where e.g.
// DCHECK is disabled, the condition and any stream arguments are still
// referenced to avoid warnings about unused variables and functions.
//
// For the (D)CHECK_EQ, etc. macros, see base/check_op.h. However, that header
// is *significantly* larger than check.h, so try to avoid including it in
// header files.

namespace partition_alloc::internal::logging {

// Class used to explicitly ignore an ostream, and optionally a boolean value.
class VoidifyStream {
 public:
  VoidifyStream() = default;
  explicit VoidifyStream(bool ignored) {}

  // This operator has lower precedence than << but higher than ?:
  void operator&(base::strings::CStringBuilder&) {}
};

// Helper macro which avoids evaluating the arguments to a stream if the
// condition is false.
#define PA_LAZY_CHECK_STREAM(stream, condition) \
  !(condition)                                  \
      ? (void)0                                 \
      : ::partition_alloc::internal::logging::VoidifyStream() & (stream)

// Macro which uses but does not evaluate expr and any stream parameters.
#define PA_EAT_CHECK_STREAM_PARAMS(expr)                             \
  true ? (void)0                                                     \
       : ::partition_alloc::internal::logging::VoidifyStream(expr) & \
             (*::partition_alloc::internal::logging::g_swallow_stream)
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
extern base::strings::CStringBuilder* g_swallow_stream;

class LogMessage;

// Class used for raising a check error upon destruction.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) CheckError {
 public:
  // Stream for adding optional details to the error message.
  base::strings::CStringBuilder& stream();
  PA_NOMERGE ~CheckError();

 protected:
  CheckError(const char* file,
             int line,
             LogSeverity severity,
             const char* condition);
  CheckError(const char* file, int line, LogSeverity severity);
  CheckError(const char* file,
             int line,
             LogSeverity severity,
             const char* condition,
             SystemErrorCode err_code);

  union {
    LogMessage log_message_;
#if PA_BUILDFLAG(IS_WIN)
    Win32ErrorLogMessage errno_log_message_;
#else
    ErrnoLogMessage errno_log_message_;
#endif
  };

  // |has_errno| describes which union member is used, |log_message_| or
  // |errno_log_message_|. If |has_errno| is true, CheckError initializes
  // |errno_log_message_| at its constructor and destroys at its destructor.
  // (This also means the CheckError is an instance of the parent class of
  // PCheck or DPCheck.)
  // If false, CheckError initializes and destroys |log_message_|.
  const bool has_errno = false;
};

namespace check_error {

// Class used for raising a check error upon destruction.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) Check : public CheckError {
 public:
  Check(const char* file, int line, const char* condition);
};

class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) DCheck : public CheckError {
 public:
  DCheck(const char* file, int line, const char* condition);
};

class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) PCheck : public CheckError {
 public:
  PCheck(const char* file, int line, const char* condition);
  PCheck(const char* file, int line);
};

class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) DPCheck : public CheckError {
 public:
  DPCheck(const char* file, int line, const char* condition);
};

class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) NotImplemented
    : public CheckError {
 public:
  NotImplemented(const char* file, int line, const char* function);
};

}  // namespace check_error

#if defined(OFFICIAL_BUILD) && PA_BUILDFLAG(IS_DEBUG)
#error "Debug builds are not expected to be optimized as official builds."
#endif  // defined(OFFICIAL_BUILD) && BUILDFLAG(IS_DEBUG)

#if defined(OFFICIAL_BUILD) && !PA_BUILDFLAG(DCHECKS_ARE_ON)

// TODO(crbug.com/357081797): Use `[[unlikely]]` instead when there's a way to
// switch the expression below to a statement without breaking
// -Wthread-safety-analysis.
#if PA_HAS_BUILTIN(__builtin_expect)
#define PA_BASE_INTERNAL_EXPECT_FALSE(cond) __builtin_expect(!(cond), 0)
#else
#define PA_BASE_INTERNAL_EXPECT_FALSE(cond) !(cond)
#endif
// Discard log strings to reduce code bloat.
//
// This is not calling BreakDebugger since this is called frequently, and
// calling an out-of-line function instead of a noreturn inline macro prevents
// compiler optimizations.
#define PA_BASE_CHECK(cond)                                  \
  PA_BASE_INTERNAL_EXPECT_FALSE(cond) ? PA_IMMEDIATE_CRASH() \
                                      : PA_EAT_CHECK_STREAM_PARAMS()

#define PA_BASE_CHECK_WILL_STREAM() false

#define PA_BASE_PCHECK(cond)                                              \
  PA_LAZY_CHECK_STREAM(                                                   \
      ::partition_alloc::internal::logging::check_error::PCheck(__FILE__, \
                                                                __LINE__) \
          .stream(),                                                      \
      PA_BASE_INTERNAL_EXPECT_FALSE(cond))

#else

#define PA_BASE_CHECK(condition)                                \
  PA_LAZY_CHECK_STREAM(                                         \
      ::partition_alloc::internal::logging::check_error::Check( \
          __FILE__, __LINE__, #condition)                       \
          .stream(),                                            \
      !PA_ANALYZER_ASSUME_TRUE(condition))

#define PA_BASE_CHECK_WILL_STREAM() true

#define PA_BASE_PCHECK(condition)                                \
  PA_LAZY_CHECK_STREAM(                                          \
      ::partition_alloc::internal::logging::check_error::PCheck( \
          __FILE__, __LINE__, #condition)                        \
          .stream(),                                             \
      !PA_ANALYZER_ASSUME_TRUE(condition))

#endif

#if PA_BUILDFLAG(DCHECKS_ARE_ON)

#define PA_BASE_DCHECK(condition)                                \
  PA_LAZY_CHECK_STREAM(                                          \
      ::partition_alloc::internal::logging::check_error::DCheck( \
          __FILE__, __LINE__, #condition)                        \
          .stream(),                                             \
      !PA_ANALYZER_ASSUME_TRUE(condition))

#define PA_BASE_DPCHECK(condition)                                \
  PA_LAZY_CHECK_STREAM(                                           \
      ::partition_alloc::internal::logging::check_error::DPCheck( \
          __FILE__, __LINE__, #condition)                         \
          .stream(),                                              \
      !PA_ANALYZER_ASSUME_TRUE(condition))

#else

#define PA_BASE_DCHECK(condition) PA_EAT_CHECK_STREAM_PARAMS(!(condition))
#define PA_BASE_DPCHECK(condition) PA_EAT_CHECK_STREAM_PARAMS(!(condition))

#endif

// Async signal safe checking mechanism.
[[noreturn]] PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) void RawCheckFailure(
    const char* message);
#define PA_RAW_CHECK(condition)                              \
  do {                                                       \
    if (!(condition))                                        \
      ::partition_alloc::internal::logging::RawCheckFailure( \
          "Check failed: " #condition "\n");                 \
  } while (0)

#if PA_BUILDFLAG(IS_WIN) && defined(COMPONENT_BUILD)
template <typename... Args>
[[noreturn]] void RawCheckFailureFormat(const char* fmt, Args... args) {
  constexpr size_t kRawCheckFailureFormatBufferSize = 256u;
  char buffer[kRawCheckFailureFormatBufferSize];
  (void)::partition_alloc::internal::base::strings::SafeSPrintf(buffer, fmt,
                                                                args...);
  ::partition_alloc::internal::logging::RawCheckFailure(buffer);
}
#endif  // PA_BUILDFLAG(IS_WIN) && defined(COMPONENT_BUILD)

}  // namespace partition_alloc::internal::logging

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_CHECK_H_
