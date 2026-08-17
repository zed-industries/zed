/*
 * Copyright 1992 Network Computing Devices
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of NCD. not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  NCD. makes no representations about the
 * suitability of this software for any purpose.  It is provided "as is"
 * without express or implied warranty.
 *
 * NCD. DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING ALL
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL NCD.
 * BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
 * OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
 * CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 */

#ifndef _LBXPROTO_H_
#define _LBXPROTO_H_

#include <X11/extensions/lbx.h>
/*
 * NOTE:  any changes or additions to the opcodes needs to be reflected
 * in the lbxCacheable array in Xserver/lbx/lbxmain.c
 */

#define X_LbxQueryVersion		0
#define X_LbxStartProxy			1
#define X_LbxStopProxy			2
#define X_LbxSwitch			3
#define X_LbxNewClient			4
#define X_LbxCloseClient		5
#define X_LbxModifySequence		6
#define X_LbxAllowMotion		7
#define X_LbxIncrementPixel		8
#define X_LbxDelta			9
#define	X_LbxGetModifierMapping		10
#define	X_LbxInvalidateTag		12
#define X_LbxPolyPoint			13
#define X_LbxPolyLine			14
#define X_LbxPolySegment		15
#define X_LbxPolyRectangle		16
#define X_LbxPolyArc			17
#define X_LbxFillPoly			18
#define X_LbxPolyFillRectangle		19
#define X_LbxPolyFillArc		20
#define	X_LbxGetKeyboardMapping		21
#define	X_LbxQueryFont			22
#define	X_LbxChangeProperty		23
#define	X_LbxGetProperty		24
#define	X_LbxTagData			25

#define X_LbxCopyArea			26
#define X_LbxCopyPlane			27
#define X_LbxPolyText8			28
#define X_LbxPolyText16			29
#define X_LbxImageText8			30
#define X_LbxImageText16		31

#define X_LbxQueryExtension		32
#define X_LbxPutImage			33
#define X_LbxGetImage			34

#define X_LbxBeginLargeRequest		35
#define X_LbxLargeRequestData		36
#define X_LbxEndLargeRequest		37

#define X_LbxInternAtoms		38
#define X_LbxGetWinAttrAndGeom		39

#define X_LbxGrabCmap			40
#define X_LbxReleaseCmap		41
#define X_LbxAllocColor			42

#define X_LbxSync			43

/*
 * Redefine some basic types used by structures defined herein.  This removes
 * any possibility on 64-bit architectures of one entity viewing communicated
 * data as 32-bit quantities and another entity viewing the same data as 64-bit
 * quantities.
 */
#define XID CARD32
#define Atom CARD32
#define Colormap CARD32
#define Drawable CARD32
#define VisualID CARD32
#define Window CARD32

typedef struct {
    BOOL	success;		/* TRUE */
    BOOL	changeType;
    CARD16	majorVersion,
		minorVersion;
    CARD16	length;			/* 1/4 additional bytes in setup info */
    CARD32	tag;
} xLbxConnSetupPrefix;

typedef struct _LbxQueryVersion {
    CARD8	reqType;		/* always LbxReqCode */
    CARD8	lbxReqType;		/* always X_LbxQueryVersion */
    CARD16	length;
} xLbxQueryVersionReq;
#define sz_xLbxQueryVersionReq	4

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	majorVersion;		/* major version of LBX protocol */
    CARD16	minorVersion;		/* minor version of LBX protocol */
    CARD32	pad0;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
} xLbxQueryVersionReply;
#define sz_xLbxQueryVersionReply	32

typedef struct _LbxStartProxy {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxStartProxy */
    CARD16	length;
} xLbxStartProxyReq;
#define sz_xLbxStartProxyReq	    4

typedef struct _LbxStopProxy {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxStopProxy */
    CARD16	length;
} xLbxStopProxyReq;
#define sz_xLbxStopProxyReq	    4

typedef struct _LbxSwitch {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxSwitch */
    CARD16	length;
    CARD32	client;		/* new client */
} xLbxSwitchReq;
#define sz_xLbxSwitchReq	8

typedef struct _LbxNewClient {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxNewClient */
    CARD16	length;
    CARD32	client;		/* new client */
} xLbxNewClientReq;
#define sz_xLbxNewClientReq	8

typedef struct _LbxCloseClient {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxCloseClient */
    CARD16	length;
    CARD32	client;		/* new client */
} xLbxCloseClientReq;
#define sz_xLbxCloseClientReq	8

typedef struct _LbxModifySequence {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxModifySequence */
    CARD16	length;
    CARD32	adjust;
} xLbxModifySequenceReq;
#define sz_xLbxModifySequenceReq    8

typedef struct _LbxAllowMotion {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxAllowMotion */
    CARD16	length;
    CARD32	num;
} xLbxAllowMotionReq;
#define sz_xLbxAllowMotionReq    8

typedef struct {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxGrabCmap */
    CARD16	length;
    Colormap	cmap;
} xLbxGrabCmapReq;
#define sz_xLbxGrabCmapReq	8

#define LBX_SMART_GRAB		0x80
#define LBX_AUTO_RELEASE	0x40
#define LBX_3CHANNELS		0x20
#define LBX_2BYTE_PIXELS	0x10
#define LBX_RGB_BITS_MASK	0x0f

#define LBX_LIST_END		0
#define LBX_PIXEL_PRIVATE	1
#define LBX_PIXEL_SHARED	2
#define LBX_PIXEL_RANGE_PRIVATE	3
#define LBX_PIXEL_RANGE_SHARED	4
#define LBX_NEXT_CHANNEL	5

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	flags;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad0;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xLbxGrabCmapReply;
#define sz_xLbxGrabCmapReply	32
#define sz_xLbxGrabCmapReplyHdr	8


typedef struct {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxReleaseCmap */
    CARD16	length;
    Colormap	cmap;
} xLbxReleaseCmapReq;
#define sz_xLbxReleaseCmapReq	8

typedef struct {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxAllocColor */
    CARD16	length;
    Colormap	cmap;
    CARD32	pixel;
    CARD16	red, green, blue;
    CARD16	pad;
} xLbxAllocColorReq;
#define sz_xLbxAllocColorReq	20

typedef struct _LbxIncrementPixel {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxIncrementPixel */
    CARD16	length;
    CARD32	cmap;
    CARD32	pixel;
} xLbxIncrementPixelReq;
#define sz_xLbxIncrementPixelReq    12

typedef struct _LbxDelta {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxDelta */
    CARD16	length;
    CARD8	diffs;		/* number of diffs */
    CARD8	cindex;		/* cache index */
				/* list of diffs follows */
} xLbxDeltaReq;
#define sz_xLbxDeltaReq    6

typedef struct _LbxGetModifierMapping {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxGetModifierMapping */
    CARD16	length;
} xLbxGetModifierMappingReq;
#define	sz_xLbxGetModifierMappingReq	4

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	keyspermod;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	tag;
    CARD32	pad0;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
} xLbxGetModifierMappingReply;
#define sz_xLbxGetModifierMappingReply	32

typedef struct _LbxGetKeyboardMapping {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxGetKeyboardMapping */
    CARD16	length;
    KeyCode	firstKeyCode;
    CARD8	count;
    CARD16	pad1;
} xLbxGetKeyboardMappingReq;
#define	sz_xLbxGetKeyboardMappingReq	8

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	keysperkeycode;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	tag;
    CARD32	pad0;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
} xLbxGetKeyboardMappingReply;
#define sz_xLbxGetKeyboardMappingReply	32

typedef struct _LbxQueryFont {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxQueryFont */
    CARD16	length;
    CARD32	fid;
} xLbxQueryFontReq;
#define	sz_xLbxQueryFontReq	8

typedef struct _LbxInternAtoms {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxInternAtoms */
    CARD16	length;
    CARD16	num;
} xLbxInternAtomsReq;
#define sz_xLbxInternAtomsReq	6

typedef struct {
    BYTE	type;		/* X_Reply */
    CARD8	unused;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	atomsStart;
    CARD32	pad0;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
} xLbxInternAtomsReply;
#define sz_xLbxInternAtomsReply		32
#define sz_xLbxInternAtomsReplyHdr	8


typedef struct _LbxGetWinAttrAndGeom {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxGetWinAttrAndGeom */
    CARD16	length;
    CARD32	id;		/* window id */
} xLbxGetWinAttrAndGeomReq;
#define sz_xLbxGetWinAttrAndGeomReq 8

typedef struct {
    BYTE type;  /* X_Reply */
    CARD8 backingStore;
    CARD16 sequenceNumber;
    CARD32 length;		/* NOT 0; this is an extra-large reply */
    VisualID visualID;
#if defined(__cplusplus) || defined(c_plusplus)
    CARD16 c_class;
#else
    CARD16 class;
#endif
    CARD8 bitGravity;
    CARD8 winGravity;
    CARD32 backingBitPlanes;
    CARD32 backingPixel;
    BOOL saveUnder;
    BOOL mapInstalled;
    CARD8 mapState;
    BOOL override;
    Colormap colormap;
    CARD32 allEventMasks;
    CARD32 yourEventMask;
    CARD16 doNotPropagateMask;
    CARD16 pad1;
    Window root;
    INT16 x, y;
    CARD16 width, height;
    CARD16 borderWidth;
    CARD8 depth;
    CARD8 pad2;
} xLbxGetWinAttrAndGeomReply;
#define sz_xLbxGetWinAttrAndGeomReply 60


typedef struct {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxSync */
    CARD16	length;
} xLbxSyncReq;
#define sz_xLbxSyncReq	4

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	pad0;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xLbxSyncReply;
#define sz_xLbxSyncReply 32


/* an LBX squished charinfo packs the data in a CARD32 as follows */
#define	LBX_WIDTH_SHIFT		26
#define	LBX_LEFT_SHIFT		20
#define	LBX_RIGHT_SHIFT		13
#define	LBX_ASCENT_SHIFT	7
#define	LBX_DESCENT_SHIFT	0

#define	LBX_WIDTH_BITS		6
#define	LBX_LEFT_BITS		6
#define	LBX_RIGHT_BITS		7
#define	LBX_ASCENT_BITS		6
#define	LBX_DESCENT_BITS	7

#define	LBX_WIDTH_MASK		0xfc000000
#define	LBX_LEFT_MASK		0x03f00000
#define	LBX_RIGHT_MASK		0x000fe000
#define	LBX_ASCENT_MASK		0x00001f80
#define	LBX_DESCENT_MASK	0x0000007f

#define	LBX_MASK_BITS(val, n)	((unsigned int) ((val) & ((1 << (n)) - 1)))

typedef struct {
    CARD32	metrics;
} xLbxCharInfo;

/* note that this is identical to xQueryFontReply except for missing
 * first 2 words
 */
typedef struct {
    xCharInfo minBounds;
/* XXX do we need to leave this gunk? */
#ifndef WORD64
    CARD32 walign1;
#endif
    xCharInfo maxBounds;
#ifndef WORD64
    CARD32 walign2;
#endif
    CARD16 minCharOrByte2, maxCharOrByte2;
    CARD16 defaultChar;
    CARD16 nFontProps;	/* followed by this many xFontProp structures */
    CARD8 drawDirection;
    CARD8 minByte1, maxByte1;
    BOOL allCharsExist;
    INT16 fontAscent, fontDescent;
    CARD32 nCharInfos;	/* followed by this many xLbxCharInfo structures */
} xLbxFontInfo;

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	compression;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	tag;
    CARD32	pad0;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    /* X_QueryFont sticks much of the data in the base reply packet,
     * but we hope that it won't be needed, (and it won't fit in 32 bytes
     * with the tag anyways)
     *
     * if any additional data is needed, its sent in a xLbxFontInfo
     */
} xLbxQueryFontReply;
#define sz_xLbxQueryFontReply	32

typedef struct _LbxChangeProperty {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxChangeProperty */
    CARD16	length;
    Window	window;
    Atom	property;
    Atom	type;
    CARD8	format;
    CARD8	mode;
    BYTE	pad[2];
    CARD32	nUnits;
} xLbxChangePropertyReq;
#define	sz_xLbxChangePropertyReq	24

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	pad;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	tag;
    CARD32	pad0;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
} xLbxChangePropertyReply;
#define sz_xLbxChangePropertyReply	32

typedef struct _LbxGetProperty {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxGetProperty */
    CARD16	length;
    Window	window;
    Atom	property;
    Atom	type;
    CARD8	delete;
    BYTE	pad[3];
    CARD32	longOffset;
    CARD32	longLength;
} xLbxGetPropertyReq;
#define	sz_xLbxGetPropertyReq	28

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	format;
    CARD16	sequenceNumber;
    CARD32	length;
    Atom	propertyType;
    CARD32	bytesAfter;
    CARD32	nItems;
    CARD32	tag;
    CARD32	pad1;
    CARD32	pad2;
} xLbxGetPropertyReply;
#define sz_xLbxGetPropertyReply	32

typedef struct _LbxTagData {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxTagData */
    CARD16	length;
    XID 	tag;
    CARD32	real_length;
    /* data */
} xLbxTagDataReq;
#define	sz_xLbxTagDataReq	12

typedef struct _LbxInvalidateTag {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxInvalidateTag */
    CARD16	length;
    CARD32	tag;
} xLbxInvalidateTagReq;
#define	sz_xLbxInvalidateTagReq	8

typedef struct _LbxPutImage {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxPutImage */
    CARD16	length;
    CARD8	compressionMethod;
    CARD8	cacheEnts;
    CARD8	bitPacked;
    /* rest is variable */
} xLbxPutImageReq;
#define sz_xLbxPutImageReq	7

typedef struct {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxGetImage */
    CARD16	length;
    Drawable	drawable;
    INT16	x, y;
    CARD16	width, height;
    CARD32	planeMask;
    CARD8	format;
    CARD8	pad1;
    CARD16	pad2;
} xLbxGetImageReq;

#define sz_xLbxGetImageReq 24

typedef struct {
    BYTE type;			/* X_Reply */
    CARD8 depth;
    CARD16 sequenceNumber;
    CARD32 lbxLength;
    CARD32 xLength;
    VisualID visual;
    CARD8 compressionMethod;
    CARD8 pad1;
    CARD16 pad2;
    CARD32 pad3;
    CARD32 pad4;
    CARD32 pad5;
} xLbxGetImageReply;

#define sz_xLbxGetImageReply 32

/* Following used for LbxPolyPoint, LbxPolyLine, LbxPolySegment,
   LbxPolyRectangle, LbxPolyArc, LbxPolyFillRectangle and LbxPolyFillArc */

#define GFX_CACHE_SIZE  15

#define GFXdCacheEnt(e)	    ((e) & 0xf)
#define GFXgCacheEnt(e)	    (((e) >> 4) & 0xf)
#define GFXCacheEnts(d,g)   (((d) & 0xf) | (((g) & 0xf) << 4))

#define GFXCacheNone   0xf

typedef struct _LbxPolyPoint {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;
    CARD16	length;
    CARD8	cacheEnts;
    CARD8	padBytes;
} xLbxPolyPointReq;

#define sz_xLbxPolyPointReq	6

typedef xLbxPolyPointReq xLbxPolyLineReq;
typedef xLbxPolyPointReq xLbxPolySegmentReq;
typedef xLbxPolyPointReq xLbxPolyRectangleReq;
typedef xLbxPolyPointReq xLbxPolyArcReq;
typedef xLbxPolyPointReq xLbxPolyFillRectangleReq;
typedef xLbxPolyPointReq xLbxPolyFillArcReq;

#define sz_xLbxPolyLineReq		sz_xLbxPolyPointReq
#define sz_xLbxPolySegmentReq		sz_xLbxPolyPointReq
#define sz_xLbxPolyRectangleReq		sz_xLbxPolyPointReq
#define sz_xLbxPolyArcReq		sz_xLbxPolyPointReq
#define sz_xLbxPolyFillRectangleReq	sz_xLbxPolyPointReq
#define sz_xLbxPolyFillArc		sz_xLbxPolyPointReq

typedef struct _LbxFillPoly {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;
    CARD16	length;
    CARD8	cacheEnts;
    BYTE	shape;
    CARD8	padBytes;
} xLbxFillPolyReq;
#define sz_xLbxFillPolyReq	7

typedef struct _LbxCopyArea {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;
    CARD16	length;
    CARD8	srcCache;	/* source drawable */
    CARD8	cacheEnts;	/* dest drawable and gc */
    /* followed by encoded src x, src y, dst x, dst y, width, height */
} xLbxCopyAreaReq;

#define sz_xLbxCopyAreaReq  6

typedef struct _LbxCopyPlane {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;
    CARD16	length;
    CARD32	bitPlane;
    CARD8	srcCache;	/* source drawable */
    CARD8	cacheEnts;	/* dest drawable and gc */
    /* followed by encoded src x, src y, dst x, dst y, width, height */
} xLbxCopyPlaneReq;

#define sz_xLbxCopyPlaneReq  10

typedef struct _LbxPolyText {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;
    CARD16	length;
    CARD8	cacheEnts;
    /* followed by encoded src x, src y coordinates and text elts */
} xLbxPolyTextReq;

#define sz_xLbxPolyTextReq  5

typedef xLbxPolyTextReq xLbxPolyText8Req;
typedef xLbxPolyTextReq xLbxPolyText16Req;

#define sz_xLbxPolyTextReq	5
#define sz_xLbxPolyText8Req	5
#define sz_xLbxPolyText16Req	5

typedef struct _LbxImageText {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;
    CARD16	length;
    CARD8	cacheEnts;
    CARD8	nChars;
    /* followed by encoded src x, src y coordinates and string */
} xLbxImageTextReq;

typedef xLbxImageTextReq xLbxImageText8Req;
typedef xLbxImageTextReq xLbxImageText16Req;

#define sz_xLbxImageTextReq	6
#define sz_xLbxImageText8Req	6
#define sz_xLbxImageText16Req	6

typedef struct {
    CARD8       offset;
    CARD8       diff;
} xLbxDiffItem;
#define sz_xLbxDiffItem    2

typedef struct {
    BYTE	type;		/* X_Reply */
    CARD8	nOpts;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	optDataStart;
    CARD32	pad0;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
} xLbxStartReply;
#define sz_xLbxStartReply	32
#define sz_xLbxStartReplyHdr	8

typedef struct _LbxQueryExtension {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxQueryExtension */
    CARD16	length;
    CARD32	nbytes;
} xLbxQueryExtensionReq;
#define	sz_xLbxQueryExtensionReq	8

typedef struct _LbxQueryExtensionReply {
    BYTE	type;			/* X_Reply */
    CARD8	numReqs;
    CARD16	sequenceNumber;
    CARD32	length;
    BOOL	present;
    CARD8	major_opcode;
    CARD8	first_event;
    CARD8	first_error;
    CARD32	pad0;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;

    /* reply & event generating requests */
} xLbxQueryExtensionReply;
#define sz_xLbxQueryExtensionReply	32


typedef struct _LbxBeginLargeRequest {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxBeginLargeRequest */
    CARD16	length;
    CARD32	largeReqLength;
} xLbxBeginLargeRequestReq;
#define	sz_BeginLargeRequestReq 8

typedef struct _LbxLargeRequestData {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxLargeRequestData */
    CARD16	length;
    /* followed by LISTofCARD8 data */
} xLbxLargeRequestDataReq;
#define	sz_LargeRequestDataReq 4

typedef struct _LbxEndLargeRequest {
    CARD8	reqType;	/* always LbxReqCode */
    CARD8	lbxReqType;	/* always X_LbxEndLargeRequest */
    CARD16	length;
} xLbxEndLargeRequestReq;
#define	sz_EndLargeRequestReq 4



typedef struct _LbxSwitchEvent {
    BYTE	type;		/* always eventBase + LbxEvent */
    BYTE	lbxType;	/* LbxSwitchEvent */
    CARD16	pad;
    CARD32	client;
} xLbxSwitchEvent;
#define sz_xLbxSwitchEvent	8

typedef struct _LbxCloseEvent {
    BYTE	type;		/* always eventBase + LbxEvent */
    BYTE	lbxType;	/* LbxCloseEvent */
    CARD16	sequenceNumber;
    CARD32	client;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xLbxCloseEvent;
#define sz_xLbxCloseEvent	32

typedef struct _LbxInvalidateTagEvent {
    BYTE	type;		/* always eventBase + LbxEvent */
    BYTE	lbxType;	/* LbxInvalidateTagEvent */
    CARD16	sequenceNumber;
    CARD32	tag;
    CARD32	tagType;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xLbxInvalidateTagEvent;
#define sz_xLbxInvalidateTagEvent 32

typedef struct _LbxSendTagDataEvent {
    BYTE	type;		/* always eventBase + LbxEvent */
    BYTE	lbxType;	/* LbxSendTagDataEvent */
    CARD16	sequenceNumber;
    CARD32	tag;
    CARD32	tagType;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xLbxSendTagDataEvent;
#define sz_xLbxSendTagDataEvent 32

typedef struct _LbxListenToOneEvent {
    BYTE	type;		/* always eventBase + LbxEvent */
    BYTE	lbxType;	/* LbxListenToOneEvent */
    CARD16	sequenceNumber;
    CARD32	client;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xLbxListenToOneEvent;
#define sz_xLbxListenToOneEvent 32

typedef struct _LbxListenToAllEvent {
    BYTE	type;		/* always eventBase + LbxEvent */
    BYTE	lbxType;	/* LbxListenToAllEvent */
    CARD16	sequenceNumber;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
    CARD32	pad7;
} xLbxListenToAllEvent;
#define sz_xLbxListenToOneEvent 32

typedef struct _LbxReleaseCmapEvent {
    BYTE	type;		/* always eventBase + LbxEvent */
    BYTE	lbxType;	/* LbxReleaseCmapEvent */
    CARD16	sequenceNumber;
    Colormap	colormap;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xLbxReleaseCmapEvent;
#define sz_xLbxReleaseCmapEvent	32


typedef struct _LbxFreeCellsEvent {
    BYTE	type;		/* always eventBase + LbxEvent */
    BYTE	lbxType;	/* LbxFreeCellsEvent */
    CARD16	sequenceNumber;
    Colormap	colormap;
    CARD32	pixelStart;
    CARD32	pixelEnd;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
} xLbxFreeCellsEvent;
#define sz_xLbxFreeCellsEvent	32


/*
 * squished X event sizes.  If these change, be sure to update lbxquish.c
 * and unsquish.c appropriately
 *
 * lbxsz_* is the padded squished length
 * lbxupsz_* is the unpadded squished length
 */

#define	  lbxsz_KeyButtonEvent		32
#define	lbxupsz_KeyButtonEvent		31

#define	  lbxsz_EnterLeaveEvent		32
#define	lbxupsz_EnterLeaveEvent		32

#define	  lbxsz_FocusEvent		12
#define	lbxupsz_FocusEvent		9

#define	  lbxsz_KeymapEvent		32
#define	lbxupsz_KeymapEvent		32

#define	  lbxsz_ExposeEvent		20
#define	lbxupsz_ExposeEvent		18

#define	  lbxsz_GfxExposeEvent		24
#define	lbxupsz_GfxExposeEvent		21

#define	  lbxsz_NoExposeEvent		12
#define	lbxupsz_NoExposeEvent		11

#define	  lbxsz_VisibilityEvent		12
#define	lbxupsz_VisibilityEvent		9

#define	  lbxsz_CreateNotifyEvent	24
#define	lbxupsz_CreateNotifyEvent	23

#define	  lbxsz_DestroyNotifyEvent	12
#define	lbxupsz_DestroyNotifyEvent	12

#define	  lbxsz_UnmapNotifyEvent	16
#define	lbxupsz_UnmapNotifyEvent	13

#define	  lbxsz_MapNotifyEvent		16
#define	lbxupsz_MapNotifyEvent		13

#define	  lbxsz_MapRequestEvent		12
#define	lbxupsz_MapRequestEvent		12

#define	  lbxsz_ReparentEvent		24
#define	lbxupsz_ReparentEvent		21

#define	  lbxsz_ConfigureNotifyEvent	28
#define	lbxupsz_ConfigureNotifyEvent	27

#define	  lbxsz_ConfigureRequestEvent	28
#define	lbxupsz_ConfigureRequestEvent	28

#define	  lbxsz_GravityEvent		16
#define	lbxupsz_GravityEvent		16

#define	  lbxsz_ResizeRequestEvent	12
#define	lbxupsz_ResizeRequestEvent	12

#define	  lbxsz_CirculateEvent		20
#define	lbxupsz_CirculateEvent		17

#define	  lbxsz_PropertyEvent		20
#define	lbxupsz_PropertyEvent		17

#define	  lbxsz_SelectionClearEvent	16
#define	lbxupsz_SelectionClearEvent	16

#define	  lbxsz_SelectionRequestEvent	28
#define	lbxupsz_SelectionRequestEvent	28

#define	  lbxsz_SelectionNotifyEvent	24
#define	lbxupsz_SelectionNotifyEvent	24

#define	  lbxsz_ColormapEvent		16
#define	lbxupsz_ColormapEvent		14

#define	  lbxsz_MappingNotifyEvent	8
#define	lbxupsz_MappingNotifyEvent	7

#define	  lbxsz_ClientMessageEvent	32
#define	lbxupsz_ClientMessageEvent	32

#define	lbxsz_UnknownEvent		32

#ifdef DEBUG

#define DBG_SWITCH	0x00000001
#define DBG_CLOSE	0x00000002
#define DBG_IO		0x00000004
#define DBG_READ_REQ	0x00000008
#define DBG_LEN		0x00000010
#define DBG_BLOCK	0x00000020
#define DBG_CLIENT	0x00000040
#define DBG_DELTA	0x00000080
#endif
/*
 * Cancel the previous redefinition of the basic types, thus restoring their
 * X.h definitions.
 */

#undef XID
#undef Atom
#undef Colormap
#undef Drawable
#undef VisualID
#undef Window

#endif	/* _LBXPROTO_H_ */
