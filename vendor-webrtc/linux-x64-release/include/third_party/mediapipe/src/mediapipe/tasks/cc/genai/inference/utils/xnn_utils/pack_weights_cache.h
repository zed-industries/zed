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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_PACK_WEIGHTS_CACHE_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_PACK_WEIGHTS_CACHE_H_

#include <cstddef>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/container/flat_hash_map.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "flatbuffers/flatbuffer_builder.h"
#include "mediapipe/tasks/cc/genai/inference/utils/llm_utils/memory_mapped_file.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/graph_builder.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/named_buffer_generated.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/xnn_tensor.h"
#include "xnnpack.h"  // from @XNNPACK

namespace mediapipe::tasks::genai::xnn_utils {

// An implementation of XnnWeightsCache that allows cross-process packed weights
// sharing. This implementation does not really support insertion, which means
// either the cache is fully built already, or will be built from scratch.
class PackWeightsCache : public XnnWeightsCache {
 public:
  // `cache_path` is used in Initialize() and Finalize().
  explicit PackWeightsCache(absl::string_view cache_path);
  ~PackWeightsCache() override;

  // Initializes the cache. The default implementation loads the serialized
  // cache from the `cache_path`.
  virtual absl::Status Initialize();

  // Adds an unpacked weight. Across different processes, the same `weight` may
  // be loaded to different memory address, however the `name` would not change.
  absl::Status AddUnpackedWeight(absl::string_view name,
                                 std::shared_ptr<Tensor> weight);

  // Finalizes the cache. This effectively sets an internal state such that no
  // more cache would be added. It also serializes the cache to `cache_path`.
  absl::Status Finalize() override;

 protected:
  // Returns true if the key is found, but we still report cache miss to XNNPack
  // and trigger packing. Later we double check if the packed weight matches
  // cached one. Inheritance classes can overwrite this function to apply
  // different strategies.
  virtual bool ShouldDoubleCheckCompatibility(
      const xnn_weights_cache_look_up_key*);

  // Returns mapped memory of `filename`. Returns nullptr in case of any error.
  // Inheritance classes can overwrite this function e.g. if there's no
  // filesystem.
  virtual std::shared_ptr<llm_utils::MemoryMappedFile> GetMmapFile(
      absl::string_view filename);

  // Appends `data` from the end of `filename`. Inheritance classes can
  // overwrite this function e.g. if there's no filesystem.
  virtual absl::Status Append(absl::string_view filename,
                              absl::string_view data);

  // Inserts `data` at the beginning of `filename`. Inheritance classes can
  // overwrite this function e.g. if there's no filesystem.
  virtual absl::Status Prepend(absl::string_view filename,
                               absl::string_view data);

 private:
  absl::Status Append(absl::string_view data);
  absl::Status Prepend(absl::string_view data);

  absl::Status InitializeFromCache(
      std::shared_ptr<llm_utils::MemoryMappedFile> mmap_cache);

  // A series of functions for `xnn_weights_cache_provider`. They need to be
  // static such that we can assign function pointers. They need to be class
  // static functions such that they can access non-public members.

  static size_t look_up(PackWeightsCache* context,
                        const xnn_weights_cache_look_up_key* cache_key);

  static void* reserve_space(PackWeightsCache* context, size_t n);

  static size_t look_up_or_insert(
      PackWeightsCache* context, const xnn_weights_cache_look_up_key* cache_key,
      void* ptr, size_t size);

  static bool is_finalized(PackWeightsCache* context);

  static void* offset_to_addr(PackWeightsCache* context, size_t offset);

  static enum xnn_status delete_cache(PackWeightsCache* context) {
    // no-op, the lifetime is assumed to be managed outside.
    return xnn_status_success;
  }

  xnn_weights_cache_provider cache_provider_;

  std::string cache_path_;
  std::shared_ptr<llm_utils::MemoryMappedFile> mmap_file_;
  // Immutable flatbuffer.
  std::shared_ptr<const NamedBuffers> named_buffers_;

  // Only initialized if cache is not present and needs to be built.
  std::unique_ptr<flatbuffers::FlatBufferBuilder> builder_;
  // Blob is the data piece appended after flatbuffer, representing the packed
  // weights.
  size_t blob_size_ = 0;
  std::string tmp_buffer_to_pack_weight_;

  bool is_finalized_ = false;
  absl::Status error_status_ = absl::OkStatus();
  std::optional<xnn_weights_cache_look_up_key> key_sent_for_double_check_;

  absl::flat_hash_map<const void* /*kernel_ptr*/, std::string /*name*/>
      kernel_to_name_;
  absl::flat_hash_map<absl::string_view /*name*/,
                      std::pair<size_t /*offset*/, size_t /*size*/>>
      name_to_offset_size_;
};

// An implementation of `WeightAccessor` interface that calls
// `AddUnpackedWeight` after each Load*Weight().
class WeightAccessorCompositeWithCache : public WeightAccessor {
 public:
  WeightAccessorCompositeWithCache(std::shared_ptr<WeightAccessor> accessor,
                                   PackWeightsCache* weights_cache)
      : accessor_(accessor), weights_cache_(weights_cache) {}
  ~WeightAccessorCompositeWithCache() override = default;

  absl::StatusOr<std::shared_ptr<Tensor>> LoadWeight(
      absl::string_view tensor_name, Tensor::DimsType expected_dims,
      size_t dim_scale_if_any) const override;

  absl::StatusOr<std::shared_ptr<Tensor>> LoadTransposedWeight(
      absl::string_view tensor_name, Tensor::DimsType expected_dims,
      size_t dim_scale_if_any) const override;

 private:
  std::shared_ptr<WeightAccessor> accessor_;
  PackWeightsCache* const weights_cache_;
};

}  // namespace mediapipe::tasks::genai::xnn_utils

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_PACK_WEIGHTS_CACHE_H_
