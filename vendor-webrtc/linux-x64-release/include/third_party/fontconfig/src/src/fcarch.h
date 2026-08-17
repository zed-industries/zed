/*
 * Copyright © 2006 Keith Packard
 * Copyright © 2010 Behdad Esfahbod
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of the author(s) not be used in
 * advertising or publicity pertaining to distribution of the software without
 * specific, written prior permission.  The authors make no
 * representations about the suitability of this software for any purpose.  It
 * is provided "as is" without express or implied warranty.
 *
 * THE AUTHOR(S) DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
#ifndef _FCARCH_H_
#define _FCARCH_H_

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

/*
 * Each unique machine architecture needs an entry in this file
 * So far the differences boil down to: endianness, 32 vs 64 bit pointers,
 * and on 32bit ones, whether double is aligned to one word or two words.
 * Those result in the 6 formats listed below.
 *
 * If any of the assertion errors in fcarch.c fail, you need to add a new
 * architecture.  Contact the fontconfig mailing list in that case.
 *
 * name		endianness	pointer-size	double-alignment
 *
 * le32d4	4321		4		4
 * le32d8	4321		4		8
 * le64		4321		8		8
 * be32d4	1234		4		4
 * be32d8	1234		4		8
 * be64		1234		8		8
 */

#if defined(__DARWIN_BYTE_ORDER) && __DARWIN_BYTE_ORDER == __DARWIN_LITTLE_ENDIAN
# define FC_ARCH_ENDIAN "le"
#elif defined(__DARWIN_BYTE_ORDER) && __DARWIN_BYTE_ORDER == __DARWIN_BIG_ENDIAN
# define FC_ARCH_ENDIAN "be"
#elif defined(__DARWIN_BYTE_ORDER) && __DARWIN_BYTE_ORDER == __DARWIN_PDP_ENDIAN
# define FC_ARCH_ENDIAN "pe"
#elif defined(WORDS_BIGENDIAN) && WORDS_BIGENDIAN
# define FC_ARCH_ENDIAN "be"
#else
# define FC_ARCH_ENDIAN "le"
#endif

#if SIZEOF_VOID_P == 4
# if ALIGNOF_DOUBLE == 4
#  define FC_ARCH_SIZE_ALIGN "32d4"
# else /* ALIGNOF_DOUBLE != 4 */
#  define FC_ARCH_SIZE_ALIGN "32d8"
# endif
#else /* SIZEOF_VOID_P != 4 */
# define FC_ARCH_SIZE_ALIGN "64"
#endif

/* config.h might override this */
#ifndef FC_ARCHITECTURE
# define FC_ARCHITECTURE FC_ARCH_ENDIAN FC_ARCH_SIZE_ALIGN
#endif

#endif /* _FCARCH_H_ */
