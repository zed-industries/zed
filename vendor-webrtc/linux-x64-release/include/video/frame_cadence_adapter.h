/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_FRAME_CADENCE_ADAPTER_H_
#define VIDEO_FRAME_CADENCE_ADAPTER_H_

#include <memory>

#include "absl/base/attributes.h"
#include "api/field_trials_view.h"
#include "api/metronome/metronome.h"
#include "api/task_queue/task_queue_base.h"
#include "api/units/time_delta.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {

// A sink adapter implementing mutations to the received frame cadence.
// With the exception of the constructor and the methods overridden in
// VideoSinkInterface, the rest of the interface to this class (including dtor)
// needs to happen on the queue passed in Create.
class FrameCadenceAdapterInterface : public VideoSinkInterface<VideoFrame> {
 public:
  // Averaging window spanning 90 frames at default 30fps, matching old media
  // optimization module defaults.
  // TODO(crbug.com/1255737): Use TimeDelta.
  static constexpr int64_t kFrameRateAveragingWindowSizeMs = (1000 / 30) * 90;
  // In zero-hertz mode, the idle repeat rate is a compromise between
  // RTP receiver keyframe-requesting timeout (3s), other backend limitations
  // and some worst case RTT.
  static constexpr TimeDelta kZeroHertzIdleRepeatRatePeriod =
      TimeDelta::Millis(1000);
  // The number of frame periods to wait for new frames until starting to
  // request refresh frames.
  static constexpr int kOnDiscardedFrameRefreshFramePeriod = 3;

  struct ZeroHertzModeParams {
    // The number of simulcast layers used in this configuration.
    size_t num_simulcast_layers = 0;
  };

  // Callback interface used to inform instance owners.
  class Callback {
   public:
    virtual ~Callback() = default;

    // Called when a frame arrives on the |queue| specified in Create.
    //
    // The |post_time| parameter indicates the current time sampled when
    // FrameCadenceAdapterInterface::OnFrame was called.
    //
    // |queue_overload| is true if the frame cadence adapter notices it's
    // not able to deliver the incoming |frame| to the |queue| in the expected
    // time.
    virtual void OnFrame(Timestamp post_time,
                         bool queue_overload,
                         const VideoFrame& frame) = 0;

    // Called when the source has discarded a frame.
    virtual void OnDiscardedFrame() = 0;

    // Called when the adapter needs the source to send a refresh frame.
    virtual void RequestRefreshFrame() = 0;
  };

  // Factory function creating a production instance. Deletion of the returned
  // instance needs to happen on the same sequence that Create() was called on.
  // Frames arriving in FrameCadenceAdapterInterface::OnFrame are posted to
  // Callback::OnFrame on the |queue|.
  static std::unique_ptr<FrameCadenceAdapterInterface> Create(
      Clock* clock,
      TaskQueueBase* queue,
      Metronome* metronome,
      TaskQueueBase* worker_queue,
      const FieldTrialsView& field_trials);

  // Call before using the rest of the API.
  virtual void Initialize(Callback* callback) = 0;

  // Pass zero hertz parameters in |params| as a prerequisite to enable
  // zero-hertz operation. If absl:::nullopt is passed, the cadence adapter will
  // switch to passthrough mode.
  virtual void SetZeroHertzModeEnabled(
      std::optional<ZeroHertzModeParams> params) = 0;

  // Returns the input framerate. This is measured by RateStatistics when
  // zero-hertz mode is off, and returns the max framerate in zero-hertz mode.
  virtual std::optional<uint32_t> GetInputFrameRateFps() = 0;

  // Updates quality convergence status for an enabled spatial layer.
  // Convergence means QP has dropped to a low-enough level to warrant ceasing
  // to send identical frames at high frequency.
  virtual void UpdateLayerQualityConvergence(size_t spatial_index,
                                             bool converged) = 0;

  // Updates spatial layer enabled status.
  virtual void UpdateLayerStatus(size_t spatial_index, bool enabled) = 0;

  // Updates the restrictions of max frame rate for the video source.
  // The new `max_frame_rate` will only affect the cadence of Callback::OnFrame
  // for non-idle (non converged) repeated frames.
  virtual void UpdateVideoSourceRestrictions(
      std::optional<double> max_frame_rate) = 0;

  // Conditionally requests a refresh frame via
  // Callback::RequestRefreshFrame.
  virtual void ProcessKeyFrameRequest() = 0;
};

}  // namespace webrtc

#endif  // VIDEO_FRAME_CADENCE_ADAPTER_H_
