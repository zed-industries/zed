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
#ifndef TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_INDEX_TABLE_SUM_H_
#define TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_INDEX_TABLE_SUM_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <vector>

#include "Eigen/Core"  // from @eigen_archive
#include "tensorflow_lite_support/scann_ondevice/cc/core/simd_utils.h"

namespace tflite {
namespace scann_ondevice {
namespace core {

template <typename LutType>
void RearrangeLUT(const LutType* input_data, int batch_elems, int batch_size,
                  LutType* const output_data) {
  std::vector<int64_t> simd_sizes;
  if (std::is_same<LutType, float>::value) {
#ifdef __AVX__
    simd_sizes = {8, 4};
#elif defined __SSE__
    simd_sizes = {4};
#elif defined __ARM_NEON
    simd_sizes = {4};
#endif
  } else {
#ifdef __AVX2__
    simd_sizes = {16, 8};
#elif defined __SSE4_1__
    simd_sizes = {8};
#elif defined __ARM_NEON
    simd_sizes = {8};
#endif
  }

  int64_t offset = 0;
  for (int64_t simd_size : simd_sizes) {
    const int64_t num_simds = batch_size / simd_size;
    const int64_t simd_batch_elems = simd_size * batch_elems;
    for (; offset < num_simds * simd_batch_elems; offset += simd_batch_elems) {
      using RowMajorMatrix = Eigen::Matrix<LutType, Eigen::Dynamic,
                                           Eigen::Dynamic, Eigen::RowMajor>;
      Eigen::Map<const RowMajorMatrix> input_map(input_data + offset, simd_size,
                                                 batch_elems);
      Eigen::Map<RowMajorMatrix> output_map(output_data + offset, batch_elems,
                                            simd_size);
      output_map = input_map.transpose();
    }
  }
  std::copy(input_data + offset, input_data + batch_elems * batch_size,
            output_data + offset);
}
const int kDefaultChunksPerBlock = 32;
const int k16CentersUint8LutChunksPerBlock = 256;
const int kUnrollSteps = 6;

template <typename T>
struct MaxQuantizationValue {
  static_assert(std::is_same<T, float>::value, "Invalid lookup table type.");
  static constexpr size_t value = 0;
};

template <>
struct MaxQuantizationValue<uint8_t> {
  static constexpr size_t value = 255;
};

template <>
struct MaxQuantizationValue<uint16_t> {
  static constexpr size_t value = (1 << 16) / kDefaultChunksPerBlock - 1;
};

template <typename SimdType, typename LutType, size_t NumCenters = 0>
size_t IndexTableSumSimdBatch(const uint8_t* indices, size_t num_chunks,
                              size_t num_outputs, const LutType* lookup_table,
                              size_t batch_size, size_t num_centers, float min,
                              float max, size_t batch_index,
                              float* const output) {
  if (num_centers == 256) {
    return IndexTableSumSimdBatch<SimdType, LutType, 256>(
        indices, num_chunks, num_outputs, lookup_table, batch_size, 0, min, max,
        batch_index, output);
  }
  const size_t lut_chunk_stride = NumCenters ? NumCenters * SimdType::size()
                                             : num_centers * SimdType::size();
  const size_t lut_item_stride =
      NumCenters ? NumCenters * num_chunks : num_chunks * num_centers;
  constexpr bool must_dequantize = !std::is_same<LutType, float>::value;
  constexpr size_t max_qval = MaxQuantizationValue<LutType>::value;
  const float dq_scale = must_dequantize ? (max - min) / max_qval : 0.0f;
  const float dq_offset_1 = must_dequantize ? min + dq_scale / 2 : 0.0f;

  const size_t chunks_per_block =
      std::is_same<LutType, uint8_t>::value &&
              (NumCenters ? NumCenters : num_centers) == 16
          ? k16CentersUint8LutChunksPerBlock
          : kDefaultChunksPerBlock;

  for (; batch_index + SimdType::size() <= batch_size;
       batch_index += SimdType::size()) {
    const LutType* batch_lut = lookup_table + batch_index * lut_item_stride;
    float* const batch_output = output + batch_index;
    for (size_t block_start = 0; block_start < num_chunks;
         block_start += chunks_per_block) {
      const size_t block_end =
          std::min(block_start + chunks_per_block, num_chunks);
      const float dq_offset_n = (block_end - block_start) * dq_offset_1;
      size_t output_index;
      for (output_index = 0; output_index + kUnrollSteps <= num_outputs;
           output_index += kUnrollSteps) {
        const uint8_t* indices_base = indices + output_index * num_chunks;
        size_t chunk_index = block_start;
        const LutType* chunk_lut = batch_lut + chunk_index * lut_chunk_stride;
        std::array<SimdType, kUnrollSteps> accums;
        for (size_t i = 0; i < kUnrollSteps; ++i) {
          const size_t center_index =
              indices_base[i * num_chunks + chunk_index];
          accums[i].load(chunk_lut + center_index * SimdType::size());
        }
        ++chunk_index;
        chunk_lut += lut_chunk_stride;
        for (; chunk_index < block_end; ++chunk_index) {
          for (size_t i = 0; i < kUnrollSteps; ++i) {
            SimdType simd;
            const size_t center_index =
                indices_base[i * num_chunks + chunk_index];
            simd.load(chunk_lut + center_index * SimdType::size());
            accums[i] += simd;
          }
          chunk_lut += lut_chunk_stride;
        }
        for (size_t i = 0; i < kUnrollSteps; ++i) {
          accums[i].dequantize_accum_storeu(
              batch_output + (output_index + i) * batch_size, dq_scale,
              dq_offset_n);
        }
      }
      for (; output_index < num_outputs; ++output_index) {
        const uint8_t* vector_indices = indices + output_index * num_chunks;

        SimdType accum;
        accum.setzero();
        size_t chunk_index = block_start;
        const LutType* chunk_lut = batch_lut + chunk_index * lut_chunk_stride;
        for (; chunk_index < block_end; ++chunk_index) {
          SimdType simd;
          simd.load(chunk_lut + vector_indices[chunk_index] * SimdType::size());
          accum += simd;
          chunk_lut += lut_chunk_stride;
        }

        accum.dequantize_accum_storeu(batch_output + output_index * batch_size,
                                      dq_scale, dq_offset_n);
      }
    }
  }

  return batch_index;
}

template <typename LutType>
void IndexTableSum(const uint8_t* indices, size_t num_chunks,
                   size_t num_outputs, const LutType* lookup_table,
                   size_t batch_size, size_t num_centers, float min, float max,
                   float* const output) {
  static_assert(std::is_same<LutType, uint8_t>::value ||
                    std::is_same<LutType, uint16_t>::value,
                "Invalid lookup table type.");
  std::fill(output, output + batch_size * num_outputs, 0.0f);
  size_t i = 0;
#ifdef __AVX2__
  i = IndexTableSumSimdBatch<SimdInt16x16, LutType>(
      indices, num_chunks, num_outputs, lookup_table, batch_size, num_centers,
      min, max, i, output);
#endif
#ifdef __SSE4_1__
  i = IndexTableSumSimdBatch<SimdInt16x8, LutType>(
      indices, num_chunks, num_outputs, lookup_table, batch_size, num_centers,
      min, max, i, output);
#endif
#ifdef __ARM_NEON
  i = IndexTableSumSimdBatch<SimdInt16x8, LutType>(
      indices, num_chunks, num_outputs, lookup_table, batch_size, num_centers,
      min, max, i, output);
#endif
  i = IndexTableSumSimdBatch<SimdInt16x1, LutType>(
      indices, num_chunks, num_outputs, lookup_table, batch_size, num_centers,
      min, max, i, output);
}

template <>
inline void IndexTableSum<float>(const uint8_t* indices, size_t num_chunks,
                                 size_t num_outputs, const float* lookup_table,
                                 size_t batch_size, size_t num_centers,
                                 float min, float max, float* const output) {
  std::fill(output, output + batch_size * num_outputs, 0.0f);
  size_t i = 0;
#ifdef __AVX__
  i = IndexTableSumSimdBatch<SimdFloat32x8, float>(
      indices, num_chunks, num_outputs, lookup_table, batch_size, num_centers,
      min, max, i, output);
#endif
#ifdef __SSE__
  i = IndexTableSumSimdBatch<SimdFloat32x4, float>(
      indices, num_chunks, num_outputs, lookup_table, batch_size, num_centers,
      min, max, i, output);
#endif
#ifdef __ARM_NEON
  i = IndexTableSumSimdBatch<SimdFloat32x4, float>(
      indices, num_chunks, num_outputs, lookup_table, batch_size, num_centers,
      min, max, i, output);
#endif
  i = IndexTableSumSimdBatch<SimdFloat32x1, float>(
      indices, num_chunks, num_outputs, lookup_table, batch_size, num_centers,
      min, max, i, output);
}

}  // namespace core
}  // namespace scann_ondevice
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_INDEX_TABLE_SUM_H_
