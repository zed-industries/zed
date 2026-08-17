/*
 * Copyright © 2018-2021, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef DAV1D_SRC_RECON_H
#define DAV1D_SRC_RECON_H

#include "src/internal.h"
#include "src/levels.h"

#define DEBUG_BLOCK_INFO 0 && \
        f->frame_hdr->frame_offset == 2 && t->by >= 0 && t->by < 4 && \
        t->bx >= 8 && t->bx < 12
#define DEBUG_B_PIXELS 0

#define decl_recon_b_intra_fn(name) \
void (name)(Dav1dTaskContext *t, enum BlockSize bs, \
            enum EdgeFlags intra_edge_flags, const Av1Block *b)
typedef decl_recon_b_intra_fn(*recon_b_intra_fn);

#define decl_recon_b_inter_fn(name) \
int (name)(Dav1dTaskContext *t, enum BlockSize bs, const Av1Block *b)
typedef decl_recon_b_inter_fn(*recon_b_inter_fn);

#define decl_filter_sbrow_fn(name) \
void (name)(Dav1dFrameContext *f, int sby)
typedef decl_filter_sbrow_fn(*filter_sbrow_fn);

#define decl_backup_ipred_edge_fn(name) \
void (name)(Dav1dTaskContext *t)
typedef decl_backup_ipred_edge_fn(*backup_ipred_edge_fn);

#define decl_read_coef_blocks_fn(name) \
void (name)(Dav1dTaskContext *t, enum BlockSize bs, const Av1Block *b)
typedef decl_read_coef_blocks_fn(*read_coef_blocks_fn);

#define decl_copy_pal_block_fn(name) \
void (name)(Dav1dTaskContext *t, int bx4, int by4, int bw4, int bh4)
typedef decl_copy_pal_block_fn(*copy_pal_block_fn);

#define decl_read_pal_plane_fn(name) \
void (name)(Dav1dTaskContext *t, Av1Block *b, int pl, int sz_ctx, int bx4, int by4)
typedef decl_read_pal_plane_fn(*read_pal_plane_fn);

#define decl_read_pal_uv_fn(name) \
void (name)(Dav1dTaskContext *t, Av1Block *b, int sz_ctx, int bx4, int by4)
typedef decl_read_pal_uv_fn(*read_pal_uv_fn);

decl_recon_b_intra_fn(dav1d_recon_b_intra_8bpc);
decl_recon_b_intra_fn(dav1d_recon_b_intra_16bpc);

decl_recon_b_inter_fn(dav1d_recon_b_inter_8bpc);
decl_recon_b_inter_fn(dav1d_recon_b_inter_16bpc);

decl_filter_sbrow_fn(dav1d_filter_sbrow_8bpc);
decl_filter_sbrow_fn(dav1d_filter_sbrow_16bpc);
decl_filter_sbrow_fn(dav1d_filter_sbrow_deblock_cols_8bpc);
decl_filter_sbrow_fn(dav1d_filter_sbrow_deblock_cols_16bpc);
decl_filter_sbrow_fn(dav1d_filter_sbrow_deblock_rows_8bpc);
decl_filter_sbrow_fn(dav1d_filter_sbrow_deblock_rows_16bpc);
void dav1d_filter_sbrow_cdef_8bpc(Dav1dTaskContext *tc, int sby);
void dav1d_filter_sbrow_cdef_16bpc(Dav1dTaskContext *tc, int sby);
decl_filter_sbrow_fn(dav1d_filter_sbrow_resize_8bpc);
decl_filter_sbrow_fn(dav1d_filter_sbrow_resize_16bpc);
decl_filter_sbrow_fn(dav1d_filter_sbrow_lr_8bpc);
decl_filter_sbrow_fn(dav1d_filter_sbrow_lr_16bpc);

decl_backup_ipred_edge_fn(dav1d_backup_ipred_edge_8bpc);
decl_backup_ipred_edge_fn(dav1d_backup_ipred_edge_16bpc);

decl_read_coef_blocks_fn(dav1d_read_coef_blocks_8bpc);
decl_read_coef_blocks_fn(dav1d_read_coef_blocks_16bpc);

decl_copy_pal_block_fn(dav1d_copy_pal_block_y_8bpc);
decl_copy_pal_block_fn(dav1d_copy_pal_block_y_16bpc);
decl_copy_pal_block_fn(dav1d_copy_pal_block_uv_8bpc);
decl_copy_pal_block_fn(dav1d_copy_pal_block_uv_16bpc);
decl_read_pal_plane_fn(dav1d_read_pal_plane_8bpc);
decl_read_pal_plane_fn(dav1d_read_pal_plane_16bpc);
decl_read_pal_uv_fn(dav1d_read_pal_uv_8bpc);
decl_read_pal_uv_fn(dav1d_read_pal_uv_16bpc);

#endif /* DAV1D_SRC_RECON_H */
