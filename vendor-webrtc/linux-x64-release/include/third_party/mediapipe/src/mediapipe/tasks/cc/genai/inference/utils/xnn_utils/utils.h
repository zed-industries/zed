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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_UTILS_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_UTILS_H_

#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "mediapipe/framework/port/file_helpers.h"
#include "mediapipe/framework/port/ret_check.h"
#include "mediapipe/tasks/cc/genai/inference/utils/llm_utils/memory_mapped_file.h"

namespace mediapipe::tasks::genai {
namespace xnn_utils {

static constexpr absl::string_view kKeySelfAttentionReshapedWeight{
    "self_attention_reshaped_weight_N"};
// Usually fully connect is [K,M] dot [M,N] => [K,N]. Some code base by default
// expects [K,M] dot [N,M] => [K,N], in which case this metadata should be set.
static constexpr absl::string_view kKeyInDimLastInWeight{
    "in_dim_last_in_weight"};

std::vector<float> FillXnnRoPEWeights(size_t max_seq_len, size_t num_channels);

// expect_size_bytes == 0 means don't check size.
template <typename element_type = char>
static absl::StatusOr<std::shared_ptr<element_type>> LoadBufferFromFile(
    absl::string_view file_path, size_t* buffer_size, bool use_mmap = true,
    size_t expect_size_bytes = 0) {
  RET_CHECK(buffer_size);
  if (use_mmap) {
    MP_ASSIGN_OR_RETURN(auto mapped_file,
                        llm_utils::MemoryMappedFile::Create(file_path));
    if (expect_size_bytes) {
      RET_CHECK_EQ(expect_size_bytes, mapped_file->length())
          << "File size " << mapped_file->length() << ", expected "
          << expect_size_bytes << ", file path " << file_path;
    }

    *buffer_size = mapped_file->length();

    // shared_ptr deleter must be copy constructible, so wrap the unique_ptr in
    // another shared_ptr.
    void* data = mapped_file->data();
    auto file_deleter =
        std::make_shared<std::unique_ptr<llm_utils::MemoryMappedFile>>(
            std::move(mapped_file));
    return std::shared_ptr<element_type>(
        static_cast<element_type*>(data),
        [file_deleter](auto* p) { file_deleter->reset(); });
  } else {
    auto read_buffer = std::make_shared<std::string>();
    MP_RETURN_IF_ERROR(
        mediapipe::file::GetContents(file_path, read_buffer.get()));

    if (expect_size_bytes) {
      RET_CHECK_EQ(expect_size_bytes, read_buffer->size())
          << "File size " << read_buffer->size() << ", expected "
          << expect_size_bytes << ", file path " << file_path;
    }

    *buffer_size = read_buffer->size();
    return std::shared_ptr<element_type>(
        read_buffer, reinterpret_cast<element_type*>(read_buffer->data()));
  }
}

template <typename element_type = char>
static absl::StatusOr<std::shared_ptr<element_type>> LoadBufferFromFile(
    absl::string_view file_path, bool use_mmap = true,
    size_t expect_size_bytes = 0) {
  size_t buffer_size;
  return LoadBufferFromFile<element_type>(file_path, &buffer_size, use_mmap,
                                          expect_size_bytes);
}

// Assumes each element in `vec` is less than 16 (4-bit), and packs into an
// array with half the original length. The 4-bit element is stored in LSB
// first, then MSB.
absl::StatusOr<std::vector<uint8_t>> PackInt4ToInt8(absl::Span<uint8_t> vec);

// Unpack the compact 8-bit elements to an array (twice the length of given
// array) of 4-bit elements. The lower 4-bits are unpacked, then followed by
// higher 4-bits.
absl::StatusOr<std::vector<uint8_t>> UnpackInt8ToInt4(
    absl::Span<uint8_t> packed_vec);

absl::StatusOr<std::vector<float>> PositionEmbedding(
    int seq_length, int embedding_dim, float min_timescale = 1.0f,
    float max_timescale = 10000.0f);

// Like PositionEmbedding() but outputs a fully padded embedding for a fixed
// sequence length of `seq_length`, with an input of `input_length` tokens where
// `input_length` is no larger than `seq_length`.
absl::StatusOr<std::vector<float>> FullPositionEmbedding(
    int input_length, int seq_length, int embedding_dim,
    float min_timescale = 1.0f, float max_timescale = 10000.0f);

absl::Status SelectTopK(int top_k,
                        std::vector<std::pair<float, int>>* logits_ids);

}  // namespace xnn_utils
}  // namespace mediapipe::tasks::genai

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_UTILS_H_
