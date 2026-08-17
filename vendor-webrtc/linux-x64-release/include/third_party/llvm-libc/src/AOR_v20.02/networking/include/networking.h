/*
 * Public API.
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

unsigned short __chksum (const void *, unsigned int);
#if __aarch64__ && __ARM_NEON
unsigned short __chksum_aarch64_simd (const void *, unsigned int);
#endif
#if __arm__ && __ARM_NEON
unsigned short __chksum_arm_simd (const void *, unsigned int);
#endif
