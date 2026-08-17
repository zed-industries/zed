/**************************************************************************

Copyright 1998-1999 Precision Insight, Inc., Cedar Park, Texas.
Copyright 2000 VA Linux Systems, Inc.
All Rights Reserved.

Permission is hereby granted, free of charge, to any person obtaining a
copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sub license, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice (including the
next paragraph) shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
IN NO EVENT SHALL PRECISION INSIGHT AND/OR ITS SUPPLIERS BE LIABLE FOR
ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

**************************************************************************/

/*
 * Authors:
 *   Kevin E. Martin <martin@valinux.com>
 *   Jens Owen <jens@tungstengraphics.com>
 *   Rickard E. (Rik) Faith <faith@valinux.com>
 *
 */

#ifndef _XF86DRISTR_H_
#define _XF86DRISTR_H_

#include "xf86dri.h"

#define XF86DRINAME "XFree86-DRI"

/* The DRI version number.  This was originally set to be the same of the
 * XFree86 version number.  However, this version is really independent of
 * the XFree86 version.
 *
 * Version History:
 *    4.0.0: Original
 *    4.0.1: Patch to bump clipstamp when windows are destroyed, 28 May 02
 *    4.1.0: Add transition from single to multi in DRMInfo rec, 24 Jun 02
 */
#define XF86DRI_MAJOR_VERSION	4
#define XF86DRI_MINOR_VERSION	1
#define XF86DRI_PATCH_VERSION	0

typedef struct _XF86DRIQueryVersion {
    CARD8	reqType;		/* always DRIReqCode */
    CARD8	driReqType;		/* always X_DRIQueryVersion */
    CARD16	length;
} xXF86DRIQueryVersionReq;
#define sz_xXF86DRIQueryVersionReq	4

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	majorVersion;		/* major version of DRI protocol */
    CARD16	minorVersion;		/* minor version of DRI protocol */
    CARD32	patchVersion;		/* patch version of DRI protocol */
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xXF86DRIQueryVersionReply;
#define sz_xXF86DRIQueryVersionReply	32

typedef struct _XF86DRIQueryDirectRenderingCapable {
    CARD8	reqType;		/* always DRIReqCode */
    CARD8	driReqType;		/* X_DRIQueryDirectRenderingCapable */
    CARD16	length;
    CARD32	screen;
} xXF86DRIQueryDirectRenderingCapableReq;
#define sz_xXF86DRIQueryDirectRenderingCapableReq	8

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    BOOL	isCapable;
    BOOL	pad2;
    BOOL	pad3;
    BOOL	pad4;
    CARD32	pad5;
    CARD32	pad6;
    CARD32	pad7;
    CARD32	pad8;
    CARD32	pad9;
} xXF86DRIQueryDirectRenderingCapableReply;
#define sz_xXF86DRIQueryDirectRenderingCapableReply	32

typedef struct _XF86DRIOpenConnection {
    CARD8	reqType;		/* always DRIReqCode */
    CARD8	driReqType;		/* always X_DRIOpenConnection */
    CARD16	length;
    CARD32	screen;
} xXF86DRIOpenConnectionReq;
#define sz_xXF86DRIOpenConnectionReq	8

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	hSAREALow;
    CARD32	hSAREAHigh;
    CARD32	busIdStringLength;
    CARD32	pad6;
    CARD32	pad7;
    CARD32	pad8;
} xXF86DRIOpenConnectionReply;
#define sz_xXF86DRIOpenConnectionReply	32

typedef struct _XF86DRIAuthConnection {
    CARD8	reqType;		/* always DRIReqCode */
    CARD8	driReqType;		/* always X_DRICloseConnection */
    CARD16	length;
    CARD32	screen;
    CARD32	magic;
} xXF86DRIAuthConnectionReq;
#define sz_xXF86DRIAuthConnectionReq	12

typedef struct {
    BYTE        type;
    BOOL        pad1;
    CARD16      sequenceNumber;
    CARD32      length;
    CARD32      authenticated;
    CARD32      pad2;
    CARD32      pad3;
    CARD32      pad4;
    CARD32      pad5;
    CARD32      pad6;
} xXF86DRIAuthConnectionReply;
#define zx_xXF86DRIAuthConnectionReply  32

typedef struct _XF86DRICloseConnection {
    CARD8	reqType;		/* always DRIReqCode */
    CARD8	driReqType;		/* always X_DRICloseConnection */
    CARD16	length;
    CARD32	screen;
} xXF86DRICloseConnectionReq;
#define sz_xXF86DRICloseConnectionReq	8

typedef struct _XF86DRIGetClientDriverName {
    CARD8	reqType;		/* always DRIReqCode */
    CARD8	driReqType;		/* always X_DRIGetClientDriverName */
    CARD16	length;
    CARD32	screen;
} xXF86DRIGetClientDriverNameReq;
#define sz_xXF86DRIGetClientDriverNameReq	8

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	ddxDriverMajorVersion;
    CARD32	ddxDriverMinorVersion;
    CARD32	ddxDriverPatchVersion;
    CARD32	clientDriverNameLength;
    CARD32	pad5;
    CARD32	pad6;
} xXF86DRIGetClientDriverNameReply;
#define sz_xXF86DRIGetClientDriverNameReply	32

typedef struct _XF86DRICreateContext {
    CARD8	reqType;		/* always DRIReqCode */
    CARD8	driReqType;		/* always X_DRICreateContext */
    CARD16	length;
    CARD32	screen;
    CARD32	visual;
    CARD32	context;
} xXF86DRICreateContextReq;
#define sz_xXF86DRICreateContextReq	16

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	hHWContext;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xXF86DRICreateContextReply;
#define sz_xXF86DRICreateContextReply	32

typedef struct _XF86DRIDestroyContext {
    CARD8	reqType;		/* always DRIReqCode */
    CARD8	driReqType;		/* always X_DRIDestroyContext */
    CARD16	length;
    CARD32	screen;
    CARD32	context;
} xXF86DRIDestroyContextReq;
#define sz_xXF86DRIDestroyContextReq	12

typedef struct _XF86DRICreateDrawable {
    CARD8	reqType;		/* always DRIReqCode */
    CARD8	driReqType;		/* always X_DRICreateDrawable */
    CARD16	length;
    CARD32	screen;
    CARD32	drawable;
} xXF86DRICreateDrawableReq;
#define sz_xXF86DRICreateDrawableReq	12

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	hHWDrawable;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xXF86DRICreateDrawableReply;
#define sz_xXF86DRICreateDrawableReply	32

typedef struct _XF86DRIDestroyDrawable {
    CARD8	reqType;		/* always DRIReqCode */
    CARD8	driReqType;		/* always X_DRIDestroyDrawable */
    CARD16	length;
    CARD32	screen;
    CARD32	drawable;
} xXF86DRIDestroyDrawableReq;
#define sz_xXF86DRIDestroyDrawableReq	12

typedef struct _XF86DRIGetDrawableInfo {
    CARD8	reqType;		/* always DRIReqCode */
    CARD8	driReqType;		/* always X_DRIGetDrawableInfo */
    CARD16	length;
    CARD32	screen;
    CARD32	drawable;
} xXF86DRIGetDrawableInfoReq;
#define sz_xXF86DRIGetDrawableInfoReq	12

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	drawableTableIndex;
    CARD32	drawableTableStamp;
    INT16	drawableX;
    INT16	drawableY;
    INT16	drawableWidth;
    INT16	drawableHeight;
    CARD32	numClipRects;
    INT16	backX;
    INT16	backY;
    CARD32	numBackClipRects;
} xXF86DRIGetDrawableInfoReply;

#define sz_xXF86DRIGetDrawableInfoReply	36


typedef struct _XF86DRIGetDeviceInfo {
    CARD8	reqType;		/* always DRIReqCode */
    CARD8	driReqType;		/* always X_DRIGetDeviceInfo */
    CARD16	length;
    CARD32	screen;
} xXF86DRIGetDeviceInfoReq;
#define sz_xXF86DRIGetDeviceInfoReq	8

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	hFrameBufferLow;
    CARD32	hFrameBufferHigh;
    CARD32	framebufferOrigin;
    CARD32	framebufferSize;
    CARD32	framebufferStride;
    CARD32	devPrivateSize;
} xXF86DRIGetDeviceInfoReply;
#define sz_xXF86DRIGetDeviceInfoReply	32

typedef struct _XF86DRIOpenFullScreen {
    CARD8       reqType;	/* always DRIReqCode */
    CARD8       driReqType;	/* always X_DRIOpenFullScreen */
    CARD16      length;
    CARD32      screen;
    CARD32      drawable;
} xXF86DRIOpenFullScreenReq;
#define sz_xXF86DRIOpenFullScreenReq    12

typedef struct {
    BYTE        type;
    BOOL        pad1;
    CARD16      sequenceNumber;
    CARD32      length;
    CARD32      isFullScreen;
    CARD32      pad2;
    CARD32      pad3;
    CARD32      pad4;
    CARD32      pad5;
    CARD32      pad6;
} xXF86DRIOpenFullScreenReply;
#define sz_xXF86DRIOpenFullScreenReply  32

typedef struct _XF86DRICloseFullScreen {
    CARD8       reqType;	/* always DRIReqCode */
    CARD8       driReqType;	/* always X_DRICloseFullScreen */
    CARD16      length;
    CARD32      screen;
    CARD32      drawable;
} xXF86DRICloseFullScreenReq;
#define sz_xXF86DRICloseFullScreenReq   12

typedef struct {
    BYTE        type;
    BOOL        pad1;
    CARD16      sequenceNumber;
    CARD32      length;
    CARD32      pad2;
    CARD32      pad3;
    CARD32      pad4;
    CARD32      pad5;
    CARD32      pad6;
    CARD32      pad7;
} xXF86DRICloseFullScreenReply;
#define sz_xXF86DRICloseFullScreenReply  32


#endif /* _XF86DRISTR_H_ */
