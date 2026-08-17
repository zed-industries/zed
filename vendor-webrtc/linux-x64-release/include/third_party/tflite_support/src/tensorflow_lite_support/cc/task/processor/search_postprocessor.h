/* Copyright 2022 The TensorFlow Authors. All Rights Reserved.

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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_SEARCH_POSTPROCESSOR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_SEARCH_POSTPROCESSOR_H_

#include <cstdint>
#include <initializer_list>
#include <memory>
#include <vector>

#include "absl/strings/string_view.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"
#include "tensorflow_lite_support/cc/task/processor/embedding_postprocessor.h"
#include "tensorflow_lite_support/cc/task/processor/embedding_searcher.h"
#include "tensorflow_lite_support/cc/task/processor/processor.h"
#include "tensorflow_lite_support/cc/task/processor/proto/embedding_options.pb.h"
#include "tensorflow_lite_support/cc/task/processor/proto/search_options.pb.h"
#include "tensorflow_lite_support/cc/task/processor/proto/search_result.pb.h"

namespace tflite {
namespace task {
namespace processor {

// Postprocessor in charge of performing embedding extraction followed by
// nearest-neighbor search.
//
// This postprocessor works with the following output tensor:
//   (kTfLiteUInt8/kTfLiteFloat32)
//    - `N` components corresponding to the `N` dimensions of the returned
//      feature vector for this output layer.
//    - Either 2 or 4 dimensions, i.e. `[1 x N]` or `[1 x 1 x 1 x N]`.
class SearchPostprocessor : public Postprocessor {
 public:
  static tflite::support::StatusOr<std::unique_ptr<SearchPostprocessor>> Create(
      tflite::task::core::TfLiteEngine* engine, int output_index,
      std::unique_ptr<SearchOptions> search_options,
      std::unique_ptr<EmbeddingOptions> embedding_options =
          std::make_unique<EmbeddingOptions>());

  // Converts the tensor outputs to embeddings, then performs a nearest-neighbor
  // search in the index.
  tflite::support::StatusOr<SearchResult> Postprocess();

  // Provides access to the opaque user info stored in the index file (if any),
  // in raw binary form. Returns an empty string if the index doesn't contain
  // user info.
  tflite::support::StatusOr<absl::string_view> GetUserInfo();

 private:
  using Postprocessor::Postprocessor;

  absl::Status Init(
      std::unique_ptr<EmbeddingPostprocessor> embedding_postprocessor,
      std::unique_ptr<SearchOptions> options);

  // Encapsulated EmbeddingPostprocessor converting raw tensors to embeddings.
  std::unique_ptr<EmbeddingPostprocessor> embedding_postprocessor_;

  // The nearest-neighbor searcher for embedding.
  std::unique_ptr<EmbeddingSearcher> embedding_searcher_;
};

}  // namespace processor
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_SEARCH_POSTPROCESSOR_H_
