/*
Copyright 1996, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included
in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall
not be used in advertising or otherwise to promote the sale, use or
other dealings in this Software without prior written authorization
from The Open Group.
*/

#ifndef _AG_H_
#define _AG_H_

#define XAGNAME "XC-APPGROUP"

#define XAG_MAJOR_VERSION	1	/* current version numbers */
#define XAG_MINOR_VERSION	0

#define XagWindowTypeX11	0
#define XagWindowTypeMacintosh	1
#define XagWindowTypeWin32	2
#define XagWindowTypeWin16	3

#define XagBadAppGroup			0
#define XagNumberErrors			(XagBadAppGroup + 1)

#define XagNsingleScreen		7
#define XagNdefaultRoot			1
#define XagNrootVisual			2
#define XagNdefaultColormap		3
#define XagNblackPixel			4
#define XagNwhitePixel			5
#define XagNappGroupLeader		6

#endif /* _AG_H_ */

