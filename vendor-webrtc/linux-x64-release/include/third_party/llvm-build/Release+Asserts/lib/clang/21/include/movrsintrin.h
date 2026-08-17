/*===---------------- movrsintrin.h - MOVRS intrinsics ----------------------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===----------------------------------------------------------------------===*/

#ifndef __IMMINTRIN_H
#error "Never use <movrsintrin.h> directly; include <immintrin.h> instead."
#endif // __IMMINTRIN_H

#ifndef __MOVRSINTRIN_H
#define __MOVRSINTRIN_H

#define __DEFAULT_FN_ATTRS                                                     \
  __attribute__((__always_inline__, __nodebug__, __target__("movrs")))

#ifdef __x86_64__
static __inline__ char __DEFAULT_FN_ATTRS _movrs_i8(const void *__A) {
  return (char)__builtin_ia32_movrsqi((const void *)__A);
}

static __inline__ short __DEFAULT_FN_ATTRS _movrs_i16(const void *__A) {
  return (short)__builtin_ia32_movrshi((const void *)__A);
}

static __inline__ int __DEFAULT_FN_ATTRS _movrs_i32(const void *__A) {
  return (int)__builtin_ia32_movrssi((const void *)__A);
}

static __inline__ long long __DEFAULT_FN_ATTRS _movrs_i64(const void *__A) {
  return (long long)__builtin_ia32_movrsdi((const void *)__A);
}
#endif // __x86_64__

// Loads a memory sequence containing the specified memory address into
/// the L3 data cache. Data will be shared (read/written) to by requesting
/// core and other cores.
///
/// Note that the effect of this intrinsic is dependent on the processor
/// implementation.
///
/// \headerfile <x86intrin.h>
///
/// This intrinsic corresponds to the \c PREFETCHRS instruction.
///
/// \param __P
///    A pointer specifying the memory address to be prefetched.
static __inline__ void __DEFAULT_FN_ATTRS
_m_prefetchrs(volatile const void *__P) {
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wcast-qual"
  __builtin_ia32_prefetchrs((const void *)__P);
#pragma clang diagnostic pop
}

#undef __DEFAULT_FN_ATTRS
#endif // __MOVRSINTRIN_H