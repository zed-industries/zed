/*
Copyright (c) 1992  X Consortium

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
X CONSORTIUM BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of the X Consortium shall not be
used in advertising or otherwise to promote the sale, use or other dealings
in this Software without prior written authorization from the X Consortium.
 *
 * Author:  Keith Packard, MIT X Consortium
 */

#ifndef _SAVER_H_
#define _SAVER_H_

#define ScreenSaverName	"MIT-SCREEN-SAVER"
#define ScreenSaverPropertyName "_MIT_SCREEN_SAVER_ID"

#define ScreenSaverNotifyMask	0x00000001
#define ScreenSaverCycleMask	0x00000002

#define ScreenSaverMajorVersion	1
#define ScreenSaverMinorVersion	1

#define ScreenSaverOff		0
#define ScreenSaverOn		1
#define ScreenSaverCycle	2
#define ScreenSaverDisabled	3

#define ScreenSaverBlanked	0
#define ScreenSaverInternal	1
#define ScreenSaverExternal	2

#define ScreenSaverNotify	0
#define ScreenSaverNumberEvents	1

#endif /* _SAVER_H_ */
