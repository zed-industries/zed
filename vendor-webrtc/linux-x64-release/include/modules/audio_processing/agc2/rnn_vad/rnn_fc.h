/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_RNN_FC_H_
#define MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_RNN_FC_H_

#include <array>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/function_view.h"
#include "modules/audio_processing/agc2/cpu_features.h"
#include "modules/audio_processing/agc2/rnn_vad/vector_math.h"

namespace webrtc {
namespace rnn_vad {

// Activation function for a neural network cell.
enum class ActivationFunction { kTansigApproximated, kSigmoidApproximated };

// Maximum number of units for an FC layer.
constexpr int kFullyConnectedLayerMaxUnits = 24;

// Fully-connected layer with a custom activation function which owns the output
// buffer.
class FullyConnectedLayer {
 public:
  // Ctor. `output_size` cannot be greater than `kFullyConnectedLayerMaxUnits`.
  FullyConnectedLayer(int input_size,
                      int output_size,
                      ArrayView<const int8_t> bias,
                      ArrayView<const int8_t> weights,
                      ActivationFunction activation_function,
                      const AvailableCpuFeatures& cpu_features,
                      absl::string_view layer_name);
  FullyConnectedLayer(const FullyConnectedLayer&) = delete;
  FullyConnectedLayer& operator=(const FullyConnectedLayer&) = delete;
  ~FullyConnectedLayer();

  // Returns the size of the input vector.
  int input_size() const { return input_size_; }
  // Returns the pointer to the first element of the output buffer.
  const float* data() const { return output_.data(); }
  // Returns the size of the output buffer.
  int size() const { return output_size_; }

  // Computes the fully-connected layer output.
  void ComputeOutput(ArrayView<const float> input);

 private:
  const int input_size_;
  const int output_size_;
  const std::vector<float> bias_;
  const std::vector<float> weights_;
  const VectorMath vector_math_;
  FunctionView<float(float)> activation_function_;
  // Over-allocated array with size equal to `output_size_`.
  std::array<float, kFullyConnectedLayerMaxUnits> output_;
};

}  // namespace rnn_vad
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_RNN_FC_H_
