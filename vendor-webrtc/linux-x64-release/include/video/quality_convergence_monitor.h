/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_QUALITY_CONVERGENCE_MONITOR_H_
#define VIDEO_QUALITY_CONVERGENCE_MONITOR_H_

#include <deque>
#include <memory>

#include "api/field_trials_view.h"
#include "api/video/video_codec_type.h"

namespace webrtc {

class QualityConvergenceMonitor {
 public:
  struct Parameters {
    // Static QP threshold. No history or even refresh-frame requirements to
    // determine that target quality is reached if the QP value is at or below
    // this threshold.
    int static_qp_threshold = 0;

    // Determines if the dynamic threshold should be used for refresh frames.
    bool dynamic_detection_enabled = false;

    // Window lengths of QP values to use when determining if refresh frames
    // have reached the target quality. The combined window length is
    // `past_window_length` + `recent_window_length`. The recent part of the
    // window contains the most recent samples. Once the recent buffer reaches
    // this length, new samples will pop the oldest samples in recent and move
    // them to the past buffer. The average of `QP_past` must be equal to or
    // less than the average of `QP_recent` to determine that target quality is
    // reached. See the implementation in `AddSample()`.
    size_t recent_window_length = 0;
    size_t past_window_length = 0;

    // During dynamic detection, the average of `QP_past` must be less than or
    // equal to this threshold to determine that target quality is reached.
    int dynamic_qp_threshold = 0;
  };

  explicit QualityConvergenceMonitor(const Parameters& params);

  static std::unique_ptr<QualityConvergenceMonitor> Create(
      int static_qp_threshold,
      VideoCodecType codec,
      const FieldTrialsView& trials);

  // Add the supplied `qp` value to the detection window.
  // `is_refresh_frame` must only be `true` if the corresponding
  // video frame is a refresh frame that is used to improve the visual quality.
  void AddSample(int qp, bool is_refresh_frame);

  // Returns `true` if the algorithm has determined that the supplied QP values
  // have converged and reached the target quality.
  bool AtTargetQuality() const;

  // Used in tests to verify that default values and field trials are set
  // correctly.
  Parameters GetParametersForTesting() const { return params_; }

 private:
  const Parameters params_;
  bool at_target_quality_ = false;

  // Contains a window of QP values. New values are added at the back while old
  // values are popped from the front to maintain the configured window length.
  std::deque<int> qp_window_;
};

}  // namespace webrtc

#endif  // VIDEO_QUALITY_CONVERGENCE_MONITOR_H_
