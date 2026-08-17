// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_PORT_LOGGING_H_
#define MEDIAPIPE_PORT_LOGGING_H_

#include "absl/time/time.h"

#ifdef _WIN32
#define GLOG_NO_ABBREVIATED_SEVERITIES
#endif

#include <cstddef>
#include <ostream>
#include <type_traits>
#include <vector>

#include "absl/strings/has_ostream_operator.h"
#include "glog/logging.h"

namespace std {

template <typename... Ts, typename = std::enable_if_t<absl::HasOstreamOperator<
                              typename std::vector<Ts...>::value_type>::value>>
std::ostream& operator<<(std::ostream& out, const std::vector<Ts...>& v) {
  auto begin = v.begin();
  auto end = v.end();
  for (size_t i = 0; begin != end && i < 100; ++i, ++begin) {
    if (i) {
      out << " ";
    }
    out << *begin;
  }
  if (begin != end) {
    out << " ...";
  }
  return out;
}

}  // namespace std

namespace mediapipe {
using LogSeverity = google::LogSeverity;
const auto SetVLOGLevel = google::SetVLOGLevel;
class LogEntry {
 public:
  LogEntry(LogSeverity severity, const struct ::tm* tm_time,
           absl::string_view message)
      : severity_(severity),
        timestamp_(absl::FromTM(*tm_time, absl::LocalTimeZone())),
        text_message_(message) {}
  LogSeverity log_severity() const { return severity_; }
  absl::Time timestamp() const { return timestamp_; }
  absl::string_view text_message() const { return text_message_; }

 private:
  LogSeverity severity_;
  absl::Time timestamp_;
  absl::string_view text_message_;
};
class LogSink : public google::LogSink {
 public:
  virtual ~LogSink() = default;
  virtual void Send(const LogEntry& entry) = 0;
  virtual void WaitTillSent() {}

 private:
  virtual void send(LogSeverity severity, const char* full_filename,
                    const char* base_filename, int line,
                    const struct ::tm* tm_time, const char* message,
                    size_t message_len) {
    LogEntry log_entry(severity, tm_time,
                       absl::string_view(message, message_len));
    Send(log_entry);
  }
};
inline void AddLogSink(LogSink* destination) {
  google::AddLogSink(destination);
}
inline void RemoveLogSink(LogSink* destination) {
  google::RemoveLogSink(destination);
}
}  // namespace mediapipe

#endif  // MEDIAPIPE_PORT_LOGGING_H_
