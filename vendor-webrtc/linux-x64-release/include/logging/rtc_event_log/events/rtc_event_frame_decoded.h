/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_FRAME_DECODED_H_
#define LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_FRAME_DECODED_H_

#include <stdint.h>

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/rtc_event_log/rtc_event.h"
#include "api/units/timestamp.h"
#include "api/video/video_codec_type.h"
#include "logging/rtc_event_log/events/rtc_event_log_parse_status.h"

namespace webrtc {

struct LoggedFrameDecoded {
  int64_t log_time_us() const { return timestamp.us(); }
  int64_t log_time_ms() const { return timestamp.ms(); }
  Timestamp log_time() const { return timestamp; }

  Timestamp timestamp = Timestamp::MinusInfinity();
  int64_t render_time_ms;
  uint32_t ssrc;
  int width;
  int height;
  VideoCodecType codec;
  uint8_t qp;
};

class RtcEventFrameDecoded final : public RtcEvent {
 public:
  static constexpr Type kType = Type::FrameDecoded;

  RtcEventFrameDecoded(int64_t render_time_ms,
                       uint32_t ssrc,
                       int width,
                       int height,
                       VideoCodecType codec,
                       uint8_t qp);
  ~RtcEventFrameDecoded() override = default;

  Type GetType() const override { return kType; }
  bool IsConfigEvent() const override { return false; }

  std::unique_ptr<RtcEventFrameDecoded> Copy() const;

  int64_t render_time_ms() const { return render_time_ms_; }
  uint32_t ssrc() const { return ssrc_; }
  int width() const { return width_; }
  int height() const { return height_; }
  VideoCodecType codec() const { return codec_; }
  uint8_t qp() const { return qp_; }

  static std::string Encode(ArrayView<const RtcEvent*> /* batch */) {
    // TODO(terelius): Implement
    return "";
  }

  static RtcEventLogParseStatus Parse(
      absl::string_view /* encoded_bytes */,
      bool /* batched */,
      std::map<uint32_t, std::vector<LoggedFrameDecoded>>& /* output */) {
    // TODO(terelius): Implement
    return RtcEventLogParseStatus::Error("Not Implemented", __FILE__, __LINE__);
  }

 private:
  RtcEventFrameDecoded(const RtcEventFrameDecoded& other);

  const int64_t render_time_ms_;
  const uint32_t ssrc_;
  const int width_;
  const int height_;
  const VideoCodecType codec_;
  const uint8_t qp_;
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_FRAME_DECODED_H_
