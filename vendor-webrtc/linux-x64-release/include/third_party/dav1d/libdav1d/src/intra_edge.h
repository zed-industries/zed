/*
 * Copyright © 2018-2023, VideoLAN and dav1d authors
 * Copyright © 2018-2023, Two Orioles, LLC
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

#ifndef DAV1D_SRC_INTRA_EDGE_H
#define DAV1D_SRC_INTRA_EDGE_H

#include <stdint.h>

enum EdgeFlags {
    EDGE_I444_TOP_HAS_RIGHT   = 1 << 0,
    EDGE_I422_TOP_HAS_RIGHT   = 1 << 1,
    EDGE_I420_TOP_HAS_RIGHT   = 1 << 2,
    EDGE_I444_LEFT_HAS_BOTTOM = 1 << 3,
    EDGE_I422_LEFT_HAS_BOTTOM = 1 << 4,
    EDGE_I420_LEFT_HAS_BOTTOM = 1 << 5,
    EDGE_ALL_TOP_HAS_RIGHT    = EDGE_I444_TOP_HAS_RIGHT |
                                EDGE_I422_TOP_HAS_RIGHT |
                                EDGE_I420_TOP_HAS_RIGHT,
    EDGE_ALL_LEFT_HAS_BOTTOM  = EDGE_I444_LEFT_HAS_BOTTOM |
                                EDGE_I422_LEFT_HAS_BOTTOM |
                                EDGE_I420_LEFT_HAS_BOTTOM,
    EDGE_ALL_TR_AND_BL        = EDGE_ALL_TOP_HAS_RIGHT |
                                EDGE_ALL_LEFT_HAS_BOTTOM,
};

#define INTRA_EDGE_SPLIT(n, i) \
    ((const EdgeNode*)((uintptr_t)(n) + ((const EdgeBranch*)(n))->split_offset[i]))

typedef struct EdgeNode {
    uint8_t /* enum EdgeFlags */ o, h[2], v[2];
} EdgeNode;

typedef struct EdgeTip {
    EdgeNode node;
    uint8_t /* enum EdgeFlags */ split[3];
} EdgeTip;

typedef struct EdgeBranch {
    EdgeNode node;
    uint8_t /* enum EdgeFlags */ h4, v4;
    uint16_t split_offset[4]; /* relative to the address of this node */
} EdgeBranch;

/* Tree to keep track of which edges are available. */
EXTERN const EdgeNode *dav1d_intra_edge_tree[2 /* BL_128X128, BL_64X64 */];

void dav1d_init_intra_edge_tree(void);

#endif /* DAV1D_SRC_INTRA_EDGE_H */
