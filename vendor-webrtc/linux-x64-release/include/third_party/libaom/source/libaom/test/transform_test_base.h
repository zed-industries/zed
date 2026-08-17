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

#ifndef AOM_TEST_TRANSFORM_TEST_BASE_H_
#define AOM_TEST_TRANSFORM_TEST_BASE_H_

#include "gtest/gtest.h"

#include "aom/aom_codec.h"
#include "aom_dsp/txfm_common.h"
#include "aom_mem/aom_mem.h"
#include "test/acm_random.h"

namespace libaom_test {

//  Note:
//   Same constant are defined in av1/common/av1_entropy.h and
//   av1/common/entropy.h.  Goal is to make this base class
//   to use for future codec transform testing.  But including
//   either of them would lead to compiling error when we do
//   unit test for another codec. Suggest to move the definition
//   to a aom header file.
const int kDctMaxValue = 16384;

template <typename OutputType>
using FhtFunc = void (*)(const int16_t *in, OutputType *out, int stride,
                         TxfmParam *txfm_param);

template <typename OutputType>
using IhtFunc = void (*)(const tran_low_t *in, uint8_t *out, int stride,
                         const TxfmParam *txfm_param);

template <typename OutType>
class TransformTestBase {
 public:
  virtual ~TransformTestBase() = default;

 protected:
  virtual void RunFwdTxfm(const int16_t *in, OutType *out, int stride) = 0;

  virtual void RunInvTxfm(const OutType *out, uint8_t *dst, int stride) = 0;

  void RunAccuracyCheck(uint32_t ref_max_error, double ref_avg_error) {
    ACMRandom rnd(ACMRandom::DeterministicSeed());
    uint32_t max_error = 0;
    int64_t total_error = 0;
    const int count_test_block = 10000;

    int16_t *test_input_block = reinterpret_cast<int16_t *>(
        aom_memalign(16, sizeof(int16_t) * num_coeffs_));
    ASSERT_NE(test_input_block, nullptr);
    OutType *test_temp_block = reinterpret_cast<OutType *>(
        aom_memalign(16, sizeof(test_temp_block[0]) * num_coeffs_));
    ASSERT_NE(test_temp_block, nullptr);
    uint8_t *dst = reinterpret_cast<uint8_t *>(
        aom_memalign(16, sizeof(uint8_t) * num_coeffs_));
    ASSERT_NE(dst, nullptr);
    uint8_t *src = reinterpret_cast<uint8_t *>(
        aom_memalign(16, sizeof(uint8_t) * num_coeffs_));
    ASSERT_NE(src, nullptr);
    uint16_t *dst16 = reinterpret_cast<uint16_t *>(
        aom_memalign(16, sizeof(uint16_t) * num_coeffs_));
    ASSERT_NE(dst16, nullptr);
    uint16_t *src16 = reinterpret_cast<uint16_t *>(
        aom_memalign(16, sizeof(uint16_t) * num_coeffs_));
    ASSERT_NE(src16, nullptr);

    for (int i = 0; i < count_test_block; ++i) {
      // Initialize a test block with input range [-255, 255].
      for (int j = 0; j < num_coeffs_; ++j) {
        if (bit_depth_ == AOM_BITS_8) {
          src[j] = rnd.Rand8();
          dst[j] = rnd.Rand8();
          test_input_block[j] = src[j] - dst[j];
        } else {
          src16[j] = rnd.Rand16() & mask_;
          dst16[j] = rnd.Rand16() & mask_;
          test_input_block[j] = src16[j] - dst16[j];
        }
      }

      API_REGISTER_STATE_CHECK(
          RunFwdTxfm(test_input_block, test_temp_block, pitch_));
      if (bit_depth_ == AOM_BITS_8) {
        API_REGISTER_STATE_CHECK(RunInvTxfm(test_temp_block, dst, pitch_));
      } else {
        API_REGISTER_STATE_CHECK(
            RunInvTxfm(test_temp_block, CONVERT_TO_BYTEPTR(dst16), pitch_));
      }

      for (int j = 0; j < num_coeffs_; ++j) {
        const int diff =
            bit_depth_ == AOM_BITS_8 ? dst[j] - src[j] : dst16[j] - src16[j];
        const uint32_t error = diff * diff;
        if (max_error < error) max_error = error;
        total_error += error;
      }
    }

    double avg_error = total_error * 1. / count_test_block / num_coeffs_;

    EXPECT_GE(ref_max_error, max_error)
        << "Error: FHT/IHT has an individual round trip error > "
        << ref_max_error;

    EXPECT_GE(ref_avg_error, avg_error)
        << "Error: FHT/IHT has average round trip error > " << ref_avg_error
        << " per block";

    aom_free(test_input_block);
    aom_free(test_temp_block);
    aom_free(dst);
    aom_free(src);
    aom_free(dst16);
    aom_free(src16);
  }

  void RunCoeffCheck() {
    ACMRandom rnd(ACMRandom::DeterministicSeed());
    const int count_test_block = 5000;

    // Use a stride value which is not the width of any transform, to catch
    // cases where the transforms use the stride incorrectly.
    int stride = 96;

    int16_t *input_block = reinterpret_cast<int16_t *>(
        aom_memalign(16, sizeof(int16_t) * stride * height_));
    ASSERT_NE(input_block, nullptr);
    OutType *output_ref_block = reinterpret_cast<OutType *>(
        aom_memalign(16, sizeof(output_ref_block[0]) * num_coeffs_));
    ASSERT_NE(output_ref_block, nullptr);
    OutType *output_block = reinterpret_cast<OutType *>(
        aom_memalign(16, sizeof(output_block[0]) * num_coeffs_));
    ASSERT_NE(output_block, nullptr);

    for (int i = 0; i < count_test_block; ++i) {
      int j, k;
      for (j = 0; j < height_; ++j) {
        for (k = 0; k < pitch_; ++k) {
          int in_idx = j * stride + k;
          int out_idx = j * pitch_ + k;
          input_block[in_idx] = (rnd.Rand16() & mask_) - (rnd.Rand16() & mask_);
          if (bit_depth_ == AOM_BITS_8) {
            output_block[out_idx] = output_ref_block[out_idx] = rnd.Rand8();
          } else {
            output_block[out_idx] = output_ref_block[out_idx] =
                rnd.Rand16() & mask_;
          }
        }
      }

      fwd_txfm_ref(input_block, output_ref_block, stride, &txfm_param_);
      API_REGISTER_STATE_CHECK(RunFwdTxfm(input_block, output_block, stride));

      // The minimum quant value is 4.
      for (j = 0; j < height_; ++j) {
        for (k = 0; k < pitch_; ++k) {
          int out_idx = j * pitch_ + k;
          ASSERT_EQ(output_block[out_idx], output_ref_block[out_idx])
              << "Error: not bit-exact result at index: " << out_idx
              << " at test block: " << i;
        }
      }
    }
    aom_free(input_block);
    aom_free(output_ref_block);
    aom_free(output_block);
  }

  void RunInvCoeffCheck() {
    ACMRandom rnd(ACMRandom::DeterministicSeed());
    const int count_test_block = 5000;

    // Use a stride value which is not the width of any transform, to catch
    // cases where the transforms use the stride incorrectly.
    int stride = 96;

    int16_t *input_block = reinterpret_cast<int16_t *>(
        aom_memalign(16, sizeof(int16_t) * num_coeffs_));
    ASSERT_NE(input_block, nullptr);
    OutType *trans_block = reinterpret_cast<OutType *>(
        aom_memalign(16, sizeof(trans_block[0]) * num_coeffs_));
    ASSERT_NE(trans_block, nullptr);
    uint8_t *output_block = reinterpret_cast<uint8_t *>(
        aom_memalign(16, sizeof(uint8_t) * stride * height_));
    ASSERT_NE(output_block, nullptr);
    uint8_t *output_ref_block = reinterpret_cast<uint8_t *>(
        aom_memalign(16, sizeof(uint8_t) * stride * height_));
    ASSERT_NE(output_ref_block, nullptr);

    for (int i = 0; i < count_test_block; ++i) {
      // Initialize a test block with input range [-mask_, mask_].
      int j, k;
      for (j = 0; j < height_; ++j) {
        for (k = 0; k < pitch_; ++k) {
          int in_idx = j * pitch_ + k;
          int out_idx = j * stride + k;
          input_block[in_idx] = (rnd.Rand16() & mask_) - (rnd.Rand16() & mask_);
          output_ref_block[out_idx] = rnd.Rand16() & mask_;
          output_block[out_idx] = output_ref_block[out_idx];
        }
      }

      fwd_txfm_ref(input_block, trans_block, pitch_, &txfm_param_);

      inv_txfm_ref(trans_block, output_ref_block, stride, &txfm_param_);
      API_REGISTER_STATE_CHECK(RunInvTxfm(trans_block, output_block, stride));

      for (j = 0; j < height_; ++j) {
        for (k = 0; k < pitch_; ++k) {
          int out_idx = j * stride + k;
          ASSERT_EQ(output_block[out_idx], output_ref_block[out_idx])
              << "Error: not bit-exact result at index: " << out_idx
              << " j = " << j << " k = " << k << " at test block: " << i;
        }
      }
    }
    aom_free(input_block);
    aom_free(trans_block);
    aom_free(output_ref_block);
    aom_free(output_block);
  }

  void RunMemCheck() {
    ACMRandom rnd(ACMRandom::DeterministicSeed());
    const int count_test_block = 5000;

    int16_t *input_extreme_block = reinterpret_cast<int16_t *>(
        aom_memalign(16, sizeof(int16_t) * num_coeffs_));
    ASSERT_NE(input_extreme_block, nullptr);
    OutType *output_ref_block = reinterpret_cast<OutType *>(
        aom_memalign(16, sizeof(output_ref_block[0]) * num_coeffs_));
    ASSERT_NE(output_ref_block, nullptr);
    OutType *output_block = reinterpret_cast<OutType *>(
        aom_memalign(16, sizeof(output_block[0]) * num_coeffs_));
    ASSERT_NE(output_block, nullptr);

    for (int i = 0; i < count_test_block; ++i) {
      // Initialize a test block with input range [-mask_, mask_].
      for (int j = 0; j < num_coeffs_; ++j) {
        input_extreme_block[j] = rnd.Rand8() % 2 ? mask_ : -mask_;
      }
      if (i == 0) {
        for (int j = 0; j < num_coeffs_; ++j) input_extreme_block[j] = mask_;
      } else if (i == 1) {
        for (int j = 0; j < num_coeffs_; ++j) input_extreme_block[j] = -mask_;
      }

      fwd_txfm_ref(input_extreme_block, output_ref_block, pitch_, &txfm_param_);
      API_REGISTER_STATE_CHECK(
          RunFwdTxfm(input_extreme_block, output_block, pitch_));

      int row_length = FindRowLength();
      // The minimum quant value is 4.
      for (int j = 0; j < num_coeffs_; ++j) {
        ASSERT_EQ(output_block[j], output_ref_block[j])
            << "Not bit-exact at test index: " << i << ", "
            << "j = " << j << std::endl;
        EXPECT_GE(row_length * kDctMaxValue << (bit_depth_ - 8),
                  abs(output_block[j]))
            << "Error: NxN FDCT has coefficient larger than N*DCT_MAX_VALUE";
      }
    }
    aom_free(input_extreme_block);
    aom_free(output_ref_block);
    aom_free(output_block);
  }

  void RunInvAccuracyCheck(int limit) {
    ACMRandom rnd(ACMRandom::DeterministicSeed());
    const int count_test_block = 1000;

    int16_t *in = reinterpret_cast<int16_t *>(
        aom_memalign(16, sizeof(int16_t) * num_coeffs_));
    ASSERT_NE(in, nullptr);
    OutType *coeff = reinterpret_cast<OutType *>(
        aom_memalign(16, sizeof(coeff[0]) * num_coeffs_));
    ASSERT_NE(coeff, nullptr);
    uint8_t *dst = reinterpret_cast<uint8_t *>(
        aom_memalign(16, sizeof(uint8_t) * num_coeffs_));
    ASSERT_NE(dst, nullptr);
    uint8_t *src = reinterpret_cast<uint8_t *>(
        aom_memalign(16, sizeof(uint8_t) * num_coeffs_));
    ASSERT_NE(src, nullptr);

    uint16_t *dst16 = reinterpret_cast<uint16_t *>(
        aom_memalign(16, sizeof(uint16_t) * num_coeffs_));
    ASSERT_NE(dst16, nullptr);
    uint16_t *src16 = reinterpret_cast<uint16_t *>(
        aom_memalign(16, sizeof(uint16_t) * num_coeffs_));
    ASSERT_NE(src16, nullptr);

    for (int i = 0; i < count_test_block; ++i) {
      // Initialize a test block with input range [-mask_, mask_].
      for (int j = 0; j < num_coeffs_; ++j) {
        if (bit_depth_ == AOM_BITS_8) {
          src[j] = rnd.Rand8();
          dst[j] = rnd.Rand8();
          in[j] = src[j] - dst[j];
        } else {
          src16[j] = rnd.Rand16() & mask_;
          dst16[j] = rnd.Rand16() & mask_;
          in[j] = src16[j] - dst16[j];
        }
      }

      fwd_txfm_ref(in, coeff, pitch_, &txfm_param_);

      if (bit_depth_ == AOM_BITS_8) {
        API_REGISTER_STATE_CHECK(RunInvTxfm(coeff, dst, pitch_));
      } else {
        API_REGISTER_STATE_CHECK(
            RunInvTxfm(coeff, CONVERT_TO_BYTEPTR(dst16), pitch_));
      }

      for (int j = 0; j < num_coeffs_; ++j) {
        const int diff =
            bit_depth_ == AOM_BITS_8 ? dst[j] - src[j] : dst16[j] - src16[j];
        const uint32_t error = diff * diff;
        ASSERT_GE(static_cast<uint32_t>(limit), error)
            << "Error: 4x4 IDCT has error " << error << " at index " << j;
      }
    }
    aom_free(in);
    aom_free(coeff);
    aom_free(dst);
    aom_free(src);
    aom_free(src16);
    aom_free(dst16);
  }

  int pitch_;
  int height_;
  FhtFunc<OutType> fwd_txfm_ref;
  IhtFunc<OutType> inv_txfm_ref;
  aom_bit_depth_t bit_depth_;
  int mask_;
  int num_coeffs_;
  TxfmParam txfm_param_;

 private:
  //  Assume transform size is 4x4, 8x8, 16x16,...
  int FindRowLength() const {
    int row = 4;
    if (16 == num_coeffs_) {
      row = 4;
    } else if (64 == num_coeffs_) {
      row = 8;
    } else if (256 == num_coeffs_) {
      row = 16;
    } else if (1024 == num_coeffs_) {
      row = 32;
    }
    return row;
  }
};

}  // namespace libaom_test

#endif  // AOM_TEST_TRANSFORM_TEST_BASE_H_
