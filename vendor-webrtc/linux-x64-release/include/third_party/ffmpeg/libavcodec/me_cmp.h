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

#ifndef AVCODEC_ME_CMP_H
#define AVCODEC_ME_CMP_H

#include <stdint.h>

#include "libavutil/attributes_internal.h"

#include "avcodec.h"

EXTERN const uint32_t ff_square_tab[512];


/* minimum alignment rules ;)
 * If you notice errors in the align stuff, need more alignment for some ASM code
 * for some CPU or need to use a function with less aligned data then send a mail
 * to the ffmpeg-devel mailing list, ...
 *
 * !warning These alignments might not match reality, (missing attribute((align))
 * stuff somewhere possible).
 * I (Michael) did not check them, these are just the alignments which I think
 * could be reached easily ...
 *
 * !future video codecs might need functions with less strict alignment
 */

struct MpegEncContext;
/* Motion estimation:
 * h is limited to { width / 2, width, 2 * width },
 * but never larger than 16 and never smaller than 2.
 * Although currently h < 4 is not used as functions with
 * width < 8 are neither used nor implemented. */
typedef int (*me_cmp_func)(struct MpegEncContext *c,
                           const uint8_t *blk1 /* align width (8 or 16) */,
                           const uint8_t *blk2 /* align 1 */, ptrdiff_t stride,
                           int h);

typedef struct MECmpContext {
    int (*sum_abs_dctelem)(const int16_t *block /* align 16 */);

    me_cmp_func sad[6]; /* identical to pix_absAxA except additional void * */
    me_cmp_func sse[6];
    me_cmp_func hadamard8_diff[6];
    me_cmp_func dct_sad[6];
    me_cmp_func quant_psnr[6];
    me_cmp_func bit[6];
    me_cmp_func rd[6];
    me_cmp_func vsad[6];
    me_cmp_func vsse[6];
    me_cmp_func nsse[6];
    me_cmp_func w53[6];
    me_cmp_func w97[6];
    me_cmp_func dct_max[6];
    me_cmp_func dct264_sad[6];

    me_cmp_func pix_abs[2][4];
    me_cmp_func median_sad[6];
} MECmpContext;

void ff_me_cmp_init(MECmpContext *c, AVCodecContext *avctx);
void ff_me_cmp_init_aarch64(MECmpContext *c, AVCodecContext *avctx);
void ff_me_cmp_init_arm(MECmpContext *c, AVCodecContext *avctx);
void ff_me_cmp_init_ppc(MECmpContext *c, AVCodecContext *avctx);
void ff_me_cmp_init_riscv(MECmpContext *c, AVCodecContext *avctx);
void ff_me_cmp_init_x86(MECmpContext *c, AVCodecContext *avctx);
void ff_me_cmp_init_mips(MECmpContext *c, AVCodecContext *avctx);

/**
 * Fill the function pointer array cmp[6] with me_cmp_funcs from
 * c based upon type. If mpvenc is not set, an error is returned
 * if the type of comparison functions requires an initialized
 * MpegEncContext.
 */
int ff_set_cmp(const MECmpContext *c, me_cmp_func *cmp,
               int type, int mpvenc);

void ff_dsputil_init_dwt(MECmpContext *c);

#endif /* AVCODEC_ME_CMP_H */
