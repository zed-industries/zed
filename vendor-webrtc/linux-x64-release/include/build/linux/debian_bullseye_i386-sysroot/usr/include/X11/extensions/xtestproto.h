/*

Copyright 1992, 1998  The Open Group

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

#ifndef _XTESTPROTO_H_
#define _XTESTPROTO_H_

#include <X11/extensions/xtestconst.h>

#define Window CARD32
#define Time CARD32
#define Cursor CARD32

#define X_XTestGetVersion	0
#define X_XTestCompareCursor	1
#define X_XTestFakeInput	2
#define X_XTestGrabControl	3

typedef struct {
    CARD8	reqType;	/* always XTestReqCode */
    CARD8	xtReqType;	/* always X_XTestGetVersion */
    CARD16	length;
    CARD8	majorVersion;
    CARD8	pad;
    CARD16	minorVersion;
} xXTestGetVersionReq;
#define sz_xXTestGetVersionReq 8

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	majorVersion;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	minorVersion;
    CARD16	pad0;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xXTestGetVersionReply;
#define sz_xXTestGetVersionReply 32

typedef struct {
    CARD8	reqType;	/* always XTestReqCode */
    CARD8	xtReqType;	/* always X_XTestCompareCursor */
    CARD16	length;
    Window	window;
    Cursor	cursor;
} xXTestCompareCursorReq;
#define sz_xXTestCompareCursorReq 12

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	same;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad0;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xXTestCompareCursorReply;
#define sz_xXTestCompareCursorReply 32

/* used only on the client side */
typedef struct {
    CARD8	reqType;	/* always XTestReqCode */
    CARD8	xtReqType;	/* always X_XTestFakeInput */
    CARD16	length;
    BYTE	type;
    BYTE	detail;
    CARD16	pad0;
    Time	time;
    Window	root;
    CARD32	pad1;
    CARD32	pad2;
    INT16	rootX, rootY;
    CARD32	pad3;
    CARD16	pad4;
    CARD8	pad5;
    CARD8	deviceid;
} xXTestFakeInputReq;
#define sz_xXTestFakeInputReq 36

typedef struct {
    CARD8	reqType;	/* always XTestReqCode */
    CARD8	xtReqType;	/* always X_XTestGrabControl */
    CARD16	length;
    BOOL	impervious;
    CARD8	pad0;
    CARD8	pad1;
    CARD8	pad2;
} xXTestGrabControlReq;
#define sz_xXTestGrabControlReq 8

#undef Window
#undef Time
#undef Cursor

#endif /* _XTESTPROTO_H_ */
