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

#ifndef _PRESENT_PROTO_H_
#define _PRESENT_PROTO_H_

#include <X11/extensions/presenttokens.h>

#define Window CARD32
#define Pixmap CARD32
#define Region CARD32
#define XSyncFence CARD32
#define EventID CARD32

typedef struct {
    Window  window;
    CARD32  serial;
} xPresentNotify;
#define sz_xPresentNotify               8

typedef struct {
    CARD8   reqType;
    CARD8   presentReqType;
    CARD16  length;
    CARD32  majorVersion;
    CARD32  minorVersion;
} xPresentQueryVersionReq;
#define sz_xPresentQueryVersionReq   12

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
} xPresentQueryVersionReply;
#define sz_xPresentQueryVersionReply	32

typedef struct {
    CARD8   reqType;
    CARD8   presentReqType;
    CARD16  length;
    Window  window;

    Pixmap  pixmap;
    CARD32  serial;

    Region  valid;
    Region  update;

    INT16   x_off;
    INT16   y_off;
    CARD32  target_crtc;

    XSyncFence wait_fence;
    XSyncFence idle_fence;

    CARD32  options;
    CARD32  pad1;

    CARD64  target_msc;
    CARD64  divisor;
    CARD64  remainder;
    /* followed by a LISTofPRESENTNOTIFY */
} xPresentPixmapReq;
#define sz_xPresentPixmapReq	72

typedef struct {
    CARD8   reqType;
    CARD8   presentReqType;
    CARD16  length;
    Window  window;

    CARD32  serial;
    CARD32  pad0;

    CARD64  target_msc;
    CARD64  divisor;
    CARD64  remainder;
} xPresentNotifyMSCReq;
#define sz_xPresentNotifyMSCReq	40

typedef struct {
    CARD8   reqType;
    CARD8   presentReqType;
    CARD16  length;
    CARD32  eid;
    CARD32  window;
    CARD32  eventMask;
} xPresentSelectInputReq;
#define sz_xPresentSelectInputReq   16

typedef struct {
    CARD8   reqType;
    CARD8   presentReqType;
    CARD16  length;
    CARD32  target;
} xPresentQueryCapabilitiesReq;
#define sz_xPresentQueryCapabilitiesReq   8

typedef struct {
    BYTE    type;   /* X_Reply */
    BYTE    pad1;
    CARD16  sequenceNumber;
    CARD32  length;
    CARD32  capabilities;
    CARD32  pad3;
    CARD32  pad4;
    CARD32  pad5;
    CARD32  pad6;
    CARD32  pad7;
} xPresentQueryCapabilitiesReply;
#define sz_xPresentQueryCapabilitiesReply       32

/*
 * Events
 *
 * All Present events are X Generic Events
 */

typedef struct {
    CARD8 type;
    CARD8 extension;
    CARD16 sequenceNumber;
    CARD32 length;
    CARD16 evtype;
    CARD16 pad2;
    CARD32 eid;
    CARD32 window;
    INT16  x;
    INT16  y;
    CARD16 width;
    CARD16 height;
    INT16  off_x;
    INT16  off_y;

    CARD16 pixmap_width;
    CARD16 pixmap_height;
    CARD32 pixmap_flags;
} xPresentConfigureNotify;
#define sz_xPresentConfigureNotify 40

typedef struct {
    CARD8 type;
    CARD8 extension;
    CARD16 sequenceNumber;
    CARD32 length;
    CARD16 evtype;
    CARD8  kind;
    CARD8  mode;
    CARD32 eid;
    Window window;
    CARD32 serial;
    CARD64 ust;

    CARD64 msc;
} xPresentCompleteNotify;
#define sz_xPresentCompleteNotify 40

typedef struct {
    CARD8 type;
    CARD8 extension;
    CARD16 sequenceNumber;
    CARD32 length;
    CARD16 evtype;
    CARD16 pad2;
    CARD32 eid;
    Window window;
    CARD32 serial;
    Pixmap pixmap;
    CARD32 idle_fence;
} xPresentIdleNotify;
#define sz_xPresentIdleNotify   32

#if PRESENT_FUTURE_VERSION
typedef struct {
    CARD8 type;
    CARD8 extension;
    CARD16 sequenceNumber;
    CARD32 length;
    CARD16 evtype;
    CARD8 update_window;
    CARD8 pad1;
    CARD32 eid;
    Window event_window;
    Window window;
    Pixmap pixmap;
    CARD32 serial;

    /* 32-byte boundary */

    Region valid_region;
    Region update_region;

    xRectangle valid_rect;

    xRectangle update_rect;

    INT16 x_off;
    INT16 y_off;
    CARD32 target_crtc;

    XSyncFence wait_fence;
    XSyncFence idle_fence;

    CARD32 options;
    CARD32 pad2;

    CARD64 target_msc;
    CARD64 divisor;
    CARD64 remainder;

} xPresentRedirectNotify;

#define sz_xPresentRedirectNotify 104
#endif

#undef Window
#undef Pixmap
#undef Region
#undef XSyncFence
#undef EventID

#endif
