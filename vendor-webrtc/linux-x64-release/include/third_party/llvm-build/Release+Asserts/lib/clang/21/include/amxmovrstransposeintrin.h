/* ===--- amxmovrstransposeintrin.h - AMX_MOVRS_TRANSPOSE intrinsics --------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 * ===-----------------------------------------------------------------------===
 */

#ifndef __IMMINTRIN_H
#error                                                                         \
    "Never use <amxmovrstransposeintrin.h> directly; use <immintrin.h> instead."
#endif /* __IMMINTRIN_H */

#ifndef __AMX_MOVRS_TRANSPOSEINTRIN_H
#define __AMX_MOVRS_TRANSPOSEINTRIN_H
#ifdef __x86_64__

#define __DEFAULT_FN_ATTRS                                                     \
  __attribute__((__always_inline__, __nodebug__,                               \
                 __target__("amx-transpose,amx-movrs")))

#define _tile_2rpntlvwz0rs(tdst, base, stride)                                 \
  __builtin_ia32_t2rpntlvwz0rs(tdst, base, stride)
#define _tile_2rpntlvwz0rst1(tdst, base, stride)                               \
  __builtin_ia32_t2rpntlvwz0rst1(tdst, base, stride)
#define _tile_2rpntlvwz1rs(tdst, base, stride)                                 \
  __builtin_ia32_t2rpntlvwz1rs(tdst, base, stride)
#define _tile_2rpntlvwz1rst1(tdst, base, stride)                               \
  __builtin_ia32_t2rpntlvwz1rst1(tdst, base, stride)

static __inline__ void __DEFAULT_FN_ATTRS _tile_2rpntlvwz0rs_internal(
    unsigned short row, unsigned short col0, unsigned short col1,
    _tile1024i *dst0, _tile1024i *dst1, const void *base,
    __SIZE_TYPE__ stride) {
  // Use __tile1024i_1024a* to escape the alignment check in
  // clang/test/Headers/x86-intrinsics-headers-clean.cpp
  __builtin_ia32_t2rpntlvwz0rs_internal(
      row, col0, col1, (_tile1024i_1024a *)dst0, (_tile1024i_1024a *)dst1, base,
      (__SIZE_TYPE__)(stride));
}

static __inline__ void __DEFAULT_FN_ATTRS _tile_2rpntlvwz0rst1_internal(
    unsigned short row, unsigned short col0, unsigned short col1,
    _tile1024i *dst0, _tile1024i *dst1, const void *base,
    __SIZE_TYPE__ stride) {
  __builtin_ia32_t2rpntlvwz0rst1_internal(
      row, col0, col1, (_tile1024i_1024a *)dst0, (_tile1024i_1024a *)dst1, base,
      (__SIZE_TYPE__)(stride));
}

static __inline__ void __DEFAULT_FN_ATTRS _tile_2rpntlvwz1rs_internal(
    unsigned short row, unsigned short col0, unsigned short col1,
    _tile1024i *dst0, _tile1024i *dst1, const void *base,
    __SIZE_TYPE__ stride) {
  __builtin_ia32_t2rpntlvwz1rs_internal(
      row, col0, col1, (_tile1024i_1024a *)dst0, (_tile1024i_1024a *)dst1, base,
      (__SIZE_TYPE__)(stride));
}

static __inline__ void __DEFAULT_FN_ATTRS _tile_2rpntlvwz1rst1_internal(
    unsigned short row, unsigned short col0, unsigned short col1,
    _tile1024i *dst0, _tile1024i *dst1, const void *base,
    __SIZE_TYPE__ stride) {
  __builtin_ia32_t2rpntlvwz1rst1_internal(
      row, col0, col1, (_tile1024i_1024a *)dst0, (_tile1024i_1024a *)dst1, base,
      (__SIZE_TYPE__)(stride));
}

/// Converts a pair of tiles from memory into VNNI format, and places the
/// results in a pair of destinations specified by dst. The pair of tiles
/// in memory is specified via a tsib; the second tile is after the first
/// one, separated by the same stride that separates each row.
/// The tile configuration for the destination tiles indicates the amount
/// of data to read from memory. The instruction will load a number of rows
/// that is equal to twice the number of rows in tmm1. The size of each row
/// is equal to the average width of the destination tiles. If the second
/// tile is configured with zero rows and columns, only the first tile will
/// be written.
/// Provides a hint to the implementation that the data will likely become
/// read shared in the near future and the data caching can be optimized.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> T2RPNTLVWZ0RS </c> instruction.
///
/// \param dst0
///    First tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param dst1
///    Second tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param base
///    A pointer to base address.
/// \param stride
///    The stride between the rows' data to be loaded in memory.
__DEFAULT_FN_ATTRS
static void __tile_2rpntlvwz0rs(__tile1024i *dst0, __tile1024i *dst1,
                                const void *base, __SIZE_TYPE__ stride) {
  _tile_2rpntlvwz0rs_internal(dst0->row, dst0->col, dst1->col, &dst0->tile,
                              &dst1->tile, base, stride);
}

/// Converts a pair of tiles from memory into VNNI format, and places the
/// results in a pair of destinations specified by dst. The pair of tiles
/// in memory is specified via a tsib; the second tile is after the first
/// one, separated by the same stride that separates each row.
/// The tile configuration for the destination tiles indicates the amount
/// of data to read from memory. The instruction will load a number of rows
/// that is equal to twice the number of rows in tmm1. The size of each row
/// is equal to the average width of the destination tiles. If the second
/// tile is configured with zero rows and columns, only the first tile will
/// be written.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> T2RPNTLVWZ0T1RS </c> instruction.
///
/// \param dst0
///    First tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param dst1
///    Second tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param base
///    A pointer to base address.
/// \param stride
///    The stride between the rows' data to be loaded in memory.
__DEFAULT_FN_ATTRS
static void __tile_2rpntlvwz0rst1(__tile1024i *dst0, __tile1024i *dst1,
                                  const void *base, __SIZE_TYPE__ stride) {
  _tile_2rpntlvwz0rst1_internal(dst0->row, dst0->col, dst1->col, &dst0->tile,
                                &dst1->tile, base, stride);
}

/// Converts a pair of tiles from memory into VNNI format, and places the
/// results in a pair of destinations specified by dst. The pair of tiles
/// in memory is specified via a tsib; the second tile is after the first
/// one, separated by the same stride that separates each row.
/// The tile configuration for the destination tiles indicates the amount
/// of data to read from memory. The instruction will load a number of rows
/// that is equal to twice the number of rows in tmm1. The size of each row
/// is equal to the average width of the destination tiles. If the second
/// tile is configured with zero rows and columns, only the first tile will
/// be written. The last row will be not be read from memory but instead
/// filled with zeros.
/// Provides a hint to the implementation that the data will likely become
/// read shared in the near future and the data caching can be optimized.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> T2RPNTLVWZ1 </c> instruction.
///
/// \param dst0
///    First tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param dst1
///    Second tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param base
///    A pointer to base address.
/// \param stride
///    The stride between the rows' data to be loaded in memory.
__DEFAULT_FN_ATTRS
static void __tile_2rpntlvwz1rs(__tile1024i *dst0, __tile1024i *dst1,
                                const void *base, __SIZE_TYPE__ stride) {
  _tile_2rpntlvwz1rs_internal(dst0->row, dst0->col, dst1->col, &dst0->tile,
                              &dst1->tile, base, stride);
}

/// Converts a pair of tiles from memory into VNNI format, and places the
/// results in a pair of destinations specified by dst. The pair of tiles
/// in memory is specified via a tsib; the second tile is after the first
/// one, separated by the same stride that separates each row.
/// The tile configuration for the destination tiles indicates the amount
/// of data to read from memory. The instruction will load a number of rows
/// that is equal to twice the number of rows in tmm1. The size of each row
/// is equal to the average width of the destination tiles. If the second
/// tile is configured with zero rows and columns, only the first tile will
/// be written. The last row will be not be read from memory but instead
/// filled with zeros.
/// Provides a hint to the implementation that the data will likely become
/// read shared in the near future and the data caching can be optimized.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> T2RPNTLVWZ1T1RS </c> instruction.
///
/// \param dst0
///    First tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param dst1
///    Second tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param base
///    A pointer to base address.
/// \param stride
///    The stride between the rows' data to be loaded in memory.
__DEFAULT_FN_ATTRS
static void __tile_2rpntlvwz1rst1(__tile1024i *dst0, __tile1024i *dst1,
                                  const void *base, __SIZE_TYPE__ stride) {
  _tile_2rpntlvwz1rst1_internal(dst0->row, dst0->col, dst1->col, &dst0->tile,
                                &dst1->tile, base, stride);
}

#undef __DEFAULT_FN_ATTRS
#endif /* __x86_64__ */
#endif /* __AMX_MOVRS_TRANSPOSEINTRIN_H */