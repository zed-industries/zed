/************************************************************

Copyright (c) 1993, Oracle and/or its affiliates. All rights reserved.

Permission is hereby granted, free of charge, to any person obtaining a
copy of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice (including the next
paragraph) shall be included in all copies or substantial portions of the
Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

********************************************************/

/*

Copyright 1994, 1998  The Open Group

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

*/
/* $XFree86: xc/lib/Xt/ThreadsI.h,v 3.5 2001/12/14 19:56:31 dawes Exp $ */

#ifndef _XtThreadsI_h
#define _XtThreadsI_h

#include <X11/XlibConf.h>

#ifdef XTHREADS

typedef struct _LockRec *LockPtr;

typedef void (*ThreadAppProc)(
    XtAppContext /* app */
);

typedef void (*ThreadAppYieldLockProc)(
    XtAppContext, /* app */
    Boolean*, /* push_thread */
    Boolean*, /* pushed_thread */
    int* /* level */
);

typedef void (*ThreadAppRestoreLockProc)(
    XtAppContext /* app */,
    int, /* level */
    Boolean* /* pushed_thread */
);

_XFUNCPROTOBEGIN

extern void (*_XtProcessLock)(
    void
);

extern void (*_XtProcessUnlock)(
    void
);

extern void (*_XtInitAppLock)(
    XtAppContext /* app */
);

_XFUNCPROTOEND

#define INIT_APP_LOCK(app) if(_XtInitAppLock) (*_XtInitAppLock)(app)
#define FREE_APP_LOCK(app) if(app && app->free_lock)(*app->free_lock)(app)

#define LOCK_PROCESS if(_XtProcessLock)(*_XtProcessLock)()
#define UNLOCK_PROCESS if(_XtProcessUnlock)(*_XtProcessUnlock)()
#define LOCK_APP(app) if(app && app->lock)(*app->lock)(app)
#define UNLOCK_APP(app) if(app && app->unlock)(*app->unlock)(app)

#define YIELD_APP_LOCK(app,push,pushed,level)\
	 if(app && app->yield_lock) (*app->yield_lock)(app,push,pushed,level)
#define RESTORE_APP_LOCK(app,level,pushed)\
	 if(app && app->restore_lock) (*app->restore_lock)(app,level,pushed)

#define WIDGET_TO_APPCON(w) \
    XtAppContext app = (w && _XtProcessLock ? \
	XtWidgetToApplicationContext(w) : NULL)

#define DPY_TO_APPCON(d) \
    XtAppContext app = (_XtProcessLock ? XtDisplayToApplicationContext(d): NULL)

#else /* defined(XTHREADS) */

#define LOCK_PROCESS
#define UNLOCK_PROCESS
#define LOCK_APP(app)
#define UNLOCK_APP(app)

#define INIT_APP_LOCK(app)
#define FREE_APP_LOCK(app)

#define WIDGET_TO_APPCON(w)
#define DPY_TO_APPCON(d)

#endif /* !defined(XTHREADS) */
#endif /* _XtThreadsI_h */
