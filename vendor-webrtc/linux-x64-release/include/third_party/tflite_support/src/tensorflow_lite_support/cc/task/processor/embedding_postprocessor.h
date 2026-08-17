/* Copyright 2021 The TensorFlow Authors. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_EMBEDDING_POSTPROCESSOR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_EMBEDDING_POSTPROCESSOR_H_

#include <cstdint>
#include <initializer_list>
#include <memory>

#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/port/status_macros.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"
#include "tensorflow_lite_support/cc/task/processor/processor.h"
#include "tensorflow_lite_support/cc/task/processor/proto/embedding_options.pb.h"

namespace tflite {
namespace task {
namespace processor {

// This postprocessor works with the following output tensor:
//   (kTfLiteUInt8/kTfLiteFloat32)
//    - `N` components corresponding to the `N` dimensions of the returned
//      feature vector for this output layer.
//    - Either 2 or 4 dimensions, i.e. `[1 x N]` or `[1 x 1 x 1 x N]`.
class EmbeddingPostprocessor : public Postprocessor {
 public:
  static tflite::support::StatusOr<std::unique_ptr<EmbeddingPostprocessor>>
  Create(core::TfLiteEngine* engine,
         const std::initializer_list<int> output_indices,
         std::unique_ptr<EmbeddingOptions> options =
             std::make_unique<EmbeddingOptions>());

  template <typename T>
  absl::Status Postprocess(T* embedding);

  // Utility function to compute cosine similarity [1] between two feature
  // vectors. May return an InvalidArgumentError if e.g. the feature vectors are
  // of different types (quantized vs. float), have different sizes, or have a
  // an L2-norm of 0.
  //
  // [1]: https://en.wikipedia.org/wiki/Cosine_similarity
  template <typename T>
  static tflite::support::StatusOr<double> CosineSimilarity(const T& u,
                                                            const T& v);

  int GetEmbeddingDimension() const { return embedding_dimension_; }

 private:
  using Postprocessor::Postprocessor;

  absl::Status Init(std::unique_ptr<EmbeddingOptions> options);

  std::unique_ptr<EmbeddingOptions> options_;

  int embedding_dimension_ = 0;

  // Performs actual cosine similarity computation.
  template <typename T>
  static tflite::support::StatusOr<double> ComputeCosineSimilarity(
      const T* u, const T* v, int num_elements);

  template <typename T>
  void NormalizeFeatureVector(T* feature_vector) const;

  template <typename T>
  void QuantizeFeatureVector(T* feature_vector) const;
};

template <typename T>
absl::Status EmbeddingPostprocessor::Postprocess(T* embedding) {
  embedding->set_output_index(tensor_indices_.at(0));
  auto* feature_vector = embedding->mutable_feature_vector();
  if (GetTensor()->type == kTfLiteUInt8) {
    const uint8_t* output_data =
        engine_->interpreter()->typed_output_tensor<uint8_t>(
            tensor_indices_.at(0));
    // Get the zero_point and scale parameters from the tensor metadata.
    const int output_tensor_index =
        engine_->interpreter()->outputs()[tensor_indices_.at(0)];
    const TfLiteTensor* output_tensor =
        engine_->interpreter()->tensor(output_tensor_index);
    for (int j = 0; j < embedding_dimension_; ++j) {
      feature_vector->add_value_float(output_tensor->params.scale *
                                      (static_cast<int>(output_data[j]) -
                                       output_tensor->params.zero_point));
    }
  } else {
    // Float
    const float* output_data =
        engine_->interpreter()->typed_output_tensor<float>(
            tensor_indices_.at(0));
    for (int j = 0; j < embedding_dimension_; ++j) {
      feature_vector->add_value_float(output_data[j]);
    }
  }
  if (options_->l2_normalize()) {
    NormalizeFeatureVector(feature_vector);
  }
  if (options_->quantize()) {
    QuantizeFeatureVector(feature_vector);
  }
  return absl::OkStatus();
}

template <typename T>
void EmbeddingPostprocessor::NormalizeFeatureVector(T* feature_vector) const {
  float squared_l2_norm = 0.0f;
  for (const float val : feature_vector->value_float()) {
    squared_l2_norm += val * val;
  }
  if (squared_l2_norm == 0.0f) {
    return;
  }
  const float inv_l2_norm = 1.0f / std::sqrt(squared_l2_norm);
  for (int i = 0; i < feature_vector->value_float().size(); ++i) {
    feature_vector->set_value_float(
        i, feature_vector->value_float(i) * inv_l2_norm);
  }
}

template <typename T>
void EmbeddingPostprocessor::QuantizeFeatureVector(T* feature_vector) const {
  auto* quantized_values = feature_vector->mutable_value_string();
  quantized_values->resize(feature_vector->value_float().size());
  for (int i = 0; i < feature_vector->value_float().size(); ++i) {
    int value = static_cast<int>(roundf(feature_vector->value_float(i) * 128));
    (*quantized_values)[i] =
        static_cast<char>(std::max(-128, std::min(value, 127)));
  }
  feature_vector->clear_value_float();
}

/* static */
template <typename T>
tflite::support::StatusOr<double>
EmbeddingPostprocessor::ComputeCosineSimilarity(const T* u, const T* v,
                                                int num_elements) {
  if (num_elements <= 0) {
    return CreateStatusWithPayload(
        absl::StatusCode::kInvalidArgument,
        "Cannot compute cosine similarity on empty feature vectors",
        support::TfLiteSupportStatus::kInvalidArgumentError);
  }
  double dot_product = 0.0;
  double norm_u = 0.0;
  double norm_v = 0.0;
  for (int i = 0; i < num_elements; ++i) {
    dot_product += u[i] * v[i];
    norm_u += u[i] * u[i];
    norm_v += v[i] * v[i];
  }
  if (norm_u <= 0.0 || norm_v <= 0.0) {
    return CreateStatusWithPayload(
        absl::StatusCode::kInvalidArgument,
        "Cannot compute cosine similarity on feature vector with 0 norm",
        support::TfLiteSupportStatus::kInvalidArgumentError);
  }
  return dot_product / std::sqrt(norm_u * norm_v);
}

/* static */
template <typename T>
tflite::support::StatusOr<double> EmbeddingPostprocessor::CosineSimilarity(
    const T& u, const T& v) {
  if (u.has_value_string() && v.has_value_string()) {
    if (u.value_string().size() != v.value_string().size()) {
      return CreateStatusWithPayload(
          absl::StatusCode::kInvalidArgument,
          absl::StrFormat("Cannot compute cosine similarity on quantized "
                          "feature vectors of different sizes (%d vs %d)",
                          u.value_string().size(), v.value_string().size()),
          support::TfLiteSupportStatus::kInvalidArgumentError);
    }
    return ComputeCosineSimilarity(
        reinterpret_cast<const int8_t*>(&u.value_string()[0]),
        reinterpret_cast<const int8_t*>(&v.value_string()[0]),
        u.value_string().size());
  }
  if (!u.has_value_string() && !v.has_value_string()) {
    if (u.value_float_size() != v.value_float_size()) {
      return CreateStatusWithPayload(
          absl::StatusCode::kInvalidArgument,
          absl::StrFormat("Cannot compute cosine similarity on float "
                          "feature vectors of different sizes (%d vs %d)",
                          u.value_float_size(), v.value_float_size()),
          support::TfLiteSupportStatus::kInvalidArgumentError);
    }
    return ComputeCosineSimilarity(
        u.value_float().data(), v.value_float().data(), u.value_float().size());
  }
  return CreateStatusWithPayload(
      absl::StatusCode::kInvalidArgument,
      "Cannot compute cosine similarity between quantized and float "
      "feature vectors",
      support::TfLiteSupportStatus::kInvalidArgumentError);
}

}  // namespace processor
}  // namespace task
}  // namespace tflite
#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_EMBEDDING_POSTPROCESSOR_H_
