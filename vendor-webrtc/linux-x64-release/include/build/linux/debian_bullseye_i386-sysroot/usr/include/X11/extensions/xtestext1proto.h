/*
 * xtestext1.h
 *
 * X11 Input Synthesis Extension include file
 */

/*
Copyright 1986, 1987, 1988, 1998  The Open Group

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


Copyright 1986, 1987, 1988 by Hewlett-Packard Corporation

Permission to use, copy, modify, and distribute this
software and its documentation for any purpose and without
fee is hereby granted, provided that the above copyright
notice appear in all copies and that both that copyright
notice and this permission notice appear in supporting
documentation, and that the name of Hewlett-Packard not be used in
advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

Hewlett-Packard makes no representations about the
suitability of this software for any purpose.  It is provided
"as is" without express or implied warranty.

This software is not subject to any license of the American
Telephone and Telegraph Company or of the Regents of the
University of California.

*/

#ifndef _XTESTEXT1PROTO_H
#define _XTESTEXT1PROTO_H 1

#include <X11/extensions/xtestext1const.h>

/*
 * the typedefs for CARD8, CARD16, and CARD32 are defined in Xmd.h
 */

/*
 * XTest request type values
 *
 * used in the XTest extension protocol requests
 */
#define X_TestFakeInput                  1
#define X_TestGetInput                   2
#define X_TestStopInput                  3
#define X_TestReset                      4
#define X_TestQueryInputSize             5

/*
 * This defines the maximum size of a list of input actions
 * to be sent to the server.  It should always be a multiple of
 * 4 so that the entire xTestFakeInputReq structure size is a
 * multiple of 4.
 */

typedef struct {
        CARD8   reqType;        /* always XTestReqCode             */
        CARD8   XTestReqType;   /* always X_TestFakeInput           */
        CARD16  length;         /* 2 + XTestMAX_ACTION_LIST_SIZE/4 */
        CARD32  ack;
        CARD8   action_list[XTestMAX_ACTION_LIST_SIZE];
} xTestFakeInputReq;
#define sz_xTestFakeInputReq (XTestMAX_ACTION_LIST_SIZE + 8)

typedef struct {
        CARD8   reqType;        /* always XTestReqCode  */
        CARD8   XTestReqType;   /* always X_TestGetInput */
        CARD16  length;         /* 2                    */
        CARD32  mode;
} xTestGetInputReq;
#define sz_xTestGetInputReq 8

typedef struct {
        CARD8   reqType;        /* always XTestReqCode   */
        CARD8   XTestReqType;   /* always X_TestStopInput */
        CARD16  length;         /* 1                     */
} xTestStopInputReq;
#define sz_xTestStopInputReq 4

typedef struct {
        CARD8   reqType;        /* always XTestReqCode */
        CARD8   XTestReqType;   /* always X_TestReset   */
        CARD16  length;         /* 1                   */
} xTestResetReq;
#define sz_xTestResetReq 4

typedef struct {
        CARD8   reqType;        /* always XTestReqCode        */
        CARD8   XTestReqType;   /* always X_TestQueryInputSize */
        CARD16  length;         /* 1                          */
} xTestQueryInputSizeReq;
#define sz_xTestQueryInputSizeReq 4

/*
 * This is the definition of the reply for the xTestQueryInputSize
 * request.  It should remain the same minimum size as other replies
 * (32 bytes).
 */
typedef struct {
        CARD8   type;           /* always X_Reply  */
        CARD8   pad1;
        CARD16  sequenceNumber;
        CARD32  length;         /* always 0 */
        CARD32  size_return;
        CARD32  pad2;
        CARD32  pad3;
        CARD32  pad4;
        CARD32  pad5;
        CARD32  pad6;
} xTestQueryInputSizeReply;

/*
 * This is the definition for the input action wire event structure.
 * This event is sent to the client when the server has one or
 * more user input actions to report to the client.  It must
 * remain the same size as all other wire events (32 bytes).
 */
typedef struct {
        CARD8   type;           /* always XTestInputActionType */
        CARD8   pad00;
        CARD16  sequenceNumber;
        CARD8   actions[XTestACTIONS_SIZE];
} xTestInputActionEvent;

/*
 * This is the definition for the xTestFakeAck wire event structure.
 * This event is sent to the client when the server has completely
 * processed its input action buffer, and is ready for more.
 * It must remain the same size as all other wire events (32 bytes).
 */
typedef struct {
        CARD8   type;           /* always XTestFakeAckType */
        CARD8   pad00;
        CARD16  sequenceNumber;
        CARD32  pad02;
        CARD32  pad03;
        CARD32  pad04;
        CARD32  pad05;
        CARD32  pad06;
        CARD32  pad07;
        CARD32  pad08;
} xTestFakeAckEvent;

/*
 * These are the definitions for key/button motion input actions.
 */
typedef struct {
        CARD8   header;         /* which device, key up/down */
        CARD8   keycode;        /* which key/button to move  */
        CARD16  delay_time;     /* how long to delay (in ms) */
} XTestKeyInfo;

/*
 * This is the definition for pointer jump input actions.
 */
typedef struct {
        CARD8   header;         /* which pointer             */
        CARD8   pad1;           /* unused padding byte       */
        CARD16  jumpx;          /* x coord to jump to        */
        CARD16  jumpy;          /* y coord to jump to        */
        CARD16  delay_time;     /* how long to delay (in ms) */
} XTestJumpInfo;

/*
 * These are the definitions for pointer relative motion input
 * actions.
 *
 * The sign bits for the x and y relative motions are contained
 * in the header byte.  The x and y relative motions are packed
 * into one byte to make things fit in 32 bits.  If the relative
 * motion range is larger than +/-15, use the pointer jump action.
 */

typedef struct {
        CARD8   header;         /* which pointer             */
        CARD8   motion_data;    /* x,y relative motion       */
        CARD16  delay_time;     /* how long to delay (in ms) */
} XTestMotionInfo;

/*
 * These are the definitions for a long delay input action.  It is
 * used when more than XTestSHORT_DELAY_TIME milliseconds of delay
 * (approximately one minute) is needed.
 *
 * The device ID for a delay is always set to XTestDELAY_DEVICE_ID.
 * This guarantees that a header byte with a value of 0 is not
 * a valid header, so it can be used as a flag to indicate that
 * there are no more input actions in an XTestInputAction event.
 */

typedef struct {
        CARD8   header;         /* always XTestDELAY_DEVICE_ID */
        CARD8   pad1;           /* unused padding byte         */
        CARD16  pad2;           /* unused padding word         */
        CARD32  delay_time;     /* how long to delay (in ms)   */
} XTestDelayInfo;

#endif /* _XTESTEXT1PROTO_H */
