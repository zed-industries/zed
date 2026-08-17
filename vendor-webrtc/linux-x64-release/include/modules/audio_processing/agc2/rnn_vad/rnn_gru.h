/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_RNN_GRU_H_
#define MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_RNN_GRU_H_

#include <array>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "modules/audio_processing/agc2/cpu_features.h"
#include "modules/audio_processing/agc2/rnn_vad/vector_math.h"

namespace webrtc {
namespace rnn_vad {

// Maximum number of units for a GRU layer.
constexpr int kGruLayerMaxUnits = 24;

// Recurrent layer with gated recurrent units (GRUs) with sigmoid and ReLU as
// activation functions for the update/reset and output gates respectively.
class GatedRecurrentLayer {
 public:
  // Ctor. `output_size` cannot be greater than `kGruLayerMaxUnits`.
  GatedRecurrentLayer(int input_size,
                      int output_size,
                      ArrayView<const int8_t> bias,
                      ArrayView<const int8_t> weights,
                      ArrayView<const int8_t> recurrent_weights,
                      const AvailableCpuFeatures& cpu_features,
                      absl::string_view layer_name);
  GatedRecurrentLayer(const GatedRecurrentLayer&) = delete;
  GatedRecurrentLayer& operator=(const GatedRecurrentLayer&) = delete;
  ~GatedRecurrentLayer();

  // Returns the size of the input vector.
  int input_size() const { return input_size_; }
  // Returns the pointer to the first element of the output buffer.
  const float* data() const { return state_.data(); }
  // Returns the size of the output buffer.
  int size() const { return output_size_; }

  // Resets the GRU state.
  void Reset();
  // Computes the recurrent layer output and updates the status.
  void ComputeOutput(ArrayView<const float> input);

 private:
  const int input_size_;
  const int output_size_;
  const std::vector<float> bias_;
  const std::vector<float> weights_;
  const std::vector<float> recurrent_weights_;
  const VectorMath vector_math_;
  // Over-allocated array with size equal to `output_size_`.
  std::array<float, kGruLayerMaxUnits> state_;
};

}  // namespace rnn_vad
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_RNN_GRU_H_
