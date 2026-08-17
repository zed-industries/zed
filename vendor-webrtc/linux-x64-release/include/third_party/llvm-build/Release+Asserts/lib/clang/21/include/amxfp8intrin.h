/*===------------- amxfp8intrin.h - AMX intrinsics -*- C++ -*----------------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===------------------------------------------------------------------------===
 */

#ifndef __IMMINTRIN_H
#error "Never use <amxfp8intrin.h> directly; include <immintrin.h> instead."
#endif /* __IMMINTRIN_H */

#ifndef __AMXFP8INTRIN_H
#define __AMXFP8INTRIN_H
#ifdef __x86_64__

#define __DEFAULT_FN_ATTRS_FP8                                                 \
  __attribute__((__always_inline__, __nodebug__, __target__("amx-fp8")))

static __inline__ _tile1024i __DEFAULT_FN_ATTRS_FP8
_tile_dpbf8ps_internal(unsigned short m, unsigned short n, unsigned short k,
                       _tile1024i dst, _tile1024i src1, _tile1024i src2) {
  return __builtin_ia32_tdpbf8ps_internal(m, n, k, dst, src1, src2);
}

/// Perform the dot product of a BF8 value \a src1 by a BF8 value \a src2
/// accumulating into a Single Precision (FP32) source/dest \a dst.
///
/// \headerfile <immintrin.h>
///
/// \code
/// void __tile_dpbf8ps (__tile1024i *dst, __tile1024i src1, __tile1024i src2)
/// \endcode
///
/// \code{.operation}
/// FOR m := 0 TO dst.rows - 1
///   temp1[(dst.colsb / 4 - 1) : 0] = 0
///   FOR k := 0 TO src1.colsb / 4 - 1
///     FOR n := 0 TO dst.colsb / 4 - 1
///       temp1[n] +=
///         INT64(src1.row[m].float8[4*k+0]) * INT64(src2.row[k].float8[4*n+0])
///         + INT64(src1.row[m].float8[4*k+1]) * INT64(src2.row[k].float8[4*n+1])
///         + INT64(src1.row[m].float8[4*k+2]) * INT64(src2.row[k].float8[4*n+2])
///         + INT64(src1.row[m].float8[4*k+3]) * INT64(src2.row[k].float8[4*n+3])
///     ENDFOR
///   ENDFOR
///   FOR n := 0 TO dst.colsb / 4 - 1
///     tmp.row[m].fp32[n] = dst.row[m].fp32[n] + FP32(temp1[n])
///   ENDFOR
/// write_row_and_zero(dst, m, tmp, dst.colsb)
/// zero_upper_rows(dst, dst.rows)
/// zero_tileconfig_start()
/// \endcode
///
/// This intrinsic corresponds to the \c TDPBF8PS instruction.
///
/// \param dst
///    The destination tile. Max size is 1024 Bytes.
/// \param src1
///    The 1st source tile. Max size is 1024 Bytes.
/// \param src2
///    The 2nd source tile. Max size is 1024 Bytes.
__DEFAULT_FN_ATTRS_FP8 static void
__tile_dpbf8ps(__tile1024i *dst, __tile1024i src1, __tile1024i src2) {
  dst->tile = _tile_dpbf8ps_internal(src1.row, src2.col, src1.col, dst->tile,
                                     src1.tile, src2.tile);
}

static __inline__ _tile1024i __DEFAULT_FN_ATTRS_FP8
_tile_dpbhf8ps_internal(unsigned short m, unsigned short n, unsigned short k,
                        _tile1024i dst, _tile1024i src1, _tile1024i src2) {
  return __builtin_ia32_tdpbhf8ps_internal(m, n, k, dst, src1, src2);
}

/// Perform the dot product of a BF8 value \a src1 by an HF8 value \a src2
/// accumulating into a Single Precision (FP32) source/dest \a dst.
///
/// \headerfile <immintrin.h>
///
/// \code
/// void __tile_dpbhf8ps (__tile1024i dst, __tile1024i src1, __tile1024i src2)
/// \endcode
///
/// \code{.operation}
/// FOR m := 0 TO dst.rows - 1
///   temp1[(dst.colsb / 4 - 1) : 0] = 0
///   FOR k := 0 TO src1.colsb / 4 - 1
///     FOR n := 0 TO dst.colsb / 4 - 1
///       temp1[n] +=
///         INT64(src1.row[m].float8[4*k+0]) * INT64(src2.row[k].float8[4*n+0])
///         + INT64(src1.row[m].float8[4*k+1]) * INT64(src2.row[k].float8[4*n+1])
///         + INT64(src1.row[m].float8[4*k+2]) * INT64(src2.row[k].float8[4*n+2])
///         + INT64(src1.row[m].float8[4*k+3]) * INT64(src2.row[k].float8[4*n+3])
///     ENDFOR
///   ENDFOR
///   FOR n := 0 TO dst.colsb / 4 - 1
///     tmp.row[m].fp32[n] = dst.row[m].fp32[n] + FP32(temp1[n])
///   ENDFOR
/// write_row_and_zero(dst, m, tmp, dst.colsb)
/// zero_upper_rows(dst, dst.rows)
/// zero_tileconfig_start()
/// \endcode
///
/// This intrinsic corresponds to the \c TDPBHF8PS instruction.
///
/// \param dst
///    The destination tile. Max size is 1024 Bytes.
/// \param src1
///    The 1st source tile. Max size is 1024 Bytes.
/// \param src2
///    The 2nd source tile. Max size is 1024 Bytes.
__DEFAULT_FN_ATTRS_FP8 static void
__tile_dpbhf8ps(__tile1024i *dst, __tile1024i src1, __tile1024i src2) {
  dst->tile = _tile_dpbhf8ps_internal(src1.row, src2.col, src1.col, dst->tile,
                                      src1.tile, src2.tile);
}

static __inline__ _tile1024i __DEFAULT_FN_ATTRS_FP8
_tile_dphbf8ps_internal(unsigned short m, unsigned short n, unsigned short k,
                        _tile1024i dst, _tile1024i src1, _tile1024i src2) {
  return __builtin_ia32_tdphbf8ps_internal(m, n, k, dst, src1, src2);
}

/// Perform the dot product of an HF8 value \a src1 by a BF8 value \a src2
/// accumulating into a Single Precision (FP32) source/dest \a dst.
///
/// \headerfile <immintrin.h>
///
/// \code
/// void __tile_dphbf8ps (__tile1024i dst, __tile1024i src1, __tile1024i src2)
/// \endcode
///
/// \code{.operation}
/// FOR m := 0 TO dst.rows - 1
///   temp1[(dst.colsb / 4 - 1) : 0] = 0
///   FOR k := 0 TO src1.colsb / 4 - 1
///     FOR n := 0 TO dst.colsb / 4 - 1
///       temp1[n] +=
///         INT64(src1.row[m].float8[4*k+0]) * INT64(src2.row[k].float8[4*n+0])
///         + INT64(src1.row[m].float8[4*k+1]) * INT64(src2.row[k].float8[4*n+1])
///         + INT64(src1.row[m].float8[4*k+2]) * INT64(src2.row[k].float8[4*n+2])
///         + INT64(src1.row[m].float8[4*k+3]) * INT64(src2.row[k].float8[4*n+3])
///     ENDFOR
///   ENDFOR
///   FOR n := 0 TO dst.colsb / 4 - 1
///     tmp.row[m].fp32[n] = dst.row[m].fp32[n] + FP32(temp1[n])
///   ENDFOR
/// write_row_and_zero(dst, m, tmp, dst.colsb)
/// zero_upper_rows(dst, dst.rows)
/// zero_tileconfig_start()
/// \endcode
///
/// This intrinsic corresponds to the \c TDPHBF8PS instruction.
///
/// \param dst
///    The destination tile. Max size is 1024 Bytes.
/// \param src1
///    The 1st source tile. Max size is 1024 Bytes.
/// \param src2
///    The 2nd source tile. Max size is 1024 Bytes.

__DEFAULT_FN_ATTRS_FP8 static void
__tile_dphbf8ps(__tile1024i *dst, __tile1024i src1, __tile1024i src2) {
  dst->tile = _tile_dphbf8ps_internal(src1.row, src2.col, src1.col, dst->tile,
                                      src1.tile, src2.tile);
}

static __inline__ _tile1024i __DEFAULT_FN_ATTRS_FP8
_tile_dphf8ps_internal(unsigned short m, unsigned short n, unsigned short k,
                       _tile1024i dst, _tile1024i src1, _tile1024i src2) {
  return __builtin_ia32_tdphf8ps_internal(m, n, k, dst, src1, src2);
}

/// Perform the dot product of an HF8 value \a src1 by an HF8 value \a src2
/// accumulating into a Single Precision (FP32) source/dest \a dst.
///
/// \headerfile <immintrin.h>
///
/// \code
/// void __tile_dphf8ps (__tile1024i dst, __tile1024i src1, __tile1024i src2)
/// \endcode
///
/// \code{.operation}
/// FOR m := 0 TO dst.rows - 1
///   temp1[(dst.colsb / 4 - 1) : 0] = 0
///   FOR k := 0 TO src1.colsb / 4 - 1
///     FOR n := 0 TO dst.colsb / 4 - 1
///       temp1[n] +=
///         INT64(src1.row[m].float8[4*k+0]) * INT64(src2.row[k].float8[4*n+0])
///         + INT64(src1.row[m].float8[4*k+1]) * INT64(src2.row[k].float8[4*n+1])
///         + INT64(src1.row[m].float8[4*k+2]) * INT64(src2.row[k].float8[4*n+2])
///         + INT64(src1.row[m].float8[4*k+3]) * INT64(src2.row[k].float8[4*n+3])
///     ENDFOR
///   ENDFOR
///   FOR n := 0 TO dst.colsb / 4 - 1
///     tmp.row[m].fp32[n] = dst.row[m].fp32[n] + FP32(temp1[n])
///   ENDFOR
/// write_row_and_zero(dst, m, tmp, dst.colsb)
/// zero_upper_rows(dst, dst.rows)
/// zero_tileconfig_start()
/// \endcode
///
/// This intrinsic corresponds to the \c TDPHF8PS instruction.
///
/// \param dst
///    The destination tile. Max size is 1024 Bytes.
/// \param src1
///    The 1st source tile. Max size is 1024 Bytes.
/// \param src2
///    The 2nd source tile. Max size is 1024 Bytes.
__DEFAULT_FN_ATTRS_FP8 static void
__tile_dphf8ps(__tile1024i *dst, __tile1024i src1, __tile1024i src2) {
  dst->tile = _tile_dphf8ps_internal(src1.row, src2.col, src1.col, dst->tile,
                                     src1.tile, src2.tile);
}

#define _tile_dpbf8ps(dst, src1, src2)                                         \
  __builtin_ia32_tdpbf8ps((dst), (src1), (src2))
#define _tile_dpbhf8ps(dst, src1, src2)                                        \
  __builtin_ia32_tdpbhf8ps((dst), (src1), (src2))
#define _tile_dphbf8ps(dst, src1, src2)                                        \
  __builtin_ia32_tdphbf8ps((dst), (src1), (src2))
#define _tile_dphf8ps(dst, src1, src2)                                         \
  __builtin_ia32_tdphf8ps((dst), (src1), (src2))

#undef __DEFAULT_FN_ATTRS_FP8

#endif /* __x86_64__ */
#endif /* __AMXFP8INTRIN_H */
