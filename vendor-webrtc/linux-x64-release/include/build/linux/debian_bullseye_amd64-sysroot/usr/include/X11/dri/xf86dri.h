/**************************************************************************

Copyright 1998-1999 Precision Insight, Inc., Cedar Park, Texas.
Copyright 2000 VA Linux Systems, Inc.
All Rights Reserved.

Permission is hereby granted, free of charge, to any person obtaining a
copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sub license, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice (including the
next paragraph) shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
IN NO EVENT SHALL PRECISION INSIGHT AND/OR ITS SUPPLIERS BE LIABLE FOR
ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

**************************************************************************/

/**
 * \file xf86dri.h
 * Protocol numbers and function prototypes for DRI X protocol.
 *
 * \author Kevin E. Martin <martin@valinux.com>
 * \author Jens Owen <jens@tungstengraphics.com>
 * \author Rickard E. (Rik) Faith <faith@valinux.com>
 */

#ifndef _XF86DRI_H_
#define _XF86DRI_H_

#include <xf86drm.h>

#define X_XF86DRIQueryVersion			0
#define X_XF86DRIQueryDirectRenderingCapable	1
#define X_XF86DRIOpenConnection			2
#define X_XF86DRICloseConnection		3
#define X_XF86DRIGetClientDriverName		4
#define X_XF86DRICreateContext			5
#define X_XF86DRIDestroyContext			6
#define X_XF86DRICreateDrawable			7
#define X_XF86DRIDestroyDrawable		8
#define X_XF86DRIGetDrawableInfo		9
#define X_XF86DRIGetDeviceInfo			10
#define X_XF86DRIAuthConnection                 11
#define X_XF86DRIOpenFullScreen                 12   /* Deprecated */
#define X_XF86DRICloseFullScreen                13   /* Deprecated */

#define XF86DRINumberEvents		0

#define XF86DRIClientNotLocal		0
#define XF86DRIOperationNotSupported	1
#define XF86DRINumberErrors		(XF86DRIOperationNotSupported + 1)

#endif /* _XF86DRI_H_ */

