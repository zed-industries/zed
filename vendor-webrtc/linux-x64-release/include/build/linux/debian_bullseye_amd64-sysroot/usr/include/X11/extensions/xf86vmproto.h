/*

Copyright 1995  Kaleb S. KEITHLEY

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL Kaleb S. KEITHLEY BE LIABLE FOR ANY CLAIM, DAMAGES
OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of Kaleb S. KEITHLEY
shall not be used in advertising or otherwise to promote the sale, use
or other dealings in this Software without prior written authorization
from Kaleb S. KEITHLEY

*/

/* THIS IS NOT AN X CONSORTIUM STANDARD OR AN X PROJECT TEAM SPECIFICATION */

#ifndef _XF86VIDMODEPROTO_H_
#define _XF86VIDMODEPROTO_H_

#include <X11/extensions/xf86vm.h>

#define XF86VIDMODENAME "XFree86-VidModeExtension"

#define XF86VIDMODE_MAJOR_VERSION	2	/* current version numbers */
#define XF86VIDMODE_MINOR_VERSION	2

#define X_XF86VidModeQueryVersion	0
#define X_XF86VidModeGetModeLine	1
#define X_XF86VidModeModModeLine	2
#define X_XF86VidModeSwitchMode		3
#define X_XF86VidModeGetMonitor		4
#define X_XF86VidModeLockModeSwitch	5
#define X_XF86VidModeGetAllModeLines	6
#define X_XF86VidModeAddModeLine	7
#define X_XF86VidModeDeleteModeLine	8
#define X_XF86VidModeValidateModeLine	9
#define X_XF86VidModeSwitchToMode	10
#define X_XF86VidModeGetViewPort	11
#define X_XF86VidModeSetViewPort	12
/* new for version 2.x of this extension */
#define X_XF86VidModeGetDotClocks	13
#define X_XF86VidModeSetClientVersion	14
#define X_XF86VidModeSetGamma		15
#define X_XF86VidModeGetGamma		16
#define X_XF86VidModeGetGammaRamp	17
#define X_XF86VidModeSetGammaRamp	18
#define X_XF86VidModeGetGammaRampSize	19
#define X_XF86VidModeGetPermissions	20
/*
 * major version 0 == uses parameter-to-wire functions in XFree86 libXxf86vm.
 * major version 1 == uses parameter-to-wire functions hard-coded in xvidtune
 *                    client.
 * major version 2 == uses new protocol version in XFree86 4.0.
 */

typedef struct _XF86VidModeQueryVersion {
    CARD8	reqType;		/* always XF86VidModeReqCode */
    CARD8	xf86vidmodeReqType;	/* always X_XF86VidModeQueryVersion */
    CARD16	length;
} xXF86VidModeQueryVersionReq;
#define sz_xXF86VidModeQueryVersionReq	4

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	majorVersion;		/* major version of XF86VidMode */
    CARD16	minorVersion;		/* minor version of XF86VidMode */
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xXF86VidModeQueryVersionReply;
#define sz_xXF86VidModeQueryVersionReply	32

typedef struct _XF86VidModeGetModeLine {
    CARD8	reqType;		/* always XF86VidModeReqCode */
    CARD8	xf86vidmodeReqType;
    CARD16	length;
    CARD16	screen;
    CARD16	pad;
} xXF86VidModeGetModeLineReq,
  xXF86VidModeGetAllModeLinesReq,
  xXF86VidModeGetMonitorReq,
  xXF86VidModeGetViewPortReq,
  xXF86VidModeGetDotClocksReq,
  xXF86VidModeGetPermissionsReq;
#define sz_xXF86VidModeGetModeLineReq		8
#define sz_xXF86VidModeGetAllModeLinesReq	8
#define sz_xXF86VidModeGetMonitorReq		8
#define sz_xXF86VidModeGetViewPortReq		8
#define sz_xXF86VidModeGetDotClocksReq		8
#define sz_xXF86VidModeGetPermissionsReq	8

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	dotclock;
    CARD16	hdisplay;
    CARD16	hsyncstart;
    CARD16	hsyncend;
    CARD16	htotal;
    CARD16	hskew;
    CARD16	vdisplay;
    CARD16	vsyncstart;
    CARD16	vsyncend;
    CARD16	vtotal;
    CARD16	pad2;
    CARD32	flags;
    CARD32	reserved1;
    CARD32	reserved2;
    CARD32	reserved3;
    CARD32	privsize;
} xXF86VidModeGetModeLineReply;
#define sz_xXF86VidModeGetModeLineReply	52

/* 0.x version */
typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	dotclock;
    CARD16	hdisplay;
    CARD16	hsyncstart;
    CARD16	hsyncend;
    CARD16	htotal;
    CARD16	vdisplay;
    CARD16	vsyncstart;
    CARD16	vsyncend;
    CARD16	vtotal;
    CARD32	flags;
    CARD32	privsize;
} xXF86OldVidModeGetModeLineReply;
#define sz_xXF86OldVidModeGetModeLineReply	36

typedef struct {
    CARD32	dotclock;
    CARD16	hdisplay;
    CARD16	hsyncstart;
    CARD16	hsyncend;
    CARD16	htotal;
    CARD32	hskew;
    CARD16	vdisplay;
    CARD16	vsyncstart;
    CARD16	vsyncend;
    CARD16	vtotal;
    CARD16	pad1;
    CARD32	flags;
    CARD32	reserved1;
    CARD32	reserved2;
    CARD32	reserved3;
    CARD32	privsize;
} xXF86VidModeModeInfo;

/* 0.x version */
typedef struct {
    CARD32	dotclock;
    CARD16	hdisplay;
    CARD16	hsyncstart;
    CARD16	hsyncend;
    CARD16	htotal;
    CARD16	vdisplay;
    CARD16	vsyncstart;
    CARD16	vsyncend;
    CARD16	vtotal;
    CARD32	flags;
    CARD32	privsize;
} xXF86OldVidModeModeInfo;

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	modecount;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xXF86VidModeGetAllModeLinesReply;
#define sz_xXF86VidModeGetAllModeLinesReply	32

typedef struct _XF86VidModeAddModeLine {
    CARD8	reqType;		/* always XF86VidModeReqCode */
    CARD8	xf86vidmodeReqType;	/* always X_XF86VidModeAddMode */
    CARD16	length;
    CARD32	screen;			/* could be CARD16 but need the pad */
    CARD32	dotclock;
    CARD16	hdisplay;
    CARD16	hsyncstart;
    CARD16	hsyncend;
    CARD16	htotal;
    CARD16	hskew;
    CARD16	vdisplay;
    CARD16	vsyncstart;
    CARD16	vsyncend;
    CARD16	vtotal;
    CARD16	pad1;
    CARD32	flags;
    CARD32	reserved1;
    CARD32	reserved2;
    CARD32	reserved3;
    CARD32	privsize;
    CARD32	after_dotclock;
    CARD16	after_hdisplay;
    CARD16	after_hsyncstart;
    CARD16	after_hsyncend;
    CARD16	after_htotal;
    CARD16	after_hskew;
    CARD16	after_vdisplay;
    CARD16	after_vsyncstart;
    CARD16	after_vsyncend;
    CARD16	after_vtotal;
    CARD16	pad2;
    CARD32	after_flags;
    CARD32	reserved4;
    CARD32	reserved5;
    CARD32	reserved6;
} xXF86VidModeAddModeLineReq;
#define sz_xXF86VidModeAddModeLineReq	92

/* 0.x version */
typedef struct _XF86OldVidModeAddModeLine {
    CARD8	reqType;		/* always XF86VidModeReqCode */
    CARD8	xf86vidmodeReqType;	/* always X_XF86VidModeAddMode */
    CARD16	length;
    CARD32	screen;			/* could be CARD16 but need the pad */
    CARD32	dotclock;
    CARD16	hdisplay;
    CARD16	hsyncstart;
    CARD16	hsyncend;
    CARD16	htotal;
    CARD16	vdisplay;
    CARD16	vsyncstart;
    CARD16	vsyncend;
    CARD16	vtotal;
    CARD32	flags;
    CARD32	privsize;
    CARD32	after_dotclock;
    CARD16	after_hdisplay;
    CARD16	after_hsyncstart;
    CARD16	after_hsyncend;
    CARD16	after_htotal;
    CARD16	after_vdisplay;
    CARD16	after_vsyncstart;
    CARD16	after_vsyncend;
    CARD16	after_vtotal;
    CARD32	after_flags;
} xXF86OldVidModeAddModeLineReq;
#define sz_xXF86OldVidModeAddModeLineReq	60

typedef struct _XF86VidModeModModeLine {
    CARD8	reqType;		/* always XF86VidModeReqCode */
    CARD8	xf86vidmodeReqType;	/* always X_XF86VidModeModModeLine */
    CARD16	length;
    CARD32	screen;			/* could be CARD16 but need the pad */
    CARD16	hdisplay;
    CARD16	hsyncstart;
    CARD16	hsyncend;
    CARD16	htotal;
    CARD16	hskew;
    CARD16	vdisplay;
    CARD16	vsyncstart;
    CARD16	vsyncend;
    CARD16	vtotal;
    CARD16	pad1;
    CARD32	flags;
    CARD32	reserved1;
    CARD32	reserved2;
    CARD32	reserved3;
    CARD32	privsize;
} xXF86VidModeModModeLineReq;
#define sz_xXF86VidModeModModeLineReq	48

/* 0.x version */
typedef struct _XF86OldVidModeModModeLine {
    CARD8	reqType;		/* always XF86OldVidModeReqCode */
    CARD8	xf86vidmodeReqType;	/* always X_XF86OldVidModeModModeLine */
    CARD16	length;
    CARD32	screen;			/* could be CARD16 but need the pad */
    CARD16	hdisplay;
    CARD16	hsyncstart;
    CARD16	hsyncend;
    CARD16	htotal;
    CARD16	vdisplay;
    CARD16	vsyncstart;
    CARD16	vsyncend;
    CARD16	vtotal;
    CARD32	flags;
    CARD32	privsize;
} xXF86OldVidModeModModeLineReq;
#define sz_xXF86OldVidModeModModeLineReq	32

typedef struct _XF86VidModeValidateModeLine {
    CARD8	reqType;		/* always XF86VidModeReqCode */
    CARD8	xf86vidmodeReqType;
    CARD16	length;
    CARD32	screen;			/* could be CARD16 but need the pad */
    CARD32	dotclock;
    CARD16	hdisplay;
    CARD16	hsyncstart;
    CARD16	hsyncend;
    CARD16	htotal;
    CARD16	hskew;
    CARD16	vdisplay;
    CARD16	vsyncstart;
    CARD16	vsyncend;
    CARD16	vtotal;
    CARD16	pad1;
    CARD32	flags;
    CARD32	reserved1;
    CARD32	reserved2;
    CARD32	reserved3;
    CARD32	privsize;
} xXF86VidModeDeleteModeLineReq,
  xXF86VidModeValidateModeLineReq,
  xXF86VidModeSwitchToModeReq;
#define sz_xXF86VidModeDeleteModeLineReq	52
#define sz_xXF86VidModeValidateModeLineReq	52
#define sz_xXF86VidModeSwitchToModeReq		52

/* 0.x version */
typedef struct _XF86OldVidModeValidateModeLine {
    CARD8	reqType;		/* always XF86OldVidModeReqCode */
    CARD8	xf86vidmodeReqType;
    CARD16	length;
    CARD32	screen;			/* could be CARD16 but need the pad */
    CARD32	dotclock;
    CARD16	hdisplay;
    CARD16	hsyncstart;
    CARD16	hsyncend;
    CARD16	htotal;
    CARD16	vdisplay;
    CARD16	vsyncstart;
    CARD16	vsyncend;
    CARD16	vtotal;
    CARD32	flags;
    CARD32	privsize;
} xXF86OldVidModeDeleteModeLineReq,
  xXF86OldVidModeValidateModeLineReq,
  xXF86OldVidModeSwitchToModeReq;
#define sz_xXF86OldVidModeDeleteModeLineReq	36
#define sz_xXF86OldVidModeValidateModeLineReq	36
#define sz_xXF86OldVidModeSwitchToModeReq	36

typedef struct _XF86VidModeSwitchMode {
    CARD8	reqType;		/* always XF86VidModeReqCode */
    CARD8	xf86vidmodeReqType;	/* always X_XF86VidModeSwitchMode */
    CARD16	length;
    CARD16	screen;
    CARD16	zoom;
} xXF86VidModeSwitchModeReq;
#define sz_xXF86VidModeSwitchModeReq	8

typedef struct _XF86VidModeLockModeSwitch {
    CARD8	reqType;		/* always XF86VidModeReqCode */
    CARD8	xf86vidmodeReqType;	/* always X_XF86VidModeLockModeSwitch */
    CARD16	length;
    CARD16	screen;
    CARD16	lock;
} xXF86VidModeLockModeSwitchReq;
#define sz_xXF86VidModeLockModeSwitchReq	8

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	status;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xXF86VidModeValidateModeLineReply;
#define sz_xXF86VidModeValidateModeLineReply	32

typedef struct {
    BYTE	type;			/* X_Reply */
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD8	vendorLength;
    CARD8	modelLength;
    CARD8	nhsync;
    CARD8	nvsync;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xXF86VidModeGetMonitorReply;
#define sz_xXF86VidModeGetMonitorReply	32

typedef struct {
    BYTE	type;
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	x;
    CARD32	y;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xXF86VidModeGetViewPortReply;
#define sz_xXF86VidModeGetViewPortReply	32

typedef struct _XF86VidModeSetViewPort {
    CARD8	reqType;		/* always VidModeReqCode */
    CARD8	xf86vidmodeReqType;	/* always X_XF86VidModeSetViewPort */
    CARD16	length;
    CARD16	screen;
    CARD16	pad;
    CARD32	x;
    CARD32	y;
} xXF86VidModeSetViewPortReq;
#define sz_xXF86VidModeSetViewPortReq	16

typedef struct {
    BYTE	type;
    BOOL	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	flags;
    CARD32	clocks;
    CARD32	maxclocks;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
} xXF86VidModeGetDotClocksReply;
#define sz_xXF86VidModeGetDotClocksReply	32

typedef struct _XF86VidModeSetClientVersion {
    CARD8	reqType;		/* always XF86VidModeReqCode */
    CARD8	xf86vidmodeReqType;
    CARD16	length;
    CARD16	major;
    CARD16	minor;
} xXF86VidModeSetClientVersionReq;
#define sz_xXF86VidModeSetClientVersionReq	8

typedef struct _XF86VidModeGetGamma {
    CARD8	reqType;		/* always XF86VidModeReqCode */
    CARD8	xf86vidmodeReqType;
    CARD16	length;
    CARD16	screen;
    CARD16	pad;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xXF86VidModeGetGammaReq;
#define sz_xXF86VidModeGetGammaReq		32

typedef struct {
    BYTE	type;
    BOOL	pad;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	red;
    CARD32	green;
    CARD32	blue;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
} xXF86VidModeGetGammaReply;
#define sz_xXF86VidModeGetGammaReply		32

typedef struct _XF86VidModeSetGamma {
    CARD8	reqType;		/* always XF86VidModeReqCode */
    CARD8	xf86vidmodeReqType;
    CARD16	length;
    CARD16	screen;
    CARD16	pad;
    CARD32	red;
    CARD32	green;
    CARD32	blue;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
} xXF86VidModeSetGammaReq;
#define sz_xXF86VidModeSetGammaReq		32


typedef struct _XF86VidModeSetGammaRamp {
    CARD8       reqType;                /* always XF86VidModeReqCode */
    CARD8       xf86vidmodeReqType;
    CARD16      length;
    CARD16      screen;
    CARD16      size;
} xXF86VidModeSetGammaRampReq;
#define sz_xXF86VidModeSetGammaRampReq             8

typedef struct _XF86VidModeGetGammaRamp {
    CARD8       reqType;                /* always XF86VidModeReqCode */
    CARD8       xf86vidmodeReqType;
    CARD16      length;
    CARD16      screen;
    CARD16      size;
} xXF86VidModeGetGammaRampReq;
#define sz_xXF86VidModeGetGammaRampReq             8

typedef struct {
    BYTE        type;
    BOOL        pad;
    CARD16      sequenceNumber;
    CARD32      length;
    CARD16      size;
    CARD16      pad0;
    CARD32      pad1;
    CARD32      pad2;
    CARD32      pad3;
    CARD32      pad4;
    CARD32      pad5;
} xXF86VidModeGetGammaRampReply;
#define sz_xXF86VidModeGetGammaRampReply            32

typedef struct _XF86VidModeGetGammaRampSize {
    CARD8       reqType;                /* always XF86VidModeReqCode */
    CARD8       xf86vidmodeReqType;
    CARD16      length;
    CARD16      screen;
    CARD16      pad;
} xXF86VidModeGetGammaRampSizeReq;
#define sz_xXF86VidModeGetGammaRampSizeReq             8

typedef struct {
    BYTE        type;
    BOOL        pad;
    CARD16      sequenceNumber;
    CARD32      length;
    CARD16      size;
    CARD16      pad0;
    CARD32      pad1;
    CARD32      pad2;
    CARD32      pad3;
    CARD32      pad4;
    CARD32      pad5;
} xXF86VidModeGetGammaRampSizeReply;
#define sz_xXF86VidModeGetGammaRampSizeReply            32

typedef struct {
    BYTE        type;
    BOOL        pad;
    CARD16      sequenceNumber;
    CARD32      length;
    CARD32      permissions;
    CARD32      pad1;
    CARD32      pad2;
    CARD32      pad3;
    CARD32      pad4;
    CARD32      pad5;
} xXF86VidModeGetPermissionsReply;
#define sz_xXF86VidModeGetPermissionsReply            32


#endif /* _XF86VIDMODEPROTO_H_ */

