// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_STANDALONE_BASE_LOGGING_H_
#define IPCZ_SRC_STANDALONE_BASE_LOGGING_H_

#include <ostream>
#include <sstream>

#include "third_party/abseil-cpp/absl/base/log_severity.h"

// This header defines a minimal polyfill for the LOG macros in Chromium //base,
// for use when NOT linking ipcz into Chromium.

#define LOG(level)                                                        \
  ::ipcz::standalone::LogMessage(                                         \
      __FILE__, __LINE__, ::ipcz::standalone::LogMessage::kLevel_##level) \
      .stream()

#ifdef NDEBUG
#define DLOG_IF(level, condition) \
  true ? (void)0 : ::ipcz::standalone::LogMessageVoidify() & LOG(level)
#define DLOG(level) DLOG_IF(level, true)
#define DVLOG(verbosity) DLOG_IF(INFO, true)
#else
#define DLOG_IF(level, condition) \
  !(condition) ? (void)0 : ::ipcz::standalone::LogMessageVoidify() & LOG(level)
#define DLOG(level) DLOG_IF(level, true)
#define DVLOG(verbosity) \
  DLOG_IF(INFO, ::ipcz::standalone::GetVerbosityLevel() >= verbosity)
#endif

#if defined(OS_WIN)
#ifndef ERROR
#define ERROR 0
#endif
#define kLevel_0 kLevel_ERROR
#endif

namespace ipcz::standalone {

class LogMessage {
 public:
  using Level = absl::LogSeverity;
  static constexpr Level kLevel_INFO = absl::LogSeverity::kInfo;
  static constexpr Level kLevel_WARNING = absl::LogSeverity::kWarning;
  static constexpr Level kLevel_ERROR = absl::LogSeverity::kError;
  static constexpr Level kLevel_FATAL = absl::LogSeverity::kFatal;

  LogMessage(const char* file, int line, Level level);
  ~LogMessage();

  std::ostream& stream() { return stream_; }

 private:
  std::stringstream stream_;
};

class LogMessageVoidify {
 public:
  LogMessageVoidify() = default;

  // Operator & is chosen because its precedence is lower than << but higher
  // than ?:. See usage in conditional log macros.
  void operator&(std::ostream&) {}
};

void SetVerbosityLevel(int level);
int GetVerbosityLevel();

}  // namespace ipcz::standalone

#endif  // IPCZ_SRC_STANDALONE_BASE_LOGGING_H_
