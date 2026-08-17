/******************************************************************************
 *
 * Copyright (c) 1994, 1995  Hewlett-Packard Company
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL HEWLETT-PACKARD COMPANY BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR
 * THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * Except as contained in this notice, the name of the Hewlett-Packard
 * Company shall not be used in advertising or otherwise to promote the
 * sale, use or other dealings in this Software without prior written
 * authorization from the Hewlett-Packard Company.
 *
 *     Header file for Xlib-related DBE
 *
 *****************************************************************************/

#ifndef DBE_PROTO_H
#define DBE_PROTO_H

#include <X11/extensions/dbe.h>

/* Request values used in (S)ProcDbeDispatch() */
#define X_DbeGetVersion                 0
#define X_DbeAllocateBackBufferName     1
#define X_DbeDeallocateBackBufferName   2
#define X_DbeSwapBuffers                3
#define X_DbeBeginIdiom                 4
#define X_DbeEndIdiom                   5
#define X_DbeGetVisualInfo              6
#define X_DbeGetBackBufferAttributes    7

typedef CARD8  xDbeSwapAction;
typedef CARD32 xDbeBackBuffer;

/* TYPEDEFS */

/* Protocol data types */

typedef struct
{
    CARD32		window;		/* window      */
    xDbeSwapAction	swapAction;	/* swap action */
    CARD8		pad1;		/* unused      */
    CARD16		pad2;

} xDbeSwapInfo;

typedef struct
{
    CARD32	visualID;	/* associated visual      */
    CARD8	depth;		/* depth of visual        */
    CARD8	perfLevel;	/* performance level hint */
    CARD16	pad1;

} xDbeVisInfo;
#define sz_xDbeVisInfo	8

typedef struct
{
    CARD32	n;	/* number of visual info items in list  */

} xDbeScreenVisInfo;	/* followed by n xDbeVisInfo items */

typedef struct
{
    CARD32	window;		/* window */

} xDbeBufferAttributes;


/* Requests and replies */

typedef struct
{
    CARD8	reqType;	/* major-opcode: always codes->major_opcode */
    CARD8	dbeReqType;	/* minor-opcode: always X_DbeGetVersion (0) */
    CARD16	length;		/* request length: (2)                      */
    CARD8	majorVersion;	/* client-major-version                     */
    CARD8	minorVersion;	/* client-minor-version                     */
    CARD16	unused;		/* unused                                   */

} xDbeGetVersionReq;
#define sz_xDbeGetVersionReq	8

typedef struct
{
    BYTE	type;			/* Reply: X_Reply (1)   */
    CARD8	unused;			/* unused               */
    CARD16	sequenceNumber;		/* sequence number      */
    CARD32	length;			/* reply length: (0)    */
    CARD8	majorVersion;		/* server-major-version */
    CARD8	minorVersion;		/* server-minor-version */
    CARD16	pad1;			/* unused               */
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;

} xDbeGetVersionReply;
#define sz_xDbeGetVersionReply	32

typedef struct
{
    CARD8		reqType;	/* major-opcode: codes->major_opcode */
    CARD8		dbeReqType;	/* X_DbeAllocateBackBufferName (1)   */
    CARD16		length;		/* request length: (4)               */
    CARD32		window;		/* window                            */
    xDbeBackBuffer	buffer;		/* back buffer name                  */
    xDbeSwapAction	swapAction;	/* swap action hint                  */
    CARD8		pad1;		/* unused                            */
    CARD16		pad2;

} xDbeAllocateBackBufferNameReq;
#define sz_xDbeAllocateBackBufferNameReq	16

typedef struct
{
    CARD8		reqType;	/* major-opcode: codes->major_opcode */
    CARD8		dbeReqType;	/* X_DbeDeallocateBackBufferName (2) */
    CARD16		length;		/* request length: (2)               */
    xDbeBackBuffer	buffer;		/* back buffer name                  */

} xDbeDeallocateBackBufferNameReq;
#define sz_xDbeDeallocateBackBufferNameReq	8

typedef struct
{
    CARD8	reqType;	/* major-opcode: always codes->major_opcode  */
    CARD8	dbeReqType;	/* minor-opcode: always X_DbeSwapBuffers (3) */
    CARD16	length;		/* request length: (2+2n)                    */
    CARD32	n;		/* n, number of window/swap action pairs     */

} xDbeSwapBuffersReq;		/* followed by n window/swap action pairs    */
#define sz_xDbeSwapBuffersReq	8

typedef struct
{
    CARD8	reqType;	/* major-opcode: always codes->major_opcode */
    CARD8	dbeReqType;	/* minor-opcode: always X_DbeBeginIdom (4)  */
    CARD16	length;		/* request length: (1)                      */

} xDbeBeginIdiomReq;
#define sz_xDbeBeginIdiomReq	4

typedef struct
{
    CARD8	reqType;	/* major-opcode: always codes->major_opcode */
    CARD8	dbeReqType;	/* minor-opcode: always X_DbeEndIdom (5)    */
    CARD16	length;		/* request length: (1)                      */

} xDbeEndIdiomReq;
#define sz_xDbeEndIdiomReq	4

typedef struct
{
    CARD8	reqType;	/* always codes->major_opcode     */
    CARD8	dbeReqType;	/* always X_DbeGetVisualInfo (6)  */
    CARD16	length;		/* request length: (2+n)          */
    CARD32	n;		/* n, number of drawables in list */

} xDbeGetVisualInfoReq;		/* followed by n drawables        */
#define sz_xDbeGetVisualInfoReq	8

typedef struct
{
    BYTE	type;			/* Reply: X_Reply (1)                */
    CARD8	unused;			/* unused                            */
    CARD16	sequenceNumber;		/* sequence number                   */
    CARD32	length;			/* reply length                      */
    CARD32	m;			/* m, number of visual infos in list */
    CARD32	pad1;			/* unused                            */
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;

} xDbeGetVisualInfoReply;		/* followed by m visual infos        */
#define sz_xDbeGetVisualInfoReply	32

typedef struct
{
    CARD8		reqType;	/* always codes->major_opcode       */
    CARD8		dbeReqType;	/* X_DbeGetBackBufferAttributes (7) */
    CARD16		length;		/* request length: (2)              */
    xDbeBackBuffer	buffer;		/* back buffer name                 */

} xDbeGetBackBufferAttributesReq;
#define sz_xDbeGetBackBufferAttributesReq	8

typedef struct
{
    BYTE	type;			/* Reply: X_Reply (1) */
    CARD8	unused;			/* unused             */
    CARD16	sequenceNumber;		/* sequence number    */
    CARD32	length;			/* reply length: (0)  */
    CARD32	attributes;		/* attributes         */
    CARD32	pad1;			/* unused             */
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;

} xDbeGetBackBufferAttributesReply;
#define sz_xDbeGetBackBufferAttributesReply	32

#endif /* DBE_PROTO_H */

