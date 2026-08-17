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
#ifndef TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_PROCESSOR_H_
#define TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_PROCESSOR_H_

#include <cstdint>
#include <utility>
#include <vector>

#include "Eigen/Core"  // from @eigen_archive
#include "tensorflow_lite_support/scann_ondevice/cc/core/serialized_searcher.pb.h"
#include "tensorflow_lite_support/scann_ondevice/cc/core/top_n_amortized_constant.h"

namespace tflite {
namespace scann_ondevice {
namespace core {
struct QueryInfo {
  template <typename T>
  using Matrix =
      Eigen::Matrix<T, Eigen::Dynamic, Eigen::Dynamic, Eigen::ColMajor>;

  float fixed_point_min = NAN;
  float fixed_point_max = NAN;
  float fixed_point_offset = NAN;
  float fixed_point_scale = NAN;

  std::shared_ptr<Matrix<float>> query_lut;
  std::shared_ptr<Matrix<uint16_t>> query_lut_uint16;
  std::shared_ptr<Matrix<uint8_t>> query_lut_uint8;
  template <typename T>
  std::shared_ptr<Matrix<T>> QueryLUT();

  std::shared_ptr<Matrix<float>> transposed_query_lut;
  std::shared_ptr<Matrix<uint16_t>> transposed_query_lut_uint16;
  std::shared_ptr<Matrix<uint8_t>> transposed_query_lut_uint8;
  template <typename T>
  std::shared_ptr<Matrix<T>> TransposedQueryLUT();
};
class PreProcessorInterface {
 public:
  virtual ~PreProcessorInterface() {}

  virtual bool Process(const Eigen::Ref<const Eigen::MatrixXf>& queries,
                       QueryInfo* query_info) const = 0;
  virtual int num_database_dims() const = 0;
  virtual int num_query_dims() const = 0;
};
class PostProcessorInterface {
 public:
  virtual ~PostProcessorInterface() {}

  virtual bool Process(std::vector<TopN>* top_n) const = 0;
};
class AsymmetricHashQuerier : public PreProcessorInterface {
 public:
  static std::unique_ptr<AsymmetricHashQuerier> Create(
      const AsymmetricHashingProto& proto);
  bool Process(const Eigen::Ref<const Eigen::MatrixXf>& queries,
               QueryInfo* query_info) const override;

  inline int num_database_dims() const override { return codebooks_.size(); }

  inline int num_query_dims() const override { return dims_; }

 private:
  AsymmetricHashQuerier(std::vector<Eigen::MatrixXf> codebooks,
                        std::vector<Eigen::VectorXf> codebook_norms,
                        DistanceMeasure query_distance,
                        AsymmetricHashingProto::LookupType lookup_type,
                        int dims)
      : dims_(dims),
        lookup_type_(lookup_type),
        query_distance_(query_distance),
        codebooks_(std::move(codebooks)),
        codebook_norms_(std::move(codebook_norms)) {}
  void RearrangeLUT(QueryInfo* query_info) const;

  int dims_;
  AsymmetricHashingProto::LookupType lookup_type_;
  DistanceMeasure query_distance_;
  std::vector<Eigen::MatrixXf> codebooks_;
  std::vector<Eigen::VectorXf> codebook_norms_;
};

}  // namespace core
}  // namespace scann_ondevice
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_PROCESSOR_H_
