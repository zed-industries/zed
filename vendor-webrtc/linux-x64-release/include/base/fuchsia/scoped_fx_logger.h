// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUCHSIA_SCOPED_FX_LOGGER_H_
#define BASE_FUCHSIA_SCOPED_FX_LOGGER_H_

#include <fidl/fuchsia.logger/cpp/fidl.h>
#include <lib/syslog/structured_backend/cpp/fuchsia_syslog.h>
#include <lib/zx/socket.h>
#include <stdint.h>

#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/logging.h"

namespace base {

// Emits log lines to a logger created via the specified LogSink.
// This class is thread-safe.
class BASE_EXPORT ScopedFxLogger {
 public:
  ScopedFxLogger();
  ~ScopedFxLogger();

  ScopedFxLogger(ScopedFxLogger&& other);
  ScopedFxLogger& operator=(ScopedFxLogger&& other);

  // Returns an instance connected to the process' incoming LogSink service.
  // The returned instance has a single tag attributing the calling process in
  // some way (e.g. by Component or process name).
  // Additional tags may optionally be specified via `tags`.
  static ScopedFxLogger CreateForProcess(
      std::vector<std::string_view> tags = {});

  // Returns an instance connected to the specified LogSink.
  static ScopedFxLogger CreateFromLogSink(
      fidl::ClientEnd<fuchsia_logger::LogSink> client_end,
      std::vector<std::string_view> tags = {});

  void LogMessage(std::string_view file,
                  uint32_t line_number,
                  std::string_view msg,
                  logging::LogSeverity severity);

  bool is_valid() const { return socket_.is_valid(); }

 private:
  ScopedFxLogger(std::vector<std::string_view> tags, zx::socket socket);

  // For thread-safety these members should be treated as read-only.
  // They are non-const only to allow move-assignment of ScopedFxLogger.
  std::vector<std::string> tags_;
  zx::socket socket_;
};

}  // namespace base

#endif  // BASE_FUCHSIA_SCOPED_FX_LOGGER_H_
