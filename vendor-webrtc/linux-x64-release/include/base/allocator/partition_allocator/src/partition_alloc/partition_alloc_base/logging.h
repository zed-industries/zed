// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_LOGGING_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_LOGGING_H_

#include <cassert>
#include <cstddef>
#include <cstdint>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/log_message.h"

// TODO(crbug.com/40158212): Need to update the description, because logging for
// PA standalone library was minimized.
//
// Optional message capabilities
// -----------------------------
// Assertion failed messages and fatal errors are displayed in a dialog box
// before the application exits. However, running this UI creates a message
// loop, which causes application messages to be processed and potentially
// dispatched to existing application windows. Since the application is in a
// bad state when this assertion dialog is displayed, these messages may not
// get processed and hang the dialog, or the application might go crazy.
//
// Therefore, it can be beneficial to display the error dialog in a separate
// process from the main application. When the logging system needs to display
// a fatal error dialog box, it will look for a program called
// "DebugMessage.exe" in the same directory as the application executable. It
// will run this application with the message as the command line, and will
// not include the name of the application as is traditional for easier
// parsing.
//
// The code for DebugMessage.exe is only one line. In WinMain, do:
//   MessageBox(NULL, GetCommandLineW(), L"Fatal Error", 0);
//
// If DebugMessage.exe is not found, the logging code will use a normal
// MessageBox, potentially causing the problems discussed above.

// Instructions
// ------------
//
// Make a bunch of macros for logging.  The way to log things is to stream
// things to PA_LOG(<a particular severity level>).  E.g.,
//
//   PA_LOG(INFO) << "Found " << num_cookies << " cookies";
//
// You can also do conditional logging:
//
//   PA_LOG_IF(INFO, num_cookies > 10) << "Got lots of cookies";
//
// The CHECK(condition) macro is active in both debug and release builds and
// effectively performs a PA_LOG(FATAL) which terminates the process and
// generates a crashdump unless a debugger is attached.
//
// There are also "debug mode" logging macros like the ones above:
//
//   PA_DLOG(INFO) << "Found cookies";
//
//   PA_DLOG_IF(INFO, num_cookies > 10) << "Got lots of cookies";
//
// All "debug mode" logging is compiled away to nothing for non-debug mode
// compiles.  PA_LOG_IF and development flags also work well together
// because the code can be compiled away sometimes.
//
// We also have
//
//   PA_LOG_ASSERT(assertion);
//   PA_DLOG_ASSERT(assertion);
//
// which is syntactic sugar for PA_{,D}LOG_IF(FATAL, assert fails) << assertion;
//
// There are "verbose level" logging macros.  They look like
//
//   PA_VLOG(1) << "I'm printed when you run the program with --v=1 or more";
//   PA_VLOG(2) << "I'm printed when you run the program with --v=2 or more";
//
// These always log at the INFO log level (when they log at all).
//
// There's also PA_VLOG_IS_ON(n) "verbose level" condition macro. To be used as
//
//   if (PA_VLOG_IS_ON(2)) {
//     // do some logging preparation and logging
//     // that can't be accomplished with just PA_VLOG(2) << ...;
//   }
//
// There is also a PA_VLOG_IF "verbose level" condition macro for sample
// cases, when some extra computation and preparation for logs is not
// needed.
//
//   PA_VLOG_IF(1, (size > 1024))
//      << "I'm printed when size is more than 1024 and when you run the "
//         "program with --v=1 or more";
//
// We also override the standard 'assert' to use 'PA_DLOG_ASSERT'.
//
// Lastly, there is:
//
//   PA_PLOG(ERROR) << "Couldn't do foo";
//   PA_DPLOG(ERROR) << "Couldn't do foo";
//   PA_PLOG_IF(ERROR, cond) << "Couldn't do foo";
//   PA_DPLOG_IF(ERROR, cond) << "Couldn't do foo";
//   PA_PCHECK(condition) << "Couldn't do foo";
//   PA_DPCHECK(condition) << "Couldn't do foo";
//
// which append the last system error to the message in string form (taken from
// GetLastError() on Windows and errno on POSIX).
//
// The supported severity levels for macros that allow you to specify one
// are (in increasing order of severity) INFO, WARNING, ERROR, and FATAL.
//
// Very important: logging a message at the FATAL severity level causes
// the program to terminate (after the message is logged).
//
// There is the special severity of DFATAL, which logs FATAL in DCHECK-enabled
// builds, ERROR in normal mode.
//
// Output is formatted as per the following example:
// [VERBOSE1:drm_device_handle.cc(90)] Succeeded
// authenticating /dev/dri/card0 in 0 ms with 1 attempt(s)
//
// The colon separated fields inside the brackets are as follows:
// 1. The log level
// 2. The filename and line number where the log was instantiated
//
// Additional logging-related information can be found here:
// https://chromium.googlesource.com/chromium/src/+/main/docs/linux/debugging.md#Logging

namespace partition_alloc::internal::logging {

// Sets the log level. Anything at or above this level will be written to the
// log file/displayed to the user (if applicable). Anything below this level
// will be silently ignored. The log level defaults to 0 (everything is logged
// up to level INFO) if this function is not called.
// Note that log messages for VLOG(x) are logged at level -x, so setting
// the min log level to negative values enables verbose logging.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) void SetMinLogLevel(int level);

// Gets the current log level.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) int GetMinLogLevel();

// Used by PA_LOG_IS_ON to lazy-evaluate stream arguments.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
bool ShouldCreateLogMessage(int severity);

// Gets the PA_VLOG default verbosity level.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) int GetVlogVerbosity();

// A few definitions of macros that don't generate much code. These are used
// by PA_LOG() and LOG_IF, etc. Since these are used all over our code, it's
// better to have compact code for these operations.
#define PA_COMPACT_GOOGLE_LOG_EX_INFO(ClassName)   \
  ::partition_alloc::internal::logging::ClassName( \
      __FILE__, __LINE__, ::partition_alloc::internal::logging::LOGGING_INFO)
#define PA_COMPACT_GOOGLE_PLOG_EX_INFO(ClassName, error_code)                 \
  ::partition_alloc::internal::logging::ClassName(                            \
      __FILE__, __LINE__, ::partition_alloc::internal::logging::LOGGING_INFO, \
      error_code)
#define PA_COMPACT_GOOGLE_LOG_EX_WARNING(ClassName) \
  ::partition_alloc::internal::logging::ClassName(  \
      __FILE__, __LINE__,                           \
      ::partition_alloc::internal::logging::LOGGING_WARNING)
#define PA_COMPACT_GOOGLE_PLOG_EX_WARNING(ClassName, error_code) \
  ::partition_alloc::internal::logging::ClassName(               \
      __FILE__, __LINE__,                                        \
      ::partition_alloc::internal::logging::LOGGING_WARNING)
#define PA_COMPACT_GOOGLE_LOG_EX_ERROR(ClassName)  \
  ::partition_alloc::internal::logging::ClassName( \
      __FILE__, __LINE__, ::partition_alloc::internal::logging::LOGGING_ERROR)
#define PA_COMPACT_GOOGLE_PLOG_EX_ERROR(ClassName, error_code)                 \
  ::partition_alloc::internal::logging::ClassName(                             \
      __FILE__, __LINE__, ::partition_alloc::internal::logging::LOGGING_ERROR, \
      error_code)
#define PA_COMPACT_GOOGLE_LOG_EX_FATAL(ClassName)  \
  ::partition_alloc::internal::logging::ClassName( \
      __FILE__, __LINE__, ::partition_alloc::internal::logging::LOGGING_FATAL)
#define PA_COMPACT_GOOGLE_PLOG_EX_FATAL(ClassName, error_code)                 \
  ::partition_alloc::internal::logging::ClassName(                             \
      __FILE__, __LINE__, ::partition_alloc::internal::logging::LOGGING_FATAL, \
      error_code)
#define PA_COMPACT_GOOGLE_LOG_EX_DFATAL(ClassName) \
  ::partition_alloc::internal::logging::ClassName( \
      __FILE__, __LINE__,                          \
      ::partition_alloc::internal::logging::LOGGING_DFATAL)
#define PA_COMPACT_GOOGLE_PLOG_EX_DFATAL(ClassName, error_code) \
  ::partition_alloc::internal::logging::ClassName(              \
      __FILE__, __LINE__,                                       \
      ::partition_alloc::internal::logging::LOGGING_DFATAL, error_code)
#define PA_COMPACT_GOOGLE_LOG_EX_DCHECK(ClassName) \
  ::partition_alloc::internal::logging::ClassName( \
      __FILE__, __LINE__,                          \
      ::partition_alloc::internal::logging::LOGGING_DCHECK)
#define PA_COMPACT_GOOGLE_PLOG_EX_DCHECK(ClassName, error_code) \
  ::partition_alloc::internal::logging::ClassName(              \
      __FILE__, __LINE__,                                       \
      ::partition_alloc::internal::logging::LOGGING_DCHECK, error_code)

#define PA_COMPACT_GOOGLE_LOG_INFO PA_COMPACT_GOOGLE_LOG_EX_INFO(LogMessage)
#define PA_COMPACT_GOOGLE_LOG_WARNING \
  PA_COMPACT_GOOGLE_LOG_EX_WARNING(LogMessage)
#define PA_COMPACT_GOOGLE_LOG_ERROR PA_COMPACT_GOOGLE_LOG_EX_ERROR(LogMessage)
#define PA_COMPACT_GOOGLE_LOG_FATAL PA_COMPACT_GOOGLE_LOG_EX_FATAL(LogMessage)
#define PA_COMPACT_GOOGLE_LOG_DFATAL PA_COMPACT_GOOGLE_LOG_EX_DFATAL(LogMessage)
#define PA_COMPACT_GOOGLE_LOG_DCHECK PA_COMPACT_GOOGLE_LOG_EX_DCHECK(LogMessage)

#if PA_BUILDFLAG(IS_WIN)
// wingdi.h defines ERROR to be 0. When we call PA_LOG(ERROR), it gets
// substituted with 0, and it expands to PA_COMPACT_GOOGLE_LOG_0. To allow us
// to keep using this syntax, we define this macro to do the same thing
// as PA_COMPACT_GOOGLE_LOG_ERROR, and also define ERROR the same way that
// the Windows SDK does for consistency.
#define PA_ERROR 0
#define PA_COMPACT_GOOGLE_LOG_EX_0(ClassName) \
  PA_COMPACT_GOOGLE_LOG_EX_ERROR(ClassName)
#define PA_COMPACT_GOOGLE_LOG_0 PA_COMPACT_GOOGLE_LOG_ERROR
// Needed for LOG_IS_ON(ERROR).
constexpr LogSeverity LOGGING_0 = LOGGING_ERROR;
#endif

// As special cases, we can assume that LOG_IS_ON(FATAL) always holds. Also,
// LOG_IS_ON(DFATAL) always holds in debug mode. In particular, CHECK()s will
// always fire if they fail.
#define PA_LOG_IS_ON(severity)                                   \
  (::partition_alloc::internal::logging::ShouldCreateLogMessage( \
      ::partition_alloc::internal::logging::LOGGING_##severity))

// We don't do any caching tricks with VLOG_IS_ON() like the
// google-glog version since it increases binary size.  This means
// that using the v-logging functions in conjunction with --vmodule
// may be slow.
#define PA_VLOG_IS_ON(verboselevel) \
  ((verboselevel) <= ::partition_alloc::internal::logging::GetVlogVerbosity())

// Helper macro which avoids evaluating the arguments to a stream if
// the condition doesn't hold. Condition is evaluated once and only once.
#define PA_LAZY_STREAM(stream, condition) \
  !(condition)                            \
      ? (void)0                           \
      : ::partition_alloc::internal::logging::LogMessageVoidify() & (stream)

// We use the preprocessor's merging operator, "##", so that, e.g.,
// PA_LOG(INFO) becomes the token PA_COMPACT_GOOGLE_LOG_INFO.  There's some
// funny subtle difference between ostream member streaming functions (e.g.,
// ostream::operator<<(int) and ostream non-member streaming functions
// (e.g., ::operator<<(ostream&, string&): it turns out that it's
// impossible to stream something like a string directly to an unnamed
// ostream. We employ a neat hack by calling the stream() member
// function of LogMessage which seems to avoid the problem.
#define PA_LOG_STREAM(severity) PA_COMPACT_GOOGLE_LOG_##severity.stream()

#define PA_LOG(severity) \
  PA_LAZY_STREAM(PA_LOG_STREAM(severity), PA_LOG_IS_ON(severity))
#define PA_LOG_IF(severity, condition) \
  PA_LAZY_STREAM(PA_LOG_STREAM(severity), PA_LOG_IS_ON(severity) && (condition))

// The VLOG macros log with negative verbosities.
#define PA_VLOG_STREAM(verbose_level)                                  \
  ::partition_alloc::internal::logging::LogMessage(__FILE__, __LINE__, \
                                                   -(verbose_level))   \
      .stream()

#define PA_VLOG(verbose_level) \
  PA_LAZY_STREAM(PA_VLOG_STREAM(verbose_level), PA_VLOG_IS_ON(verbose_level))

#define PA_VLOG_IF(verbose_level, condition)    \
  PA_LAZY_STREAM(PA_VLOG_STREAM(verbose_level), \
                 PA_VLOG_IS_ON(verbose_level) && (condition))

#if PA_BUILDFLAG(IS_WIN)
#define PA_VPLOG_STREAM(verbose_level)                                \
  ::partition_alloc::internal::logging::Win32ErrorLogMessage(         \
      __FILE__, __LINE__, -(verbose_level),                           \
      ::partition_alloc::internal::logging::GetLastSystemErrorCode()) \
      .stream()
#elif PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)
#define PA_VPLOG_STREAM(verbose_level)                                \
  ::partition_alloc::internal::logging::ErrnoLogMessage(              \
      __FILE__, __LINE__, -(verbose_level),                           \
      ::partition_alloc::internal::logging::GetLastSystemErrorCode()) \
      .stream()
#endif

#define PA_VPLOG(verbose_level) \
  PA_LAZY_STREAM(PA_VPLOG_STREAM(verbose_level), PA_VLOG_IS_ON(verbose_level))

#define PA_VPLOG_IF(verbose_level, condition)    \
  PA_LAZY_STREAM(PA_VPLOG_STREAM(verbose_level), \
                 PA_VLOG_IS_ON(verbose_level) && (condition))

// TODO(akalin): Add more VLOG variants, e.g. VPLOG.

#define PA_LOG_ASSERT(condition)                          \
  PA_LOG_IF(FATAL, !(PA_ANALYZER_ASSUME_TRUE(condition))) \
      << "Assert failed: " #condition ". "

#if PA_BUILDFLAG(IS_WIN)
#define PA_PLOG_STREAM(severity)                                      \
  PA_COMPACT_GOOGLE_PLOG_EX_##severity(                               \
      Win32ErrorLogMessage,                                           \
      ::partition_alloc::internal::logging::GetLastSystemErrorCode()) \
      .stream()
#elif PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)
#define PA_PLOG_STREAM(severity)                                      \
  PA_COMPACT_GOOGLE_PLOG_EX_##severity(                               \
      ErrnoLogMessage,                                                \
      ::partition_alloc::internal::logging::GetLastSystemErrorCode()) \
      .stream()
#endif

#define PA_PLOG(severity) \
  PA_LAZY_STREAM(PA_PLOG_STREAM(severity), PA_LOG_IS_ON(severity))

#define PA_PLOG_IF(severity, condition)    \
  PA_LAZY_STREAM(PA_PLOG_STREAM(severity), \
                 PA_LOG_IS_ON(severity) && (condition))

// Note that g_swallow_stream is used instead of an arbitrary PA_LOG() stream to
// avoid the creation of an object with a non-trivial destructor (LogMessage).
// On MSVC x86 (checked on 2015 Update 3), this causes a few additional
// pointless instructions to be emitted even at full optimization level, even
// though the : arm of the ternary operator is clearly never executed. Using a
// simpler object to be &'d with Voidify() avoids these extra instructions.
// Using a simpler POD object with a templated operator<< also works to avoid
// these instructions. However, this causes warnings on statically defined
// implementations of operator<<(std::ostream, ...) in some .cc files, because
// they become defined-but-unreferenced functions. A reinterpret_cast of 0 to an
// ostream* also is not suitable, because some compilers warn of undefined
// behavior.
#define PA_EAT_STREAM_PARAMETERS                                     \
  true ? (void)0                                                     \
       : ::partition_alloc::internal::logging::LogMessageVoidify() & \
             (*::partition_alloc::internal::logging::g_swallow_stream)

// Definitions for DLOG et al.

#if PA_BUILDFLAG(DCHECKS_ARE_ON)

#define PA_DLOG_IS_ON(severity) PA_LOG_IS_ON(severity)
#define PA_DLOG_IF(severity, condition) PA_LOG_IF(severity, condition)
#define PA_DLOG_ASSERT(condition) PA_LOG_ASSERT(condition)
#define PA_DPLOG_IF(severity, condition) PA_PLOG_IF(severity, condition)
#define PA_DVLOG_IF(verboselevel, condition) PA_VLOG_IF(verboselevel, condition)
#define PA_DVPLOG_IF(verboselevel, condition) \
  PA_VPLOG_IF(verboselevel, condition)

#else  // PA_BUILDFLAG(DCHECKS_ARE_ON)

// If !PA_BUILDFLAG(DCHECKS_ARE_ON), we want to avoid emitting any references
// to |condition| (which may reference a variable defined only if
// PA_BUILDFLAG(DCHECKS_ARE_ON)). Contrast this with DCHECK et al., which has
// different behavior.

#define PA_DLOG_IS_ON(severity) false
#define PA_DLOG_IF(severity, condition) PA_EAT_STREAM_PARAMETERS
#define PA_DLOG_ASSERT(condition) PA_EAT_STREAM_PARAMETERS
#define PA_DPLOG_IF(severity, condition) PA_EAT_STREAM_PARAMETERS
#define PA_DVLOG_IF(verboselevel, condition) PA_EAT_STREAM_PARAMETERS
#define PA_DVPLOG_IF(verboselevel, condition) PA_EAT_STREAM_PARAMETERS

#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

#define PA_DLOG(severity) \
  PA_LAZY_STREAM(PA_LOG_STREAM(severity), PA_DLOG_IS_ON(severity))

#define PA_DPLOG(severity) \
  PA_LAZY_STREAM(PA_PLOG_STREAM(severity), PA_DLOG_IS_ON(severity))

#define PA_DVLOG(verboselevel) PA_DVLOG_IF(verboselevel, true)

#define PA_DVPLOG(verboselevel) PA_DVPLOG_IF(verboselevel, true)

// Definitions for DCHECK et al.

#if PA_BUILDFLAG(DCHECK_IS_CONFIGURABLE)
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) extern LogSeverity LOGGING_DCHECK;
#else
constexpr LogSeverity LOGGING_DCHECK = LOGGING_FATAL;
#endif  // PA_BUILDFLAG(DCHECK_IS_CONFIGURABLE)

// Redefine the standard assert to use our nice log files
#undef assert
#define assert(x) PA_DLOG_ASSERT(x)

// Async signal safe logging mechanism.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
void RawLog(int level, const char* message);

#define PA_RAW_LOG(level, message)              \
  ::partition_alloc::internal::logging::RawLog( \
      ::partition_alloc::internal::logging::LOGGING_##level, message)

}  // namespace partition_alloc::internal::logging

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_LOGGING_H_
