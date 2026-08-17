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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_EMBEDDING_SEARCHER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_EMBEDDING_SEARCHER_H_

#include <cstdint>
#include <initializer_list>
#include <memory>
#include <optional>
#include <vector>

#include "tensorflow_lite_support/scann_ondevice/cc/core/partitioner.h"
#include "tensorflow_lite_support/scann_ondevice/cc/core/processor.h"
#include "tensorflow_lite_support/scann_ondevice/cc/core/top_n_amortized_constant.h"
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "absl/types/span.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/external_file_handler.h"
#include "tensorflow_lite_support/cc/task/processor/proto/embedding.pb.h"
#include "tensorflow_lite_support/cc/task/processor/proto/search_options.pb.h"
#include "tensorflow_lite_support/cc/task/processor/proto/search_result.pb.h"
#include "tensorflow_lite_support/scann_ondevice/cc/index.h"
#include "tensorflow_lite_support/scann_ondevice/proto/index_config.pb.h"

namespace tflite {
namespace task {
namespace processor {

// A utility class for performing nearest-neighbor search on embedding results.
class EmbeddingSearcher {
 public:
  EmbeddingSearcher() = default;
  virtual ~EmbeddingSearcher() = default;
  // Neither copyable or movable.
  EmbeddingSearcher(const EmbeddingSearcher&) = delete;
  EmbeddingSearcher& operator=(const EmbeddingSearcher&) = delete;

  /** The factory method for EmbeddingSearcher.
   *  @param search_options              The search options.
   *  @param optional_index_file_content Required when index_file option is not
   *                                     provided in search_options.
   */
  static tflite::support::StatusOr<std::unique_ptr<EmbeddingSearcher>> Create(
      std::unique_ptr<SearchOptions> search_options,
      std::optional<absl::string_view> optional_index_file_content =
          std::nullopt);

  // Performs a nearest-neighbor search in the index on the provided embedding.
  absl::StatusOr<SearchResult> Search(
      const ::tflite::task::processor::Embedding& embedding);

  // Provides access to the opaque user info stored in the index file (if any),
  // in raw binary form. Returns an empty string if the index doesn't contain
  // user info.
  tflite::support::StatusOr<absl::string_view> GetUserInfo();

 private:
  absl::Status Init(
      std::unique_ptr<SearchOptions> options,
      std::optional<absl::string_view> optional_index_file_content);

  absl::Status QuantizedSearch(Eigen::Ref<Eigen::MatrixXf> query,
                               std::vector<int> leaves_to_search,
                               absl::Span<tflite::scann_ondevice::core::TopN> top_n);
  absl::Status LinearSearch(Eigen::Ref<Eigen::MatrixXf> query,
                            std::vector<int> leaves_to_search,
                            absl::Span<tflite::scann_ondevice::core::TopN> top_n);

  std::unique_ptr<SearchOptions> options_;

  // Index management.
  std::unique_ptr<tflite::task::core::ExternalFileHandler> index_file_handler_;
  std::unique_ptr<tflite::scann_ondevice::Index> index_;
  tflite::scann_ondevice::IndexConfig index_config_;

  // ScaNN management.
  int num_leaves_to_search_;
  tflite::scann_ondevice::core::DistanceMeasure distance_measure_;
  std::unique_ptr<tflite::scann_ondevice::core::PartitionerInterface> partitioner_;
  std::shared_ptr<tflite::scann_ondevice::core::AsymmetricHashQuerier> quantizer_;
};

}  // namespace processor
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_EMBEDDING_SEARCHER_H_
