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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_STABLELM_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_STABLELM_H_

#include <memory>
#include <utility>

#include "absl/status/statusor.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/llm.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/llm_weights.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/xnn_tensor.h"

namespace mediapipe::tasks::genai::xnn_utils {

class Stablelm4E1T3BBuilder : public LlmBuilder {
 public:
  using LlmBuilder::LlmBuilder;
  ~Stablelm4E1T3BBuilder() override = default;

 protected:
  // Overrides the default PreProcess with the following changes:
  // * Initialized `resource.segment_pos` with partial rope dimensions.
  // * Creates dummy `resource.segment_pos` that is not used.
  // * Skips token embedding scaling.
  absl::StatusOr<std::pair<std::shared_ptr<Tensor>, InputResource>> PreProcess(
      std::shared_ptr<Tensor> token_embedding, bool is_prefix) override;

  // Defines an alternative to `SelfAttentionExcludeNorm` defined in the base
  // class with the following changes:
  // * Replaces `Rope` with `PartialRope`.
  absl::StatusOr<std::shared_ptr<Tensor>> SelfAttentionExcludeNorm(
      std::shared_ptr<Tensor> input, InputResource resource,
      const LlmWeights::SelfAttentionWeights& sa_weights) override;
};

}  // namespace mediapipe::tasks::genai::xnn_utils

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_STABLELM_H_
