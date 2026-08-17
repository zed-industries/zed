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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_BENCHMARK_WEIGHT_ACCESSOR_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_BENCHMARK_WEIGHT_ACCESSOR_H_

#include <cstddef>
#include <memory>
#include <optional>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/xnn_tensor.h"
#include "xnnpack.h"  // from @XNNPACK

namespace mediapipe::tasks::genai {
namespace xnn_utils {

// Generate weights with some random value, according to given LlmParams.
class BenchmarkWeightAccessor : public WeightAccessor {
 public:
  // data_type is type of the weights, e.g. fp32, qc8 etc. data_type only
  // affects MLP linear weights, weights used in e.g. element-wise multiply are
  // always f32.
  explicit BenchmarkWeightAccessor(xnn_datatype data_type = xnn_datatype_fp32,
                                   std::optional<int> seed = std::nullopt)
      : data_type_(data_type), seed_(seed) {}

  // Return tensor with expected shape, filled with random data.
  absl::StatusOr<std::shared_ptr<Tensor>> LoadWeight(
      absl::string_view, Tensor::DimsType,
      size_t dim_scale_if_any) const override;
  // Return tensor with transposed shape, filled with random data.
  absl::StatusOr<std::shared_ptr<Tensor>> LoadTransposedWeight(
      absl::string_view, Tensor::DimsType,
      size_t dim_scale_if_any) const override;

 protected:
  xnn_datatype data_type_;
  std::optional<int> seed_;
};

// Generate mixed 4/8-bit weights. Following layers are 4-bit, otherwise default
// to 8-bit:
// * ff_layer.ffn_layer1
// * ff_layer.ffn_layer1_gate
// * ff_layer.ffn_layer2
// * softmax.logits_ffn
class BenchmarkMixedInt48WeightAccessor : public BenchmarkWeightAccessor {
 public:
  explicit BenchmarkMixedInt48WeightAccessor(
      std::optional<int> seed = std::nullopt)
      : BenchmarkWeightAccessor(xnn_datatype_qcint8, seed) {
    int4_weight_loader_ =
        std::make_unique<BenchmarkWeightAccessor>(xnn_datatype_qcint4, seed);
  }

  absl::StatusOr<std::shared_ptr<Tensor>> LoadWeight(
      absl::string_view, Tensor::DimsType,
      size_t dim_scale_if_any) const override;

 protected:
  std::unique_ptr<BenchmarkWeightAccessor> int4_weight_loader_;
};

}  // namespace xnn_utils
}  // namespace mediapipe::tasks::genai

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_BENCHMARK_WEIGHT_ACCESSOR_H_
