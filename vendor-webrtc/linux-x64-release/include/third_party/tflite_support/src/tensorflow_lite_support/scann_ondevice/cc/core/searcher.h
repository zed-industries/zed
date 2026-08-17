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
#ifndef TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_SEARCHER_H_
#define TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_SEARCHER_H_

#include <algorithm>
#include <cstdint>
#include <limits>
#include <utility>
#include <vector>

#include <glog/logging.h>
#include "absl/types/span.h"  // from @com_google_absl
#include "Eigen/Core"  // from @eigen_archive
#include "tensorflow_lite_support/scann_ondevice/cc/core/index_table_sum.h"
#include "tensorflow_lite_support/scann_ondevice/cc/core/processor.h"
#include "tensorflow_lite_support/scann_ondevice/cc/core/serialized_searcher.pb.h"
#include "tensorflow_lite_support/scann_ondevice/cc/core/simd_utils.h"
#include "tensorflow_lite_support/scann_ondevice/cc/core/top_n_amortized_constant.h"

namespace tflite {
namespace scann_ondevice {
namespace core {

using Matrix8u =
    Eigen::Matrix<uint8_t, Eigen::Dynamic, Eigen::Dynamic, Eigen::ColMajor>;

namespace internal {
void ComputeAHDistance(const QueryInfo& query_info,
                       Eigen::Ref<const Matrix8u> database,
                       Eigen::Ref<Eigen::MatrixXf> output);

}
template <class T>
bool AsymmetricHashFindNeighbors(const QueryInfo& query_info,
                                 Eigen::Ref<const Matrix8u> database,
                                 size_t global_offset, absl::Span<T> topn) {
  const int batch_size = query_info.query_lut->cols();
  if (topn.size() != batch_size) {
    return false;
  }
  int database_size = database.cols();
  Eigen::MatrixXf output(batch_size, database_size);
  internal::ComputeAHDistance(query_info, database, output);

  for (int i = 0; i < database_size; i++) {
    for (int j = 0; j < topn.size(); ++j) {
      topn[j].emplace(output(j, i), i + global_offset);
    }
  }
  return true;
}
template <class T>
bool AsymmetricHashFindNeighbors(Eigen::Ref<const Eigen::MatrixXf> queries,
                                 const PreProcessorInterface& preprocessor,
                                 Eigen::Ref<const Matrix8u> database,
                                 size_t global_offset, absl::Span<T> topn) {
  if (queries.cols() != topn.size()) {
    return false;
  }
  QueryInfo query_info;
  return preprocessor.Process(queries, &query_info) &&
         AsymmetricHashFindNeighbors(query_info, database, global_offset, topn);
}
template <class T>
bool FloatFindNeighbors(Eigen::Ref<const Eigen::MatrixXf> queries,
                        Eigen::Ref<const Eigen::MatrixXf> database,
                        const size_t global_offset,
                        const DistanceMeasure distance_measure,
                        absl::Span<T> topn) {
  int query_size = queries.cols();
  int database_size = database.cols();
  Eigen::MatrixXf pairwise_distances(query_size, database_size);

  if (distance_measure == SQUARED_L2_DISTANCE) {
    pairwise_distances.colwise() = queries.colwise().squaredNorm().transpose();
    pairwise_distances.rowwise() += database.colwise().squaredNorm();
    pairwise_distances -= 2 * queries.transpose() * database;
  } else if (distance_measure == DOT_PRODUCT) {
    pairwise_distances = -1 * queries.transpose() * database;
  } else {
    LOG(ERROR) << "Unsupported distance measure: "
               << DistanceMeasure_Name(distance_measure);
    return false;
  }

  for (int i = 0; i < database_size; ++i) {
    for (int j = 0; j < query_size; ++j) {
      topn[j].emplace(pairwise_distances(j, i), i + global_offset);
    }
  }
  return true;
}
template <class T>
class SearcherInterfaceT {
 public:
  virtual ~SearcherInterfaceT() {}

  virtual bool FindNeighbors(const Eigen::Ref<const Eigen::MatrixXf>& queries,
                             std::vector<T>* topn) const = 0;
};
template <class T>
class AsymmetricHashLeafSearcherT : public SearcherInterfaceT<T> {
 public:
  static std::unique_ptr<AsymmetricHashLeafSearcherT<T>> Create(
      std::shared_ptr<QueryInfo::Matrix<uint8_t>> database, int global_offset,
      std::shared_ptr<PreProcessorInterface> preprocessor);
  static std::unique_ptr<AsymmetricHashLeafSearcherT<T>> Create(
      std::shared_ptr<QueryInfo::Matrix<uint8_t>> database, int global_offset,
      std::shared_ptr<PreProcessorInterface> preprocessor,
      size_t mini_batch_size);
  bool FindNeighbors(const Eigen::Ref<const Eigen::MatrixXf>& queries,
                     std::vector<T>* topn) const override;
  bool FindNeighbors(const QueryInfo& query_info, std::vector<T>* topn) const;

 private:
  AsymmetricHashLeafSearcherT(
      std::shared_ptr<QueryInfo::Matrix<uint8_t>> database, int global_offset,
      std::shared_ptr<PreProcessorInterface> preprocessor,
      size_t mini_batch_size)
      : database_(std::move(database)),
        global_offset_(global_offset),
        preprocessor_(std::move(preprocessor)),
        mini_batch_size_(mini_batch_size) {}
  std::shared_ptr<QueryInfo::Matrix<uint8_t>> database_ = nullptr;
  int global_offset_;
  std::shared_ptr<PreProcessorInterface> preprocessor_ = nullptr;
  const size_t mini_batch_size_;
};
template <class T>
class LinearLeafSearcherT : public SearcherInterfaceT<T> {
 public:
  ~LinearLeafSearcherT() override {}
  static std::unique_ptr<LinearLeafSearcherT<T>> Create(
      std::shared_ptr<Eigen::MatrixXf> database,
      DistanceMeasure distance_measure = SQUARED_L2_DISTANCE,
      int global_offset = 0);

  bool FindNeighbors(const Eigen::Ref<const Eigen::MatrixXf>& queries,
                     std::vector<T>* topn) const override;

 private:
  LinearLeafSearcherT(std::shared_ptr<Eigen::MatrixXf> database,
                      DistanceMeasure distance_measure, int global_offset)
      : database_(std::move(database)),
        distance_measure_(distance_measure),
        global_offset_(global_offset) {}

  std::shared_ptr<Eigen::MatrixXf> database_ = nullptr;
  const DistanceMeasure distance_measure_;
  int global_offset_;
};

template <class T>
std::unique_ptr<AsymmetricHashLeafSearcherT<T>>
AsymmetricHashLeafSearcherT<T>::Create(
    std::shared_ptr<Matrix8u> database, int global_offset,
    std::shared_ptr<PreProcessorInterface> preprocessor) {
  return AsymmetricHashLeafSearcherT<T>::Create(
      database, global_offset, preprocessor,
      std::numeric_limits<size_t>::max());
}

template <class T>
std::unique_ptr<AsymmetricHashLeafSearcherT<T>>
AsymmetricHashLeafSearcherT<T>::Create(
    std::shared_ptr<Matrix8u> database, int global_offset,
    std::shared_ptr<PreProcessorInterface> preprocessor,
    size_t mini_batch_size) {
  if (mini_batch_size == 0 || global_offset < 0) {
    return nullptr;
  }
  return std::unique_ptr<AsymmetricHashLeafSearcherT<T>>(
      new AsymmetricHashLeafSearcherT<T>(std::move(database), global_offset,
                                         std::move(preprocessor),
                                         mini_batch_size));
}

template <class T>
bool AsymmetricHashLeafSearcherT<T>::FindNeighbors(
    const Eigen::Ref<const Eigen::MatrixXf>& queries,
    std::vector<T>* topn) const {
  if (queries.cols() != topn->size()) {
    return false;
  }

  absl::Span<T> topn_span = absl::MakeSpan(*topn);
  for (size_t i = 0; i < queries.cols(); i += mini_batch_size_) {
    const size_t num_queries_in_batch =
        std::min(mini_batch_size_, queries.cols() - i);
    if (!AsymmetricHashFindNeighbors<T>(
            queries.middleCols(i, num_queries_in_batch), *preprocessor_,
            *database_, global_offset_,
            topn_span.subspan(i, num_queries_in_batch))) {
      return false;
    }
  }
  return true;
}

template <class T>
bool AsymmetricHashLeafSearcherT<T>::FindNeighbors(const QueryInfo& query_info,
                                                   std::vector<T>* topn) const {
  return AsymmetricHashFindNeighbors<T>(query_info, *database_, global_offset_,
                                        absl::MakeSpan(*topn));
}

template <class T>
std::unique_ptr<LinearLeafSearcherT<T>> LinearLeafSearcherT<T>::Create(
    std::shared_ptr<Eigen::MatrixXf> database, DistanceMeasure distance_measure,
    int global_offset) {
  if (global_offset < 0) {
    return nullptr;
  }
  return std::unique_ptr<LinearLeafSearcherT<T>>(new LinearLeafSearcherT<T>(
      std::move(database), distance_measure, global_offset));
}

template <class T>
bool LinearLeafSearcherT<T>::FindNeighbors(
    const Eigen::Ref<const Eigen::MatrixXf>& queries,
    std::vector<T>* topn) const {
  return FloatFindNeighbors<T>(queries, *database_, global_offset_,
                               distance_measure_, absl::MakeSpan(*topn));
}

using SearcherInterface = SearcherInterfaceT<TopN>;
using AsymmetricHashLeafSearcher = AsymmetricHashLeafSearcherT<TopN>;
using LinearLeafSearcher = LinearLeafSearcherT<TopN>;
}  // namespace core
}  // namespace scann_ondevice
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_SEARCHER_H_
