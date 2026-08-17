/*===--------------------- amxavx512intrin.h - AMXAVX512 --------------------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===------------------------------------------------------------------------===
 */
#ifndef __IMMINTRIN_H
#error "Never use <amxavx512intrin.h> directly; include <immintrin.h> instead."
#endif // __IMMINTRIN_H

#ifndef __AMX_AVX512INTRIN_H
#define __AMX_AVX512INTRIN_H
#if defined(__x86_64__) && defined(__SSE2__)

#define __DEFAULT_FN_ATTRS_AVX512                                              \
  __attribute__((__always_inline__, __nodebug__,                               \
                 __target__("amx-avx512,avx10.2-512")))

/// Moves a row from a tile register to a zmm destination register, converting
///    the int32 source elements to fp32. The row of the tile is selected by a
///    32b GPR.
///
/// \headerfile <x86intrin.h>
///
/// \code
/// __m512i _tile_cvtrowd2ps(__tile tsrc, unsigned int row);
/// \endcode
///
/// \code{.operation}
/// VL := 512
/// VL_bytes := VL >> 3
/// row_index := row & 0xffff
/// row_chunk := ((row >> 16) & 0xffff) * VL_bytes
/// FOR i := 0 TO (VL_bytes / 4) - 1
///     IF i + row_chunk / 4 >= tsrc.colsb / 4
///         dst.dword[i] := 0
///     ELSE
///         dst.f32[i] := CONVERT_INT32_TO_FP32(tsrc.row[row_index].dword[row_chunk/4+i], RNE)
///     FI
/// ENDFOR
/// dst[MAX_VL-1:VL] := 0
/// zero_tileconfig_start()
/// \endcode
///
/// This intrinsic corresponds to the \c TCVTROWD2PS instruction.
///
/// \param tsrc
///    The source tile. Max size is 1024 Bytes.
/// \param row
///    The row of the source tile
#define _tile_cvtrowd2ps(tsrc, row) __builtin_ia32_tcvtrowd2ps(tsrc, row)

/// Moves a row from a tile register to a zmm destination register, converting
///    the fp32 source elements to bf16. It places the resulting bf16 elements
///    in the high 16 bits within each dword. The row of the tile is selected
///    by a 32b GPR.
///
/// \headerfile <x86intrin.h>
///
/// \code
/// __m512i _tile_cvtrowps2bf16h(__tile tsrc, unsigned int row);
/// \endcode
///
/// \code{.operation}
/// VL := 512
/// VL_bytes := VL >> 3
/// row_index := row & 0xffff
/// row_chunk := ((row >> 16) & 0xffff) * VL_bytes
/// FOR i := 0 TO (VL_bytes / 4) - 1
///     IF i + row_chunk / 4 >= tsrc.colsb / 4
///         dst.dword[i] := 0
///     ELSE
///         dst.word[2*i+0] := 0
///         dst.bf16[2*i+1] := CONVERT_FP32_TO_BF16(tsrc.row[row_index].fp32[row_chunk/4+i], RNE)
///     FI
/// ENDFOR
/// dst[MAX_VL-1:VL] := 0
/// zero_tileconfig_start()
/// \endcode
///
/// This intrinsic corresponds to the \c TCVTROWPS2BF16H instruction.
///
/// \param tsrc
///    The source tile. Max size is 1024 Bytes.
/// \param row
///    The the row of the source tile.
#define _tile_cvtrowps2bf16h(tsrc, row)                                        \
  __builtin_ia32_tcvtrowps2bf16h(tsrc, row)

/// Moves a row from a tile register to a zmm destination register, converting
///    the fp32 source elements to bf16. It places the resulting bf16 elements
///    in the low 16 bits within each dword. The row of the tile is selected
///    by a 32b GPR.
///
/// \headerfile <x86intrin.h>
///
/// \code
/// __m512i _tile_cvtrowps2bf16l(__tile tsrc, unsigned int row);
/// \endcode
///
/// \code{.operation}
/// VL := 512
/// VL_bytes := VL >> 3
/// row_index := row & 0xffff
/// row_chunk := ((row >> 16) & 0xffff) * VL_bytes
/// FOR i := 0 TO (VL_bytes / 4) - 1
///     IF i + row_chunk / 4 >= tsrc.colsb / 4
///         dst.dword[i] := 0
///     ELSE
///         dst.word[2*i+1] := 0
///         dst.bf16[2*i+0] := CONVERT_FP32_TO_BF16(tsrc.row[row_index].fp32[row_chunk/4+i], RNE)
///     FI
/// ENDFOR
/// dst[MAX_VL-1:VL] := 0
/// zero_tileconfig_start()
/// \endcode
///
/// This intrinsic corresponds to the \c TCVTROWPS2BF16L instruction.
///
/// \param tsrc
///    The source tile. Max size is 1024 Bytes.
/// \param row
///    The the row of the source tile.
#define _tile_cvtrowps2bf16l(tsrc, row)                                        \
  __builtin_ia32_tcvtrowps2bf16l(tsrc, row)

/// Moves a row from a tile register to a zmm destination register, converting
///    the fp32 source elements to fp16. It places the resulting fp16 elements
///    in the high 16 bits within each dword. The row of the tile is selected
///    by a 32b GPR.
///
/// \headerfile <x86intrin.h>
///
/// \code
/// __m512i _tile_cvtrowps2phh(__tile tsrc, unsigned int row);
/// \endcode
///
/// \code{.operation}
/// VL := 512
/// VL_bytes := VL >> 3
/// row_index := row & 0xffff
/// row_chunk := ((row >> 16) & 0xffff) * VL_bytes
/// FOR i := 0 TO (VL_bytes / 4) - 1
///     IF i + row_chunk / 4 >= tsrc.colsb / 4
///         dst.dword[i] := 0
///     ELSE
///         dst.word[2*i+0] := 0
///         dst.fp16[2*i+1] := CONVERT_FP32_TO_FP16(tsrc.row[row_index].fp32[row_chunk/4+i], RNE)
///     FI
/// ENDFOR
/// dst[MAX_VL-1:VL] := 0
/// zero_tileconfig_start()
/// \endcode
///
/// This intrinsic corresponds to the \c TCVTROWPS2PHH instruction.
///
/// \param tsrc
///    The source tile. Max size is 1024 Bytes.
/// \param row
///    The the row of the source tile.
#define _tile_cvtrowps2phh(tsrc, row) __builtin_ia32_tcvtrowps2phh(tsrc, row)

/// Moves a row from a tile register to a zmm destination register, converting
///    the fp32 source elements to fp16. It places the resulting fp16 elements
///    in the low 16 bits within each dword. The row of the tile is selected
///    by a 32b GPR.
///
/// \headerfile <x86intrin.h>
///
/// \code
/// __m512i _tile_cvtrowps2phl(__tile tsrc, unsigned int row);
/// \endcode
///
/// \code{.operation}
/// VL := 512
/// VL_bytes := VL >> 3
/// row_index := row & 0xffff
/// row_chunk := ((row >> 16) & 0xffff) * VL_bytes
/// FOR i := 0 TO (VL_bytes / 4) - 1
///     IF i + row_chunk / 4 >= tsrc.colsb / 4
///         dst.dword[i] := 0
///     ELSE
///         dst.word[2*i+1] := 0
///         dst.fp16[2*i+0] := CONVERT_FP32_TO_FP16(tsrc.row[row_index].fp32[row_chunk/4+i], RNE)
///     FI
/// ENDFOR
/// dst[MAX_VL-1:VL] := 0
/// zero_tileconfig_start()
/// \endcode
///
/// This intrinsic corresponds to the \c TCVTROWPS2PHL instruction.
///
/// \param tsrc
///    The source tile. Max size is 1024 Bytes.
/// \param row
///    The the row of the source tile.
#define _tile_cvtrowps2phl(tsrc, row) __builtin_ia32_tcvtrowps2phl(tsrc, row)

/// Move one row of a tile data to a v16f32 data.
/// The row of the tile is selected by a 32b GPR.
///
/// \headerfile <immintrin.h>
///
/// \code
/// __m512 _tile_movrow(__tile a, unsigned b);
/// \endcode
///
/// This intrinsic corresponds to the <c> TILEMOVROW </c> instruction.
///
/// \param a
///     The 1st source tile. Max size is 1024 Bytes.
/// \param b
///     The 2nd source r32. Size is 4 Bytes.
/// \returns
///     The destination v16f32 data. Size is 64 Bytes.
///
/// \code{.operation}
/// VL := 512
/// VL_bytes := VL>>3
/// row_index := b&0xffff
/// row_chunk := ((b>>16)&0xffff) * VL_bytes
/// FOR i := 0 TO (VL_bytes-1)
///     IF (row_chunk + i >= a.colsb)
///             dst.byte[i] := 0
///     ELSE
///             dst.byte[i] := a.row[row_index].byte[row_chunk+i]
/// ENDFOR
/// \endcode
#define _tile_movrow(a, b) ((__m512i)__builtin_ia32_tilemovrow(a, b))

/// This is internal intrinsic. C/C++ user should avoid calling it directly.

static __inline__ __m512 __DEFAULT_FN_ATTRS_AVX512 _tile_cvtrowd2ps_internal(
    unsigned short m, unsigned short n, _tile1024i src, unsigned u) {
  return __builtin_ia32_tcvtrowd2ps_internal(m, n, src, u);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS_AVX512
_tile_cvtrowps2bf16h_internal(unsigned short m, unsigned short n,
                              _tile1024i src, unsigned u) {
  return __builtin_ia32_tcvtrowps2bf16h_internal(m, n, src, u);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS_AVX512
_tile_cvtrowps2bf16l_internal(unsigned short m, unsigned short n,
                              _tile1024i src, unsigned u) {
  return __builtin_ia32_tcvtrowps2bf16l_internal(m, n, src, u);
}

static __inline__ __m512h __DEFAULT_FN_ATTRS_AVX512 _tile_cvtrowps2phh_internal(
    unsigned short m, unsigned short n, _tile1024i src, unsigned u) {
  return __builtin_ia32_tcvtrowps2phh_internal(m, n, src, u);
}

static __inline__ __m512h __DEFAULT_FN_ATTRS_AVX512 _tile_cvtrowps2phl_internal(
    unsigned short m, unsigned short n, _tile1024i src, unsigned u) {
  return __builtin_ia32_tcvtrowps2phl_internal(m, n, src, u);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS_AVX512 _tile_movrow_internal(
    unsigned short m, unsigned short n, _tile1024i src, unsigned u) {
  return (__m512i)__builtin_ia32_tilemovrow_internal(m, n, src, u);
}

/// Move a row from a tile (src0) to a v16f32 dst, converting the int32 source
/// elements to fp32. No SIMD exceptions are generated. Rounding is done as if
/// MXCSR.RC=RNE. Embedded rounding is not supported.
/// The row and chunk elements of tile is fetched from 32bit src1.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> TCVTROWD2PS </c> instruction.
///
/// \param src0
///    The 1st source tile. Max size is 1024 Bytes.
/// \param src1
///    The 2nd source r32. Size is 4 Bytes.
/// \returns
///    The destination v16f32 data. Size is 64 Bytes.
__DEFAULT_FN_ATTRS_AVX512
static __m512 __tile_cvtrowd2ps(__tile1024i src0, unsigned src1) {
  return _tile_cvtrowd2ps_internal(src0.row, src0.col, src0.tile, src1);
}

/// Move a row from a tile (src0) to a v32bf16 dst, converting the fp32 source
/// elements to bf16 at high 16-bits of each dword.
/// The row and chunk elements of tile is fetched from 32bit src1.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> TCVTROWPS2BF16H </c> instruction.
///
/// \param src0
///    The 1st source tile. Max size is 1024 Bytes.
/// \param src1
///    The 2nd source r32. Size is 4 Bytes.
/// \returns
///    The destination v32bf16 data. Size is 64 Bytes.
__DEFAULT_FN_ATTRS_AVX512
static __m512bh __tile_cvtrowps2bf16h(__tile1024i src0, unsigned src1) {
  return _tile_cvtrowps2bf16h_internal(src0.row, src0.col, src0.tile, src1);
}

/// Move a row from a tile (src0) to a v32bf16 dst, converting the fp32 source
/// elements to bf16 at low 16-bits of each dword.
/// The row and chunk elements of tile is fetched from 32bit src1.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> TCVTROWPS2BF16L </c> instruction.
///
/// \param src0
///    The 1st source tile. Max size is 1024 Bytes.
/// \param src1
///    The 2nd source r32. Size is 4 Bytes.
/// \returns
///    The destination v32bf16 data. Size is 64 Bytes.
__DEFAULT_FN_ATTRS_AVX512
static __m512bh __tile_cvtrowps2bf16l(__tile1024i src0, unsigned src1) {
  return _tile_cvtrowps2bf16l_internal(src0.row, src0.col, src0.tile, src1);
}

/// Move a row from a tile (src0) to a v32fp16 dst, converting the fp32 source
/// elements to fp16 at high 16-bits of each dword.
/// The row and chunk elements of tile is fetched from 32bit src1.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> TCVTROWPS2PHH </c> instruction.
///
/// \param src0
///    The 1st source tile. Max size is 1024 Bytes.
/// \param src1
///    The 2nd source r32. Size is 4 Bytes.
/// \returns
///    The destination v32fp16 data. Size is 64 Bytes.
__DEFAULT_FN_ATTRS_AVX512
static __m512h __tile_cvtrowps2phh(__tile1024i src0, unsigned src1) {
  return _tile_cvtrowps2phh_internal(src0.row, src0.col, src0.tile, src1);
}

/// Move a row from a tile (src0) to a v32fp16 dst, converting the fp32 source
/// elements to fp16 at low 16-bits of each dword.
/// The row and chunk elements of tile is fetched from 32bit src1.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> TCVTROWPS2PHL </c> instruction.
///
/// \param src0
///    The 1st source tile. Max size is 1024 Bytes.
/// \param src1
///    The 2nd source r32. Size is 4 Bytes.
/// \returns
///    The destination v32fp16 data. Size is 64 Bytes.
__DEFAULT_FN_ATTRS_AVX512
static __m512h __tile_cvtrowps2phl(__tile1024i src0, unsigned src1) {
  return _tile_cvtrowps2phl_internal(src0.row, src0.col, src0.tile, src1);
}

/// Move one row of a tile data to a v16f32 data.
/// The row of the tile is selected by a 32b GPR.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> TILEMOVROW </c> instruction.
///
/// \param src0
///    The 1st source tile. Max size is 1024 Bytes.
/// \param src1
///    The 2nd source r32. Size is 4 Bytes.
/// \returns
///    The destination v16i32 data. Size is 64 Bytes.
__DEFAULT_FN_ATTRS_AVX512
static __m512i __tile_movrow(__tile1024i src0, unsigned src1) {
  return (__m512i)_tile_movrow_internal(src0.row, src0.col, src0.tile, src1);
}

#endif // __x86_64__ && __SSE2__
#endif // __AMX_AVX512INTRIN_H
