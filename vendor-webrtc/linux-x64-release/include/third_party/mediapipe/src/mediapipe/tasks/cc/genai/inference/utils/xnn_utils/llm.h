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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_LLM_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_LLM_H_

#include <cstddef>
#include <memory>
#include <optional>
#include <utility>
#include <vector>

#include "absl/base/attributes.h"
#include "absl/base/nullability.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "mediapipe/tasks/cc/genai/inference/common/mdspan.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/graph_builder.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/llm_weights.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/sampling.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/xnn_tensor.h"

namespace mediapipe::tasks::genai {
namespace xnn_utils {

class LlmBuilder;

// The base class that hosts the XNNPACK graph for large language models. It is
// responsible for hosting the assets required to run the models, including
// pointers to the construct tensors, KV-cache, as well as constructing the
// whole models. Note that this class is designed to serve the models that share
// similar "structures" so please be mindful when you plan to inherit from it
// and perform customization. A general guideline is that if you are
// implementing a decode-only model with prefix/decode graphs, you shouldn't
// need to update the class but to perform the customization in the
// LlmGraphBuilder. For example:
//   1) to implement Llama, one should make the changes in the LlmGraphBuilder.
//   2) to implement an "encoder-only" model (i.e. only run the prefix graph
//      with no decode graphs and kv-cache), one should inherit from this and
//      update the logics. See llm_encoder_only.h for more details.
class Llm : protected xnn_utils::XnnGraph {
 public:
  explicit Llm(XnnGraph&& other) : XnnGraph(std::move(other)) {}
  Llm(Llm&&) = default;
  ~Llm() override = default;

  // Enable if enable_kv_cache
  struct KVCache {
    std::shared_ptr<Tensor> k_cache;
    std::shared_ptr<Tensor> v_cache;
    std::shared_ptr<Tensor> k_slice;
    std::shared_ptr<Tensor> v_slice;
  };

  // An aggregation of all the data that can represent the context of the
  // model.
  struct Context {
    // Previous ids, including prompt.
    std::vector<std::vector<int>> batch_prev_ids;
    std::vector<KVCache> kv_cache;
  };

  // Reduce the number of previous ids to effectively undo the last
  // `batch_num_tokens` tokens. Used for reverting incorrect draft tokens in
  // speculative decoding.
  static absl::Status ReduceContextPrevIds(std::shared_ptr<Context> context,
                                           std::vector<int> batch_num_tokens);

  // Create LLM graph using the `DefaultLlmWeightsLoader` to load model from
  // `weights_folder`.
  static absl::StatusOr<std::unique_ptr<Llm>> CreateLlm(
      absl::string_view weights_folder, const LlmParams& llm_params,
      std::unique_ptr<xnn_utils::RuntimeConfigs> runtime_configs = nullptr);

  // Create LLM graph using provided `weight_loader`, which provides LlmParams
  // through llm_params() and LlmWeights through LoadWeights(). This is
  // typically used when you would like to load weights from somewhere other
  // than filesystem (e.g. fake weights during benchmark):
  //
  // MP_ASSIGN_OR_RETURN(auto llm, CreateLlm(
  //     std::make_unique<BenchmarkLlmWeightsLoader>(llm_params)));
  static absl::StatusOr<std::unique_ptr<Llm>> CreateLlm(
      std::unique_ptr<LlmWeightsLoader> weight_loader,
      std::unique_ptr<xnn_utils::RuntimeConfigs> runtime_configs = nullptr);

  // Create LLM graph using provided `weight_loader` and `builder`.
  // `weight_loader` is used the same way as above version. This is typically
  // used when you would like to customize wiring logic of model construction
  // through `builder`:
  //
  // MP_ASSIGN_OR_RETURN(auto llm, CreateLlm(
  //     std::make_unique<LlmEncoderOnlyWeightsLoader>(llm_params),
  //     std::make_unique<LlmEncoderOnlyBuilder>(runtime_conigs)));
  static absl::StatusOr<std::unique_ptr<Llm>> CreateLlm(
      std::unique_ptr<LlmWeightsLoader> weight_loader,
      std::unique_ptr<LlmBuilder> builder);

  // Add input token ids at the end of all previously added tokens.
  virtual absl::Status AddInputTokens(
      absl::Span<const std::vector<int>> batch_input_ids);

  // Seeks to the given time step. This is typically used to go back to certain
  // status for speculative decoding. SeekTimeStep(0) is effectively resetting
  // the internal state.
  absl::Status SeekTimeStep(size_t time_step);

  // Samples the logits from ComputeLogits() and returns the sampled ids. This
  // also AddInputTokens() with the sampled ids.
  ABSL_DEPRECATED("Use ComputeLogits() and do your own sampling.")
  virtual absl::Status GetNextToken(std::vector<int>* output_ids);

  // Computes logits with all previously added tokens. Output is in shape of
  // [batch_B, expected_seq_len, vacab_size_V] representing the last
  // `expected_seq_len` along the sequence dimension.
  virtual absl::StatusOr<std::shared_ptr<Tensor>> ComputeLogits(
      size_t expected_seq_len);
  absl::StatusOr<std::shared_ptr<Tensor>> ComputeLogits() {
    return this->ComputeLogits(1);
  }

  // The size of all tokens, including prompt and generated tokens.
  virtual size_t TotalTokenSize() const;

  const LlmParams& GetLlmParams() { return llm_params_; }

  // Create a new context with internal model parameters. The variables in the
  // context will have proper batch size, sequence length, etc.
  virtual absl::StatusOr<Context> NewContext() const;

  // If `context` is non-null, and different from existing context_, load the
  // context into the model.
  virtual absl::Status LoadContext(
      absl::Nullable<std::shared_ptr<Context>> context);

  // protected:
  friend class PrefixDecodeLlm;
  friend class LlmTest;
  friend class LlmBuilder;

  Llm() : XnnGraph(XnnSubgraphPtr{nullptr, nullptr}, nullptr) {}

  // Internal parameters to control prefix model.
  struct InternalLlmParams {
    // Stops at last KV cache, so we don't waste computation.
    bool stop_at_last_kv_cache = false;
  };

  // Creates a `Llm` instance with prefix-decoder architecture.
  static absl::StatusOr<std::unique_ptr<Llm>> CreatePrefixDecodeLlm(
      LlmWeights, std::shared_ptr<LlmBuilder> builder);

  std::shared_ptr<Tensor>& transformer_input();
  const std::shared_ptr<Tensor>& transformer_input() const;
  std::shared_ptr<Tensor>& logits_output();
  const std::shared_ptr<Tensor>& logits_output() const;

  // Previous ids, including prompt.
  std::vector<std::vector<int>>& batch_prev_ids();
  const std::vector<std::vector<int>>& batch_prev_ids() const;
  std::vector<KVCache>& kv_cache();
  const std::vector<KVCache>& kv_cache() const;

  // Fill `embedding` according to given `ids`, by table lookup the token
  // embedding provided through weights. The first ids.size() * model_dim_D
  // elements pointed by `embedding` will be filled.
  absl::Status GetTokenEmbedding(const std::vector<int>& ids, float* embedding);
  virtual absl::Status GetInputTokenEmbeddings(
      absl::Span<const std::vector<int>> batch_input_ids);

  absl::Status ReshapeInputResource();

  LlmWeights weights_;
  LlmParams llm_params_;

  std::shared_ptr<Tensor> pos_embedding_;
  std::shared_ptr<Tensor> atten_masks_;
  std::shared_ptr<Tensor> segment_pos_;
  std::shared_ptr<Tensor> query_positions_;
  std::shared_ptr<Tensor> key_positions_;

  // Embedding input to the model.
  std::shared_ptr<Tensor> transformer_input_;
  // Logits output from the model.
  std::shared_ptr<Tensor> logits_output_;
  std::shared_ptr<Context> context_;

  // Hold a shared_ptr to the LlmBuilder for initializing the input resources
  // as well as performing necessary wiring customizations at decoding time.
  std::shared_ptr<LlmBuilder> builder_;
};

// Responsible for creating the high-level components that are required by large
// language models. The high-level components are:
//   1) PreProcess: including embedding lookup/attention mask/positional
//      embedding preparations..etc.
//   2) SelfAttentionIncludeResidual: the self-attention module along with
//      residual connections and some normalizations.
//   3) FeedForward: The feedforward layers that follows the
//      attention outputs, including residual connections and normalizations.
//   4) PostProcess: the final projection layer after the stacked transformers.
// The LlmBuilder allows developers to overwrite the logics of those components
// whenever needed (i.e. the existing Llm/LlmBuilder's configuration/settings
// don't capture the required changes).
class LlmBuilder : protected XnnGraphBuilder {
 public:
  // The following struct define the "resources" that are required by each
  // high-level modules. For clarification, even though most of the input/output
  // of those high-level modules are actually all "xnn_utils::Tensor", their
  // definitions are as the following:
  //   1) Weight: refers to the model weights which are static during
  //      initialization and runtime. For example:
  //      LlmWeights::FeedForwardWeights.
  //   2) Resource: the tensors that host the values which can be "precomputed"
  //      and remain reusable/fixed during inference (i.e. independent of the
  //      input values). For example: pos_embedding, atten_mask.
  //   3) Tensor: The data values that depends on the input data at the runtime.
  //      For example: the return value of PreProcess.
  struct InputResource {
    std::shared_ptr<Tensor> pos_embedding;
    std::shared_ptr<Tensor> atten_mask;
    std::shared_ptr<Tensor> segment_pos;
    std::shared_ptr<Tensor> query_positions;
    std::shared_ptr<Tensor> key_positions;

    // The type of this field will be updated in the future. Please contact
    // odml-llm-support if you'd like to use this field.
    Llm::KVCache* cache = nullptr;
  };

  explicit LlmBuilder(LlmParams llm_params,
                      std::unique_ptr<RuntimeConfigs> runtime_configs = nullptr,
                      xnn_datatype datatype = xnn_datatype_fp32)
      : LlmBuilder(llm_params, nullptr, std::move(runtime_configs), datatype) {}
  LlmBuilder(LlmParams llm_params, std::unique_ptr<Sampler> sampler,
             std::unique_ptr<RuntimeConfigs> runtime_configs = nullptr,
             xnn_datatype datatype = xnn_datatype_fp32)
      : XnnGraphBuilder(std::move(runtime_configs), datatype),
        llm_params_(llm_params),
        sampler_(std::move(sampler)) {}

  virtual std::unique_ptr<Llm> GetLlm(XnnGraph&& graph) {
    return std::make_unique<Llm>(std::move(graph));
  }

  using XnnGraphBuilder::Build;
  using XnnGraphBuilder::NewInput;

  // Apply pre-processing to the input before feeding to stacked transformers as
  // well as preparing the InputResource that will be used by other modules,
  // e.g. positional embedding.
  // `token_embedding` represents the token embedding ([batch_B, S,
  // model_dim_D], where S varies from 1 to seq_size_T).
  // `is_prefix` indicates whether this function is called by the prefix graph
  // as some resource preparation might be different between prefix vs. decode.
  virtual absl::StatusOr<std::pair<std::shared_ptr<Tensor>, InputResource>>
  PreProcess(std::shared_ptr<Tensor> token_embedding, bool is_prefix);

  // One transformer block consisting of self-attention and feedforward modules.
  // The default version builds a sequential SA and FF block. This can be
  // overwritten for fine-grained control over each OneStackTransformer.
  virtual absl::StatusOr<std::shared_ptr<Tensor>> OneStackTransformer(
      int layer_index, std::shared_ptr<Tensor> input, InputResource resource,
      const LlmWeights::SelfAttentionWeights& sa_weights,
      const LlmWeights::FeedForwardWeights& ff_weights, bool is_prefix);

  // Building blocks used within `DecoderLayer`.
  virtual absl::StatusOr<std::shared_ptr<Tensor>> SelfAttentionIncludeResidual(
      std::shared_ptr<Tensor> input, InputResource resource,
      const LlmWeights::SelfAttentionWeights& sa_weights);
  virtual absl::StatusOr<std::shared_ptr<Tensor>> FeedForward(
      std::shared_ptr<Tensor> input,
      const LlmWeights::FeedForwardWeights& ff_weights);
  virtual absl::StatusOr<std::shared_ptr<Tensor>> FeedForwardIncludeResidual(
      std::shared_ptr<Tensor> input,
      const LlmWeights::FeedForwardWeights& ff_weights);

  // Apply post-processing to the output of stacked transformers, e.g. final
  // norm, final projection, etc.
  virtual absl::StatusOr<std::shared_ptr<Tensor>> PostProcess(
      std::shared_ptr<Tensor> transformer_out, const LlmWeights& weights);

  // The following functions are related to the InputResource preparation and
  // handling.

  // Set the value of `out_attn_mask` given the condition that `current_seq_len`
  // number of tokens has been processed, and it's about to process
  // `process_seq_len` number of tokens.
  virtual absl::Status InitAttentionMask(size_t current_seq_len,
                                         size_t process_seq_len,
                                         Tensor& out_attn_mask);

  // Initialize the `out_pos_embedding` values given the condition that
  // `current_seq_len` number of tokens has been processed, and it's about to
  // process `process_seq_len` number of tokens.
  virtual absl::Status InitPosEmbedding(size_t current_seq_len,
                                        size_t process_seq_len,
                                        Tensor& out_pos_embedding);

  // Initialize the `out_segment_pos` values given the condition that
  // `current_seq_len` number of tokens has been processed, and it's about to
  // process `process_seq_len` number of tokens. E.g. in decoding mode, assume
  // 17 tokens have been processed, this function will be called with
  // `current_seq_len` to be 17, and `process_seq_len` to be 1 (decoding one
  // token). `out_segment_pos` will be reshaped to [process_seq_len, rope_size].
  virtual absl::Status InitSegmentPos(size_t current_seq_len,
                                      size_t process_seq_len,
                                      Tensor& out_segment_pos);

  absl::Status InitQueryPositions(size_t current_seq_len, size_t input_seq_len,
                                  Tensor& out_positions);
  absl::Status InitKeyPositions(size_t current_seq_len, size_t input_seq_len,
                                Tensor& out_positions);
  // Run sampling on model's output logits.
  absl::StatusOr<std::vector<std::vector<int>>> Sample(const Tensor& logits);

  // Apply normalization according to `norm_type`, generally the output tensor
  // should have the same shape as `input`.
  absl::StatusOr<std::shared_ptr<Tensor>> ApplyNorm(
      std::shared_ptr<Tensor> input,
      std::optional<LlmWeights::NormWeights> weights,
      LlmParams::Norm norm_type);

 protected:
  friend class Llm;
  friend class LlmBuilderTest;
  friend absl::StatusOr<std::unique_ptr<Llm>> Llm::CreatePrefixDecodeLlm(
      LlmWeights, std::shared_ptr<LlmBuilder>);

  absl::Status InitAttentionMaskValues(size_t process_seq_len);
  absl::Status InitPosEmbeddingValues(size_t process_seq_len);
  absl::Status InitSegmentPosValues(size_t rope_size);

  absl::StatusOr<std::shared_ptr<Tensor>> ScaleQuery(
      std::shared_ptr<Tensor> query_proj,
      const LlmWeights::SelfAttentionWeights& sa_weights);

  absl::StatusOr<std::shared_ptr<Tensor>> DotAttention(
      std::shared_ptr<Tensor> query_proj, std::shared_ptr<Tensor> key_proj,
      std::shared_ptr<Tensor> value_proj, std::shared_ptr<Tensor> atten_mask,
      const LlmWeights::SelfAttentionWeights& sa_weights);

  virtual absl::StatusOr<std::shared_ptr<Tensor>> SelfAttentionExcludeNorm(
      std::shared_ptr<Tensor> input, InputResource resource,
      const LlmWeights::SelfAttentionWeights& sa_weights);

  virtual absl::StatusOr<std::shared_ptr<Tensor>> FeedForwardExcludeNorm(
      std::shared_ptr<Tensor> input,
      const LlmWeights::FeedForwardWeights& ff_weights);

  absl::Status BuildKVCache(std::shared_ptr<Tensor>& key,
                            std::shared_ptr<Tensor>& value,
                            InputResource& resource);

  LlmParams llm_params_;
  Llm::InternalLlmParams internal_llm_params_;

  // Storing values of attention mask with shape [max_seq_len, max_seq_len]
  MdSpan<float, 2> attention_mask_values_;
  // Storing values of positional embedding with shape [max_seq_len,
  // model_dimension]
  std::shared_ptr<std::vector<float>> position_embedding_values_;
  // Storing values of segment pos with shape [max_seq_len, head_dimension]
  MdSpan<float, 2> segment_pos_values_;

  std::unique_ptr<Sampler> sampler_;
};

}  // namespace xnn_utils
}  // namespace mediapipe::tasks::genai

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_LLM_H_
