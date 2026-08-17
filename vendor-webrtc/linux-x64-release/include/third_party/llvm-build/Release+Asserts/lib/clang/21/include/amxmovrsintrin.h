/*===-------- amxmovrsintrin.h - AMX MOVRS intrinsics -*- C++ -*---------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 * ===-------------------------------------------------------------------=== */

#ifndef __IMMINTRIN_H
#error "Never use <amxmovrsintrin.h> directly; include <immintrin.h> instead."
#endif /* __IMMINTRIN_H */

#ifndef __AMXMOVRSINTRIN_H
#define __AMXMOVRSINTRIN_H
#ifdef __x86_64__

#define __DEFAULT_FN_ATTRS_MOVRS                                               \
  __attribute__((__always_inline__, __nodebug__, __target__("amx-movrs")))

#define _tile_loaddrs(dst, base, stride)                                       \
  __builtin_ia32_tileloaddrs64((dst), ((const void *)(base)),                  \
                               (__SIZE_TYPE__)(stride))
#define _tile_stream_loaddrs(dst, base, stride)                                \
  __builtin_ia32_tileloaddrst164((dst), ((const void *)(base)),                \
                                 (__SIZE_TYPE__)(stride))
static __inline__ _tile1024i __DEFAULT_FN_ATTRS_MOVRS
_tile_loaddrs_internal(unsigned short m, unsigned short n, const void *base,
                       __SIZE_TYPE__ stride) {
  return __builtin_ia32_tileloaddrs64_internal(m, n, base,
                                               (__SIZE_TYPE__)(stride));
}
static __inline__ _tile1024i __DEFAULT_FN_ATTRS_MOVRS
_tile_loaddrst1_internal(unsigned short m, unsigned short n, const void *base,
                         __SIZE_TYPE__ stride) {
  return __builtin_ia32_tileloaddrst164_internal(m, n, base,
                                                 (__SIZE_TYPE__)(stride));
}
static __inline__ void __DEFAULT_FN_ATTRS_MOVRS
__tile_loaddrs(__tile1024i *dst, const void *base, __SIZE_TYPE__ stride) {
  dst->tile = _tile_loaddrs_internal(dst->row, dst->col, base, stride);
}
static __inline__ void __DEFAULT_FN_ATTRS_MOVRS __tile_stream_loaddrs(
    __tile1024i *dst, const void *base, __SIZE_TYPE__ stride) {
  dst->tile = _tile_loaddrst1_internal(dst->row, dst->col, base, stride);
}
#undef __DEFAULT_FN_ATTRS_MOVRS
#endif /* __x86_64__ */
#endif /* __AMXMOVRSINTRIN_H */
