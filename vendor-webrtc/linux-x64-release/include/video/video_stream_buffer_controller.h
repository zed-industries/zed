/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_VIDEO_STREAM_BUFFER_CONTROLLER_H_
#define VIDEO_VIDEO_STREAM_BUFFER_CONTROLLER_H_

#include <memory>

#include "api/field_trials_view.h"
#include "api/task_queue/task_queue_base.h"
#include "api/video/encoded_frame.h"
#include "api/video/frame_buffer.h"
#include "modules/video_coding/include/video_coding_defines.h"
#include "modules/video_coding/timing/inter_frame_delay_variation_calculator.h"
#include "modules/video_coding/timing/jitter_estimator.h"
#include "modules/video_coding/timing/timing.h"
#include "system_wrappers/include/clock.h"
#include "video/decode_synchronizer.h"
#include "video/video_receive_stream_timeout_tracker.h"

namespace webrtc {

class FrameSchedulingReceiver {
 public:
  virtual ~FrameSchedulingReceiver() = default;

  virtual void OnEncodedFrame(std::unique_ptr<EncodedFrame> frame) = 0;
  virtual void OnDecodableFrameTimeout(TimeDelta wait_time) = 0;
};

class VideoStreamBufferControllerStatsObserver {
 public:
  virtual ~VideoStreamBufferControllerStatsObserver() = default;

  virtual void OnCompleteFrame(bool is_keyframe,
                               size_t size_bytes,
                               VideoContentType content_type) = 0;

  virtual void OnDroppedFrames(uint32_t frames_dropped) = 0;

  // `jitter_buffer_delay` is the delay experienced by a single frame,
  // whereas `target_delay` and `minimum_delay` are the current delays
  // applied by the jitter buffer.
  virtual void OnDecodableFrame(TimeDelta jitter_buffer_delay,
                                TimeDelta target_delay,
                                TimeDelta minimum_delay) = 0;

  // Various jitter buffer delays determined by VCMTiming.
  virtual void OnFrameBufferTimingsUpdated(int estimated_max_decode_time_ms,
                                           int current_delay_ms,
                                           int target_delay_ms,
                                           int jitter_delay_ms,
                                           int min_playout_delay_ms,
                                           int render_delay_ms) = 0;

  virtual void OnTimingFrameInfoUpdated(const TimingFrameInfo& info) = 0;
};

class VideoStreamBufferController {
 public:
  VideoStreamBufferController(
      Clock* clock,
      TaskQueueBase* worker_queue,
      VCMTiming* timing,
      VideoStreamBufferControllerStatsObserver* stats_proxy,
      FrameSchedulingReceiver* receiver,
      TimeDelta max_wait_for_keyframe,
      TimeDelta max_wait_for_frame,
      std::unique_ptr<FrameDecodeScheduler> frame_decode_scheduler,
      const FieldTrialsView& field_trials);
  virtual ~VideoStreamBufferController() = default;

  void Stop();
  void SetProtectionMode(VCMVideoProtection protection_mode);
  void Clear();
  std::optional<int64_t> InsertFrame(std::unique_ptr<EncodedFrame> frame);
  void UpdateRtt(int64_t max_rtt_ms);
  void SetMaxWaits(TimeDelta max_wait_for_keyframe,
                   TimeDelta max_wait_for_frame);
  void StartNextDecode(bool keyframe_required);
  int Size();

 private:
  void OnFrameReady(
      absl::InlinedVector<std::unique_ptr<EncodedFrame>, 4> frames,
      Timestamp render_time);
  void OnTimeout(TimeDelta delay);
  void FrameReadyForDecode(uint32_t rtp_timestamp, Timestamp render_time);
  void UpdateDroppedFrames() RTC_RUN_ON(&worker_sequence_checker_);
  void UpdateFrameBufferTimings(Timestamp min_receive_time, Timestamp now);
  void UpdateTimingFrameInfo();
  bool IsTooManyFramesQueued() const RTC_RUN_ON(&worker_sequence_checker_);
  void ForceKeyFrameReleaseImmediately() RTC_RUN_ON(&worker_sequence_checker_);
  void MaybeScheduleFrameForRelease() RTC_RUN_ON(&worker_sequence_checker_);

  RTC_NO_UNIQUE_ADDRESS SequenceChecker worker_sequence_checker_;
  const FieldTrialsView& field_trials_;
  Clock* const clock_;
  VideoStreamBufferControllerStatsObserver* const stats_proxy_;
  FrameSchedulingReceiver* const receiver_;
  VCMTiming* const timing_;
  const std::unique_ptr<FrameDecodeScheduler> frame_decode_scheduler_
      RTC_GUARDED_BY(&worker_sequence_checker_);

  JitterEstimator jitter_estimator_ RTC_GUARDED_BY(&worker_sequence_checker_);
  InterFrameDelayVariationCalculator ifdv_calculator_
      RTC_GUARDED_BY(&worker_sequence_checker_);
  bool keyframe_required_ RTC_GUARDED_BY(&worker_sequence_checker_) = false;
  std::unique_ptr<FrameBuffer> buffer_
      RTC_GUARDED_BY(&worker_sequence_checker_);
  FrameDecodeTiming decode_timing_ RTC_GUARDED_BY(&worker_sequence_checker_);
  VideoReceiveStreamTimeoutTracker timeout_tracker_
      RTC_GUARDED_BY(&worker_sequence_checker_);
  int frames_dropped_before_last_new_frame_
      RTC_GUARDED_BY(&worker_sequence_checker_) = 0;
  VCMVideoProtection protection_mode_
      RTC_GUARDED_BY(&worker_sequence_checker_) = kProtectionNack;

  // This flag guards frames from queuing in front of the decoder. Without this
  // guard, encoded frames will not wait for the decoder to finish decoding a
  // frame and just queue up, meaning frames will not be dropped or
  // fast-forwarded when the decoder is slow or hangs.
  bool decoder_ready_for_new_frame_ RTC_GUARDED_BY(&worker_sequence_checker_) =
      false;

  // Maximum number of frames in the decode queue to allow pacing. If the
  // queue grows beyond the max limit, pacing will be disabled and frames will
  // be pushed to the decoder as soon as possible. This only has an effect
  // when the low-latency rendering path is active, which is indicated by
  // the frame's render time == 0.
  FieldTrialParameter<unsigned> zero_playout_delay_max_decode_queue_size_;

  ScopedTaskSafety worker_safety_;
};

}  // namespace webrtc

#endif  // VIDEO_VIDEO_STREAM_BUFFER_CONTROLLER_H_
