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

#ifndef TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_INDEX_FILE_MUTATOR_H_
#define TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_INDEX_FILE_MUTATOR_H_

#include "tensorflow_lite_support/scann_ondevice/cc/core/serialized_searcher.pb.h"
#include "absl/status/statusor.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "absl/types/optional.h"  // from @com_google_absl
#include "absl/types/span.h"  // from @com_google_absl
#include "leveldb/db.h"  // from @com_google_leveldb
#include "tensorflow_lite_support/scann_ondevice/proto/index_config.pb.h"

namespace tflite {
namespace scann_ondevice {

struct IndexedArtifacts {
  // Config for on-device scam. Contains pretrained parts such as partition
  // centroids, compression codebook.
  tflite::scann_ondevice::core::ScannOnDeviceConfig config;

  // The dimension of each processed embedding in either hashed_database or
  // float_database. Note that if hashing is enabled, it can be different from
  // the original embedding dimension depending on the config.
  uint32_t embedding_dim;

  // Flattened database embeddings. The embeddings should be stored
  // consecutively in row major layout. Exactly one of the hashed_database and
  // float_database is expected. hashed_database can be either AH compressed or
  // 8-bit quantized. In the case of 8-bit quantization, it's casted to uint8_t.
  absl::optional<absl::Span<const uint8_t>> hashed_database;
  absl::optional<absl::Span<const float>> float_database;

  // The partition each of the database point belongs to, if the index uses a
  // partitioner. The size should be the same as how many database points there
  // are.
  absl::optional<absl::Span<const uint32_t>> partition_assignment;

  // The metadata (label) for each database point.The size should be the same as
  // how many database points there are.
  absl::Span<const std::string> metadata;

  // An arbitrary user supplied string for storing custom information.
  std::string userinfo;
};

// Creates a byte buffer for the index file from the artifacts. Returns errors
// when there are not exactly one database specified, or other issues with input
// such as shape mismatch, invalid partition indices etc.
absl::StatusOr<std::string> CreateIndexBuffer(
    const IndexedArtifacts& artifacts, bool compression);

}  // namespace scann_ondevice
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_INDEX_FILE_MUTATOR_H_
