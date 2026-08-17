/*
 * Copyright © 2013 Keith Packard
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that copyright
 * notice and this permission notice appear in supporting documentation, and
 * that the name of the copyright holders not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  The copyright holders make no representations
 * about the suitability of this software for any purpose.  It is provided "as
 * is" without express or implied warranty.
 *
 * THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

#ifndef _DRI3_PROTO_H_
#define _DRI3_PROTO_H_

#define DRI3_NAME			"DRI3"
#define DRI3_MAJOR			1
#define DRI3_MINOR			2

#define DRI3NumberErrors		0
#define DRI3NumberEvents		0

#define X_DRI3QueryVersion		0
#define X_DRI3Open			1
#define X_DRI3PixmapFromBuffer          2
#define X_DRI3BufferFromPixmap          3
#define X_DRI3FenceFromFD               4
#define X_DRI3FDFromFence               5

/* v1.2 */
#define xDRI3GetSupportedModifiers      6
#define xDRI3PixmapFromBuffers          7
#define xDRI3BuffersFromPixmap          8

#define DRI3NumberRequests		9

typedef struct {
    CARD8   reqType;
    CARD8   dri3ReqType;
    CARD16  length;
    CARD32  majorVersion;
    CARD32  minorVersion;
} xDRI3QueryVersionReq;
#define sz_xDRI3QueryVersionReq   12

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
} xDRI3QueryVersionReply;
#define sz_xDRI3QueryVersionReply	32

typedef struct {
    CARD8   reqType;
    CARD8   dri3ReqType;
    CARD16  length;
    CARD32  drawable;
    CARD32  provider;
} xDRI3OpenReq;
#define sz_xDRI3OpenReq	12

typedef struct {
    BYTE    type;   /* X_Reply */
    CARD8   nfd;
    CARD16  sequenceNumber;
    CARD32  length;
    CARD32  pad2;
    CARD32  pad3;
    CARD32  pad4;
    CARD32  pad5;
    CARD32  pad6;
    CARD32  pad7;
} xDRI3OpenReply;
#define sz_xDRI3OpenReply	32

typedef struct {
    CARD8   reqType;
    CARD8   dri3ReqType;
    CARD16  length;
    CARD32  pixmap;
    CARD32  drawable;
    CARD32  size;
    CARD16  width;
    CARD16  height;
    CARD16  stride;
    CARD8   depth;
    CARD8   bpp;
} xDRI3PixmapFromBufferReq;

#define sz_xDRI3PixmapFromBufferReq     24

typedef struct {
    CARD8   reqType;
    CARD8   dri3ReqType;
    CARD16  length;
    CARD32  pixmap;
} xDRI3BufferFromPixmapReq;
#define sz_xDRI3BufferFromPixmapReq     8

typedef struct {
    BYTE    type;   /* X_Reply */
    CARD8   nfd;    /* Number of file descriptors returned (1) */
    CARD16  sequenceNumber;
    CARD32  length;
    CARD32  size;
    CARD16  width;
    CARD16  height;
    CARD16  stride;
    CARD8   depth;
    CARD8   bpp;
    CARD32  pad20;
    CARD32  pad24;
    CARD32  pad28;
} xDRI3BufferFromPixmapReply;
#define sz_xDRI3BufferFromPixmapReply   32

typedef struct {
    CARD8   reqType;
    CARD8   dri3ReqType;
    CARD16  length;
    CARD32  drawable;
    CARD32  fence;
    BOOL    initially_triggered;
    CARD8   pad13;
    CARD16  pad14;
} xDRI3FenceFromFDReq;

#define sz_xDRI3FenceFromFDReq  16

typedef struct {
    CARD8   reqType;
    CARD8   dri3ReqType;
    CARD16  length;
    CARD32  drawable;
    CARD32  fence;
} xDRI3FDFromFenceReq;

#define sz_xDRI3FDFromFenceReq  12

typedef struct {
    BYTE    type;   /* X_Reply */
    CARD8   nfd;    /* Number of file descriptors returned (1) */
    CARD16  sequenceNumber;
    CARD32  length;
    CARD32  pad08;
    CARD32  pad12;
    CARD32  pad16;
    CARD32  pad20;
    CARD32  pad24;
    CARD32  pad28;
} xDRI3FDFromFenceReply;

#define sz_xDRI3FDFromFenceReply   32

/* v1.2 */

typedef struct {
    CARD8   reqType;
    CARD8   dri3ReqType;
    CARD16  length;
    CARD32  window;
    CARD8   depth;
    CARD8   bpp;
    CARD16  pad10;
} xDRI3GetSupportedModifiersReq;
#define sz_xDRI3GetSupportedModifiersReq     12

typedef struct {
    BYTE    type;   /* X_Reply */
    CARD8   pad1;
    CARD16  sequenceNumber;
    CARD32  length;
    CARD32  numWindowModifiers;
    CARD32  numScreenModifiers;
    CARD32  pad16;
    CARD32  pad20;
    CARD32  pad24;
    CARD32  pad28;
} xDRI3GetSupportedModifiersReply;
#define sz_xDRI3GetSupportedModifiersReply   32

typedef struct {
    CARD8   reqType;
    CARD8   dri3ReqType;
    CARD16  length;
    CARD32  pixmap;
    CARD32  window;
    CARD8   num_buffers; /* Number of file descriptors passed */
    CARD8   pad13;
    CARD16  pad14;
    CARD16  width;
    CARD16  height;
    CARD32  stride0;
    CARD32  offset0;
    CARD32  stride1;
    CARD32  offset1;
    CARD32  stride2;
    CARD32  offset2;
    CARD32  stride3;
    CARD32  offset3;
    CARD8   depth;
    CARD8   bpp;
    CARD16  pad54;
    CARD64  modifier;
} xDRI3PixmapFromBuffersReq;
#define sz_xDRI3PixmapFromBuffersReq 64

typedef struct {
    CARD8   reqType;
    CARD8   dri3ReqType;
    CARD16  length;
    CARD32  pixmap;
} xDRI3BuffersFromPixmapReq;
#define sz_xDRI3BuffersFromPixmapReq     8

typedef struct {
    BYTE    type;   /* X_Reply */
    CARD8   nfd;    /* Number of file descriptors returned */
    CARD16  sequenceNumber;
    CARD32  length;
    CARD16  width;
    CARD16  height;
    CARD32  pad12;
    CARD64  modifier;
    CARD8   depth;
    CARD8   bpp;
    CARD16  pad26;
    CARD32  pad28;
} xDRI3BuffersFromPixmapReply;
#define sz_xDRI3BuffersFromPixmapReply   32

#endif
