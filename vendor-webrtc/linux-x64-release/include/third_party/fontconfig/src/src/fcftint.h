/*
 * Copyright © 2007 Keith Packard
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that copyright
 * notice and this permission notice appear in supporting documentation, and
 * that the name of the copyright holders not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  The copyright holders make no representations
 * about the suitability of this software for any purpose.  It is provided "as
 * is" without express or implied warranty.
 *
 * THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

#ifndef _FCFTINT_H_
#define _FCFTINT_H_

#include <fontconfig/fcfreetype.h>

#if (__GNUC__ > 3 || (__GNUC__ == 3 && __GNUC_MINOR__ >= 3)) && defined(__ELF__) && !defined(__sun)
#define FcPrivate		__attribute__((__visibility__("hidden")))
#define HAVE_GNUC_ATTRIBUTE 1
#include "fcftalias.h"
#elif defined(__SUNPRO_C) && (__SUNPRO_C >= 0x550)
#define FcPrivate		__hidden
#else /* not gcc >= 3.3 and not Sun Studio >= 8 */
#define FcPrivate
#endif

/* fcfreetype.c */
FcPrivate FcBool
FcFreeTypeIsExclusiveLang (const FcChar8  *lang);

FcPrivate FcBool
FcFreeTypeHasLang (FcPattern *pattern, const FcChar8 *lang);

FcPrivate FcChar32
FcFreeTypeUcs4ToPrivate (FcChar32 ucs4, const FcCharMap *map);

FcPrivate FcChar32
FcFreeTypePrivateToUcs4 (FcChar32 private, const FcCharMap *map);

FcPrivate const FcCharMap *
FcFreeTypeGetPrivateMap (FT_Encoding encoding);

#endif /* _FCFTINT_H_ */
