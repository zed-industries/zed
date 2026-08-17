/* ===--- amxtransposeintrin.h - AMX_TRANSPOSE intrinsics -*- C++ -*---------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 * ===-----------------------------------------------------------------------===
 */

#ifndef __IMMINTRIN_H
#error "Never use <amxtransposeintrin.h> directly; use <immintrin.h> instead."
#endif /* __IMMINTRIN_H */

#ifndef __AMX_TRANSPOSEINTRIN_H
#define __AMX_TRANSPOSEINTRIN_H
#ifdef __x86_64__

#define __DEFAULT_FN_ATTRS_TRANSPOSE                                           \
  __attribute__((__always_inline__, __nodebug__, __target__("amx-transpose")))

#define _tile_2rpntlvwz0(tdst, base, stride)                                   \
  __builtin_ia32_t2rpntlvwz0(tdst, base, stride)
#define _tile_2rpntlvwz0t1(tdst, base, stride)                                 \
  __builtin_ia32_t2rpntlvwz0t1(tdst, base, stride)
#define _tile_2rpntlvwz1(tdst, base, stride)                                   \
  __builtin_ia32_t2rpntlvwz1(tdst, base, stride)
#define _tile_2rpntlvwz1t1(tdst, base, stride)                                 \
  __builtin_ia32_t2rpntlvwz1t1(tdst, base, stride)

/// Transpose 32-bit elements from \a src and write the result to \a dst.
///
/// \headerfile <immintrin.h>
///
/// \code
/// void _tile_transposed(__tile dst, __tile src);
/// \endcode
///
/// This intrinsic corresponds to the <c> TTRANSPOSED </c> instruction.
///
/// \param dst
/// 	The destination tile. Max size is 1024 Bytes.
/// \param src
/// 	The source tile. Max size is 1024 Bytes.
///
/// \code{.operation}
///
/// FOR i := 0 TO (dst.rows-1)
/// 	tmp[511:0] := 0
/// 	FOR j := 0 TO (dst.colsb/4-1)
/// 		tmp.dword[j] := src.row[j].dword[i]
/// 	ENDFOR
/// 	dst.row[i] := tmp
/// ENDFOR
///
/// zero_upper_rows(dst, dst.rows)
/// zero_tileconfig_start()
/// \endcode
#define _tile_transposed(dst, src) __builtin_ia32_ttransposed(dst, src)

static __inline__ void __DEFAULT_FN_ATTRS_TRANSPOSE _tile_2rpntlvwz0_internal(
    unsigned short row, unsigned short col0, unsigned short col1,
    _tile1024i *dst0, _tile1024i *dst1, const void *base,
    __SIZE_TYPE__ stride) {
  // Use __tile1024i_1024a* to escape the alignment check in
  // clang/test/Headers/x86-intrinsics-headers-clean.cpp
  __builtin_ia32_t2rpntlvwz0_internal(row, col0, col1, (_tile1024i_1024a *)dst0,
                                      (_tile1024i_1024a *)dst1, base,
                                      (__SIZE_TYPE__)(stride));
}

static __inline__ void __DEFAULT_FN_ATTRS_TRANSPOSE _tile_2rpntlvwz0t1_internal(
    unsigned short row, unsigned short col0, unsigned short col1,
    _tile1024i *dst0, _tile1024i *dst1, const void *base,
    __SIZE_TYPE__ stride) {
  __builtin_ia32_t2rpntlvwz0t1_internal(
      row, col0, col1, (_tile1024i_1024a *)dst0, (_tile1024i_1024a *)dst1, base,
      (__SIZE_TYPE__)(stride));
}

static __inline__ void __DEFAULT_FN_ATTRS_TRANSPOSE _tile_2rpntlvwz1_internal(
    unsigned short row, unsigned short col0, unsigned short col1,
    _tile1024i *dst0, _tile1024i *dst1, const void *base,
    __SIZE_TYPE__ stride) {
  __builtin_ia32_t2rpntlvwz1_internal(row, col0, col1, (_tile1024i_1024a *)dst0,
                                      (_tile1024i_1024a *)dst1, base,
                                      (__SIZE_TYPE__)(stride));
}

static __inline__ void __DEFAULT_FN_ATTRS_TRANSPOSE _tile_2rpntlvwz1t1_internal(
    unsigned short row, unsigned short col0, unsigned short col1,
    _tile1024i *dst0, _tile1024i *dst1, const void *base,
    __SIZE_TYPE__ stride) {
  __builtin_ia32_t2rpntlvwz1t1_internal(
      row, col0, col1, (_tile1024i_1024a *)dst0, (_tile1024i_1024a *)dst1, base,
      (__SIZE_TYPE__)(stride));
}

// This is internal intrinsic. C/C++ user should avoid calling it directly.
static __inline__ _tile1024i __DEFAULT_FN_ATTRS_TRANSPOSE
_tile_transposed_internal(unsigned short m, unsigned short n, _tile1024i src) {
  return __builtin_ia32_ttransposed_internal(m, n, src);
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
/// Provides a hint to the implementation that the data will likely not be
/// reused in the near future and the data caching can be optimized.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> T2RPNTLVWZ0 </c> instruction.
///
/// \param dst0
///    First tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param dst1
///    Second tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param base
///    A pointer to base address.
/// \param stride
///    The stride between the rows' data to be loaded in memory.
__DEFAULT_FN_ATTRS_TRANSPOSE
static void __tile_2rpntlvwz0(__tile1024i *dst0, __tile1024i *dst1,
                              const void *base, __SIZE_TYPE__ stride) {
  _tile_2rpntlvwz0_internal(dst0->row, dst0->col, dst1->col, &dst0->tile,
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
/// This intrinsic corresponds to the <c> T2RPNTLVWZ0T1 </c> instruction.
///
/// \param dst0
///    First tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param dst1
///    Second tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param base
///    A pointer to base address.
/// \param stride
///    The stride between the rows' data to be loaded in memory.
__DEFAULT_FN_ATTRS_TRANSPOSE
static void __tile_2rpntlvwz0t1(__tile1024i *dst0, __tile1024i *dst1,
                                const void *base, __SIZE_TYPE__ stride) {
  _tile_2rpntlvwz0t1_internal(dst0->row, dst0->col, dst1->col, &dst0->tile,
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
/// Provides a hint to the implementation that the data will likely not be
/// reused in the near future and the data caching can be optimized.
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
__DEFAULT_FN_ATTRS_TRANSPOSE
static void __tile_2rpntlvwz1(__tile1024i *dst0, __tile1024i *dst1,
                              const void *base, __SIZE_TYPE__ stride) {
  _tile_2rpntlvwz1_internal(dst0->row, dst0->col, dst1->col, &dst0->tile,
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
/// Provides a hint to the implementation that the data will likely not be
/// reused in the near future and the data caching can be optimized.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> T2RPNTLVWZ1T1 </c> instruction.
///
/// \param dst0
///    First tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param dst1
///    Second tile of destination tile pair. Max size is 1024i*2 Bytes.
/// \param base
///    A pointer to base address.
/// \param stride
///    The stride between the rows' data to be loaded in memory.
__DEFAULT_FN_ATTRS_TRANSPOSE
static void __tile_2rpntlvwz1t1(__tile1024i *dst0, __tile1024i *dst1,
                                const void *base, __SIZE_TYPE__ stride) {
  _tile_2rpntlvwz1t1_internal(dst0->row, dst0->col, dst1->col, &dst0->tile,
                              &dst1->tile, base, stride);
}

/// Transpose 32-bit elements from src and write the result to dst.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> TTRANSPOSED </c> instruction.
///
/// \param dst
///    The destination tile. Max size is 1024 Bytes.
/// \param src
///    The source tile. Max size is 1024 Bytes.
__DEFAULT_FN_ATTRS_TRANSPOSE
static void __tile_transposed(__tile1024i *dst, __tile1024i src) {
  dst->tile = _tile_transposed_internal(dst->row, dst->col, src.tile);
}

#endif /* __x86_64__ */
#endif /* __AMX_TRANSPOSEINTRIN_H */
