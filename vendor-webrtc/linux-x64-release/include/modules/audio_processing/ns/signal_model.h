/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_NS_SIGNAL_MODEL_H_
#define MODULES_AUDIO_PROCESSING_NS_SIGNAL_MODEL_H_

#include <array>

#include "modules/audio_processing/ns/ns_common.h"

namespace webrtc {

struct SignalModel {
  SignalModel();
  SignalModel(const SignalModel&) = delete;
  SignalModel& operator=(const SignalModel&) = delete;

  float lrt;
  float spectral_diff;
  float spectral_flatness;
  // Log LRT factor with time-smoothing.
  std::array<float, kFftSizeBy2Plus1> avg_log_lrt;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_NS_SIGNAL_MODEL_H_
