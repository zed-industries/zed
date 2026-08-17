/*
 * Copyright (c) 2006, Oracle and/or its affiliates. All rights reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */
/*
 * Copyright © 2003 Keith Packard
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of Keith Packard not be used in
 * advertising or publicity pertaining to distribution of the software without
 * specific, written prior permission.  Keith Packard makes no
 * representations about the suitability of this software for any purpose.  It
 * is provided "as is" without express or implied warranty.
 *
 * KEITH PACKARD DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL KEITH PACKARD BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */

#ifndef _COMPOSITEPROTO_H_
#define _COMPOSITEPROTO_H_

#include <X11/Xmd.h>
#include <X11/extensions/composite.h>

#define Window CARD32
#define Region CARD32
#define Pixmap CARD32

/*
 * requests and replies
 */
typedef struct {
    CARD8   reqType;
    CARD8   compositeReqType;
    CARD16  length;
    CARD32  majorVersion;
    CARD32  minorVersion;
} xCompositeQueryVersionReq;

#define sz_xCompositeQueryVersionReq   12

typedef struct {
    BYTE    type;   /* X_Reply */
    BYTE    pad1;
    CARD16  sequenceNumber;
    CARD32  length;
    CARD32  majorVersion;
    CARD32  minorVersion;
    CARD32  pad2;
    CARD32  pad3;
    CARD32  pad4;
    CARD32  pad5;
} xCompositeQueryVersionReply;

#define sz_xCompositeQueryVersionReply	32

typedef struct {
    CARD8   reqType;
    CARD8   compositeReqType;
    CARD16  length;
    Window  window;
    CARD8   update;
    CARD8   pad1;
    CARD16  pad2;
} xCompositeRedirectWindowReq;

#define sz_xCompositeRedirectWindowReq	12

typedef struct {
    CARD8   reqType;
    CARD8   compositeReqType;
    CARD16  length;
    Window  window;
    CARD8   update;
    CARD8   pad1;
    CARD16  pad2;
} xCompositeRedirectSubwindowsReq;

#define sz_xCompositeRedirectSubwindowsReq	    12

typedef struct {
    CARD8   reqType;
    CARD8   compositeReqType;
    CARD16  length;
    Window  window;
    CARD8   update;
    CARD8   pad1;
    CARD16  pad2;
} xCompositeUnredirectWindowReq;

#define sz_xCompositeUnredirectWindowReq    12

typedef struct {
    CARD8   reqType;
    CARD8   compositeReqType;
    CARD16  length;
    Window  window;
    CARD8   update;
    CARD8   pad1;
    CARD16  pad2;
} xCompositeUnredirectSubwindowsReq;

#define sz_xCompositeUnredirectSubwindowsReq   12

typedef struct {
    CARD8   reqType;
    CARD8   compositeReqType;
    CARD16  length;
    Region  region;
    Window  window;
} xCompositeCreateRegionFromBorderClipReq;

#define sz_xCompositeCreateRegionFromBorderClipReq  12

/* Version 0.2 additions */

typedef struct {
    CARD8   reqType;
    CARD8   compositeReqType;
    CARD16  length;
    Window  window;
    Pixmap  pixmap;
} xCompositeNameWindowPixmapReq;

#define sz_xCompositeNameWindowPixmapReq	    12

/* Version 0.3 additions */

typedef struct {
    CARD8   reqType;
    CARD8   compositeReqType;
    CARD16  length;
    Window  window;
} xCompositeGetOverlayWindowReq;

#define sz_xCompositeGetOverlayWindowReq sizeof(xCompositeGetOverlayWindowReq)

typedef struct {
    BYTE    type;   /* X_Reply */
    BYTE    pad1;
    CARD16  sequenceNumber;
    CARD32  length;
    Window  overlayWin;
    CARD32  pad2;
    CARD32  pad3;
    CARD32  pad4;
    CARD32  pad5;
    CARD32  pad6;
} xCompositeGetOverlayWindowReply;

#define sz_xCompositeGetOverlayWindowReply sizeof(xCompositeGetOverlayWindowReply)

typedef struct {
    CARD8   reqType;
    CARD8   compositeReqType;
    CARD16  length;
    Window  window;
} xCompositeReleaseOverlayWindowReq;

#define sz_xCompositeReleaseOverlayWindowReq sizeof(xCompositeReleaseOverlayWindowReq)

#undef Window
#undef Region
#undef Pixmap

#endif /* _COMPOSITEPROTO_H_ */
