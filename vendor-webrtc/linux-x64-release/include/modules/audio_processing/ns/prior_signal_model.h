/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_NS_PRIOR_SIGNAL_MODEL_H_
#define MODULES_AUDIO_PROCESSING_NS_PRIOR_SIGNAL_MODEL_H_

namespace webrtc {

// Struct for storing the prior signal model parameters.
struct PriorSignalModel {
  explicit PriorSignalModel(float lrt_initial_value);
  PriorSignalModel(const PriorSignalModel&) = delete;
  PriorSignalModel& operator=(const PriorSignalModel&) = delete;

  float lrt;
  float flatness_threshold = .5f;
  float template_diff_threshold = .5f;
  float lrt_weighting = 1.f;
  float flatness_weighting = 0.f;
  float difference_weighting = 0.f;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_NS_PRIOR_SIGNAL_MODEL_H_
