/*
 * Copyright © 2018, VideoLAN and dav1d authors
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

#ifndef DAV1D_SRC_ENV_H
#define DAV1D_SRC_ENV_H

#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#include "src/levels.h"
#include "src/refmvs.h"
#include "src/tables.h"

typedef struct BlockContext {
    uint8_t ALIGN(mode[32], 8);
    uint8_t ALIGN(lcoef[32], 8);
    uint8_t ALIGN(ccoef[2][32], 8);
    uint8_t ALIGN(seg_pred[32], 8);
    uint8_t ALIGN(skip[32], 8);
    uint8_t ALIGN(skip_mode[32], 8);
    uint8_t ALIGN(intra[32], 8);
    uint8_t ALIGN(comp_type[32], 8);
    int8_t ALIGN(ref[2][32], 8); // -1 means intra
    uint8_t ALIGN(filter[2][32], 8); // 3 means unset
    int8_t ALIGN(tx_intra[32], 8);
    int8_t ALIGN(tx[32], 8);
    uint8_t ALIGN(tx_lpf_y[32], 8);
    uint8_t ALIGN(tx_lpf_uv[32], 8);
    uint8_t ALIGN(partition[16], 8);
    uint8_t ALIGN(uvmode[32], 8);
    uint8_t ALIGN(pal_sz[32], 8);
} BlockContext;

static inline int get_intra_ctx(const BlockContext *const a,
                                const BlockContext *const l,
                                const int yb4, const int xb4,
                                const int have_top, const int have_left)
{
    if (have_left) {
        if (have_top) {
            const int ctx = l->intra[yb4] + a->intra[xb4];
            return ctx + (ctx == 2);
        } else
            return l->intra[yb4] * 2;
    } else {
        return have_top ? a->intra[xb4] * 2 : 0;
    }
}

static inline int get_tx_ctx(const BlockContext *const a,
                             const BlockContext *const l,
                             const TxfmInfo *const max_tx,
                             const int yb4, const int xb4)
{
    return (l->tx_intra[yb4] >= max_tx->lh) + (a->tx_intra[xb4] >= max_tx->lw);
}

static inline int get_partition_ctx(const BlockContext *const a,
                                    const BlockContext *const l,
                                    const enum BlockLevel bl,
                                    const int yb8, const int xb8)
{
    return ((a->partition[xb8] >> (4 - bl)) & 1) +
          (((l->partition[yb8] >> (4 - bl)) & 1) << 1);
}

static inline unsigned gather_left_partition_prob(const uint16_t *const in,
                                                  const enum BlockLevel bl)
{
    unsigned out = in[PARTITION_H - 1] - in[PARTITION_H];
    // Exploit the fact that cdfs for PARTITION_SPLIT, PARTITION_T_TOP_SPLIT,
    // PARTITION_T_BOTTOM_SPLIT and PARTITION_T_LEFT_SPLIT are neighbors.
    out += in[PARTITION_SPLIT - 1] - in[PARTITION_T_LEFT_SPLIT];
    if (bl != BL_128X128)
        out += in[PARTITION_H4 - 1] - in[PARTITION_H4];
    return out;
}

static inline unsigned gather_top_partition_prob(const uint16_t *const in,
                                                 const enum BlockLevel bl)
{
    // Exploit the fact that cdfs for PARTITION_V, PARTITION_SPLIT and
    // PARTITION_T_TOP_SPLIT are neighbors.
    unsigned out = in[PARTITION_V - 1] - in[PARTITION_T_TOP_SPLIT];
    // Exploit the facts that cdfs for PARTITION_T_LEFT_SPLIT and
    // PARTITION_T_RIGHT_SPLIT are neighbors, the probability for
    // PARTITION_V4 is always zero, and the probability for
    // PARTITION_T_RIGHT_SPLIT is zero in 128x128 blocks.
    out += in[PARTITION_T_LEFT_SPLIT - 1];
    if (bl != BL_128X128)
        out += in[PARTITION_V4 - 1] - in[PARTITION_T_RIGHT_SPLIT];
    return out;
}

static inline enum TxfmType get_uv_inter_txtp(const TxfmInfo *const uvt_dim,
                                              const enum TxfmType ytxtp)
{
    if (uvt_dim->max == TX_32X32)
        return ytxtp == IDTX ? IDTX : DCT_DCT;
    if (uvt_dim->min == TX_16X16 &&
        ((1 << ytxtp) & ((1 << H_FLIPADST) | (1 << V_FLIPADST) |
                         (1 << H_ADST) | (1 << V_ADST))))
    {
        return DCT_DCT;
    }

    return ytxtp;
}

static inline int get_filter_ctx(const BlockContext *const a,
                                 const BlockContext *const l,
                                 const int comp, const int dir, const int ref,
                                 const int yb4, const int xb4)
{
    const int a_filter = (a->ref[0][xb4] == ref || a->ref[1][xb4] == ref) ?
                         a->filter[dir][xb4] : DAV1D_N_SWITCHABLE_FILTERS;
    const int l_filter = (l->ref[0][yb4] == ref || l->ref[1][yb4] == ref) ?
                         l->filter[dir][yb4] : DAV1D_N_SWITCHABLE_FILTERS;

    if (a_filter == l_filter) {
        return comp * 4 + a_filter;
    } else if (a_filter == DAV1D_N_SWITCHABLE_FILTERS) {
        return comp * 4 + l_filter;
    } else if (l_filter == DAV1D_N_SWITCHABLE_FILTERS) {
        return comp * 4 + a_filter;
    } else {
        return comp * 4 + DAV1D_N_SWITCHABLE_FILTERS;
    }
}

static inline int get_comp_ctx(const BlockContext *const a,
                               const BlockContext *const l,
                               const int yb4, const int xb4,
                               const int have_top, const int have_left)
{
    if (have_top) {
        if (have_left) {
            if (a->comp_type[xb4]) {
                if (l->comp_type[yb4]) {
                    return 4;
                } else {
                    // 4U means intra (-1) or bwd (>= 4)
                    return 2 + ((unsigned)l->ref[0][yb4] >= 4U);
                }
            } else if (l->comp_type[yb4]) {
                // 4U means intra (-1) or bwd (>= 4)
                return 2 + ((unsigned)a->ref[0][xb4] >= 4U);
            } else {
                return (l->ref[0][yb4] >= 4) ^ (a->ref[0][xb4] >= 4);
            }
        } else {
            return a->comp_type[xb4] ? 3 : a->ref[0][xb4] >= 4;
        }
    } else if (have_left) {
        return l->comp_type[yb4] ? 3 : l->ref[0][yb4] >= 4;
    } else {
        return 1;
    }
}

static inline int get_comp_dir_ctx(const BlockContext *const a,
                                   const BlockContext *const l,
                                   const int yb4, const int xb4,
                                   const int have_top, const int have_left)
{
#define has_uni_comp(edge, off) \
    ((edge->ref[0][off] < 4) == (edge->ref[1][off] < 4))

    if (have_top && have_left) {
        const int a_intra = a->intra[xb4], l_intra = l->intra[yb4];

        if (a_intra && l_intra) return 2;
        if (a_intra || l_intra) {
            const BlockContext *const edge = a_intra ? l : a;
            const int off = a_intra ? yb4 : xb4;

            if (edge->comp_type[off] == COMP_INTER_NONE) return 2;
            return 1 + 2 * has_uni_comp(edge, off);
        }

        const int a_comp = a->comp_type[xb4] != COMP_INTER_NONE;
        const int l_comp = l->comp_type[yb4] != COMP_INTER_NONE;
        const int a_ref0 = a->ref[0][xb4], l_ref0 = l->ref[0][yb4];

        if (!a_comp && !l_comp) {
            return 1 + 2 * ((a_ref0 >= 4) == (l_ref0 >= 4));
        } else if (!a_comp || !l_comp) {
            const BlockContext *const edge = a_comp ? a : l;
            const int off = a_comp ? xb4 : yb4;

            if (!has_uni_comp(edge, off)) return 1;
            return 3 + ((a_ref0 >= 4) == (l_ref0 >= 4));
        } else {
            const int a_uni = has_uni_comp(a, xb4), l_uni = has_uni_comp(l, yb4);

            if (!a_uni && !l_uni) return 0;
            if (!a_uni || !l_uni) return 2;
            return 3 + ((a_ref0 == 4) == (l_ref0 == 4));
        }
    } else if (have_top || have_left) {
        const BlockContext *const edge = have_left ? l : a;
        const int off = have_left ? yb4 : xb4;

        if (edge->intra[off]) return 2;
        if (edge->comp_type[off] == COMP_INTER_NONE) return 2;
        return 4 * has_uni_comp(edge, off);
    } else {
        return 2;
    }
}

static inline int get_poc_diff(const int order_hint_n_bits,
                               const int poc0, const int poc1)
{
    if (!order_hint_n_bits) return 0;
    const int mask = 1 << (order_hint_n_bits - 1);
    const int diff = poc0 - poc1;
    return (diff & (mask - 1)) - (diff & mask);
}

static inline int get_jnt_comp_ctx(const int order_hint_n_bits,
                                   const unsigned poc, const unsigned ref0poc,
                                   const unsigned ref1poc,
                                   const BlockContext *const a,
                                   const BlockContext *const l,
                                   const int yb4, const int xb4)
{
    const unsigned d0 = abs(get_poc_diff(order_hint_n_bits, ref0poc, poc));
    const unsigned d1 = abs(get_poc_diff(order_hint_n_bits, poc, ref1poc));
    const int offset = d0 == d1;
    const int a_ctx = a->comp_type[xb4] >= COMP_INTER_AVG ||
                      a->ref[0][xb4] == 6;
    const int l_ctx = l->comp_type[yb4] >= COMP_INTER_AVG ||
                      l->ref[0][yb4] == 6;

    return 3 * offset + a_ctx + l_ctx;
}

static inline int get_mask_comp_ctx(const BlockContext *const a,
                                    const BlockContext *const l,
                                    const int yb4, const int xb4)
{
    const int a_ctx = a->comp_type[xb4] >= COMP_INTER_SEG ? 1 :
                      a->ref[0][xb4] == 6 ? 3 : 0;
    const int l_ctx = l->comp_type[yb4] >= COMP_INTER_SEG ? 1 :
                      l->ref[0][yb4] == 6 ? 3 : 0;

    return imin(a_ctx + l_ctx, 5);
}

#define av1_get_ref_2_ctx av1_get_bwd_ref_ctx
#define av1_get_ref_3_ctx av1_get_fwd_ref_ctx
#define av1_get_ref_4_ctx av1_get_fwd_ref_1_ctx
#define av1_get_ref_5_ctx av1_get_fwd_ref_2_ctx
#define av1_get_ref_6_ctx av1_get_bwd_ref_1_ctx
#define av1_get_uni_p_ctx av1_get_ref_ctx
#define av1_get_uni_p2_ctx av1_get_fwd_ref_2_ctx

static inline int av1_get_ref_ctx(const BlockContext *const a,
                                  const BlockContext *const l,
                                  const int yb4, const int xb4,
                                  int have_top, int have_left)
{
    int cnt[2] = { 0 };

    if (have_top && !a->intra[xb4]) {
        cnt[a->ref[0][xb4] >= 4]++;
        if (a->comp_type[xb4]) cnt[a->ref[1][xb4] >= 4]++;
    }

    if (have_left && !l->intra[yb4]) {
        cnt[l->ref[0][yb4] >= 4]++;
        if (l->comp_type[yb4]) cnt[l->ref[1][yb4] >= 4]++;
    }

    return cnt[0] == cnt[1] ? 1 : cnt[0] < cnt[1] ? 0 : 2;
}

static inline int av1_get_fwd_ref_ctx(const BlockContext *const a,
                                      const BlockContext *const l,
                                      const int yb4, const int xb4,
                                      const int have_top, const int have_left)
{
    int cnt[4] = { 0 };

    if (have_top && !a->intra[xb4]) {
        if (a->ref[0][xb4] < 4) cnt[a->ref[0][xb4]]++;
        if (a->comp_type[xb4] && a->ref[1][xb4] < 4) cnt[a->ref[1][xb4]]++;
    }

    if (have_left && !l->intra[yb4]) {
        if (l->ref[0][yb4] < 4) cnt[l->ref[0][yb4]]++;
        if (l->comp_type[yb4] && l->ref[1][yb4] < 4) cnt[l->ref[1][yb4]]++;
    }

    cnt[0] += cnt[1];
    cnt[2] += cnt[3];

    return cnt[0] == cnt[2] ? 1 : cnt[0] < cnt[2] ? 0 : 2;
}

static inline int av1_get_fwd_ref_1_ctx(const BlockContext *const a,
                                        const BlockContext *const l,
                                        const int yb4, const int xb4,
                                        const int have_top, const int have_left)
{
    int cnt[2] = { 0 };

    if (have_top && !a->intra[xb4]) {
        if (a->ref[0][xb4] < 2) cnt[a->ref[0][xb4]]++;
        if (a->comp_type[xb4] && a->ref[1][xb4] < 2) cnt[a->ref[1][xb4]]++;
    }

    if (have_left && !l->intra[yb4]) {
        if (l->ref[0][yb4] < 2) cnt[l->ref[0][yb4]]++;
        if (l->comp_type[yb4] && l->ref[1][yb4] < 2) cnt[l->ref[1][yb4]]++;
    }

    return cnt[0] == cnt[1] ? 1 : cnt[0] < cnt[1] ? 0 : 2;
}

static inline int av1_get_fwd_ref_2_ctx(const BlockContext *const a,
                                        const BlockContext *const l,
                                        const int yb4, const int xb4,
                                        const int have_top, const int have_left)
{
    int cnt[2] = { 0 };

    if (have_top && !a->intra[xb4]) {
        if ((a->ref[0][xb4] ^ 2U) < 2) cnt[a->ref[0][xb4] - 2]++;
        if (a->comp_type[xb4] && (a->ref[1][xb4] ^ 2U) < 2) cnt[a->ref[1][xb4] - 2]++;
    }

    if (have_left && !l->intra[yb4]) {
        if ((l->ref[0][yb4] ^ 2U) < 2) cnt[l->ref[0][yb4] - 2]++;
        if (l->comp_type[yb4] && (l->ref[1][yb4] ^ 2U) < 2) cnt[l->ref[1][yb4] - 2]++;
    }

    return cnt[0] == cnt[1] ? 1 : cnt[0] < cnt[1] ? 0 : 2;
}

static inline int av1_get_bwd_ref_ctx(const BlockContext *const a,
                                      const BlockContext *const l,
                                      const int yb4, const int xb4,
                                      const int have_top, const int have_left)
{
    int cnt[3] = { 0 };

    if (have_top && !a->intra[xb4]) {
        if (a->ref[0][xb4] >= 4) cnt[a->ref[0][xb4] - 4]++;
        if (a->comp_type[xb4] && a->ref[1][xb4] >= 4) cnt[a->ref[1][xb4] - 4]++;
    }

    if (have_left && !l->intra[yb4]) {
        if (l->ref[0][yb4] >= 4) cnt[l->ref[0][yb4] - 4]++;
        if (l->comp_type[yb4] && l->ref[1][yb4] >= 4) cnt[l->ref[1][yb4] - 4]++;
    }

    cnt[1] += cnt[0];

    return cnt[2] == cnt[1] ? 1 : cnt[1] < cnt[2] ? 0 : 2;
}

static inline int av1_get_bwd_ref_1_ctx(const BlockContext *const a,
                                        const BlockContext *const l,
                                        const int yb4, const int xb4,
                                        const int have_top, const int have_left)
{
    int cnt[3] = { 0 };

    if (have_top && !a->intra[xb4]) {
        if (a->ref[0][xb4] >= 4) cnt[a->ref[0][xb4] - 4]++;
        if (a->comp_type[xb4] && a->ref[1][xb4] >= 4) cnt[a->ref[1][xb4] - 4]++;
    }

    if (have_left && !l->intra[yb4]) {
        if (l->ref[0][yb4] >= 4) cnt[l->ref[0][yb4] - 4]++;
        if (l->comp_type[yb4] && l->ref[1][yb4] >= 4) cnt[l->ref[1][yb4] - 4]++;
    }

    return cnt[0] == cnt[1] ? 1 : cnt[0] < cnt[1] ? 0 : 2;
}

static inline int av1_get_uni_p1_ctx(const BlockContext *const a,
                                     const BlockContext *const l,
                                     const int yb4, const int xb4,
                                     const int have_top, const int have_left)
{
    int cnt[3] = { 0 };

    if (have_top && !a->intra[xb4]) {
        if (a->ref[0][xb4] - 1U < 3) cnt[a->ref[0][xb4] - 1]++;
        if (a->comp_type[xb4] && a->ref[1][xb4] - 1U < 3) cnt[a->ref[1][xb4] - 1]++;
    }

    if (have_left && !l->intra[yb4]) {
        if (l->ref[0][yb4] - 1U < 3) cnt[l->ref[0][yb4] - 1]++;
        if (l->comp_type[yb4] && l->ref[1][yb4] - 1U < 3) cnt[l->ref[1][yb4] - 1]++;
    }

    cnt[1] += cnt[2];

    return cnt[0] == cnt[1] ? 1 : cnt[0] < cnt[1] ? 0 : 2;
}

static inline int get_drl_context(const refmvs_candidate *const ref_mv_stack,
                                  const int ref_idx)
{
    if (ref_mv_stack[ref_idx].weight >= 640)
        return ref_mv_stack[ref_idx + 1].weight < 640;

    return ref_mv_stack[ref_idx + 1].weight < 640 ? 2 : 0;
}

static inline unsigned get_cur_frame_segid(const int by, const int bx,
                                           const int have_top,
                                           const int have_left,
                                           int *const seg_ctx,
                                           const uint8_t *cur_seg_map,
                                           const ptrdiff_t stride)
{
    cur_seg_map += bx + by * stride;
    if (have_left && have_top) {
        const int l = cur_seg_map[-1];
        const int a = cur_seg_map[-stride];
        const int al = cur_seg_map[-(stride + 1)];

        if (l == a && al == l) *seg_ctx = 2;
        else if (l == a || al == l || a == al) *seg_ctx = 1;
        else *seg_ctx = 0;
        return a == al ? a : l;
    } else {
        *seg_ctx = 0;
        return have_left ? cur_seg_map[-1] : have_top ? cur_seg_map[-stride] : 0;
    }
}

static inline void fix_int_mv_precision(mv *const mv) {
    mv->x = (mv->x - (mv->x >> 15) + 3) & ~7U;
    mv->y = (mv->y - (mv->y >> 15) + 3) & ~7U;
}

static inline void fix_mv_precision(const Dav1dFrameHeader *const hdr,
                                    mv *const mv)
{
    if (hdr->force_integer_mv) {
        fix_int_mv_precision(mv);
    } else if (!hdr->hp) {
        mv->x = (mv->x - (mv->x >> 15)) & ~1U;
        mv->y = (mv->y - (mv->y >> 15)) & ~1U;
    }
}

static inline mv get_gmv_2d(const Dav1dWarpedMotionParams *const gmv,
                            const int bx4, const int by4,
                            const int bw4, const int bh4,
                            const Dav1dFrameHeader *const hdr)
{
    switch (gmv->type) {
    case DAV1D_WM_TYPE_ROT_ZOOM:
        assert(gmv->matrix[5] ==  gmv->matrix[2]);
        assert(gmv->matrix[4] == -gmv->matrix[3]);
        // fall-through
    default:
    case DAV1D_WM_TYPE_AFFINE: {
        const int x = bx4 * 4 + bw4 * 2 - 1;
        const int y = by4 * 4 + bh4 * 2 - 1;
        const int xc = (gmv->matrix[2] - (1 << 16)) * x +
                       gmv->matrix[3] * y + gmv->matrix[0];
        const int yc = (gmv->matrix[5] - (1 << 16)) * y +
                       gmv->matrix[4] * x + gmv->matrix[1];
        const int shift = 16 - (3 - !hdr->hp);
        const int round = (1 << shift) >> 1;
        mv res = (mv) {
            .y = apply_sign(((abs(yc) + round) >> shift) << !hdr->hp, yc),
            .x = apply_sign(((abs(xc) + round) >> shift) << !hdr->hp, xc),
        };
        if (hdr->force_integer_mv)
            fix_int_mv_precision(&res);
        return res;
    }
    case DAV1D_WM_TYPE_TRANSLATION: {
        mv res = (mv) {
            .y = gmv->matrix[0] >> 13,
            .x = gmv->matrix[1] >> 13,
        };
        if (hdr->force_integer_mv)
            fix_int_mv_precision(&res);
        return res;
    }
    case DAV1D_WM_TYPE_IDENTITY:
        return (mv) { .x = 0, .y = 0 };
    }
}

#endif /* DAV1D_SRC_ENV_H */
