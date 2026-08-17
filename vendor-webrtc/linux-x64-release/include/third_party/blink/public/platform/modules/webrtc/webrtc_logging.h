// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_WEBRTC_WEBRTC_LOGGING_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_WEBRTC_WEBRTC_LOGGING_H_

#include <stdarg.h>

#include <string>

#include "base/compiler_specific.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

// This interface is implemented by a handler in the embedder and used for
// initializing the logging and passing log messages to the handler. The
// purpose is to forward mainly libjingle log messages to embedder (besides
// the ordinary logging stream) that will be used for diagnostic purposes.
class BLINK_PLATFORM_EXPORT WebRtcLogMessageDelegate {
 public:
  // Pass a diagnostic WebRTC log message.
  virtual void LogMessage(const std::string& message) = 0;

 protected:
  virtual ~WebRtcLogMessageDelegate() {}
};

// Must only be called once, and |delegate| must be non-null.
BLINK_PLATFORM_EXPORT void InitWebRtcLoggingDelegate(
    WebRtcLogMessageDelegate* delegate);

// Called to start the diagnostic WebRTC log.
BLINK_PLATFORM_EXPORT void InitWebRtcLogging();

// This function will add |message| to the diagnostic WebRTC log, if started.
// Otherwise it will be ignored. Note that this log may be uploaded to a
// server by the embedder - no sensitive information should be logged. May be
// called on any thread.
void BLINK_PLATFORM_EXPORT WebRtcLogMessage(const std::string& message);

// Helper methods which wraps calls to WebRtcLogMessage() using different
// printf-like inputs allowing the user to create log messages with a similar
// syntax as for printf format specifiers. These mathods use different versions
// of base::StringPrintf() internally and can also prepend the logged string
// with a |prefix| string and/or append a formated this pointer in |thiz|.

// Example: WebRtcLog("%s({foo=%d})", "Foo", 10) <=>
// WebRtcLogMessage("Foo({foo=10})")
PRINTF_FORMAT(1, 2)
void BLINK_PLATFORM_EXPORT WebRtcLog(const char* format, ...);

// Example: WebRtcLog(this, "%s({foo=%d})", "Foo", 10) <=>
// WebRtcLogMessage("Foo({foo=10}) [this=0x24514CB47A0]")
PRINTF_FORMAT(2, 3)
void BLINK_PLATFORM_EXPORT WebRtcLog(void* thiz, const char* format, ...);

// Example: WebRtcLog("RTC::", this, "%s({foo=%d})", "Foo", 10) <=>
// WebRtcLogMessage("RTC::Foo({foo=10}) [this=0x24514CB47A0]")
PRINTF_FORMAT(3, 4)
void BLINK_PLATFORM_EXPORT WebRtcLog(const char* prefix,
                                     void* thiz,
                                     const char* format,
                                     ...);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_WEBRTC_WEBRTC_LOGGING_H_
