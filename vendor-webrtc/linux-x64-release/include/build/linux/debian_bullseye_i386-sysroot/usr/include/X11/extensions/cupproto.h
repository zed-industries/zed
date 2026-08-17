/*

Copyright 1987, 1988, 1998  The Open Group

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

#ifndef _XCUPPROTO_H_ /* { */
#define _XCUPPROTO_H_

#include <X11/extensions/cup.h>

#define X_XcupQueryVersion			0
#define X_XcupGetReservedColormapEntries	1
#define X_XcupStoreColors			2

typedef struct _XcupQueryVersion {
    CARD8	reqType;	/* always XcupReqCode */
    CARD8	xcupReqType;	/* always X_XcupQueryVersion */
    CARD16	length;
    CARD16	client_major_version;
    CARD16	client_minor_version;
} xXcupQueryVersionReq;
#define sz_xXcupQueryVersionReq		8

typedef struct {
    BYTE	type;		/* X_Reply */
    BOOL	pad1;
    CARD16	sequence_number;
    CARD32	length;
    CARD16	server_major_version;
    CARD16	server_minor_version;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xXcupQueryVersionReply;
#define sz_xXcupQueryVersionReply	32

typedef struct _XcupGetReservedColormapEntries {
    CARD8	reqType;	/* always XcupReqCode */
    CARD8	xcupReqType;	/* always X_XcupGetReservedColormapEntries */
    CARD16	length;
    CARD32	screen;
} xXcupGetReservedColormapEntriesReq;
#define sz_xXcupGetReservedColormapEntriesReq 8

typedef struct {
    BYTE	type;		/* X_Reply */
    BOOL	pad1;
    CARD16	sequence_number;
    CARD32	length;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
    CARD32	pad7;
} xXcupGetReservedColormapEntriesReply;
#define sz_xXcupGetReservedColormapEntriesReply		32

typedef struct _XcupStoreColors {
    CARD8	reqType;	/* always XcupReqCode */
    CARD8	xcupReqType;	/* always X_XcupStoreColors */
    CARD16	length;
    CARD32	cmap;
} xXcupStoreColorsReq;
#define sz_xXcupStoreColorsReq		8

typedef struct {
    BYTE	type;		/* X_Reply */
    BOOL	pad1;
    CARD16	sequence_number;
    CARD32	length;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
    CARD32	pad7;
} xXcupStoreColorsReply;
#define sz_xXcupStoreColorsReply	32

#endif /* } _XCUPPROTO_H_ */

