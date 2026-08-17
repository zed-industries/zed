/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_TIMING_INTER_FRAME_DELAY_VARIATION_CALCULATOR_H_
#define MODULES_VIDEO_CODING_TIMING_INTER_FRAME_DELAY_VARIATION_CALCULATOR_H_

#include <stdint.h>

#include <optional>

#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "rtc_base/numerics/sequence_number_unwrapper.h"

namespace webrtc {

// This class calculates the inter-frame delay variation (see RFC5481) between
// the current frame (as supplied through the current call to `Calculate`) and
// the previous frame (as supplied through the previous call to `Calculate`).
class InterFrameDelayVariationCalculator {
 public:
  InterFrameDelayVariationCalculator();

  // Resets the calculator.
  void Reset();

  // Calculates the inter-frame delay variation of a frame with the given
  // RTP timestamp. This method is called when the frame is complete.
  std::optional<TimeDelta> Calculate(uint32_t rtp_timestamp, Timestamp now);

 private:
  // The previous wall clock timestamp used in the calculation.
  std::optional<Timestamp> prev_wall_clock_;
  // The previous RTP timestamp used in the calculation.
  int64_t prev_rtp_timestamp_unwrapped_;

  RtpTimestampUnwrapper unwrapper_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_TIMING_INTER_FRAME_DELAY_VARIATION_CALCULATOR_H_
