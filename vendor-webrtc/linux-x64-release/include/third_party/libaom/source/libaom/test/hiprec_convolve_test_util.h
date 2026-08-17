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

#ifndef AOM_TEST_HIPREC_CONVOLVE_TEST_UTIL_H_
#define AOM_TEST_HIPREC_CONVOLVE_TEST_UTIL_H_

#include <tuple>

#include "config/av1_rtcd.h"

#include "gtest/gtest.h"
#include "test/acm_random.h"
#include "test/util.h"
#include "test/register_state_check.h"

#include "aom_ports/aom_timer.h"
#include "av1/common/convolve.h"
#include "av1/common/mv.h"

namespace libaom_test {

namespace AV1HiprecConvolve {

typedef void (*hiprec_convolve_func)(const uint8_t *src, ptrdiff_t src_stride,
                                     uint8_t *dst, ptrdiff_t dst_stride,
                                     const int16_t *filter_x, int x_step_q4,
                                     const int16_t *filter_y, int y_step_q4,
                                     int w, int h,
                                     const WienerConvolveParams *conv_params);

typedef std::tuple<int, int, int, hiprec_convolve_func> HiprecConvolveParam;

::testing::internal::ParamGenerator<HiprecConvolveParam> BuildParams(
    hiprec_convolve_func filter);

class AV1HiprecConvolveTest
    : public ::testing::TestWithParam<HiprecConvolveParam> {
 public:
  ~AV1HiprecConvolveTest() override;
  void SetUp() override;

 protected:
  void RunCheckOutput(hiprec_convolve_func test_impl);
  void RunSpeedTest(hiprec_convolve_func test_impl);

  libaom_test::ACMRandom rnd_;
};

}  // namespace AV1HiprecConvolve

#if CONFIG_AV1_HIGHBITDEPTH
namespace AV1HighbdHiprecConvolve {
typedef void (*highbd_hiprec_convolve_func)(
    const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst,
    ptrdiff_t dst_stride, const int16_t *filter_x, int x_step_q4,
    const int16_t *filter_y, int y_step_q4, int w, int h,
    const WienerConvolveParams *conv_params, int bps);

typedef std::tuple<int, int, int, int, highbd_hiprec_convolve_func>
    HighbdHiprecConvolveParam;

::testing::internal::ParamGenerator<HighbdHiprecConvolveParam> BuildParams(
    highbd_hiprec_convolve_func filter);

class AV1HighbdHiprecConvolveTest
    : public ::testing::TestWithParam<HighbdHiprecConvolveParam> {
 public:
  ~AV1HighbdHiprecConvolveTest() override;
  void SetUp() override;

 protected:
  void RunCheckOutput(highbd_hiprec_convolve_func test_impl);
  void RunSpeedTest(highbd_hiprec_convolve_func test_impl);

  libaom_test::ACMRandom rnd_;
};

}  // namespace AV1HighbdHiprecConvolve
#endif  // CONFIG_AV1_HIGHBITDEPTH
}  // namespace libaom_test

#endif  // AOM_TEST_HIPREC_CONVOLVE_TEST_UTIL_H_
