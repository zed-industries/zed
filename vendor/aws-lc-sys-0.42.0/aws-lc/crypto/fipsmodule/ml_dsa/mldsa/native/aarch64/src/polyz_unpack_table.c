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

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API)
/* Table of indices used for tbl instructions in polyz_unpack_{17,19}.
 * See autogen for details. */

#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || MLD_CONFIG_PARAMETER_SET == 44
MLD_ALIGN MLD_INTERNAL_DATA_DEFINITION const uint8_t
    mld_polyz_unpack_17_indices[64] = {
        0,  1,  2,  255, 2,  3,  4,  255, 4,  5,  6,  255, 6,  7,  8,  255,
        9,  10, 11, 255, 11, 12, 13, 255, 13, 14, 15, 255, 15, 16, 17, 255,
        2,  3,  4,  255, 4,  5,  6,  255, 6,  7,  8,  255, 8,  9,  10, 255,
        11, 12, 13, 255, 13, 14, 15, 255, 15, 28, 29, 255, 29, 30, 31, 255,
};
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLD_CONFIG_PARAMETER_SET == 44 \
        */

#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || \
    (MLD_CONFIG_PARAMETER_SET == 65 || MLD_CONFIG_PARAMETER_SET == 87)
MLD_ALIGN MLD_INTERNAL_DATA_DEFINITION const uint8_t
    mld_polyz_unpack_19_indices[64] = {
        0,  1,  2,  255, 2,  3,  4,  255, 5,  6,  7,  255, 7,  8,  9,  255,
        10, 11, 12, 255, 12, 13, 14, 255, 15, 16, 17, 255, 17, 18, 19, 255,
        4,  5,  6,  255, 6,  7,  8,  255, 9,  10, 11, 255, 11, 12, 13, 255,
        14, 15, 24, 255, 24, 25, 26, 255, 27, 28, 29, 255, 29, 30, 31, 255,
};
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLD_CONFIG_PARAMETER_SET == 65 \
          || MLD_CONFIG_PARAMETER_SET == 87 */

#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API */

#else /* MLD_ARITH_BACKEND_AARCH64 && !MLD_CONFIG_MULTILEVEL_NO_SHARED */

MLD_EMPTY_CU(aarch64_polyz_unpack_table)

#endif /* !(MLD_ARITH_BACKEND_AARCH64 && !MLD_CONFIG_MULTILEVEL_NO_SHARED) */
