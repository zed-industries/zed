/*
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
 */

#ifndef _MULTIBUFPROTO_H_
#define _MULTIBUFPROTO_H_

#include <X11/extensions/multibufconst.h>

/*
 * Protocol requests constants and alignment values
 */

#define Window CARD32
#define Drawable CARD32
#define VisualID CARD32
#define Multibuffer CARD32

#define X_MbufGetBufferVersion		0
#define X_MbufCreateImageBuffers	1
#define X_MbufDestroyImageBuffers	2
#define X_MbufDisplayImageBuffers	3
#define X_MbufSetMBufferAttributes	4
#define X_MbufGetMBufferAttributes	5
#define X_MbufSetBufferAttributes	6
#define X_MbufGetBufferAttributes	7
#define X_MbufGetBufferInfo		8
#define X_MbufCreateStereoWindow	9
#define X_MbufClearImageBufferArea	10


typedef struct xMbufBufferInfo {
	CARD32	visualID;		/* associated visual */
	CARD16	maxBuffers;		/* maximum supported buffers */
	CARD8	depth;			/* depth of visual (redundant) */
	CARD8	unused;
} xMbufBufferInfo;
#define sz_xMbufBufferInfo 8

typedef struct {
    BYTE    type;
    BYTE    unused;
    CARD16  sequenceNumber;
    CARD32  buffer;			/* affected buffer */
    BYTE    state;			/* current status */
    CARD8   unused1;
    CARD16  unused2;
    CARD32  unused3;
    CARD32  unused4;
    CARD32  unused5;
    CARD32  unused6;
    CARD32  unused7;
} xMbufClobberNotifyEvent;

typedef struct {
    BYTE    type;
    BYTE    unused;
    CARD16  sequenceNumber;
    CARD32  buffer;			/* affected buffer */
    CARD32  timeStamp;			/* update time */
    CARD32  unused1;
    CARD32  unused2;
    CARD32  unused3;
    CARD32  unused4;
    CARD32  unused5;
    CARD32  unused6;
} xMbufUpdateNotifyEvent;

typedef struct {
    CARD8	reqType;		/* always codes->major_opcode */
    CARD8	mbufReqType;		/* always X_MbufGetBufferVersion */
    CARD16	length;
} xMbufGetBufferVersionReq;
#define sz_xMbufGetBufferVersionReq	4

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD8	majorVersion;	/* major version of Multi-Buffering protocol */
    CARD8	minorVersion;	/* minor version of Multi-Buffering protocol */
    CARD16	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xMbufGetBufferVersionReply;
#define sz_xMbufGetBufferVersionReply	32

typedef struct {
    CARD8	reqType;	/* always codes->major_opcode */
    CARD8	mbufReqType;	/* always X_MbufCreateImageBuffers */
    CARD16	length;
    CARD32	window; 	/* associated window */
    CARD8	updateAction;	/* action at update */
    CARD8	updateHint;	/* hint as to frequency of updates */
    CARD16	unused;
} xMbufCreateImageBuffersReq;	/* followed by buffer ids */
#define sz_xMbufCreateImageBuffersReq	12

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	numberBuffer;		/* number successfully allocated */
    CARD16	unused1;
    CARD32	unused2;
    CARD32	unused3;
    CARD32	unused4;
    CARD32	unused5;
    CARD32	unused6;
} xMbufCreateImageBuffersReply;
#define sz_xMbufCreateImageBuffersReply 32

typedef struct {
    CARD8	reqType;	/* always codes->major_opcode */
    CARD8	mbufReqType;	/* always X_MbufDestroyImageBuffers */
    CARD16	length;
    CARD32	window; 	/* associated window */
} xMbufDestroyImageBuffersReq;
#define sz_xMbufDestroyImageBuffersReq	8

typedef struct {
    CARD8	reqType;	/* always codes->major_opcode */
    CARD8	mbufReqType;	/* always X_MbufDisplayImageBuffers */
    CARD16	length;
    CARD16	minDelay;	/* minimum time between last update and now */
    CARD16	maxDelay;	/* maximum time between last update and now */
} xMbufDisplayImageBuffersReq;	/* followed by list of buffers */
#define sz_xMbufDisplayImageBuffersReq	8

typedef struct {
    CARD8	reqType;	/* always codes->major_opcode */
    CARD8	mbufReqType;	/* always X_MbufSetMBufferAttributes */
    CARD16	length;
    CARD32	window; 	/* associated window */
    CARD32	valueMask;	/* modified entries */
} xMbufSetMBufferAttributesReq;	/* followed by values */
#define sz_xMbufSetMBufferAttributesReq 12

typedef struct {
    CARD8	reqType;	/* always codes->major_opcode */
    CARD8	mbufReqType;	/* always X_MbufGetMBufferAttributes */
    CARD16	length;
    CARD32	window; 	/* associated window */
} xMbufGetMBufferAttributesReq;
#define sz_xMbufGetMBufferAttributesReq 8

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	displayedBuffer;	/* currently visible buffer */
    CARD8	updateAction;
    CARD8	updateHint;
    CARD8	windowMode;
    CARD8	unused0;
    CARD16	unused1;
    CARD32	unused2;
    CARD32	unused3;
    CARD32	unused4;
    CARD32	unused5;
} xMbufGetMBufferAttributesReply;
#define sz_xMbufGetMBufferAttributesReply 32

typedef struct {
    CARD8	reqType;	/* always codes->major_opcode */
    CARD8	mbufReqType;	/* always X_MbufSetBufferAttributes */
    CARD16	length;
    CARD32	buffer;
    CARD32	valueMask;
} xMbufSetBufferAttributesReq;	/* followed by values */
#define sz_xMbufSetBufferAttributesReq 12

typedef struct {
    CARD8	reqType;	/* always codes->major_opcode */
    CARD8	mbufReqType;	/* always X_MbufGetBufferAttributes */
    CARD16	length;
    CARD32	buffer;
} xMbufGetBufferAttributesReq;
#define sz_xMbufGetBufferAttributesReq 8

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	window;
    CARD32	eventMask;
    CARD16	bufferIndex;
    CARD8	side;
    CARD8	unused0;
    CARD32	unused1;
    CARD32	unused2;
    CARD32	unused3;
} xMbufGetBufferAttributesReply;
#define sz_xMbufGetBufferAttributesReply 32

typedef struct {
    CARD8	reqType;	/* always codes->major_opcode */
    CARD8	mbufReqType;	/* always X_MbufGetBufferInfo */
    CARD16	length;
    Drawable	drawable;
} xMbufGetBufferInfoReq;
#define sz_xMbufGetBufferInfoReq 8

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	normalInfo;
    CARD16	stereoInfo;
    CARD32	unused1;
    CARD32	unused2;
    CARD32	unused3;
    CARD32	unused4;
    CARD32	unused5;
} xMbufGetBufferInfoReply;			/* followed by buffer infos */
#define sz_xMbufGetBufferInfoReply 32


typedef struct {
    CARD8	reqType;	/* always codes->major_opcode */
    CARD8	mbufReqType;	/* always X_MbufCreateStereoWindow */
    CARD16	length;
    CARD8	unused0;
    CARD8	unused1;
    CARD8	unused2;
    CARD8	depth;
    Window	wid;
    Window	parent;
    Multibuffer	left;		/* associated buffers */
    Multibuffer	right;
    INT16	x;
    INT16	y;
    CARD16	width;
    CARD16	height;
    CARD16	borderWidth;
#if defined(__cplusplus) || defined(c_plusplus)
    CARD16	c_class;
#else
    CARD16	class;
#endif
    VisualID	visual;
    CARD32	mask;
} xMbufCreateStereoWindowReq;		/* followed by value list */
#define sz_xMbufCreateStereoWindowReq 44

typedef struct {
    CARD8     reqType;        /* always codes->major_opcode */
    CARD8     mbufReqType;    /* always X_MbufClearImageBufferArea */
    CARD16    length;
    Multibuffer       buffer;
    INT16     x;
    INT16     y;
    CARD16    width;
    CARD16    height;
    CARD8     unused0;
    CARD8     unused1;
    CARD8     unused2;
    BOOL      exposures;
} xMbufClearImageBufferAreaReq;
#define sz_xMbufClearImageBufferAreaReq 20

#undef Window
#undef Drawable
#undef VisualID
#undef Multibuffer

#endif /* _MULTIBUFPROTO_H_ */
