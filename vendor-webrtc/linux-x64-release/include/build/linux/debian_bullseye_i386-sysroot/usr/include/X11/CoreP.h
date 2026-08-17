/***********************************************************

Copyright 1987, 1988, 1998  The Open Group

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

#ifndef XtCoreP_h
#define XtCoreP_h

#include <X11/Core.h>

_XFUNCPROTOBEGIN

externalref int _XtInheritTranslations;

#define XtInheritTranslations  ((String) &_XtInheritTranslations)
#define XtInheritRealize ((XtRealizeProc) _XtInherit)
#define XtInheritResize ((XtWidgetProc) _XtInherit)
#define XtInheritExpose ((XtExposeProc) _XtInherit)
#define XtInheritSetValuesAlmost ((XtAlmostProc) _XtInherit)
#define XtInheritAcceptFocus ((XtAcceptFocusProc) _XtInherit)
#define XtInheritQueryGeometry ((XtGeometryHandler) _XtInherit)
#define XtInheritDisplayAccelerator ((XtStringProc) _XtInherit)

/***************************************************************
 * Widget Core Data Structures
 *
 *
 **************************************************************/

typedef struct _CorePart {
    Widget	    self;		/* pointer to widget itself	     */
    WidgetClass	    widget_class;	/* pointer to Widget's ClassRec	     */
    Widget	    parent;		/* parent widget	  	     */
    XrmName         xrm_name;		/* widget resource name quarkified   */
    Boolean         being_destroyed;	/* marked for destroy		     */
    XtCallbackList  destroy_callbacks;	/* who to call when widget destroyed */
    XtPointer       constraints;        /* constraint record                 */
    Position        x, y;		/* window position		     */
    Dimension       width, height;	/* window dimensions		     */
    Dimension       border_width;	/* window border width		     */
    Boolean         managed;            /* is widget geometry managed?       */
    Boolean	    sensitive;		/* is widget sensitive to user events*/
    Boolean         ancestor_sensitive;	/* are all ancestors sensitive?      */
    XtEventTable    event_table;	/* private to event dispatcher       */
    XtTMRec	    tm;                 /* translation management            */
    XtTranslations  accelerators;       /* accelerator translations          */
    Pixel	    border_pixel;	/* window border pixel		     */
    Pixmap          border_pixmap;	/* window border pixmap or NULL      */
    WidgetList      popup_list;         /* list of popups                    */
    Cardinal        num_popups;         /* how many popups                   */
    String          name;		/* widget resource name		     */
    Screen	    *screen;		/* window's screen		     */
    Colormap        colormap;           /* colormap                          */
    Window	    window;		/* window ID			     */
    Cardinal        depth;		/* number of planes in window        */
    Pixel	    background_pixel;	/* window background pixel	     */
    Pixmap          background_pixmap;	/* window background pixmap or NULL  */
    Boolean         visible;		/* is window mapped and not occluded?*/
    Boolean	    mapped_when_managed;/* map window if it's managed?       */
} CorePart;

typedef struct _WidgetRec {
    CorePart    core;
 } WidgetRec, CoreRec;



/******************************************************************
 *
 * Core Class Structure. Widgets, regardless of their class, will have
 * these fields.  All widgets of a given class will have the same values
 * for these fields.  Widgets of a given class may also have additional
 * common fields.  These additional fields are included in incremental
 * class structures, such as CommandClass.
 *
 * The fields that are specific to this subclass, as opposed to fields that
 * are part of the superclass, are called "subclass fields" below.  Many
 * procedures are responsible only for the subclass fields, and not for
 * any superclass fields.
 *
 ********************************************************************/

typedef struct _CoreClassPart {
    WidgetClass     superclass;		/* pointer to superclass ClassRec   */
    String          class_name;		/* widget resource class name       */
    Cardinal        widget_size;	/* size in bytes of widget record   */
    XtProc	    class_initialize;   /* class initialization proc	    */
    XtWidgetClassProc class_part_initialize; /* dynamic initialization	    */
    XtEnum          class_inited;       /* has class been initialized?      */
    XtInitProc      initialize;		/* initialize subclass fields       */
    XtArgsProc      initialize_hook;    /* notify that initialize called    */
    XtRealizeProc   realize;		/* XCreateWindow for widget	    */
    XtActionList    actions;		/* widget semantics name to proc map */
    Cardinal	    num_actions;	/* number of entries in actions     */
    XtResourceList  resources;		/* resources for subclass fields    */
    Cardinal        num_resources;      /* number of entries in resources   */
    XrmClass        xrm_class;		/* resource class quarkified	    */
    Boolean         compress_motion;    /* compress MotionNotify for widget */
    XtEnum          compress_exposure;  /* compress Expose events for widget*/
    Boolean         compress_enterleave;/* compress enter and leave events  */
    Boolean         visible_interest;   /* select for VisibilityNotify      */
    XtWidgetProc    destroy;		/* free data for subclass pointers  */
    XtWidgetProc    resize;		/* geom manager changed widget size */
    XtExposeProc    expose;		/* rediplay window		    */
    XtSetValuesFunc set_values;		/* set subclass resource values     */
    XtArgsFunc      set_values_hook;    /* notify that set_values called    */
    XtAlmostProc    set_values_almost;  /* set_values got "Almost" geo reply */
    XtArgsProc      get_values_hook;    /* notify that get_values called    */
    XtAcceptFocusProc accept_focus;     /* assign input focus to widget     */
    XtVersionType   version;	        /* version of intrinsics used	    */
    XtPointer       callback_private;   /* list of callback offsets       */
    String          tm_table;           /* state machine                    */
    XtGeometryHandler query_geometry;	/* return preferred geometry        */
    XtStringProc    display_accelerator;/* display your accelerator	    */
    XtPointer	    extension;		/* pointer to extension record      */
 } CoreClassPart;

typedef struct _WidgetClassRec {
    CoreClassPart core_class;
} WidgetClassRec, CoreClassRec;

externalref WidgetClassRec widgetClassRec;
#define coreClassRec widgetClassRec

_XFUNCPROTOEND

#endif /* _XtCoreP_h */
/* DON'T ADD STUFF AFTER THIS #endif */
