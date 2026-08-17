/*

Copyright 1993, 1994, 1998  The Open Group

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

#ifndef _XCMISCPROTO_H_
#define _XCMISCPROTO_H_

#define X_XCMiscGetVersion	0
#define X_XCMiscGetXIDRange	1
#define X_XCMiscGetXIDList	2

#define XCMiscNumberEvents	0

#define XCMiscNumberErrors	0

#define XCMiscMajorVersion	1
#define XCMiscMinorVersion	1

#define XCMiscExtensionName	"XC-MISC"

typedef struct {
    CARD8	reqType;	/* always XCMiscCode */
    CARD8	miscReqType;	/* always X_XCMiscGetVersion */
    CARD16	length;
    CARD16	majorVersion;
    CARD16	minorVersion;
} xXCMiscGetVersionReq;
#define sz_xXCMiscGetVersionReq 8

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	pad0;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	majorVersion;
    CARD16	minorVersion;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xXCMiscGetVersionReply;
#define sz_xXCMiscGetVersionReply 32

typedef struct {
    CARD8	reqType;	/* always XCMiscCode */
    CARD8	miscReqType;	/* always X_XCMiscGetXIDRange */
    CARD16	length;
} xXCMiscGetXIDRangeReq;
#define sz_xXCMiscGetXIDRangeReq 4

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	pad0;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	start_id;
    CARD32	count;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
} xXCMiscGetXIDRangeReply;
#define sz_xXCMiscGetXIDRangeReply 32

typedef struct {
    CARD8	reqType;	/* always XCMiscCode */
    CARD8	miscReqType;	/* always X_XCMiscGetXIDList */
    CARD16	length;
    CARD32	count;		/* number of IDs requested */
} xXCMiscGetXIDListReq;
#define sz_xXCMiscGetXIDListReq 8

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	pad0;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	count;		/* number of IDs requested */
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xXCMiscGetXIDListReply;
#define sz_xXCMiscGetXIDListReply 32

#endif /* _XCMISCPROTO_H_ */
