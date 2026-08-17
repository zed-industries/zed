/**************************************************************************

Copyright (c) 2002 Apple Computer, Inc.
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

#ifndef _APPLEWMCONST_H_
#define _APPLEWMCONST_H_

/* Events */
#define AppleWMControllerNotify		0
#define AppleWMActivationNotify		1
#define AppleWMPasteboardNotify		2
#define AppleWMNumberEvents		3

#define AppleWMControllerNotifyMask	(1L << 0)
#define AppleWMActivationNotifyMask	(1L << 1)
#define AppleWMPasteboardNotifyMask	(1L << 2)

/* "Kinds" of ControllerNotify events */
#define AppleWMMinimizeWindow		0
#define AppleWMZoomWindow		1
#define AppleWMCloseWindow		2
#define AppleWMBringAllToFront		3
#define AppleWMHideWindow		4
#define AppleWMHideAll			5
#define AppleWMShowAll			6
#define AppleWMWindowMenuItem		9
#define AppleWMWindowMenuNotify		10
#define AppleWMNextWindow		11
#define AppleWMPreviousWindow		12

/* "Kinds" of ActivationNotify events */
#define AppleWMIsActive			0
#define AppleWMIsInactive		1
#define AppleWMReloadPreferences	2

/* "Kinds" of PasteboardNotify events */
#define AppleWMCopyToPasteboard		0

/* Errors */
#define AppleWMClientNotLocal		0
#define AppleWMOperationNotSupported	1
#define AppleWMNumberErrors		(AppleWMOperationNotSupported + 1)

/* Window level ids */
#define AppleWMWindowLevelNormal	0
#define AppleWMWindowLevelFloating	1
#define AppleWMWindowLevelTornOff	2
#define AppleWMWindowLevelDock		3
#define AppleWMWindowLevelDesktop	4
#define AppleWMNumWindowLevels		5

/* Possible value for frame_rect argument to XAppleWMFrameGetRect() */
/* Use xp_frame_rect_enum from <Xplugin.h> */

/* Window frame classes */
/* Use xp_frame_class_enum from <Xplugin.h> */

/* Window frame attributes */
/* Use xp_frame_attr_enum from <Xplugin.h> */

#endif /* _APPLEWMCONST_H_ */
