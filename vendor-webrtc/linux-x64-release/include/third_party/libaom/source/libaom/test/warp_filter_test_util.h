/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_TEST_WARP_FILTER_TEST_UTIL_H_
#define AOM_TEST_WARP_FILTER_TEST_UTIL_H_

#include <tuple>

#include "config/av1_rtcd.h"
#include "config/aom_dsp_rtcd.h"

#include "gtest/gtest.h"
#include "test/acm_random.h"
#include "test/util.h"
#include "test/register_state_check.h"

#include "av1/common/mv.h"
#include "av1/common/common_data.h"

namespace libaom_test {

void generate_warped_model(libaom_test::ACMRandom *rnd, int32_t *mat,
                           int16_t *alpha, int16_t *beta, int16_t *gamma,
                           int16_t *delta, int is_alpha_zero, int is_beta_zero,
                           int is_gamma_zero, int is_delta_zero);

namespace AV1WarpFilter {

typedef void (*warp_affine_func)(const int32_t *mat, const uint8_t *ref,
                                 int width, int height, int stride,
                                 uint8_t *pred, int p_col, int p_row,
                                 int p_width, int p_height, int p_stride,
                                 int subsampling_x, int subsampling_y,
                                 ConvolveParams *conv_params, int16_t alpha,
                                 int16_t beta, int16_t gamma, int16_t delta);

typedef std::tuple<int, int, int, warp_affine_func> WarpTestParam;
typedef std::tuple<WarpTestParam, int, int, int, int> WarpTestParams;

::testing::internal::ParamGenerator<WarpTestParams> BuildParams(
    warp_affine_func filter);

class AV1WarpFilterTest : public ::testing::TestWithParam<WarpTestParams> {
 public:
  ~AV1WarpFilterTest() override;
  void SetUp() override;

 protected:
  void RunCheckOutput(warp_affine_func test_impl);
  void RunSpeedTest(warp_affine_func test_impl);

  libaom_test::ACMRandom rnd_;
};

}  // namespace AV1WarpFilter

#if CONFIG_AV1_HIGHBITDEPTH
namespace AV1HighbdWarpFilter {
typedef void (*highbd_warp_affine_func)(const int32_t *mat, const uint16_t *ref,
                                        int width, int height, int stride,
                                        uint16_t *pred, int p_col, int p_row,
                                        int p_width, int p_height, int p_stride,
                                        int subsampling_x, int subsampling_y,
                                        int bd, ConvolveParams *conv_params,
                                        int16_t alpha, int16_t beta,
                                        int16_t gamma, int16_t delta);

typedef std::tuple<int, int, int, int, highbd_warp_affine_func>
    HighbdWarpTestParam;
typedef std::tuple<HighbdWarpTestParam, int, int, int, int>
    HighbdWarpTestParams;

::testing::internal::ParamGenerator<HighbdWarpTestParams> BuildParams(
    highbd_warp_affine_func filter);

class AV1HighbdWarpFilterTest
    : public ::testing::TestWithParam<HighbdWarpTestParams> {
 public:
  ~AV1HighbdWarpFilterTest() override;
  void SetUp() override;

 protected:
  void RunCheckOutput(highbd_warp_affine_func test_impl);
  void RunSpeedTest(highbd_warp_affine_func test_impl);

  libaom_test::ACMRandom rnd_;
};

}  // namespace AV1HighbdWarpFilter
#endif  // CONFIG_AV1_HIGHBITDEPTH

}  // namespace libaom_test

#endif  // AOM_TEST_WARP_FILTER_TEST_UTIL_H_
