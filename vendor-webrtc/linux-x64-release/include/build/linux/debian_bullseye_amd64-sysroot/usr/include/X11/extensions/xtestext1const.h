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

#ifndef _XTESTEXT1CONST_H
#define _XTESTEXT1CONST_H 1

#define XTestMAX_ACTION_LIST_SIZE       64
#define XTestACTIONS_SIZE	28


/*
 * used in the XTestPressButton and XTestPressKey functions
 */
#define XTestPRESS                      1 << 0
#define XTestRELEASE                    1 << 1
#define XTestSTROKE                     1 << 2

/*
 * When doing a key or button stroke, the number of milliseconds
 * to delay between the press and the release of a key or button
 * in the XTestPressButton and XTestPressKey functions.
 */

#define XTestSTROKE_DELAY_TIME		10

/*
 * used in the XTestGetInput function
 */
#define XTestEXCLUSIVE                  1 << 0
#define XTestPACKED_ACTIONS             1 << 1
#define XTestPACKED_MOTION              1 << 2

/*
 * used in the XTestFakeInput function
 */
#define XTestFAKE_ACK_NOT_NEEDED        0
#define XTestFAKE_ACK_REQUEST           1

/*
 * used in the XTest extension initialization routine
 */
#define XTestEXTENSION_NAME             "XTestExtension1"
#define XTestEVENT_COUNT                2

/*
 * This is the definition for the format of the header byte
 * in the input action structures.
 */
#define XTestACTION_TYPE_MASK   0x03    /* bits 0 and 1          */
#define XTestKEY_STATE_MASK     0x04    /* bit 2 (key action)    */
#define XTestX_SIGN_BIT_MASK    0x04    /* bit 2 (motion action) */
#define XTestY_SIGN_BIT_MASK    0x08    /* bit 3 (motion action) */
#define XTestDEVICE_ID_MASK     0xf0    /* bits 4 through 7      */

#define XTestMAX_DEVICE_ID	0x0f
#define XTestPackDeviceID(x)	(((x) & XTestMAX_DEVICE_ID) << 4)
#define XTestUnpackDeviceID(x)	(((x) & XTestDEVICE_ID_MASK) >> 4)

/*
 * These are the possible action types.
 */
#define XTestDELAY_ACTION       0
#define XTestKEY_ACTION         1
#define XTestMOTION_ACTION      2
#define XTestJUMP_ACTION        3

/*
 * These are the definitions for key/button motion input actions.
 */
#define XTestKEY_UP             0x04
#define XTestKEY_DOWN           0x00

/*
 * These are the definitions for pointer relative motion input
 * actions.
 *
 * The sign bits for the x and y relative motions are contained
 * in the header byte.  The x and y relative motions are packed
 * into one byte to make things fit in 32 bits.  If the relative
 * motion range is larger than +/-15, use the pointer jump action.
 */
#define XTestMOTION_MAX            15
#define XTestMOTION_MIN            -15

#define XTestX_NEGATIVE            0x04
#define XTestY_NEGATIVE            0x08

#define XTestX_MOTION_MASK         0x0f
#define XTestY_MOTION_MASK         0xf0

#define XTestPackXMotionValue(x)   ((x) & XTestX_MOTION_MASK)
#define XTestPackYMotionValue(x)   (((x) << 4) & XTestY_MOTION_MASK)

#define XTestUnpackXMotionValue(x) ((x) & XTestX_MOTION_MASK)
#define XTestUnpackYMotionValue(x) (((x) & XTestY_MOTION_MASK) >> 4)
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

#define XTestSHORT_DELAY_TIME	0xffff
#define XTestDELAY_DEVICE_ID    0x0f

#endif /* _XTESTEXT1CONST_H */
