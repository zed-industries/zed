/*
 * Raster file definitions for CUPS.
 *
 * Copyright © 2007-2018 by Apple Inc.
 * Copyright © 1997-2006 by Easy Software Products.
 *
 * This file is part of the CUPS Imaging library.
 *
 * Licensed under Apache License v2.0.  See the file "LICENSE" for more
 * information.
 */

#ifndef _CUPS_RASTER_H_
#  define _CUPS_RASTER_H_

/*
 * Include necessary headers...
 */

#  include "cups.h"


#  ifdef __cplusplus
extern "C" {
#  endif /* __cplusplus */

/*
 * Every non-PostScript printer driver that supports raster images
 * should use the application/vnd.cups-raster image file format.
 * Since both the PostScript RIP (pstoraster, based on GNU/GPL
 * Ghostscript) and Image RIP (imagetoraster, located in the filter
 * directory) use it, using this format saves you a lot of work.
 * Also, the PostScript RIP passes any printer options that are in
 * a PS file to your driver this way as well...
 */

/*
 * Constants...
 */

#  define CUPS_RASTER_SYNC	0x52615333	/* RaS3 */
#  define CUPS_RASTER_REVSYNC	0x33536152	/* 3SaR */

#  define CUPS_RASTER_SYNCv1	0x52615374	/* RaSt */
#  define CUPS_RASTER_REVSYNCv1	0x74536152	/* tSaR */

#  define CUPS_RASTER_SYNCv2	0x52615332	/* RaS2 */
#  define CUPS_RASTER_REVSYNCv2	0x32536152	/* 2SaR */

#  define CUPS_RASTER_SYNCapple	0x554E4952	/* UNIR */
#  define CUPS_RASTER_REVSYNCapple 0x52494E55	/* RINU */

#  define CUPS_RASTER_SYNC_PWG	CUPS_RASTER_SYNCv2

/*
 * The following definition can be used to determine if the
 * colorimetric colorspaces (CIEXYZ, CIELAB, and ICCn) are
 * defined...
 */

#  define CUPS_RASTER_HAVE_COLORIMETRIC 1

/*
 * The following definition can be used to determine if the
 * device colorspaces (DEVICEn) are defined...
 */

#  define CUPS_RASTER_HAVE_DEVICE 1

/*
 * The following definition can be used to determine if PWG Raster is supported.
 */

#  define CUPS_RASTER_HAVE_PWGRASTER 1

/*
 * The following definition can be used to determine if Apple Raster is
 * supported (beta).
 */

#  define CUPS_RASTER_HAVE_APPLERASTER 1

/*
 * The following PWG 5102.4 definitions specify indices into the
 * cupsInteger[] array in the raster header.
 */

#  define CUPS_RASTER_PWG_TotalPageCount	0
#  define CUPS_RASTER_PWG_CrossFeedTransform	1
#  define CUPS_RASTER_PWG_FeedTransform		2
#  define CUPS_RASTER_PWG_ImageBoxLeft		3
#  define CUPS_RASTER_PWG_ImageBoxTop		4
#  define CUPS_RASTER_PWG_ImageBoxRight		5
#  define CUPS_RASTER_PWG_ImageBoxBottom	6
#  define CUPS_RASTER_PWG_AlternatePrimary	7
#  define CUPS_RASTER_PWG_PrintQuality		8
#  define CUPS_RASTER_PWG_VendorIdentifier	14
#  define CUPS_RASTER_PWG_VendorLength		15




/*
 * Types...
 */

typedef enum cups_adv_e			/**** AdvanceMedia attribute values ****/
{
  CUPS_ADVANCE_NONE = 0,		/* Never advance the roll */
  CUPS_ADVANCE_FILE = 1,		/* Advance the roll after this file */
  CUPS_ADVANCE_JOB = 2,			/* Advance the roll after this job */
  CUPS_ADVANCE_SET = 3,			/* Advance the roll after this set */
  CUPS_ADVANCE_PAGE = 4			/* Advance the roll after this page */
} cups_adv_t;

typedef enum cups_bool_e		/**** Boolean type ****/
{
  CUPS_FALSE = 0,			/* Logical false */
  CUPS_TRUE = 1				/* Logical true */
} cups_bool_t;

typedef enum cups_cspace_e		/**** cupsColorSpace attribute values ****/
{
  CUPS_CSPACE_W = 0,			/* Luminance (DeviceGray, gamma 2.2 by default) */
  CUPS_CSPACE_RGB = 1,			/* Red, green, blue (DeviceRGB, sRGB by default) */
  CUPS_CSPACE_RGBA = 2,			/* Red, green, blue, alpha (DeviceRGB, sRGB by default) */
  CUPS_CSPACE_K = 3,			/* Black (DeviceK) */
  CUPS_CSPACE_CMY = 4,			/* Cyan, magenta, yellow (DeviceCMY) */
  CUPS_CSPACE_YMC = 5,			/* Yellow, magenta, cyan @deprecated@ */
  CUPS_CSPACE_CMYK = 6,			/* Cyan, magenta, yellow, black (DeviceCMYK) */
  CUPS_CSPACE_YMCK = 7,			/* Yellow, magenta, cyan, black @deprecated@ */
  CUPS_CSPACE_KCMY = 8,			/* Black, cyan, magenta, yellow @deprecated@ */
  CUPS_CSPACE_KCMYcm = 9,		/* Black, cyan, magenta, yellow, light-cyan, light-magenta @deprecated@ */
  CUPS_CSPACE_GMCK = 10,		/* Gold, magenta, yellow, black @deprecated@ */
  CUPS_CSPACE_GMCS = 11,		/* Gold, magenta, yellow, silver @deprecated@ */
  CUPS_CSPACE_WHITE = 12,		/* White ink (as black) @deprecated@ */
  CUPS_CSPACE_GOLD = 13,		/* Gold foil @deprecated@ */
  CUPS_CSPACE_SILVER = 14,		/* Silver foil @deprecated@ */

  CUPS_CSPACE_CIEXYZ = 15,		/* CIE XYZ @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_CIELab = 16,		/* CIE Lab @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_RGBW = 17,		/* Red, green, blue, white (DeviceRGB, sRGB by default) @since CUPS 1.2/macOS 10.5@ */
  CUPS_CSPACE_SW = 18,			/* Luminance (gamma 2.2) @since CUPS 1.4.5@ */
  CUPS_CSPACE_SRGB = 19,		/* Red, green, blue (sRGB) @since CUPS 1.4.5@ */
  CUPS_CSPACE_ADOBERGB = 20,		/* Red, green, blue (Adobe RGB) @since CUPS 1.4.5@ */

  CUPS_CSPACE_ICC1 = 32,		/* ICC-based, 1 color @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICC2 = 33,		/* ICC-based, 2 colors @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICC3 = 34,		/* ICC-based, 3 colors @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICC4 = 35,		/* ICC-based, 4 colors @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICC5 = 36,		/* ICC-based, 5 colors @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICC6 = 37,		/* ICC-based, 6 colors @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICC7 = 38,		/* ICC-based, 7 colors @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICC8 = 39,		/* ICC-based, 8 colors @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICC9 = 40,		/* ICC-based, 9 colors @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICCA = 41,		/* ICC-based, 10 colors @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICCB = 42,		/* ICC-based, 11 colors @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICCC = 43,		/* ICC-based, 12 colors @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICCD = 44,		/* ICC-based, 13 colors @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICCE = 45,		/* ICC-based, 14 colors @since CUPS 1.1.19/macOS 10.3@ */
  CUPS_CSPACE_ICCF = 46,		/* ICC-based, 15 colors @since CUPS 1.1.19/macOS 10.3@ */

  CUPS_CSPACE_DEVICE1 = 48,		/* DeviceN, 1 color @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICE2 = 49,		/* DeviceN, 2 colors @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICE3 = 50,		/* DeviceN, 3 colors @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICE4 = 51,		/* DeviceN, 4 colors @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICE5 = 52,		/* DeviceN, 5 colors @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICE6 = 53,		/* DeviceN, 6 colors @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICE7 = 54,		/* DeviceN, 7 colors @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICE8 = 55,		/* DeviceN, 8 colors @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICE9 = 56,		/* DeviceN, 9 colors @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICEA = 57,		/* DeviceN, 10 colors @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICEB = 58,		/* DeviceN, 11 colors @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICEC = 59,		/* DeviceN, 12 colors @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICED = 60,		/* DeviceN, 13 colors @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICEE = 61,		/* DeviceN, 14 colors @since CUPS 1.4.5@ */
  CUPS_CSPACE_DEVICEF = 62		/* DeviceN, 15 colors @since CUPS 1.4.5@ */
} cups_cspace_t;

typedef enum cups_cut_e			/**** CutMedia attribute values ****/
{
  CUPS_CUT_NONE = 0,			/* Never cut the roll */
  CUPS_CUT_FILE = 1,			/* Cut the roll after this file */
  CUPS_CUT_JOB = 2,			/* Cut the roll after this job */
  CUPS_CUT_SET = 3,			/* Cut the roll after this set */
  CUPS_CUT_PAGE = 4			/* Cut the roll after this page */
} cups_cut_t;

typedef enum cups_edge_e		/**** LeadingEdge attribute values ****/
{
  CUPS_EDGE_TOP = 0,			/* Leading edge is the top of the page */
  CUPS_EDGE_RIGHT = 1,			/* Leading edge is the right of the page */
  CUPS_EDGE_BOTTOM = 2,			/* Leading edge is the bottom of the page */
  CUPS_EDGE_LEFT = 3			/* Leading edge is the left of the page */
} cups_edge_t;

typedef enum cups_jog_e			/**** Jog attribute values ****/
{
  CUPS_JOG_NONE = 0,			/* Never move pages */
  CUPS_JOG_FILE = 1,			/* Move pages after this file */
  CUPS_JOG_JOB = 2,			/* Move pages after this job */
  CUPS_JOG_SET = 3			/* Move pages after this set */
} cups_jog_t;

enum cups_mode_e			/**** cupsRasterOpen modes ****/
{
  CUPS_RASTER_READ = 0,			/* Open stream for reading */
  CUPS_RASTER_WRITE = 1,		/* Open stream for writing */
  CUPS_RASTER_WRITE_COMPRESSED = 2,	/* Open stream for compressed writing @since CUPS 1.3/macOS 10.5@ */
  CUPS_RASTER_WRITE_PWG = 3,		/* Open stream for compressed writing in PWG Raster mode @since CUPS 1.5/macOS 10.7@ */
  CUPS_RASTER_WRITE_APPLE = 4		/* Open stream for compressed writing in AppleRaster mode (beta) @private@ */
};

typedef enum cups_mode_e cups_mode_t;	/**** cupsRasterOpen modes ****/

typedef enum cups_order_e		/**** cupsColorOrder attribute values ****/
{
  CUPS_ORDER_CHUNKED = 0,		/* CMYK CMYK CMYK ... */
  CUPS_ORDER_BANDED = 1,		/* CCC MMM YYY KKK ... */
  CUPS_ORDER_PLANAR = 2			/* CCC ... MMM ... YYY ... KKK ... */
} cups_order_t;

typedef enum cups_orient_e		/**** Orientation attribute values ****/
{
  CUPS_ORIENT_0 = 0,			/* Don't rotate the page */
  CUPS_ORIENT_90 = 1,			/* Rotate the page counter-clockwise */
  CUPS_ORIENT_180 = 2,			/* Turn the page upside down */
  CUPS_ORIENT_270 = 3			/* Rotate the page clockwise */
} cups_orient_t;


/*
 * The page header structure contains the standard PostScript page device
 * dictionary, along with some CUPS-specific parameters that are provided
 * by the RIPs...
 *
 * The API supports a "version 1" (from CUPS 1.0 and 1.1) and a "version 2"
 * (from CUPS 1.2 and higher) page header, for binary compatibility.
 */

typedef struct cups_page_header_s	/**** Version 1 page header @deprecated@ ****/
{
  /**** Standard Page Device Dictionary String Values ****/
  char		MediaClass[64];		/* MediaClass string */
  char		MediaColor[64];		/* MediaColor string */
  char		MediaType[64];		/* MediaType string */
  char		OutputType[64];		/* OutputType string */

  /**** Standard Page Device Dictionary Integer Values ****/
  unsigned	AdvanceDistance;	/* AdvanceDistance value in points */
  cups_adv_t	AdvanceMedia;		/* AdvanceMedia value (@link cups_adv_t@) */
  cups_bool_t	Collate;		/* Collated copies value */
  cups_cut_t	CutMedia;		/* CutMedia value (@link cups_cut_t@) */
  cups_bool_t	Duplex;			/* Duplexed (double-sided) value */
  unsigned	HWResolution[2];	/* Resolution in dots-per-inch */
  unsigned	ImagingBoundingBox[4];	/* Pixel region that is painted (points, left, bottom, right, top) */
  cups_bool_t	InsertSheet;		/* InsertSheet value */
  cups_jog_t	Jog;			/* Jog value (@link cups_jog_t@) */
  cups_edge_t	LeadingEdge;		/* LeadingEdge value (@link cups_edge_t@) */
  unsigned	Margins[2];		/* Lower-lefthand margins in points */
  cups_bool_t	ManualFeed;		/* ManualFeed value */
  unsigned	MediaPosition;		/* MediaPosition value */
  unsigned	MediaWeight;		/* MediaWeight value in grams/m^2 */
  cups_bool_t	MirrorPrint;		/* MirrorPrint value */
  cups_bool_t	NegativePrint;		/* NegativePrint value */
  unsigned	NumCopies;		/* Number of copies to produce */
  cups_orient_t	Orientation;		/* Orientation value (@link cups_orient_t@) */
  cups_bool_t	OutputFaceUp;		/* OutputFaceUp value */
  unsigned	PageSize[2];		/* Width and length of page in points */
  cups_bool_t	Separations;		/* Separations value */
  cups_bool_t	TraySwitch;		/* TraySwitch value */
  cups_bool_t	Tumble;			/* Tumble value */

  /**** CUPS Page Device Dictionary Values ****/
  unsigned	cupsWidth;		/* Width of page image in pixels */
  unsigned	cupsHeight;		/* Height of page image in pixels */
  unsigned	cupsMediaType;		/* Media type code */
  unsigned	cupsBitsPerColor;	/* Number of bits for each color */
  unsigned	cupsBitsPerPixel;	/* Number of bits for each pixel */
  unsigned	cupsBytesPerLine;	/* Number of bytes per line */
  cups_order_t	cupsColorOrder;		/* Order of colors */
  cups_cspace_t	cupsColorSpace;		/* True colorspace */
  unsigned	cupsCompression;	/* Device compression to use */
  unsigned	cupsRowCount;		/* Rows per band */
  unsigned	cupsRowFeed;		/* Feed between bands */
  unsigned	cupsRowStep;		/* Spacing between lines */
} cups_page_header_t;

/**** New in CUPS 1.2 ****/
typedef struct cups_page_header2_s	/**** Version 2 page header @since CUPS 1.2/macOS 10.5@ ****/
{
  /**** Standard Page Device Dictionary String Values ****/
  char		MediaClass[64];		/* MediaClass string */
  char		MediaColor[64];		/* MediaColor string */
  char		MediaType[64];		/* MediaType string */
  char		OutputType[64];		/* OutputType string */

  /**** Standard Page Device Dictionary Integer Values ****/
  unsigned	AdvanceDistance;	/* AdvanceDistance value in points */
  cups_adv_t	AdvanceMedia;		/* AdvanceMedia value (@link cups_adv_t@) */
  cups_bool_t	Collate;		/* Collated copies value */
  cups_cut_t	CutMedia;		/* CutMedia value (@link cups_cut_t@) */
  cups_bool_t	Duplex;			/* Duplexed (double-sided) value */
  unsigned	HWResolution[2];	/* Resolution in dots-per-inch */
  unsigned	ImagingBoundingBox[4];	/* Pixel region that is painted (points, left, bottom, right, top) */
  cups_bool_t	InsertSheet;		/* InsertSheet value */
  cups_jog_t	Jog;			/* Jog value (@link cups_jog_t@) */
  cups_edge_t	LeadingEdge;		/* LeadingEdge value (@link cups_edge_t@) */
  unsigned	Margins[2];		/* Lower-lefthand margins in points */
  cups_bool_t	ManualFeed;		/* ManualFeed value */
  unsigned	MediaPosition;		/* MediaPosition value */
  unsigned	MediaWeight;		/* MediaWeight value in grams/m^2 */
  cups_bool_t	MirrorPrint;		/* MirrorPrint value */
  cups_bool_t	NegativePrint;		/* NegativePrint value */
  unsigned	NumCopies;		/* Number of copies to produce */
  cups_orient_t	Orientation;		/* Orientation value (@link cups_orient_t@) */
  cups_bool_t	OutputFaceUp;		/* OutputFaceUp value */
  unsigned	PageSize[2];		/* Width and length of page in points */
  cups_bool_t	Separations;		/* Separations value */
  cups_bool_t	TraySwitch;		/* TraySwitch value */
  cups_bool_t	Tumble;			/* Tumble value */

  /**** CUPS Page Device Dictionary Values ****/
  unsigned	cupsWidth;		/* Width of page image in pixels */
  unsigned	cupsHeight;		/* Height of page image in pixels */
  unsigned	cupsMediaType;		/* Media type code */
  unsigned	cupsBitsPerColor;	/* Number of bits for each color */
  unsigned	cupsBitsPerPixel;	/* Number of bits for each pixel */
  unsigned	cupsBytesPerLine;	/* Number of bytes per line */
  cups_order_t	cupsColorOrder;		/* Order of colors */
  cups_cspace_t	cupsColorSpace;		/* True colorspace */
  unsigned	cupsCompression;	/* Device compression to use */
  unsigned	cupsRowCount;		/* Rows per band */
  unsigned	cupsRowFeed;		/* Feed between bands */
  unsigned	cupsRowStep;		/* Spacing between lines */

  /**** Version 2 Dictionary Values ****/
  unsigned	cupsNumColors;		/* Number of color compoents @since CUPS 1.2/macOS 10.5@ */
  float		cupsBorderlessScalingFactor;
					/* Scaling that was applied to page data @since CUPS 1.2/macOS 10.5@ */
  float		cupsPageSize[2];	/* Floating point PageSize (scaling *
  					 * factor not applied) @since CUPS 1.2/macOS 10.5@ */
  float		cupsImagingBBox[4];	/* Floating point ImagingBoundingBox
					 * (scaling factor not applied, left,
					 * bottom, right, top) @since CUPS 1.2/macOS 10.5@ */
  unsigned	cupsInteger[16];	/* User-defined integer values @since CUPS 1.2/macOS 10.5@ */
  float		cupsReal[16];		/* User-defined floating-point values @since CUPS 1.2/macOS 10.5@ */
  char		cupsString[16][64];	/* User-defined string values @since CUPS 1.2/macOS 10.5@ */
  char		cupsMarkerType[64];	/* Ink/toner type @since CUPS 1.2/macOS 10.5@ */
  char		cupsRenderingIntent[64];/* Color rendering intent @since CUPS 1.2/macOS 10.5@ */
  char		cupsPageSizeName[64];	/* PageSize name @since CUPS 1.2/macOS 10.5@ */
} cups_page_header2_t;

typedef struct _cups_raster_s cups_raster_t;
					/**** Raster stream data ****/

/**** New in CUPS 1.5 ****/
typedef ssize_t (*cups_raster_iocb_t)(void *ctx, unsigned char *buffer, size_t length);
					/**** cupsRasterOpenIO callback function
					 *
					 * This function is specified when
					 * creating a raster stream with
					 * @link cupsRasterOpenIO@ and handles
					 * generic reading and writing of raster
					 * data. It must return -1 on error or
					 * the number of bytes specified by
					 * "length" on success.
					 ****/


/*
 * Prototypes...
 */

extern void		cupsRasterClose(cups_raster_t *r) _CUPS_PUBLIC;
extern cups_raster_t	*cupsRasterOpen(int fd, cups_mode_t mode) _CUPS_PUBLIC;
extern unsigned		cupsRasterReadHeader(cups_raster_t *r, cups_page_header_t *h) _CUPS_DEPRECATED_MSG("Use cupsRasterReadHeader2 instead.") _CUPS_PUBLIC;
extern unsigned		cupsRasterReadPixels(cups_raster_t *r, unsigned char *p, unsigned len) _CUPS_PUBLIC;
extern unsigned		cupsRasterWriteHeader(cups_raster_t *r, cups_page_header_t *h) _CUPS_DEPRECATED_MSG("Use cupsRasterWriteHeader2 instead.") _CUPS_PUBLIC;
extern unsigned		cupsRasterWritePixels(cups_raster_t *r, unsigned char *p, unsigned len) _CUPS_PUBLIC;

/**** New in CUPS 1.2 ****/
extern unsigned		cupsRasterReadHeader2(cups_raster_t *r, cups_page_header2_t *h) _CUPS_API_1_2;
extern unsigned		cupsRasterWriteHeader2(cups_raster_t *r, cups_page_header2_t *h) _CUPS_API_1_2;

/**** New in CUPS 1.3 ****/
extern const char	*cupsRasterErrorString(void) _CUPS_API_1_3;

/**** New in CUPS 1.5 ****/
extern cups_raster_t	*cupsRasterOpenIO(cups_raster_iocb_t iocb, void *ctx, cups_mode_t mode) _CUPS_API_1_5;

/**** New in CUPS 2.2/macOS 10.12 ****/
extern int		cupsRasterInitPWGHeader(cups_page_header2_t *h, pwg_media_t *media, const char *type, int xdpi, int ydpi, const char *sides, const char *sheet_back) _CUPS_API_2_2;

#  ifdef __cplusplus
}
#  endif /* __cplusplus */

#endif /* !_CUPS_RASTER_H_ */
