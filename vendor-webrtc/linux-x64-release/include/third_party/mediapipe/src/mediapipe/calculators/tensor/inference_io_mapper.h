// Copyright 2024 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_INFERENCE_CALCULATOR_IO_MAP_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_INFERENCE_CALCULATOR_IO_MAP_H_

#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/calculators/tensor/inference_calculator.pb.h"
#include "mediapipe/calculators/tensor/tensor_span.h"
#include "mediapipe/framework/formats/tensor.h"
#include "mediapipe/util/tflite/tflite_signature_reader.h"
#include "tensorflow/lite/interpreter.h"
#include "tensorflow/lite/model_builder.h"

namespace mediapipe {
// Maps signature names to a list of input and output tensor names in the
// order in which they are expected by the model.
using InputOutputTensorNames =
    absl::flat_hash_map<SignatureName, SignatureInputOutputTensorNames>;

class InferenceIoMapper {
 public:
  // Extracts the input and output tensor names in the order they are expected
  // by the model from the provided interpreter. This method can be used by
  // InferenceCalculator implementations to initialize tensor name-based I/O
  // remapping.
  static absl::StatusOr<InputOutputTensorNames>
  GetInputOutputTensorNamesFromInterpreter(
      const tflite::Interpreter& interpreter);

  // Extracts the input and output tensor names in the order they are expected
  // by the model from the provided flatbuffer. This method can be used by
  // InferenceCalculator implementations to initialize tensor name-based I/O
  // remapping.
  static absl::StatusOr<InputOutputTensorNames>
  GetInputOutputTensorNamesFromModel(const tflite::FlatBufferModel& flatbuffer,
                                     const tflite::OpResolver& op_resolver);

  // Update the internal mapping of input and output tensors according to the
  // provided initialized tflite interpreter.
  absl::Status UpdateIoMap(
      const mediapipe::InferenceCalculatorOptions::InputOutputConfig& io_config,
      const InputOutputTensorNames& input_output_tensor_names);

  // Reorders input tensors according to the provided mappings.
  absl::StatusOr<TensorSpan> RemapInputTensors(
      const TensorSpan& unmapped_tensors);

  // Reorders output tensors according to the provided mappings.
  absl::StatusOr<std::vector<Tensor>> RemapOutputTensors(
      std::vector<Tensor>&& unmapped_tensors);

 private:
  int num_feedback_tensors_ = 0;
  std::vector<int> input_tensor_indices_;
  std::vector<int> output_tensor_indices_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_INFERENCE_CALCULATOR_IO_MAP_H_
