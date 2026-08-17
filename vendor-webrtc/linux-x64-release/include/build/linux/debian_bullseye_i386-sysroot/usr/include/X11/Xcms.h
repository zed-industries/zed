
/*
 * Code and supporting documentation (c) Copyright 1990 1991 Tektronix, Inc.
 * 	All Rights Reserved
 *
 * This file is a component of an X Window System-specific implementation
 * of Xcms based on the TekColor Color Management System.  Permission is
 * hereby granted to use, copy, modify, sell, and otherwise distribute this
 * software and its documentation for any purpose and without fee, provided
 * that this copyright, permission, and disclaimer notice is reproduced in
 * all copies of this software and in supporting documentation.  TekColor
 * is a trademark of Tektronix, Inc.
 *
 * Tektronix makes no representation about the suitability of this software
 * for any purpose.  It is provided "as is" and with all faults.
 *
 * TEKTRONIX DISCLAIMS ALL WARRANTIES APPLICABLE TO THIS SOFTWARE,
 * INCLUDING THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE.  IN NO EVENT SHALL TEKTRONIX BE LIABLE FOR ANY
 * SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER
 * RESULTING FROM LOSS OF USE, DATA, OR PROFITS, WHETHER IN AN ACTION OF
 * CONTRACT, NEGLIGENCE, OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
 * CONNECTION WITH THE USE OR THE PERFORMANCE OF THIS SOFTWARE.
 *
 *
 *	DESCRIPTION
 *		Public include file for X Color Management System
 */
#ifndef _X11_XCMS_H_
#define _X11_XCMS_H_

#include <X11/Xlib.h>

/* The Xcms structs are full of implicit padding to properly align members.
   We can't clean that up without breaking ABI, so tell clang not to bother
   complaining about it. */
#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wpadded"
#endif

    /*
     * XCMS Status Values
     */
#define XcmsFailure			0
#define XcmsSuccess			1
#define XcmsSuccessWithCompression	2

    /*
     * Color Space Format ID's
     *    Color Space ID's are of XcmsColorFormat type.
     *
     *    bit 31
     *	    0 == Device-Independent
     *	    1 == Device-Dependent
     *
     *    bit 30:
     *	    0 == Registered with X Consortium
     *	    1 == Unregistered
     */
#define XcmsUndefinedFormat	(XcmsColorFormat)0x00000000
#define XcmsCIEXYZFormat	(XcmsColorFormat)0x00000001
#define XcmsCIEuvYFormat	(XcmsColorFormat)0x00000002
#define XcmsCIExyYFormat	(XcmsColorFormat)0x00000003
#define XcmsCIELabFormat	(XcmsColorFormat)0x00000004
#define XcmsCIELuvFormat	(XcmsColorFormat)0x00000005
#define XcmsTekHVCFormat	(XcmsColorFormat)0x00000006
#define XcmsRGBFormat		(XcmsColorFormat)0x80000000
#define XcmsRGBiFormat		(XcmsColorFormat)0x80000001

    /*
     * State of XcmsPerScrnInfo
     */
#define XcmsInitNone		0x00	/* no initialization attempted */
#define XcmsInitSuccess		0x01	/* initialization successful */
#define XcmsInitFailure		0xff	/* failure, use defaults */

#define DisplayOfCCC(ccc)		((ccc)->dpy)
#define ScreenNumberOfCCC(ccc)		((ccc)->screenNumber)
#define VisualOfCCC(ccc)		((ccc)->visual)
#define ClientWhitePointOfCCC(ccc)	(&(ccc)->clientWhitePt)
#define ScreenWhitePointOfCCC(ccc)	(&(ccc)->pPerScrnInfo->screenWhitePt)
#define FunctionSetOfCCC(ccc)		((ccc)->pPerScrnInfo->functionSet)

typedef unsigned long XcmsColorFormat;	/* Color Space Format ID */

typedef double XcmsFloat;

    /*
     * Device RGB
     */
typedef struct {
    unsigned short red;		/* scaled from 0x0000 to 0xffff */
    unsigned short green;	/* scaled from 0x0000 to 0xffff */
    unsigned short blue;	/* scaled from 0x0000 to 0xffff */
} XcmsRGB;

    /*
     * RGB Intensity
     */
typedef struct {
    XcmsFloat red;	/* 0.0 - 1.0 */
    XcmsFloat green;	/* 0.0 - 1.0 */
    XcmsFloat blue;	/* 0.0 - 1.0 */
} XcmsRGBi;

    /*
     * CIE XYZ
     */
typedef struct {
    XcmsFloat X;
    XcmsFloat Y;
    XcmsFloat Z;
} XcmsCIEXYZ;

    /*
     * CIE u'v'Y
     */
typedef struct {
    XcmsFloat u_prime;		/* 0.0 - 1.0 */
    XcmsFloat v_prime;		/* 0.0 - 1.0 */
    XcmsFloat Y;		/* 0.0 - 1.0 */
} XcmsCIEuvY;

    /*
     * CIE xyY
     */
typedef struct {
    XcmsFloat x;		/* 0.0 - 1.0 */
    XcmsFloat y;		/* 0.0 - 1.0 */
    XcmsFloat Y;		/* 0.0 - 1.0 */
} XcmsCIExyY;

    /*
     * CIE L*a*b*
     */
typedef struct {
    XcmsFloat L_star;		/* 0.0 - 100.0 */
    XcmsFloat a_star;
    XcmsFloat b_star;
} XcmsCIELab;

    /*
     * CIE L*u*v*
     */
typedef struct {
    XcmsFloat L_star;		/* 0.0 - 100.0 */
    XcmsFloat u_star;
    XcmsFloat v_star;
} XcmsCIELuv;

    /*
     * TekHVC
     */
typedef struct {
    XcmsFloat H;		/* 0.0 - 360.0 */
    XcmsFloat V;		/* 0.0 - 100.0 */
    XcmsFloat C;		/* 0.0 - 100.0 */
} XcmsTekHVC;

    /*
     * PAD
     */
typedef struct {
    XcmsFloat pad0;
    XcmsFloat pad1;
    XcmsFloat pad2;
    XcmsFloat pad3;
} XcmsPad;


    /*
     * XCMS Color Structure
     */
typedef struct {
    union {
	XcmsRGB RGB;
	XcmsRGBi RGBi;
	XcmsCIEXYZ CIEXYZ;
	XcmsCIEuvY CIEuvY;
	XcmsCIExyY CIExyY;
	XcmsCIELab CIELab;
	XcmsCIELuv CIELuv;
	XcmsTekHVC TekHVC;
	XcmsPad Pad;
    } spec;			/* the color specification	*/
    unsigned long pixel;	/* pixel value (as needed)	*/
    XcmsColorFormat	format;		/* the specification format	*/
} XcmsColor;


    /*
     * XCMS Per Screen related data
     */

typedef struct _XcmsPerScrnInfo {
    XcmsColor	screenWhitePt;	/* Screen White point */
    XPointer	functionSet;	/* pointer to Screen Color Characterization */
				/*      Function Set structure		*/
    XPointer	screenData;	/* pointer to corresponding Screen Color*/
				/*	Characterization Data		*/
    unsigned char state;   /* XcmsInitNone, XcmsInitSuccess, XcmsInitFailure */
    char	pad[3];
} XcmsPerScrnInfo;

typedef struct _XcmsCCC *XcmsCCC;

typedef Status (*XcmsCompressionProc)(		/* Gamut Compression Proc */
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    unsigned int	/* index */,
    Bool*		/* compression_flags_return */
);

typedef Status (*XcmsWhiteAdjustProc)(	 	/* White Point Adjust Proc */
    XcmsCCC		/* ccc */,
    XcmsColor*		/* initial_white_point*/,
    XcmsColor*		/* target_white_point*/,
    XcmsColorFormat	/* target_format */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    Bool*		/* compression_flags_return */
);

    /*
     * XCMS Color Conversion Context
     */
typedef struct _XcmsCCC {
    Display	*dpy;			/* X Display */
    int		screenNumber;		/* X screen number */
    Visual	*visual;		/* X Visual */
    XcmsColor	clientWhitePt;		/* Client White Point */
    XcmsCompressionProc	gamutCompProc;	/* Gamut Compression Function */
    XPointer	gamutCompClientData;	/* Gamut Comp Func Client Data */
    XcmsWhiteAdjustProc	whitePtAdjProc;	/* White Point Adjustment Function */
    XPointer	whitePtAdjClientData;	/* White Pt Adj Func Client Data */
    XcmsPerScrnInfo *pPerScrnInfo;	/* pointer to per screen information */
					/*  associated with the above display */
					/*  screenNumber */
} XcmsCCCRec;

typedef Status (*XcmsScreenInitProc)(	/* Screen Initialization Proc */
    Display*		/* dpy */,
    int			/* screen_number */,
    XcmsPerScrnInfo*	/* screen_info */
);

typedef void (*XcmsScreenFreeProc)(
    XPointer		/* screenData */
);

    /*
     * Function List Pointer -- pointer to an array of function pointers.
     *    The end of list is indicated by a NULL pointer.
     */
/*
 * XXX:  The use of the XcmsConversionProc type is broken.  The
 *       device-independent colour conversion code uses it as:

typedef Status (*XcmsConversionProc)(XcmsCCC, XcmsColor *, XcmsColor *,
				     unsigned int);

 *       while the device-dependent code uses it as:

typedef Status (*XcmsConversionProc)(XcmsCCC, XcmsColor *, unsigned int,
				     Bool *);

 *       Until this is reworked, it's probably best to leave it unprotoized.
 *       The code works regardless.
 */
typedef Status (*XcmsDDConversionProc)( /* using device-dependent version */
    XcmsCCC             /* ccc */,
    XcmsColor*          /* pcolors_in_out */,
    unsigned int        /* ncolors */,
    Bool*               /* pCompressed */
    );

typedef Status (*XcmsDIConversionProc)( /* using device-independent version */
    XcmsCCC             /* ccc */,
    XcmsColor*          /* white_point */,
    XcmsColor*          /* pcolors_in_out */,
    unsigned int        /* ncolors */
    );

typedef XcmsDIConversionProc XcmsConversionProc;
typedef XcmsConversionProc *XcmsFuncListPtr;

typedef int (*XcmsParseStringProc)(	/* Color String Parsing Proc */
    char*		/* color_string */,
    XcmsColor*		/* color_return */
);

    /*
     * Color Space -- per Color Space related data (Device-Independent
     *    or Device-Dependent)
     */
typedef struct _XcmsColorSpace {
    const char *prefix;		/* Prefix of string format.		*/
    XcmsColorFormat id;		/* Format ID number.			*/
    XcmsParseStringProc parseString;
				/* String format parsing function	*/
    XcmsFuncListPtr to_CIEXYZ;	/* Pointer to an array of function 	*/
				/*   pointers such that when the	*/
				/*   functions are executed in sequence	*/
				/*   will convert a XcmsColor structure	*/
				/*   from this color space to CIEXYZ	*/
				/*   space.				*/
    XcmsFuncListPtr from_CIEXYZ;/* Pointer to an array of function 	*/
				/*   pointers such that when the	*/
				/*   functions are executed in sequence	*/
				/*   will convert a XcmsColor structure	*/
				/*   from CIEXYZ space to this color	*/
				/*   space.				*/
    int inverse_flag;		/* If 1, indicates that for 0 <= i < n	*/
				/*   where n is the number of function	*/
				/*   pointers in the lists to_CIEXYZ	*/
				/*   and from_CIEXYZ; for each function */
				/*   to_CIEXYZ[i] its inverse function	*/
				/*   is from_CIEXYZ[n - i].		*/

} XcmsColorSpace;

    /*
     * Screen Color Characterization Function Set -- per device class
     *    color space conversion functions.
     */
typedef struct _XcmsFunctionSet {
    XcmsColorSpace **DDColorSpaces;
				/* Pointer to an array of pointers to	*/
				/*   Device-DEPENDENT color spaces	*/
				/*   understood by this SCCFuncSet.	*/
    XcmsScreenInitProc screenInitProc;
				/* Screen initialization function that	*/
				/*   reads Screen Color Characterization*/
				/*   Data off properties on the screen's*/
				/*   root window.			*/
    XcmsScreenFreeProc screenFreeProc;
				/* Function that frees the SCCData	*/
				/*   structures.			*/
} XcmsFunctionSet;

_XFUNCPROTOBEGIN

extern Status XcmsAddColorSpace (
    XcmsColorSpace*	/* pColorSpace */
);

extern Status XcmsAddFunctionSet (
    XcmsFunctionSet*	/* functionSet */
);

extern Status XcmsAllocColor (
    Display*		/* dpy */,
    Colormap		/* colormap */,
    XcmsColor*		/* color_in_out */,
    XcmsColorFormat		/* result_format */
);

extern Status XcmsAllocNamedColor (
    Display*		/* dpy */,
    Colormap		/* colormap */,
    _Xconst char*	/* color_string */,
    XcmsColor*		/* color_scrn_return */,
    XcmsColor*		/* color_exact_return */,
    XcmsColorFormat		/* result_format */
);

extern XcmsCCC XcmsCCCOfColormap (
    Display*		/* dpy */,
    Colormap		/* colormap */
);

extern Status XcmsCIELabClipab(
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    unsigned int	/* index */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsCIELabClipL(
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    unsigned int	/* index */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsCIELabClipLab(
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    unsigned int	/* index */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsCIELabQueryMaxC (
    XcmsCCC		/* ccc */,
    XcmsFloat		/* hue_angle */,
    XcmsFloat		/* L_star */,
    XcmsColor*		/* color_return */
);

extern Status XcmsCIELabQueryMaxL (
    XcmsCCC		/* ccc */,
    XcmsFloat		/* hue_angle */,
    XcmsFloat		/* chroma */,
    XcmsColor*		/* color_return */
);

extern Status XcmsCIELabQueryMaxLC (
    XcmsCCC		/* ccc */,
    XcmsFloat		/* hue_angle */,
    XcmsColor*		/* color_return */
);

extern Status XcmsCIELabQueryMinL (
    XcmsCCC		/* ccc */,
    XcmsFloat		/* hue_angle */,
    XcmsFloat		/* chroma */,
    XcmsColor*		/* color_return */
);

extern Status XcmsCIELabToCIEXYZ (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* white_point */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */
);

extern Status XcmsCIELabWhiteShiftColors(
    XcmsCCC		/* ccc */,
    XcmsColor*		/* initial_white_point*/,
    XcmsColor*		/* target_white_point*/,
    XcmsColorFormat	/* target_format */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsCIELuvClipL(
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    unsigned int	/* index */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsCIELuvClipLuv(
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    unsigned int	/* index */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsCIELuvClipuv(
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    unsigned int	/* index */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsCIELuvQueryMaxC (
    XcmsCCC		/* ccc */,
    XcmsFloat		/* hue_angle */,
    XcmsFloat		/* L_star */,
    XcmsColor*		/* color_return */
);

extern Status XcmsCIELuvQueryMaxL (
    XcmsCCC		/* ccc */,
    XcmsFloat		/* hue_angle */,
    XcmsFloat		/* chroma */,
    XcmsColor*		/* color_return */
);

extern Status XcmsCIELuvQueryMaxLC (
    XcmsCCC		/* ccc */,
    XcmsFloat		/* hue_angle */,
    XcmsColor*		/* color_return */
);

extern Status XcmsCIELuvQueryMinL (
    XcmsCCC		/* ccc */,
    XcmsFloat		/* hue_angle */,
    XcmsFloat		/* chroma */,
    XcmsColor*		/* color_return */
);

extern Status XcmsCIELuvToCIEuvY (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* white_point */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */
);

extern Status XcmsCIELuvWhiteShiftColors(
    XcmsCCC		/* ccc */,
    XcmsColor*		/* initial_white_point*/,
    XcmsColor*		/* target_white_point*/,
    XcmsColorFormat	/* target_format */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsCIEXYZToCIELab (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* white_point */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */
);

extern Status XcmsCIEXYZToCIEuvY (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* white_point */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */
);

extern Status XcmsCIEXYZToCIExyY (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* white_point */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */
);

extern Status XcmsCIEXYZToRGBi (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsCIEuvYToCIELuv (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* white_point */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */
);

extern Status XcmsCIEuvYToCIEXYZ (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* white_point */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */
);

extern Status XcmsCIEuvYToTekHVC (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* white_point */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */
);

extern Status XcmsCIExyYToCIEXYZ (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* white_point */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */
);

extern XcmsColor *XcmsClientWhitePointOfCCC (
    XcmsCCC		/* ccc */
);

extern Status XcmsConvertColors (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colorArry_in_out */,
    unsigned int	/* nColors */,
    XcmsColorFormat		/* targetFormat */,
    Bool*		/* compArry_return */
);

extern XcmsCCC XcmsCreateCCC (
    Display*		/* dpy */,
    int			/* screenNumber */,
    Visual*		/* visual */,
    XcmsColor*		/* clientWhitePt */,
    XcmsCompressionProc /* gamutCompProc */,
    XPointer		/* gamutCompClientData */,
    XcmsWhiteAdjustProc	/* whitePtAdjProc */,
    XPointer		/* whitePtAdjClientData */
);

extern XcmsCCC XcmsDefaultCCC (
    Display*		/* dpy */,
    int			/* screenNumber */
);

extern Display *XcmsDisplayOfCCC (
    XcmsCCC		/* ccc */
);

extern XcmsColorFormat XcmsFormatOfPrefix (
    char*		/* prefix */
);

extern void XcmsFreeCCC (
    XcmsCCC		/* ccc */
);

extern Status XcmsLookupColor (
    Display*		/* dpy */,
    Colormap		/* colormap */,
    _Xconst char*	/* color_string */,
    XcmsColor*		/* pColor_exact_in_out */,
    XcmsColor*		/* pColor_scrn_in_out */,
    XcmsColorFormat		/* result_format */
);

extern char *XcmsPrefixOfFormat (
    XcmsColorFormat		/* id */
);

extern Status XcmsQueryBlack (
    XcmsCCC		/* ccc */,
    XcmsColorFormat	/* target_format */,
    XcmsColor*		/* color_return */
);

extern Status XcmsQueryBlue (
    XcmsCCC		/* ccc */,
    XcmsColorFormat	/* target_format */,
    XcmsColor*		/* color_return */
);

extern Status XcmsQueryColor (
    Display*		/* dpy */,
    Colormap		/* colormap */,
    XcmsColor*		/* pColor_in_out */,
    XcmsColorFormat		/* result_format */
);

extern Status XcmsQueryColors (
    Display*		/* dpy */,
    Colormap		/* colormap */,
    XcmsColor*		/* colorArry_in_out */,
    unsigned int	/* nColors */,
    XcmsColorFormat	/* result_format */
);

extern Status XcmsQueryGreen (
    XcmsCCC		/* ccc */,
    XcmsColorFormat	/* target_format */,
    XcmsColor*		/* color_return */
);

extern Status XcmsQueryRed (
    XcmsCCC		/* ccc */,
    XcmsColorFormat	/* target_format */,
    XcmsColor*		/* color_return */
);

extern Status XcmsQueryWhite (
    XcmsCCC		/* ccc */,
    XcmsColorFormat	/* target_format */,
    XcmsColor*		/* color_return */
);

extern Status XcmsRGBiToCIEXYZ (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsRGBiToRGB (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsRGBToRGBi (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */,
    Bool*		/* compression_flags_return */
);

extern int XcmsScreenNumberOfCCC (
    XcmsCCC		/* ccc */
);

extern XcmsColor *XcmsScreenWhitePointOfCCC (
    XcmsCCC		/* ccc */
);

extern XcmsCCC XcmsSetCCCOfColormap(
    Display*		/* dpy */,
    Colormap		/* colormap */,
    XcmsCCC		/* ccc */
);

extern XcmsCompressionProc XcmsSetCompressionProc (
    XcmsCCC		/* ccc */,
    XcmsCompressionProc	/* compression_proc */,
    XPointer		/* client_data */
);

extern XcmsWhiteAdjustProc XcmsSetWhiteAdjustProc (
    XcmsCCC		/* ccc */,
    XcmsWhiteAdjustProc	/* white_adjust_proc */,
    XPointer		/* client_data */
);

extern Status XcmsSetWhitePoint (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* color */
);

extern Status XcmsStoreColor (
    Display*		/* dpy */,
    Colormap		/* colormap */,
    XcmsColor*		/* pColor_in */
);

extern Status XcmsStoreColors (
    Display*		/* dpy */,
    Colormap		/* colormap */,
    XcmsColor*		/* colorArry_in */,
    unsigned int	/* nColors */,
    Bool*		/* compArry_return */
);

extern Status XcmsTekHVCClipC(
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    unsigned int	/* index */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsTekHVCClipV(
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    unsigned int	/* index */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsTekHVCClipVC(
    XcmsCCC		/* ccc */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    unsigned int	/* index */,
    Bool*		/* compression_flags_return */
);

extern Status XcmsTekHVCQueryMaxC (
    XcmsCCC		/* ccc */,
    XcmsFloat		/* hue */,
    XcmsFloat		/* value */,
    XcmsColor*		/* color_return */
);

extern Status XcmsTekHVCQueryMaxV (
    XcmsCCC		/* ccc */,
    XcmsFloat		/* hue */,
    XcmsFloat		/* chroma */,
    XcmsColor*		/* color_return */
);

extern Status XcmsTekHVCQueryMaxVC (
    XcmsCCC		/* ccc */,
    XcmsFloat		/* hue */,
    XcmsColor*		/* color_return */
);

extern Status XcmsTekHVCQueryMaxVSamples (
    XcmsCCC		/* ccc */,
    XcmsFloat		/* hue */,
    XcmsColor*		/* colors_return */,
    unsigned int	/* nsamples */
);

extern Status XcmsTekHVCQueryMinV (
    XcmsCCC		/* ccc */,
    XcmsFloat		/* hue */,
    XcmsFloat		/* chroma */,
    XcmsColor*		/* color_return */
);

extern Status XcmsTekHVCToCIEuvY (
    XcmsCCC		/* ccc */,
    XcmsColor*		/* white_point */,
    XcmsColor*		/* colors */,
    unsigned int	/* ncolors */
);

extern Status XcmsTekHVCWhiteShiftColors(
    XcmsCCC		/* ccc */,
    XcmsColor*		/* initial_white_point*/,
    XcmsColor*		/* target_white_point*/,
    XcmsColorFormat	/* target_format */,
    XcmsColor*		/* colors_in_out */,
    unsigned int	/* ncolors */,
    Bool*		/* compression_flags_return */
);

extern Visual *XcmsVisualOfCCC (
    XcmsCCC		/* ccc */
);

#ifdef __clang__
#pragma clang diagnostic pop
#endif

_XFUNCPROTOEND

#endif /* _X11_XCMS_H_ */
