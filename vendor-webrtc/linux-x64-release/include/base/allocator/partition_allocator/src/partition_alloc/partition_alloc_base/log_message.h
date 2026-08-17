// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_LOG_MESSAGE_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_LOG_MESSAGE_H_

#include <cstddef>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/scoped_clear_last_error.h"
#include "partition_alloc/partition_alloc_base/strings/cstring_builder.h"

namespace partition_alloc::internal::logging {

// Sets the Log Message Handler that gets passed every log message before
// it's sent to other log destinations (if any).
// Returns true to signal that it handled the message and the message
// should not be sent to other log destinations.
typedef bool (*LogMessageHandlerFunction)(int severity,
                                          const char* file,
                                          int line,
                                          size_t message_start,
                                          const char* str);
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
void SetLogMessageHandler(LogMessageHandlerFunction handler);
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
LogMessageHandlerFunction GetLogMessageHandler();

using LogSeverity = int;
constexpr LogSeverity LOGGING_VERBOSE = -1;  // This is level 1 verbosity
// Note: the log severities are used to index into the array of names,
// see log_severity_names.
constexpr LogSeverity LOGGING_INFO = 0;
constexpr LogSeverity LOGGING_WARNING = 1;
constexpr LogSeverity LOGGING_ERROR = 2;
constexpr LogSeverity LOGGING_FATAL = 3;
constexpr LogSeverity LOGGING_NUM_SEVERITIES = 4;

// LOGGING_DFATAL is LOGGING_FATAL in DCHECK-enabled builds, ERROR in normal
// mode.
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
constexpr LogSeverity LOGGING_DFATAL = LOGGING_FATAL;
#else
constexpr LogSeverity LOGGING_DFATAL = LOGGING_ERROR;
#endif

PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
extern base::strings::CStringBuilder* g_swallow_stream;

// This class more or less represents a particular log message.  You
// create an instance of LogMessage and then stream stuff to it.
// When you finish streaming to it, ~LogMessage is called and the
// full message gets streamed to the appropriate destination.
//
// You shouldn't actually use LogMessage's constructor to log things,
// though.  You should use the PA_LOG() macro (and variants thereof)
// above.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) LogMessage {
 public:
  // Used for PA_LOG(severity).
  LogMessage(const char* file, int line, LogSeverity severity);

  // Used for CHECK().  Implied severity = LOGGING_FATAL.
  LogMessage(const char* file, int line, const char* condition);
  LogMessage(const LogMessage&) = delete;
  LogMessage& operator=(const LogMessage&) = delete;
  virtual ~LogMessage();

  base::strings::CStringBuilder& stream() { return stream_; }

  LogSeverity severity() { return severity_; }
  const char* c_str() { return stream_.c_str(); }

 private:
  void Init(const char* file, int line);

  const LogSeverity severity_;
  base::strings::CStringBuilder stream_;
  size_t message_start_;  // Offset of the start of the message (past prefix
                          // info).
  // The file and line information passed in to the constructor.
  const char* const file_;
  const int line_;

  // This is useful since the LogMessage class uses a lot of Win32 calls
  // that will lose the value of GLE and the code that called the log function
  // will have lost the thread error value when the log call returns.
  base::ScopedClearLastError last_error_;
};

// This class is used to explicitly ignore values in the conditional
// logging macros.  This avoids compiler warnings like "value computed
// is not used" and "statement has no effect".
class LogMessageVoidify {
 public:
  LogMessageVoidify() = default;
  // This has to be an operator with a precedence lower than << but
  // higher than ?:
  void operator&(base::strings::CStringBuilder&) {}
};

#if PA_BUILDFLAG(IS_WIN)
typedef unsigned long SystemErrorCode;
#elif PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)
typedef int SystemErrorCode;
#endif

// Alias for ::GetLastError() on Windows and errno on POSIX. Avoids having to
// pull in windows.h just for GetLastError() and DWORD.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
SystemErrorCode GetLastSystemErrorCode();

#if PA_BUILDFLAG(IS_WIN)
// Appends a formatted system message of the GetLastError() type.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) Win32ErrorLogMessage
    : public LogMessage {
 public:
  Win32ErrorLogMessage(const char* file,
                       int line,
                       LogSeverity severity,
                       SystemErrorCode err);
  Win32ErrorLogMessage(const Win32ErrorLogMessage&) = delete;
  Win32ErrorLogMessage& operator=(const Win32ErrorLogMessage&) = delete;
  // Appends the error message before destructing the encapsulated class.
  ~Win32ErrorLogMessage() override;

 private:
  SystemErrorCode err_;
};
#elif PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)
// Appends a formatted system message of the errno type
class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) ErrnoLogMessage
    : public LogMessage {
 public:
  ErrnoLogMessage(const char* file,
                  int line,
                  LogSeverity severity,
                  SystemErrorCode err);
  ErrnoLogMessage(const ErrnoLogMessage&) = delete;
  ErrnoLogMessage& operator=(const ErrnoLogMessage&) = delete;
  // Appends the error message before destructing the encapsulated class.
  ~ErrnoLogMessage() override;

 private:
  SystemErrorCode err_;
};
#endif  // PA_BUILDFLAG(IS_WIN)

}  // namespace partition_alloc::internal::logging

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_LOG_MESSAGE_H_
