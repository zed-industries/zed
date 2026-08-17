/*
 * Declarations for double-precision e^x vector function.
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

#include "v_math.h"
#if WANT_VMATH

#define V_EXP_TABLE_BITS 7

extern const u64_t __v_exp_data[1 << V_EXP_TABLE_BITS] HIDDEN;
#endif
