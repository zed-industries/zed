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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_TFLITE_WEIGHT_ACCESSOR_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_TFLITE_WEIGHT_ACCESSOR_H_

#include <cstddef>
#include <memory>

#include "absl/container/flat_hash_map.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/xnn_tensor.h"
#include "tensorflow/lite/schema/schema_generated.h"

namespace mediapipe::tasks::genai::xnn_utils {

// An implementation of `WeightAccessor` that loads static tensor from TfLite
// file.
class TfLiteWeightAccessor : public WeightAccessor {
 public:
  // `data` should point to the start of the file because TfLite relies on the
  // offset against `data` to access static tensors. TfLiteWeightAccessor keeps
  // `tflite_model` alive, and assumes `data` outlives `tflite_model`.
  TfLiteWeightAccessor(std::shared_ptr<const tflite::Model> tflite_model,
                       char* data);
  explicit TfLiteWeightAccessor(absl::string_view filename);
  ~TfLiteWeightAccessor() override = default;

  // Returns Tensor wrapping the data buffer from tflite model. Possible errors:
  // * NOT_FOUND: the given tensor_name cannot be found in model.
  absl::StatusOr<std::shared_ptr<Tensor>> LoadWeight(
      absl::string_view tensor_name, Tensor::DimsType expected_dims,
      size_t dim_scale_if_any) const override;

  // Similar to LoadWeight except the returning tensor has a transposed
  // `expected_dims`.
  absl::StatusOr<std::shared_ptr<Tensor>> LoadTransposedWeight(
      absl::string_view tensor_name, Tensor::DimsType expected_dims,
      size_t dim_scale_if_any) const override;

 private:
  void BuildWeightsMapFromTfliteModel(char* data);

  std::shared_ptr<const tflite::Model> tflite_model_;
  absl::flat_hash_map<absl::string_view /*tensor_name*/,
                      std::shared_ptr<Tensor>>
      weights_;
};

}  // namespace mediapipe::tasks::genai::xnn_utils

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_TFLITE_WEIGHT_ACCESSOR_H_
