/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_X86_MASKED_SAD_INTRIN_SSSE3_H_
#define AOM_AOM_DSP_X86_MASKED_SAD_INTRIN_SSSE3_H_

unsigned int aom_masked_sad8xh_ssse3(const uint8_t *src_ptr, int src_stride,
                                     const uint8_t *a_ptr, int a_stride,
                                     const uint8_t *b_ptr, int b_stride,
                                     const uint8_t *m_ptr, int m_stride,
                                     int height);

unsigned int aom_masked_sad4xh_ssse3(const uint8_t *src_ptr, int src_stride,
                                     const uint8_t *a_ptr, int a_stride,
                                     const uint8_t *b_ptr, int b_stride,
                                     const uint8_t *m_ptr, int m_stride,
                                     int height);

unsigned int aom_highbd_masked_sad4xh_ssse3(const uint8_t *src8, int src_stride,
                                            const uint8_t *a8, int a_stride,
                                            const uint8_t *b8, int b_stride,
                                            const uint8_t *m_ptr, int m_stride,
                                            int height);

#endif  // AOM_AOM_DSP_X86_MASKED_SAD_INTRIN_SSSE3_H_
