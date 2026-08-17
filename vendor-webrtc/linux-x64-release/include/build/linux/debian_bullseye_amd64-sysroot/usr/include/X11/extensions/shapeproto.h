/************************************************************

Copyright 1989, 1998  The Open Group

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

********************************************************/

#ifndef _SHAPEPROTO_H_
#define _SHAPEPROTO_H_

#include <X11/extensions/shapeconst.h>

/*
 * Protocol requests constants and alignment values
 * These would really be in SHAPE's X.h and Xproto.h equivalents
 */

#define Window CARD32
#define Time CARD32

#define X_ShapeQueryVersion		0
#define X_ShapeRectangles		1
#define X_ShapeMask			2
#define X_ShapeCombine			3
#define X_ShapeOffset			4
#define X_ShapeQueryExtents		5
#define X_ShapeSelectInput		6
#define X_ShapeInputSelected		7
#define X_ShapeGetRectangles		8

typedef struct _ShapeQueryVersion {
	CARD8	reqType;		/* always ShapeReqCode */
	CARD8	shapeReqType;		/* always X_ShapeQueryVersion */
	CARD16	length;
} xShapeQueryVersionReq;
#define sz_xShapeQueryVersionReq	4

typedef struct {
	BYTE	type;			/* X_Reply */
	CARD8	unused;			/* not used */
	CARD16	sequenceNumber;
	CARD32	length;
	CARD16	majorVersion;		/* major version of SHAPE protocol */
	CARD16	minorVersion;		/* minor version of SHAPE protocol */
	CARD32	pad0;
	CARD32	pad1;
	CARD32	pad2;
	CARD32	pad3;
	CARD32	pad4;
} xShapeQueryVersionReply;
#define sz_xShapeQueryVersionReply	32

typedef struct _ShapeRectangles {
	CARD8	reqType;	/* always ShapeReqCode */
	CARD8	shapeReqType;	/* always X_ShapeRectangles */
	CARD16	length;
	CARD8	op;		/* Set, ... */
	CARD8	destKind;	/* ShapeBounding or ShapeClip */
	CARD8	ordering;	/* UnSorted, YSorted, YXSorted, YXBanded */
	CARD8	pad0;		/* not used */
	Window	dest;
	INT16	xOff;
	INT16	yOff;
} xShapeRectanglesReq;		/* followed by xRects */
#define sz_xShapeRectanglesReq	16

typedef struct _ShapeMask {
	CARD8	reqType;	/* always ShapeReqCode */
	CARD8	shapeReqType;	/* always X_ShapeMask */
	CARD16	length;

	CARD8	op;		/* Set, ... */
	CARD8	destKind;	/* ShapeBounding or ShapeClip */
	CARD16	junk;		/* not used */

	Window	dest;
	INT16	xOff;
	INT16	yOff;
	CARD32	src;		/* 1 bit pixmap */
} xShapeMaskReq;
#define sz_xShapeMaskReq	20

typedef struct _ShapeCombine {
	CARD8	reqType;	/* always ShapeReqCode */
	CARD8	shapeReqType;	/* always X_ShapeCombine */
	CARD16	length;
	CARD8	op;		/* Set, ... */
	CARD8	destKind;	/* ShapeBounding or ShapeClip */
	CARD8	srcKind;	/* ShapeBounding or ShapeClip */
	CARD8	junk;		/* not used */
	Window	dest;
	INT16	xOff;
	INT16	yOff;
	Window	src;
} xShapeCombineReq;
#define sz_xShapeCombineReq	20

typedef struct _ShapeOffset {
	CARD8	reqType;	/* always ShapeReqCode */
	CARD8	shapeReqType;	/* always X_ShapeOffset */
	CARD16	length;
	CARD8	destKind;	/* ShapeBounding or ShapeClip */
	CARD8	junk1;		/* not used */
	CARD16	junk2;		/* not used */
	Window	dest;
	INT16	xOff;
	INT16	yOff;
} xShapeOffsetReq;
#define sz_xShapeOffsetReq	16

typedef struct _ShapeQueryExtents {
	CARD8	reqType;	/* always ShapeReqCode */
	CARD8	shapeReqType;	/* always X_ShapeQueryExtents */
	CARD16	length;
	Window	window;
} xShapeQueryExtentsReq;
#define sz_xShapeQueryExtentsReq	8

typedef struct {
	BYTE	type;			/* X_Reply */
	CARD8	unused;			/* not used */
	CARD16	sequenceNumber;
	CARD32	length;			/* 0 */
	CARD8	boundingShaped;		/* window has bounding shape */
	CARD8	clipShaped;		/* window has clip shape */
	CARD16	unused1;
	INT16	xBoundingShape;		/* extents of bounding shape */
	INT16	yBoundingShape;
	CARD16	widthBoundingShape;
	CARD16	heightBoundingShape;
	INT16	xClipShape;		/* extents of clip shape */
	INT16	yClipShape;
	CARD16	widthClipShape;
	CARD16	heightClipShape;
	CARD32	pad1;
} xShapeQueryExtentsReply;
#define sz_xShapeQueryExtentsReply	32

typedef struct _ShapeSelectInput {
	CARD8	reqType;	/* always ShapeReqCode */
	CARD8	shapeReqType;	/* always X_ShapeSelectInput */
	CARD16	length;
	Window	window;
	BYTE	enable;		/* xTrue -> send events */
	BYTE	pad1;
	CARD16	pad2;
} xShapeSelectInputReq;
#define sz_xShapeSelectInputReq	12

typedef struct _ShapeNotify {
	BYTE	type;		/* always eventBase + ShapeNotify */
	BYTE	kind;		/* either ShapeBounding or ShapeClip */
	CARD16	sequenceNumber;
	Window	window;
	INT16	x;
	INT16	y;		/* extents of new shape */
	CARD16	width;
	CARD16	height;
	Time	time;		/* time of change */
	BYTE	shaped;		/* set when a shape actual exists */
	BYTE	pad0;
	CARD16	pad1;
	CARD32	pad2;
	CARD32	pad3;
} xShapeNotifyEvent;
#define sz_xShapeNotifyEvent	32

typedef struct _ShapeInputSelected {
	CARD8	reqType;	/* always ShapeReqCode */
	CARD8	shapeReqType;	/* always X_ShapeInputSelected */
	CARD16	length;
	Window	window;
} xShapeInputSelectedReq;
#define sz_xShapeInputSelectedReq 8

typedef struct {
	BYTE	type;			/* X_Reply */
	CARD8	enabled;		/* current status */
	CARD16	sequenceNumber;
	CARD32	length;		/* 0 */
	CARD32	pad1;		/* unused */
	CARD32	pad2;
	CARD32	pad3;
	CARD32	pad4;
	CARD32	pad5;
	CARD32	pad6;
} xShapeInputSelectedReply;
#define sz_xShapeInputSelectedReply	32

typedef struct _ShapeGetRectangles {
    CARD8   reqType;		/* always ShapeReqCode */
    CARD8   shapeReqType;	/* always X_ShapeGetRectangles */
    CARD16  length;
    Window  window;
    CARD8   kind;		/* ShapeBounding or ShapeClip */
    CARD8   junk1;
    CARD16  junk2;
} xShapeGetRectanglesReq;
#define sz_xShapeGetRectanglesReq	12

typedef struct {
	BYTE	type;			/* X_Reply */
	CARD8	ordering;	/* UnSorted, YSorted, YXSorted, YXBanded */
	CARD16	sequenceNumber;
	CARD32	length;		/* not zero */
	CARD32	nrects;		/* number of rectangles */
	CARD32	pad1;
	CARD32	pad2;
	CARD32	pad3;
	CARD32	pad4;
	CARD32	pad5;
} xShapeGetRectanglesReply;		/* followed by xRectangles */
#define sz_xShapeGetRectanglesReply 32

#undef Window
#undef Time

#endif /* _SHAPEPROTO_H_ */
