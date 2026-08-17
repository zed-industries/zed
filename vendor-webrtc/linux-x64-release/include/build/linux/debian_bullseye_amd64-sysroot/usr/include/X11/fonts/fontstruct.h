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

#ifndef FONTSTR_H
#define FONTSTR_H

#include <X11/Xproto.h>
#include "font.h"
#include <X11/Xfuncproto.h>
#include <X11/Xdefs.h>

/*
 * This version of the server font data structure is only for describing
 * the in memory data structure. The file structure is not necessarily a
 * copy of this. That is up to the compiler and the OS layer font loading
 * machinery.
 */

#define GLYPHPADOPTIONS 4	/* 1, 2, 4, or 8 */

typedef struct _FontProp {
    long        name;
    long        value;		/* assumes ATOM is not larger than INT32 */
}           FontPropRec;

typedef struct _FontResolution {
    unsigned short x_resolution;
    unsigned short y_resolution;
    unsigned short point_size;
}           FontResolutionRec;

typedef struct _ExtentInfo {
    DrawDirection drawDirection;
    int         fontAscent;
    int         fontDescent;
    int         overallAscent;
    int         overallDescent;
    int         overallWidth;
    int         overallLeft;
    int         overallRight;
}           ExtentInfoRec;

typedef struct _CharInfo {
    xCharInfo   metrics;	/* info preformatted for Queries */
    char       *bits;		/* pointer to glyph image */
}           CharInfoRec;

/*
 * Font is created at font load time. It is specific to a single encoding.
 * e.g. not all of the glyphs in a font may be part of a single encoding.
 */

typedef struct _FontInfo {
    unsigned short firstCol;
    unsigned short lastCol;
    unsigned short firstRow;
    unsigned short lastRow;
    unsigned short defaultCh;
    unsigned int noOverlap:1;
    unsigned int terminalFont:1;
    unsigned int constantMetrics:1;
    unsigned int constantWidth:1;
    unsigned int inkInside:1;
    unsigned int inkMetrics:1;
    unsigned int allExist:1;
    unsigned int drawDirection:2;
    unsigned int cachable:1;
    unsigned int anamorphic:1;
    short       maxOverlap;
    short       pad;
    xCharInfo   maxbounds;
    xCharInfo   minbounds;
    xCharInfo   ink_maxbounds;
    xCharInfo   ink_minbounds;
    short       fontAscent;
    short       fontDescent;
    int         nprops;
    FontPropPtr props;
    char       *isStringProp;
}           FontInfoRec;

typedef struct _Font {
    int         refcnt;
    FontInfoRec info;
    char        bit;
    char        byte;
    char        glyph;
    char        scan;
    fsBitmapFormat format;
    int         (*get_glyphs) (FontPtr         /* font */,
			       unsigned long   /* count */,
			       unsigned char * /* chars */,
			       FontEncoding    /* encoding */,
			       unsigned long * /* count */,
			       CharInfoPtr *   /* glyphs */);
    int         (*get_metrics) (FontPtr         /* font */,
				unsigned long   /* count */,
				unsigned char * /* chars */,
				FontEncoding    /* encoding */,
				unsigned long * /* count */,
				xCharInfo **    /* glyphs */);
    void        (*unload_font) (FontPtr         /* font */);
    void        (*unload_glyphs) (FontPtr         /* font */);
    FontPathElementPtr fpe;
    void        *svrPrivate;
    void        *fontPrivate;
    void        *fpePrivate;
    int		maxPrivate;
    void        **devPrivates;
}           FontRec;

#define FontGetPrivate(pFont,n) ((n) > (pFont)->maxPrivate ? (void *) 0 : \
			     (pFont)->devPrivates[n])

#define FontSetPrivate(pFont,n,ptr) ((n) > (pFont)->maxPrivate ? \
			_FontSetNewPrivate (pFont, n, ptr) : \
			((((pFont)->devPrivates[n] = (ptr)) != 0) || TRUE))

typedef struct _FontNames {
    int         nnames;
    int         size;
    int        *length;
    char      **names;
}           FontNamesRec;


/* External view of font paths */
typedef struct _FontPathElement {
    int         name_length;
#if FONT_PATH_ELEMENT_NAME_CONST
    const
#endif
    char        *name;
    int         type;
    int         refcount;
    void        *private;
}           FontPathElementRec;

typedef Bool (*NameCheckFunc) (const char *name);
typedef int (*InitFpeFunc) (FontPathElementPtr fpe);
typedef int (*FreeFpeFunc) (FontPathElementPtr fpe);
typedef int (*ResetFpeFunc) (FontPathElementPtr fpe);
typedef int (*OpenFontFunc) ( void *client,
			      FontPathElementPtr fpe,
			      Mask flags,
			      const char* name,
			      int namelen,
			      fsBitmapFormat format,
			      fsBitmapFormatMask fmask,
			      XID id,
			      FontPtr* pFont,
			      char** aliasName,
			      FontPtr non_cachable_font);
typedef void (*CloseFontFunc) (FontPathElementPtr fpe, FontPtr pFont);
typedef int (*ListFontsFunc) (void *client,
			      FontPathElementPtr fpe,
			      const char* pat,
			      int len,
			      int max,
			      FontNamesPtr names);

typedef int (*StartLfwiFunc) (void *client,
			      FontPathElementPtr fpe,
			      const char* pat,
			      int len,
			      int max,
			      void ** privatep);

typedef int (*NextLfwiFunc) (void *client,
			     FontPathElementPtr fpe,
			     char** name,
			     int* namelen,
			     FontInfoPtr* info,
			     int* numFonts,
			     void *private);

typedef int (*WakeupFpeFunc) (FontPathElementPtr fpe,
			      unsigned long* LastSelectMask);

typedef void (*ClientDiedFunc) (void *client,
			       FontPathElementPtr fpe);

typedef int (*LoadGlyphsFunc) (void *client,
			       FontPtr pfont,
			       Bool range_flag,
			       unsigned int nchars,
			       int item_size,
			       unsigned char* data);

typedef int (*StartLaFunc) (void *client,
			    FontPathElementPtr fpe,
			    const char* pat,
			    int len,
			    int max,
			    void ** privatep);

typedef int (*NextLaFunc) (void *client,
			   FontPathElementPtr fpe,
			   char** namep,
			   int* namelenp,
			   char** resolvedp,
			   int* resolvedlenp,
			   void *private);

typedef void (*SetPathFunc)(void);

typedef struct _FPEFunctions {
    NameCheckFunc       name_check;
    InitFpeFunc 	init_fpe;
    ResetFpeFunc	reset_fpe;
    FreeFpeFunc         free_fpe;
    OpenFontFunc        open_font;
    CloseFontFunc       close_font;
    ListFontsFunc       list_fonts;
    StartLaFunc         start_list_fonts_and_aliases;
    NextLaFunc          list_next_font_or_alias;
    StartLfwiFunc       start_list_fonts_with_info;
    NextLfwiFunc        list_next_font_with_info;
    WakeupFpeFunc       wakeup_fpe;
    ClientDiedFunc 	client_died;
		/* for load_glyphs, range_flag = 0 ->
			nchars = # of characters in data
			item_size = bytes/char
			data = list of characters
		   range_flag = 1 ->
			nchars = # of fsChar2b's in data
			item_size is ignored
			data = list of fsChar2b's */
    LoadGlyphsFunc	load_glyphs;
    SetPathFunc		set_path_hook;
} FPEFunctionsRec, FPEFunctions;

/*
 * Various macros for computing values based on contents of
 * the above structures
 */

#define	GLYPHWIDTHPIXELS(pci) \
	((pci)->metrics.rightSideBearing - (pci)->metrics.leftSideBearing)

#define	GLYPHHEIGHTPIXELS(pci) \
 	((pci)->metrics.ascent + (pci)->metrics.descent)

#define	GLYPHWIDTHBYTES(pci)	(((GLYPHWIDTHPIXELS(pci))+7) >> 3)

#define GLYPHWIDTHPADDED(bc)	(((bc)+7) & ~0x7)

#define BYTES_PER_ROW(bits, nbytes) \
	((nbytes) == 1 ? (((bits)+7)>>3)	/* pad to 1 byte */ \
	:(nbytes) == 2 ? ((((bits)+15)>>3)&~1)	/* pad to 2 bytes */ \
	:(nbytes) == 4 ? ((((bits)+31)>>3)&~3)	/* pad to 4 bytes */ \
	:(nbytes) == 8 ? ((((bits)+63)>>3)&~7)	/* pad to 8 bytes */ \
	: 0)

#define BYTES_FOR_GLYPH(ci,pad)	(GLYPHHEIGHTPIXELS(ci) * \
				 BYTES_PER_ROW(GLYPHWIDTHPIXELS(ci),pad))
/*
 * Macros for computing different bounding boxes for fonts; from
 * the font protocol
 */

#define FONT_MAX_ASCENT(pi)	((pi)->fontAscent > (pi)->ink_maxbounds.ascent ? \
			    (pi)->fontAscent : (pi)->ink_maxbounds.ascent)
#define FONT_MAX_DESCENT(pi)	((pi)->fontDescent > (pi)->ink_maxbounds.descent ? \
			    (pi)->fontDescent : (pi)->ink_maxbounds.descent)
#define FONT_MAX_HEIGHT(pi)	(FONT_MAX_ASCENT(pi) + FONT_MAX_DESCENT(pi))
#define FONT_MIN_LEFT(pi)	((pi)->ink_minbounds.leftSideBearing < 0 ? \
			    (pi)->ink_minbounds.leftSideBearing : 0)
#define FONT_MAX_RIGHT(pi)	((pi)->ink_maxbounds.rightSideBearing > \
				(pi)->ink_maxbounds.characterWidth ? \
			    (pi)->ink_maxbounds.rightSideBearing : \
				(pi)->ink_maxbounds.characterWidth)
#define FONT_MAX_WIDTH(pi)	(FONT_MAX_RIGHT(pi) - FONT_MIN_LEFT(pi))

#include "fontproto.h"

#endif				/* FONTSTR_H */
