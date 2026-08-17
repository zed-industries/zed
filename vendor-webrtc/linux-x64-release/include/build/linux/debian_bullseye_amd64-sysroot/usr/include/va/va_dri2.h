/*
 * Copyright © 2007,2008 Red Hat, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Soft-
 * ware"), to deal in the Software without restriction, including without
 * limitation the rights to use, copy, modify, merge, publish, distribute,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, provided that the above copyright
 * notice(s) and this permission notice appear in all copies of the Soft-
 * ware and that both the above copyright notice(s) and this permission
 * notice appear in supporting documentation.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABIL-
 * ITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY
 * RIGHTS. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN
 * THIS NOTICE BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSE-
 * QUENTIAL DAMAGES, OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFOR-
 * MANCE OF THIS SOFTWARE.
 *
 * Except as contained in this notice, the name of a copyright holder shall
 * not be used in advertising or otherwise to promote the sale, use or
 * other dealings in this Software without prior written authorization of
 * the copyright holder.
 *
 * Authors:
 *   Kristian Høgsberg (krh@redhat.com)
 */

#ifndef _VA_DRI2_H_
#define _VA_DRI2_H_

#include <X11/Xproto.h>
#include <X11/extensions/Xfixes.h>
#include <X11/Xfuncproto.h>
#include <xf86drm.h>

typedef struct {
    unsigned int attachment;
    unsigned int name;
    unsigned int pitch;
    unsigned int cpp;
    unsigned int flags;
} VA_DRI2Buffer;

extern Bool
VA_DRI2QueryExtension(Display *display, int *eventBase, int *errorBase);
extern Bool
VA_DRI2QueryVersion(Display *display, int *major, int *minor);
extern Bool
VA_DRI2Connect(Display *display, XID window,
               char **driverName, char **deviceName);
extern Bool
VA_DRI2Authenticate(Display *display, XID window, drm_magic_t magic);
extern void
VA_DRI2CreateDrawable(Display *display, XID drawable);
extern void
VA_DRI2DestroyDrawable(Display *display, XID handle);
extern VA_DRI2Buffer *
VA_DRI2GetBuffers(Display *dpy, XID drawable,
                  int *width, int *height,
                  unsigned int *attachments, int count,
                  int *outCount);
extern void
VA_DRI2CopyRegion(Display *dpy, XID drawable, XserverRegion region,
                  CARD32 dest, CARD32 src);
extern void
VA_DRI2SwapBuffers(Display *dpy, XID drawable, CARD64 target_msc, CARD64 divisor,
                   CARD64 remainder, CARD64 *count);
#endif
