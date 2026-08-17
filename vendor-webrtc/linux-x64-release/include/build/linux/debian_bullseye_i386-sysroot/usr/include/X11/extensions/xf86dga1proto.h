/*

Copyright (c) 1995  Jon Tombs
Copyright (c) 1995  XFree86 Inc.

*/

#ifndef _XF86DGAPROTO1_H_
#define _XF86DGAPROTO1_H_

#include <X11/extensions/xf86dga1const.h>

typedef struct _XF86DGAQueryVersion {
    CARD8	reqType;		/* always DGAReqCode */
    CARD8	dgaReqType;		/* always X_DGAQueryVersion */
    CARD16	length;
} xXF86DGAQueryVersionReq;
#define sz_xXF86DGAQueryVersionReq	4

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
} xXF86DGAQueryVersionReply;
#define sz_xXF86DGAQueryVersionReply	32

typedef struct _XF86DGAGetVideoLL {
    CARD8	reqType;		/* always DGAReqCode */
    CARD8	dgaReqType;		/* always X_XF86DGAGetVideoLL */
    CARD16	length;
    CARD16	screen;
    CARD16	pad;
} xXF86DGAGetVideoLLReq;
#define sz_xXF86DGAGetVideoLLReq	8

typedef struct _XF86DGAInstallColormap{
    CARD8	reqType;
    CARD8	dgaReqType;
    CARD16	length;
    CARD16	screen;
    CARD16	pad2;
    CARD32	id;  /* colormap. */
} xXF86DGAInstallColormapReq;
#define sz_xXF86DGAInstallColormapReq        12


typedef struct {
    BYTE	type;
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	offset;
    CARD32	width;
    CARD32	bank_size;
    CARD32	ram_size;
    CARD32	pad4;
    CARD32	pad5;
} xXF86DGAGetVideoLLReply;
#define sz_xXF86DGAGetVideoLLReply	32

typedef struct _XF86DGADirectVideo {
    CARD8	reqType;		/* always DGAReqCode */
    CARD8	dgaReqType;		/* always X_XF86DGADirectVideo */
    CARD16	length;
    CARD16	screen;
    CARD16	enable;
} xXF86DGADirectVideoReq;
#define sz_xXF86DGADirectVideoReq	8


typedef struct _XF86DGAGetViewPortSize {
    CARD8	reqType;		/* always DGAReqCode */
    CARD8	dgaReqType;		/* always X_XF86DGAGetViewPort */
    CARD16	length;
    CARD16	screen;
    CARD16	pad;
} xXF86DGAGetViewPortSizeReq;
#define sz_xXF86DGAGetViewPortSizeReq	8

typedef struct {
    BYTE	type;
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	width;
    CARD32	height;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xXF86DGAGetViewPortSizeReply;
#define sz_xXF86DGAGetViewPortSizeReply	32

typedef struct _XF86DGASetViewPort {
    CARD8	reqType;		/* always DGAReqCode */
    CARD8	dgaReqType;		/* always X_XF86DGASetViewPort */
    CARD16	length;
    CARD16	screen;
    CARD16	pad;
    CARD32	x;
    CARD32	y;
} xXF86DGASetViewPortReq;
#define sz_xXF86DGASetViewPortReq	16

typedef struct _XF86DGAGetVidPage {
    CARD8	reqType;		/* always DGAReqCode */
    CARD8	dgaReqType;		/* always X_XF86DGAGetVidPage */
    CARD16	length;
    CARD16	screen;
    CARD16	pad;
} xXF86DGAGetVidPageReq;
#define sz_xXF86DGAGetVidPageReq	8

typedef struct {
    BYTE	type;
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	vpage;
    CARD32	pad;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xXF86DGAGetVidPageReply;
#define sz_xXF86DGAGetVidPageReply	32


typedef struct _XF86DGASetVidPage {
    CARD8	reqType;		/* always DGAReqCode */
    CARD8	dgaReqType;		/* always X_XF86DGASetVidPage */
    CARD16	length;
    CARD16	screen;
    CARD16	vpage;
} xXF86DGASetVidPageReq;
#define sz_xXF86DGASetVidPageReq	8


typedef struct _XF86DGAQueryDirectVideo {
    CARD8	reqType;		/* always DGAReqCode */
    CARD8	dgaReqType;		/* always X_DGAQueryVersion */
    CARD16	length;
    CARD16	screen;
    CARD16	pad;
} xXF86DGAQueryDirectVideoReq;
#define sz_xXF86DGAQueryDirectVideoReq	8

typedef struct {
    BYTE	type;
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	flags;
    CARD32	pad;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xXF86DGAQueryDirectVideoReply;
#define sz_xXF86DGAQueryDirectVideoReply 32


typedef struct _XF86DGAViewPortChanged {
    CARD8	reqType;		/* always DGAReqCode */
    CARD8	dgaReqType;		/* always X_DGAQueryVersion */
    CARD16	length;
    CARD16	screen;
    CARD16	n;
} xXF86DGAViewPortChangedReq;
#define sz_xXF86DGAViewPortChangedReq	8

typedef struct {
    BYTE	type;
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	result;
    CARD32	pad;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xXF86DGAViewPortChangedReply;
#define sz_xXF86DGAViewPortChangedReply 32

#endif /* _XF86DGAPROTO1_H_ */

