/*
   Copyright (c) 1999  XFree86 Inc
*/

#ifndef _XF86DGACONST_H_
#define _XF86DGACONST_H_

#include <X11/extensions/xf86dga1const.h>

#define X_XDGAQueryVersion		0

/* 1 through 9 are in xf86dga1.h */

/* 10 and 11 are reserved to avoid conflicts with rogue DGA extensions */

#define X_XDGAQueryModes		12
#define X_XDGASetMode			13
#define X_XDGASetViewport		14
#define X_XDGAInstallColormap		15
#define X_XDGASelectInput		16
#define X_XDGAFillRectangle		17
#define X_XDGACopyArea			18
#define X_XDGACopyTransparentArea	19
#define X_XDGAGetViewportStatus		20
#define X_XDGASync			21
#define X_XDGAOpenFramebuffer		22
#define X_XDGACloseFramebuffer		23
#define X_XDGASetClientVersion		24
#define X_XDGAChangePixmapMode		25
#define X_XDGACreateColormap		26


#define XDGAConcurrentAccess	0x00000001
#define XDGASolidFillRect	0x00000002
#define XDGABlitRect		0x00000004
#define XDGABlitTransRect	0x00000008
#define XDGAPixmap    		0x00000010

#define XDGAInterlaced          0x00010000
#define XDGADoublescan          0x00020000

#define XDGAFlipImmediate	0x00000001
#define XDGAFlipRetrace		0x00000002

#define XDGANeedRoot		0x00000001

#define XF86DGANumberEvents		7

#define XDGAPixmapModeLarge		0
#define XDGAPixmapModeSmall		1

#define XF86DGAClientNotLocal		0
#define XF86DGANoDirectVideoMode	1
#define XF86DGAScreenNotActive		2
#define XF86DGADirectNotActivated	3
#define XF86DGAOperationNotSupported	4
#define XF86DGANumberErrors		(XF86DGAOperationNotSupported + 1)


typedef struct {
   int num;		/* A unique identifier for the mode (num > 0) */
   char *name;		/* name of mode given in the XF86Config */
   float verticalRefresh;
   int flags;		/* DGA_CONCURRENT_ACCESS, etc... */
   int imageWidth;	/* linear accessible portion (pixels) */
   int imageHeight;
   int pixmapWidth;	/* Xlib accessible portion (pixels) */
   int pixmapHeight;	/* both fields ignored if no concurrent access */
   int bytesPerScanline;
   int byteOrder;	/* MSBFirst, LSBFirst */
   int depth;
   int bitsPerPixel;
   unsigned long redMask;
   unsigned long greenMask;
   unsigned long blueMask;
   short visualClass;
   int viewportWidth;
   int viewportHeight;
   int xViewportStep;	/* viewport position granularity */
   int yViewportStep;
   int maxViewportX;	/* max viewport origin */
   int maxViewportY;
   int viewportFlags;	/* types of page flipping possible */
   int reserved1;
   int reserved2;
} XDGAMode;


typedef struct {
   XDGAMode mode;
   unsigned char *data;
   Pixmap pixmap;
} XDGADevice;


#endif /* _XF86DGACONST_H_ */
