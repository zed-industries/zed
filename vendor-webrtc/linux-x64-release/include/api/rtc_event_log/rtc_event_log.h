/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_RTC_EVENT_LOG_RTC_EVENT_LOG_H_
#define API_RTC_EVENT_LOG_RTC_EVENT_LOG_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <memory>

#include "api/rtc_event_log/rtc_event.h"
#include "api/rtc_event_log_output.h"

namespace webrtc {

class RtcEventLog {
 public:
  enum : size_t { kUnlimitedOutput = 0 };
  enum : int64_t { kImmediateOutput = 0 };

  // TODO(eladalon):  Get rid of the legacy encoding and this enum once all
  // clients have migrated to the new format.
  enum class EncodingType { Legacy, NewFormat, ProtoFree };

  virtual ~RtcEventLog() = default;

  // Starts logging to a given output. The output might be limited in size,
  // and may close itself once it has reached the maximum size.
  virtual bool StartLogging(std::unique_ptr<RtcEventLogOutput> output,
                            int64_t output_period_ms) = 0;

  // Stops logging to file and waits until the file has been closed, after
  // which it would be permissible to read and/or modify it.
  virtual void StopLogging() = 0;

  // Stops logging to file and calls `callback` when the file has been closed.
  // Note that it is not safe to call any other members, including the
  // destructor, until the callback has been called.
  // TODO(srte): Remove default implementation when it's safe to do so.
  virtual void StopLogging(std::function<void()> callback) {
    StopLogging();
    callback();
  }

  // Log an RTC event (the type of event is determined by the subclass).
  virtual void Log(std::unique_ptr<RtcEvent> event) = 0;
};

// No-op implementation is used if flag is not set, or in tests.
class RtcEventLogNull final : public RtcEventLog {
 public:
  bool StartLogging(std::unique_ptr<RtcEventLogOutput> output,
                    int64_t output_period_ms) override;
  void StopLogging() override {}
  void Log(std::unique_ptr<RtcEvent> /* event */) override {}
};

}  // namespace webrtc

#endif  // API_RTC_EVENT_LOG_RTC_EVENT_LOG_H_
