/*
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

#ifndef AVCODEC_IDCTDSP_H
#define AVCODEC_IDCTDSP_H

#include <stddef.h>
#include <stdint.h>

struct AVCodecContext;

enum idct_permutation_type {
    FF_IDCT_PERM_NONE,
    FF_IDCT_PERM_LIBMPEG2,
    FF_IDCT_PERM_SIMPLE,
    FF_IDCT_PERM_TRANSPOSE,
    FF_IDCT_PERM_PARTTRANS,
    FF_IDCT_PERM_SSE2,
};

void ff_permute_scantable(uint8_t dst[64], const uint8_t src[64],
                          const uint8_t permutation[64]);
void ff_init_scantable_permutation(uint8_t *idct_permutation,
                                   enum idct_permutation_type perm_type);
int ff_init_scantable_permutation_x86(uint8_t *idct_permutation,
                                      enum idct_permutation_type perm_type);

typedef struct IDCTDSPContext {
    /* pixel ops : interface with DCT */
    void (*put_pixels_clamped)(const int16_t *block /* align 16 */,
                               uint8_t *restrict pixels /* align 8 */,
                               ptrdiff_t line_size);
    void (*put_signed_pixels_clamped)(const int16_t *block /* align 16 */,
                                      uint8_t *restrict pixels /* align 8 */,
                                      ptrdiff_t line_size);
    void (*add_pixels_clamped)(const int16_t *block /* align 16 */,
                               uint8_t *restrict pixels /* align 8 */,
                               ptrdiff_t line_size);

    void (*idct)(int16_t *block /* align 16 */);

    /**
     * block -> idct -> clip to unsigned 8 bit -> dest.
     * (-1392, 0, 0, ...) -> idct -> (-174, -174, ...) -> put -> (0, 0, ...)
     * @param line_size size in bytes of a horizontal line of dest
     */
    void (*idct_put)(uint8_t *dest /* align 8 */,
                     ptrdiff_t line_size, int16_t *block /* align 16 */);

    /**
     * block -> idct -> add dest -> clip to unsigned 8 bit -> dest.
     * @param line_size size in bytes of a horizontal line of dest
     */
    void (*idct_add)(uint8_t *dest /* align 8 */,
                     ptrdiff_t line_size, int16_t *block /* align 16 */);

    /**
     * IDCT input permutation.
     * Several optimized IDCTs need a permutated input (relative to the
     * normal order of the reference IDCT).
     * This permutation must be performed before the idct_put/add.
     * Note, normally this can be merged with the zigzag/alternate scan<br>
     * An example to avoid confusion:
     * - (->decode coeffs -> zigzag reorder -> dequant -> reference IDCT -> ...)
     * - (x -> reference DCT -> reference IDCT -> x)
     * - (x -> reference DCT -> simple_mmx_perm = idct_permutation
     *    -> simple_idct_mmx -> x)
     * - (-> decode coeffs -> zigzag reorder -> simple_mmx_perm -> dequant
     *    -> simple_idct_mmx -> ...)
     */
    uint8_t idct_permutation[64];
    enum idct_permutation_type perm_type;

    int mpeg4_studio_profile;
} IDCTDSPContext;

void ff_put_pixels_clamped_c(const int16_t *block, uint8_t *restrict pixels,
                             ptrdiff_t line_size);
void ff_add_pixels_clamped_c(const int16_t *block, uint8_t *restrict pixels,
                             ptrdiff_t line_size);

void ff_idctdsp_init(IDCTDSPContext *c, struct AVCodecContext *avctx);

void ff_idctdsp_init_aarch64(IDCTDSPContext *c, struct AVCodecContext *avctx,
                             unsigned high_bit_depth);
void ff_idctdsp_init_arm(IDCTDSPContext *c, struct AVCodecContext *avctx,
                         unsigned high_bit_depth);
void ff_idctdsp_init_ppc(IDCTDSPContext *c, struct AVCodecContext *avctx,
                         unsigned high_bit_depth);
void ff_idctdsp_init_riscv(IDCTDSPContext *c, struct AVCodecContext *avctx,
                           unsigned high_bit_depth);
void ff_idctdsp_init_x86(IDCTDSPContext *c, struct AVCodecContext *avctx,
                         unsigned high_bit_depth);
void ff_idctdsp_init_mips(IDCTDSPContext *c, struct AVCodecContext *avctx,
                          unsigned high_bit_depth);
void ff_idctdsp_init_loongarch(IDCTDSPContext *c, struct AVCodecContext *avctx,
                               unsigned high_bit_depth);

#endif /* AVCODEC_IDCTDSP_H */
