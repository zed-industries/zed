/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_QUALITY_CONVERGENCE_CONTROLLER_H_
#define VIDEO_QUALITY_CONVERGENCE_CONTROLLER_H_

#include <memory>
#include <optional>
#include <vector>

#include "api/field_trials_view.h"
#include "api/sequence_checker.h"
#include "api/video/video_codec_type.h"
#include "video/quality_convergence_monitor.h"

namespace webrtc {

class QualityConvergenceController {
 public:
  void Initialize(int number_of_layers,
                  std::optional<int> static_qp_threshold,
                  VideoCodecType codec,
                  const FieldTrialsView& trials);

  // Add the supplied `qp` value to the detection window for specified layer.
  // `is_refresh_frame` must only be `true` if the corresponding
  // video frame is a refresh frame that is used to improve the visual quality.
  // Returns `true` if the algorithm has determined that the supplied QP values
  // have converged and reached the target quality for this layer.
  bool AddSampleAndCheckTargetQuality(int layer_index,
                                      int qp,
                                      bool is_refresh_frame);

 private:
  bool initialized_ = false;
  int number_of_layers_ = 0;
  std::vector<std::unique_ptr<QualityConvergenceMonitor>> convergence_monitors_;
  SequenceChecker sequence_checker_{SequenceChecker::kDetached};
};

}  // namespace webrtc

#endif  // VIDEO_QUALITY_CONVERGENCE_CONTROLLER_H_
