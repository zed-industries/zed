/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_MIXER_GAIN_CHANGE_CALCULATOR_H_
#define MODULES_AUDIO_MIXER_GAIN_CHANGE_CALCULATOR_H_

#include <stdint.h>

#include "api/array_view.h"

namespace webrtc {

class GainChangeCalculator {
 public:
  // The 'out' signal is assumed to be produced from 'in' by applying
  // a smoothly varying gain. This method computes variations of the
  // gain and handles special cases when the samples are small.
  float CalculateGainChange(ArrayView<const int16_t> in,
                            ArrayView<const int16_t> out);

  float LatestGain() const;

 private:
  void CalculateGain(ArrayView<const int16_t> in,
                     ArrayView<const int16_t> out,
                     ArrayView<float> gain);

  float CalculateDifferences(ArrayView<const float> values);
  float last_value_ = 0.f;
  float last_reliable_gain_ = 1.0f;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_MIXER_GAIN_CHANGE_CALCULATOR_H_
