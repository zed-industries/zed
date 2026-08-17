/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/*
 * WARNING: This file is auto-generated from scripts/autogen
 *          in the mldsa-native repository.
 *          Do not modify it directly.
 */

#include "../../../common.h"

#if defined(MLD_ARITH_BACKEND_AARCH64) && \
    !defined(MLD_CONFIG_MULTILEVEL_NO_SHARED)

#include "arith_native_aarch64.h"

/*
 * Lookup table used by rejection sampling of the public matrix.
 * See autogen for details.
 */
MLD_ALIGN MLD_INTERNAL_DATA_DEFINITION const uint8_t
    mld_rej_uniform_table[256] = {
        255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255 /* 0 */,
        0,   1,   2,   3,   255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255 /* 1 */,
        4,   5,   6,   7,   255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255 /* 2 */,
        0,   1,   2,   3,   4,   5,   6,   7,
        255, 255, 255, 255, 255, 255, 255, 255 /* 3 */,
        8,   9,   10,  11,  255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255 /* 4 */,
        0,   1,   2,   3,   8,   9,   10,  11,
        255, 255, 255, 255, 255, 255, 255, 255 /* 5 */,
        4,   5,   6,   7,   8,   9,   10,  11,
        255, 255, 255, 255, 255, 255, 255, 255 /* 6 */,
        0,   1,   2,   3,   4,   5,   6,   7,
        8,   9,   10,  11,  255, 255, 255, 255 /* 7 */,
        12,  13,  14,  15,  255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255 /* 8 */,
        0,   1,   2,   3,   12,  13,  14,  15,
        255, 255, 255, 255, 255, 255, 255, 255 /* 9 */,
        4,   5,   6,   7,   12,  13,  14,  15,
        255, 255, 255, 255, 255, 255, 255, 255 /* 10 */,
        0,   1,   2,   3,   4,   5,   6,   7,
        12,  13,  14,  15,  255, 255, 255, 255 /* 11 */,
        8,   9,   10,  11,  12,  13,  14,  15,
        255, 255, 255, 255, 255, 255, 255, 255 /* 12 */,
        0,   1,   2,   3,   8,   9,   10,  11,
        12,  13,  14,  15,  255, 255, 255, 255 /* 13 */,
        4,   5,   6,   7,   8,   9,   10,  11,
        12,  13,  14,  15,  255, 255, 255, 255 /* 14 */,
        0,   1,   2,   3,   4,   5,   6,   7,
        8,   9,   10,  11,  12,  13,  14,  15 /* 15 */,
};

#else /* MLD_ARITH_BACKEND_AARCH64 && !MLD_CONFIG_MULTILEVEL_NO_SHARED */

MLD_EMPTY_CU(aarch64_rej_uniform_table)

#endif /* !(MLD_ARITH_BACKEND_AARCH64 && !MLD_CONFIG_MULTILEVEL_NO_SHARED) */
