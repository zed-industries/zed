/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_ABSOLUTE_CAPTURE_TIME_INTERPOLATOR_H_
#define MODULES_RTP_RTCP_SOURCE_ABSOLUTE_CAPTURE_TIME_INTERPOLATOR_H_

#include <cstdint>
#include <optional>

#include "api/array_view.h"
#include "api/rtp_headers.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {

//
// Helper class for interpolating the `AbsoluteCaptureTime` header extension.
//
// Supports the "timestamp interpolation" optimization:
//   A receiver SHOULD memorize the capture system (i.e. CSRC/SSRC), capture
//   timestamp, and RTP timestamp of the most recently received abs-capture-time
//   packet on each received stream. It can then use that information, in
//   combination with RTP timestamps of packets without abs-capture-time, to
//   extrapolate missing capture timestamps.
//
// See: https://webrtc.org/experiments/rtp-hdrext/abs-capture-time/
//
class AbsoluteCaptureTimeInterpolator {
 public:
  static constexpr TimeDelta kInterpolationMaxInterval = TimeDelta::Seconds(5);

  explicit AbsoluteCaptureTimeInterpolator(Clock* clock);

  // Returns the source (i.e. SSRC or CSRC) of the capture system.
  static uint32_t GetSource(uint32_t ssrc, ArrayView<const uint32_t> csrcs);

  // Returns a received header extension, an interpolated header extension, or
  // `std::nullopt` if it's not possible to interpolate a header extension.
  std::optional<AbsoluteCaptureTime> OnReceivePacket(
      uint32_t source,
      uint32_t rtp_timestamp,
      int rtp_clock_frequency_hz,
      const std::optional<AbsoluteCaptureTime>& received_extension);

 private:
  friend class AbsoluteCaptureTimeSender;

  static uint64_t InterpolateAbsoluteCaptureTimestamp(
      uint32_t rtp_timestamp,
      int rtp_clock_frequency_hz,
      uint32_t last_rtp_timestamp,
      uint64_t last_absolute_capture_timestamp);

  bool ShouldInterpolateExtension(Timestamp receive_time,
                                  uint32_t source,
                                  uint32_t rtp_timestamp,
                                  int rtp_clock_frequency_hz) const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  Clock* const clock_;

  Mutex mutex_;

  // Time of the last received header extension eligible for interpolation,
  // MinusInfinity() if no extension was received, or last received one is
  // not eligible for interpolation.
  Timestamp last_receive_time_ RTC_GUARDED_BY(mutex_) =
      Timestamp::MinusInfinity();

  uint32_t last_source_ RTC_GUARDED_BY(mutex_);
  uint32_t last_rtp_timestamp_ RTC_GUARDED_BY(mutex_);
  int last_rtp_clock_frequency_hz_ RTC_GUARDED_BY(mutex_);
  AbsoluteCaptureTime last_received_extension_ RTC_GUARDED_BY(mutex_);
  // Variables used for statistics generation
  std::optional<Timestamp> first_packet_time_;
  std::optional<Timestamp> first_offset_time_;
  std::optional<Timestamp> first_extension_time_;
  std::optional<TimeDelta> previous_capture_delta_;
  std::optional<TimeDelta> previous_offset_as_delta_;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_ABSOLUTE_CAPTURE_TIME_INTERPOLATOR_H_
