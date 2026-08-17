// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file overrides the logging macros in WebRTC (webrtc/rtc_base/logging.h).
// Instead of using WebRTC's logging implementation, the WebRTC macros are
// mapped to DIAGNOSTIC_LOGGING. In its implementation (DiagnosticLogMessage in
// third_party/webrtc_overrides/rtc_base/diagnostic_logging.h), the
// corresponding base/logging.h macros (e.g. Chromium's VLOG) are used.
// If this file is included outside of WebRTC/libjingle it should be included
// after base/logging.h (if any) or compiler error or unexpected behavior may
// occur (macros that have the same name in WebRTC as in Chromium will use
// the WebRTC definition if this file is included first).

// Setting the LoggingSeverity (and lower) that should be written to file should
// be done via command line by specifying the flags:
// --vmodule or --v please see base/logging.h for details on how to use them.
// Specifying what file to write to is done using InitLogging also in
// base/logging.h.

// The macros and classes declared in here are not described as they are
// NOT TO BE USED outside of WebRTC/libjingle.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_WEBRTC_RTC_BASE_LOGGING_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_WEBRTC_RTC_BASE_LOGGING_H_

#include <cstddef>

#include "third_party/webrtc_overrides/rtc_base/diagnostic_logging.h"

//////////////////////////////////////////////////////////////////////
// WebRTC macros which in DiagnosticLogMessage are mapped over to
// their VLOG equivalent in base/logging.h.
//////////////////////////////////////////////////////////////////////

#if defined(LOGGING_INSIDE_WEBRTC)

#include <errno.h>

namespace webrtc {

// Note that |N| is the size *with* the null terminator.
bool CheckVlogIsOnHelper(LoggingSeverity severity, const char* file, size_t N);

template <size_t N>
bool CheckVlogIsOn(LoggingSeverity severity, const char (&file)[N]) {
  return CheckVlogIsOnHelper(severity, file, N);
}

}  // namespace webrtc

// The LogMessageVoidify() call suppresses -Wunreachable-code diagnostics
// in certain conditions (see https://crbug.com/1065568)
// TODO(thakis): Make the warning smarter and then remove this again.
#define DIAGNOSTIC_LOG(sev, ctx, err, ...)                                   \
  webrtc::LogMessageVoidify() &                                              \
      webrtc::DiagnosticLogMessage(__FILE__, __LINE__, sev,                  \
                                   webrtc::ERRCTX_##ctx, err, ##__VA_ARGS__) \
          .stream()

#define RTC_LOG_CHECK_LEVEL(sev) CheckVlogIsOn(webrtc::sev, __FILE__)
#define RTC_LOG_CHECK_LEVEL_V(sev) CheckVlogIsOn(sev, __FILE__)

#define RTC_LOG_V(sev) DIAGNOSTIC_LOG(sev, NONE, 0)
#undef RTC_LOG
#define RTC_LOG(sev) DIAGNOSTIC_LOG(webrtc::sev, NONE, 0)
// Log if condition evaluates to true. DIAGNOSTIC_LOG comes second in the
// expression so users can properly stream log message to the logger.
#define RTC_LOG_IF(sev, condition) \
  !(condition) ? (void)0 : DIAGNOSTIC_LOG(webrtc::sev, NONE, 0)

// The _F version prefixes the message with the current function name.
#if defined(__GNUC__) && defined(_DEBUG)
#define RTC_LOG_F(sev) RTC_LOG(sev) << __PRETTY_FUNCTION__ << ": "
#define RTC_LOG_IF_F(sev, condition) \
  RTC_LOG_IF(sev, condition) << __PRETTY_FUNCTION__ << ": "
#else
#define RTC_LOG_F(sev) RTC_LOG(sev) << __FUNCTION__ << ": "
#define RTC_LOG_IF_F(sev, condition) \
  RTC_LOG_IF(sev, condition) << __FUNCTION__ << ": "
#endif

#define RTC_LOG_E(sev, ctx, err, ...) \
  DIAGNOSTIC_LOG(webrtc::sev, ctx, err, ##__VA_ARGS__)

#undef RTC_LOG_ERRNO_EX
#define RTC_LOG_ERRNO_EX(sev, err) RTC_LOG_E(sev, ERRNO, err)
#undef RTC_LOG_ERRNO
#define RTC_LOG_ERRNO(sev) RTC_LOG_ERRNO_EX(sev, errno)

#if defined(WEBRTC_WIN)
#define RTC_LOG_GLE_EX(sev, err) RTC_LOG_E(sev, HRESULT, err)
#define RTC_LOG_GLE(sev) RTC_LOG_GLE_EX(sev, GetLastError())
#define RTC_LOG_GLEM(sev, mod) RTC_LOG_E(sev, HRESULT, GetLastError(), mod)
#define RTC_LOG_ERR_EX(sev, err) RTC_LOG_GLE_EX(sev, err)
#define RTC_LOG_ERR(sev) RTC_LOG_GLE(sev)
#define RTC_LAST_SYSTEM_ERROR (::GetLastError())
#else
#define RTC_LOG_ERR_EX(sev, err) RTC_LOG_ERRNO_EX(sev, err)
#define RTC_LOG_ERR(sev) RTC_LOG_ERRNO(sev)
#define RTC_LAST_SYSTEM_ERROR (errno)
#endif  // OS_WIN

#undef RTC_PLOG
#define RTC_PLOG(sev, err) RTC_LOG_ERR_EX(sev, err)

// The RTC_DLOG macros are equivalent to their RTC_LOG counterparts except that
// they only generate code in debug builds.
#if defined(NDEBUG) && !defined(DCHECK_ALWAYS_ON)
#define RTC_DLOG_IS_ON 0
#else
#define RTC_DLOG_IS_ON 1
#endif

#if RTC_DLOG_IS_ON
#define RTC_DLOG(sev) RTC_LOG(sev)
#define RTC_DLOG_IF(sev, condition) RTC_LOG_IF(sev, condition)
#define RTC_DLOG_V(sev) RTC_LOG_V(sev)
#define RTC_DLOG_F(sev) RTC_LOG_F(sev)
#define RTC_DLOG_IF_F(sev, condition) RTC_LOG_IF_F(sev, condition)
#else
#define RTC_DLOG_EAT_STREAM_PARAMS(sev)                                   \
  (true ? true : ((void)(webrtc::sev), true))                             \
      ? static_cast<void>(0)                                              \
      : webrtc::LogMessageVoidify() &                                     \
            webrtc::DiagnosticLogMessage(__FILE__, __LINE__, webrtc::sev, \
                                         webrtc::ERRCTX_NONE, 0)          \
                .stream()
#define RTC_DLOG(sev) RTC_DLOG_EAT_STREAM_PARAMS(sev)
#define RTC_DLOG_IF(sev, condition) RTC_DLOG_EAT_STREAM_PARAMS(sev)
#define RTC_DLOG_V(sev) RTC_DLOG_EAT_STREAM_PARAMS(sev)
#define RTC_DLOG_F(sev) RTC_DLOG_EAT_STREAM_PARAMS(sev)
#define RTC_DLOG_IF_F(sev, condition) RTC_DLOG_EAT_STREAM_PARAMS(sev)
#endif

#endif  // LOGGING_INSIDE_WEBRTC

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_WEBRTC_RTC_BASE_LOGGING_H_
