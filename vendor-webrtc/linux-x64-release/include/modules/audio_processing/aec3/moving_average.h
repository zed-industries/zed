/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_MOVING_AVERAGE_H_
#define MODULES_AUDIO_PROCESSING_AEC3_MOVING_AVERAGE_H_

#include <stddef.h>

#include <vector>

#include "api/array_view.h"

namespace webrtc {
namespace aec3 {

class MovingAverage {
 public:
  // Creates an instance of MovingAverage that accepts inputs of length num_elem
  // and averages over mem_len inputs.
  MovingAverage(size_t num_elem, size_t mem_len);
  ~MovingAverage();

  // Computes the average of input and mem_len-1 previous inputs and stores the
  // result in output.
  void Average(ArrayView<const float> input, ArrayView<float> output);

 private:
  const size_t num_elem_;
  const size_t mem_len_;
  const float scaling_;
  std::vector<float> memory_;
  size_t mem_index_;
};

}  // namespace aec3
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_MOVING_AVERAGE_H_
