/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prbit_h___
#define prbit_h___

#include "prtypes.h"
PR_BEGIN_EXTERN_C

/*
** Replace compare/jump/add/shift sequence with compiler built-in/intrinsic
** functions.
*/
#if defined(_WIN32) && (_MSC_VER >= 1300) && \
    (defined(_M_IX86) || defined(_M_X64) || defined(_M_ARM) || \
     defined(_M_ARM64))
# include <intrin.h>
# pragma  intrinsic(_BitScanForward,_BitScanReverse)
__forceinline static int __prBitScanForward32(unsigned int val)
{
    unsigned long idx;
    _BitScanForward(&idx, (unsigned long)val);
    return( (int)idx );
}
__forceinline static int __prBitScanReverse32(unsigned int val)
{
    unsigned long idx;
    _BitScanReverse(&idx, (unsigned long)val);
    return( (int)(31-idx) );
}
# define pr_bitscan_ctz32(val)  __prBitScanForward32(val)
# define pr_bitscan_clz32(val)  __prBitScanReverse32(val)
# define  PR_HAVE_BUILTIN_BITSCAN32
#elif ((__GNUC__ >= 4) || (__GNUC__ == 3 && __GNUC_MINOR__ >= 4)) && \
       (defined(__i386__) || defined(__x86_64__) || defined(__arm__) || \
        defined(__aarch64__))
# define pr_bitscan_ctz32(val)  __builtin_ctz(val)
# define pr_bitscan_clz32(val)  __builtin_clz(val)
# define  PR_HAVE_BUILTIN_BITSCAN32
#endif /* MSVC || GCC */

/*
** A prbitmap_t is a long integer that can be used for bitmaps
*/
typedef unsigned long prbitmap_t;

#define PR_TEST_BIT(_map,_bit) \
    ((_map)[(_bit)>>PR_BITS_PER_LONG_LOG2] & (1L << ((_bit) & (PR_BITS_PER_LONG-1))))
#define PR_SET_BIT(_map,_bit) \
    ((_map)[(_bit)>>PR_BITS_PER_LONG_LOG2] |= (1L << ((_bit) & (PR_BITS_PER_LONG-1))))
#define PR_CLEAR_BIT(_map,_bit) \
    ((_map)[(_bit)>>PR_BITS_PER_LONG_LOG2] &= ~(1L << ((_bit) & (PR_BITS_PER_LONG-1))))

/*
** Compute the log of the least power of 2 greater than or equal to n
*/
NSPR_API(PRIntn) PR_CeilingLog2(PRUint32 i);

/*
** Compute the log of the greatest power of 2 less than or equal to n
*/
NSPR_API(PRIntn) PR_FloorLog2(PRUint32 i);

/*
** Macro version of PR_CeilingLog2: Compute the log of the least power of
** 2 greater than or equal to _n. The result is returned in _log2.
*/
#ifdef PR_HAVE_BUILTIN_BITSCAN32
#define PR_CEILING_LOG2(_log2,_n)      \
  PR_BEGIN_MACRO                       \
    PRUint32 j_ = (PRUint32)(_n);      \
    (_log2) = (j_ <= 1 ? 0 : 32 - pr_bitscan_clz32(j_ - 1)); \
  PR_END_MACRO
#else
#define PR_CEILING_LOG2(_log2,_n)   \
  PR_BEGIN_MACRO                    \
    PRUint32 j_ = (PRUint32)(_n);   \
    (_log2) = 0;                    \
    if ((j_) & ((j_)-1))            \
    (_log2) += 1;               \
    if ((j_) >> 16)                 \
    (_log2) += 16, (j_) >>= 16; \
    if ((j_) >> 8)                  \
    (_log2) += 8, (j_) >>= 8;   \
    if ((j_) >> 4)                  \
    (_log2) += 4, (j_) >>= 4;   \
    if ((j_) >> 2)                  \
    (_log2) += 2, (j_) >>= 2;   \
    if ((j_) >> 1)                  \
    (_log2) += 1;               \
  PR_END_MACRO
#endif /* PR_HAVE_BUILTIN_BITSCAN32 */

/*
** Macro version of PR_FloorLog2: Compute the log of the greatest power of
** 2 less than or equal to _n. The result is returned in _log2.
**
** This is equivalent to finding the highest set bit in the word.
*/
#ifdef PR_HAVE_BUILTIN_BITSCAN32
#define PR_FLOOR_LOG2(_log2,_n)     \
  PR_BEGIN_MACRO                    \
    PRUint32 j_ = (PRUint32)(_n);   \
    (_log2) = 31 - pr_bitscan_clz32((j_) | 1); \
  PR_END_MACRO
#else
#define PR_FLOOR_LOG2(_log2,_n)   \
  PR_BEGIN_MACRO                    \
    PRUint32 j_ = (PRUint32)(_n);   \
    (_log2) = 0;                    \
    if ((j_) >> 16)                 \
    (_log2) += 16, (j_) >>= 16; \
    if ((j_) >> 8)                  \
    (_log2) += 8, (j_) >>= 8;   \
    if ((j_) >> 4)                  \
    (_log2) += 4, (j_) >>= 4;   \
    if ((j_) >> 2)                  \
    (_log2) += 2, (j_) >>= 2;   \
    if ((j_) >> 1)                  \
    (_log2) += 1;               \
  PR_END_MACRO
#endif /* PR_HAVE_BUILTIN_BITSCAN32 */

/*
** Macros for rotate left and right. The argument 'a' must be an unsigned
** 32-bit integer type such as PRUint32.
**
** There is no rotate operation in the C Language, so the construct
** (a << 4) | (a >> 28) is frequently used instead. Most compilers convert
** this to a rotate instruction, but MSVC doesn't without a little help.
** To get MSVC to generate a rotate instruction, we have to use the _rotl
** or _rotr intrinsic and use a pragma to make it inline.
**
** Note: MSVC in VS2005 will do an inline rotate instruction on the above
** construct.
*/

#if defined(_MSC_VER) && (defined(_M_IX86) || defined(_M_AMD64) || \
    defined(_M_X64) || defined(_M_ARM) || defined(_M_ARM64))
#include <stdlib.h>
#pragma intrinsic(_rotl, _rotr)
#define PR_ROTATE_LEFT32(a, bits) _rotl(a, bits)
#define PR_ROTATE_RIGHT32(a, bits) _rotr(a, bits)
#else
#define PR_ROTATE_LEFT32(a, bits) (((a) << (bits)) | ((a) >> (32 - (bits))))
#define PR_ROTATE_RIGHT32(a, bits) (((a) >> (bits)) | ((a) << (32 - (bits))))
#endif

PR_END_EXTERN_C
#endif /* prbit_h___ */
