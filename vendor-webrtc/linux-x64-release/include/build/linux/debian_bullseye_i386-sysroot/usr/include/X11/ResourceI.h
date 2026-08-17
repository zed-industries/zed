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

/****************************************************************
 *
 * Resources
 *
 ****************************************************************/

#ifndef _XtresourceI_h
#define _XtresourceI_h

#define StringToQuark(string) XrmStringToQuark(string)
#define StringToName(string) XrmStringToName(string)
#define StringToClass(string) XrmStringToClass(string)

_XFUNCPROTOBEGIN

extern void _XtDependencies(
    XtResourceList  * /* class_resp */,
    Cardinal	    * /* class_num_resp */,
    XrmResourceList * /* super_res */,
    Cardinal	     /* super_num_res */,
    Cardinal	     /* super_widget_size */);

extern void _XtResourceDependencies(
    WidgetClass  /* wc */
);

extern void _XtConstraintResDependencies(
    ConstraintWidgetClass  /* wc */
);

extern XtCacheRef* _XtGetResources(
    Widget	    /* w */,
    ArgList	    /* args */,
    Cardinal	    /* num_args */,
    XtTypedArgList  /* typed_args */,
    Cardinal*	    /* num_typed_args */
);

extern void _XtCopyFromParent(
    Widget		/* widget */,
    int			/* offset */,
    XrmValue*		/* value */
);

extern void _XtCopyToArg(char *src, XtArgVal *dst, unsigned int size);
extern void _XtCopyFromArg(XtArgVal src, char *dst, unsigned int size);
extern XrmResourceList* _XtCreateIndirectionTable(XtResourceList resources,
						  Cardinal num_resources);
extern void _XtResourceListInitialize(void);

_XFUNCPROTOEND

#endif /* _XtresourceI_h */
