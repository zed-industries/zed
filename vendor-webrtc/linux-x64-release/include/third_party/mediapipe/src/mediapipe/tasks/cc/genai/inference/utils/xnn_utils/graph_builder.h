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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_GRAPH_BUILDER_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_GRAPH_BUILDER_H_

#include <sys/types.h>

#include <cstddef>
#include <functional>
#include <limits>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/absl_check.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/xnn_tensor.h"
#include "xnnpack.h"  // from @XNNPACK

namespace mediapipe::tasks::genai {
namespace xnn_utils {

using XnnSubgraphPtr =
    std::unique_ptr<xnn_subgraph, decltype(&xnn_delete_subgraph)>;
using XnnRuntimePtr =
    std::unique_ptr<xnn_runtime, decltype(&xnn_delete_runtime)>;
using XnnThreadpoolPtr =
    std::unique_ptr<pthreadpool, decltype(&pthreadpool_destroy)>;

struct XnnWeightsCache {
 public:
  explicit XnnWeightsCache(xnn_weights_cache_t weights_cache = nullptr);
  virtual ~XnnWeightsCache();

  // Hard finalize the cache. This should be called after creating *all* XNN
  // runtime.
  virtual absl::Status Finalize();

  xnn_weights_cache_t Get() const { return xnn_weights_cache; }

 protected:
  xnn_weights_cache_t xnn_weights_cache;
};

struct ClampParams {
  float out_min = -std::numeric_limits<float>::infinity();
  float out_max = std::numeric_limits<float>::infinity();
};

struct FullConnParams : public ClampParams {
  bool transpose = false;
};

struct RuntimeConfigs {
  // Whether to enable xnn profling.
  bool xnn_profile = false;
  // If profiling is enabled, dump profiling results to a CSV.
  std::string xnn_profile_csv;

  // Number of thread used to create XNN runtime.
  size_t xnn_num_threads = 4;

  // Packed weights to be reused among multiple runtime.
  std::shared_ptr<XnnWeightsCache> weights_cache;

  // Whether or not to use dynamic quantization to speed up. If not provided,
  // we will try best to enable it, given tensor/weight data type.
  std::optional<bool> use_dynamic_quantization;

  enum class ActivationPrecision : int {
    kFP32,
    kFP16
  } activation_precision = ActivationPrecision::kFP32;
};

absl::StatusOr<std::shared_ptr<XnnWeightsCache>> CreateWeightsCache(
    size_t buffer_size = /*XNN_DEFAULT_WEIGHTS_BUFFER_SIZE=*/1048576);

class XnnGraph;

// XnnGraphBuilder is used to construct XnnGraph (through Build()). Once a
// XnnGraph is constructed, it can run for multiple times.
class XnnGraphBuilder {
 public:
  explicit XnnGraphBuilder(
      std::unique_ptr<RuntimeConfigs> runtime_configs = nullptr,
      xnn_datatype data_type = xnn_datatype_fp32)
      : runtime_configs_(std::move(runtime_configs)), data_type_(data_type) {
    if (!runtime_configs_) {
      runtime_configs_ = std::make_unique<RuntimeConfigs>();
    }
  }
  virtual ~XnnGraphBuilder() = default;

  absl::StatusOr<std::unique_ptr<XnnGraph>> Build();

  // New input or output tensor.
  absl::StatusOr<std::shared_ptr<Tensor>> NewInput(Tensor::DimsType dims,
                                                   absl::string_view tag = "");
  absl::Status MarkInput(std::shared_ptr<Tensor> t);

  // New static weight, populate value before Build()
  void NewWeight(std::shared_ptr<Tensor> t);

  // Element wise square.
  absl::StatusOr<std::shared_ptr<Tensor>> Square(std::shared_ptr<Tensor> input);

  absl::StatusOr<std::shared_ptr<Tensor>> SquareRoot(
      std::shared_ptr<Tensor> input);

  absl::StatusOr<std::shared_ptr<Tensor>> Gelu(std::shared_ptr<Tensor> input);

  absl::StatusOr<std::shared_ptr<Tensor>> Sigmoid(
      std::shared_ptr<Tensor> input);

  absl::StatusOr<std::shared_ptr<Tensor>> Silu(std::shared_ptr<Tensor> input);

  absl::StatusOr<std::shared_ptr<Tensor>> Relu(std::shared_ptr<Tensor> input);

  absl::StatusOr<std::shared_ptr<Tensor>> Relu1p5(
      std::shared_ptr<Tensor> input);

  absl::StatusOr<std::shared_ptr<Tensor>> Abs(std::shared_ptr<Tensor> input);

  absl::StatusOr<std::shared_ptr<Tensor>> Log(std::shared_ptr<Tensor> input);

  absl::StatusOr<std::shared_ptr<Tensor>> CopySign(std::shared_ptr<Tensor> lhs,
                                                   std::shared_ptr<Tensor> rhs);

  absl::StatusOr<std::shared_ptr<Tensor>> Clamp(std::shared_ptr<Tensor> input,
                                                ClampParams params);

  absl::StatusOr<std::shared_ptr<Tensor>> Tanh(std::shared_ptr<Tensor> input);

  // logits = cap * jnp.tanh(logits / cap)
  absl::StatusOr<std::shared_ptr<Tensor>> CapTanh(std::shared_ptr<Tensor> input,
                                                  float cap);

  // Average over last dimension, keep num of dims same.
  absl::StatusOr<std::shared_ptr<Tensor>> AvgLastDim(
      std::shared_ptr<Tensor> input);

  absl::StatusOr<std::shared_ptr<Tensor>> Rms(std::shared_ptr<Tensor> input);

  // Root Mean Square normalization
  // out = input / rms(input) * (1 + scale)
  // if scale is absent, scale is considered to be zero.
  absl::StatusOr<std::shared_ptr<Tensor>> RmsNorm(
      std::shared_ptr<Tensor> input, std::shared_ptr<Tensor> scale = nullptr);

  absl::StatusOr<std::shared_ptr<Tensor>> Reshape(std::shared_ptr<Tensor> input,
                                                  Tensor::DimsType new_dims);

  absl::StatusOr<std::shared_ptr<Tensor>> Permute(std::shared_ptr<Tensor> input,
                                                  Tensor::DimsType permute);

  // Create a slice of the input tensor. Both `starts` and `ends` must have
  // the same sizes as the number of dimensions in the input tensor. The
  // resulting slice includes data from `[start[i], end[i])` for each dimension.
  // For instance, for input A = [1, 2, 3, 4] and starts = [1] and ends = [3],
  // the resulting slice would be [2, 3].
  absl::StatusOr<std::shared_ptr<Tensor>> Slice(std::shared_ptr<Tensor> input,
                                                Tensor::DimsType starts,
                                                Tensor::DimsType ends);

  // Create a slice of the input tensor along the provided axis, with other
  // dimensions unchanged. For instance, for input A = [B, M, N] and axis = 1,
  // the output slice would be [B, offset:offset+length, N].
  absl::StatusOr<std::shared_ptr<Tensor>> Slice(std::shared_ptr<Tensor> input,
                                                size_t axis, int64_t offset,
                                                size_t length);

  // Concatenate two input tensors along the provided axis. Both input tensors
  // must have same number of dimensions and dimension values can only differ
  // along the concatenation axis.
  absl::StatusOr<std::shared_ptr<Tensor>> Concat(
      size_t axis, std::shared_ptr<Tensor> input1,
      std::shared_ptr<Tensor> input2);

  // input: [B * I]
  // filter: [O * I], [I * O] if transpose
  // return: [B * O]
  absl::StatusOr<std::shared_ptr<Tensor>> MatMul(
      std::shared_ptr<Tensor> input, std::shared_ptr<Tensor> weight) {
    return MatMul(input, weight, FullConnParams());
  }

  absl::StatusOr<std::shared_ptr<Tensor>> MatMul(std::shared_ptr<Tensor> input,
                                                 std::shared_ptr<Tensor> weight,
                                                 FullConnParams params) {
    return FullConn(input, weight, nullptr, params);
  }

  absl::StatusOr<std::shared_ptr<Tensor>> BatchMatMul(
      std::shared_ptr<Tensor> input, std::shared_ptr<Tensor> weight,
      FullConnParams params = FullConnParams());

  absl::StatusOr<std::shared_ptr<Tensor>> FullConn(
      std::shared_ptr<Tensor> input, std::shared_ptr<Tensor> weight,
      std::shared_ptr<Tensor> bias) {
    return FullConn(input, weight, bias, FullConnParams());
  }

  absl::StatusOr<std::shared_ptr<Tensor>> FullConn(
      std::shared_ptr<Tensor> input, std::shared_ptr<Tensor> weight,
      std::shared_ptr<Tensor> bias, FullConnParams params);

  absl::StatusOr<std::shared_ptr<Tensor>> Softmax(
      std::shared_ptr<Tensor> input);

  absl::StatusOr<std::shared_ptr<Tensor>> SelfAttentionProj(
      std::shared_ptr<Tensor> input, std::shared_ptr<Tensor> weight,
      std::shared_ptr<Tensor> bias, size_t num_heads);

  absl::StatusOr<std::shared_ptr<Tensor>> SelfAttentionProj(
      std::shared_ptr<Tensor> input, std::shared_ptr<Tensor> weight,
      std::shared_ptr<Tensor> bias);

  absl::StatusOr<std::shared_ptr<Tensor>> SelfAttentionProj(
      std::shared_ptr<Tensor> input, std::shared_ptr<Tensor> weight);

  // Mimic einsum(BNTH.BN'SH -> BNTS) for attention between query and key/value,
  // i.e. just batch matrix multiply between 2 tensors, assuming the inputs are
  // 4-d tensors, and their first/last dimension should match. This function
  // checks the 2nd dimension of `key_or_value` to apply MHA/MQA.
  absl::StatusOr<std::shared_ptr<Tensor>> QKVAttention(
      std::shared_ptr<Tensor> query, std::shared_ptr<Tensor> key_or_value,
      Tensor::DimsType reshape_hint);

  absl::StatusOr<std::shared_ptr<Tensor>> ElementAdd(
      std::shared_ptr<Tensor> lhs, std::shared_ptr<Tensor> rhs,
      ClampParams params = ClampParams());

  absl::StatusOr<std::shared_ptr<Tensor>> ElementAdd(
      std::shared_ptr<Tensor> lhs, float rhs,
      ClampParams params = ClampParams());

  absl::StatusOr<std::shared_ptr<Tensor>> ElementSub(
      std::shared_ptr<Tensor> lhs, std::shared_ptr<Tensor> rhs,
      ClampParams params = ClampParams());

  absl::StatusOr<std::shared_ptr<Tensor>> ElementSub(
      std::shared_ptr<Tensor> lhs, float rhs,
      ClampParams params = ClampParams());

  absl::StatusOr<std::shared_ptr<Tensor>> ElementSub(
      float lhs, std::shared_ptr<Tensor> rhs,
      ClampParams params = ClampParams());

  absl::StatusOr<std::shared_ptr<Tensor>> ElementMul(
      std::shared_ptr<Tensor> lhs, std::shared_ptr<Tensor> rhs,
      ClampParams params = ClampParams());

  absl::StatusOr<std::shared_ptr<Tensor>> ElementMul(
      std::shared_ptr<Tensor> lhs, float rhs,
      ClampParams params = ClampParams());

  absl::StatusOr<std::shared_ptr<Tensor>> ElementDiv(
      std::shared_ptr<Tensor> lhs, std::shared_ptr<Tensor> rhs,
      ClampParams params = ClampParams());

  absl::StatusOr<std::shared_ptr<Tensor>> ElementDiv(
      std::shared_ptr<Tensor> lhs, float rhs,
      ClampParams params = ClampParams());

  absl::StatusOr<std::shared_ptr<Tensor>> Rope(
      std::shared_ptr<Tensor> input, std::shared_ptr<Tensor> segment_pos);

  // An extension of the Rope operator that allows applying embeddings to a
  // slice of the input tensor upto the specified `idx` value. The expected
  // input shape is `B,T,N,H` and the tensor is sliced along the H axis.
  // `segment_pos`, which specifies precomputed Rope `sin` and `cos` values,
  // must be sized based on the slice of the input tensor that goes through
  // Rope.
  absl::StatusOr<std::shared_ptr<Tensor>> PartialRope(
      std::shared_ptr<Tensor> input, size_t idx,
      std::shared_ptr<Tensor> segment_pos);

  absl::StatusOr<std::shared_ptr<Tensor>> PerDimScale(
      std::shared_ptr<Tensor> input, std::shared_ptr<Tensor> per_dim_scale);

  absl::StatusOr<std::shared_ptr<Tensor>> SquaredDifference(
      std::shared_ptr<Tensor> lhs, std::shared_ptr<Tensor> rhs);

  absl::StatusOr<std::shared_ptr<Tensor>> LayerNorm(
      std::shared_ptr<Tensor> input, float epsilon = 1e-5,
      std::shared_ptr<Tensor> gamma = nullptr,
      std::shared_ptr<Tensor> beta = nullptr);

 protected:
  absl::StatusOr<std::shared_ptr<Tensor>> ExpandDims(
      std::shared_ptr<Tensor> input, Tensor::DimsType new_axes);

  absl::StatusOr<std::shared_ptr<Tensor>> IntermediateTensor(
      Tensor::DimsType dims, absl::string_view tag = "");
  absl::StatusOr<std::shared_ptr<Tensor>> IntermediateTensor(
      Tensor::DimsType dims, xnn_datatype data_type,
      absl::string_view tag = "");

  std::unique_ptr<RuntimeConfigs> runtime_configs_;
  const xnn_datatype data_type_;

  std::vector<std::function<absl::Status(xnn_subgraph_t)>> build_steps_;

  // Input tensors keeping the same order as how they were added.
  std::vector<std::shared_ptr<Tensor>> input_tensors_added_order_;
  // Input tensors in hash_set, for easy existence check.
  absl::flat_hash_set<std::shared_ptr<Tensor>> input_tensors_;
  // Intermediate tensors keeping the same order as how they were added.
  std::vector<std::shared_ptr<Tensor>> interm_tensors_added_order_;
  // Intermediate tensors in hash_set, for easy existence check.
  absl::flat_hash_set<std::shared_ptr<Tensor>> interm_tensors_;

  // Static weights keeping the same order as how they were added.
  std::vector<std::shared_ptr<Tensor>> static_weights_added_order_;
  absl::flat_hash_set<std::shared_ptr<Tensor>> static_weights_;

  // Caches
  absl::flat_hash_map<
      size_t /*dim*/,
      absl::flat_hash_map<const Tensor* /*scale*/, std::shared_ptr<Tensor>>>
      per_dim_scale_cache_;
};

class XnnGraph {
 public:
  XnnGraph(XnnSubgraphPtr subgraph,
           std::unique_ptr<RuntimeConfigs> runtime_configs)
      : owned_subgraph_(std::move(subgraph)),
        runtime_configs_(std::move(runtime_configs)) {}
  XnnGraph(XnnGraph&& other) = default;
  virtual ~XnnGraph() = default;

  virtual absl::Status SetupRuntime();

  // xnn_subgraph should be created with same size.
  virtual absl::Status Run();

 protected:
  friend class XnnGraphBuilder;

  absl::Status CreateRuntime();

  XnnSubgraphPtr owned_subgraph_;

  absl::flat_hash_map<size_t, Tensor> avg_cache_;
  absl::flat_hash_map<size_t, Tensor> cap_tanh_cache_;

  // Runtime
  std::unique_ptr<RuntimeConfigs> runtime_configs_;
  XnnRuntimePtr runtime_{nullptr, xnn_delete_runtime};
  std::vector<xnn_external_value> externals_;

  XnnThreadpoolPtr threadpool_{nullptr, pthreadpool_destroy};

  std::vector<std::shared_ptr<Tensor>> input_tensors_;
  std::vector<std::shared_ptr<Tensor>> output_tensors_;
};

}  // namespace xnn_utils
}  // namespace mediapipe::tasks::genai

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_GRAPH_BUILDER_H_
