/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_FRAME_DECODE_SCHEDULER_H_
#define VIDEO_FRAME_DECODE_SCHEDULER_H_

#include <stdint.h>

#include <optional>

#include "absl/functional/any_invocable.h"
#include "api/units/timestamp.h"
#include "video/frame_decode_timing.h"

namespace webrtc {

class FrameDecodeScheduler {
 public:
  // Invoked when a frame with `rtp_timestamp` is ready for decoding.
  using FrameReleaseCallback =
      absl::AnyInvocable<void(uint32_t rtp_timestamp,
                              Timestamp render_time) &&>;

  virtual ~FrameDecodeScheduler() = default;

  // Returns the rtp timestamp of the next frame scheduled for release, or
  // `nullopt` if no frame is currently scheduled.
  virtual std::optional<uint32_t> ScheduledRtpTimestamp() = 0;

  // Schedules a frame for release based on `schedule`. When released,
  // `callback` will be invoked with the `rtp` timestamp of the frame and the
  // `render_time`
  virtual void ScheduleFrame(uint32_t rtp,
                             FrameDecodeTiming::FrameSchedule schedule,
                             FrameReleaseCallback callback) = 0;

  // Cancels all scheduled frames.
  virtual void CancelOutstanding() = 0;

  // Stop() Must be called before destruction.
  virtual void Stop() = 0;
};

}  // namespace webrtc

#endif  // VIDEO_FRAME_DECODE_SCHEDULER_H_
