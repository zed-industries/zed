/*
 * Copyright 息 2006 Keith Packard
 * Copyright 息 2010 Behdad Esfahbod
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

/* This header file is supposed to be included in config.h */

/* just a hack to build the fat binaries:
 * https://bugs.freedesktop.org/show_bug.cgi?id=20208
 */
#ifdef __APPLE__
# include <machine/endian.h>
# undef SIZEOF_VOID_P
# undef ALIGNOF_DOUBLE
# ifdef __LP64__
#  define SIZEOF_VOID_P 8
#  define ALIGNOF_DOUBLE 8
# else
#  define SIZEOF_VOID_P 4
#  define ALIGNOF_DOUBLE 4
# endif
#endif
