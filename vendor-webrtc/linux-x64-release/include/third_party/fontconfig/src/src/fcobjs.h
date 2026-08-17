/*
 * fontconfig/src/fcobjs.h
 *
 * Copyright © 2000 Keith Packard
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
/* DON'T REORDER!  The order is part of the cache signature. */
FC_OBJECT (FAMILY,		FcTypeString,	FcCompareFamily)
FC_OBJECT (FAMILYLANG,		FcTypeString,	NULL)
FC_OBJECT (STYLE,		FcTypeString,	FcCompareString)
FC_OBJECT (STYLELANG,		FcTypeString,	NULL)
FC_OBJECT (FULLNAME,		FcTypeString,	NULL)
FC_OBJECT (FULLNAMELANG,	FcTypeString,	NULL)
FC_OBJECT (SLANT,		FcTypeInteger,	FcCompareNumber)
FC_OBJECT (WEIGHT,		FcTypeRange,	FcCompareRange)
FC_OBJECT (WIDTH,		FcTypeRange,	FcCompareRange)
FC_OBJECT (SIZE,		FcTypeRange,	FcCompareSize)
FC_OBJECT (ASPECT,		FcTypeDouble,	NULL)
FC_OBJECT (PIXEL_SIZE,		FcTypeDouble,	FcCompareNumber)
FC_OBJECT (SPACING,		FcTypeInteger,	FcCompareNumber)
FC_OBJECT (FOUNDRY,		FcTypeString,	FcCompareString)
FC_OBJECT (ANTIALIAS,		FcTypeBool,	FcCompareBool)
FC_OBJECT (HINT_STYLE,		FcTypeInteger,	NULL)
FC_OBJECT (HINTING,		FcTypeBool,	NULL)
FC_OBJECT (VERTICAL_LAYOUT,	FcTypeBool,	NULL)
FC_OBJECT (AUTOHINT,		FcTypeBool,	NULL)
FC_OBJECT (GLOBAL_ADVANCE,	FcTypeBool,	NULL)	/* deprecated */
FC_OBJECT (FILE,		FcTypeString,	FcCompareFilename)
FC_OBJECT (INDEX,		FcTypeInteger,	NULL)
FC_OBJECT (RASTERIZER,		FcTypeString,	FcCompareString)	/* deprecated */
FC_OBJECT (OUTLINE,		FcTypeBool,	FcCompareBool)
FC_OBJECT (SCALABLE,		FcTypeBool,	FcCompareBool)
FC_OBJECT (DPI,			FcTypeDouble,	NULL)
FC_OBJECT (RGBA,		FcTypeInteger,	NULL)
FC_OBJECT (SCALE,		FcTypeDouble,	NULL)
FC_OBJECT (MINSPACE,		FcTypeBool,	NULL)
FC_OBJECT (CHARWIDTH,		FcTypeInteger,	NULL)
FC_OBJECT (CHAR_HEIGHT,		FcTypeInteger,	NULL)
FC_OBJECT (MATRIX,		FcTypeMatrix,	NULL)
FC_OBJECT (CHARSET,		FcTypeCharSet,	FcCompareCharSet)
FC_OBJECT (LANG,		FcTypeLangSet,	FcCompareLang)
FC_OBJECT (FONTVERSION,		FcTypeInteger,	FcCompareNumber)
FC_OBJECT (CAPABILITY,		FcTypeString,	NULL)
FC_OBJECT (FONTFORMAT,		FcTypeString,	FcCompareString)
FC_OBJECT (EMBOLDEN,		FcTypeBool,	NULL)
FC_OBJECT (EMBEDDED_BITMAP,	FcTypeBool,	NULL)
FC_OBJECT (DECORATIVE,		FcTypeBool,	FcCompareBool)
FC_OBJECT (LCD_FILTER,		FcTypeInteger,	NULL)
FC_OBJECT (NAMELANG,		FcTypeString,	NULL)
FC_OBJECT (FONT_FEATURES,	FcTypeString,	NULL)
FC_OBJECT (PRGNAME,		FcTypeString,	NULL)
FC_OBJECT (HASH,		FcTypeString,	NULL)	/* deprecated */
FC_OBJECT (POSTSCRIPT_NAME,	FcTypeString,	FcComparePostScript)
FC_OBJECT (COLOR,		FcTypeBool,	FcCompareBool)
FC_OBJECT (SYMBOL,		FcTypeBool,	FcCompareBool)
FC_OBJECT (FONT_VARIATIONS,	FcTypeString,	NULL)
FC_OBJECT (VARIABLE,		FcTypeBool,	FcCompareBool)
FC_OBJECT (FONT_HAS_HINT,	FcTypeBool,	FcCompareBool)
FC_OBJECT (ORDER,		FcTypeInteger,	FcCompareNumber)
FC_OBJECT (DESKTOP_NAME,	FcTypeString,	NULL)
FC_OBJECT (NAMED_INSTANCE,	FcTypeBool,	FcCompareBool)
FC_OBJECT (FONT_WRAPPER,	FcTypeString,	FcCompareString)
/* ^-------------- Add new objects here. */
