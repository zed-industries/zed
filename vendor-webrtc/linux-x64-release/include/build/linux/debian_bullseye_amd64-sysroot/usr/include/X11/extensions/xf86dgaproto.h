/*

Copyright (c) 1995  Jon Tombs
Copyright (c) 1995  XFree86 Inc.

*/

#ifndef _XF86DGAPROTO_H_
#define _XF86DGAPROTO_H_

#include <X11/extensions/xf86dga1proto.h>
#include <X11/extensions/xf86dgaconst.h>

#define XF86DGANAME "XFree86-DGA"

#define XDGA_MAJOR_VERSION	2	/* current version numbers */
#define XDGA_MINOR_VERSION	0


typedef struct _XDGAQueryVersion {
    CARD8	reqType;		/* always DGAReqCode */
    CARD8	dgaReqType;		/* always X_DGAQueryVersion */
    CARD16	length;
} xXDGAQueryVersionReq;
#define sz_xXDGAQueryVersionReq		4

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	majorVersion;		/* major version of DGA protocol */
    CARD16	minorVersion;		/* minor version of DGA protocol */
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xXDGAQueryVersionReply;
#define sz_xXDGAQueryVersionReply	32

typedef struct _XDGAQueryModes {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
} xXDGAQueryModesReq;
#define sz_xXDGAQueryModesReq		8

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	number;			/* number of modes available */
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xXDGAQueryModesReply;
#define sz_xXDGAQueryModesReply	32


typedef struct _XDGASetMode {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
    CARD32	mode;			/* mode number to init */
    CARD32	pid;			/* Pixmap descriptor */
} xXDGASetModeReq;
#define sz_xXDGASetModeReq		16

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	offset;			/* offset into framebuffer map */
    CARD32	flags;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xXDGASetModeReply;
#define sz_xXDGASetModeReply	32

typedef struct {
   CARD8	byte_order;
   CARD8	depth;
   CARD16 	num;
   CARD16	bpp;
   CARD16	name_size;
   CARD32	vsync_num;
   CARD32	vsync_den;
   CARD32	flags;
   CARD16	image_width;
   CARD16	image_height;
   CARD16	pixmap_width;
   CARD16	pixmap_height;
   CARD32	bytes_per_scanline;
   CARD32	red_mask;
   CARD32	green_mask;
   CARD32	blue_mask;
   CARD16	visual_class;
   CARD16	pad1;
   CARD16	viewport_width;
   CARD16	viewport_height;
   CARD16	viewport_xstep;
   CARD16	viewport_ystep;
   CARD16	viewport_xmax;
   CARD16	viewport_ymax;
   CARD32	viewport_flags;
   CARD32	reserved1;
   CARD32	reserved2;
} xXDGAModeInfo;
#define sz_xXDGAModeInfo 72

typedef struct _XDGAOpenFramebuffer {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
} xXDGAOpenFramebufferReq;
#define sz_xXDGAOpenFramebufferReq	8

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;			/* device name size if there is one */
    CARD32	mem1;			/* physical memory */
    CARD32	mem2;			/* spillover for _alpha_ */
    CARD32	size;			/* size of map in bytes */
    CARD32	offset;			/* optional offset into device */
    CARD32	extra;			/* extra info associated with the map */
    CARD32	pad2;
} xXDGAOpenFramebufferReply;
#define sz_xXDGAOpenFramebufferReply	32


typedef struct _XDGACloseFramebuffer {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
} xXDGACloseFramebufferReq;
#define sz_xXDGACloseFramebufferReq	8


typedef struct _XDGASetViewport {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
    CARD16	x;
    CARD16	y;
    CARD32	flags;
} xXDGASetViewportReq;
#define sz_xXDGASetViewportReq	16


typedef struct _XDGAInstallColormap {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
    CARD32	cmap;
} xXDGAInstallColormapReq;
#define sz_xXDGAInstallColormapReq	12

typedef struct _XDGASelectInput {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
    CARD32	mask;
} xXDGASelectInputReq;
#define sz_xXDGASelectInputReq	12

typedef struct _XDGAFillRectangle {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
    CARD16	x;
    CARD16	y;
    CARD16	width;
    CARD16	height;
    CARD32	color;
} xXDGAFillRectangleReq;
#define sz_xXDGAFillRectangleReq	20


typedef struct _XDGACopyArea {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
    CARD16	srcx;
    CARD16	srcy;
    CARD16	width;
    CARD16	height;
    CARD16	dstx;
    CARD16	dsty;
} xXDGACopyAreaReq;
#define sz_xXDGACopyAreaReq	20

typedef struct _XDGACopyTransparentArea {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
    CARD16	srcx;
    CARD16	srcy;
    CARD16	width;
    CARD16	height;
    CARD16	dstx;
    CARD16	dsty;
    CARD32	key;
} xXDGACopyTransparentAreaReq;
#define sz_xXDGACopyTransparentAreaReq	24


typedef struct _XDGAGetViewportStatus {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
} xXDGAGetViewportStatusReq;
#define sz_xXDGAGetViewportStatusReq	8

typedef struct {
    BYTE	type;
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	status;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xXDGAGetViewportStatusReply;
#define sz_xXDGAGetViewportStatusReply	32

typedef struct _XDGASync {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
} xXDGASyncReq;
#define sz_xXDGASyncReq	8

typedef struct {
    BYTE	type;
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
    CARD32	pad7;
} xXDGASyncReply;
#define sz_xXDGASyncReply	32

typedef struct _XDGASetClientVersion {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD16	major;
    CARD16	minor;
} xXDGASetClientVersionReq;
#define sz_xXDGASetClientVersionReq	8


typedef struct {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
    CARD16	x;
    CARD16	y;
    CARD32	flags;
} xXDGAChangePixmapModeReq;
#define sz_xXDGAChangePixmapModeReq	16

typedef struct {
    BYTE	type;
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	x;
    CARD16	y;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
    CARD32	pad7;
} xXDGAChangePixmapModeReply;
#define sz_xXDGAChangePixmapModeReply	32

typedef struct _XDGACreateColormap {
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD32	screen;
    CARD32	id;
    CARD32	mode;
    CARD8	alloc;
    CARD8	pad1;
    CARD16	pad2;
} xXDGACreateColormapReq;
#define sz_xXDGACreateColormapReq	20


typedef struct {
  union {
    struct {
      BYTE type;
      BYTE detail;
      CARD16 sequenceNumber;
    } u;
    struct {
      CARD32 pad0;
      CARD32 time;
      INT16 dx;
      INT16 dy;
      INT16 screen;
      CARD16 state;
      CARD32 pad1;
      CARD32 pad2;
      CARD32 pad3;
      CARD32 pad4;
    } event;
  } u;
} dgaEvent;


#endif /* _XF86DGAPROTO_H_ */

