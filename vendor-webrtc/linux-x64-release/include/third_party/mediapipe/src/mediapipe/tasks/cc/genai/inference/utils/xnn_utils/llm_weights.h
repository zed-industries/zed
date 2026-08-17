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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_LLM_WEIGHTS_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_LLM_WEIGHTS_H_

#include <cstddef>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "absl/base/attributes.h"
#include "absl/container/flat_hash_map.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "mediapipe/tasks/cc/genai/inference/proto/llm_params.pb.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/pack_weights_cache.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/xnn_tensor.h"
#include "tensorflow/lite/model_builder.h"

namespace mediapipe::tasks::genai::xnn_utils {

struct LlmParams {
  // Construct LlmParams from proto.
  static LlmParams FromLLMParametersProto(
      const odml::infra::proto::LlmParameters& llm_params);

  size_t num_transformer_M = 0;
  size_t batch_size_B = 0;
  size_t seq_size_T = 0;
  size_t model_dim_D = 0;
  size_t hidden_dim_HD = 0;
  size_t head_dim_H = 0;
  size_t n_heads_N = 0;
  size_t voc_size_V = 0;
  size_t draft_size_G = 0;
  float query_rescale_factor = 1.f;

  // Number of kv heads. In case of Multi-Head-Attention (MHA), num_kv_heads is
  // the same as n_heads_N, which is number of query heads; In case of
  // Multi-Query-Attention (MQA), key and value have one head; otherwise, this
  // specifies the number of heads for key and value, and
  // Grouped-Query-Attention (GQA) will be used. See
  // https://arxiv.org/pdf/2305.13245.pdf for details.
  size_t num_kv_heads = 0;

  // Meant to be a mapping of pax LanguageModelType. This will affect e.g.
  // attention mask shape.
  enum class ModelType {
    UNSPECIFIED = 0,
    // Attention mask for input are prefixed to be bidirectional.
    PREFIX = 1,
    // Attention mask are forward only.
    CAUSAL = 2,
  } model_type = ModelType::CAUSAL;

  enum class Activation {
    UNSPECIFIED = 0,
    // Gaussian Error Linear Unit.
    GELU = 1,
    // Sigmoid-Weighted Linear Unit.
    SILU = 2,
    // Rectified Linear Unit.
    RELU = 3,
    // Rectified Linear Unit 1p5
    RELU1P5 = 4,
  };

  enum class Norm {
    UNSPECIFIED = 0,
    NO_NORM = 1,
    RMS_NORM = 2,
    LAYER_NORM = 3,
  };

  enum class AttentionScaleType {
    UNSPECIFIED = 0,
    // Per dimension scale, query is scaled by log_2(1 + exp(w)) /
    // sqrt(head_dim) where w is s static weight.
    PER_DIM_SCALE = 1,
    // Query is scaled by 1/sqrt(head_dim).
    INV_SQRT_HEAD_DIM = 2,
    // Query is scaled by rescale_factor / head_dim.
    RESCALE_FACTOR_INV_HEAD_DIM = 3,
  };

  // If false, add absolute positional embeddings.
  bool skip_absolute_positional_embeddings = false;

  struct SelfAttentionParams {
    bool qkv_no_bias = false;
    bool post_proj_no_bias = false;
    Norm pre_norm = Norm::RMS_NORM;
    Norm post_norm = Norm::RMS_NORM;

    // If greater than 0, CapTanh will be applied. Otherwise, no cap will be
    // applied.
    float soft_cap_value = 0.0f;

    // Attention scale type to be applied within the transformer.
    AttentionScaleType attention_scale_type;
  } sa_params;

  struct FeedForwardParams {
    // If `no_bias`, fully connect will degrade to matrix multiply.
    bool no_bias = false;
    Activation activation = Activation::GELU;
    Norm pre_norm = Norm::RMS_NORM;
    Norm post_norm = Norm::RMS_NORM;
  } ff_params;

  Norm final_norm = Norm::RMS_NORM;

  struct FinalProjectParams {
    // If `no_bias`, final fully connect will degrade to matrix multiply.
    bool no_bias = false;
    float soft_cap_value = 0.0f;
  } final_proj_params;

  /*
   * Parameters below do NOT change the "correctness" of the model, they
   * configure the acceleration of inference.
   */

  bool enable_kv_cache = false;
  // If true, inference engine will optimize tensor shape according to current
  // sequence length to avoid computation waste.
  bool enable_dynamic_shape ABSL_DEPRECATED(
      "This is always enabled if enable_kv_cache is true.") = false;

  // If provided, the runtime will prepare cache at the provided directory.
  // Otherwise, cache will be prepared besides the original model.
  std::string cache_dir;
};

struct RMSNormWeights {
  std::shared_ptr<Tensor> norm_weight;
};

struct LayerNormWeights {
  float epsilon = 1e-5;
  std::shared_ptr<Tensor> gamma;
  std::shared_ptr<Tensor> beta;
};

struct LlmWeights {
  using NormWeights = std::variant<RMSNormWeights, LayerNormWeights>;

  struct SelfAttentionWeights {
    std::optional<NormWeights> pre_norm_weight;

    std::shared_ptr<Tensor> k_weight;
    std::shared_ptr<Tensor> k_bias;
    std::shared_ptr<Tensor> q_weight;
    std::shared_ptr<Tensor> q_bias;
    std::shared_ptr<Tensor> v_weight;
    std::shared_ptr<Tensor> v_bias;
    std::shared_ptr<Tensor> per_dim_scale;
    std::shared_ptr<Tensor> post_proj_weight;
    std::shared_ptr<Tensor> post_proj_bias;

    std::optional<NormWeights> post_norm_weight;
  };

  struct FeedForwardWeights {
    std::optional<NormWeights> pre_norm_weight;
    std::shared_ptr<Tensor> layer_1_weight;
    std::shared_ptr<Tensor> layer_1_bias;
    std::shared_ptr<Tensor> layer_1_gate_weight;
    std::shared_ptr<Tensor> layer_1_gate_bias;
    std::shared_ptr<Tensor> layer_2_weight;
    std::shared_ptr<Tensor> layer_2_bias;
    std::optional<NormWeights> post_norm_weight;
  };

  std::vector<FeedForwardWeights> ffs;
  std::vector<SelfAttentionWeights> sas;
  std::vector<SelfAttentionWeights> cas;
  std::optional<NormWeights> final_norm_weight;
  std::shared_ptr<Tensor> softmax_linear;
  std::shared_ptr<Tensor> softmax_bias;
  std::optional<NormWeights> embedding_norm_weight;

  // Usually same as softmax_linear, but some models use different
  // softmax_linear v.s. embedding table.
  std::shared_ptr<Tensor> token_embedding;

  // For models that inherit Llm that need more weights other than above defined
  // ones, they can load custom weights through their custom weight loader, and
  // store in this map. The builder can then access these custom weights.
  absl::flat_hash_map<std::string, std::shared_ptr<Tensor>> custom_weights;
};

absl::StatusOr<std::optional<LlmWeights::NormWeights>> LoadNormWeights(
    LlmParams::Norm norm_type, std::vector<size_t> dims,
    absl::string_view basename, WeightAccessor& weight_accessor);

inline absl::StatusOr<std::optional<LlmWeights::NormWeights>> LoadNormWeights(
    LlmParams::Norm norm_type, const LlmParams& params,
    absl::string_view basename, WeightAccessor& weight_accessor) {
  return LoadNormWeights(norm_type, std::vector<size_t>{params.model_dim_D},
                         basename, weight_accessor);
}

class LlmWeightsLoader {
 public:
  constexpr static absl::string_view kTokenEmbedding{
      "params.lm.token_embedding.w"};
  constexpr static absl::string_view kTransformerWeightPrefix{
      "params.lm.transformer.x_layers_"};
  constexpr static absl::string_view kLogitsFfnBiasFilename{
      "params.lm.softmax.logits_ffn.bias.b"};
  constexpr static absl::string_view kLogitsFfnWeightFilename{
      "params.lm.softmax.logits_ffn.linear.w"};

  LlmWeightsLoader(std::unique_ptr<WeightAccessor> weight_accessor,
                   const LlmParams& params)
      : weight_accessor_(std::move(weight_accessor)), params_(params) {}
  virtual ~LlmWeightsLoader() = default;

  virtual absl::StatusOr<LlmWeights> LoadWeights();

  LlmParams& llm_params() { return params_; }
  const LlmParams& llm_params() const { return params_; }

 protected:
  virtual absl::StatusOr<LlmWeights::SelfAttentionWeights> LoadSelfAttention(
      int layer_id);
  virtual absl::StatusOr<LlmWeights::FeedForwardWeights> LoadFeedForward(
      int layer_id);

  // is_query: indicating whether the weight is for query projection or not.
  // Note that the key/value projection weights are handled differently between
  // MHA vs. MQA.
  absl::StatusOr<std::shared_ptr<Tensor>> TryCacheThenLoadSelfAttention(
      absl::string_view filename_prefix, absl::string_view alt_filename_prefix,
      bool is_query);

  std::unique_ptr<WeightAccessor> weight_accessor_;
  LlmParams params_;
};

class DefaultLlmWeightsLoader : public LlmWeightsLoader {
 public:
  DefaultLlmWeightsLoader(std::unique_ptr<WeightAccessor> weight_accessor,
                          const LlmParams& params)
      : LlmWeightsLoader(std::move(weight_accessor), params) {}
  DefaultLlmWeightsLoader(
      absl::string_view weight_path, const LlmParams& params,
      std::shared_ptr<tflite::FlatBufferModel> flat_buffer_model = nullptr);

 private:
  std::shared_ptr<PackWeightsCache> xnn_weights_cache_;
};

}  // namespace mediapipe::tasks::genai::xnn_utils

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_LLM_WEIGHTS_H_
