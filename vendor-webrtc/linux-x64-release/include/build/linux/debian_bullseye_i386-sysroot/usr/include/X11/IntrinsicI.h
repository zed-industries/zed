/***********************************************************

Copyright 1987, 1988, 1994, 1998  The Open Group

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


Copyright 1987, 1988 by Digital Equipment Corporation, Maynard, Massachusetts.

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the name of Digital not be
used in advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

DIGITAL DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING
ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL
DIGITAL BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR
ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
SOFTWARE.

******************************************************************/

#ifndef _XtintrinsicI_h
#define _XtintrinsicI_h

#include "Xtos.h"
#include "IntrinsicP.h"
#ifdef WIN32
#define _WILLWINSOCK_
#endif
#include <X11/Xos.h>

#include "Object.h"
#include "RectObj.h"
#include "ObjectP.h"
#include "RectObjP.h"

#include "ConvertI.h"
#include "TranslateI.h"

#define RectObjClassFlag	0x02
#define WidgetClassFlag		0x04
#define CompositeClassFlag	0x08
#define ConstraintClassFlag	0x10
#define ShellClassFlag		0x20
#define WMShellClassFlag	0x40
#define TopLevelClassFlag	0x80

/*
 * The following macros, though very handy, are not suitable for
 * IntrinsicP.h as they violate the rule that arguments are to
 * be evaluated exactly once.
 */

#define XtDisplayOfObject(object) \
    (XtIsWidget(object) ? (object)->core.screen->display : \
    _XtIsHookObject(object) ? ((HookObject)(object))->hooks.screen->display : \
    _XtWindowedAncestor(object)->core.screen->display)

#define XtScreenOfObject(object) \
    (XtIsWidget(object) ? (object)->core.screen : \
    _XtIsHookObject(object) ? ((HookObject)(object))->hooks.screen : \
    _XtWindowedAncestor(object)->core.screen)

#define XtWindowOfObject(object) \
    ((XtIsWidget(object) ? (object) : _XtWindowedAncestor(object)) \
     ->core.window)

#define XtIsManaged(object) \
    (XtIsRectObj(object) ? (object)->core.managed : False)

#define XtIsSensitive(object) \
    (XtIsRectObj(object) ? ((object)->core.sensitive && \
			    (object)->core.ancestor_sensitive) : False)


/****************************************************************
 *
 * Bit utilities
 *
 ****************************************************************/
#define XtSetBits(dst,src,len)  dst = (((1U << (len)) - 1) & (unsigned)(src))
#define XtSetBit(dst,src)  XtSetBits(dst,src,1)

/****************************************************************
 *
 * Byte utilities
 *
 ****************************************************************/

#define _XBCOPYFUNC _XtBcopy
#include <X11/Xfuncs.h>

#define XtMemmove(dst, src, size)	\
    if ((const void *)(dst) != (const void *)(src)) {		    \
	(void) memcpy((void *) (dst), (const void *) (src), (size_t) (size)); \
    }

#define XtBZero(dst, size) 	\
	memset((void *) (dst), 0, (size_t) (size))

#define XtMemcmp(b1, b2, size) 		\
	memcmp((const void *) (b1), (const void *) (b2), (size_t) (size))


/****************************************************************
 *
 * Stack cache allocation/free
 *
 ****************************************************************/

#define XtStackAlloc(size, stack_cache_array)     \
    ((size) <= sizeof(stack_cache_array)	  \
    ?  (XtPointer)(stack_cache_array)		  \
    :  XtMalloc((Cardinal)(size)))

#define XtStackFree(pointer, stack_cache_array) \
    { if ((pointer) != ((XtPointer)(stack_cache_array))) XtFree(pointer); }

/***************************************************************
 *
 * Filename defines
 *
 **************************************************************/

/* used by XtResolvePathname */
#ifndef XFILESEARCHPATHDEFAULT
#define XFILESEARCHPATHDEFAULT "/usr/lib/X11/%L/%T/%N%S:/usr/lib/X11/%l/%T/%N%S:/usr/lib/X11/%T/%N%S"
#endif

/* the following two were both "X Toolkit " prior to R4 */
#ifndef XTERROR_PREFIX
#define XTERROR_PREFIX ""
#endif

#ifndef XTWARNING_PREFIX
#define XTWARNING_PREFIX ""
#endif

#ifndef ERRORDB
#define ERRORDB "/usr/lib/X11/XtErrorDB"
#endif

_XFUNCPROTOBEGIN

extern String XtCXtToolkitError;

extern void _XtAllocError(
    String	/* alloc_type */
) _X_NORETURN;

extern void _XtCompileResourceList(
    XtResourceList 	/* resources */,
    Cardinal 		/* num_resources */
);

extern XtGeometryResult _XtMakeGeometryRequest(
    Widget 		/* widget */,
    XtWidgetGeometry*	/* request */,
    XtWidgetGeometry*	/* reply_return */,
    Boolean*		/* clear_rect_obj */
);

extern Boolean _XtIsHookObject(
    Widget      /* widget */
);

extern void _XtAddShellToHookObj(
    Widget      /* widget */
);

/* GCManager.c */
extern void _XtGClistFree(Display *dpy, XtPerDisplay pd);

/** GeoTattler stuff */

#ifdef XT_GEO_TATTLER

extern void _XtGeoTab (int);
extern void _XtGeoTrace (
			    Widget widget,
			    const char *,
			    ...
) _X_ATTRIBUTE_PRINTF(2,3);

#define CALLGEOTAT(f) f

#else /* XT_GEO_TATTLER */

#define CALLGEOTAT(f)

#endif /* XT_GEO_TATTLER */

#ifndef XTTRACEMEMORY

extern char* __XtMalloc (
    unsigned	/* size */
);
extern char* __XtCalloc (
    unsigned	/* num */,
    unsigned	/* size */
);

#else

#define __XtMalloc XtMalloc
#define __XtCalloc XtCalloc
#endif

_XFUNCPROTOEND

#endif /* _XtintrinsicI_h */
/* DON'T ADD STUFF AFTER THIS #endif */
