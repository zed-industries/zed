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
#ifndef AOM_AOM_DSP_REDUCE_SUM_HWY_H_
#define AOM_AOM_DSP_REDUCE_SUM_HWY_H_

#include <type_traits>
#include "third_party/highway/hwy/highway.h"

HWY_BEFORE_NAMESPACE();

namespace {
namespace HWY_NAMESPACE {

namespace hn = hwy::HWY_NAMESPACE;

template <size_t NumBlocks>
struct BlockReduceTraits;

template <>
struct BlockReduceTraits<1> {
  template <typename D>
  HWY_ATTR HWY_INLINE static hn::VFromD<D> ReduceSum(D d, hn::VFromD<D> v) {
    (void)d;
    return v;
  }
};

template <size_t NumBlocks>
struct BlockReduceTraits {
  static_assert(NumBlocks > 1,
                "Primary template BlockReduceTraits assumes NumBlocks > 1");
  static_assert((NumBlocks & (NumBlocks - 1)) == 0,
                "BlockReduceTraits requires NumBlocks to be a power of 2.");

  template <typename D>
  HWY_ATTR HWY_INLINE static hn::VFromD<hn::BlockDFromD<D>> ReduceSum(
      D d, hn::VFromD<D> v) {
    (void)d;
    constexpr hn::Half<D> half_d;
    auto v_half = hn::Add(hn::LowerHalf(half_d, v), hn::UpperHalf(half_d, v));
    return BlockReduceTraits<NumBlocks / 2>::ReduceSum(half_d, v_half);
  }
};

// ReduceSum across blocks.
// For example, with a 4-block vector with 16 lanes of uint32_t:
// [a3 b3 c3 d3 a2 b2 c2 d2 a1 b1 c1 d1 a0 b0 c0 d0]
// returns a vector with 4 lanes:
// [a3+a2+a1+a0 b3+b2+b1+b0 c3+c2+c1+c0 d3+d2+d1+d0]
template <typename D>
HWY_ATTR HWY_INLINE hn::Vec<hn::BlockDFromD<D>> BlockReduceSum(
    D int_tag, hn::VFromD<D> v) {
  return BlockReduceTraits<int_tag.MaxBlocks()>::ReduceSum(int_tag, v);
}

}  // namespace HWY_NAMESPACE
}  // namespace

HWY_AFTER_NAMESPACE();

#endif  // AOM_AOM_DSP_REDUCE_SUM_HWY_H_
