// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CHECK_H_
#define BASE_CHECK_H_

#include <iosfwd>
#include <memory>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/dcheck_is_on.h"
#include "base/immediate_crash.h"
#include "base/location.h"
#include "base/macros/if.h"
#include "base/macros/is_empty.h"
#include "base/not_fatal_until.h"

// This header defines the CHECK, DCHECK, and DPCHECK macros.
//
// CHECK dies with a fatal error if its condition is not true. It is not
// controlled by NDEBUG, so the check will be executed regardless of compilation
// mode.
//
// DCHECK, the "debug mode" check, is enabled depending on NDEBUG and
// DCHECK_ALWAYS_ON, and its severity depends on DCHECK_IS_CONFIGURABLE.
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
// An optional base::NotFatalUntil argument can be provided to make the
// instance non-fatal (dumps without crashing) before a provided milestone. That
// is: CHECK(false, base::NotFatalUntil::M120); starts crashing in M120. CHECKs
// with a milestone argument preserve logging even in official builds, and
// will upload the CHECK's log message in crash reports for remote diagnostics.
// This is recommended for use in situations that are not flag guarded, or where
// we have low pre-stable coverage. Using this lets us probe for would-be CHECK
// failures for a milestone or two before rolling out a CHECK.
//
// For the (D)CHECK_EQ, etc. macros, see base/check_op.h. However, that header
// is *significantly* larger than check.h, so try to avoid including it in
// header files.

namespace logging {

// Class used to explicitly ignore an ostream, and optionally a boolean value.
class VoidifyStream {
 public:
  VoidifyStream() = default;
  explicit VoidifyStream(bool) {}

  // Binary & has lower precedence than << but higher than ?:
  void operator&(std::ostream&) {}
};

// Macro which uses but does not evaluate expr and any stream parameters.
#define EAT_CHECK_STREAM_PARAMS(expr) \
  true ? (void)0                      \
       : ::logging::VoidifyStream(expr) & (*::logging::g_swallow_stream)
BASE_EXPORT extern std::ostream* g_swallow_stream;

class LogMessage;

// Class used for raising a check error upon destruction.
class BASE_EXPORT CheckError {
 public:
  // Takes ownership of `log_message`.
  explicit CheckError(LogMessage* log_message);

  // All instances that take a base::Location should use
  // base::Location::CurrentWithoutFunctionName() by default since we
  // immediately pass file_name() and line_number() to LogMessage's constructor
  // and discard the function_name() anyways. This saves ~23k on the Android
  // size bots (as of 2024-12-17) but that's the official build that barely uses
  // these for CHECKs. The size gains are believed to be significantly larger on
  // developer builds and official+DCHECK where all CHECK failures generate
  // logs.

  // TODO(pbos): Make all static methods that currently return some version of
  // CheckError return LogMessage*.
  static CheckError Check(const char* condition,
                          base::NotFatalUntil fatal_milestone,
                          const base::Location& location =
                              base::Location::CurrentWithoutFunctionName());
  // Takes ownership over (free()s after using) `log_message_str`, for use with
  // CHECK_op macros.
  static LogMessage* CheckOp(char* log_message_str,
                             base::NotFatalUntil fatal_milestone,
                             const base::Location& location =
                                 base::Location::CurrentWithoutFunctionName());

  static CheckError DCheck(const char* condition,
                           const base::Location& location =
                               base::Location::CurrentWithoutFunctionName());
  // Takes ownership over (free()s after using) `log_message_str`, for use with
  // DCHECK_op macros.
  static LogMessage* DCheckOp(char* log_message_str,
                              const base::Location& location =
                                  base::Location::CurrentWithoutFunctionName());

  static CheckError DumpWillBeCheck(
      const char* condition,
      const base::Location& location =
          base::Location::CurrentWithoutFunctionName());
  // Takes ownership over (free()s after using) `log_message_str`, for use with
  // DUMP_WILL_BE_CHECK_op macros.
  static LogMessage* DumpWillBeCheckOp(
      char* log_message_str,
      const base::Location& location =
          base::Location::CurrentWithoutFunctionName());

  static CheckError DPCheck(const char* condition,
                            const base::Location& location =
                                base::Location::CurrentWithoutFunctionName());

  static CheckError NotImplemented(
      const char* function,
      const base::Location& location =
          base::Location::CurrentWithoutFunctionName());

  // Stream for adding optional details to the error message.
  std::ostream& stream();

  // Try really hard to get the call site and callee as separate stack frames in
  // crash reports.
  NOMERGE NOINLINE NOT_TAIL_CALLED ~CheckError();

  CheckError(const CheckError&) = delete;
  CheckError& operator=(const CheckError&) = delete;

  template <typename T>
  CheckError& operator<<(T&& streamed_type) {
    stream() << streamed_type;
    return *this;
  }

 protected:
  std::unique_ptr<LogMessage> log_message_;
};

// Used for NOTREACHED(), its destructor is importantly [[noreturn]].
class BASE_EXPORT CheckNoreturnError : public CheckError {
 public:
  [[noreturn]] NOMERGE NOINLINE NOT_TAIL_CALLED ~CheckNoreturnError();

  static CheckNoreturnError Check(
      const char* condition,
      const base::Location& location =
          base::Location::CurrentWithoutFunctionName());
  // Takes ownership over (free()s after using) `log_message_str`, for use with
  // CHECK_op macros.
  static LogMessage* CheckOp(char* log_message_str,
                             const base::Location& location =
                                 base::Location::CurrentWithoutFunctionName());

  static CheckNoreturnError PCheck(
      const char* condition,
      const base::Location& location =
          base::Location::CurrentWithoutFunctionName());
  static CheckNoreturnError PCheck(
      const base::Location& location =
          base::Location::CurrentWithoutFunctionName());

 private:
  using CheckError::CheckError;
};

// Used for NOTREACHED(base::NotFatalUntil) and DUMP_WILL_BE_NOTREACHED().
class BASE_EXPORT NotReachedError : public CheckError {
 public:
  static NotReachedError NotReached(
      base::NotFatalUntil fatal_milestone,
      const base::Location& location =
          base::Location::CurrentWithoutFunctionName());

  static NotReachedError DumpWillBeNotReached(
      const base::Location& location =
          base::Location::CurrentWithoutFunctionName());

  NOMERGE NOINLINE NOT_TAIL_CALLED ~NotReachedError();

 private:
  using CheckError::CheckError;
};

// Used for NOTREACHED(), its destructor is importantly [[noreturn]].
class BASE_EXPORT NotReachedNoreturnError : public CheckError {
 public:
  explicit NotReachedNoreturnError(
      const base::Location& location =
          base::Location::CurrentWithoutFunctionName());

  [[noreturn]] NOMERGE NOINLINE NOT_TAIL_CALLED ~NotReachedNoreturnError();
};

// A helper macro for checks that log to streams that makes it easier for the
// compiler to identify and warn about dead code, e.g.:
//
//   return 2;
//   NOTREACHED_IN_MIGRATION();
//
// The 'switch' is used to prevent the 'else' from being ambiguous when the
// macro is used in an 'if' clause such as:
// if (a == 1)
//   CHECK(Foo());
//
// The weird ternary is to still generate an "is not contextually convertible to
// 'bool' when provided weird parameters (regardless of ANALYZER_ASSUME_TRUE's
// implementation). See base/check_nocompile.nc.
//
// The lambda is here to here permit the compiler to out-of-line much of the
// CHECK-failure path and optimize better for the fast path.
#define LOGGING_CHECK_FUNCTION_IMPL(check_stream, condition) \
  switch (0)                                                 \
  case 0:                                                    \
  default:                                                   \
    if (ANALYZER_ASSUME_TRUE((condition) ? true : false))    \
      [[likely]];                                            \
    else                                                     \
      [&]() { return (check_stream); }()

// A helper macro like LOGGING_CHECK_FUNCTION_IMPL above but discarding any
// log-stream parameters rather than evaluate them on failure.
#define DISCARDING_CHECK_FUNCTION_IMPL(check_failure, condition) \
  switch (0)                                                     \
  case 0:                                                        \
  default:                                                       \
    if (!ANALYZER_ASSUME_TRUE((condition) ? true : false))       \
      check_failure;                                             \
    else [[likely]]                                              \
      EAT_CHECK_STREAM_PARAMS()

#if defined(OFFICIAL_BUILD) && !defined(NDEBUG)
#error "Debug builds are not expected to be optimized as official builds."
#endif  // defined(OFFICIAL_BUILD) && !defined(NDEBUG)

#if defined(OFFICIAL_BUILD) && !DCHECK_IS_ON()

// Official non-DCHECK builds do not preserve CHECK() logging (including
// evaluation of logging arguments). This generates more compact code which is
// good for both speed and binary size.
#define CHECK_WILL_STREAM() false

// Note that this uses IMMEDIATE_CRASH_ALWAYS_INLINE to force-inline in debug
// mode as well. See LoggingTest.CheckCausesDistinctBreakpoints.
[[noreturn]] NOMERGE IMMEDIATE_CRASH_ALWAYS_INLINE void CheckFailure() {
  base::ImmediateCrash();
}

// Discard log strings to reduce code bloat when there is no NotFatalUntil
// argument (which temporarily preserves logging both locally and in crash
// reports).
#define CHECK_INTERNAL_IMPL(cond) \
  DISCARDING_CHECK_FUNCTION_IMPL(::logging::CheckFailure(), cond)

#else

// Generate logging versions of CHECKs to help diagnosing failures.
#define CHECK_WILL_STREAM() true

#define CHECK_INTERNAL_IMPL(cond) \
  LOGGING_CHECK_FUNCTION_IMPL(::logging::CheckNoreturnError::Check(#cond), cond)

#endif

#define CHECK(cond, ...)                                         \
  BASE_IF(BASE_IS_EMPTY(__VA_ARGS__), CHECK_INTERNAL_IMPL(cond), \
          LOGGING_CHECK_FUNCTION_IMPL(                           \
              logging::CheckError::Check(#cond, __VA_ARGS__), cond))

// Strip the conditional string based on CHECK_WILL_STREAM()
#define PCHECK(cond)                                        \
  LOGGING_CHECK_FUNCTION_IMPL(                              \
      BASE_IF(CHECK_WILL_STREAM(),                          \
              ::logging::CheckNoreturnError::PCheck(#cond), \
              ::logging::CheckNoreturnError::PCheck()),     \
      cond)

#if DCHECK_IS_ON()

#define DCHECK(condition)                                                \
  LOGGING_CHECK_FUNCTION_IMPL(::logging::CheckError::DCheck(#condition), \
                              condition)
#define DPCHECK(condition)                                                \
  LOGGING_CHECK_FUNCTION_IMPL(::logging::CheckError::DPCheck(#condition), \
                              condition)

#else

#define DCHECK(condition) EAT_CHECK_STREAM_PARAMS(!(condition))
#define DPCHECK(condition) EAT_CHECK_STREAM_PARAMS(!(condition))

#endif  // DCHECK_IS_ON()

// The DUMP_WILL_BE_CHECK() macro provides a convenient way to non-fatally dump
// in official builds if a condition is false. This is used to more cautiously
// roll out a new CHECK() (or upgrade a DCHECK) where the caller isn't entirely
// sure that something holds true in practice (but asserts that it should). This
// is especially useful for platforms that have a low pre-stable population and
// code areas that are rarely exercised.
//
// On DCHECK builds this macro matches DCHECK behavior.
//
// This macro isn't optimized (preserves filename, line number and log messages
// in official builds), as they are expected to be in product temporarily. When
// using this macro, leave a TODO(crbug.com/nnnn) entry referring to a bug
// related to its rollout. Then put a NextAction on the bug to come back and
// clean this up (replace with a CHECK). A DUMP_WILL_BE_CHECK() that's been left
// untouched for a long time without bug updates suggests that issues that
// would've prevented enabling this CHECK have either not been discovered or
// have been resolved.
//
// Using this macro is preferred over direct base::debug::DumpWithoutCrashing()
// invocations as it communicates intent to eventually end up as a CHECK. It
// also preserves the log message so setting crash keys to get additional debug
// info isn't required as often.
#define DUMP_WILL_BE_CHECK(condition, ...)                                \
  LOGGING_CHECK_FUNCTION_IMPL(::logging::CheckError::DumpWillBeCheck(     \
                                  #condition __VA_OPT__(, ) __VA_ARGS__), \
                              condition)

// Async signal safe checking mechanism.
[[noreturn]] BASE_EXPORT void RawCheckFailure(const char* message);
#define RAW_CHECK(condition)                                        \
  do {                                                              \
    if (!(condition)) [[unlikely]] {                                \
      ::logging::RawCheckFailure("Check failed: " #condition "\n"); \
    }                                                               \
  } while (0)

}  // namespace logging

#endif  // BASE_CHECK_H_
