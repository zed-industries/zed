/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_LOGGING_LOG_WRITER_H_
#define TEST_LOGGING_LOG_WRITER_H_

#include <stdarg.h>

#include <memory>
#include <string>
#include <utility>

#include "absl/strings/string_view.h"
#include "api/rtc_event_log_output.h"
#include "rtc_base/strings/string_builder.h"

namespace webrtc {
template <class... Args>
inline void LogWriteFormat(RtcEventLogOutput* out_, const char* fmt, ...) {
  va_list args, copy;
  va_start(args, fmt);
  va_copy(copy, args);
  const int predicted_length = std::vsnprintf(nullptr, 0, fmt, copy);
  va_end(copy);

  RTC_DCHECK_GE(predicted_length, 0);
  std::string out_str(predicted_length, '\0');
  if (predicted_length > 0) {
    // Pass "+ 1" to vsnprintf to include space for the '\0'.
    const int actual_length =
        std::vsnprintf(&out_str.front(), predicted_length + 1, fmt, args);
    RTC_DCHECK_GE(actual_length, 0);
  }
  va_end(args);
  out_->Write(out_str);
}

class LogWriterFactoryInterface {
 public:
  virtual std::unique_ptr<RtcEventLogOutput> Create(
      absl::string_view filename) = 0;
  virtual ~LogWriterFactoryInterface() = default;
};

class LogWriterFactoryAddPrefix : public LogWriterFactoryInterface {
 public:
  LogWriterFactoryAddPrefix(LogWriterFactoryInterface* base,
                            absl::string_view prefix);
  std::unique_ptr<RtcEventLogOutput> Create(
      absl::string_view filename) override;

 private:
  LogWriterFactoryInterface* const base_factory_;
  const std::string prefix_;
};

}  // namespace webrtc

#endif  // TEST_LOGGING_LOG_WRITER_H_
