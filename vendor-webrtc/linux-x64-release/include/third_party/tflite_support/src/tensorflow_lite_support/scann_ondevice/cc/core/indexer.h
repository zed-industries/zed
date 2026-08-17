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
#ifndef TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_INDEXER_H_
#define TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_INDEXER_H_

#include <algorithm>
#include <cstdint>
#include <memory>
#include <string>

#include "absl/status/status.h"  // from @com_google_absl
#include "absl/types/span.h"  // from @com_google_absl
#include "tensorflow_lite_support/scann_ondevice/cc/core/serialized_searcher.pb.h"

namespace tflite {
namespace scann_ondevice {
namespace core {
class Indexer {
 public:
  virtual ~Indexer() {}
  virtual void EncodeDatapoint(absl::Span<const float> original,
                               absl::Span<uint8_t> encoded) const = 0;
  virtual absl::Status DecodeDatapoint(
      absl::Span<const uint8_t> encoded,
      absl::Span<float> reconstructed) const = 0;
  virtual int get_input_dimension() const = 0;
  virtual int get_output_dimension() const = 0;
};
class AsymmetricHashingIndexer : public Indexer {
 public:
  explicit AsymmetricHashingIndexer(const AsymmetricHashingProto& ah_proto);

  void EncodeDatapoint(absl::Span<const float> original,
                       absl::Span<uint8_t> encoded) const final;

  absl::Status DecodeDatapoint(absl::Span<const uint8_t> encoded,
                               absl::Span<float> reconstructed) const final;

  int get_input_dimension() const final { return total_dimension_; }

  int get_output_dimension() const final { return codebooks_.size(); }

 private:
  std::vector<uint8_t> dimensions_;
  int32_t total_dimension_;
  std::vector<std::vector<std::vector<float>>> codebooks_;
  DistanceMeasure distance_measure_;
};

}  // namespace core
}  // namespace scann_ondevice
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_INDEXER_H_
