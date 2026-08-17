/*
 * Copyright © 2001 Keith Packard
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of Keith Packard not be used in
 * advertising or publicity pertaining to distribution of the software without
 * specific, written prior permission.  Keith Packard makes no
 * representations about the suitability of this software for any purpose.  It
 * is provided "as is" without express or implied warranty.
 *
 * KEITH PACKARD DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL KEITH PACKARD BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */

#ifndef _XFTCOMPAT_H_
#define _XFTCOMPAT_H_
#include <X11/Xfuncproto.h>

/*
 * Compatibility definitions -- map Fc names to Xft names
 */

typedef FcChar8	    XftChar8;
typedef FcChar16    XftChar16;
typedef FcChar32    XftChar32;

#define XFT_FAMILY	FC_FAMILY
#define XFT_STYLE	FC_STYLE
#define XFT_SLANT	FC_SLANT
#define XFT_WEIGHT	FC_WEIGHT
#define XFT_SIZE	FC_SIZE
#define XFT_PIXEL_SIZE	FC_PIXEL_SIZE
#define XFT_SPACING	FC_SPACING
#define XFT_FOUNDRY	FC_FOUNDRY
#define XFT_ANTIALIAS	FC_ANTIALIAS
#define XFT_FILE	FC_FILE
#define XFT_INDEX	FC_INDEX
#define XFT_RASTERIZER	FC_RASTERIZER
#define XFT_OUTLINE	FC_OUTLINE
#define XFT_SCALABLE	FC_SCALABLE
#define XFT_RGBA	FC_RGBA

/* defaults from resources */
#define XFT_SCALE	FC_SCALE
#define XFT_MINSPACE	FC_MINSPACE
#define XFT_DPI		FC_DPI

/* specific to FreeType rasterizer */
#define XFT_CHAR_WIDTH	FC_CHAR_WIDTH
#define XFT_CHAR_HEIGHT	FC_CHAR_HEIGHT
#define XFT_MATRIX	FC_MATRIX

#define XFT_WEIGHT_LIGHT	FC_WEIGHT_LIGHT
#define XFT_WEIGHT_MEDIUM	FC_WEIGHT_MEDIUM
#define XFT_WEIGHT_DEMIBOLD	FC_WEIGHT_DEMIBOLD
#define XFT_WEIGHT_BOLD		FC_WEIGHT_BOLD
#define XFT_WEIGHT_BLACK	FC_WEIGHT_BLACK

#define XFT_SLANT_ROMAN		FC_SLANT_ROMAN
#define XFT_SLANT_ITALIC	FC_SLANT_ITALIC
#define XFT_SLANT_OBLIQUE	FC_SLANT_OBLIQUE

#define XFT_PROPORTIONAL	FC_PROPORTIONAL
#define XFT_MONO		FC_MONO
#define XFT_CHARCELL		FC_CHARCELL

#define XFT_RGBA_UNKNOWN	FC_RGBA_UNKNOWN
#define XFT_RGBA_RGB		FC_RGBA_RGB
#define XFT_RGBA_BGR		FC_RGBA_BGR
#define XFT_RGBA_VRGB		FC_RGBA_VRGB
#define XFT_RGBA_VBGR		FC_RGBA_VBGR
#define XFT_RGBA_NONE		FC_RGBA_NONE

/*
 * Old constants
 */
#define XFT_ENCODING		"encoding"

typedef FcType XftType;

typedef FcMatrix XftMatrix;

#define XftMatrixInit(m)	FcMatrixInit(m)

typedef FcResult    XftResult;

#define XftResultMatch		FcResultMatch
#define XftResultNoMatch	FcResultNoMatch
#define XftResultTypeMismatch	FcResultTypeMismatch
#define XftResultNoId		FcResultNoId

typedef FcValue		XftValue;
typedef FcPattern	XftPattern;
typedef FcFontSet	XftFontSet;
typedef FcObjectSet	XftObjectSet;

#define XftGlyphExists		XftCharExists

#define XftObjectSetCreate	FcObjectSetCreate
#define XftObjectSetAdd		FcObjectSetAdd
#define XftObjectSetDestroy	FcObjectSetDestroy
#define XftObjectSetVaBuild	FcObjectSetVaBuild
#define XftObjectSetBuild	FcObjectSetBuild

#define XftFontSetMatch		FcFontSetMatch
#define XftFontSetDestroy	FcFontSetDestroy

#define XftMatrixEqual		FcMatrixEqual
#define XftMatrixMultiply	FcMatrixMultiply
#define XftMatrixRotate		FcMatrixRotate
#define XftMatrixScale		FcMatrixScale
#define XftMatrixShear		FcMatrixShear

#define XftPatternCreate	FcPatternCreate
#define XftPatternDuplicate	FcPatternDuplicate
#define XftValueDestroy		FcValueDestroy
#define XftValueListDestroy	FcValueListDestroy
#define XftPatternDestroy	FcPatternDestroy
#define XftPatternFind		FcPatternFind
#define XftPatternAdd		FcPatternAdd
#define XftPatternGet		FcPatternGet
#define XftPatternDel		FcPatternDel
#define XftPatternAddInteger	FcPatternAddInteger
#define XftPatternAddDouble	FcPatternAddDouble
#define XftPatternAddString(p,e,s)	FcPatternAddString(p,e,(FcChar8 *)(s))
#define XftPatternAddMatrix	FcPatternAddMatrix
#define XftPatternAddBool	FcPatternAddBool
#define XftPatternGetInteger	FcPatternGetInteger
#define XftPatternGetDouble	FcPatternGetDouble
#define XftPatternGetString(p,e,i,n)	FcPatternGetString(p,e,i,(FcChar8 **) (n))
#define XftPatternGetMatrix	FcPatternGetMatrix
#define XftPatternGetBool	FcPatternGetBool
#define XftPatternVaBuild	FcPatternVaBuild
#define XftPatternBuild		FcPatternBuild

#define XftUtf8ToUcs4		FcUtf8ToUcs4
#define XftUtf8Len		FcUtf8Len

#define XftTypeVoid	FcTypeVoid
#define XftTypeInteger	FcTypeInteger
#define XftTypeDouble	FcTypeDouble
#define XftTypeString	FcTypeString
#define XftTypeBool	FcTypeBool
#define XftTypeMatrix	FcTypeMatrix

#define XftConfigSubstitute(p) FcConfigSubstitute (0, p, FcMatchPattern)

_XFUNCPROTOBEGIN

FcBool
XftNameUnparse (XftPattern *pat, char *dest, int len);

_XFUNCPROTOEND

#endif /* _XFTCOMPAT_H_ */
