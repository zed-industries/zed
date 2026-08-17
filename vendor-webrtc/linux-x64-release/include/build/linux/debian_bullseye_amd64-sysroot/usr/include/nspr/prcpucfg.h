/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * This file is used by not only Linux but also other glibc systems
 * such as GNU/Hurd and GNU/k*BSD.
 */

#ifndef nspr_cpucfg___
#define nspr_cpucfg___

#ifndef XP_UNIX
#define XP_UNIX
#endif

#if !defined(LINUX) && defined(__linux__)
#define LINUX
#endif

#ifdef __FreeBSD_kernel__
#define PR_AF_INET6 28  /* same as AF_INET6 */
#elif defined(__GNU__)
#define PR_AF_INET6 26  /* same as AF_INET6 */
#else
#define PR_AF_INET6 10  /* same as AF_INET6 */
#endif

#ifdef __powerpc64__

#ifdef __LITTLE_ENDIAN__
#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN
#else
#undef  IS_LITTLE_ENDIAN
#define IS_BIG_ENDIAN    1
#endif
#define IS_64

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   8
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   8
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    64
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    64

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   6
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   6

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    8
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 8
#define PR_ALIGN_OF_WORD    8

#define PR_BYTES_PER_WORD_LOG2   3
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__powerpc__)

#ifdef __LITTLE_ENDIAN__
#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN
#else
#undef  IS_LITTLE_ENDIAN
#define IS_BIG_ENDIAN    1
#endif

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__alpha)

#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN
#define IS_64

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   8
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   8
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    64
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    64

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   6
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   6

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    8
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 8
#define PR_ALIGN_OF_WORD    8

#define PR_BYTES_PER_WORD_LOG2  3
#define PR_BYTES_PER_DWORD_LOG2 3

#elif defined(__ia64__)

#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN
#define IS_64

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   8
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   8
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    64
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    64

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   6
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   6

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    8
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 8
#define PR_ALIGN_OF_WORD    8

#define PR_BYTES_PER_WORD_LOG2  3
#define PR_BYTES_PER_DWORD_LOG2 3

#elif defined(__x86_64__)

#ifdef __ILP32__

#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#else

#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN
#define IS_64

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   8
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   8
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    64
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    64

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   6
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   6

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    8
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 8
#define PR_ALIGN_OF_WORD    8

#define PR_BYTES_PER_WORD_LOG2  3
#define PR_BYTES_PER_DWORD_LOG2 3

#endif

#elif defined(__mc68000__)

#undef  IS_LITTLE_ENDIAN
#define IS_BIG_ENDIAN 1

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     2
#define PR_ALIGN_OF_LONG    2
#define PR_ALIGN_OF_INT64   2
#define PR_ALIGN_OF_FLOAT   2
#define PR_ALIGN_OF_DOUBLE  2
#define PR_ALIGN_OF_POINTER 2
#define PR_ALIGN_OF_WORD    2

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__sparc__) && defined (__arch64__)

#undef	IS_LITTLE_ENDIAN
#define	IS_BIG_ENDIAN 1
#define IS_64

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   8
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   8
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    64
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    64

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   6
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   6

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_LONG    8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 8
#define PR_ALIGN_OF_WORD    8

#define PR_BYTES_PER_WORD_LOG2   3
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__sparc__)

#undef	IS_LITTLE_ENDIAN
#define	IS_BIG_ENDIAN 1

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__i386__)

#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   4
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  4
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__mips__)

/* For _ABI64 */
#include <sgidefs.h>

#ifdef __MIPSEB__
#define IS_BIG_ENDIAN 1
#undef  IS_LITTLE_ENDIAN
#elif defined(__MIPSEL__)
#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN
#else
#error "Unknown MIPS endianness."
#endif

#if _MIPS_SIM == _ABI64

#define IS_64

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   8
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   8
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    64
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    64

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   6
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   6

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    8
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 8
#define PR_ALIGN_OF_WORD    8

#define PR_BYTES_PER_WORD_LOG2   3
#define PR_BYTES_PER_DWORD_LOG2  3

#else /* _ABI64 */

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#endif /* _ABI64 */

#elif defined(__arm__)

#ifdef __ARMEB__
#undef  IS_LITTLE_ENDIAN
#define IS_BIG_ENDIAN 1
#elif defined(__ARMEL__)
#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN
#else
#error "Unknown ARM endianness."
#endif

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   4
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  4
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__aarch64__)

#ifdef __AARCH64EB__
#undef  IS_LITTLE_ENDIAN
#define IS_BIG_ENDIAN 1
#elif defined(__AARCH64EL__)
#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN
#else
#error "Unknown Aarch64 endianness."
#endif
#define IS_64

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   8
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   8
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    64
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    64

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   6
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   6

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    8
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 8
#define PR_ALIGN_OF_WORD    8

#define PR_BYTES_PER_WORD_LOG2  3
#define PR_BYTES_PER_DWORD_LOG2 3

#elif defined(__hppa__)

#undef  IS_LITTLE_ENDIAN
#define IS_BIG_ENDIAN    1

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__s390x__)

#define IS_BIG_ENDIAN 1
#undef  IS_LITTLE_ENDIAN
#define IS_64

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   8
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   8
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    64
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    64

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   6
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   6

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    8
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 8
#define PR_ALIGN_OF_WORD    8

#define PR_BYTES_PER_WORD_LOG2   3
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__s390__)

#define IS_BIG_ENDIAN 1
#undef  IS_LITTLE_ENDIAN

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   4
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  4
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__sh__)

#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   4
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  4
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__avr32__)

#undef  IS_LITTLE_ENDIAN
#define IS_BIG_ENDIAN 1

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   4
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  4
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__m32r__)

#undef  IS_LITTLE_ENDIAN
#define IS_BIG_ENDIAN 1

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   4
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  4
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__or1k__)

#undef  IS_LITTLE_ENDIAN
#define IS_BIG_ENDIAN 1

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   4
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  4
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__riscv) && (__riscv_xlen == 32)

#undef  IS_BIG_ENDIAN
#define IS_LITTLE_ENDIAN 1
#undef  IS_64

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2  2
#define PR_BYTES_PER_DWORD_LOG2 3

#elif defined(__riscv) && (__riscv_xlen == 64)

#undef  IS_BIG_ENDIAN
#define IS_LITTLE_ENDIAN 1
#define IS_64

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   8
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   8
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    64
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    64

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   6
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   6

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    8
#define PR_ALIGN_OF_INT64   8
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 8
#define PR_ALIGN_OF_WORD    8

#define PR_BYTES_PER_WORD_LOG2  3
#define PR_BYTES_PER_DWORD_LOG2 3

#elif defined(__arc__)

#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   4
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  4
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__nios2__) || defined(__microblaze__) || defined(__nds32__) || \
      defined(__xtensa__)

#if defined(__microblaze__) && defined(__BIG_ENDIAN__)
#define IS_BIG_ENDIAN 1
#undef  IS_LITTLE_ENDIAN
#else
#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN
#endif

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  8
#define PR_BYTES_PER_LONG   4
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   4
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   64
#define PR_BITS_PER_LONG    32
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    32

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  6
#define PR_BITS_PER_LONG_LOG2   5
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   5

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    4
#define PR_ALIGN_OF_INT64   4
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  4
#define PR_ALIGN_OF_POINTER 4
#define PR_ALIGN_OF_WORD    4

#define PR_BYTES_PER_WORD_LOG2   2
#define PR_BYTES_PER_DWORD_LOG2  3

#elif defined(__e2k__)

#define IS_LITTLE_ENDIAN 1
#undef  IS_BIG_ENDIAN

#define IS_64

#define PR_BYTES_PER_BYTE   1
#define PR_BYTES_PER_SHORT  2
#define PR_BYTES_PER_INT    4
#define PR_BYTES_PER_INT64  4
#define PR_BYTES_PER_LONG   8
#define PR_BYTES_PER_FLOAT  4
#define PR_BYTES_PER_DOUBLE 8
#define PR_BYTES_PER_WORD   8
#define PR_BYTES_PER_DWORD  8

#define PR_BITS_PER_BYTE    8
#define PR_BITS_PER_SHORT   16
#define PR_BITS_PER_INT     32
#define PR_BITS_PER_INT64   32
#define PR_BITS_PER_LONG    64
#define PR_BITS_PER_FLOAT   32
#define PR_BITS_PER_DOUBLE  64
#define PR_BITS_PER_WORD    64

#define PR_BITS_PER_BYTE_LOG2   3
#define PR_BITS_PER_SHORT_LOG2  4
#define PR_BITS_PER_INT_LOG2    5
#define PR_BITS_PER_INT64_LOG2  5
#define PR_BITS_PER_LONG_LOG2   6
#define PR_BITS_PER_FLOAT_LOG2  5
#define PR_BITS_PER_DOUBLE_LOG2 6
#define PR_BITS_PER_WORD_LOG2   6

#define PR_ALIGN_OF_SHORT   2
#define PR_ALIGN_OF_INT     4
#define PR_ALIGN_OF_LONG    8
#define PR_ALIGN_OF_INT64   4
#define PR_ALIGN_OF_FLOAT   4
#define PR_ALIGN_OF_DOUBLE  8
#define PR_ALIGN_OF_POINTER 8
#define PR_ALIGN_OF_WORD    8

#define PR_BYTES_PER_WORD_LOG2  3
#define PR_BYTES_PER_DWORD_LOG2 3

#else

#error "Unknown CPU architecture"

#endif

#ifndef HAVE_LONG_LONG
#define	HAVE_LONG_LONG
#endif
#if PR_ALIGN_OF_DOUBLE == 8
#define HAVE_ALIGNED_DOUBLES
#endif
#if PR_ALIGN_OF_INT64 == 8
#define HAVE_ALIGNED_LONGLONGS
#endif

#ifndef NO_NSPR_10_SUPPORT

#define BYTES_PER_BYTE		PR_BYTES_PER_BYTE
#define BYTES_PER_SHORT 	PR_BYTES_PER_SHORT
#define BYTES_PER_INT 		PR_BYTES_PER_INT
#define BYTES_PER_INT64		PR_BYTES_PER_INT64
#define BYTES_PER_LONG		PR_BYTES_PER_LONG
#define BYTES_PER_FLOAT		PR_BYTES_PER_FLOAT
#define BYTES_PER_DOUBLE	PR_BYTES_PER_DOUBLE
#define BYTES_PER_WORD		PR_BYTES_PER_WORD
#define BYTES_PER_DWORD		PR_BYTES_PER_DWORD

#define BITS_PER_BYTE		PR_BITS_PER_BYTE
#define BITS_PER_SHORT		PR_BITS_PER_SHORT
#define BITS_PER_INT		PR_BITS_PER_INT
#define BITS_PER_INT64		PR_BITS_PER_INT64
#define BITS_PER_LONG		PR_BITS_PER_LONG
#define BITS_PER_FLOAT		PR_BITS_PER_FLOAT
#define BITS_PER_DOUBLE		PR_BITS_PER_DOUBLE
#define BITS_PER_WORD		PR_BITS_PER_WORD

#define BITS_PER_BYTE_LOG2	PR_BITS_PER_BYTE_LOG2
#define BITS_PER_SHORT_LOG2	PR_BITS_PER_SHORT_LOG2
#define BITS_PER_INT_LOG2	PR_BITS_PER_INT_LOG2
#define BITS_PER_INT64_LOG2	PR_BITS_PER_INT64_LOG2
#define BITS_PER_LONG_LOG2	PR_BITS_PER_LONG_LOG2
#define BITS_PER_FLOAT_LOG2	PR_BITS_PER_FLOAT_LOG2
#define BITS_PER_DOUBLE_LOG2 	PR_BITS_PER_DOUBLE_LOG2
#define BITS_PER_WORD_LOG2	PR_BITS_PER_WORD_LOG2

#define ALIGN_OF_SHORT		PR_ALIGN_OF_SHORT
#define ALIGN_OF_INT		PR_ALIGN_OF_INT
#define ALIGN_OF_LONG		PR_ALIGN_OF_LONG
#define ALIGN_OF_INT64		PR_ALIGN_OF_INT64
#define ALIGN_OF_FLOAT		PR_ALIGN_OF_FLOAT
#define ALIGN_OF_DOUBLE		PR_ALIGN_OF_DOUBLE
#define ALIGN_OF_POINTER	PR_ALIGN_OF_POINTER
#define ALIGN_OF_WORD		PR_ALIGN_OF_WORD

#define BYTES_PER_WORD_LOG2	PR_BYTES_PER_WORD_LOG2
#define BYTES_PER_DWORD_LOG2	PR_BYTES_PER_DWORD_LOG2
#define WORDS_PER_DWORD_LOG2	PR_WORDS_PER_DWORD_LOG2

#endif /* NO_NSPR_10_SUPPORT */

#endif /* nspr_cpucfg___ */
