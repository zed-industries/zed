/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/*
 * WARNING: This file is auto-generated from scripts/autogen
 *          in the mldsa-native repository.
 *          Do not modify it directly.
 */

#ifndef MLD_NATIVE_X86_64_SRC_CONSTS_H
#define MLD_NATIVE_X86_64_SRC_CONSTS_H
#include "../../../common.h"
#define MLD_AVX2_BACKEND_DATA_OFFSET_8XQ 0
#define MLD_AVX2_BACKEND_DATA_OFFSET_8XQINV 8
#define MLD_AVX2_BACKEND_DATA_OFFSET_8XDIV_QINV 16
#define MLD_AVX2_BACKEND_DATA_OFFSET_8XDIV 24
#define MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV 32
#define MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS 328

#ifndef __ASSEMBLER__
#define mld_qdata MLD_NAMESPACE(qdata)
MLD_INTERNAL_DATA_DECLARATION const int32_t mld_qdata[624];
#endif

#endif /* !MLD_NATIVE_X86_64_SRC_CONSTS_H */
