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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_XNN_TENSOR_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_XNN_TENSOR_H_

#include <fcntl.h>

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <functional>
#include <memory>
#include <numeric>
#include <optional>
#include <ostream>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "mediapipe/framework/formats/tensor.h"
#include "mediapipe/framework/port/logging.h"
#include "xnnpack.h"  // from @XNNPACK

namespace mediapipe::tasks::genai {
namespace xnn_utils {

static constexpr absl::string_view kQuantizedScaleSuffix{"_quantized_scale"};
static constexpr absl::string_view kSparsityParamsSuffix{"_sparsity_params"};

struct Tensor {
  using DimsType = std::vector<size_t>;

  explicit Tensor(DimsType in_dims, xnn_datatype datatype_ = xnn_datatype_fp32,
                  bool is_sparse_ = false)
      : datatype(datatype_),
        internal_dims(std::move(in_dims)),
        internal_num_elements(dims.empty()
                                  ? 0
                                  : std::accumulate(std::begin(dims),
                                                    std::end(dims), size_t(1),
                                                    std::multiplies<size_t>())),
        is_sparse_tensor(is_sparse_) {
    elements_capacity = internal_num_elements;
  }
  Tensor(Tensor&& other) = default;

  Tensor& operator=(const Tensor& other) = delete;
  Tensor& operator=(Tensor&& other) = default;

  virtual ~Tensor() = default;

  bool operator==(const Tensor& other) const;

  void SetMetadata(absl::string_view key, int value) { metadata[key] = value; }

  std::optional<int> GetMetadata(absl::string_view key) const;
  int GetMetadata(absl::string_view key, int default_value) const;

  // Add the tensor into subgraph.
  absl::Status DefineAsInput(xnn_subgraph& subgraph);
  absl::Status DefineAsOutput(xnn_subgraph& subgraph);
  absl::Status DefineAsIntermediateTensor(xnn_subgraph& subgraph);
  virtual absl::Status DefineWeight(xnn_subgraph& subgraph, uint32_t flags);
  absl::Status DefineWeight(xnn_subgraph& subgraph);

  // Load the tensor from buffer, assuming the buffer is long enough.
  absl::Status LoadFromBuffer(const void* buffer);
  // Load the tensor from vector of data. If not exact_match, data can hold less
  // than num_elements.
  absl::Status LoadFromVec(const std::vector<float>& data,
                           bool exact_match = false);
  // Load the tensor from file.
  //   file_path: a string representing the path to the file to load from.
  //   use_mmap: whether or not to use mmap to access the file.
  //   exact_match: if true, the file should contain exactly num_elements of
  //       data.
  virtual absl::Status LoadFromFile(absl::string_view file_path, bool use_mmap,
                                    bool exact_match);
  absl::Status LoadFromFile(absl::string_view file_path) {
    return LoadFromFile(file_path, true, true);
  }

  // Dump the tensor to buffer, assuming the buffer is long enough.
  absl::Status DumpToBuffer(void* buffer);
  // Dump the tensor to vector. If exact_match is set to false, out_data may be
  // resized.
  absl::Status DumpToVec(std::vector<float>& out_data, bool exact_match = true);
  // Dump the tensor to a file specified by file_path.
  virtual absl::Status DumpToFile(absl::string_view file_path);

  // If i'th offset is 0, view's ith dim equals to original i'th dim,
  // otherwise 1. e.g. Tensor[A,B,C,D].Slice([0,b,0,0]) returns a tensor of
  // shape [A,1,C,D].
  std::shared_ptr<Tensor> Slice(DimsType offset);
  // Slice along the `index`th dimension, offset at this dimension.
  virtual std::shared_ptr<Tensor> Slice(size_t index, size_t offset);
  // Slice along the `index`th dimension from index start to index end. e.g.
  // Tensor[A,B,C,D].Slice(1, 0, 5) returns a tensor of shape [A,5,C,D].
  virtual std::shared_ptr<Tensor> Slice(size_t index, size_t start, size_t end);

  // Point the underline data to the borrowed tensor's data.
  Tensor& Borrow(std::shared_ptr<Tensor>, size_t element_offset = 0);

  Tensor& Resize(DimsType new_dims);

  // Hint that this is an output of the graph.
  Tensor& MarkOutput() {
    AllocateBufferIfNeeded();
    is_output_tensor = true;
    return *this;
  }

  // Access the tensor data.
  virtual void* Data();
  const void* Data() const;

  // Access the tensor data as certain type.
  template <typename T>
  T* DataAs() {
    ABSL_DCHECK_EQ(ElementSize(1), sizeof(T));
    return static_cast<T*>(Data());
  }
  template <typename T>
  const T* DataAs() const {
    return static_cast<const T*>(Data());
  }

  // Transpose the tensor.
  virtual std::shared_ptr<Tensor> Transpose();

  // Print the tensor values. Tensors with dims > 4 unsupported.
  void PrintSpan();

  // Convert the tensor to f32 format.
  virtual absl::StatusOr<std::shared_ptr<Tensor>> ConvertToF32();

  // Convert the tensor to ::mediapipe::Tensor.
  virtual absl::StatusOr<::mediapipe::Tensor> ConvertToMediapipeTensor();

  // Indicates whether the tensor data is sparse i.e. contains a lot of zeros.
  bool is_sparse() const { return is_sparse_tensor; }

  // Check if the tensor is close to the expected tensor, only used in test.
  absl::Status IsCloseTo(const Tensor& expected_tensor, float atol = 0,
                         float rtol = 2e-3);

  const DimsType& dims = internal_dims;
  const size_t& num_elements = internal_num_elements;
  const xnn_datatype datatype = xnn_datatype_invalid;

  // Get and set id to a given subgraph.
  uint32_t tensor_id(xnn_subgraph_t);
  void set_tensor_id(xnn_subgraph_t, uint32_t id);

  // shared_ptr to make TensorMetadata copyable.
  std::shared_ptr<char> flat_data;
  size_t elements_capacity = 0;

  // Optional, annotates where the tensor comes from. E.g. the filename where
  // the tensor is loaded from.
  std::string tag;

  // Actually allocate buffer unless necessary.
  virtual void AllocateBufferIfNeeded();

 protected:
  friend class XnnGraphBuilder;
  friend class XnnGraph;
  friend std::ostream& operator<<(std::ostream& os, const Tensor& tensor);

  // Invoke xnn_define_*tensor_value to add this tensor to the `subgraph`.
  virtual absl::Status DefineInSubgraph(xnn_subgraph& subgraph, uint32_t flags);

  virtual size_t ElementSize(size_t num_elements) const {
    return num_elements * 4;
  }

  DimsType internal_dims;
  size_t internal_num_elements;

  bool is_output_tensor = false;
  bool is_sparse_tensor = false;

  absl::flat_hash_map<std::string, int> metadata;

  // The same tensor can be used in multiple subgraphs, this is mapping from
  // subgraph to a per-subgraph id.
  absl::flat_hash_map<xnn_subgraph_t, uint32_t> map_subgraph_to_tensor_id;
};

std::ostream& operator<<(std::ostream& os, const Tensor& tensor);

// Channelwise Quantized.
struct QCTensor : public Tensor {
  // in_dims[dim_scale_] == dims of scale data.
  QCTensor(DimsType in_dims, size_t dim_scale_,
           xnn_datatype datatype_ = xnn_datatype_qcint8,
           bool is_sparse_ = false)
      : Tensor(std::move(in_dims), datatype_, is_sparse_),
        dim_scale(dim_scale_) {
    ABSL_CHECK_LT(dim_scale, 4);
    if (datatype == xnn_datatype_qcint4) {
      zero_point = 8;
    } else {
      zero_point = 0;
    }
  }

  void AllocateBufferIfNeeded() override;
  size_t ElementSize(size_t num_elements) const override {
    switch (datatype) {
      case xnn_datatype_qcint8:
        return num_elements;
      case xnn_datatype_qcint4:
        return (num_elements + 1) / 2;
      default:
        ABSL_LOG(FATAL) << "Unsupported datatype: " << datatype;
        return 0;
    }
  }

  virtual absl::Status LoadFromFile(absl::string_view quantized_weight_filename,
                                    absl::string_view scale_filename,
                                    bool use_mmap, bool exact_match);
  // Append kQuantizedScaleSuffix to use as scale filename.
  absl::Status LoadFromFile(absl::string_view file_path, bool use_mmap,
                            bool exact_match) override {
    return LoadFromFile(file_path,
                        absl::StrCat(file_path, kQuantizedScaleSuffix),
                        use_mmap, exact_match);
  }

  absl::Status DumpToFile(absl::string_view file_path) override;

  absl::Status DefineWeight(xnn_subgraph& subgraph, uint32_t flags) override;

  std::shared_ptr<Tensor> Transpose() override;

  absl::StatusOr<std::shared_ptr<Tensor>> ConvertToF32() override;

  std::shared_ptr<Tensor> Slice(size_t index, size_t offset) override;

  std::shared_ptr<float> scale_data;
  // Index of the dimension to scale.
  size_t dim_scale;
  int32_t zero_point;

 private:
  friend std::ostream& operator<<(std::ostream& os, const QCTensor& tensor);
};

std::ostream& operator<<(std::ostream& os, const QCTensor& tensor);

// Interface to access weights. The interface allows e.g. benchmark test to
// return random-initialized weights content, without preparing real weights.
class WeightAccessor {
 public:
  virtual ~WeightAccessor() = default;

  // Load a static weight tensor according to tensor name. The loader tries the
  // best to ensure the dimensions match expected dimension.
  virtual absl::StatusOr<std::shared_ptr<Tensor>> LoadWeight(
      absl::string_view, Tensor::DimsType, size_t dim_scale_if_any) const = 0;
  absl::StatusOr<std::shared_ptr<Tensor>> LoadWeight(
      absl::string_view filename_prefix, Tensor::DimsType expected_dims) const {
    return LoadWeight(filename_prefix, std::move(expected_dims), 0);
  }

  // Load weight, then transpose before return.
  virtual absl::StatusOr<std::shared_ptr<Tensor>> LoadTransposedWeight(
      absl::string_view, Tensor::DimsType, size_t dim_scale_if_any) const = 0;
};

// May be attached to an LLM graph as a side input to override how weights are
// accessed.
using WeightAccessorProvider = std::function<std::unique_ptr<WeightAccessor>()>;

}  // namespace xnn_utils
}  // namespace mediapipe::tasks::genai

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_XNN_TENSOR_H_
