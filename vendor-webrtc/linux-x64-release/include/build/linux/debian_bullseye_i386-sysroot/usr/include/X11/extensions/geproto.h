/*
 * Copyright © 2007-2008 Peter Hutterer
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
 *
 * Authors: Peter Hutterer, University of South Australia, NICTA
 *
 */

#ifndef _GEPROTO_H_
#define _GEPROTO_H_

#include<X11/Xproto.h>
#include<X11/X.h>
#include<X11/extensions/ge.h>


/*********************************************************
 *
 * Protocol request constants
 *
 */

#define X_GEGetExtensionVersion 1

/*********************************************************
 *
 * XGE protocol requests/replies
 *
 */

/* generic request */
typedef struct {
    CARD8   reqType;
    CARD8   ReqType;
    CARD16  length;
} xGEReq;


/* QueryVersion */
typedef struct {
    CARD8	reqType;       /* input extension major code   */
    CARD8	ReqType;       /* always X_GEQueryVersion */
    CARD16	length;
    CARD16	majorVersion;
    CARD16	minorVersion;
} xGEQueryVersionReq;

#define sz_xGEQueryVersionReq    8

typedef struct {
    CARD8	repType;	/* X_Reply			*/
    CARD8	RepType;	/* always X_GEQueryVersion */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	majorVersion;
    CARD16	minorVersion;
    CARD32	pad00;
    CARD32	pad01;
    CARD32	pad02;
    CARD32	pad03;
    CARD32	pad04;
} xGEQueryVersionReply;

#define sz_xGEQueryVersionReply    32

#endif /* _GEPROTO_H_ */

