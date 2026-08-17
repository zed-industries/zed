#ifndef _GLX_glxproto_h_
#define _GLX_glxproto_h_

/*
 * SGI FREE SOFTWARE LICENSE B (Version 2.0, Sept. 18, 2008)
 * Copyright (C) 1991-2000 Silicon Graphics, Inc. All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice including the dates of first publication and
 * either this permission notice or a reference to
 * http://oss.sgi.com/projects/FreeB/
 * shall be included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * SILICON GRAPHICS, INC. BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF
 * OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Except as contained in this notice, the name of Silicon Graphics, Inc.
 * shall not be used in advertising or otherwise to promote the sale, use or
 * other dealings in this Software without prior written authorization from
 * Silicon Graphics, Inc.
 */

#include <GL/glxmd.h>

/*****************************************************************************/

/*
** Errors.
*/
#define GLXBadContext		0
#define GLXBadContextState	1
#define GLXBadDrawable		2
#define GLXBadPixmap		3
#define GLXBadContextTag	4
#define GLXBadCurrentWindow	5
#define GLXBadRenderRequest	6
#define GLXBadLargeRequest	7
#define GLXUnsupportedPrivateRequest	8
#define GLXBadFBConfig		9
#define GLXBadPbuffer		10
#define GLXBadCurrentDrawable	11
#define GLXBadWindow		12
#define GLXBadProfileARB        13

#define __GLX_NUMBER_ERRORS 14

/*
** Events.
** __GLX_NUMBER_EVENTS is set to 17 to account for the BufferClobberSGIX
**  event - this helps initialization if the server supports the pbuffer
**  extension and the client doesn't.
*/
#define GLX_PbufferClobber	0
#define GLX_BufferSwapComplete	1

#define __GLX_NUMBER_EVENTS 17

#define GLX_EXTENSION_NAME	"GLX"
#define GLX_EXTENSION_ALIAS	"SGI-GLX"

#define __GLX_MAX_CONTEXT_PROPS 3

#ifndef GLX_VENDOR
#define GLX_VENDOR		0x1
#endif
#ifndef GLX_VERSION
#define GLX_VERSION		0x2
#endif
#ifndef GLX_EXTENSIONS
#define GLX_EXTENSIONS		0x3
#endif

/*****************************************************************************/

/*
** For the structure definitions in this file, we must redefine these types in
** terms of Xmd.h types, which may include bitfields.  All of these are
** undef'ed at the end of this file, restoring the definitions in glx.h.
*/
#define GLXContextID CARD32
#define GLXPixmap CARD32
#define GLXDrawable CARD32
#define GLXPbuffer CARD32
#define GLXWindow CARD32
#define GLXFBConfigID CARD32
#define GLXFBConfigIDSGIX CARD32
#define GLXPbufferSGIX CARD32

/*
** ContextTag is not exposed to the API.
*/
typedef CARD32 GLXContextTag;

/*****************************************************************************/

/*
** Sizes of basic wire types.
*/
#define __GLX_SIZE_INT8		1
#define __GLX_SIZE_INT16	2
#define __GLX_SIZE_INT32	4
#define __GLX_SIZE_CARD8	1
#define __GLX_SIZE_CARD16	2
#define __GLX_SIZE_CARD32	4
#define __GLX_SIZE_FLOAT32	4
#define __GLX_SIZE_FLOAT64	8

/*****************************************************************************/

/* Requests */

/*
** Render command request.  A bunch of rendering commands are packed into
** a single X extension request.
*/
typedef struct GLXRender {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextTag contextTag;
} xGLXRenderReq;
#define sz_xGLXRenderReq 8

/*
** The maximum size that a GLXRender command can be.  The value must fit
** in 16 bits and should be a multiple of 4.
*/
#define __GLX_MAX_RENDER_CMD_SIZE	64000

/*
** Large render command request.  A single large rendering command
** is output in multiple X extension requests.	The first packet
** contains an opcode dependent header (see below) that describes
** the data that follows.
*/
typedef struct GLXRenderLarge {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextTag contextTag;
    CARD16	requestNumber;
    CARD16	requestTotal;
    CARD32	dataBytes;
} xGLXRenderLargeReq;
#define sz_xGLXRenderLargeReq 16

/*
** GLX single request.	Commands that go over as single GLX protocol
** requests use this structure.  The glxCode will be one of the X_GLsop
** opcodes.
*/
typedef struct GLXSingle {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextTag contextTag;
} xGLXSingleReq;
#define sz_xGLXSingleReq 8

/*
** glXQueryVersion request
*/
typedef struct GLXQueryVersion {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	majorVersion;
    CARD32	minorVersion;
} xGLXQueryVersionReq;
#define sz_xGLXQueryVersionReq 12

/*
** glXIsDirect request
*/
typedef struct GLXIsDirect {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextID context;
} xGLXIsDirectReq;
#define sz_xGLXIsDirectReq 8

/*
** glXCreateContext request
*/
typedef struct GLXCreateContext {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextID context;
    CARD32	visual;
    CARD32	screen;
    GLXContextID shareList;
    BOOL	isDirect;
    CARD8	reserved1;
    CARD16	reserved2;
} xGLXCreateContextReq;
#define sz_xGLXCreateContextReq 24

/*
** glXDestroyContext request
*/
typedef struct GLXDestroyContext {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextID context;
} xGLXDestroyContextReq;
#define sz_xGLXDestroyContextReq 8

/*
** glXMakeCurrent request
*/
typedef struct GLXMakeCurrent {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXDrawable drawable;
    GLXContextID context;
    GLXContextTag oldContextTag;
} xGLXMakeCurrentReq;
#define sz_xGLXMakeCurrentReq 16

/*
** glXWaitGL request
*/
typedef struct GLXWaitGL {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextTag contextTag;
} xGLXWaitGLReq;
#define sz_xGLXWaitGLReq 8

/*
** glXWaitX request
*/
typedef struct GLXWaitX {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextTag contextTag;
} xGLXWaitXReq;
#define sz_xGLXWaitXReq 8

/*
** glXCopyContext request
*/
typedef struct GLXCopyContext {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextID source;
    GLXContextID dest;
    CARD32	mask;
    GLXContextTag contextTag;
} xGLXCopyContextReq;
#define sz_xGLXCopyContextReq 20

/*
** glXSwapBuffers request
*/
typedef struct GLXSwapBuffers {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextTag contextTag;
    GLXDrawable drawable;
} xGLXSwapBuffersReq;
#define sz_xGLXSwapBuffersReq 12

/*
** glXUseXFont request
*/
typedef struct GLXUseXFont {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextTag contextTag;
    CARD32	font;
    CARD32	first;
    CARD32	count;
    CARD32	listBase;
} xGLXUseXFontReq;
#define sz_xGLXUseXFontReq 24

/*
** glXCreateGLXPixmap request
*/
typedef struct GLXCreateGLXPixmap {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	screen;
    CARD32	visual;
    CARD32	pixmap;
    GLXPixmap	glxpixmap;
} xGLXCreateGLXPixmapReq;
#define sz_xGLXCreateGLXPixmapReq 20

/*
** glXDestroyGLXPixmap request
*/
typedef struct GLXDestroyGLXPixmap {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXPixmap	glxpixmap;
} xGLXDestroyGLXPixmapReq;
#define sz_xGLXDestroyGLXPixmapReq 8

/*
** glXGetVisualConfigs request
*/
typedef struct GLXGetVisualConfigs {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	screen;
} xGLXGetVisualConfigsReq;
#define sz_xGLXGetVisualConfigsReq 8

/*
** glXVendorPrivate request.
*/
typedef struct GLXVendorPrivate {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    GLXContextTag contextTag;
    /*
    ** More data may follow; this is just the header.
    */
} xGLXVendorPrivateReq;
#define sz_xGLXVendorPrivateReq 12

/*
** glXVendorPrivateWithReply request
*/
typedef struct GLXVendorPrivateWithReply {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    GLXContextTag contextTag;
    /*
    ** More data may follow; this is just the header.
    */
} xGLXVendorPrivateWithReplyReq;
#define sz_xGLXVendorPrivateWithReplyReq 12

/*
** glXQueryExtensionsString request
*/
typedef struct GLXQueryExtensionsString {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	screen;
} xGLXQueryExtensionsStringReq;
#define sz_xGLXQueryExtensionsStringReq 8

/*
** glXQueryServerString request
*/
typedef struct GLXQueryServerString {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	screen;
    CARD32	name;
} xGLXQueryServerStringReq;
#define sz_xGLXQueryServerStringReq 12

/*
** glXClientInfo request
*/
typedef struct GLXClientInfo {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	major;
    CARD32	minor;
    CARD32	numbytes;
} xGLXClientInfoReq;
#define sz_xGLXClientInfoReq 16

/*** Start of GLX 1.3 requests */

/*
** glXGetFBConfigs request
*/
typedef struct GLXGetFBConfigs {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	screen;
} xGLXGetFBConfigsReq;
#define sz_xGLXGetFBConfigsReq 8

/*
** glXCreatePixmap request
*/
typedef struct GLXCreatePixmap {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	screen;
    GLXFBConfigID fbconfig;
    CARD32	pixmap;
    GLXPixmap	glxpixmap;
    CARD32	numAttribs;
    /* followed by attribute list */
} xGLXCreatePixmapReq;
#define sz_xGLXCreatePixmapReq 24

/*
** glXDestroyPixmap request
*/
typedef struct GLXDestroyPixmap {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXPixmap	glxpixmap;
} xGLXDestroyPixmapReq;
#define sz_xGLXDestroyPixmapReq 8

/*
** glXCreateNewContext request
*/
typedef struct GLXCreateNewContext {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextID context;
    GLXFBConfigID fbconfig;
    CARD32	screen;
    CARD32	renderType;
    GLXContextID shareList;
    BOOL	isDirect;
    CARD8	reserved1;
    CARD16	reserved2;
} xGLXCreateNewContextReq;
#define sz_xGLXCreateNewContextReq 28

/*
** glXQueryContext request
*/
typedef struct GLXQueryContext {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextID context;
} xGLXQueryContextReq;
#define sz_xGLXQueryContextReq 8

/*
** glXMakeContextCurrent request
*/
typedef struct GLXMakeContextCurrent {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextTag oldContextTag;
    GLXDrawable drawable;
    GLXDrawable readdrawable;
    GLXContextID context;
} xGLXMakeContextCurrentReq;
#define sz_xGLXMakeContextCurrentReq 20

/*
** glXCreatePbuffer request
*/
typedef struct GLXCreatePbuffer {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	screen;
    GLXFBConfigID fbconfig;
    GLXPbuffer	pbuffer;
    CARD32	numAttribs;
    /* followed by attribute list */
} xGLXCreatePbufferReq;
#define sz_xGLXCreatePbufferReq 20

/*
** glXDestroyPbuffer request
*/
typedef struct GLXDestroyPbuffer {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXPbuffer	pbuffer;
} xGLXDestroyPbufferReq;
#define sz_xGLXDestroyPbufferReq 8

/*
** glXGetDrawableAttributes request
*/
typedef struct GLXGetDrawableAttributes {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXDrawable drawable;
} xGLXGetDrawableAttributesReq;
#define sz_xGLXGetDrawableAttributesReq 8

/*
** glXChangeDrawableAttributes request
*/
typedef struct GLXChangeDrawableAttributes {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXDrawable drawable;
    CARD32	numAttribs;
    /* followed by attribute list */
} xGLXChangeDrawableAttributesReq;
#define sz_xGLXChangeDrawableAttributesReq 12

/*
** glXCreateWindow request
*/
typedef struct GLXCreateWindow {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	screen;
    GLXFBConfigID fbconfig;
    CARD32	window;
    GLXWindow	glxwindow;
    CARD32	numAttribs;
    /* followed by attribute list */
} xGLXCreateWindowReq;
#define sz_xGLXCreateWindowReq 24

/*
** glXDestroyWindow request
*/
typedef struct GLXDestroyWindow {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXWindow	glxwindow;
} xGLXDestroyWindowReq;
#define sz_xGLXDestroyWindowReq 8

/* Replies */

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	error;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXGetErrorReply;
#define sz_xGLXGetErrorReply 32

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    GLXContextTag contextTag;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXMakeCurrentReply;
#define sz_xGLXMakeCurrentReply 32

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXReadPixelsReply;
#define sz_xGLXReadPixelsReply 32

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	width;
    CARD32	height;
    CARD32	depth;
    CARD32	pad6;
} xGLXGetTexImageReply;
#define sz_xGLXGetTexImageReply 32

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	width;
    CARD32	height;
    CARD32	pad5;
    CARD32	pad6;
} xGLXGetSeparableFilterReply;
#define sz_xGLXGetSeparableFilterReply 32

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	width;
    CARD32	height;
    CARD32	pad5;
    CARD32	pad6;
} xGLXGetConvolutionFilterReply;
#define sz_xGLXGetConvolutionFilterReply 32

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	width;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXGetHistogramReply;
#define sz_xGLXGetHistogramReply 32

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXGetMinmaxReply;
#define sz_xGLXGetMinmaxReply 32

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	retval;
    CARD32	size;
    CARD32	newMode;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXRenderModeReply;
#define sz_xGLXRenderModeReply 32

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	majorVersion;
    CARD32	minorVersion;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXQueryVersionReply;
#define sz_xGLXQueryVersionReply 32

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	numVisuals;
    CARD32	numProps;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXGetVisualConfigsReply;
#define sz_xGLXGetVisualConfigsReply 32

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    BOOL	isDirect;
    CARD8	pad1;
    CARD16	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
    CARD32	pad7;
} xGLXIsDirectReply;
#define sz_xGLXIsDirectReply	32

/*
** This reply structure is used for all single replies.  Single replies
** ship either 1 piece of data or N pieces of data.  In these cases
** size indicates how much data is to be returned.
*/
typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	retval;
    CARD32	size;
    CARD32	pad3;			/* NOTE: may hold a single value */
    CARD32	pad4;			/* NOTE: may hold half a double */
    CARD32	pad5;
    CARD32	pad6;
} xGLXSingleReply;
#define sz_xGLXSingleReply 32

/*
** This reply structure is used for all Vendor Private replies. Vendor
** Private replies can ship up to 24 bytes within the header or can
** be variable sized, in which case, the reply length field indicates
** the number of words of data which follow the header.
*/
typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	retval;
    CARD32	size;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXVendorPrivReply;
#define sz_xGLXVendorPrivReply 32

/*
**  QueryExtensionsStringReply
**  n indicates the number of bytes to be returned.
*/
typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad1;
    CARD32	n;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXQueryExtensionsStringReply;
#define sz_xGLXQueryExtensionsStringReply 32

/*
** QueryServerString Reply struct
** n indicates the number of bytes to be returned.
*/
typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad1;
    CARD32	n;
    CARD32	pad3;			/* NOTE: may hold a single value */
    CARD32	pad4;			/* NOTE: may hold half a double */
    CARD32	pad5;
    CARD32	pad6;
} xGLXQueryServerStringReply;
#define sz_xGLXQueryServerStringReply 32

/*** Start of GLX 1.3 replies */

/*
** glXGetFBConfigs reply
*/
typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	numFBConfigs;
    CARD32	numAttribs;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXGetFBConfigsReply;
#define sz_xGLXGetFBConfigsReply 32

/*
** glXQueryContext reply
*/
typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	n;			/* number of attribute/value pairs */
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXQueryContextReply;
#define sz_xGLXQueryContextReply 32

/*
** glXMakeContextCurrent reply
*/
typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    GLXContextTag contextTag;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXMakeContextCurrentReply;
#define sz_xGLXMakeContextCurrentReply 32

/*
** glXCreateGLXPbuffer reply
** This is used only in the direct rendering case on SGIs - otherwise
**  CreateGLXPbuffer has no reply. It is not part of GLX 1.3.
*/
typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	success;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXCreateGLXPbufferReply;
#define sz_xGLXCreateGLXPbufferReply 32

/*
** glXGetDrawableAttributes reply
*/
typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	numAttribs;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXGetDrawableAttributesReply;
#define sz_xGLXGetDrawableAttributesReply 32

/*
** glXGetColorTable reply
*/
typedef struct {
    BYTE	type;		       /* X_Reply */
    CARD8	unused;		       /* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	width;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXGetColorTableReply;
#define sz_xGLXGetColorTableReply 32

/************************************************************************/

/* GLX extension requests and replies */

/*
** glXQueryContextInfoEXT request
*/
typedef struct GLXQueryContextInfoEXT {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    CARD32	pad1;	/* unused; corresponds to contextTag in header */
    GLXContextID context;
} xGLXQueryContextInfoEXTReq;
#define sz_xGLXQueryContextInfoEXTReq 16

/*
** glXQueryContextInfoEXT reply
*/
typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	n;			/* number of attribute/value pairs */
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXQueryContextInfoEXTReply;
#define sz_xGLXQueryContextInfoEXTReply 32

/*
** glXMakeCurrentReadSGI request
*/
typedef struct GLXMakeCurrentReadSGI {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    GLXContextTag oldContextTag;
    GLXDrawable drawable;
    GLXDrawable readable;
    GLXContextID context;
} xGLXMakeCurrentReadSGIReq;
#define sz_xGLXMakeCurrentReadSGIReq 24

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    GLXContextTag contextTag;
    CARD32	writeVid;
    CARD32	writeType;
    CARD32	readVid;
    CARD32	readType;
    CARD32	pad6;
} xGLXMakeCurrentReadSGIReply;
#define sz_xGLXMakeCurrentReadSGIReply 32

/*
** glXGetFBConfigsSGIX request
*/
typedef struct GLXGetFBConfigsSGIX {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    CARD32	pad1;   /* unused; corresponds to contextTag in header */
    CARD32	screen;
} xGLXGetFBConfigsSGIXReq;
#define sz_xGLXGetFBConfigsSGIXReq 16

/*
** glXCreateContextWithConfigSGIX request
*/

typedef struct GLXCreateContextWithConfigSGIX {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    CARD32	pad1;   /* unused; corresponds to contextTag in header */
    GLXContextID context;
    GLXFBConfigID fbconfig;
    CARD32	screen;
    CARD32	renderType;
    GLXContextID shareList;
    BOOL	isDirect;
    CARD8	reserved1;
    CARD16	reserved2;
} xGLXCreateContextWithConfigSGIXReq;
#define sz_xGLXCreateContextWithConfigSGIXReq 36

/*
** glXCreatePixmapWithConfigSGIX request
*/

typedef struct GLXCreateGLXPixmapWithConfigSGIX {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    CARD32	pad1;   /* unused; corresponds to contextTag in header */
    CARD32	screen;
    GLXFBConfigID fbconfig;
    CARD32	pixmap;
    GLXPixmap	glxpixmap;
} xGLXCreateGLXPixmapWithConfigSGIXReq;
#define sz_xGLXCreateGLXPixmapWithConfigSGIXReq 28

/*
** glXCreateGLXPbufferSGIX request
*/
typedef struct GLXCreateGLXPbufferSGIX {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    CARD32	pad1;   /* unused; corresponds to contextTag in header */
    CARD32	screen;
    GLXFBConfigID fbconfig;
    GLXPbuffer	pbuffer;
    CARD32	width;
    CARD32	height;
    /* followed by attribute list */
} xGLXCreateGLXPbufferSGIXReq;
#define sz_xGLXCreateGLXPbufferSGIXReq 32

/*
** glXDestroyGLXPbufferSGIX request
*/
typedef struct GLXDestroyGLXPbuffer {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    CARD32	pad1;   /* unused; corresponds to contextTag in header */
    GLXPbuffer	pbuffer;
} xGLXDestroyGLXPbufferSGIXReq;
#define sz_xGLXDestroyGLXPbufferSGIXReq 16

/*
** glXChangeDrawableAttributesSGIX request
*/
typedef struct GLXChangeDrawableAttributesSGIX {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    CARD32	pad1;   /* unused; corresponds to contextTag in header */
    GLXDrawable drawable;
    CARD32	numAttribs;
    /* followed by attribute list */
} xGLXChangeDrawableAttributesSGIXReq;
#define sz_xGLXChangeDrawableAttributesSGIXReq 20

/*
** glXGetDrawableAttributesSGIX request
*/
typedef struct GLXGetDrawableAttributesSGIX {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    CARD32	pad1;   /* unused; corresponds to contextTag in header */
    GLXDrawable drawable;
} xGLXGetDrawableAttributesSGIXReq;
#define sz_xGLXGetDrawableAttributesSGIXReq 16

/*
** glXGetDrawableAttributesSGIX reply
*/
typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	numAttribs;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXGetDrawableAttributesSGIXReply;
#define sz_xGLXGetDrawableAttributesSGIXReply 32

/*
** glXJoinSwapGroupSGIX request
*/
typedef struct GLXJoinSwapGroupSGIX {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    CARD32 	unused;		/* corresponds to contextTag in hdr */
    GLXDrawable	drawable;
    GLXDrawable	member;
} xGLXJoinSwapGroupSGIXReq;
#define sz_xGLXJoinSwapGroupSGIXReq 20

/*
** glXBindSwapBarrierSGIX request
*/
typedef struct GLXBindSwapBarrierSGIX {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    CARD32 	unused;		/* corresponds to contextTag in hdr */
    GLXDrawable	drawable;
    CARD32	barrier;
} xGLXBindSwapBarrierSGIXReq;
#define sz_xGLXBindSwapBarrierSGIXReq 20

/*
** glXQueryMaxSwapBarriersSGIX request
*/
typedef struct GLXQueryMaxSwapBarriersSGIX {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	vendorCode;		/* vendor-specific opcode */
    CARD32 	unused;		/* corresponds to contextTag in hdr */
    CARD32	screen;
} xGLXQueryMaxSwapBarriersSGIXReq;
#define sz_xGLXQueryMaxSwapBarriersSGIXReq 16

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	unused;			/* not used */
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	max;
    CARD32	size;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
} xGLXQueryMaxSwapBarriersSGIXReply;
#define sz_xGLXQueryMaxSwapBarriersSGIXReply 32

/*
** glXQueryHyperpipeNetworkSGIX request
*/
typedef struct GLXQueryHyperpipeNetworkSGIX {
    CARD8       reqType;
    CARD8       glxCode;
    CARD16      length;
    CARD32      vendorCode;		/* vendor-specific opcode */
    CARD32      pad1;   /* unused; corresponds to contextTag in header */
    CARD32      screen;
} xGLXQueryHyperpipeNetworkSGIXReq;
#define sz_xGLXQueryHyperpipeNetworkSGIXReq 16

/*
** glXQueryHyperpipeNetworkSGIX reply
*/
typedef struct {
    BYTE        type;                   /* X_Reply */
    CARD8       unused;                 /* not used */
    CARD16      sequenceNumber;
    CARD32      length;
    CARD32      pad1;
    CARD32      n;
    CARD32	npipes;			/* NOTE: may hold a single value */
    CARD32	pad4;			/* NOTE: may hold half a double */
    CARD32      pad5;
    CARD32      pad6;
} xGLXQueryHyperpipeNetworkSGIXReply;
#define sz_xGLXQueryHyperpipeNetworkSGIXReply 32

/*
** glXDestroyHyperpipeConfigSGIX request
*/
typedef struct GLXDestroyHyperpipeConfigSGIX {
    CARD8       reqType;
    CARD8       glxCode;
    CARD16      length;
    CARD32      vendorCode;		/* vendor-specific opcode */
    CARD32      pad1;   /* unused; corresponds to contextTag in header */
    CARD32      screen;
    CARD32      hpId;
    CARD32      pad2;
    CARD32      pad3;
    CARD32      pad4;
} xGLXDestroyHyperpipeConfigSGIXReq;
#define sz_xGLXDestroyHyperpipeConfigSGIXReq 32

/*
** glXDestroyHyperpipeConfigSGIX reply
*/
typedef struct {
    BYTE        type;                   /* X_Reply */
    CARD8       unused;                 /* not used */
    CARD16      sequenceNumber;
    CARD32      length;
    CARD32      pad1;
    CARD32      n;
    CARD32      success;		/* NOTE: may hold a single value */
    CARD32      pad4;			/* NOTE: may hold half a double */
    CARD32      pad5;
    CARD32      pad6;
} xGLXDestroyHyperpipeConfigSGIXReply;
#define sz_xGLXDestroyHyperpipeConfigSGIXReply 32

/*
** glXQueryHyperpipeConfigSGIX request
*/
typedef struct GLXQueryHyperpipeConfigSGIX {
    CARD8       reqType;
    CARD8       glxCode;
    CARD16      length;
    CARD32      vendorCode;		/* vendor-specific opcode */
    CARD32      pad1;   /* unused; corresponds to contextTag in header */
    CARD32      screen;
    CARD32      hpId;
    CARD32      pad2;
    CARD32      pad3;
    CARD32      pad4;
} xGLXQueryHyperpipeConfigSGIXReq;
#define sz_xGLXQueryHyperpipeConfigSGIXReq 32

/*
** glXQueryHyperpipeConfigSGIX reply
*/
typedef struct {
    BYTE        type;                   /* X_Reply */
    CARD8       unused;                 /* not used */
    CARD16      sequenceNumber;
    CARD32      length;
    CARD32      pad1;
    CARD32      n;
    CARD32      npipes;
    CARD32      pad4;
    CARD32      pad5;
    CARD32      pad6;
} xGLXQueryHyperpipeConfigSGIXReply;
#define sz_xGLXQueryHyperpipeConfigSGIXReply 32

/*
** glXHyperpipeConfigSGIX request
*/
typedef struct {
    CARD8       reqType;
    CARD8       glxCode;
    CARD16      length;
    CARD32      vendorCode;		/* vendor-specific opcode */
    CARD32      pad1;   /* unused; corresponds to contextTag in header */
    CARD32      screen;
    CARD32      npipes;
    CARD32      networkId;
    CARD32      pad2;
    CARD32      pad3;
    /* followed by attribute list */
} xGLXHyperpipeConfigSGIXReq;
#define sz_xGLXHyperpipeConfigSGIXReq 32

/*
** glXHyperpipeConfigSGIX reply
*/
typedef struct {
    BYTE        type;                   /* X_Reply */
    CARD8       unused;                 /* not used */
    CARD16      sequenceNumber;
    CARD32      length;
    CARD32      pad1;
    CARD32      n;
    CARD32      npipes;
    CARD32      hpId;
    CARD32      pad5;
    CARD32      pad6;
} xGLXHyperpipeConfigSGIXReply;
#define sz_xGLXHyperpipeConfigSGIXReply 32

/**
 * \name Protocol structures for GLX_ARB_create_context and
 * GLX_ARB_create_context_profile
 */
/*@{*/
/**
 * Protocol header for glXSetClientInfoARB
 *
 * This structure is followed by \c numVersions * 2 \c CARD32 values listing
 * the OpenGL versions supported by the client.  The pairs of values are an
 * OpenGL major version followed by a minor version.  For example,
 *
 *      CARD32 versions[4] = { 2, 1, 3, 0 };
 *
 * says that the client supports OpenGL 2.1 and OpenGL 3.0.
 *
 * These are followed by \c numGLExtensionBytes bytes of \c STRING8 containing
 * the OpenGL extension string supported by the client and up to 3 bytes of
 * padding.
 *
 * The list of OpenGL extensions is followed by \c numGLXExtensionBytes bytes
 * of \c STRING8 containing the GLX extension string supported by the client
 * and up to 3 bytes of padding.
 *
 * This protocol replaces \c GLXClientInfo.
 *
 * \sa GLXClientInfo, GLXSetClientInfo2ARB
 */
typedef struct GLXSetClientInfoARB {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	major;
    CARD32	minor;
    CARD32	numVersions;
    CARD32	numGLExtensionBytes;
    CARD32	numGLXExtensionBytes;
    /*
    ** More data may follow; this is just the header.
    */
} xGLXSetClientInfoARBReq;
#define sz_xGLXSetClientInfoARBReq 24

/**
 * Protocol head for glXCreateContextAttribsARB
 *
 * This protocol replaces \c GLXCreateContext, \c GLXCreateNewContext, and
 * \c GLXCreateContextWithConfigSGIX.
 */
typedef struct GLXCreateContextAttribsARB {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    GLXContextID	context;
    GLXFBConfigID	fbconfig;
    CARD32	screen;
    GLXContextID	shareList;
    BOOL	isDirect;
    CARD8	reserved1;
    CARD16	reserved2;
    CARD32	numAttribs;
    /* followed by attribute list */
} xGLXCreateContextAttribsARBReq;
#define sz_xGLXCreateContextAttribsARBReq 28

/**
 * Protocol header for glXSetClientInfo2ARB
 *
 * The glXSetClientInfo2ARB protocol differs from glXSetClientInfoARB in that
 * the list of OpenGL versions supported by the client is 3 \c CARD32 values
 * per version: major version, minor version, and supported profile mask.
 *
 * This protocol replaces \c GLXClientInfo and \c GLXSetClientInfoARB.
 *
 * \sa GLXClientInfo, GLXSetClientInfoARB
 */
typedef struct GLXSetClientInfo2ARB {
    CARD8	reqType;
    CARD8	glxCode;
    CARD16	length;
    CARD32	major;
    CARD32	minor;
    CARD32	numVersions;
    CARD32	numGLExtensionBytes;
    CARD32	numGLXExtensionBytes;
    /*
    ** More data may follow; this is just the header.
    */
} xGLXSetClientInfo2ARBReq;
#define sz_xGLXSetClientInfo2ARBReq 24
/*@}*/

/************************************************************************/

/*
** Events
*/

typedef struct {
    BYTE type;
    BYTE pad;
    CARD16 sequenceNumber;
    CARD16 event_type;  /*** was clobber_class */
    CARD16 draw_type;
    CARD32 drawable;
    CARD32 buffer_mask; /*** was mask */
    CARD16 aux_buffer;
    CARD16 x;
    CARD16 y;
    CARD16 width;
    CARD16 height;
    CARD16 count;
    CARD32 unused2;
} xGLXPbufferClobberEvent;

typedef struct {
    BYTE type;
    BYTE pad;
    CARD16 sequenceNumber;
    CARD16 event_type;
    CARD32 drawable;
    CARD32 ust_hi;
    CARD32 ust_lo;
    CARD32 msc_hi;
    CARD32 msc_lo;
    CARD32 sbc_hi;
    CARD32 sbc_lo;
} xGLXBufferSwapComplete;

typedef struct {
    BYTE type;
    BYTE pad;
    CARD16 sequenceNumber;
    CARD16 event_type;
    CARD16 pad2;
    CARD32 drawable;
    CARD32 ust_hi;
    CARD32 ust_lo;
    CARD32 msc_hi;
    CARD32 msc_lo;
    CARD32 sbc;
} xGLXBufferSwapComplete2;

/************************************************************************/

/*
** Size of the standard X request header.
*/
#define __GLX_SINGLE_HDR_SIZE sz_xGLXSingleReq
#define __GLX_VENDPRIV_HDR_SIZE sz_xGLXVendorPrivateReq

#define __GLX_RENDER_HDR    \
    CARD16	length;	    \
    CARD16	opcode

#define __GLX_RENDER_HDR_SIZE 4

typedef struct {
    __GLX_RENDER_HDR;
} __GLXrenderHeader;

#define __GLX_RENDER_LARGE_HDR \
    CARD32	length;	       \
    CARD32	opcode

#define __GLX_RENDER_LARGE_HDR_SIZE 8

typedef struct {
    __GLX_RENDER_LARGE_HDR;
} __GLXrenderLargeHeader;

/*
** The glBitmap, glPolygonStipple, glTexImage[12]D, glTexSubImage[12]D
** and glDrawPixels calls all have a pixel header transmitted after the
** Render or RenderLarge header and before their own opcode specific
** headers.
*/
#define __GLX_PIXEL_HDR		\
    BOOL	swapBytes;	\
    BOOL	lsbFirst;	\
    CARD8	reserved0;	\
    CARD8	reserved1;	\
    CARD32	rowLength;	\
    CARD32	skipRows;	\
    CARD32	skipPixels;	\
    CARD32	alignment

#define __GLX_PIXEL_HDR_SIZE 20

typedef struct {
    __GLX_PIXEL_HDR;
} __GLXpixelHeader;

/*
** glTexImage[34]D and glTexSubImage[34]D calls
** all have a pixel header transmitted after the Render or RenderLarge
** header and before their own opcode specific headers.
*/
#define __GLX_PIXEL_3D_HDR		\
    BOOL	swapBytes;		\
    BOOL	lsbFirst;		\
    CARD8	reserved0;		\
    CARD8	reserved1;		\
    CARD32	rowLength;		\
    CARD32	imageHeight;		\
    CARD32	imageDepth;		\
    CARD32	skipRows;		\
    CARD32	skipImages;		\
    CARD32	skipVolumes;		\
    CARD32	skipPixels;		\
    CARD32	alignment

#define __GLX_PIXEL_3D_HDR_SIZE 36

/*
** Data that is specific to a glBitmap call.  The data is sent in the
** following order:
**	Render or RenderLarge header
**	Pixel header
**	Bitmap header
*/
#define __GLX_BITMAP_HDR    \
    CARD32	width;	    \
    CARD32	height;	    \
    FLOAT32	xorig F32;  \
    FLOAT32	yorig F32;  \
    FLOAT32	xmove F32;  \
    FLOAT32	ymove F32

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_PIXEL_HDR;
    __GLX_BITMAP_HDR;
} __GLXbitmapHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_PIXEL_HDR;
    __GLX_BITMAP_HDR;
} __GLXbitmapLargeHeader;

typedef struct {
    __GLX_PIXEL_HDR;
    __GLX_BITMAP_HDR;
} __GLXdispatchBitmapHeader;

#define __GLX_BITMAP_HDR_SIZE 24

#define __GLX_BITMAP_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_PIXEL_HDR_SIZE + __GLX_BITMAP_HDR_SIZE)

#define __GLX_BITMAP_CMD_DISPATCH_HDR_SIZE \
    (__GLX_PIXEL_HDR_SIZE + __GLX_BITMAP_HDR_SIZE)

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_PIXEL_HDR;
} __GLXpolygonStippleHeader;

#define __GLX_POLYGONSTIPPLE_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_PIXEL_HDR_SIZE)

/*
** Data that is specific to a glTexImage1D or glTexImage2D call.  The
** data is sent in the following order:
**	Render or RenderLarge header
**	Pixel header
**	TexImage header
** When a glTexImage1D call the height field is unexamined by the server.
*/
#define __GLX_TEXIMAGE_HDR	\
    CARD32	target;		\
    CARD32	level;		\
    CARD32	components;	\
    CARD32	width;		\
    CARD32	height;		\
    CARD32	border;		\
    CARD32	format;		\
    CARD32	type

#define __GLX_TEXIMAGE_HDR_SIZE 32

#define __GLX_TEXIMAGE_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_PIXEL_HDR_SIZE + __GLX_TEXIMAGE_HDR_SIZE)

#define __GLX_TEXIMAGE_CMD_DISPATCH_HDR_SIZE \
    (__GLX_PIXEL_HDR_SIZE + __GLX_TEXIMAGE_HDR_SIZE)

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_PIXEL_HDR;
    __GLX_TEXIMAGE_HDR;
} __GLXtexImageHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_PIXEL_HDR;
    __GLX_TEXIMAGE_HDR;
} __GLXtexImageLargeHeader;

typedef struct {
    __GLX_PIXEL_HDR;
    __GLX_TEXIMAGE_HDR;
} __GLXdispatchTexImageHeader;

/*
** Data that is specific to a glTexImage3D or glTexImage4D call.  The
** data is sent in the following order:
**	Render or RenderLarge header
**	Pixel 3D header
**	TexImage 3D header
** When a glTexImage3D call the size4d and woffset fields are unexamined
** by the server.
** Could be used by all TexImage commands and perhaps should be in the
** future.
*/
#define __GLX_TEXIMAGE_3D_HDR \
    CARD32	target;		\
    CARD32	level;		\
    CARD32	internalformat;	\
    CARD32	width;		\
    CARD32	height;		\
    CARD32	depth;		\
    CARD32	size4d;		\
    CARD32	border;		\
    CARD32	format;		\
    CARD32	type;		\
    CARD32	nullimage

#define __GLX_TEXIMAGE_3D_HDR_SIZE 44

#define __GLX_TEXIMAGE_3D_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_PIXEL_3D_HDR_SIZE + \
		__GLX_TEXIMAGE_3D_HDR_SIZE)

#define __GLX_TEXIMAGE_3D_CMD_DISPATCH_HDR_SIZE \
    (__GLX_PIXEL_3D_HDR_SIZE + __GLX_TEXIMAGE_3D_HDR_SIZE)

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_PIXEL_3D_HDR;
    __GLX_TEXIMAGE_3D_HDR;
} __GLXtexImage3DHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_PIXEL_3D_HDR;
    __GLX_TEXIMAGE_3D_HDR;
} __GLXtexImage3DLargeHeader;

typedef struct {
    __GLX_PIXEL_3D_HDR;
    __GLX_TEXIMAGE_3D_HDR;
} __GLXdispatchTexImage3DHeader;

/*
** Data that is specific to a glTexSubImage1D or glTexSubImage2D call.	The
** data is sent in the following order:
**	Render or RenderLarge header
**	Pixel header
**	TexSubImage header
** When a glTexSubImage1D call is made, the yoffset and height fields
** are unexamined by the server and are  considered to be padding.
*/
#define __GLX_TEXSUBIMAGE_HDR	\
    CARD32	target;		\
    CARD32	level;		\
    CARD32	xoffset;	\
    CARD32	yoffset;	\
    CARD32	width;		\
    CARD32	height;		\
    CARD32	format;		\
    CARD32	type;		\
    CARD32	nullImage	\

#define __GLX_TEXSUBIMAGE_HDR_SIZE 36

#define __GLX_TEXSUBIMAGE_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_PIXEL_HDR_SIZE + __GLX_TEXSUBIMAGE_HDR_SIZE)

#define __GLX_TEXSUBIMAGE_CMD_DISPATCH_HDR_SIZE \
    (__GLX_PIXEL_HDR_SIZE + __GLX_TEXSUBIMAGE_HDR_SIZE)

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_PIXEL_HDR;
    __GLX_TEXSUBIMAGE_HDR;
} __GLXtexSubImageHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_PIXEL_HDR;
    __GLX_TEXSUBIMAGE_HDR;
} __GLXtexSubImageLargeHeader;

typedef struct {
    __GLX_PIXEL_HDR;
    __GLX_TEXSUBIMAGE_HDR;
} __GLXdispatchTexSubImageHeader;

/*
** Data that is specific to a glTexSubImage3D and 4D calls.  The
** data is sent in the following order:
**	Render or RenderLarge header
**	Pixel 3D header
**	TexSubImage 3D header
** When a glTexSubImage3D call is made, the woffset and size4d fields
** are unexamined by the server and are considered to be padding.
*/
#define __GLX_TEXSUBIMAGE_3D_HDR	\
    CARD32	target;			\
    CARD32	level;			\
    CARD32	xoffset;		\
    CARD32	yoffset;		\
    CARD32	zoffset;		\
    CARD32	woffset;		\
    CARD32	width;			\
    CARD32	height;			\
    CARD32	depth;			\
    CARD32	size4d;			\
    CARD32	format;			\
    CARD32	type;			\
    CARD32	nullImage		\

#define __GLX_TEXSUBIMAGE_3D_HDR_SIZE 52

#define __GLX_TEXSUBIMAGE_3D_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_PIXEL_3D_HDR_SIZE + \
		__GLX_TEXSUBIMAGE_3D_HDR_SIZE)

#define __GLX_TEXSUBIMAGE_3D_CMD_DISPATCH_HDR_SIZE \
    (__GLX_PIXEL_3D_HDR_SIZE + __GLX_TEXSUBIMAGE_3D_HDR_SIZE)

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_PIXEL_3D_HDR;
    __GLX_TEXSUBIMAGE_3D_HDR;
} __GLXtexSubImage3DHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_PIXEL_3D_HDR;
    __GLX_TEXSUBIMAGE_3D_HDR;
} __GLXtexSubImage3DLargeHeader;

typedef struct {
    __GLX_PIXEL_3D_HDR;
    __GLX_TEXSUBIMAGE_3D_HDR;
} __GLXdispatchTexSubImage3DHeader;

/**
 * Data that is specific to a \c glCompressedTexImage1D or
 * \c glCompressedTexImage2D call.  The data is sent in the following
 * order:
 *     - Render or RenderLarge header
 *     - CompressedTexImage header
 *
 * When a \c glCompressedTexImage1D call is made, the \c height field is
 * not examined by the server and is considered padding.
 */

#define __GLX_COMPRESSED_TEXIMAGE_HDR \
    CARD32     target;		      \
    CARD32     level;		      \
    CARD32     internalFormat;	      \
    CARD32     width;		      \
    CARD32     height;		      \
    CARD32     border;		      \
    CARD32     imageSize

#define __GLX_COMPRESSED_TEXIMAGE_HDR_SIZE 28

#define __GLX_COMPRESSED_TEXIMAGE_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_COMPRESSED_TEXIMAGE_HDR_SIZE)

#define __GLX_COMPRESSED_TEXIMAGE_DISPATCH_HDR_SIZE \
    (__GLX_COMPRESSED_TEXIMAGE_HDR_SIZE)

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_COMPRESSED_TEXIMAGE_HDR;
} __GLXcompressedTexImageHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_COMPRESSED_TEXIMAGE_HDR;
} __GLXcompressedTexImageLargeHeader;

typedef struct {
    __GLX_COMPRESSED_TEXIMAGE_HDR;
} __GLXdispatchCompressedTexImageHeader;

/**
 * Data that is specifi to a \c glCompressedTexSubImage1D or
 * \c glCompressedTexSubImage2D call.  The data is sent in the following
 * order:
 *     - Render or RenderLarge header
 *     - CompressedTexSubImage header
 *
 * When a \c glCompressedTexSubImage1D call is made, the \c yoffset and
 * \c height fields are not examined by the server and are considered padding.
 */

#define __GLX_COMPRESSED_TEXSUBIMAGE_HDR \
    CARD32     target;			 \
    CARD32     level;			 \
    CARD32     xoffset;			 \
    CARD32     yoffset;			 \
    CARD32     width;			 \
    CARD32     height;			 \
    CARD32     format;			 \
    CARD32     imageSize

#define __GLX_COMPRESSED_TEXSUBIMAGE_HDR_SIZE 32

#define __GLX_COMPRESSED_TEXSUBIMAGE_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_COMPRESSED_TEXSUBIMAGE_HDR_SIZE)

#define __GLX_COMPRESSED_TEXSUBIMAGE_DISPATCH_HDR_SIZE \
    (__GLX_COMPRESSED_TEXSUBIMAGE_HDR_SIZE)

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_COMPRESSED_TEXSUBIMAGE_HDR;
} __GLXcompressedTexSubImageHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_COMPRESSED_TEXSUBIMAGE_HDR;
} __GLXcompressedTexSubImageLargeHeader;

typedef struct {
    __GLX_COMPRESSED_TEXSUBIMAGE_HDR;
} __GLXdispatchCompressedTexSubImageHeader;

/**
 * Data that is specific to a \c glCompressedTexImage3D call.  The data is
 * sent in the following order:
 *     - Render or RenderLarge header
 *     - CompressedTexImage3D header
 */

#define __GLX_COMPRESSED_TEXIMAGE_3D_HDR \
    CARD32     target;			 \
    CARD32     level;			 \
    CARD32     internalFormat;		 \
    CARD32     width;			 \
    CARD32     height;			 \
    CARD32     depth;			 \
    CARD32     border;			 \
    CARD32     imageSize

#define __GLX_COMPRESSED_TEXIMAGE_3D_HDR_SIZE 32

#define __GLX_COMPRESSED_TEXIMAGE_3D_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_COMPRESSED_TEXIMAGE_3D_HDR_SIZE)

#define __GLX_COMPRESSED_TEXIMAGE_3D_DISPATCH_HDR_SIZE \
    (__GLX_COMPRESSED_TEXIMAGE_3D_HDR_SIZE)

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_COMPRESSED_TEXIMAGE_3D_HDR;
} __GLXcompressedTexImage3DHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_COMPRESSED_TEXIMAGE_3D_HDR;
} __GLXcompressedTexImage3DLargeHeader;

typedef struct {
    __GLX_COMPRESSED_TEXIMAGE_3D_HDR;
} __GLXdispatchCompressedTexImage3DHeader;

/**
 * Data that is specifi to a \c glCompressedTexSubImage3D call.  The data is
 * sent in the following order:
 *     - Render or RenderLarge header
 *     - CompressedTexSubImage3D header
 */

#define __GLX_COMPRESSED_TEXSUBIMAGE_3D_HDR \
    CARD32     target;			    \
    CARD32     level;			    \
    CARD32     xoffset;			    \
    CARD32     yoffset;			    \
    CARD32     zoffset;			    \
    CARD32     width;			    \
    CARD32     height;			    \
    CARD32     depth;			    \
    CARD32     format;			    \
    CARD32     imageSize

#define __GLX_COMPRESSED_TEXSUBIMAGE_3D_HDR_SIZE 32

#define __GLX_COMPRESSED_TEXSUBIMAGE_3D_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_COMPRESSED_TEXSUBIMAGE_3D_HDR_SIZE)

#define __GLX_COMPRESSED_TEXSUBIMAGE_3D_DISPATCH_HDR_SIZE \
    (__GLX_COMPRESSED_TEXSUBIMAGE_3D_HDR_SIZE)

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_COMPRESSED_TEXSUBIMAGE_3D_HDR;
} __GLXcompressedTexSubImage3DHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_COMPRESSED_TEXSUBIMAGE_3D_HDR;
} __GLXcompressedTexSubImage3DLargeHeader;

typedef struct {
    __GLX_COMPRESSED_TEXSUBIMAGE_3D_HDR;
} __GLXdispatchCompressedTexSubImage3DHeader;

/*
** Data that is specific to a glDrawPixels call.  The data is sent in the
** following order:
**	Render or RenderLarge header
**	Pixel header
**	DrawPixels header
*/
#define __GLX_DRAWPIXELS_HDR \
    CARD32	width;	     \
    CARD32	height;	     \
    CARD32	format;	     \
    CARD32	type

#define __GLX_DRAWPIXELS_HDR_SIZE 16

#define __GLX_DRAWPIXELS_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_PIXEL_HDR_SIZE + __GLX_DRAWPIXELS_HDR_SIZE)

#define __GLX_DRAWPIXELS_CMD_DISPATCH_HDR_SIZE \
    (__GLX_PIXEL_HDR_SIZE + __GLX_DRAWPIXELS_HDR_SIZE)

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_PIXEL_HDR;
    __GLX_DRAWPIXELS_HDR;
} __GLXdrawPixelsHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_PIXEL_HDR;
    __GLX_DRAWPIXELS_HDR;
} __GLXdrawPixelsLargeHeader;

typedef struct {
    __GLX_PIXEL_HDR;
    __GLX_DRAWPIXELS_HDR;
} __GLXdispatchDrawPixelsHeader;

/*
** Data that is specific to a glConvolutionFilter1D or glConvolutionFilter2D
** call.  The data is sent in the following order:
**	Render or RenderLarge header
**	Pixel header
**	ConvolutionFilter header
** When a glConvolutionFilter1D call the height field is unexamined by the server.
*/
#define __GLX_CONV_FILT_HDR	\
    CARD32	target;		\
    CARD32	internalformat;	\
    CARD32	width;		\
    CARD32	height;		\
    CARD32	format;		\
    CARD32	type

#define __GLX_CONV_FILT_HDR_SIZE 24

#define __GLX_CONV_FILT_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_PIXEL_HDR_SIZE + __GLX_CONV_FILT_HDR_SIZE)

#define __GLX_CONV_FILT_CMD_DISPATCH_HDR_SIZE \
    (__GLX_PIXEL_HDR_SIZE + __GLX_CONV_FILT_HDR_SIZE)
typedef struct {
    __GLX_RENDER_HDR;
    __GLX_PIXEL_HDR;
    __GLX_CONV_FILT_HDR;
} __GLXConvolutionFilterHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_PIXEL_HDR;
    __GLX_CONV_FILT_HDR;
} __GLXConvolutionFilterLargeHeader;

typedef struct {
    __GLX_PIXEL_HDR;
    __GLX_CONV_FILT_HDR;
} __GLXdispatchConvolutionFilterHeader;

/*
** Data that is specific to a glDrawArraysEXT call.  The data is sent in the
** following order:
**	Render or RenderLarge header
**	Draw Arrays header
**	a variable number of Component headers
**	vertex data for each component type
*/

#define __GLX_DRAWARRAYS_HDR \
    CARD32	numVertexes;   \
    CARD32	numComponents; \
    CARD32	primType

#define __GLX_DRAWARRAYS_HDR_SIZE 12

#define __GLX_DRAWARRAYS_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_DRAWARRAYS_HDR_SIZE)

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_DRAWARRAYS_HDR;
} __GLXdrawArraysHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_DRAWARRAYS_HDR;
} __GLXdrawArraysLargeHeader;

typedef struct {
    __GLX_DRAWARRAYS_HDR;
} __GLXdispatchDrawArraysHeader;

#define __GLX_COMPONENT_HDR \
    CARD32	datatype;   \
    INT32	numVals;    \
    CARD32	component

typedef struct {
    __GLX_COMPONENT_HDR;
} __GLXdispatchDrawArraysComponentHeader;

#define __GLX_COMPONENT_HDR_SIZE 12

/*
** Data that is specific to a glColorTable call
**	The data is sent in the following order:
**	Render or RenderLarge header
**	Pixel header
**	ColorTable header
*/

#define __GLX_COLOR_TABLE_HDR	     \
    CARD32	target;		     \
    CARD32	internalformat;	     \
    CARD32	width;		     \
    CARD32	format;		     \
    CARD32	type

#define __GLX_COLOR_TABLE_HDR_SIZE 20

#define __GLX_COLOR_TABLE_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_PIXEL_HDR_SIZE + __GLX_COLOR_TABLE_HDR_SIZE)

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_PIXEL_HDR;
    __GLX_COLOR_TABLE_HDR;
} __GLXColorTableHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_PIXEL_HDR;
    __GLX_COLOR_TABLE_HDR;
} __GLXColorTableLargeHeader;

typedef struct {
    __GLX_PIXEL_HDR;
    __GLX_COLOR_TABLE_HDR;
} __GLXdispatchColorTableHeader;

/*
** Data that is specific to a glColorSubTable call
**	The data is sent in the following order:
**	Render or RenderLarge header
**	Pixel header
**	ColorTable header
*/

#define __GLX_COLOR_SUBTABLE_HDR    \
    CARD32	target;		    \
    CARD32	start;		    \
    CARD32	count;		    \
    CARD32	format;		    \
    CARD32	type

#define __GLX_COLOR_SUBTABLE_HDR_SIZE 20

#define __GLX_COLOR_SUBTABLE_CMD_HDR_SIZE \
    (__GLX_RENDER_HDR_SIZE + __GLX_PIXEL_HDR_SIZE + \
     __GLX_COLOR_SUBTABLE_HDR_SIZE)

typedef struct {
    __GLX_RENDER_HDR;
    __GLX_PIXEL_HDR;
    __GLX_COLOR_SUBTABLE_HDR;
} __GLXColorSubTableHeader;

typedef struct {
    __GLX_RENDER_LARGE_HDR;
    __GLX_PIXEL_HDR;
    __GLX_COLOR_SUBTABLE_HDR;
} __GLXColorSubTableLargeHeader;

typedef struct {
    __GLX_PIXEL_HDR;
    __GLX_COLOR_SUBTABLE_HDR;
} __GLXdispatchColorSubTableHeader;

#define GLX_WINDOW_TYPE		1
#define GLX_PIXMAP_TYPE		2
#define GLX_VIDEO_SOURCE_TYPE	3
#define GLX_PBUFFER_TYPE	4
/* 5 is for DM_PBUFFER */
#define GLX_GLXWINDOW_TYPE	6

/*****************************************************************************/

/*
** Restore these definitions back to the typedefs in glx.h
*/
#undef GLXContextID
#undef GLXPixmap
#undef GLXDrawable
#undef GLXPbuffer
#undef GLXWindow
#undef GLXFBConfigID
#undef GLXFBConfigIDSGIX
#undef GLXPbufferSGIX


/* Opcodes for GLX commands */

#define X_GLXRender                       1
#define X_GLXRenderLarge                  2
#define X_GLXCreateContext                3
#define X_GLXDestroyContext               4
#define X_GLXMakeCurrent                  5
#define X_GLXIsDirect                     6
#define X_GLXQueryVersion                 7
#define X_GLXWaitGL                       8
#define X_GLXWaitX                        9
#define X_GLXCopyContext                 10
#define X_GLXSwapBuffers                 11
#define X_GLXUseXFont                    12
#define X_GLXCreateGLXPixmap             13
#define X_GLXGetVisualConfigs            14
#define X_GLXDestroyGLXPixmap            15
#define X_GLXVendorPrivate               16
#define X_GLXVendorPrivateWithReply      17
#define X_GLXQueryExtensionsString       18
#define X_GLXQueryServerString           19
#define X_GLXClientInfo                  20
#define X_GLXGetFBConfigs                21
#define X_GLXCreatePixmap                22
#define X_GLXDestroyPixmap               23
#define X_GLXCreateNewContext            24
#define X_GLXQueryContext                25
#define X_GLXMakeContextCurrent          26
#define X_GLXCreatePbuffer               27
#define X_GLXDestroyPbuffer              28
#define X_GLXGetDrawableAttributes       29
#define X_GLXChangeDrawableAttributes    30
#define X_GLXCreateWindow                31
#define X_GLXDestroyWindow               32
#define X_GLXSetClientInfoARB            33
#define X_GLXCreateContextAttribsARB     34
#define X_GLXSetClientInfo2ARB           35

/* typo compatibility with older headers */
#define X_GLXCreateContextAtrribsARB     34
#define X_GLXSetConfigInfo2ARB           35

/* Opcodes for single commands (part of GLX command space) */

#define X_GLsop_NewList                    101
#define X_GLsop_EndList                    102
#define X_GLsop_DeleteLists                103
#define X_GLsop_GenLists                   104
#define X_GLsop_FeedbackBuffer             105
#define X_GLsop_SelectBuffer               106
#define X_GLsop_RenderMode                 107
#define X_GLsop_Finish                     108
#define X_GLsop_Flush                      142
#define X_GLsop_PixelStoref                109
#define X_GLsop_PixelStorei                110
#define X_GLsop_ReadPixels                 111
#define X_GLsop_GetBooleanv                112
#define X_GLsop_GetClipPlane               113
#define X_GLsop_GetDoublev                 114
#define X_GLsop_GetError                   115
#define X_GLsop_GetFloatv                  116
#define X_GLsop_GetIntegerv                117
#define X_GLsop_GetLightfv                 118
#define X_GLsop_GetLightiv                 119
#define X_GLsop_GetMapdv                   120
#define X_GLsop_GetMapfv                   121
#define X_GLsop_GetMapiv                   122
#define X_GLsop_GetMaterialfv              123
#define X_GLsop_GetMaterialiv              124
#define X_GLsop_GetPixelMapfv              125
#define X_GLsop_GetPixelMapuiv             126
#define X_GLsop_GetPixelMapusv             127
#define X_GLsop_GetPolygonStipple          128
#define X_GLsop_GetString                  129
#define X_GLsop_GetTexEnvfv                130
#define X_GLsop_GetTexEnviv                131
#define X_GLsop_GetTexGendv                132
#define X_GLsop_GetTexGenfv                133
#define X_GLsop_GetTexGeniv                134
#define X_GLsop_GetTexImage                135
#define X_GLsop_GetTexParameterfv          136
#define X_GLsop_GetTexParameteriv          137
#define X_GLsop_GetTexLevelParameterfv     138
#define X_GLsop_GetTexLevelParameteriv     139
#define X_GLsop_IsEnabled                  140
#define X_GLsop_IsList                     141
#define X_GLsop_AreTexturesResident        143
#define X_GLsop_DeleteTextures             144
#define X_GLsop_GenTextures                145
#define X_GLsop_IsTexture                  146
#define X_GLsop_GetColorTable              147
#define X_GLsop_GetColorTableParameterfv   148
#define X_GLsop_GetColorTableParameteriv   149
#define X_GLsop_GetConvolutionFilter       150
#define X_GLsop_GetConvolutionParameterfv  151
#define X_GLsop_GetConvolutionParameteriv  152
#define X_GLsop_GetSeparableFilter         153
#define X_GLsop_GetHistogram               154
#define X_GLsop_GetHistogramParameterfv    155
#define X_GLsop_GetHistogramParameteriv    156
#define X_GLsop_GetMinmax                  157
#define X_GLsop_GetMinmaxParameterfv       158
#define X_GLsop_GetMinmaxParameteriv       159
#define X_GLsop_GetCompressedTexImage      160


/* Opcodes for rendering commands */

#define X_GLrop_CallList                     1
#define X_GLrop_CallLists                    2
#define X_GLrop_ListBase                     3
#define X_GLrop_Begin                        4
#define X_GLrop_Bitmap                       5
#define X_GLrop_Color3bv                     6
#define X_GLrop_Color3dv                     7
#define X_GLrop_Color3fv                     8
#define X_GLrop_Color3iv                     9
#define X_GLrop_Color3sv                    10
#define X_GLrop_Color3ubv                   11
#define X_GLrop_Color3uiv                   12
#define X_GLrop_Color3usv                   13
#define X_GLrop_Color4bv                    14
#define X_GLrop_Color4dv                    15
#define X_GLrop_Color4fv                    16
#define X_GLrop_Color4iv                    17
#define X_GLrop_Color4sv                    18
#define X_GLrop_Color4ubv                   19
#define X_GLrop_Color4uiv                   20
#define X_GLrop_Color4usv                   21
#define X_GLrop_EdgeFlagv                   22
#define X_GLrop_End                         23
#define X_GLrop_Indexdv                     24
#define X_GLrop_Indexfv                     25
#define X_GLrop_Indexiv                     26
#define X_GLrop_Indexsv                     27
#define X_GLrop_Normal3bv                   28
#define X_GLrop_Normal3dv                   29
#define X_GLrop_Normal3fv                   30
#define X_GLrop_Normal3iv                   31
#define X_GLrop_Normal3sv                   32
#define X_GLrop_RasterPos2dv                33
#define X_GLrop_RasterPos2fv                34
#define X_GLrop_RasterPos2iv                35
#define X_GLrop_RasterPos2sv                36
#define X_GLrop_RasterPos3dv                37
#define X_GLrop_RasterPos3fv                38
#define X_GLrop_RasterPos3iv                39
#define X_GLrop_RasterPos3sv                40
#define X_GLrop_RasterPos4dv                41
#define X_GLrop_RasterPos4fv                42
#define X_GLrop_RasterPos4iv                43
#define X_GLrop_RasterPos4sv                44
#define X_GLrop_Rectdv                      45
#define X_GLrop_Rectfv                      46
#define X_GLrop_Rectiv                      47
#define X_GLrop_Rectsv                      48
#define X_GLrop_TexCoord1dv                 49
#define X_GLrop_TexCoord1fv                 50
#define X_GLrop_TexCoord1iv                 51
#define X_GLrop_TexCoord1sv                 52
#define X_GLrop_TexCoord2dv                 53
#define X_GLrop_TexCoord2fv                 54
#define X_GLrop_TexCoord2iv                 55
#define X_GLrop_TexCoord2sv                 56
#define X_GLrop_TexCoord3dv                 57
#define X_GLrop_TexCoord3fv                 58
#define X_GLrop_TexCoord3iv                 59
#define X_GLrop_TexCoord3sv                 60
#define X_GLrop_TexCoord4dv                 61
#define X_GLrop_TexCoord4fv                 62
#define X_GLrop_TexCoord4iv                 63
#define X_GLrop_TexCoord4sv                 64
#define X_GLrop_Vertex2dv                   65
#define X_GLrop_Vertex2fv                   66
#define X_GLrop_Vertex2iv                   67
#define X_GLrop_Vertex2sv                   68
#define X_GLrop_Vertex3dv                   69
#define X_GLrop_Vertex3fv                   70
#define X_GLrop_Vertex3iv                   71
#define X_GLrop_Vertex3sv                   72
#define X_GLrop_Vertex4dv                   73
#define X_GLrop_Vertex4fv                   74
#define X_GLrop_Vertex4iv                   75
#define X_GLrop_Vertex4sv                   76
#define X_GLrop_ClipPlane                   77
#define X_GLrop_ColorMaterial               78
#define X_GLrop_CullFace                    79
#define X_GLrop_Fogf                        80
#define X_GLrop_Fogfv                       81
#define X_GLrop_Fogi                        82
#define X_GLrop_Fogiv                       83
#define X_GLrop_FrontFace                   84
#define X_GLrop_Hint                        85
#define X_GLrop_Lightf                      86
#define X_GLrop_Lightfv                     87
#define X_GLrop_Lighti                      88
#define X_GLrop_Lightiv                     89
#define X_GLrop_LightModelf                 90
#define X_GLrop_LightModelfv                91
#define X_GLrop_LightModeli                 92
#define X_GLrop_LightModeliv                93
#define X_GLrop_LineStipple                 94
#define X_GLrop_LineWidth                   95
#define X_GLrop_Materialf                   96
#define X_GLrop_Materialfv                  97
#define X_GLrop_Materiali                   98
#define X_GLrop_Materialiv                  99
#define X_GLrop_PointSize                  100
#define X_GLrop_PolygonMode                101
#define X_GLrop_PolygonStipple             102
#define X_GLrop_Scissor                    103
#define X_GLrop_ShadeModel                 104
#define X_GLrop_TexParameterf              105
#define X_GLrop_TexParameterfv             106
#define X_GLrop_TexParameteri              107
#define X_GLrop_TexParameteriv             108
#define X_GLrop_TexImage1D                 109
#define X_GLrop_TexImage2D                 110
#define X_GLrop_TexEnvf                    111
#define X_GLrop_TexEnvfv                   112
#define X_GLrop_TexEnvi                    113
#define X_GLrop_TexEnviv                   114
#define X_GLrop_TexGend                    115
#define X_GLrop_TexGendv                   116
#define X_GLrop_TexGenf                    117
#define X_GLrop_TexGenfv                   118
#define X_GLrop_TexGeni                    119
#define X_GLrop_TexGeniv                   120
#define X_GLrop_InitNames                  121
#define X_GLrop_LoadName                   122
#define X_GLrop_PassThrough                123
#define X_GLrop_PopName                    124
#define X_GLrop_PushName                   125
#define X_GLrop_DrawBuffer                 126
#define X_GLrop_Clear                      127
#define X_GLrop_ClearAccum                 128
#define X_GLrop_ClearIndex                 129
#define X_GLrop_ClearColor                 130
#define X_GLrop_ClearStencil               131
#define X_GLrop_ClearDepth                 132
#define X_GLrop_StencilMask                133
#define X_GLrop_ColorMask                  134
#define X_GLrop_DepthMask                  135
#define X_GLrop_IndexMask                  136
#define X_GLrop_Accum                      137
#define X_GLrop_Disable                    138
#define X_GLrop_Enable                     139
#define X_GLrop_PopAttrib                  141
#define X_GLrop_PushAttrib                 142
#define X_GLrop_Map1d                      143
#define X_GLrop_Map1f                      144
#define X_GLrop_Map2d                      145
#define X_GLrop_Map2f                      146
#define X_GLrop_MapGrid1d                  147
#define X_GLrop_MapGrid1f                  148
#define X_GLrop_MapGrid2d                  149
#define X_GLrop_MapGrid2f                  150
#define X_GLrop_EvalCoord1dv               151
#define X_GLrop_EvalCoord1fv               152
#define X_GLrop_EvalCoord2dv               153
#define X_GLrop_EvalCoord2fv               154
#define X_GLrop_EvalMesh1                  155
#define X_GLrop_EvalPoint1                 156
#define X_GLrop_EvalMesh2                  157
#define X_GLrop_EvalPoint2                 158
#define X_GLrop_AlphaFunc                  159
#define X_GLrop_BlendFunc                  160
#define X_GLrop_LogicOp                    161
#define X_GLrop_StencilFunc                162
#define X_GLrop_StencilOp                  163
#define X_GLrop_DepthFunc                  164
#define X_GLrop_PixelZoom                  165
#define X_GLrop_PixelTransferf             166
#define X_GLrop_PixelTransferi             167
#define X_GLrop_PixelMapfv                 168
#define X_GLrop_PixelMapuiv                169
#define X_GLrop_PixelMapusv                170
#define X_GLrop_ReadBuffer                 171
#define X_GLrop_CopyPixels                 172
#define X_GLrop_DrawPixels                 173
#define X_GLrop_DepthRange                 174
#define X_GLrop_Frustum                    175
#define X_GLrop_LoadIdentity               176
#define X_GLrop_LoadMatrixf                177
#define X_GLrop_LoadMatrixd                178
#define X_GLrop_MatrixMode                 179
#define X_GLrop_MultMatrixf                180
#define X_GLrop_MultMatrixd                181
#define X_GLrop_Ortho                      182
#define X_GLrop_PopMatrix                  183
#define X_GLrop_PushMatrix                 184
#define X_GLrop_Rotated                    185
#define X_GLrop_Rotatef                    186
#define X_GLrop_Scaled                     187
#define X_GLrop_Scalef                     188
#define X_GLrop_Translated                 189
#define X_GLrop_Translatef                 190
#define X_GLrop_Viewport                   191
#define X_GLrop_DrawArrays                 193
#define X_GLrop_PolygonOffset              192
#define X_GLrop_CopyTexImage1D             4119
#define X_GLrop_CopyTexImage2D             4120
#define X_GLrop_CopyTexSubImage1D          4121
#define X_GLrop_CopyTexSubImage2D          4122
#define X_GLrop_TexSubImage1D              4099
#define X_GLrop_TexSubImage2D              4100
#define X_GLrop_BindTexture                4117
#define X_GLrop_PrioritizeTextures         4118
#define X_GLrop_Indexubv                   194
#define X_GLrop_BlendColor                 4096
#define X_GLrop_BlendEquation              4097
#define X_GLrop_ColorTable                 2053
#define X_GLrop_ColorTableParameterfv      2054
#define X_GLrop_ColorTableParameteriv      2055
#define X_GLrop_CopyColorTable             2056
#define X_GLrop_ColorSubTable              195
#define X_GLrop_CopyColorSubTable          196
#define X_GLrop_ConvolutionFilter1D        4101
#define X_GLrop_ConvolutionFilter2D        4102
#define X_GLrop_ConvolutionParameterf      4103
#define X_GLrop_ConvolutionParameterfv     4104
#define X_GLrop_ConvolutionParameteri      4105
#define X_GLrop_ConvolutionParameteriv     4106
#define X_GLrop_CopyConvolutionFilter1D    4107
#define X_GLrop_CopyConvolutionFilter2D    4108
#define X_GLrop_SeparableFilter2D          4109
#define X_GLrop_Histogram                  4110
#define X_GLrop_Minmax                     4111
#define X_GLrop_ResetHistogram             4112
#define X_GLrop_ResetMinmax                4113
#define X_GLrop_TexImage3D                 4114
#define X_GLrop_TexSubImage3D              4115
#define X_GLrop_CopyTexSubImage3D          4123
#define X_GLrop_DrawArraysEXT              4116

/* Added for core GL version 1.3 */

#define X_GLrop_ActiveTextureARB            197
#define X_GLrop_MultiTexCoord1dvARB         198
#define X_GLrop_MultiTexCoord1fvARB         199
#define X_GLrop_MultiTexCoord1ivARB         200
#define X_GLrop_MultiTexCoord1svARB         201
#define X_GLrop_MultiTexCoord2dvARB         202
#define X_GLrop_MultiTexCoord2fvARB         203
#define X_GLrop_MultiTexCoord2ivARB         204
#define X_GLrop_MultiTexCoord2svARB         205
#define X_GLrop_MultiTexCoord3dvARB         206
#define X_GLrop_MultiTexCoord3fvARB         207
#define X_GLrop_MultiTexCoord3ivARB         208
#define X_GLrop_MultiTexCoord3svARB         209
#define X_GLrop_MultiTexCoord4dvARB         210
#define X_GLrop_MultiTexCoord4fvARB         211
#define X_GLrop_MultiTexCoord4ivARB         212
#define X_GLrop_MultiTexCoord4svARB         213
#define X_GLrop_CompressedTexImage1D        214
#define X_GLrop_CompressedTexImage2D        215
#define X_GLrop_CompressedTexImage3D        216
#define X_GLrop_CompressedTexSubImage1D     217
#define X_GLrop_CompressedTexSubImage2D     218
#define X_GLrop_CompressedTexSubImage3D     219
#define X_GLrop_SampleCoverageARB           229

/* Added for core GL version 1.4 */

#define X_GLrop_WindowPos3fARB              230
#define X_GLrop_FogCoordfv                  4124
#define X_GLrop_FogCoorddv                  4125
#define X_GLrop_PointParameterfARB          2065
#define X_GLrop_PointParameterfvARB         2066
#define X_GLrop_SecondaryColor3bv           4126
#define X_GLrop_SecondaryColor3sv           4127
#define X_GLrop_SecondaryColor3iv           4128
#define X_GLrop_SecondaryColor3fv           4129
#define X_GLrop_SecondaryColor3dv           4130
#define X_GLrop_SecondaryColor3ubv          4131
#define X_GLrop_SecondaryColor3usv          4132
#define X_GLrop_SecondaryColor3uiv          4133
#define X_GLrop_BlendFuncSeparate           4134
#define X_GLrop_PointParameteri             4221
#define X_GLrop_PointParameteriv            4222

/* Added for core GL version 1.5 */
/* XXX opcodes not defined in the spec */

/* Opcodes for Vendor Private commands */


#define X_GLvop_GetConvolutionFilterEXT        1
#define X_GLvop_GetConvolutionParameterfvEXT   2
#define X_GLvop_GetConvolutionParameterivEXT   3
#define X_GLvop_GetSeparableFilterEXT          4
#define X_GLvop_GetHistogramEXT                5
#define X_GLvop_GetHistogramParameterfvEXT     6
#define X_GLvop_GetHistogramParameterivEXT     7
#define X_GLvop_GetMinmaxEXT                   8
#define X_GLvop_GetMinmaxParameterfvEXT        9
#define X_GLvop_GetMinmaxParameterivEXT        10
#define X_GLvop_AreTexturesResidentEXT         11
#define X_GLvop_DeleteTexturesEXT              12
#define X_GLvop_GenTexturesEXT                 13
#define X_GLvop_IsTextureEXT                   14
#define X_GLvop_GetCombinerInputParameterfvNV  1270
#define X_GLvop_GetCombinerInputParameterivNV  1271
#define X_GLvop_GetCombinerOutputParameterfvNV 1272
#define X_GLvop_GetCombinerOutputParameterivNV 1273
#define X_GLvop_GetFinalCombinerOutputParameterfvNV 1274
#define X_GLvop_GetFinalCombinerOutputParameterivNV 1275
#define X_GLvop_DeleteFenceNV                  1276
#define X_GLvop_GenFencesNV                    1277
#define X_GLvop_IsFenceNV                      1278
#define X_GLvop_TestFenceNV                    1279
#define X_GLvop_GetFenceivNV                   1280
#define X_GLvop_AreProgramsResidentNV          1293
#define X_GLvop_DeleteProgramARB               1294
#define X_GLvop_GenProgramsARB                 1295
#define X_GLvop_GetProgramEnvParameterfvARB    1296
#define X_GLvop_GetProgramEnvParameterdvARB    1297
#define X_GLvop_GetProgramEnvParameterivNV     1298
#define X_GLvop_GetProgramStringNV             1299
#define X_GLvop_GetTrackMatrixivNV             1300
#define X_GLvop_GetVertexAttribdvARB           1301
#define X_GLvop_GetVertexAttribfvARB           1302
#define X_GLvop_GetVertexAttribivARB           1303
#define X_GLvop_IsProgramARB                   1304
#define X_GLvop_GetProgramLocalParameterfvARB  1305
#define X_GLvop_GetProgramLocalParameterdvARB  1306
#define X_GLvop_GetProgramivARB                1307
#define X_GLvop_GetProgramStringARB            1308
#define X_GLvop_GetProgramNamedParameter4fvNV  1310
#define X_GLvop_GetProgramNamedParameter4dvNV  1311
#define X_GLvop_SampleMaskSGIS                 2048
#define X_GLvop_SamplePatternSGIS              2049
#define X_GLvop_GetDetailTexFuncSGIS           4096
#define X_GLvop_GetSharpenTexFuncSGIS          4097
#define X_GLvop_GetColorTableSGI               4098
#define X_GLvop_GetColorTableParameterfvSGI    4099
#define X_GLvop_GetColorTableParameterivSGI    4100
#define X_GLvop_GetTexFilterFuncSGIS           4101
#define X_GLvop_GetInstrumentsSGIX             4102
#define X_GLvop_InstrumentsBufferSGIX          4103
#define X_GLvop_PollInstrumentsSGIX            4104
#define X_GLvop_FlushRasterSGIX                4105

/* Opcodes for GLX vendor private commands */

#define X_GLXvop_QueryContextInfoEXT            1024
#define X_GLXvop_BindTexImageEXT                1330
#define X_GLXvop_ReleaseTexImageEXT             1331
#define X_GLXvop_SwapIntervalSGI                65536
#define X_GLXvop_MakeCurrentReadSGI             65537
#define X_GLXvop_CreateGLXVideoSourceSGIX       65538
#define X_GLXvop_DestroyGLXVideoSourceSGIX      65539
#define X_GLXvop_GetFBConfigsSGIX               65540
#define X_GLXvop_CreateContextWithConfigSGIX    65541
#define X_GLXvop_CreateGLXPixmapWithConfigSGIX  65542
#define X_GLXvop_CreateGLXPbufferSGIX           65543
#define X_GLXvop_DestroyGLXPbufferSGIX          65544
#define X_GLXvop_ChangeDrawableAttributesSGIX   65545
#define X_GLXvop_GetDrawableAttributesSGIX      65546
#define X_GLXvop_JoinSwapGroupSGIX              65547
#define X_GLXvop_BindSwapBarrierSGIX            65548
#define X_GLXvop_QueryMaxSwapBarriersSGIX       65549
#define X_GLXvop_QueryHyperpipeNetworkSGIX      65550
#define X_GLXvop_QueryHyperpipeConfigSGIX       65551
#define X_GLXvop_HyperpipeConfigSGIX            65552
#define X_GLXvop_DestroyHyperpipeConfigSGIX     65553

/* ARB extension opcodes */

/*  1. GL_ARB_multitexture - see GL 1.2 opcodes */
/*  5. GL_ARB_multisample - see GL 1.3 opcodes */
/* 12. GL_ARB_texture_compression - see GL 1.3 opcodes */
/* 14. GL_ARB_point_parameters - see GL 1.4 opcodees */

/* 15. GL_ARB_vertex_blend */
#define X_GLrop_WeightbvARB                  220
#define X_GLrop_WeightubvARB                 221
#define X_GLrop_WeightsvARB                  222
#define X_GLrop_WeightusvARB                 223
#define X_GLrop_WeightivARB                  224
#define X_GLrop_WeightuivARB                 225
#define X_GLrop_VertexBlendARB               226
#define X_GLrop_WeightfvARB                  227
#define X_GLrop_WeightdvARB                  228

/* 16. GL_ARB_matrix_palette */
/* XXX opcodes not defined in the spec */

/* 25. GL_ARB_window_pos - see GL 1.4 opcodes */

/* 26. GL_ARB_vertex_program */
#define X_GLrop_BindProgramARB              4180
#define X_GLrop_ProgramEnvParameter4fvARB   4184
#define X_GLrop_ProgramEnvParameter4dvARB   4185
#define X_GLrop_VertexAttrib1svARB          4189
#define X_GLrop_VertexAttrib2svARB          4190
#define X_GLrop_VertexAttrib3svARB          4191
#define X_GLrop_VertexAttrib4svARB          4192
#define X_GLrop_VertexAttrib1fvARB          4193
#define X_GLrop_VertexAttrib2fvARB          4194
#define X_GLrop_VertexAttrib3fvARB          4195
#define X_GLrop_VertexAttrib4fvARB          4196
#define X_GLrop_VertexAttrib1dvARB          4197
#define X_GLrop_VertexAttrib2dvARB          4198
#define X_GLrop_VertexAttrib3dvARB          4199
#define X_GLrop_ProgramLocalParameter4fvARB 4215
#define X_GLrop_ProgramLocalParameter4dvARB 4216
#define X_GLrop_ProgramStringARB            4217
#define X_GLrop_VertexAttrib4dvARB          4200
#define X_GLrop_VertexAttrib4NubvARB        4201
#define X_GLrop_VertexAttrib4bvARB          4230
#define X_GLrop_VertexAttrib4ivARB          4231
#define X_GLrop_VertexAttrib4ubvARB         4232
#define X_GLrop_VertexAttrib4usvARB         4233
#define X_GLrop_VertexAttrib4uivARB         4234
#define X_GLrop_VertexAttrib4NbvARB         4235
#define X_GLrop_VertexAttrib4NsvARB         4236
#define X_GLrop_VertexAttrib4NivARB         4237
#define X_GLrop_VertexAttrib4NusvARB        4238
#define X_GLrop_VertexAttrib4NuivARB        4239

/* 27. GL_ARB_fragment_program - see GL_ARB_vertex_program opcodes */

/* 29. GL_ARB_occlusion_query */
/* XXX opcodes not defined in the spec */


/* New extension opcodes */

/* 145. GL_EXT_secondary_color - see GL 1.4 opcodes */

/* 188. GL_EXT_vertex_weighting */
#define X_GLrop_VertexWeightfvEXT           4135

/* 191. GL_NV_register_combiners */
#define X_GLrop_CombinerParameterfNV        4136
#define X_GLrop_CombinerParameterfvNV       4137
#define X_GLrop_CombinerParameteriNV        4138
#define X_GLrop_CombinerParameterivNV       4139
#define X_GLrop_CombinerInputNV             4140
#define X_GLrop_CombinerOutputNV            4141
#define X_GLrop_FinalCombinerInputNV        4142

/* 222. GL_NV_fence */
#define X_GLrop_SetFenceNV                  4143
#define X_GLrop_FinishFenceNV               4144

/* 227. GL_NV_register_combiners2 */
/* XXX opcodes not defined in the spec */

/* 233. GL_NV_vertex_program - see also GL_ARB_vertex_program opcodes */
#define X_GLrop_ExecuteProgramNV            4181
#define X_GLrop_RequestResidentProgramsNV   4182
#define X_GLrop_LoadProgamNV                4183
#define X_GLrop_ProgramParameters4fvNV      4186
#define X_GLrop_ProgramParameters4dvNV      4187
#define X_GLrop_TrackMatrixNV               4188
#define X_GLrop_VertexAttribs1svNV          4202
#define X_GLrop_VertexAttribs2svNV          4203
#define X_GLrop_VertexAttribs3svNV          4204
#define X_GLrop_VertexAttribs4svNV          4205
#define X_GLrop_VertexAttribs1fvNV          4206
#define X_GLrop_VertexAttribs2fvNV          4207
#define X_GLrop_VertexAttribs3fvNV          4208
#define X_GLrop_VertexAttribs4fvNV          4209
#define X_GLrop_VertexAttribs1dvNV          4210
#define X_GLrop_VertexAttribs2dvNV          4211
#define X_GLrop_VertexAttribs3dvNV          4212
#define X_GLrop_VertexAttribs4dvNV          4213
#define X_GLrop_VertexAttribs4ubvNV         4214

/* 261. GL_NV_occlusion_query */
/* XXX opcodes not defined in the spec */

/* 262. GL_NV_point_sprite - see GL 1.4 opcodes */

/* 268. GL_EXT_stencil_two_side */
#define X_GLrop_ActiveStencilFaceEXT        4220

/* 282. GL_NV_fragment_program - see also GL_NV_vertex_program and GL_ARB_vertex_program opcodes */
#define X_GLrop_ProgramNamedParameter4fvNV  4218
#define X_GLrop_ProgramNamedParameter4dvNV  4219

/* 285. GL_NV_primitive_restart */
/* XXX opcodes not defined in the spec */

/* 297. GL_EXT_depth_bounds_test */
#define X_GLrop_DepthBoundsEXT              4229

/* 299. GL_EXT_blend_equation_separate */
#define X_GLrop_BlendEquationSeparateEXT    4228

/* 310. GL_EXT_framebuffer_object */
#define X_GLvop_IsRenderbufferEXT                      1422
#define X_GLvop_GenRenderbuffersEXT                    1423
#define X_GLvop_GetRenderbufferParameterivEXT          1424
#define X_GLvop_IsFramebufferEXT                       1425
#define X_GLvop_GenFramebuffersEXT                     1426
#define X_GLvop_CheckFramebufferStatusEXT              1427
#define X_GLvop_GetFramebufferAttachmentParameterivEXT 1428

#endif /* _GLX_glxproto_h_ */
