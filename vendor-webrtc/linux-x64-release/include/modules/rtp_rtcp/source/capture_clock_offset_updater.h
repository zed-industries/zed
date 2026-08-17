/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_CAPTURE_CLOCK_OFFSET_UPDATER_H_
#define MODULES_RTP_RTCP_SOURCE_CAPTURE_CLOCK_OFFSET_UPDATER_H_

#include <stdint.h>

#include <optional>

#include "api/units/time_delta.h"

namespace webrtc {

//
// Helper class for calculating the clock offset against the capturer's clock.
//
// This is achieved by adjusting the estimated capture clock offset in received
// Absolute Capture Time RTP header extension (see
// https://webrtc.org/experiments/rtp-hdrext/abs-capture-time/), which
// represents the clock offset between a remote sender and the capturer, by
// adding local-to-remote clock offset.

class CaptureClockOffsetUpdater {
 public:
  // Adjusts remote_capture_clock_offset, which originates from Absolute Capture
  // Time RTP header extension, to get the local clock offset against the
  // capturer's clock.
  std::optional<int64_t> AdjustEstimatedCaptureClockOffset(
      std::optional<int64_t> remote_capture_clock_offset) const;

  // Sets the NTP clock offset between the sender system (which may be different
  // from the capture system) and the local system. This information is normally
  // provided by passing half the value of the Round-Trip Time estimation given
  // by RTCP sender reports (see DLSR/DLRR).
  //
  // Note that the value must be in Q32.32-formatted fixed-point seconds.
  void SetRemoteToLocalClockOffset(std::optional<int64_t> offset_q32x32);

  // Converts a signed Q32.32-formatted fixed-point to a TimeDelta.
  static std::optional<TimeDelta> ConvertsToTimeDela(
      std::optional<int64_t> q32x32);

 private:
  std::optional<int64_t> remote_to_local_clock_offset_;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_CAPTURE_CLOCK_OFFSET_UPDATER_H_
