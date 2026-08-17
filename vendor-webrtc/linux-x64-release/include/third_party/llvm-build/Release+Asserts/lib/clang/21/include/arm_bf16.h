/*===---- arm_bf16.h - ARM BF16 intrinsics -----------------------------------===
 *
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */

#ifndef __ARM_BF16_H
#define __ARM_BF16_H

typedef __bf16 bfloat16_t;
#define __ai static __inline__ __attribute__((__always_inline__, __nodebug__))


#undef __ai

#endif
