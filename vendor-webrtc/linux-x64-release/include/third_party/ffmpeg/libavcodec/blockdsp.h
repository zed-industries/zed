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

#ifndef AVCODEC_BLOCKDSP_H
#define AVCODEC_BLOCKDSP_H

#include <stddef.h>
#include <stdint.h>

/* add and put pixel (decoding)
 * Block sizes for op_pixels_func are 8x4,8x8 16x8 16x16.
 * h for op_pixels_func is limited to { width / 2, width },
 * but never larger than 16 and never smaller than 4. */
typedef void (*op_fill_func)(uint8_t *block /* align width (8 or 16) */,
                             uint8_t value, ptrdiff_t line_size, int h);

typedef struct BlockDSPContext {
    void (*clear_block)(int16_t *block /* align 32 */);
    void (*clear_blocks)(int16_t *blocks /* align 32 */);

    op_fill_func fill_block_tab[2];
} BlockDSPContext;

void ff_blockdsp_init(BlockDSPContext *c);

void ff_blockdsp_init_arm(BlockDSPContext *c);
void ff_blockdsp_init_ppc(BlockDSPContext *c);
void ff_blockdsp_init_riscv(BlockDSPContext *c);
void ff_blockdsp_init_x86(BlockDSPContext *c);
void ff_blockdsp_init_mips(BlockDSPContext *c);

#endif /* AVCODEC_BLOCKDSP_H */
