/***********************************************************
Copyright 1991 by Digital Equipment Corporation, Maynard, Massachusetts,
and the Massachusetts Institute of Technology, Cambridge, Massachusetts.

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the names of Digital or MIT not be
used in advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

DIGITAL DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING
ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL
DIGITAL BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR
ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
SOFTWARE.

******************************************************************/

#ifndef XVPROTO_H
#define XVPROTO_H
/*
** File:
**
**   Xvproto.h --- Xv protocol header file
**
** Author:
**
**   David Carver (Digital Workstation Engineering/Project Athena)
**
** Revisions:
**
**   11.06.91 Carver
**     - changed SetPortControl to SetPortAttribute
**     - changed GetPortControl to GetPortAttribute
**     - changed QueryBestSize
**
**   15.05.91 Carver
**     - version 2.0 upgrade
**
**   24.01.91 Carver
**     - version 1.4 upgrade
**
*/

#include <X11/Xmd.h>

/* Symbols: These are undefined at the end of this file to restore the
   values they have in Xv.h */

#define XvPortID CARD32
#define XvEncodingID CARD32
#define ShmSeg CARD32
#define VisualID CARD32
#define Drawable CARD32
#define GContext CARD32
#define Time CARD32
#define Atom CARD32

/* Structures */

typedef struct {
  INT32 numerator;
  INT32 denominator;
} xvRational;
#define sz_xvRational 8

typedef struct {
  XvPortID base_id;
  CARD16 name_size;
  CARD16 num_ports;
  CARD16 num_formats;
  CARD8 type;
  CARD8 pad;
} xvAdaptorInfo;
#define sz_xvAdaptorInfo 12

typedef struct {
  XvEncodingID encoding;
  CARD16 name_size;
  CARD16 width, height;
  CARD16 pad;
  xvRational rate;
} xvEncodingInfo;
#define sz_xvEncodingInfo (12 + sz_xvRational)

typedef struct {
  VisualID visual;
  CARD8 depth;
  CARD8 pad1;
  CARD16 pad2;
} xvFormat;
#define sz_xvFormat 8

typedef struct {
  CARD32 flags;
  INT32 min;
  INT32 max;
  CARD32 size;
} xvAttributeInfo;
#define sz_xvAttributeInfo 16

typedef struct {
  CARD32 id;
  CARD8 type;
  CARD8 byte_order;
  CARD16 pad1;
  CARD8 guid[16];
  CARD8 bpp;
  CARD8 num_planes;
  CARD16 pad2;
  CARD8 depth;
  CARD8 pad3;
  CARD16 pad4;
  CARD32 red_mask;
  CARD32 green_mask;
  CARD32 blue_mask;
  CARD8 format;
  CARD8 pad5;
  CARD16 pad6;
  CARD32 y_sample_bits;
  CARD32 u_sample_bits;
  CARD32 v_sample_bits;
  CARD32 horz_y_period;
  CARD32 horz_u_period;
  CARD32 horz_v_period;
  CARD32 vert_y_period;
  CARD32 vert_u_period;
  CARD32 vert_v_period;
  CARD8 comp_order[32];
  CARD8 scanline_order;
  CARD8 pad7;
  CARD16 pad8;
  CARD32 pad9;
  CARD32 pad10;
} xvImageFormatInfo;
#define sz_xvImageFormatInfo 128


/* Requests */

#define xv_QueryExtension                  0
#define	xv_QueryAdaptors                   1
#define	xv_QueryEncodings                  2
#define xv_GrabPort                        3
#define xv_UngrabPort                      4
#define xv_PutVideo                        5
#define xv_PutStill                        6
#define xv_GetVideo                        7
#define xv_GetStill                        8
#define xv_StopVideo                       9
#define xv_SelectVideoNotify              10
#define xv_SelectPortNotify               11
#define xv_QueryBestSize                  12
#define xv_SetPortAttribute               13
#define xv_GetPortAttribute               14
#define xv_QueryPortAttributes            15
#define xv_ListImageFormats               16
#define xv_QueryImageAttributes           17
#define xv_PutImage                       18
#define xv_ShmPutImage                    19
#define xv_LastRequest                    xv_ShmPutImage

#define xvNumRequests                     (xv_LastRequest + 1)

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
} xvQueryExtensionReq;
#define sz_xvQueryExtensionReq 4

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  CARD32 window;
} xvQueryAdaptorsReq;
#define sz_xvQueryAdaptorsReq 8

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  CARD32 port;
} xvQueryEncodingsReq;
#define sz_xvQueryEncodingsReq 8

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
  Drawable drawable;
  GContext gc;
  INT16 vid_x;
  INT16 vid_y;
  CARD16 vid_w;
  CARD16 vid_h;
  INT16 drw_x;
  INT16 drw_y;
  CARD16 drw_w;
  CARD16 drw_h;
} xvPutVideoReq;
#define sz_xvPutVideoReq 32

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
  Drawable drawable;
  GContext gc;
  INT16 vid_x;
  INT16 vid_y;
  CARD16 vid_w;
  CARD16 vid_h;
  INT16 drw_x;
  INT16 drw_y;
  CARD16 drw_w;
  CARD16 drw_h;
} xvPutStillReq;
#define sz_xvPutStillReq 32

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
  Drawable drawable;
  GContext gc;
  INT16 vid_x;
  INT16 vid_y;
  CARD16 vid_w;
  CARD16 vid_h;
  INT16 drw_x;
  INT16 drw_y;
  CARD16 drw_w;
  CARD16 drw_h;
} xvGetVideoReq;
#define sz_xvGetVideoReq 32

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
  Drawable drawable;
  GContext gc;
  INT16 vid_x;
  INT16 vid_y;
  CARD16 vid_w;
  CARD16 vid_h;
  INT16 drw_x;
  INT16 drw_y;
  CARD16 drw_w;
  CARD16 drw_h;
} xvGetStillReq;
#define sz_xvGetStillReq 32

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
  Time time;
} xvGrabPortReq;
#define sz_xvGrabPortReq 12

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
  Time time;
} xvUngrabPortReq;
#define sz_xvUngrabPortReq 12

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  Drawable drawable;
  BOOL onoff;
  CARD8 pad1;
  CARD16 pad2;
} xvSelectVideoNotifyReq;
#define sz_xvSelectVideoNotifyReq 12

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
  BOOL onoff;
  CARD8 pad1;
  CARD16 pad2;
} xvSelectPortNotifyReq;
#define sz_xvSelectPortNotifyReq 12

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
  Drawable drawable;
} xvStopVideoReq;
#define sz_xvStopVideoReq 12

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
  Atom attribute;
  INT32 value;
} xvSetPortAttributeReq;
#define sz_xvSetPortAttributeReq 16

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
  Atom attribute;
} xvGetPortAttributeReq;
#define sz_xvGetPortAttributeReq 12

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
  CARD16 vid_w;
  CARD16 vid_h;
  CARD16 drw_w;
  CARD16 drw_h;
  CARD8 motion;
  CARD8 pad1;
  CARD16 pad2;
} xvQueryBestSizeReq;
#define sz_xvQueryBestSizeReq 20

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
} xvQueryPortAttributesReq;
#define sz_xvQueryPortAttributesReq 8

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
  Drawable drawable;
  GContext gc;
  CARD32 id;
  INT16 src_x;
  INT16 src_y;
  CARD16 src_w;
  CARD16 src_h;
  INT16 drw_x;
  INT16 drw_y;
  CARD16 drw_w;
  CARD16 drw_h;
  CARD16 width;
  CARD16 height;
} xvPutImageReq;
#define sz_xvPutImageReq 40

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
  Drawable drawable;
  GContext gc;
  ShmSeg shmseg;
  CARD32 id;
  CARD32 offset;
  INT16 src_x;
  INT16 src_y;
  CARD16 src_w;
  CARD16 src_h;
  INT16 drw_x;
  INT16 drw_y;
  CARD16 drw_w;
  CARD16 drw_h;
  CARD16 width;
  CARD16 height;
  CARD8 send_event;
  CARD8 pad1;
  CARD16 pad2;
} xvShmPutImageReq;
#define sz_xvShmPutImageReq 52

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  XvPortID port;
} xvListImageFormatsReq;
#define sz_xvListImageFormatsReq 8

typedef struct {
  CARD8 reqType;
  CARD8 xvReqType;
  CARD16 length;
  CARD32 port;
  CARD32 id;
  CARD16 width;
  CARD16 height;
} xvQueryImageAttributesReq;
#define sz_xvQueryImageAttributesReq 16


/* Replies */

typedef struct _QueryExtensionReply {
  BYTE type;   /* X_Reply */
  CARD8 padb1;
  CARD16 sequenceNumber;
  CARD32 length;
  CARD16 version;
  CARD16 revision;
  CARD32 padl4;
  CARD32 padl5;
  CARD32 padl6;
  CARD32 padl7;
  CARD32 padl8;
} xvQueryExtensionReply;
#define sz_xvQueryExtensionReply 32

typedef struct _QueryAdaptorsReply {
  BYTE type;   /* X_Reply */
  CARD8 padb1;
  CARD16 sequenceNumber;
  CARD32 length;
  CARD16 num_adaptors;
  CARD16 pads3;
  CARD32 padl4;
  CARD32 padl5;
  CARD32 padl6;
  CARD32 padl7;
  CARD32 padl8;
} xvQueryAdaptorsReply;
#define sz_xvQueryAdaptorsReply 32

typedef struct _QueryEncodingsReply {
  BYTE type;   /* X_Reply */
  CARD8 padb1;
  CARD16 sequenceNumber;
  CARD32 length;
  CARD16 num_encodings;
  CARD16 padl3;
  CARD32 padl4;
  CARD32 padl5;
  CARD32 padl6;
  CARD32 padl7;
  CARD32 padl8;
} xvQueryEncodingsReply;
#define sz_xvQueryEncodingsReply 32

typedef struct {
  BYTE type;  /* X_Reply */
  BYTE result;
  CARD16 sequenceNumber;
  CARD32 length;  /* 0 */
  CARD32 padl3;
  CARD32 padl4;
  CARD32 padl5;
  CARD32 padl6;
  CARD32 padl7;
  CARD32 padl8;
} xvGrabPortReply;
#define sz_xvGrabPortReply 32

typedef struct {
  BYTE type;  /* X_Reply */
  BYTE padb1;
  CARD16 sequenceNumber;
  CARD32 length;  /* 0 */
  INT32 value;
  CARD32 padl4;
  CARD32 padl5;
  CARD32 padl6;
  CARD32 padl7;
  CARD32 padl8;
} xvGetPortAttributeReply;
#define sz_xvGetPortAttributeReply 32

typedef struct {
  BYTE type;  /* X_Reply */
  BYTE padb1;
  CARD16 sequenceNumber;
  CARD32 length;  /* 0 */
  CARD16 actual_width;
  CARD16 actual_height;
  CARD32 padl4;
  CARD32 padl5;
  CARD32 padl6;
  CARD32 padl7;
  CARD32 padl8;
} xvQueryBestSizeReply;
#define sz_xvQueryBestSizeReply 32

typedef struct {
  BYTE type;  /* X_Reply */
  BYTE padb1;
  CARD16 sequenceNumber;
  CARD32 length;  /* 0 */
  CARD32 num_attributes;
  CARD32 text_size;
  CARD32 padl5;
  CARD32 padl6;
  CARD32 padl7;
  CARD32 padl8;
} xvQueryPortAttributesReply;
#define sz_xvQueryPortAttributesReply 32

typedef struct {
  BYTE type;  /* X_Reply */
  BYTE padb1;
  CARD16 sequenceNumber;
  CARD32 length;
  CARD32 num_formats;
  CARD32 padl4;
  CARD32 padl5;
  CARD32 padl6;
  CARD32 padl7;
  CARD32 padl8;
} xvListImageFormatsReply;
#define sz_xvListImageFormatsReply 32

typedef struct {
  BYTE type;  /* X_Reply */
  BYTE padb1;
  CARD16 sequenceNumber;
  CARD32 length;
  CARD32 num_planes;
  CARD32 data_size;
  CARD16 width;
  CARD16 height;
  CARD32 padl6;
  CARD32 padl7;
  CARD32 padl8;
} xvQueryImageAttributesReply;
#define sz_xvQueryImageAttributesReply 32

/* DEFINE EVENT STRUCTURE */

typedef struct {
  union {
    struct {
      BYTE type;
      BYTE detail;
      CARD16 sequenceNumber;
    } u;
    struct {
      BYTE type;
      BYTE reason;
      CARD16 sequenceNumber;
      Time time;
      Drawable drawable;
      XvPortID port;
      CARD32 padl5;
      CARD32 padl6;
      CARD32 padl7;
      CARD32 padl8;
    } videoNotify;
    struct {
      BYTE type;
      BYTE padb1;
      CARD16 sequenceNumber;
      Time time;
      XvPortID port;
      Atom attribute;
      INT32 value;
      CARD32 padl6;
      CARD32 padl7;
      CARD32 padl8;
    } portNotify;
  } u;
} xvEvent;

#undef XvPortID
#undef XvEncodingID
#undef ShmSeg
#undef VisualID
#undef Drawable
#undef GContext
#undef Time
#undef Atom

#endif /* XVPROTO_H */

