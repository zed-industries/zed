/***********************************************************
Copyright 1987 by Digital Equipment Corporation, Maynard, Massachusetts.

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the name of Digital not be
used in advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

DIGITAL DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING
ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL
DIGITAL BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR
ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
SOFTWARE.

******************************************************************/

#ifndef FONT_H
#define FONT_H

#include <X11/Xfuncproto.h>

#ifndef BitmapFormatByteOrderMask
#include	"fsmasks.h"
#endif

/* data structures */
#ifndef _XTYPEDEF_FONTPTR
typedef struct _Font *FontPtr;
#define _XTYPEDEF_FONTPTR
#endif

typedef struct _FontInfo *FontInfoPtr;
typedef struct _FontProp *FontPropPtr;
typedef struct _ExtentInfo *ExtentInfoPtr;
typedef struct _FontPathElement *FontPathElementPtr;

#ifndef _XTYPEDEF_CHARINFOPTR
typedef struct _CharInfo *CharInfoPtr;
#define _XTYPEDEF_CHARINFOPTR
#endif

typedef struct _FontNames *FontNamesPtr;
typedef struct _FontResolution *FontResolutionPtr;

#define NullCharInfo	((CharInfoPtr) 0)
#define NullFont	((FontPtr) 0)
#define NullFontInfo	((FontInfoPtr) 0)

 /* draw direction */
#define LeftToRight 0
#define RightToLeft 1
#define BottomToTop 2
#define TopToBottom 3
typedef int DrawDirection;

#define NO_SUCH_CHAR	-1


#define	FontAliasType	0x1000

#define	AllocError	80
#define	StillWorking	81
#define	FontNameAlias	82
#define	BadFontName	83
#define	Suspended	84
#define	Successful	85
#define	BadFontPath	86
#define	BadCharRange	87
#define	BadFontFormat	88
#define	FPEResetFailed	89	/* for when an FPE reset won't work */

/* OpenFont flags */
#define FontLoadInfo	0x0001
#define FontLoadProps	0x0002
#define FontLoadMetrics	0x0004
#define FontLoadBitmaps	0x0008
#define FontLoadAll	0x000f
#define	FontOpenSync	0x0010
#define FontReopen	0x0020

/* Query flags */
#define	LoadAll		0x1
#define	FinishRamge	0x2
#define       EightBitFont    0x4
#define       SixteenBitFont  0x8

/* Glyph Caching Modes */
#define CACHING_OFF 0
#define CACHE_16_BIT_GLYPHS 1
#define CACHE_ALL_GLYPHS 2
#define DEFAULT_GLYPH_CACHING_MODE CACHE_16_BIT_GLYPHS
extern int glyphCachingMode;

struct _Client;

extern int StartListFontsWithInfo(
    struct _Client * /*client*/,
    int /*length*/,
    unsigned char * /*pattern*/,
    int /*max_names*/
);

extern FontNamesPtr MakeFontNamesRecord(
    unsigned /* size */
);

extern void FreeFontNames(
    FontNamesPtr /* pFN*/
);

extern int  AddFontNamesName(
    FontNamesPtr /* names */,
    char * /* name */,
    int /* length */
);

#if 0 /* unused */
extern int  FontToFSError();
extern FontResolutionPtr GetClientResolution();
#endif

typedef struct _FontPatternCache    *FontPatternCachePtr;

extern FontPatternCachePtr  MakeFontPatternCache (
    void
);

extern void		    FreeFontPatternCache (
    FontPatternCachePtr /* cache */
);

extern void		    EmptyFontPatternCache (
    FontPatternCachePtr /* cache */
);

extern void		    CacheFontPattern (
    FontPatternCachePtr /* cache */,
    const char * /* pattern */,
    int /* patlen */,
    FontPtr /* pFont */
);
extern _X_EXPORT FontResolutionPtr GetClientResolutions(
    int * /* num */
);

extern FontPtr		    FindCachedFontPattern (
    FontPatternCachePtr /* cache */,
    const char * /* pattern */,
    int /* patlen */
);

extern void		    RemoveCachedFontPattern (
    FontPatternCachePtr /* cache */,
    FontPtr /* pFont */
);

typedef enum {
    Linear8Bit, TwoD8Bit, Linear16Bit, TwoD16Bit
}           FontEncoding;

#endif				/* FONT_H */
