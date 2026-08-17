// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_WEBRTC_RTC_BASE_DIAGNOSTIC_LOGGING_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_WEBRTC_RTC_BASE_DIAGNOSTIC_LOGGING_H_

#include <sstream>
#include <string>

#include "third_party/abseil-cpp/absl/base/attributes.h"
#include "third_party/abseil-cpp/absl/strings/has_absl_stringify.h"
#include "third_party/abseil-cpp/absl/strings/has_ostream_operator.h"
#include "third_party/abseil-cpp/absl/strings/str_cat.h"
#include "third_party/webrtc/rtc_base/system/rtc_export.h"

namespace webrtc {

//////////////////////////////////////////////////////////////////////
// Note that the non-standard LoggingSeverity aliases exist because they are
// still in broad use.  The meanings of the levels are:
//  LS_SENSITIVE: Information which should only be logged with the consent
//   of the user, due to privacy concerns.
//  LS_VERBOSE: This level is for data which we do not want to appear in the
//   normal debug log, but should appear in diagnostic logs.
//  LS_INFO: Chatty level used in debugging for all sorts of things, the default
//   in debug builds.
//  LS_WARNING: Something that may warrant investigation.
//  LS_ERROR: Something that should not have occurred.
//  LS_NONE: Set this as minimum severity to disable logging.
// Note that LoggingSeverity is mapped over to chromiums verbosity levels where
// anything lower than or equal to the current verbosity level is written to
// file which is the opposite of logging severity in libjingle where higher
// severity numbers than or equal to the current severity level are written to
// file. Also, note that the values are explicitly defined here for convenience
// since the command line flag must be set using numerical values.
// TODO(tommi): To keep things simple, we should just use the same values for
// these constants as Chrome does.
enum LoggingSeverity {
  LS_NONE = 0,
  LS_ERROR = 1,
  LS_WARNING = 2,
  LS_INFO = 3,
  LS_VERBOSE = 4,
  LS_SENSITIVE = 5,
  INFO = LS_INFO,
  WARNING = LS_WARNING,
  LERROR = LS_ERROR
};

// LogErrorContext assists in interpreting the meaning of an error value.
enum LogErrorContext {
  ERRCTX_NONE,
  ERRCTX_ERRNO,     // System-local errno
  ERRCTX_HRESULT,   // Windows HRESULT
  ERRCTX_OSSTATUS,  // MacOS OSStatus

  // Abbreviations for LOG_E macro
  ERRCTX_EN = ERRCTX_ERRNO,     // LOG_E(sev, EN, x)
  ERRCTX_HR = ERRCTX_HRESULT,   // LOG_E(sev, HR, x)
  ERRCTX_OS = ERRCTX_OSSTATUS,  // LOG_E(sev, OS, x)
};

// Class that writes a log message to the logging delegate ("WebRTC logging
// stream" in Chrome) and to Chrome's logging stream.
class RTC_EXPORT DiagnosticLogMessage {
 public:
  template <typename T>
  ABSL_ATTRIBUTE_NOINLINE DiagnosticLogMessage& operator<<(const T& v) {
    if constexpr (absl::HasAbslStringify<T>::value) {
      print_stream_ << absl::StrCat(v);
    } else if constexpr (absl::HasOstreamOperator<T>::value) {
      print_stream_ << v;
    } else {
      static_assert(false, "Unsupported type to log");
    }
    return *this;
  }

  DiagnosticLogMessage(const char* file,
                       int line,
                       LoggingSeverity severity,
                       LogErrorContext err_ctx,
                       int err);
  DiagnosticLogMessage(const char* file,
                       int line,
                       LoggingSeverity severity,
                       LogErrorContext err_ctx,
                       int err,
                       const char* module);
  ~DiagnosticLogMessage();

  void CreateTimestamp();

  DiagnosticLogMessage& stream() { return *this; }

 private:
  const char* file_name_;
  const int line_;
  const LoggingSeverity severity_;
  const LogErrorContext err_ctx_;
  const int err_;
  const char* const module_;
  const bool log_to_chrome_;

  std::ostringstream print_stream_;
};

// This class is used to explicitly ignore values in the conditional
// logging macros.  This avoids compiler warnings like "value computed
// is not used" and "statement has no effect".
class LogMessageVoidify {
 public:
  LogMessageVoidify() {}
  // This has to be an operator with a precedence lower than << but
  // higher than ?:
  void operator&(DiagnosticLogMessage&) {}
};

//////////////////////////////////////////////////////////////////////
// Logging Helpers
//////////////////////////////////////////////////////////////////////

class LogMessage {
 public:
  static void LogToDebug(int min_sev);
};

// TODO(grunell): Change name to InitDiagnosticLoggingDelegate or
// InitDiagnosticLogging. Change also in init_webrtc.h/cc.
// TODO(grunell): typedef the delegate function.
RTC_EXPORT void InitDiagnosticLoggingDelegateFunction(
    void (*delegate)(const std::string&));

void SetExtraLoggingInit(
    void (*function)(void (*delegate)(const std::string&)));

}  // namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
namespace rtc {
using ::webrtc::DiagnosticLogMessage;
using ::webrtc::InitDiagnosticLoggingDelegateFunction;
using ::webrtc::LogMessage;
using ::webrtc::LogMessageVoidify;
using ::webrtc::LogErrorContext;
using enum ::webrtc::LogErrorContext;
using ::webrtc::LoggingSeverity;
using enum ::webrtc::LoggingSeverity;
using ::webrtc::SetExtraLoggingInit;
}  // namespace rtc

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_WEBRTC_RTC_BASE_DIAGNOSTIC_LOGGING_H_
