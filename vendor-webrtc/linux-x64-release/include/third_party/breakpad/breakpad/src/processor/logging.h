// Copyright 2007 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// logging.h: Breakpad logging
//
// Breakpad itself uses Breakpad logging with statements of the form:
//   BPLOG(severity) << "message";
// severity may be INFO, ERROR, or other values defined in this file.
//
// BPLOG is an overridable macro so that users can customize Breakpad's
// logging.  Left at the default, logging messages are sent to stderr along
// with a timestamp and the source code location that produced a message.
// The streams may be changed by redefining BPLOG_*_STREAM, the logging
// behavior may be changed by redefining BPLOG_*, and the entire logging
// system may be overridden by redefining BPLOG(severity).  These
// redefinitions may be passed to the preprocessor as a command-line flag
// (-D).
//
// If an additional header is required to override Breakpad logging, it can
// be specified by the BP_LOGGING_INCLUDE macro.  If defined, this header
// will #include the header specified by that macro.
//
// If any initialization is needed before logging, it can be performed by
// a function called through the BPLOG_INIT macro.  Each main function of
// an executable program in the Breakpad processor library calls
// BPLOG_INIT(&argc, &argv); before any logging can be performed; define
// BPLOG_INIT appropriately if initialization is required.
//
// Author: Mark Mentovai

#ifndef PROCESSOR_LOGGING_H__
#define PROCESSOR_LOGGING_H__

#include <iostream>
#include <sstream>
#include <string>
#include <iomanip>

#include "common/using_std_string.h"
#include "google_breakpad/common/breakpad_types.h"

#ifdef BP_LOGGING_INCLUDE
#include BP_LOGGING_INCLUDE
#endif  // BP_LOGGING_INCLUDE

namespace google_breakpad {

// These are defined in Microsoft headers.
#ifdef SEVERITY_ERROR
#undef SEVERITY_ERROR
#endif

#ifdef ERROR
#undef ERROR
#endif

class LogStream {
 public:
  enum Severity {
    SEVERITY_INFO,
    SEVERITY_ERROR,
    SEVERITY_CRITICAL
  };

  // Begin logging a message to the stream identified by |stream|, at the
  // indicated severity.  The file and line parameters should be set so as to
  // identify the line of source code that is producing a message.
  LogStream(std::ostream& stream, Severity severity,
            const char* file, int line);

  // Finish logging by printing a newline and flushing the output stream.
  ~LogStream();

  template<typename T> std::ostream& operator<<(const T& t) {
    return stream_ << t;
  }

 private:
  std::ostream& stream_;

  // Disallow copy constructor and assignment operator
  explicit LogStream(const LogStream& that);
  void operator=(const LogStream& that);
};

// This class is used to explicitly ignore values in the conditional logging
// macros.  This avoids compiler warnings like "value computed is not used"
// and "statement has no effect".
class LogMessageVoidify {
 public:
  LogMessageVoidify() {}

  // This has to be an operator with a precedence lower than << but higher
  // than ?:
  void operator&(std::ostream&) {}
};

// Returns number formatted as a hexadecimal string, such as "0x7b".
template<typename T>
string HexString(T number) {
  std::stringstream stream;
  stream << "0x" << std::hex << number;
  return stream.str();
}

// Returns the error code as set in the global errno variable, and sets
// error_string, a required argument, to a string describing that error
// code.
int ErrnoString(string* error_string);

}  // namespace google_breakpad

#ifndef BPLOG_INIT
#define BPLOG_INIT(pargc, pargv)
#endif  // BPLOG_INIT

#ifndef BPLOG_LAZY_STREAM
#define BPLOG_LAZY_STREAM(stream, condition) \
    !(condition) ? (void) 0 : \
                   google_breakpad::LogMessageVoidify() & (BPLOG_ ## stream)
#endif

#ifndef BPLOG_MINIMUM_SEVERITY
#define BPLOG_MINIMUM_SEVERITY SEVERITY_INFO
#endif

#define BPLOG_LOG_IS_ON(severity) \
    ((google_breakpad::LogStream::SEVERITY_ ## severity) >= \
     (google_breakpad::LogStream::BPLOG_MINIMUM_SEVERITY))

#ifndef BPLOG
#define BPLOG(severity) BPLOG_LAZY_STREAM(severity, BPLOG_LOG_IS_ON(severity))
#endif  // BPLOG

#ifndef BPLOG_INFO
#ifndef BPLOG_INFO_STREAM
#define BPLOG_INFO_STREAM std::clog
#endif  // BPLOG_INFO_STREAM
#define BPLOG_INFO google_breakpad::LogStream(BPLOG_INFO_STREAM, \
                       google_breakpad::LogStream::SEVERITY_INFO, \
                       __FILE__, __LINE__)
#endif  // BPLOG_INFO

#ifndef BPLOG_ERROR
#ifndef BPLOG_ERROR_STREAM
#define BPLOG_ERROR_STREAM std::cerr
#endif  // BPLOG_ERROR_STREAM
#define BPLOG_ERROR google_breakpad::LogStream(BPLOG_ERROR_STREAM, \
                        google_breakpad::LogStream::SEVERITY_ERROR, \
                        __FILE__, __LINE__)
#endif  // BPLOG_ERROR

#ifndef BPLOG_CRITICAL
#ifndef BPLOG_CRITICAL_STREAM
#define BPLOG_CRITICAL_STREAM std::cerr
#endif  // BPLOG_CRITICAL_STREAM
#define BPLOG_CRITICAL google_breakpad::LogStream(BPLOG_CRITICAL_STREAM, \
                        google_breakpad::LogStream::SEVERITY_CRITICAL, \
                        __FILE__, __LINE__)
#endif  // BPLOG_CRITICAL

#ifndef BPLOG_IF
#define BPLOG_IF(severity, condition) \
    BPLOG_LAZY_STREAM(severity, ((condition) && BPLOG_LOG_IS_ON(severity)))
#endif  // BPLOG_IF

#endif  // PROCESSOR_LOGGING_H__
