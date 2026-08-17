/* Copyright 2022 The MediaPipe Authors.

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

#ifndef MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_EMBEDDING_RESULT_H_
#define MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_EMBEDDING_RESULT_H_

#include <optional>
#include <string>
#include <vector>

#include "mediapipe/tasks/cc/components/containers/proto/embeddings.pb.h"

namespace mediapipe::tasks::components::containers {

// Embedding result for a given embedder head.
//
// One and only one of the two 'float_embedding' and 'quantized_embedding' will
// contain data, based on whether or not the embedder was configured to perform
// scalar quantization.
struct Embedding {
  // Floating-point embedding. Empty if the embedder was configured to perform
  // scalar-quantization.
  std::vector<float> float_embedding;
  // Scalar-quantized embedding. Empty if the embedder was not configured to
  // perform scalar quantization.
  std::string quantized_embedding;
  // The index of the embedder head (i.e. output tensor) this embedding comes
  // from. This is useful for multi-head models.
  int head_index;
  // The optional name of the embedder head, as provided in the TFLite Model
  // Metadata [1] if present. This is useful for multi-head models.
  //
  // [1]: https://www.tensorflow.org/lite/convert/metadata
  std::optional<std::string> head_name = std::nullopt;
};

// Defines embedding results of a model.
struct EmbeddingResult {
  // The embedding results for each head of the model.
  std::vector<Embedding> embeddings;
  // The optional timestamp (in milliseconds) of the start of the chunk of data
  // corresponding to these results.
  //
  // This is only used for embedding extraction on time series (e.g. audio
  // embedding). In these use cases, the amount of data to process might
  // exceed the maximum size that the model can process: to solve this, the
  // input data is split into multiple chunks starting at different timestamps.
  std::optional<int64_t> timestamp_ms = std::nullopt;
};

// Utility function to convert from Embedding proto to Embedding struct.
Embedding ConvertToEmbedding(const proto::Embedding& proto);

// Utility function to convert from EmbeddingResult proto to EmbeddingResult
// struct.
EmbeddingResult ConvertToEmbeddingResult(const proto::EmbeddingResult& proto);

}  // namespace mediapipe::tasks::components::containers

#endif  // MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_EMBEDDING_RESULT_H_
