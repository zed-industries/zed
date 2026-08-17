/*
 * Copyright (c) 2025, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
#ifndef AOM_AOM_DSP_SAD_HWY_H_
#define AOM_AOM_DSP_SAD_HWY_H_

#include "aom_dsp/reduce_sum_hwy.h"
#include "third_party/highway/hwy/highway.h"

HWY_BEFORE_NAMESPACE();

namespace {
namespace HWY_NAMESPACE {

namespace hn = hwy::HWY_NAMESPACE;

template <int BlockWidth>
HWY_MAYBE_UNUSED unsigned int SumOfAbsoluteDiff(
    const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr,
    int ref_stride, int h, const uint8_t *second_pred = nullptr) {
  constexpr hn::CappedTag<uint8_t, BlockWidth> pixel_tag;
  constexpr hn::Repartition<uint64_t, decltype(pixel_tag)> intermediate_sum_tag;
  const int vw = hn::Lanes(pixel_tag);
  auto sum_sad = hn::Zero(intermediate_sum_tag);
  const bool is_sad_avg = second_pred != nullptr;
  for (int i = 0; i < h; ++i) {
    for (int j = 0; j < BlockWidth; j += vw) {
      auto src_vec = hn::LoadU(pixel_tag, &src_ptr[j]);
      auto ref_vec = hn::LoadU(pixel_tag, &ref_ptr[j]);
      if (is_sad_avg) {
        auto sec_pred_vec = hn::LoadU(pixel_tag, &second_pred[j]);
        ref_vec = hn::AverageRound(ref_vec, sec_pred_vec);
      }
      auto sad = hn::SumsOf8AbsDiff(src_vec, ref_vec);
      sum_sad = hn::Add(sum_sad, sad);
    }
    src_ptr += src_stride;
    ref_ptr += ref_stride;
    if (is_sad_avg) {
      second_pred += BlockWidth;
    }
  }
  return static_cast<unsigned int>(
      hn::ReduceSum(intermediate_sum_tag, sum_sad));
}

}  // namespace HWY_NAMESPACE
}  // namespace

#define FSAD(w, h, suffix)                                                   \
  extern "C" unsigned int aom_sad##w##x##h##_##suffix(                       \
      const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr,        \
      int ref_stride);                                                       \
  HWY_ATTR unsigned int aom_sad##w##x##h##_##suffix(                         \
      const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr,        \
      int ref_stride) {                                                      \
    return HWY_NAMESPACE::SumOfAbsoluteDiff<w>(src_ptr, src_stride, ref_ptr, \
                                               ref_stride, h);               \
  }

#define FOR_EACH_SAD_BLOCK_SIZE(X, suffix) \
  X(128, 128, suffix)                      \
  X(128, 64, suffix)                       \
  X(64, 128, suffix)                       \
  X(64, 64, suffix)                        \
  X(64, 32, suffix)

HWY_AFTER_NAMESPACE();

#endif  // AOM_AOM_DSP_SAD_HWY_H_
