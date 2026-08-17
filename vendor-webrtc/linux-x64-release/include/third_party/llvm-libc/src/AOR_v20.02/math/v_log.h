/*
 * Declarations for double-precision log(x) vector function.
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

#include "v_math.h"
#if WANT_VMATH

#define V_LOG_TABLE_BITS 7

extern const struct v_log_data
{
  f64_t invc;
  f64_t logc;
} __v_log_data[1 << V_LOG_TABLE_BITS] HIDDEN;
#endif
