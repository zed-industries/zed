// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUCHSIA_FUCHSIA_LOGGING_H_
#define BASE_FUCHSIA_FUCHSIA_LOGGING_H_

#include <lib/fidl/cpp/wire/connect_service.h>
#include <lib/fidl/cpp/wire/traits.h>
#include <lib/fit/function.h>
#include <lib/zx/result.h>
#include <zircon/types.h>

#include <string>
#include <string_view>

#include "base/base_export.h"
#include "base/check.h"
#include "base/logging.h"

// Use the ZX_LOG family of macros along with a zx_status_t containing a Zircon
// error. The error value will be decoded so that logged messages explain the
// error.

namespace logging {

class BASE_EXPORT ZxLogMessage : public logging::LogMessage {
 public:
  ZxLogMessage(const char* file_path,
               int line,
               LogSeverity severity,
               zx_status_t zx_status);

  ZxLogMessage(const ZxLogMessage&) = delete;
  ZxLogMessage& operator=(const ZxLogMessage&) = delete;

  ~ZxLogMessage() override;

 protected:
  void AppendError();

 private:
  zx_status_t zx_status_;
};

class BASE_EXPORT ZxLogMessageFatal final : public ZxLogMessage {
 public:
  using ZxLogMessage::ZxLogMessage;
  [[noreturn]] ~ZxLogMessageFatal() override;
};

}  // namespace logging

#define ZX_LOG_STREAM(severity, zx_status) \
  COMPACT_GOOGLE_LOG_EX_##severity(ZxLogMessage, zx_status).stream()

#define ZX_LOG(severity, zx_status) \
  LAZY_STREAM(ZX_LOG_STREAM(severity, zx_status), LOG_IS_ON(severity))
#define ZX_LOG_IF(severity, condition, zx_status) \
  LAZY_STREAM(ZX_LOG_STREAM(severity, zx_status), \
              LOG_IS_ON(severity) && (condition))

#define ZX_CHECK(condition, zx_status)                       \
  LAZY_STREAM(ZX_LOG_STREAM(FATAL, zx_status), !(condition)) \
      << "Check failed: " #condition << ". "

#define ZX_DLOG(severity, zx_status) \
  LAZY_STREAM(ZX_LOG_STREAM(severity, zx_status), DLOG_IS_ON(severity))

#if DCHECK_IS_ON()
#define ZX_DLOG_IF(severity, condition, zx_status) \
  LAZY_STREAM(ZX_LOG_STREAM(severity, zx_status),  \
              DLOG_IS_ON(severity) && (condition))
#else  // DCHECK_IS_ON()
#define ZX_DLOG_IF(severity, condition, zx_status) EAT_STREAM_PARAMETERS
#endif  // DCHECK_IS_ON()

#define ZX_DCHECK(condition, zx_status)         \
  LAZY_STREAM(ZX_LOG_STREAM(DFATAL, zx_status), \
              DCHECK_IS_ON() && !(condition))   \
      << "Check failed: " #condition << ". "

namespace base {

namespace internal {

BASE_EXPORT std::string FidlMethodResultErrorMessage(
    std::string_view formatted_error,
    std::string_view method_name);

BASE_EXPORT std::string FidlConnectionErrorMessage(
    std::string_view protocol_name,
    std::string_view status_string);

}  // namespace internal

class Location;

// Returns a function suitable for use as error-handler for a FIDL binding or
// helper (e.g. ScenicSession) required by the process to function. Typically
// it is unhelpful to simply crash on such failures, so the returned handler
// will instead log an ERROR and exit the process.
// The Location and protocol name string must be kept valid by the caller, for
// as long as the returned fit::function<> remains live.
BASE_EXPORT fit::function<void(zx_status_t)> LogFidlErrorAndExitProcess(
    const Location& from_here,
    std::string_view protocol_name);

template <typename Protocol>
BASE_EXPORT std::string FidlConnectionErrorMessage(
    const zx::result<fidl::ClientEnd<Protocol>>& result) {
  CHECK(result.is_error());
  return internal::FidlConnectionErrorMessage(
      fidl::DiscoverableProtocolName<Protocol>, result.status_string());
}

template <typename FidlMethod>
BASE_EXPORT std::string FidlMethodResultErrorMessage(
    const fidl::Result<FidlMethod>& result,
    std::string_view method_name) {
  CHECK(result.is_error());
  return internal::FidlMethodResultErrorMessage(
      result.error_value().FormatDescription(), method_name);
}

BASE_EXPORT std::string FidlMethodResultErrorMessage(
    const fit::result<fidl::OneWayError>& result,
    std::string_view method_name);

BASE_EXPORT fit::function<void(fidl::UnbindInfo)>
FidlBindingClosureWarningLogger(std::string_view protocol_name);

template <typename Protocol>
BASE_EXPORT fit::function<void(fidl::UnbindInfo)>
FidlBindingClosureWarningLogger() {
  return FidlBindingClosureWarningLogger(
      fidl::DiscoverableProtocolName<Protocol>);
}

}  // namespace base

#endif  // BASE_FUCHSIA_FUCHSIA_LOGGING_H_
