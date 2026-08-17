/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_ABSOLUTE_CAPTURE_TIME_SENDER_H_
#define MODULES_RTP_RTCP_SOURCE_ABSOLUTE_CAPTURE_TIME_SENDER_H_

#include <cstdint>
#include <optional>

#include "api/array_view.h"
#include "api/rtp_headers.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "system_wrappers/include/clock.h"
#include "system_wrappers/include/ntp_time.h"

namespace webrtc {

//
// Helper class for sending the `AbsoluteCaptureTime` header extension.
//
// Supports the "timestamp interpolation" optimization:
//   A sender SHOULD save bandwidth by not sending abs-capture-time with every
//   RTP packet. It SHOULD still send them at regular intervals (e.g. every
//   second) to help mitigate the impact of clock drift and packet loss. Mixers
//   SHOULD always send abs-capture-time with the first RTP packet after
//   changing capture system.
//
//   Timestamp interpolation works fine as long as there’s reasonably low
//   NTP/RTP clock drift. This is not always true. Senders that detect “jumps”
//   between its NTP and RTP clock mappings SHOULD send abs-capture-time with
//   the first RTP packet after such a thing happening.
//
// See: https://webrtc.org/experiments/rtp-hdrext/abs-capture-time/
//
class AbsoluteCaptureTimeSender {
 public:
  static constexpr TimeDelta kInterpolationMaxInterval = TimeDelta::Seconds(1);
  static constexpr TimeDelta kInterpolationMaxError = TimeDelta::Millis(1);

  explicit AbsoluteCaptureTimeSender(Clock* clock);

  // Returns the source (i.e. SSRC or CSRC) of the capture system.
  static uint32_t GetSource(uint32_t ssrc, ArrayView<const uint32_t> csrcs);

  // Returns value to write into AbsoluteCaptureTime RTP header extension to be
  // sent, or `std::nullopt` if the header extension shouldn't be attached to
  // the outgoing packet.
  //
  // - `source` - id of the capture system.
  // - `rtp_timestamp` - capture time represented as rtp timestamp in the
  // outgoing packet
  // - `rtp_clock_frequency_hz` - description of the `rtp_timestamp` units -
  // `rtp_timetamp` delta of `rtp_clock_freqnecy_hz` represents 1 second.
  // - `absolute_capture_time` - time when a frame was captured by the capture
  // system.
  // - `estimated_capture_clock_offset` - estimated offset between capture
  // system clock and local `clock` passed as the AbsoluteCaptureTimeSender
  // construction paramter. Uses the same units as `absolute_capture_time`,
  // i.e. delta of 2^32 represents 1 second. See AbsoluteCaptureTime type
  // comments for more details.
  // - `force` - when set to true, OnSendPacket is forced to return non-nullopt.
  std::optional<AbsoluteCaptureTime> OnSendPacket(
      uint32_t source,
      uint32_t rtp_timestamp,
      int rtp_clock_frequency_hz,
      NtpTime absolute_capture_time,
      std::optional<int64_t> estimated_capture_clock_offset,
      bool force = false);

  // Returns a header extension to be sent, or `std::nullopt` if the header
  // extension shouldn't be sent.
  [[deprecated]] std::optional<AbsoluteCaptureTime> OnSendPacket(
      uint32_t source,
      uint32_t rtp_timestamp,
      uint32_t rtp_clock_frequency,
      uint64_t absolute_capture_timestamp,
      std::optional<int64_t> estimated_capture_clock_offset);

 private:
  bool ShouldSendExtension(
      Timestamp send_time,
      uint32_t source,
      uint32_t rtp_timestamp,
      int rtp_clock_frequency_hz,
      NtpTime absolute_capture_time,
      std::optional<int64_t> estimated_capture_clock_offset) const;

  Clock* const clock_;

  Timestamp last_send_time_ = Timestamp::MinusInfinity();

  uint32_t last_source_;
  uint32_t last_rtp_timestamp_;
  int last_rtp_clock_frequency_hz_;
  NtpTime last_absolute_capture_time_;
  std::optional<int64_t> last_estimated_capture_clock_offset_;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_ABSOLUTE_CAPTURE_TIME_SENDER_H_
