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

#ifndef _XTESTEXT1_H
#define _XTESTEXT1_H
/*
 * the typedefs for CARD8, CARD16, and CARD32 are defined in Xmd.h
 */

#include <X11/extensions/xtestext1const.h>
/*
 * This is the definition for the input action host format event structure.
 * This is the form that a client using this extension will see when
 * it receives an input action event.
 */
typedef struct {
        int     type;           /* always XTestInputActionType */
	Display *display;
	Window  window;
        CARD8   actions[XTestACTIONS_SIZE];
} XTestInputActionEvent;

/*
 * This is the definition for the xTestFakeAck host format event structure.
 * This is the form that a client using this extension will see when
 * it receives an XTestFakeAck event.
 */
typedef struct {
        int     type;           /* always XTestFakeAckType */
	Display *display;
	Window  window;
} XTestFakeAckEvent;

_XFUNCPROTOBEGIN

int XTestFakeInput(register Display *dpy, char *action_list_addr, int action_list_size, int ack_flag);
int XTestGetInput(register Display *dpy, int action_handling);
int XTestQueryInputSize(register Display *dpy, unsigned long *size_return);
int XTestPressKey(Display *display, int device_id, unsigned long delay, unsigned int keycode, unsigned int key_action);
int XTestPressButton(Display * display, int device_id, unsigned long delay, unsigned int button_number, unsigned int button_action);
int XTestMovePointer(Display *display, int device_id, unsigned long delay[], int x[], int y[], unsigned int count);
int XTestFlush(Display *display);
int XTestStopInput(register Display *dpy);
int XTestReset(register Display *dpy);

_XFUNCPROTOEND

#endif /* _XTESTEXT1_H */
