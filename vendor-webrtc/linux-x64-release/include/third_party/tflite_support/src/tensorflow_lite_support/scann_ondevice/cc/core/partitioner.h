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
#ifndef TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_PARTITIONER_H_
#define TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_PARTITIONER_H_

#include <utility>

#include "absl/types/optional.h"  // from @com_google_absl
#include "Eigen/Core"  // from @eigen_archive
#include "tensorflow_lite_support/scann_ondevice/cc/core/serialized_searcher.pb.h"

namespace tflite {
namespace scann_ondevice {
namespace core {
class PartitionerInterface {
 public:
  virtual ~PartitionerInterface() {}
  virtual bool Partition(const Eigen::Ref<const Eigen::MatrixXf>& queries,
                         std::vector<std::vector<int>>* tokens) const = 0;

  virtual int NumPartitions() const = 0;
  virtual absl::optional<int> get_vector_dimension() const = 0;
};
class Partitioner : public PartitionerInterface {
 public:
  static std::unique_ptr<Partitioner> Create(const PartitionerProto& proto);
  bool Partition(const Eigen::Ref<const Eigen::MatrixXf>& queries,
                 std::vector<std::vector<int>>* tokens) const override;
  int NumPartitions() const override;

  inline absl::optional<int> get_vector_dimension() const override {
    return leaves_.cols();
  }

 private:
  Partitioner(Eigen::MatrixXf leaves, Eigen::VectorXf leaf_norms,
              DistanceMeasure distance)
      : leaves_(std::move(leaves)),
        leaf_norms_(std::move(leaf_norms)),
        distance_(distance) {}

  Eigen::MatrixXf leaves_;
  Eigen::VectorXf leaf_norms_;
  DistanceMeasure distance_;
};
class NoOpPartitioner : public PartitionerInterface {
 public:
  ~NoOpPartitioner() override {}

  bool Partition(const Eigen::Ref<const Eigen::MatrixXf>& queries,
                 std::vector<std::vector<int>>* tokens) const override;

  int NumPartitions() const override;
  inline absl::optional<int> get_vector_dimension() const override {
    return absl::optional<int>();
  }
};

}  // namespace core
}  // namespace scann_ondevice
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_PARTITIONER_H_
