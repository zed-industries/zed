/*===----- amxcomplextransposeintrin.h - AMX-COMPLEX and AMX-TRANSPOSE ------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===------------------------------------------------------------------------===
 */

#ifndef __IMMINTRIN_H
#error                                                                         \
    "Never use <amxcomplextransposeintrin.h> directly; include <immintrin.h> instead."
#endif // __IMMINTRIN_H

#ifndef __AMX_COMPLEXTRANSPOSEINTRIN_H
#define __AMX_COMPLEXTRANSPOSEINTRIN_H
#ifdef __x86_64__

#define __DEFAULT_FN_ATTRS                                                     \
  __attribute__((__always_inline__, __nodebug__,                               \
                 __target__("amx-complex,amx-transpose")))

/// Perform matrix multiplication of two tiles containing complex elements and
///    accumulate the results into a packed single precision tile. Each dword
///    element in input tiles \a a and \a b is interpreted as a complex number
///    with FP16 real part and FP16 imaginary part.
/// Calculates the imaginary part of the result. For each possible combination
///    of (transposed column of \a a, column of \a b), it performs a set of
///    multiplication and accumulations on all corresponding complex numbers
///    (one from \a a and one from \a b). The imaginary part of the \a a element
///    is multiplied with the real part of the corresponding \a b element, and
///    the real part of the \a a element is multiplied with the imaginary part
///    of the corresponding \a b elements. The two accumulated results are
///    added, and then accumulated into the corresponding row and column of
///    \a dst.
///
/// \headerfile <x86intrin.h>
///
/// \code
/// void _tile_tcmmimfp16ps(__tile dst, __tile a, __tile b);
/// \endcode
///
/// \code{.operation}
/// FOR m := 0 TO dst.rows - 1
///	tmp := dst.row[m]
///	FOR k := 0 TO a.rows - 1
///		FOR n := 0 TO (dst.colsb / 4) - 1
///			tmp.fp32[n] += FP32(a.row[m].fp16[2*k+0]) * FP32(b.row[k].fp16[2*n+1])
///			tmp.fp32[n] += FP32(a.row[m].fp16[2*k+1]) * FP32(b.row[k].fp16[2*n+0])
///		ENDFOR
///	ENDFOR
///	write_row_and_zero(dst, m, tmp, dst.colsb)
/// ENDFOR
/// zero_upper_rows(dst, dst.rows)
/// zero_tileconfig_start()
/// \endcode
///
/// This intrinsic corresponds to the \c TTCMMIMFP16PS instruction.
///
/// \param dst
///    The destination tile. Max size is 1024 Bytes.
/// \param a
///    The 1st source tile. Max size is 1024 Bytes.
/// \param b
///    The 2nd source tile. Max size is 1024 Bytes.
#define _tile_tcmmimfp16ps(dst, a, b)                                          \
  __builtin_ia32_ttcmmimfp16ps((dst), (a), (b))

/// Perform matrix multiplication of two tiles containing complex elements and
///    accumulate the results into a packed single precision tile. Each dword
///    element in input tiles \a a and \a b is interpreted as a complex number
///    with FP16 real part and FP16 imaginary part.
/// Calculates the real part of the result. For each possible combination
///    of (rtransposed colum of \a a, column of \a b), it performs a set of
///    multiplication and accumulations on all corresponding complex numbers
///    (one from \a a and one from \a b). The real part of the \a a element is
///    multiplied with the real part of the corresponding \a b element, and the
///    negated imaginary part of the \a a element is multiplied with the
///    imaginary part of the corresponding \a b elements. The two accumulated
///    results are added, and then accumulated into the corresponding row and
///    column of \a dst.
///
/// \headerfile <x86intrin.h>
///
/// \code
/// void _tile_tcmmrlfp16ps(__tile dst, __tile a, __tile b);
/// \endcode
///
/// \code{.operation}
/// FOR m := 0 TO dst.rows - 1
///	tmp := dst.row[m]
///	FOR k := 0 TO a.rows - 1
///		FOR n := 0 TO (dst.colsb / 4) - 1
///			tmp.fp32[n] += FP32(a.row[m].fp16[2*k+0]) * FP32(b.row[k].fp16[2*n+0])
///			tmp.fp32[n] += FP32(-a.row[m].fp16[2*k+1]) * FP32(b.row[k].fp16[2*n+1])
///		ENDFOR
///	ENDFOR
///	write_row_and_zero(dst, m, tmp, dst.colsb)
/// ENDFOR
/// zero_upper_rows(dst, dst.rows)
/// zero_tileconfig_start()
/// \endcode
///
/// This intrinsic corresponds to the \c TTCMMIMFP16PS instruction.
///
/// \param dst
///    The destination tile. Max size is 1024 Bytes.
/// \param a
///    The 1st source tile. Max size is 1024 Bytes.
/// \param b
///    The 2nd source tile. Max size is 1024 Bytes.
#define _tile_tcmmrlfp16ps(dst, a, b)                                          \
  __builtin_ia32_ttcmmrlfp16ps((dst), (a), (b))

/// Perform matrix conjugate transpose and multiplication of two tiles
///    containing complex elements and accumulate the results into a packed
///    single precision tile. Each dword element in input tiles \a a and \a b
///    is interpreted as a complex number with FP16 real part and FP16 imaginary
///    part.
/// Calculates the imaginary part of the result. For each possible combination
///    of (transposed column of \a a, column of \a b), it performs a set of
///    multiplication and accumulations on all corresponding complex numbers
///    (one from \a a and one from \a b). The negated imaginary part of the \a a
///    element is multiplied with the real part of the corresponding \a b
///    element, and the real part of the \a a element is multiplied with the
///    imaginary part of the corresponding \a b elements. The two accumulated
///    results are added, and then accumulated into the corresponding row and
///    column of \a dst.
///
/// \headerfile <x86intrin.h>
///
/// \code
/// void _tile_conjtcmmimfp16ps(__tile dst, __tile a, __tile b);
/// \endcode
///
/// \code{.operation}
/// FOR m := 0 TO dst.rows - 1
///	tmp := dst.row[m]
///	FOR k := 0 TO a.rows - 1
///		FOR n := 0 TO (dst.colsb / 4) - 1
///			tmp.fp32[n] += FP32(a.row[m].fp16[2*k+0]) * FP32(b.row[k].fp16[2*n+1])
///			tmp.fp32[n] += FP32(-a.row[m].fp16[2*k+1]) * FP32(b.row[k].fp16[2*n+0])
///		ENDFOR
///	ENDFOR
///	write_row_and_zero(dst, m, tmp, dst.colsb)
/// ENDFOR
/// zero_upper_rows(dst, dst.rows)
/// zero_tileconfig_start()
/// \endcode
///
/// This intrinsic corresponds to the \c TCONJTCMMIMFP16PS instruction.
///
/// \param dst
///    The destination tile. Max size is 1024 Bytes.
/// \param a
///    The 1st source tile. Max size is 1024 Bytes.
/// \param b
///    The 2nd source tile. Max size is 1024 Bytes.
#define _tile_conjtcmmimfp16ps(dst, a, b)                                      \
  __builtin_ia32_tconjtcmmimfp16ps((dst), (a), (b))

/// Perform conjugate transpose of an FP16-pair of complex elements from \a a
///    and writes the result to \a dst.
///
/// \headerfile <x86intrin.h>
///
/// \code
/// void _tile_conjtfp16(__tile dst, __tile a);
/// \endcode
///
/// \code{.operation}
/// FOR i := 0 TO dst.rows - 1
///	FOR j := 0 TO (dst.colsb / 4) - 1
///		tmp.fp16[2*j+0] := a.row[j].fp16[2*i+0]
///		tmp.fp16[2*j+1] := -a.row[j].fp16[2*i+1]
///	ENDFOR
///	write_row_and_zero(dst, i, tmp, dst.colsb)
/// ENDFOR
/// zero_upper_rows(dst, dst.rows)
/// zero_tileconfig_start()
/// \endcode
///
/// This intrinsic corresponds to the \c TCONJTFP16 instruction.
///
/// \param dst
///    The destination tile. Max size is 1024 Bytes.
/// \param a
///    The source tile. Max size is 1024 Bytes.
#define _tile_conjtfp16(dst, a) __builtin_ia32_tconjtfp16((dst), (a))

static __inline__ _tile1024i __DEFAULT_FN_ATTRS _tile_tcmmimfp16ps_internal(
    unsigned short m, unsigned short n, unsigned short k, _tile1024i dst,
    _tile1024i src1, _tile1024i src2) {
  return __builtin_ia32_ttcmmimfp16ps_internal(m, n, k, dst, src1, src2);
}

static __inline__ _tile1024i __DEFAULT_FN_ATTRS _tile_tcmmrlfp16ps_internal(
    unsigned short m, unsigned short n, unsigned short k, _tile1024i dst,
    _tile1024i src1, _tile1024i src2) {
  return __builtin_ia32_ttcmmrlfp16ps_internal(m, n, k, dst, src1, src2);
}

static __inline__ _tile1024i __DEFAULT_FN_ATTRS _tile_conjtcmmimfp16ps_internal(
    unsigned short m, unsigned short n, unsigned short k, _tile1024i dst,
    _tile1024i src1, _tile1024i src2) {
  return __builtin_ia32_tconjtcmmimfp16ps_internal(m, n, k, dst, src1, src2);
}

static __inline__ _tile1024i __DEFAULT_FN_ATTRS
_tile_conjtfp16_internal(unsigned short m, unsigned short n, _tile1024i src) {
  return __builtin_ia32_tconjtfp16_internal(m, n, src);
}

/// Perform matrix multiplication of two tiles containing complex elements and
///    accumulate the results into a packed single precision tile. Each dword
///    element in input tiles src0 and src1 is interpreted as a complex number
///    with FP16 real part and FP16 imaginary part.
///    This function calculates the imaginary part of the result.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> TTCMMIMFP16PS </c> instruction.
///
/// \param dst
///    The destination tile. Max size is 1024 Bytes.
/// \param src0
///    The 1st source tile. Max size is 1024 Bytes.
/// \param src1
///    The 2nd source tile. Max size is 1024 Bytes.
__DEFAULT_FN_ATTRS
static void __tile_tcmmimfp16ps(__tile1024i *dst, __tile1024i src0,
                                __tile1024i src1) {
  dst->tile = _tile_tcmmimfp16ps_internal(src0.row, src1.col, src0.col,
                                          dst->tile, src0.tile, src1.tile);
}

/// Perform matrix multiplication of two tiles containing complex elements and
///    accumulate the results into a packed single precision tile. Each dword
///    element in input tiles src0 and src1 is interpreted as a complex number
///    with FP16 real part and FP16 imaginary part.
///    This function calculates the real part of the result.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> TTCMMRLFP16PS </c> instruction.
///
/// \param dst
///    The destination tile. Max size is 1024 Bytes.
/// \param src0
///    The 1st source tile. Max size is 1024 Bytes.
/// \param src1
///    The 2nd source tile. Max size is 1024 Bytes.
__DEFAULT_FN_ATTRS
static void __tile_tcmmrlfp16ps(__tile1024i *dst, __tile1024i src0,
                                __tile1024i src1) {
  dst->tile = _tile_tcmmrlfp16ps_internal(src0.row, src1.col, src0.col,
                                          dst->tile, src0.tile, src1.tile);
}

/// Perform matrix conjugate transpose and multiplication of two tiles
///    containing complex elements and accumulate the results into a packed
///    single precision tile. Each dword element in input tiles src0 and src1
///    is interpreted as a complex number with FP16 real part and FP16 imaginary
///    part.
///    This function calculates the imaginary part of the result.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> TCONJTCMMIMFP16PS </c> instruction.
///
/// \param dst
///    The destination tile. Max size is 1024 Bytes.
/// \param src0
///    The 1st source tile. Max size is 1024 Bytes.
/// \param src1
///    The 2nd source tile. Max size is 1024 Bytes.
__DEFAULT_FN_ATTRS
static void __tile_conjtcmmimfp16ps(__tile1024i *dst, __tile1024i src0,
                                    __tile1024i src1) {
  dst->tile = _tile_conjtcmmimfp16ps_internal(src0.row, src1.col, src0.col,
                                              dst->tile, src0.tile, src1.tile);
}

/// Perform conjugate transpose of an FP16-pair of complex elements from src and
///    writes the result to dst.
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> TCONJTFP16 </c> instruction.
///
/// \param dst
///    The destination tile. Max size is 1024 Bytes.
/// \param src
///    The source tile. Max size is 1024 Bytes.
__DEFAULT_FN_ATTRS
static void __tile_conjtfp16(__tile1024i *dst, __tile1024i src) {
  dst->tile = _tile_conjtfp16_internal(src.row, src.col, src.tile);
}

#undef __DEFAULT_FN_ATTRS

#endif // __x86_64__
#endif // __AMX_COMPLEXTRANSPOSEINTRIN_H
