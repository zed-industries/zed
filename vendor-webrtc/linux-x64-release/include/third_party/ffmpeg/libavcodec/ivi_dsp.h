/*
 * DSP functions for Indeo Video Interactive codecs (Indeo4 and Indeo5)
 *
 * Copyright (c) 2009-2011 Maxim Poliakovski
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

/**
 * @file
 * DSP functions (inverse transforms, motion compensations, wavelet recomposition)
 * for Indeo Video Interactive codecs.
 */

#ifndef AVCODEC_IVI_DSP_H
#define AVCODEC_IVI_DSP_H

#include <stddef.h>
#include <stdint.h>

#include "ivi.h"

/**
 *  5/3 wavelet recomposition filter for Indeo5
 *
 *  @param[in]   plane        pointer to the descriptor of the plane being processed
 *  @param[out]  dst          pointer to the destination buffer
 *  @param[in]   dst_pitch    pitch of the destination buffer
 */
void ff_ivi_recompose53(const IVIPlaneDesc *plane, uint8_t *dst,
                        const ptrdiff_t dst_pitch);

/**
 *  Haar wavelet recomposition filter for Indeo 4
 *
 *  @param[in]  plane        pointer to the descriptor of the plane being processed
 *  @param[out] dst          pointer to the destination buffer
 *  @param[in]  dst_pitch    pitch of the destination buffer
 */
void ff_ivi_recompose_haar(const IVIPlaneDesc *plane, uint8_t *dst,
                           const ptrdiff_t dst_pitch);

/**
 *  two-dimensional inverse Haar 8x8 transform for Indeo 4
 *
 *  @param[in]  in        pointer to the vector of transform coefficients
 *  @param[out] out       pointer to the output buffer (frame)
 *  @param[in]  pitch     pitch to move to the next y line
 *  @param[in]  flags     pointer to the array of column flags:
 *                        != 0 - non_empty column, 0 - empty one
 *                        (this array must be filled by caller)
 */
void ff_ivi_inverse_haar_8x8(const int32_t *in, int16_t *out, ptrdiff_t pitch,
                             const uint8_t *flags);
void ff_ivi_inverse_haar_8x1(const int32_t *in, int16_t *out, uint32_t pitch,
                             const uint8_t *flags);
void ff_ivi_inverse_haar_1x8(const int32_t *in, int16_t *out, uint32_t pitch,
                             const uint8_t *flags);

/**
 *  one-dimensional inverse 8-point Haar transform on rows for Indeo 4
 *
 *  @param[in]  in        pointer to the vector of transform coefficients
 *  @param[out] out       pointer to the output buffer (frame)
 *  @param[in]  pitch     pitch to move to the next y line
 *  @param[in]  flags     pointer to the array of column flags:
 *                        != 0 - non_empty column, 0 - empty one
 *                        (this array must be filled by caller)
 */
void ff_ivi_row_haar8(const int32_t *in, int16_t *out, ptrdiff_t pitch,
                      const uint8_t *flags);

/**
 *  one-dimensional inverse 8-point Haar transform on columns for Indeo 4
 *
 *  @param[in]  in        pointer to the vector of transform coefficients
 *  @param[out] out       pointer to the output buffer (frame)
 *  @param[in]  pitch     pitch to move to the next y line
 *  @param[in]  flags     pointer to the array of column flags:
 *                        != 0 - non_empty column, 0 - empty one
 *                        (this array must be filled by caller)
 */
void ff_ivi_col_haar8(const int32_t *in, int16_t *out, ptrdiff_t pitch,
                      const uint8_t *flags);

/**
 *  two-dimensional inverse Haar 4x4 transform for Indeo 4
 *
 *  @param[in]  in        pointer to the vector of transform coefficients
 *  @param[out] out       pointer to the output buffer (frame)
 *  @param[in]  pitch     pitch to move to the next y line
 *  @param[in]  flags     pointer to the array of column flags:
 *                        != 0 - non_empty column, 0 - empty one
 *                        (this array must be filled by caller)
 */
void ff_ivi_inverse_haar_4x4(const int32_t *in, int16_t *out, ptrdiff_t pitch,
                             const uint8_t *flags);

/**
 *  one-dimensional inverse 4-point Haar transform on rows for Indeo 4
 *
 *  @param[in]  in        pointer to the vector of transform coefficients
 *  @param[out] out       pointer to the output buffer (frame)
 *  @param[in]  pitch     pitch to move to the next y line
 *  @param[in]  flags     pointer to the array of column flags:
 *                        != 0 - non_empty column, 0 - empty one
 *                        (this array must be filled by caller)
 */
void ff_ivi_row_haar4(const int32_t *in, int16_t *out, ptrdiff_t pitch,
                      const uint8_t *flags);

/**
 *  one-dimensional inverse 4-point Haar transform on columns for Indeo 4
 *
 *  @param[in]  in        pointer to the vector of transform coefficients
 *  @param[out] out       pointer to the output buffer (frame)
 *  @param[in]  pitch     pitch to move to the next y line
 *  @param[in]  flags     pointer to the array of column flags:
 *                        != 0 - non_empty column, 0 - empty one
 *                        (this array must be filled by caller)
 */
void ff_ivi_col_haar4(const int32_t *in, int16_t *out, ptrdiff_t pitch,
                      const uint8_t *flags);

/**
 *  DC-only two-dimensional inverse Haar transform for Indeo 4.
 *  Performing the inverse transform in this case is equivalent to
 *  spreading DC_coeff >> 3 over the whole block.
 *
 *  @param[in]  in          pointer to the dc coefficient
 *  @param[out] out         pointer to the output buffer (frame)
 *  @param[in]  pitch       pitch to move to the next y line
 *  @param[in]  blk_size    transform block size
 */
void ff_ivi_dc_haar_2d(const int32_t *in, int16_t *out, ptrdiff_t pitch,
                       int blk_size);

/**
 *  two-dimensional inverse slant 8x8 transform
 *
 *  @param[in]    in      pointer to the vector of transform coefficients
 *  @param[out]   out     pointer to the output buffer (frame)
 *  @param[in]    pitch   pitch to move to the next y line
 *  @param[in]    flags   pointer to the array of column flags:
 *                        != 0 - non_empty column, 0 - empty one
 *                        (this array must be filled by caller)
 */
void ff_ivi_inverse_slant_8x8(const int32_t *in, int16_t *out, ptrdiff_t pitch,
                              const uint8_t *flags);

/**
 *  two-dimensional inverse slant 4x4 transform
 *
 *  @param[in]    in      pointer to the vector of transform coefficients
 *  @param[out]   out     pointer to the output buffer (frame)
 *  @param[in]    pitch   pitch to move to the next y line
 *  @param[in]    flags   pointer to the array of column flags:
 *                        != 0 - non_empty column, 0 - empty one
 *                        (this array must be filled by caller)
 */
void ff_ivi_inverse_slant_4x4(const int32_t *in, int16_t *out, ptrdiff_t pitch,
                              const uint8_t *flags);

/**
 *  DC-only two-dimensional inverse slant transform.
 *  Performing the inverse slant transform in this case is equivalent to
 *  spreading (DC_coeff + 1)/2 over the whole block.
 *  It works much faster than performing the slant transform on a vector of zeroes.
 *
 *  @param[in]    in          pointer to the dc coefficient
 *  @param[out]   out         pointer to the output buffer (frame)
 *  @param[in]    pitch       pitch to move to the next y line
 *  @param[in]    blk_size    transform block size
 */
void ff_ivi_dc_slant_2d(const int32_t *in, int16_t *out, ptrdiff_t pitch, int blk_size);

/**
 *  inverse 1D row slant transform
 *
 *  @param[in]    in      pointer to the vector of transform coefficients
 *  @param[out]   out     pointer to the output buffer (frame)
 *  @param[in]    pitch   pitch to move to the next y line
 *  @param[in]    flags   pointer to the array of column flags (unused here)
 */
void ff_ivi_row_slant8(const int32_t *in, int16_t *out, ptrdiff_t pitch,
                       const uint8_t *flags);

/**
 *  inverse 1D column slant transform
 *
 *  @param[in]    in      pointer to the vector of transform coefficients
 *  @param[out]   out     pointer to the output buffer (frame)
 *  @param[in]    pitch   pitch to move to the next y line
 *  @param[in]    flags   pointer to the array of column flags:
 *                        != 0 - non_empty column, 0 - empty one
 *                        (this array must be filled by caller)
 */
void ff_ivi_col_slant8(const int32_t *in, int16_t *out, ptrdiff_t pitch,
                       const uint8_t *flags);

/**
 *  inverse 1D row slant transform
 *
 *  @param[in]    in      pointer to the vector of transform coefficients
 *  @param[out]   out     pointer to the output buffer (frame)
 *  @param[in]    pitch   pitch to move to the next y line
 *  @param[in]    flags   pointer to the array of column flags (unused here)
 */
void ff_ivi_row_slant4(const int32_t *in, int16_t *out, ptrdiff_t pitch,
                       const uint8_t *flags);

/**
 *  inverse 1D column slant transform
 *
 *  @param[in]    in      pointer to the vector of transform coefficients
 *  @param[out]   out     pointer to the output buffer (frame)
 *  @param[in]    pitch   pitch to move to the next y line
 *  @param[in]    flags   pointer to the array of column flags:
 *                        != 0 - non_empty column, 0 - empty one
 *                        (this array must be filled by caller)
 */
void ff_ivi_col_slant4(const int32_t *in, int16_t *out, ptrdiff_t pitch,
                       const uint8_t *flags);

/**
 *  DC-only inverse row slant transform
 */
void ff_ivi_dc_row_slant(const int32_t *in, int16_t *out, ptrdiff_t pitch, int blk_size);

/**
 *  DC-only inverse column slant transform
 */
void ff_ivi_dc_col_slant(const int32_t *in, int16_t *out, ptrdiff_t pitch, int blk_size);

/**
 *  Copy the pixels into the frame buffer.
 */
void ff_ivi_put_pixels_8x8(const int32_t *in, int16_t *out, ptrdiff_t pitch, const uint8_t *flags);

/**
 *  Copy the DC coefficient into the first pixel of the block and
 *  zero all others.
 */
void ff_ivi_put_dc_pixel_8x8(const int32_t *in, int16_t *out, ptrdiff_t pitch, int blk_size);

/**
 *  8x8 block motion compensation with adding delta
 *
 *  @param[in,out]   buf      pointer to the block in the current frame buffer containing delta
 *  @param[in]       ref_buf  pointer to the corresponding block in the reference frame
 *  @param[in]       pitch    pitch for moving to the next y line
 *  @param[in]       mc_type  interpolation type
 */
void ff_ivi_mc_8x8_delta(int16_t *buf, const int16_t *ref_buf, ptrdiff_t pitch, int mc_type);

/**
 *  4x4 block motion compensation with adding delta
 *
 *  @param[in,out]   buf      pointer to the block in the current frame buffer containing delta
 *  @param[in]       ref_buf  pointer to the corresponding block in the reference frame
 *  @param[in]       pitch    pitch for moving to the next y line
 *  @param[in]       mc_type  interpolation type
 */
void ff_ivi_mc_4x4_delta(int16_t *buf, const int16_t *ref_buf, ptrdiff_t pitch, int mc_type);

/**
 *  motion compensation without adding delta
 *
 *  @param[in,out]  buf      pointer to the block in the current frame receiving the result
 *  @param[in]      ref_buf  pointer to the corresponding block in the reference frame
 *  @param[in]      pitch    pitch for moving to the next y line
 *  @param[in]      mc_type  interpolation type
 */
void ff_ivi_mc_8x8_no_delta(int16_t *buf, const int16_t *ref_buf, ptrdiff_t pitch, int mc_type);

/**
 *  4x4 block motion compensation without adding delta
 *
 *  @param[in,out]  buf      pointer to the block in the current frame receiving the result
 *  @param[in]      ref_buf  pointer to the corresponding block in the reference frame
 *  @param[in]      pitch    pitch for moving to the next y line
 *  @param[in]      mc_type  interpolation type
 */
void ff_ivi_mc_4x4_no_delta(int16_t *buf, const int16_t *ref_buf, ptrdiff_t pitch, int mc_type);

/**
 *  8x8 block motion compensation with adding delta
 *
 *  @param[in,out]  buf      pointer to the block in the current frame buffer containing delta
 *  @param[in]      ref_buf  pointer to the corresponding block in the backward reference frame
 *  @param[in]      ref_buf2 pointer to the corresponding block in the forward reference frame
 *  @param[in]      pitch    pitch for moving to the next y line
 *  @param[in]      mc_type  interpolation type for backward reference
 *  @param[in]      mc_type2 interpolation type for forward reference
 */
void ff_ivi_mc_avg_8x8_delta(int16_t *buf, const int16_t *ref_buf, const int16_t *ref_buf2, ptrdiff_t pitch, int mc_type, int mc_type2);

/**
 *  4x4 block motion compensation with adding delta
 *
 *  @param[in,out]  buf      pointer to the block in the current frame buffer containing delta
 *  @param[in]      ref_buf  pointer to the corresponding block in the backward reference frame
 *  @param[in]      ref_buf2 pointer to the corresponding block in the forward reference frame
 *  @param[in]      pitch    pitch for moving to the next y line
 *  @param[in]      mc_type  interpolation type for backward reference
 *  @param[in]      mc_type2 interpolation type for forward reference
 */
void ff_ivi_mc_avg_4x4_delta(int16_t *buf, const int16_t *ref_buf, const int16_t *ref_buf2, ptrdiff_t pitch, int mc_type, int mc_type2);

/**
 *  motion compensation without adding delta for B-frames
 *
 *  @param[in,out]  buf      pointer to the block in the current frame receiving the result
 *  @param[in]      ref_buf  pointer to the corresponding block in the backward reference frame
 *  @param[in]      ref_buf2 pointer to the corresponding block in the forward reference frame
 *  @param[in]      pitch    pitch for moving to the next y line
 *  @param[in]      mc_type  interpolation type for backward reference
 *  @param[in]      mc_type2 interpolation type for forward reference
 */
void ff_ivi_mc_avg_8x8_no_delta(int16_t *buf, const int16_t *ref_buf, const int16_t *ref_buf2, ptrdiff_t pitch, int mc_type, int mc_type2);

/**
 *  4x4 block motion compensation without adding delta for B-frames
 *
 *  @param[in,out]  buf      pointer to the block in the current frame receiving the result
 *  @param[in]      ref_buf  pointer to the corresponding block in the backward reference frame
 *  @param[in]      ref_buf2 pointer to the corresponding block in the forward reference frame
 *  @param[in]      pitch    pitch for moving to the next y line
 *  @param[in]      mc_type  interpolation type for backward reference
 *  @param[in]      mc_type2 interpolation type for forward reference
 */
void ff_ivi_mc_avg_4x4_no_delta(int16_t *buf, const int16_t *ref_buf, const int16_t *ref_buf2, ptrdiff_t pitch, int mc_type, int mc_type2);

#endif /* AVCODEC_IVI_DSP_H */
