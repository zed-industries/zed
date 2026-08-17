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

/* RANDOM CRUFT! THIS HAS NO OFFICIAL X CONSORTIUM OR X PROJECT TEAM BLESSING */

#ifndef _MITMISCPROTO_H_
#define _MITMISCPROTO_H_

#include <X11/extensions/mitmiscconst.h>

#define X_MITSetBugMode			0
#define X_MITGetBugMode			1

typedef struct _SetBugMode {
    CARD8	reqType;	/* always MITReqCode */
    CARD8	mitReqType;	/* always X_MITSetBugMode */
    CARD16	length;
    BOOL	onOff;
    BYTE	pad0;
    CARD16	pad1;
} xMITSetBugModeReq;
#define sz_xMITSetBugModeReq	8

typedef struct _GetBugMode {
    CARD8	reqType;	/* always MITReqCode */
    CARD8	mitReqType;	/* always X_MITGetBugMode */
    CARD16	length;
} xMITGetBugModeReq;
#define sz_xMITGetBugModeReq	4

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	onOff;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad0;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xMITGetBugModeReply;
#define sz_xMITGetBugModeReply	32

#endif /* _MITMISCPROTO_H_ */
