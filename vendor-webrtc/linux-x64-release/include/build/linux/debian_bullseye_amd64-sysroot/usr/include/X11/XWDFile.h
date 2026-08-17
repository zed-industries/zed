/*

Copyright 1985, 1986, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall not be
used in advertising or otherwise to promote the sale, use or other dealings
in this Software without prior written authorization from The Open Group.

*/

/*
 * XWDFile.h	MIT Project Athena, X Window system window raster
 *		image dumper, dump file format header file.
 *
 *  Author:	Tony Della Fera, DEC
 *		27-Jun-85
 *
 * Modifier:    William F. Wyatt, SAO
 *              18-Nov-86  - version 6 for saving/restoring color maps
 */

#ifndef XWDFILE_H
#define XWDFILE_H

#include <X11/Xmd.h>

#define XWD_FILE_VERSION 7
#define sz_XWDheader 100
#define sz_XWDColor 12

typedef CARD32 xwdval;		/* for old broken programs */

/* Values in the file are most significant byte first. */

typedef struct _xwd_file_header {
	/* header_size = SIZEOF(XWDheader) + length of null-terminated
	 * window name. */
	CARD32 header_size;

	CARD32 file_version;		/* = XWD_FILE_VERSION above */
	CARD32 pixmap_format;		/* ZPixmap or XYPixmap */
	CARD32 pixmap_depth;		/* Pixmap depth */
	CARD32 pixmap_width;		/* Pixmap width */
	CARD32 pixmap_height;		/* Pixmap height */
	CARD32 xoffset;			/* Bitmap x offset, normally 0 */
	CARD32 byte_order;		/* of image data: MSBFirst, LSBFirst */

	/* bitmap_unit applies to bitmaps (depth 1 format XY) only.
	 * It is the number of bits that each scanline is padded to. */
	CARD32 bitmap_unit;

	CARD32 bitmap_bit_order;	/* bitmaps only: MSBFirst, LSBFirst */

	/* bitmap_pad applies to pixmaps (non-bitmaps) only.
	 * It is the number of bits that each scanline is padded to. */
	CARD32 bitmap_pad;

	CARD32 bits_per_pixel;		/* Bits per pixel */

	/* bytes_per_line is pixmap_width padded to bitmap_unit (bitmaps)
	 * or bitmap_pad (pixmaps).  It is the delta (in bytes) to get
	 * to the same x position on an adjacent row. */
	CARD32 bytes_per_line;
	CARD32 visual_class;		/* Class of colormap */
	CARD32 red_mask;		/* Z red mask */
	CARD32 green_mask;		/* Z green mask */
	CARD32 blue_mask;		/* Z blue mask */
	CARD32 bits_per_rgb;		/* Log2 of distinct color values */
	CARD32 colormap_entries;	/* Number of entries in colormap; not used? */
	CARD32 ncolors;			/* Number of XWDColor structures */
	CARD32 window_width;		/* Window width */
	CARD32 window_height;		/* Window height */
	CARD32 window_x;		/* Window upper left X coordinate */
	CARD32 window_y;		/* Window upper left Y coordinate */
	CARD32 window_bdrwidth;		/* Window border width */
} XWDFileHeader;

/* Null-terminated window name follows the above structure. */

/* Next comes XWDColor structures, at offset XWDFileHeader.header_size in
 * the file.  XWDFileHeader.ncolors tells how many XWDColor structures
 * there are.
 */

typedef struct {
        CARD32	pixel;
        CARD16	red;
        CARD16	green;
        CARD16	blue;
        CARD8	flags;
        CARD8	pad;
} XWDColor;

/* Last comes the image data in the format described by XWDFileHeader. */

#endif /* XWDFILE_H */

