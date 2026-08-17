/*
 * Copyright (c) 2024, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#include <assert.h>
#include <arm_neon.h>

#include "config/aom_config.h"
#include "config/av1_rtcd.h"

#include "aom_dsp/aom_dsp_common.h"
#include "aom_dsp/arm/mem_neon.h"
#include "aom_ports/mem.h"

#define ROUND_SHIFT 2 * FILTER_BITS - ROUND0_BITS - COMPOUND_ROUND1_BITS

static inline void highbd_12_comp_avg_neon(const uint16_t *src_ptr,
                                           int src_stride, uint16_t *dst_ptr,
                                           int dst_stride, int w, int h,
                                           ConvolveParams *conv_params) {
  const int offset_bits = 12 + 2 * FILTER_BITS - ROUND0_BITS - 2;
  const int offset = (1 << (offset_bits - COMPOUND_ROUND1_BITS)) +
                     (1 << (offset_bits - COMPOUND_ROUND1_BITS - 1));

  CONV_BUF_TYPE *ref_ptr = conv_params->dst;
  const int ref_stride = conv_params->dst_stride;
  const uint16x4_t offset_vec = vdup_n_u16((uint16_t)offset);
  const uint16x8_t max = vdupq_n_u16((1 << 12) - 1);

  if (w == 4) {
    do {
      const uint16x4_t src = vld1_u16(src_ptr);
      const uint16x4_t ref = vld1_u16(ref_ptr);

      uint16x4_t avg = vhadd_u16(src, ref);
      int32x4_t d0 = vreinterpretq_s32_u32(vsubl_u16(avg, offset_vec));

      uint16x4_t d0_u16 = vqrshrun_n_s32(d0, ROUND_SHIFT - 2);
      d0_u16 = vmin_u16(d0_u16, vget_low_u16(max));

      vst1_u16(dst_ptr, d0_u16);

      src_ptr += src_stride;
      ref_ptr += ref_stride;
      dst_ptr += dst_stride;
    } while (--h != 0);
  } else {
    do {
      int width = w;
      const uint16_t *src = src_ptr;
      const uint16_t *ref = ref_ptr;
      uint16_t *dst = dst_ptr;
      do {
        const uint16x8_t s = vld1q_u16(src);
        const uint16x8_t r = vld1q_u16(ref);

        uint16x8_t avg = vhaddq_u16(s, r);
        int32x4_t d0_lo =
            vreinterpretq_s32_u32(vsubl_u16(vget_low_u16(avg), offset_vec));
        int32x4_t d0_hi =
            vreinterpretq_s32_u32(vsubl_u16(vget_high_u16(avg), offset_vec));

        uint16x8_t d0 = vcombine_u16(vqrshrun_n_s32(d0_lo, ROUND_SHIFT - 2),
                                     vqrshrun_n_s32(d0_hi, ROUND_SHIFT - 2));
        d0 = vminq_u16(d0, max);
        vst1q_u16(dst, d0);

        src += 8;
        ref += 8;
        dst += 8;
        width -= 8;
      } while (width != 0);

      src_ptr += src_stride;
      ref_ptr += ref_stride;
      dst_ptr += dst_stride;
    } while (--h != 0);
  }
}

static inline void highbd_comp_avg_neon(const uint16_t *src_ptr, int src_stride,
                                        uint16_t *dst_ptr, int dst_stride,
                                        int w, int h,
                                        ConvolveParams *conv_params,
                                        const int bd) {
  const int offset_bits = bd + 2 * FILTER_BITS - ROUND0_BITS;
  const int offset = (1 << (offset_bits - COMPOUND_ROUND1_BITS)) +
                     (1 << (offset_bits - COMPOUND_ROUND1_BITS - 1));

  CONV_BUF_TYPE *ref_ptr = conv_params->dst;
  const int ref_stride = conv_params->dst_stride;
  const uint16x4_t offset_vec = vdup_n_u16((uint16_t)offset);
  const uint16x8_t max = vdupq_n_u16((1 << bd) - 1);

  if (w == 4) {
    do {
      const uint16x4_t src = vld1_u16(src_ptr);
      const uint16x4_t ref = vld1_u16(ref_ptr);

      uint16x4_t avg = vhadd_u16(src, ref);
      int32x4_t d0 = vreinterpretq_s32_u32(vsubl_u16(avg, offset_vec));

      uint16x4_t d0_u16 = vqrshrun_n_s32(d0, ROUND_SHIFT);
      d0_u16 = vmin_u16(d0_u16, vget_low_u16(max));

      vst1_u16(dst_ptr, d0_u16);

      src_ptr += src_stride;
      ref_ptr += ref_stride;
      dst_ptr += dst_stride;
    } while (--h != 0);
  } else {
    do {
      int width = w;
      const uint16_t *src = src_ptr;
      const uint16_t *ref = ref_ptr;
      uint16_t *dst = dst_ptr;
      do {
        const uint16x8_t s = vld1q_u16(src);
        const uint16x8_t r = vld1q_u16(ref);

        uint16x8_t avg = vhaddq_u16(s, r);
        int32x4_t d0_lo =
            vreinterpretq_s32_u32(vsubl_u16(vget_low_u16(avg), offset_vec));
        int32x4_t d0_hi =
            vreinterpretq_s32_u32(vsubl_u16(vget_high_u16(avg), offset_vec));

        uint16x8_t d0 = vcombine_u16(vqrshrun_n_s32(d0_lo, ROUND_SHIFT),
                                     vqrshrun_n_s32(d0_hi, ROUND_SHIFT));
        d0 = vminq_u16(d0, max);
        vst1q_u16(dst, d0);

        src += 8;
        ref += 8;
        dst += 8;
        width -= 8;
      } while (width != 0);

      src_ptr += src_stride;
      ref_ptr += ref_stride;
      dst_ptr += dst_stride;
    } while (--h != 0);
  }
}

static inline void highbd_12_dist_wtd_comp_avg_neon(
    const uint16_t *src_ptr, int src_stride, uint16_t *dst_ptr, int dst_stride,
    int w, int h, ConvolveParams *conv_params) {
  const int offset_bits = 12 + 2 * FILTER_BITS - ROUND0_BITS - 2;
  const int offset = (1 << (offset_bits - COMPOUND_ROUND1_BITS)) +
                     (1 << (offset_bits - COMPOUND_ROUND1_BITS - 1));

  CONV_BUF_TYPE *ref_ptr = conv_params->dst;
  const int ref_stride = conv_params->dst_stride;
  const uint32x4_t offset_vec = vdupq_n_u32(offset);
  const uint16x8_t max = vdupq_n_u16((1 << 12) - 1);
  uint16x4_t fwd_offset = vdup_n_u16(conv_params->fwd_offset);
  uint16x4_t bck_offset = vdup_n_u16(conv_params->bck_offset);

  // Weighted averaging
  if (w == 4) {
    do {
      const uint16x4_t src = vld1_u16(src_ptr);
      const uint16x4_t ref = vld1_u16(ref_ptr);

      uint32x4_t wtd_avg = vmull_u16(ref, fwd_offset);
      wtd_avg = vmlal_u16(wtd_avg, src, bck_offset);
      wtd_avg = vshrq_n_u32(wtd_avg, DIST_PRECISION_BITS);
      int32x4_t d0 = vreinterpretq_s32_u32(vsubq_u32(wtd_avg, offset_vec));

      uint16x4_t d0_u16 = vqrshrun_n_s32(d0, ROUND_SHIFT - 2);
      d0_u16 = vmin_u16(d0_u16, vget_low_u16(max));

      vst1_u16(dst_ptr, d0_u16);

      src_ptr += src_stride;
      dst_ptr += dst_stride;
      ref_ptr += ref_stride;
    } while (--h != 0);
  } else {
    do {
      int width = w;
      const uint16_t *src = src_ptr;
      const uint16_t *ref = ref_ptr;
      uint16_t *dst = dst_ptr;
      do {
        const uint16x8_t s = vld1q_u16(src);
        const uint16x8_t r = vld1q_u16(ref);

        uint32x4_t wtd_avg0 = vmull_u16(vget_low_u16(r), fwd_offset);
        wtd_avg0 = vmlal_u16(wtd_avg0, vget_low_u16(s), bck_offset);
        wtd_avg0 = vshrq_n_u32(wtd_avg0, DIST_PRECISION_BITS);
        int32x4_t d0 = vreinterpretq_s32_u32(vsubq_u32(wtd_avg0, offset_vec));

        uint32x4_t wtd_avg1 = vmull_u16(vget_high_u16(r), fwd_offset);
        wtd_avg1 = vmlal_u16(wtd_avg1, vget_high_u16(s), bck_offset);
        wtd_avg1 = vshrq_n_u32(wtd_avg1, DIST_PRECISION_BITS);
        int32x4_t d1 = vreinterpretq_s32_u32(vsubq_u32(wtd_avg1, offset_vec));

        uint16x8_t d01 = vcombine_u16(vqrshrun_n_s32(d0, ROUND_SHIFT - 2),
                                      vqrshrun_n_s32(d1, ROUND_SHIFT - 2));
        d01 = vminq_u16(d01, max);
        vst1q_u16(dst, d01);

        src += 8;
        ref += 8;
        dst += 8;
        width -= 8;
      } while (width != 0);
      src_ptr += src_stride;
      dst_ptr += dst_stride;
      ref_ptr += ref_stride;
    } while (--h != 0);
  }
}

static inline void highbd_dist_wtd_comp_avg_neon(
    const uint16_t *src_ptr, int src_stride, uint16_t *dst_ptr, int dst_stride,
    int w, int h, ConvolveParams *conv_params, const int bd) {
  const int offset_bits = bd + 2 * FILTER_BITS - ROUND0_BITS;
  const int offset = (1 << (offset_bits - COMPOUND_ROUND1_BITS)) +
                     (1 << (offset_bits - COMPOUND_ROUND1_BITS - 1));

  CONV_BUF_TYPE *ref_ptr = conv_params->dst;
  const int ref_stride = conv_params->dst_stride;
  const uint32x4_t offset_vec = vdupq_n_u32(offset);
  const uint16x8_t max = vdupq_n_u16((1 << bd) - 1);
  uint16x4_t fwd_offset = vdup_n_u16(conv_params->fwd_offset);
  uint16x4_t bck_offset = vdup_n_u16(conv_params->bck_offset);

  // Weighted averaging
  if (w == 4) {
    do {
      const uint16x4_t src = vld1_u16(src_ptr);
      const uint16x4_t ref = vld1_u16(ref_ptr);

      uint32x4_t wtd_avg = vmull_u16(ref, fwd_offset);
      wtd_avg = vmlal_u16(wtd_avg, src, bck_offset);
      wtd_avg = vshrq_n_u32(wtd_avg, DIST_PRECISION_BITS);
      int32x4_t d0 = vreinterpretq_s32_u32(vsubq_u32(wtd_avg, offset_vec));

      uint16x4_t d0_u16 = vqrshrun_n_s32(d0, ROUND_SHIFT);
      d0_u16 = vmin_u16(d0_u16, vget_low_u16(max));

      vst1_u16(dst_ptr, d0_u16);

      src_ptr += src_stride;
      dst_ptr += dst_stride;
      ref_ptr += ref_stride;
    } while (--h != 0);
  } else {
    do {
      int width = w;
      const uint16_t *src = src_ptr;
      const uint16_t *ref = ref_ptr;
      uint16_t *dst = dst_ptr;
      do {
        const uint16x8_t s = vld1q_u16(src);
        const uint16x8_t r = vld1q_u16(ref);

        uint32x4_t wtd_avg0 = vmull_u16(vget_low_u16(r), fwd_offset);
        wtd_avg0 = vmlal_u16(wtd_avg0, vget_low_u16(s), bck_offset);
        wtd_avg0 = vshrq_n_u32(wtd_avg0, DIST_PRECISION_BITS);
        int32x4_t d0 = vreinterpretq_s32_u32(vsubq_u32(wtd_avg0, offset_vec));

        uint32x4_t wtd_avg1 = vmull_u16(vget_high_u16(r), fwd_offset);
        wtd_avg1 = vmlal_u16(wtd_avg1, vget_high_u16(s), bck_offset);
        wtd_avg1 = vshrq_n_u32(wtd_avg1, DIST_PRECISION_BITS);
        int32x4_t d1 = vreinterpretq_s32_u32(vsubq_u32(wtd_avg1, offset_vec));

        uint16x8_t d01 = vcombine_u16(vqrshrun_n_s32(d0, ROUND_SHIFT),
                                      vqrshrun_n_s32(d1, ROUND_SHIFT));
        d01 = vminq_u16(d01, max);
        vst1q_u16(dst, d01);

        src += 8;
        ref += 8;
        dst += 8;
        width -= 8;
      } while (width != 0);
      src_ptr += src_stride;
      dst_ptr += dst_stride;
      ref_ptr += ref_stride;
    } while (--h != 0);
  }
}
