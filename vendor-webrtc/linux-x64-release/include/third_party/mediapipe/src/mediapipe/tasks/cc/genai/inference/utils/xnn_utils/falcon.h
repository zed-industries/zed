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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_FALCON_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_FALCON_H_

#include <cstddef>
#include <memory>
#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/tasks/cc/genai/inference/common/mdspan.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/llm.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/llm_weights.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/xnn_tensor.h"

namespace mediapipe::tasks::genai::xnn_utils {

class FalconRW1BBuilder : public LlmBuilder {
 public:
  using LlmBuilder::LlmBuilder;
  ~FalconRW1BBuilder() override = default;

 protected:
  // Overrides the default PreProcess with the following changes:
  // * Fused Alibi and attention mask uses a 3-dimensional tensor.
  // * Creates dummy `segment_pos` and `pos_embedding` tensors.
  // * Skips token embedding scaling.
  absl::StatusOr<std::pair<std::shared_ptr<Tensor>, InputResource>> PreProcess(
      std::shared_ptr<Tensor> token_embedding, bool is_prefix) override;

  // Defines an alternative to `SelfAttentionExcludeNorm` defined in the base
  // class with the following changes:
  // * Does not use RoPE.
  // * 3D Alibi fused attention mask is transposed before being added to the
  //   logits. This is necessary to allow slicing mask during decoding.
  absl::StatusOr<std::shared_ptr<Tensor>> SelfAttentionExcludeNorm(
      std::shared_ptr<Tensor> input, InputResource resource,
      const LlmWeights::SelfAttentionWeights& sa_weights) override;

  // Defines an alternative to `FeedForwardExcludeNorm` defined in the base
  // class with the following changes:
  // * Vanilla feed forward network with sequential structure (as opposed to
  //   gated FFNs.)
  absl::StatusOr<std::shared_ptr<Tensor>> FeedForwardExcludeNorm(
      std::shared_ptr<Tensor> input,
      const LlmWeights::FeedForwardWeights& ff_weights) override;

  // Creates an Alibi fused attention mask.
  absl::Status InitAttentionMask(size_t current_seq_len, size_t process_seq_len,
                                 Tensor& out_attn_mask) override;

  absl::Status InitAlibiAttentionMaskValues();

  // Storing values of Alibi attention mask with shape [max_seq_len, num_heads,
  // max_seq_len]
  MdSpan<float, 3> attention_mask_values_;
};

}  // namespace mediapipe::tasks::genai::xnn_utils

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_FALCON_H_
