/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_STREAM_SYNCHRONIZATION_H_
#define VIDEO_STREAM_SYNCHRONIZATION_H_

#include <stdint.h>

#include "system_wrappers/include/rtp_to_ntp_estimator.h"

namespace webrtc {

class StreamSynchronization {
 public:
  struct Measurements {
    Measurements() : latest_receive_time_ms(0), latest_timestamp(0) {}
    RtpToNtpEstimator rtp_to_ntp;
    int64_t latest_receive_time_ms;
    uint32_t latest_timestamp;
  };

  StreamSynchronization(uint32_t video_stream_id, uint32_t audio_stream_id);

  bool ComputeDelays(int relative_delay_ms,
                     int current_audio_delay_ms,
                     int* total_audio_delay_target_ms,
                     int* total_video_delay_target_ms);

  // On success `relative_delay_ms` contains the number of milliseconds later
  // video is rendered relative audio. If audio is played back later than video
  // `relative_delay_ms` will be negative.
  static bool ComputeRelativeDelay(const Measurements& audio_measurement,
                                   const Measurements& video_measurement,
                                   int* relative_delay_ms);

  // Set target buffering delay. Audio and video will be delayed by at least
  // `target_delay_ms`.
  void SetTargetBufferingDelay(int target_delay_ms);

  // Lowers the audio delay by 10%. Can be used to recover from errors.
  void ReduceAudioDelay();

  // Lowers the video delay by 10%. Can be used to recover from errors.
  void ReduceVideoDelay();

  uint32_t audio_stream_id() const { return audio_stream_id_; }
  uint32_t video_stream_id() const { return video_stream_id_; }

 private:
  struct SynchronizationDelays {
    int extra_ms = 0;
    int last_ms = 0;
  };

  const uint32_t video_stream_id_;
  const uint32_t audio_stream_id_;
  SynchronizationDelays audio_delay_;
  SynchronizationDelays video_delay_;
  int base_target_delay_ms_;
  int avg_diff_ms_;
};
}  // namespace webrtc

#endif  // VIDEO_STREAM_SYNCHRONIZATION_H_
