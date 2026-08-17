/***********************************************************
Copyright 1987, 1988 by Digital Equipment Corporation, Maynard, Massachusetts,

			All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the name Digital not be
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

/*

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

*/

#ifndef _XtIntrinsic_h
#define _XtIntrinsic_h

#include	<X11/Xlib.h>
#include	<X11/Xutil.h>
#include	<X11/Xresource.h>
#include	<X11/Xfuncproto.h>
#ifdef XT_BC
#include <X11/Xos.h>		/* for R4 compatibility */
#else
#include <X11/Xosdefs.h>
#include <string.h>		/* for XtNewString */
#endif /* XT_BC else */

#define XtSpecificationRelease 7

/*
 * As used in its function interface, the String type of libXt can be readonly.
 * But compiling libXt with this feature may require some internal changes,
 * e.g., casts and occasionally using a plain "char*".
 */
#ifdef _CONST_X_STRING
typedef const char *String;
#else
typedef char *String;
#endif

/* We do this in order to get "const" declarations to work right.  We
 * use _XtString instead of String so that C++ applications can
 * #define String to something else if they choose, to avoid conflicts
 * with other C++ libraries.
 */
#define _XtString char*

/* _Xt names are private to Xt implementation, do not use in client code */
#if NeedWidePrototypes
#define _XtBoolean	int
#define _XtDimension	unsigned int
#define _XtKeyCode	unsigned int
#define _XtPosition	int
#define _XtXtEnum	unsigned int
#else
#define _XtBoolean	Boolean
#define _XtDimension	Dimension
#define _XtKeyCode	KeyCode
#define _XtPosition	Position
#define _XtXtEnum	XtEnum
#endif /* NeedWidePrototypes */

#include <stddef.h>

#ifdef VMS
#define externalref globalref
#define externaldef(psect) globaldef {"psect"} noshare
#else
#define externalref extern
#define externaldef(psect)
#endif /* VMS */

#ifndef FALSE
#define FALSE 0
#define TRUE 1
#endif

#define XtNumber(arr)		((Cardinal) (sizeof(arr) / sizeof(arr[0])))

typedef struct _WidgetRec *Widget;
typedef Widget *WidgetList;
typedef struct _WidgetClassRec *WidgetClass;
typedef struct _CompositeRec *CompositeWidget;
typedef struct _XtActionsRec *XtActionList;
typedef struct _XtEventRec *XtEventTable;

typedef struct _XtAppStruct *XtAppContext;
typedef unsigned long	XtValueMask;
typedef unsigned long	XtIntervalId;
typedef unsigned long	XtInputId;
typedef unsigned long	XtWorkProcId;
typedef unsigned long	XtSignalId;
typedef unsigned int	XtGeometryMask;
typedef unsigned long	XtGCMask;   /* Mask of values that are used by widget*/
typedef unsigned long	Pixel;	    /* Index into colormap		*/
typedef int		XtCacheType;
#define			XtCacheNone	  0x001
#define			XtCacheAll	  0x002
#define			XtCacheByDisplay  0x003
#define			XtCacheRefCount	  0x100

/****************************************************************
 *
 * System Dependent Definitions; see spec for specific range
 * requirements.  Do not assume every implementation uses the
 * same base types!
 *
 *
 * XtArgVal ought to be a union of XtPointer, char *, long, int *, and proc *
 * but casting to union types is not really supported.
 *
 * So the typedef for XtArgVal should be chosen such that
 *
 *	sizeof (XtArgVal) >=	sizeof(XtPointer)
 *				sizeof(char *)
 *				sizeof(long)
 *				sizeof(int *)
 *				sizeof(proc *)
 *
 * ArgLists rely heavily on the above typedef.
 *
 ****************************************************************/
typedef char		Boolean;
typedef long		XtArgVal;
typedef unsigned char	XtEnum;

typedef unsigned int	Cardinal;
typedef unsigned short	Dimension;  /* Size in pixels			*/
typedef short		Position;   /* Offset from 0 coordinate		*/

typedef void*		XtPointer;

/* The type Opaque is NOT part of the Xt standard, do NOT use it. */
/* (It remains here only for backward compatibility.) */
typedef XtPointer	Opaque;

#include <X11/Core.h>
#include <X11/Composite.h>
#include <X11/Constraint.h>
#include <X11/Object.h>
#include <X11/RectObj.h>

typedef struct _TranslationData *XtTranslations;
typedef struct _TranslationData *XtAccelerators;
typedef unsigned int Modifiers;

typedef void (*XtActionProc)(
    Widget 		/* widget */,
    XEvent*		/* event */,
    String*		/* params */,
    Cardinal*		/* num_params */
);

typedef XtActionProc* XtBoundActions;

typedef struct _XtActionsRec{
    String	 string;
    XtActionProc proc;
} XtActionsRec;

typedef enum {
/* address mode		parameter representation    */
/* ------------		------------------------    */
    XtAddress,		/* address		    */
    XtBaseOffset,	/* offset		    */
    XtImmediate,	/* constant		    */
    XtResourceString,	/* resource name string	    */
    XtResourceQuark,	/* resource name quark	    */
    XtWidgetBaseOffset,	/* offset from ancestor	    */
    XtProcedureArg	/* procedure to invoke	    */
} XtAddressMode;

typedef struct {
    XtAddressMode   address_mode;
    XtPointer	    address_id;
    Cardinal	    size;
} XtConvertArgRec, *XtConvertArgList;

typedef void (*XtConvertArgProc)(
    Widget 		/* widget */,
    Cardinal*		/* size */,
    XrmValue*		/* value */
);

typedef struct {
    XtGeometryMask request_mode;
    Position x, y;
    Dimension width, height, border_width;
    Widget sibling;
    int stack_mode;   /* Above, Below, TopIf, BottomIf, Opposite, DontChange */
} XtWidgetGeometry;

/* Additions to Xlib geometry requests: ask what would happen, don't do it */
#define XtCWQueryOnly	(1 << 7)

/* Additions to Xlib stack modes: don't change stack order */
#define XtSMDontChange	5

typedef void (*XtConverter)( /* obsolete */
    XrmValue*		/* args */,
    Cardinal*		/* num_args */,
    XrmValue*		/* from */,
    XrmValue*		/* to */
);

typedef Boolean (*XtTypeConverter)(
    Display*		/* dpy */,
    XrmValue*		/* args */,
    Cardinal*		/* num_args */,
    XrmValue*		/* from */,
    XrmValue*		/* to */,
    XtPointer*		/* converter_data */
);

typedef void (*XtDestructor)(
    XtAppContext	/* app */,
    XrmValue*		/* to */,
    XtPointer 		/* converter_data */,
    XrmValue*		/* args */,
    Cardinal*		/* num_args */
);

typedef Opaque XtCacheRef;

typedef Opaque XtActionHookId;

typedef void (*XtActionHookProc)(
    Widget		/* w */,
    XtPointer		/* client_data */,
    String		/* action_name */,
    XEvent*		/* event */,
    String*		/* params */,
    Cardinal*		/* num_params */
);

typedef unsigned long XtBlockHookId;

typedef void (*XtBlockHookProc)(
    XtPointer		/* client_data */
);

typedef void (*XtKeyProc)(
    Display*		/* dpy */,
    _XtKeyCode 		/* keycode */,
    Modifiers		/* modifiers */,
    Modifiers*		/* modifiers_return */,
    KeySym*		/* keysym_return */
);

typedef void (*XtCaseProc)(
    Display*		/* display */,
    KeySym		/* keysym */,
    KeySym*		/* lower_return */,
    KeySym*		/* upper_return */
);

typedef void (*XtEventHandler)(
    Widget 		/* widget */,
    XtPointer 		/* closure */,
    XEvent*		/* event */,
    Boolean*		/* continue_to_dispatch */
);
typedef unsigned long EventMask;

typedef enum {XtListHead, XtListTail } XtListPosition;

typedef unsigned long	XtInputMask;
#define XtInputNoneMask		0L
#define XtInputReadMask		(1L<<0)
#define XtInputWriteMask	(1L<<1)
#define XtInputExceptMask	(1L<<2)

typedef void (*XtTimerCallbackProc)(
    XtPointer 		/* closure */,
    XtIntervalId*	/* id */
);

typedef void (*XtInputCallbackProc)(
    XtPointer 		/* closure */,
    int*		/* source */,
    XtInputId*		/* id */
);

typedef void (*XtSignalCallbackProc)(
    XtPointer		/* closure */,
    XtSignalId*		/* id */
);

typedef struct {
    String	name;
    XtArgVal	value;
} Arg, *ArgList;

typedef XtPointer	XtVarArgsList;

typedef void (*XtCallbackProc)(
    Widget 		/* widget */,
    XtPointer 		/* closure */,	/* data the application registered */
    XtPointer 		/* call_data */	/* callback specific data */
);

typedef struct _XtCallbackRec {
    XtCallbackProc  callback;
    XtPointer	    closure;
} XtCallbackRec, *XtCallbackList;

typedef enum {
	XtCallbackNoList,
	XtCallbackHasNone,
	XtCallbackHasSome
} XtCallbackStatus;

typedef enum  {
    XtGeometryYes,	  /* Request accepted. */
    XtGeometryNo,	  /* Request denied. */
    XtGeometryAlmost,	  /* Request denied, but willing to take replyBox. */
    XtGeometryDone	  /* Request accepted and done. */
} XtGeometryResult;

typedef enum {XtGrabNone, XtGrabNonexclusive, XtGrabExclusive} XtGrabKind;

typedef struct {
    Widget  shell_widget;
    Widget  enable_widget;
} XtPopdownIDRec, *XtPopdownID;

typedef struct _XtResource {
    String	resource_name;	/* Resource name			    */
    String	resource_class;	/* Resource class			    */
    String	resource_type;	/* Representation type desired		    */
    Cardinal	resource_size;	/* Size in bytes of representation	    */
    Cardinal	resource_offset;/* Offset from base to put resource value   */
    String	default_type;	/* representation type of specified default */
    XtPointer	default_addr;	/* Address of default resource		    */
} XtResource, *XtResourceList;

typedef void (*XtResourceDefaultProc)(
    Widget	/* widget */,
    int		/* offset */,
    XrmValue*	/* value */
);

typedef String (*XtLanguageProc)(
    Display*	/* dpy */,
    String	/* xnl */,
    XtPointer	/* client_data */
);

typedef void (*XtErrorMsgHandler)(
    String 		/* name */,
    String		/* type */,
    String		/* class */,
    String		/* default */,
    String*		/* params */,
    Cardinal*		/* num_params */
);

typedef void (*XtErrorHandler)(
  String		/* msg */
);

typedef void (*XtCreatePopupChildProc)(
    Widget	/* shell */
);

typedef Boolean (*XtWorkProc)(
    XtPointer 		/* closure */	/* data the application registered */
);

typedef struct {
    char match;
    _XtString substitution;
} SubstitutionRec, *Substitution;

typedef Boolean (*XtFilePredicate)(
   String /* filename */
);

typedef XtPointer XtRequestId;

typedef Boolean (*XtConvertSelectionProc)(
    Widget 		/* widget */,
    Atom*		/* selection */,
    Atom*		/* target */,
    Atom*		/* type_return */,
    XtPointer*		/* value_return */,
    unsigned long*	/* length_return */,
    int*		/* format_return */
);

typedef void (*XtLoseSelectionProc)(
    Widget 		/* widget */,
    Atom*		/* selection */
);

typedef void (*XtSelectionDoneProc)(
    Widget 		/* widget */,
    Atom*		/* selection */,
    Atom*		/* target */
);

typedef void (*XtSelectionCallbackProc)(
    Widget 		/* widget */,
    XtPointer 		/* closure */,
    Atom*		/* selection */,
    Atom*		/* type */,
    XtPointer 		/* value */,
    unsigned long*	/* length */,
    int*		/* format */
);

typedef void (*XtLoseSelectionIncrProc)(
    Widget 		/* widget */,
    Atom*		/* selection */,
    XtPointer 		/* client_data */
);

typedef void (*XtSelectionDoneIncrProc)(
    Widget 		/* widget */,
    Atom*		/* selection */,
    Atom*		/* target */,
    XtRequestId*	/* receiver_id */,
    XtPointer 		/* client_data */
);

typedef Boolean (*XtConvertSelectionIncrProc)(
    Widget 		/* widget */,
    Atom*		/* selection */,
    Atom*		/* target */,
    Atom*		/* type */,
    XtPointer*		/* value */,
    unsigned long*	/* length */,
    int*		/* format */,
    unsigned long*	/* max_length */,
    XtPointer 		/* client_data */,
    XtRequestId*	/* receiver_id */
);

typedef void (*XtCancelConvertSelectionProc)(
    Widget 		/* widget */,
    Atom*		/* selection */,
    Atom*		/* target */,
    XtRequestId*	/* receiver_id */,
    XtPointer 		/* client_data */
);

typedef Boolean (*XtEventDispatchProc)(
    XEvent*		/* event */
);

typedef void (*XtExtensionSelectProc)(
    Widget		/* widget */,
    int*		/* event_types */,
    XtPointer*		/* select_data */,
    int			/* count */,
    XtPointer		/* client_data */
);

/***************************************************************
 *
 * Exported Interfaces
 *
 ****************************************************************/

_XFUNCPROTOBEGIN

extern Boolean XtConvertAndStore(
    Widget 		/* widget */,
    _Xconst _XtString 	/* from_type */,
    XrmValue*		/* from */,
    _Xconst _XtString 	/* to_type */,
    XrmValue*		/* to_in_out */
);

extern Boolean XtCallConverter(
    Display*		/* dpy */,
    XtTypeConverter 	/* converter */,
    XrmValuePtr 	/* args */,
    Cardinal 		/* num_args */,
    XrmValuePtr 	/* from */,
    XrmValue*		/* to_in_out */,
    XtCacheRef*		/* cache_ref_return */
);

extern Boolean XtDispatchEvent(
    XEvent* 		/* event */
);

extern Boolean XtCallAcceptFocus(
    Widget 		/* widget */,
    Time*		/* time */
);

extern Boolean XtPeekEvent( /* obsolete */
    XEvent*		/* event_return */
);

extern Boolean XtAppPeekEvent(
    XtAppContext 	/* app_context */,
    XEvent*		/* event_return */
);

extern Boolean XtIsSubclass(
    Widget 		/* widget */,
    WidgetClass 	/* widgetClass */
);

extern Boolean XtIsObject(
    Widget 		/* object */
);

extern Boolean _XtCheckSubclassFlag( /* implementation-private */
    Widget		/* object */,
    _XtXtEnum		/* type_flag */
);

extern Boolean _XtIsSubclassOf( /* implementation-private */
    Widget		/* object */,
    WidgetClass		/* widget_class */,
    WidgetClass		/* flag_class */,
    _XtXtEnum		/* type_flag */
);

extern Boolean XtIsManaged(
    Widget 		/* rectobj */
);

extern Boolean XtIsRealized(
    Widget 		/* widget */
);

extern Boolean XtIsSensitive(
    Widget 		/* widget */
);

extern Boolean XtOwnSelection(
    Widget 		/* widget */,
    Atom 		/* selection */,
    Time 		/* time */,
    XtConvertSelectionProc /* convert */,
    XtLoseSelectionProc	/* lose */,
    XtSelectionDoneProc /* done */
);

extern Boolean XtOwnSelectionIncremental(
    Widget 		/* widget */,
    Atom 		/* selection */,
    Time 		/* time */,
    XtConvertSelectionIncrProc	/* convert_callback */,
    XtLoseSelectionIncrProc	/* lose_callback */,
    XtSelectionDoneIncrProc	/* done_callback */,
    XtCancelConvertSelectionProc /* cancel_callback */,
    XtPointer 		/* client_data */
);

extern XtGeometryResult XtMakeResizeRequest(
    Widget 		/* widget */,
    _XtDimension	/* width */,
    _XtDimension	/* height */,
    Dimension*		/* width_return */,
    Dimension*		/* height_return */
);

extern void XtTranslateCoords(
    Widget 		/* widget */,
    _XtPosition		/* x */,
    _XtPosition		/* y */,
    Position*		/* rootx_return */,
    Position*		/* rooty_return */
);

extern KeySym* XtGetKeysymTable(
    Display*		/* dpy */,
    KeyCode*		/* min_keycode_return */,
    int*		/* keysyms_per_keycode_return */
);

extern void XtKeysymToKeycodeList(
    Display*		/* dpy */,
    KeySym 		/* keysym */,
    KeyCode**		/* keycodes_return */,
    Cardinal*		/* keycount_return */
);

extern void XtStringConversionWarning( /* obsolete */
    _Xconst _XtString	/* from_value */,
    _Xconst _XtString	/* to_type */
);

extern void XtDisplayStringConversionWarning(
    Display*	 	/* dpy */,
    _Xconst _XtString	/* from_value */,
    _Xconst _XtString	/* to_type */
);

externalref XtConvertArgRec const colorConvertArgs[];
externalref XtConvertArgRec const screenConvertArg[];

extern void XtAppAddConverter( /* obsolete */
    XtAppContext	/* app_context */,
    _Xconst _XtString	/* from_type */,
    _Xconst _XtString	/* to_type */,
    XtConverter 	/* converter */,
    XtConvertArgList	/* convert_args */,
    Cardinal 		/* num_args */
);

extern void XtAddConverter( /* obsolete */
    _Xconst _XtString	/* from_type */,
    _Xconst _XtString 	/* to_type */,
    XtConverter 	/* converter */,
    XtConvertArgList 	/* convert_args */,
    Cardinal 		/* num_args */
);

extern void XtSetTypeConverter(
    _Xconst _XtString 	/* from_type */,
    _Xconst _XtString 	/* to_type */,
    XtTypeConverter 	/* converter */,
    XtConvertArgList 	/* convert_args */,
    Cardinal 		/* num_args */,
    XtCacheType 	/* cache_type */,
    XtDestructor 	/* destructor */
);

extern void XtAppSetTypeConverter(
    XtAppContext 	/* app_context */,
    _Xconst _XtString 	/* from_type */,
    _Xconst _XtString 	/* to_type */,
    XtTypeConverter 	/* converter */,
    XtConvertArgList 	/* convert_args */,
    Cardinal 		/* num_args */,
    XtCacheType 	/* cache_type */,
    XtDestructor 	/* destructor */
);

extern void XtConvert( /* obsolete */
    Widget 		/* widget */,
    _Xconst _XtString 	/* from_type */,
    XrmValue*		/* from */,
    _Xconst _XtString 	/* to_type */,
    XrmValue*		/* to_return */
);

extern void XtDirectConvert( /* obsolete */
    XtConverter 	/* converter */,
    XrmValuePtr 	/* args */,
    Cardinal 		/* num_args */,
    XrmValuePtr 	/* from */,
    XrmValue*		/* to_return */
);

/****************************************************************
 *
 * Translation Management
 *
 ****************************************************************/

extern XtTranslations XtParseTranslationTable(
    _Xconst _XtString	/* table */
);

extern XtAccelerators XtParseAcceleratorTable(
    _Xconst _XtString	/* source */
);

extern void XtOverrideTranslations(
    Widget 		/* widget */,
    XtTranslations 	/* translations */
);

extern void XtAugmentTranslations(
    Widget 		/* widget */,
    XtTranslations 	/* translations */
);

extern void XtInstallAccelerators(
    Widget 		/* destination */,
    Widget		/* source */
);

extern void XtInstallAllAccelerators(
    Widget 		/* destination */,
    Widget		/* source */
);

extern void XtUninstallTranslations(
    Widget 		/* widget */
);

extern void XtAppAddActions(
    XtAppContext 	/* app_context */,
    XtActionList 	/* actions */,
    Cardinal 		/* num_actions */
);

extern void XtAddActions( /* obsolete */
    XtActionList 	/* actions */,
    Cardinal 		/* num_actions */
);

extern XtActionHookId XtAppAddActionHook(
    XtAppContext 	/* app_context */,
    XtActionHookProc 	/* proc */,
    XtPointer 		/* client_data */
);

extern void XtRemoveActionHook(
    XtActionHookId 	/* id */
);

extern void XtGetActionList(
    WidgetClass		/* widget_class */,
    XtActionList*	/* actions_return */,
    Cardinal*		/* num_actions_return */
);

extern void XtCallActionProc(
    Widget		/* widget */,
    _Xconst _XtString	/* action */,
    XEvent*		/* event */,
    String*		/* params */,
    Cardinal		/* num_params */
);

extern void XtRegisterGrabAction(
    XtActionProc 	/* action_proc */,
    _XtBoolean 		/* owner_events */,
    unsigned int 	/* event_mask */,
    int			/* pointer_mode */,
    int	 		/* keyboard_mode */
);

extern void XtSetMultiClickTime(
    Display*		/* dpy */,
    int 		/* milliseconds */
);

extern int XtGetMultiClickTime(
    Display*		/* dpy */
);

extern KeySym XtGetActionKeysym(
    XEvent*		/* event */,
    Modifiers*		/* modifiers_return */
);

/***************************************************************
 *
 * Keycode and Keysym procedures for translation management
 *
 ****************************************************************/

extern void XtTranslateKeycode(
    Display*		/* dpy */,
    _XtKeyCode 		/* keycode */,
    Modifiers 		/* modifiers */,
    Modifiers*		/* modifiers_return */,
    KeySym*		/* keysym_return */
);

extern void XtTranslateKey(
    Display*		/* dpy */,
    _XtKeyCode		/* keycode */,
    Modifiers		/* modifiers */,
    Modifiers*		/* modifiers_return */,
    KeySym*		/* keysym_return */
);

extern void XtSetKeyTranslator(
    Display*		/* dpy */,
    XtKeyProc 		/* proc */
);

extern void XtRegisterCaseConverter(
    Display*		/* dpy */,
    XtCaseProc 		/* proc */,
    KeySym 		/* start */,
    KeySym 		/* stop */
);

extern void XtConvertCase(
    Display*		/* dpy */,
    KeySym 		/* keysym */,
    KeySym*		/* lower_return */,
    KeySym*		/* upper_return */
);

/****************************************************************
 *
 * Event Management
 *
 ****************************************************************/

/* XtAllEvents is valid only for XtRemoveEventHandler and
 * XtRemoveRawEventHandler; don't use it to select events!
 */
#define XtAllEvents ((EventMask) -1L)

extern void XtAddEventHandler(
    Widget 		/* widget */,
    EventMask 		/* event_mask */,
    _XtBoolean 		/* nonmaskable */,
    XtEventHandler 	/* proc */,
    XtPointer 		/* closure */
);

extern void XtRemoveEventHandler(
    Widget 		/* widget */,
    EventMask 		/* event_mask */,
    _XtBoolean 		/* nonmaskable */,
    XtEventHandler 	/* proc */,
    XtPointer 		/* closure */
);

extern void XtAddRawEventHandler(
    Widget 		/* widget */,
    EventMask 		/* event_mask */,
    _XtBoolean 		/* nonmaskable */,
    XtEventHandler 	/* proc */,
    XtPointer 		/* closure */
);

extern void XtRemoveRawEventHandler(
    Widget 		/* widget */,
    EventMask 		/* event_mask */,
    _XtBoolean 		/* nonmaskable */,
    XtEventHandler 	/* proc */,
    XtPointer 		/* closure */
);

extern void XtInsertEventHandler(
    Widget 		/* widget */,
    EventMask 		/* event_mask */,
    _XtBoolean 		/* nonmaskable */,
    XtEventHandler 	/* proc */,
    XtPointer 		/* closure */,
    XtListPosition 	/* position */
);

extern void XtInsertRawEventHandler(
    Widget 		/* widget */,
    EventMask 		/* event_mask */,
    _XtBoolean 		/* nonmaskable */,
    XtEventHandler 	/* proc */,
    XtPointer 		/* closure */,
    XtListPosition 	/* position */
);

extern XtEventDispatchProc XtSetEventDispatcher(
    Display*		/* dpy */,
    int			/* event_type */,
    XtEventDispatchProc	/* proc */
);

extern Boolean XtDispatchEventToWidget(
    Widget		/* widget */,
    XEvent*		/* event */
);

extern void XtInsertEventTypeHandler(
    Widget		/* widget */,
    int			/* type */,
    XtPointer		/* select_data */,
    XtEventHandler	/* proc */,
    XtPointer		/* closure */,
    XtListPosition	/* position */
);

extern void XtRemoveEventTypeHandler(
    Widget		/* widget */,
    int			/* type */,
    XtPointer		/* select_data */,
    XtEventHandler	/* proc */,
    XtPointer		/* closure */
);

extern EventMask XtBuildEventMask(
    Widget 		/* widget */
);

extern void XtRegisterExtensionSelector(
    Display*		/* dpy */,
    int			/* min_event_type */,
    int			/* max_event_type */,
    XtExtensionSelectProc /* proc */,
    XtPointer		/* client_data */
);

extern void XtAddGrab(
    Widget 		/* widget */,
    _XtBoolean 		/* exclusive */,
    _XtBoolean 		/* spring_loaded */
);

extern void XtRemoveGrab(
    Widget 		/* widget */
);

extern void XtProcessEvent( /* obsolete */
    XtInputMask 		/* mask */
);

extern void XtAppProcessEvent(
    XtAppContext 		/* app_context */,
    XtInputMask 		/* mask */
);

extern void XtMainLoop( /* obsolete */
    void
);

extern void XtAppMainLoop(
    XtAppContext 		/* app_context */
);

extern void XtAddExposureToRegion(
    XEvent*		/* event */,
    Region 		/* region */
);

extern void XtSetKeyboardFocus(
    Widget		/* subtree */,
    Widget 		/* descendent */
);

extern Widget XtGetKeyboardFocusWidget(
    Widget		/* widget */
);

extern XEvent* XtLastEventProcessed(
    Display*		/* dpy */
);

extern Time XtLastTimestampProcessed(
    Display*		/* dpy */
);

/****************************************************************
 *
 * Event Gathering Routines
 *
 ****************************************************************/

extern XtIntervalId XtAddTimeOut( /* obsolete */
    unsigned long 	/* interval */,
    XtTimerCallbackProc /* proc */,
    XtPointer 		/* closure */
);

extern XtIntervalId XtAppAddTimeOut(
    XtAppContext 	/* app_context */,
    unsigned long 	/* interval */,
    XtTimerCallbackProc /* proc */,
    XtPointer 		/* closure */
);

extern void XtRemoveTimeOut(
    XtIntervalId 	/* timer */
);

extern XtInputId XtAddInput( /* obsolete */
    int 		/* source */,
    XtPointer 		/* condition */,
    XtInputCallbackProc /* proc */,
    XtPointer 		/* closure */
);

extern XtInputId XtAppAddInput(
    XtAppContext       	/* app_context */,
    int 		/* source */,
    XtPointer 		/* condition */,
    XtInputCallbackProc /* proc */,
    XtPointer 		/* closure */
);

extern void XtRemoveInput(
    XtInputId 		/* id */
);

extern XtSignalId XtAddSignal(
    XtSignalCallbackProc /* proc */,
    XtPointer		/* closure */);

extern XtSignalId XtAppAddSignal(
    XtAppContext       	/* app_context */,
    XtSignalCallbackProc /* proc */,
    XtPointer 		/* closure */
);

extern void XtRemoveSignal(
    XtSignalId 		/* id */
);

extern void XtNoticeSignal(
    XtSignalId		/* id */
);

extern void XtNextEvent( /* obsolete */
    XEvent* 		/* event */
);

extern void XtAppNextEvent(
    XtAppContext 	/* app_context */,
    XEvent*		/* event_return */
);

#define XtIMXEvent		1
#define XtIMTimer		2
#define XtIMAlternateInput	4
#define XtIMSignal		8
#define XtIMAll (XtIMXEvent | XtIMTimer | XtIMAlternateInput | XtIMSignal)

extern Boolean XtPending( /* obsolete */
    void
);

extern XtInputMask XtAppPending(
    XtAppContext 	/* app_context */
);

extern XtBlockHookId XtAppAddBlockHook(
    XtAppContext 	/* app_context */,
    XtBlockHookProc 	/* proc */,
    XtPointer 		/* client_data */
);

extern void XtRemoveBlockHook(
    XtBlockHookId 	/* id */
);

/****************************************************************
 *
 * Random utility routines
 *
 ****************************************************************/

#define XtIsRectObj(object)	(_XtCheckSubclassFlag(object, (XtEnum)0x02))
#define XtIsWidget(object)	(_XtCheckSubclassFlag(object, (XtEnum)0x04))
#define XtIsComposite(widget)	(_XtCheckSubclassFlag(widget, (XtEnum)0x08))
#define XtIsConstraint(widget)	(_XtCheckSubclassFlag(widget, (XtEnum)0x10))
#define XtIsShell(widget)	(_XtCheckSubclassFlag(widget, (XtEnum)0x20))

#undef XtIsOverrideShell
extern Boolean XtIsOverrideShell(Widget /* object */);
#define XtIsOverrideShell(widget) \
    (_XtIsSubclassOf(widget, (WidgetClass)overrideShellWidgetClass, \
		     (WidgetClass)shellWidgetClass, (XtEnum)0x20))

#define XtIsWMShell(widget)	(_XtCheckSubclassFlag(widget, (XtEnum)0x40))

#undef XtIsVendorShell
extern Boolean XtIsVendorShell(Widget /* object */);
#define XtIsVendorShell(widget)	\
    (_XtIsSubclassOf(widget, (WidgetClass)vendorShellWidgetClass, \
		     (WidgetClass)wmShellWidgetClass, (XtEnum)0x40))

#undef XtIsTransientShell
extern Boolean XtIsTransientShell(Widget /* object */);
#define XtIsTransientShell(widget) \
    (_XtIsSubclassOf(widget, (WidgetClass)transientShellWidgetClass, \
		     (WidgetClass)wmShellWidgetClass, (XtEnum)0x40))
#define XtIsTopLevelShell(widget) (_XtCheckSubclassFlag(widget, (XtEnum)0x80))

#undef XtIsApplicationShell
extern Boolean XtIsApplicationShell(Widget /* object */);
#define XtIsApplicationShell(widget) \
    (_XtIsSubclassOf(widget, (WidgetClass)applicationShellWidgetClass, \
		     (WidgetClass)topLevelShellWidgetClass, (XtEnum)0x80))

#undef XtIsSessionShell
extern Boolean XtIsSessionShell(Widget /* object */);
#define XtIsSessionShell(widget) \
    (_XtIsSubclassOf(widget, (WidgetClass)sessionShellWidgetClass, \
		     (WidgetClass)topLevelShellWidgetClass, (XtEnum)0x80))

extern void XtRealizeWidget(
    Widget 		/* widget */
);

void XtUnrealizeWidget(
    Widget 		/* widget */
);

extern void XtDestroyWidget(
    Widget 		/* widget */
);

extern void XtSetSensitive(
    Widget 		/* widget */,
    _XtBoolean 		/* sensitive */
);

extern void XtSetMappedWhenManaged(
    Widget 		/* widget */,
    _XtBoolean 		/* mapped_when_managed */
);

extern Widget XtNameToWidget(
    Widget 		/* reference */,
    _Xconst _XtString	/* names */
);

extern Widget XtWindowToWidget(
    Display*		/* display */,
    Window 		/* window */
);

extern XtPointer XtGetClassExtension(
    WidgetClass		/* object_class */,
    Cardinal		/* byte_offset */,
    XrmQuark		/* type */,
    long		/* version */,
    Cardinal		/* record_size */
);

/***************************************************************
 *
 * Arg lists
 *
 ****************************************************************/


#define XtSetArg(arg, n, d) \
    ((void)( (arg).name = (n), (arg).value = (XtArgVal)(d) ))

extern ArgList XtMergeArgLists(
    ArgList 		/* args1 */,
    Cardinal 		/* num_args1 */,
    ArgList 		/* args2 */,
    Cardinal 		/* num_args2 */
);

/***************************************************************
 *
 * Vararg lists
 *
 ****************************************************************/

#define XtVaNestedList  "XtVaNestedList"
#define XtVaTypedArg    "XtVaTypedArg"

extern XtVarArgsList XtVaCreateArgsList(
    XtPointer		/*unused*/, ...
) _X_SENTINEL(0);

/*************************************************************
 *
 * Information routines
 *
 ************************************************************/

#ifndef _XtIntrinsicP_h

/* We're not included from the private file, so define these */

extern Display *XtDisplay(
    Widget 		/* widget */
);

extern Display *XtDisplayOfObject(
    Widget 		/* object */
);

extern Screen *XtScreen(
    Widget 		/* widget */
);

extern Screen *XtScreenOfObject(
    Widget 		/* object */
);

extern Window XtWindow(
    Widget 		/* widget */
);

extern Window XtWindowOfObject(
    Widget 		/* object */
);

extern String XtName(
    Widget 		/* object */
);

extern WidgetClass XtSuperclass(
    Widget 		/* object */
);

extern WidgetClass XtClass(
    Widget 		/* object */
);

extern Widget XtParent(
    Widget 		/* widget */
);

#endif /*_XtIntrinsicP_h*/

#undef XtMapWidget
extern void XtMapWidget(Widget /* w */);
#define XtMapWidget(widget)	XMapWindow(XtDisplay(widget), XtWindow(widget))

#undef XtUnmapWidget
extern void XtUnmapWidget(Widget /* w */);
#define XtUnmapWidget(widget)	\
		XUnmapWindow(XtDisplay(widget), XtWindow(widget))

extern void XtAddCallback(
    Widget 		/* widget */,
    _Xconst _XtString 	/* callback_name */,
    XtCallbackProc 	/* callback */,
    XtPointer 		/* closure */
);

extern void XtRemoveCallback(
    Widget 		/* widget */,
    _Xconst _XtString 	/* callback_name */,
    XtCallbackProc 	/* callback */,
    XtPointer 		/* closure */
);

extern void XtAddCallbacks(
    Widget 		/* widget */,
    _Xconst _XtString	/* callback_name */,
    XtCallbackList 	/* callbacks */
);

extern void XtRemoveCallbacks(
    Widget 		/* widget */,
    _Xconst _XtString 	/* callback_name */,
    XtCallbackList 	/* callbacks */
);

extern void XtRemoveAllCallbacks(
    Widget 		/* widget */,
    _Xconst _XtString 	/* callback_name */
);


extern void XtCallCallbacks(
    Widget 		/* widget */,
    _Xconst _XtString 	/* callback_name */,
    XtPointer 		/* call_data */
);

extern void XtCallCallbackList(
    Widget		/* widget */,
    XtCallbackList 	/* callbacks */,
    XtPointer 		/* call_data */
);

extern XtCallbackStatus XtHasCallbacks(
    Widget 		/* widget */,
    _Xconst _XtString 	/* callback_name */
);

/****************************************************************
 *
 * Geometry Management
 *
 ****************************************************************/


extern XtGeometryResult XtMakeGeometryRequest(
    Widget 		/* widget */,
    XtWidgetGeometry*	/* request */,
    XtWidgetGeometry*	/* reply_return */
);

extern XtGeometryResult XtQueryGeometry(
    Widget 		/* widget */,
    XtWidgetGeometry*	/* intended */,
    XtWidgetGeometry*	/* preferred_return */
);

extern Widget XtCreatePopupShell(
    _Xconst _XtString	/* name */,
    WidgetClass 	/* widgetClass */,
    Widget 		/* parent */,
    ArgList 		/* args */,
    Cardinal 		/* num_args */
);

extern Widget XtVaCreatePopupShell(
    _Xconst _XtString	/* name */,
    WidgetClass		/* widgetClass */,
    Widget		/* parent */,
    ...
) _X_SENTINEL(0);

extern void XtPopup(
    Widget 		/* popup_shell */,
    XtGrabKind 		/* grab_kind */
);

extern void XtPopupSpringLoaded(
    Widget 		/* popup_shell */
);

extern void XtCallbackNone(
    Widget 		/* widget */,
    XtPointer 		/* closure */,
    XtPointer 		/* call_data */
);

extern void XtCallbackNonexclusive(
    Widget 		/* widget */,
    XtPointer 		/* closure */,
    XtPointer 		/* call_data */
);

extern void XtCallbackExclusive(
    Widget 		/* widget */,
    XtPointer 		/* closure */,
    XtPointer 		/* call_data */
);

extern void XtPopdown(
    Widget 		/* popup_shell */
);

extern void XtCallbackPopdown(
    Widget 		/* widget */,
    XtPointer 		/* closure */,
    XtPointer 		/* call_data */
);

extern void XtMenuPopupAction(
    Widget 		/* widget */,
    XEvent*		/* event */,
    String*		/* params */,
    Cardinal*		/* num_params */
);

extern Widget XtCreateWidget(
    _Xconst _XtString 	/* name */,
    WidgetClass 	/* widget_class */,
    Widget 		/* parent */,
    ArgList 		/* args */,
    Cardinal 		/* num_args */
);

extern Widget XtCreateManagedWidget(
    _Xconst _XtString 	/* name */,
    WidgetClass 	/* widget_class */,
    Widget 		/* parent */,
    ArgList 		/* args */,
    Cardinal 		/* num_args */
);

extern Widget XtVaCreateWidget(
    _Xconst _XtString	/* name */,
    WidgetClass		/* widget */,
    Widget		/* parent */,
    ...
) _X_SENTINEL(0);

extern Widget XtVaCreateManagedWidget(
    _Xconst _XtString	/* name */,
    WidgetClass		/* widget_class */,
    Widget		/* parent */,
    ...
) _X_SENTINEL(0);

extern Widget XtCreateApplicationShell( /* obsolete */
    _Xconst _XtString 	/* name */,
    WidgetClass 	/* widget_class */,
    ArgList 		/* args */,
    Cardinal 		/* num_args */
);

extern Widget XtAppCreateShell(
    _Xconst _XtString	/* application_name */,
    _Xconst _XtString	/* application_class */,
    WidgetClass 	/* widget_class */,
    Display*		/* display */,
    ArgList 		/* args */,
    Cardinal 		/* num_args */
);

extern Widget XtVaAppCreateShell(
    _Xconst _XtString	/* application_name */,
    _Xconst _XtString	/* application_class */,
    WidgetClass		/* widget_class */,
    Display*		/* display */,
    ...
) _X_SENTINEL(0);

/****************************************************************
 *
 * Toolkit initialization
 *
 ****************************************************************/

extern void XtToolkitInitialize(
    void
);

extern XtLanguageProc XtSetLanguageProc(
    XtAppContext	/* app_context */,
    XtLanguageProc	/* proc */,
    XtPointer		/* client_data */
);

extern void XtDisplayInitialize(
    XtAppContext 	/* app_context */,
    Display*		/* dpy */,
    _Xconst _XtString	/* application_name */,
    _Xconst _XtString	/* application_class */,
    XrmOptionDescRec* 	/* options */,
    Cardinal 		/* num_options */,
    int*		/* argc */,
    _XtString*		/* argv */
);

extern Widget XtOpenApplication(
    XtAppContext*	/* app_context_return */,
    _Xconst _XtString	/* application_class */,
    XrmOptionDescList 	/* options */,
    Cardinal 		/* num_options */,
    int*		/* argc_in_out */,
    _XtString*		/* argv_in_out */,
    String*		/* fallback_resources */,
    WidgetClass		/* widget_class */,
    ArgList 		/* args */,
    Cardinal 		/* num_args */
);

extern Widget XtVaOpenApplication(
    XtAppContext*	/* app_context_return */,
    _Xconst _XtString	/* application_class */,
    XrmOptionDescList	/* options */,
    Cardinal		/* num_options */,
    int*		/* argc_in_out */,
    _XtString*		/* argv_in_out */,
    String*		/* fallback_resources */,
    WidgetClass		/* widget_class */,
    ...
) _X_SENTINEL(0);

extern Widget XtAppInitialize( /* obsolete */
    XtAppContext*	/* app_context_return */,
    _Xconst _XtString	/* application_class */,
    XrmOptionDescList 	/* options */,
    Cardinal 		/* num_options */,
    int*		/* argc_in_out */,
    _XtString*		/* argv_in_out */,
    String*		/* fallback_resources */,
    ArgList 		/* args */,
    Cardinal 		/* num_args */
);

extern Widget XtVaAppInitialize( /* obsolete */
    XtAppContext*	/* app_context_return */,
    _Xconst _XtString	/* application_class */,
    XrmOptionDescList	/* options */,
    Cardinal		/* num_options */,
    int*		/* argc_in_out */,
    _XtString*		/* argv_in_out */,
    String*		/* fallback_resources */,
    ...
) _X_SENTINEL(0);

extern Widget XtInitialize( /* obsolete */
    _Xconst _XtString 	/* shell_name */,
    _Xconst _XtString 	/* application_class */,
    XrmOptionDescRec* 	/* options */,
    Cardinal 		/* num_options */,
    int*		/* argc */,
    _XtString*		/* argv */
);

extern Display *XtOpenDisplay(
    XtAppContext 	/* app_context */,
    _Xconst _XtString	/* display_string */,
    _Xconst _XtString	/* application_name */,
    _Xconst _XtString	/* application_class */,
    XrmOptionDescRec*	/* options */,
    Cardinal 		/* num_options */,
    int*		/* argc */,
    _XtString*		/* argv */
);

extern XtAppContext XtCreateApplicationContext(
    void
);

extern void XtAppSetFallbackResources(
    XtAppContext 	/* app_context */,
    String*		/* specification_list */
);

extern void XtDestroyApplicationContext(
    XtAppContext 	/* app_context */
);

extern void XtInitializeWidgetClass(
    WidgetClass 	/* widget_class */
);

extern XtAppContext XtWidgetToApplicationContext(
    Widget 		/* widget */
);

extern XtAppContext XtDisplayToApplicationContext(
    Display*		/* dpy */
);

extern XrmDatabase XtDatabase(
    Display*		/* dpy */
);

extern XrmDatabase XtScreenDatabase(
    Screen*		/* screen */
);

extern void XtCloseDisplay(
    Display*		/* dpy */
);

extern void XtGetApplicationResources(
    Widget 		/* widget */,
    XtPointer 		/* base */,
    XtResourceList 	/* resources */,
    Cardinal 		/* num_resources */,
    ArgList 		/* args */,
    Cardinal 		/* num_args */
);

extern void XtVaGetApplicationResources(
    Widget		/* widget */,
    XtPointer		/* base */,
    XtResourceList	/* resources */,
    Cardinal		/* num_resources */,
    ...
) _X_SENTINEL(0);

extern void XtGetSubresources(
    Widget 		/* widget */,
    XtPointer 		/* base */,
    _Xconst _XtString 	/* name */,
    _Xconst _XtString 	/* class */,
    XtResourceList 	/* resources */,
    Cardinal 		/* num_resources */,
    ArgList 		/* args */,
    Cardinal 		/* num_args */
);

extern void XtVaGetSubresources(
    Widget		/* widget */,
    XtPointer		/* base */,
    _Xconst _XtString	/* name */,
    _Xconst _XtString	/* class */,
    XtResourceList	/* resources */,
    Cardinal		/* num_resources */,
    ...
) _X_SENTINEL(0);

extern void XtSetValues(
    Widget 		/* widget */,
    ArgList 		/* args */,
    Cardinal 		/* num_args */
);

extern void XtVaSetValues(
    Widget		/* widget */,
    ...
) _X_SENTINEL(0);

extern void XtGetValues(
    Widget 		/* widget */,
    ArgList 		/* args */,
    Cardinal 		/* num_args */
);

extern void XtVaGetValues(
    Widget		/* widget */,
    ...
) _X_SENTINEL(0);

extern void XtSetSubvalues(
    XtPointer 		/* base */,
    XtResourceList 	/* resources */,
    Cardinal 		/* num_resources */,
    ArgList 		/* args */,
    Cardinal 		/* num_args */
);

extern void XtVaSetSubvalues(
    XtPointer		/* base */,
    XtResourceList	/* resources */,
    Cardinal		/* num_resources */,
    ...
) _X_SENTINEL(0);

extern void XtGetSubvalues(
    XtPointer 		/* base */,
    XtResourceList 	/* resources */,
    Cardinal 		/* num_resources */,
    ArgList 		/* args */,
    Cardinal 		/* num_args */
);

extern void XtVaGetSubvalues(
    XtPointer		/* base */,
    XtResourceList	/* resources */,
    Cardinal		/* num_resources */,
    ...
) _X_SENTINEL(0);

extern void XtGetResourceList(
    WidgetClass 	/* widget_class */,
    XtResourceList*	/* resources_return */,
    Cardinal*		/* num_resources_return */
);

extern void XtGetConstraintResourceList(
    WidgetClass 	/* widget_class */,
    XtResourceList*	/* resources_return */,
    Cardinal*		/* num_resources_return */
);

#define XtUnspecifiedPixmap	((Pixmap)2)
#define XtUnspecifiedShellInt	(-1)
#define XtUnspecifiedWindow	((Window)2)
#define XtUnspecifiedWindowGroup ((Window)3)
#define XtCurrentDirectory	"XtCurrentDirectory"
#define XtDefaultForeground	"XtDefaultForeground"
#define XtDefaultBackground	"XtDefaultBackground"
#define XtDefaultFont		"XtDefaultFont"
#define XtDefaultFontSet	"XtDefaultFontSet"

#define XtOffset(p_type,field) \
	((Cardinal) (((char *) (&(((p_type)NULL)->field))) - ((char *) NULL)))

#ifdef offsetof
#define XtOffsetOf(s_type,field) offsetof(s_type,field)
#else
#define XtOffsetOf(s_type,field) XtOffset(s_type*,field)
#endif

/*************************************************************
 *
 * Session Management
 *
 ************************************************************/

typedef struct _XtCheckpointTokenRec {
    int		save_type;
    int		interact_style;
    Boolean	shutdown;
    Boolean	fast;
    Boolean	cancel_shutdown;
    int		phase;
    int		interact_dialog_type;	/* return */
    Boolean	request_cancel;		/* return */
    Boolean	request_next_phase;	/* return */
    Boolean	save_success;		/* return */
    int		type;		/* implementation private */
    Widget	widget;		/* implementation private */
} XtCheckpointTokenRec, *XtCheckpointToken;

XtCheckpointToken XtSessionGetToken(
    Widget		/* widget */
);

void XtSessionReturnToken(
    XtCheckpointToken	/* token */
);

/*************************************************************
 *
 * Error Handling
 *
 ************************************************************/

extern XtErrorMsgHandler XtAppSetErrorMsgHandler(
    XtAppContext 	/* app_context */,
    XtErrorMsgHandler 	/* handler */ _X_NORETURN
);

extern void XtSetErrorMsgHandler( /* obsolete */
    XtErrorMsgHandler 	/* handler */ _X_NORETURN
);

extern XtErrorMsgHandler XtAppSetWarningMsgHandler(
    XtAppContext 	/* app_context */,
    XtErrorMsgHandler 	/* handler */
);

extern void XtSetWarningMsgHandler( /* obsolete */
    XtErrorMsgHandler 	/* handler */
);

extern void XtAppErrorMsg(
    XtAppContext 	/* app_context */,
    _Xconst _XtString 	/* name */,
    _Xconst _XtString	/* type */,
    _Xconst _XtString	/* class */,
    _Xconst _XtString	/* default */,
    String*		/* params */,
    Cardinal*		/* num_params */
) _X_NORETURN;

extern void XtErrorMsg( /* obsolete */
    _Xconst _XtString 	/* name */,
    _Xconst _XtString	/* type */,
    _Xconst _XtString	/* class */,
    _Xconst _XtString	/* default */,
    String*		/* params */,
    Cardinal*		/* num_params */
) _X_NORETURN;

extern void XtAppWarningMsg(
    XtAppContext 	/* app_context */,
    _Xconst _XtString 	/* name */,
    _Xconst _XtString 	/* type */,
    _Xconst _XtString 	/* class */,
    _Xconst _XtString 	/* default */,
    String*		/* params */,
    Cardinal*		/* num_params */
);

extern void XtWarningMsg( /* obsolete */
    _Xconst _XtString	/* name */,
    _Xconst _XtString	/* type */,
    _Xconst _XtString	/* class */,
    _Xconst _XtString	/* default */,
    String*		/* params */,
    Cardinal*		/* num_params */
);

extern XtErrorHandler XtAppSetErrorHandler(
    XtAppContext 	/* app_context */,
    XtErrorHandler 	/* handler */ _X_NORETURN
);

extern void XtSetErrorHandler( /* obsolete */
    XtErrorHandler 	/* handler */ _X_NORETURN
);

extern XtErrorHandler XtAppSetWarningHandler(
    XtAppContext 	/* app_context */,
    XtErrorHandler 	/* handler */
);

extern void XtSetWarningHandler( /* obsolete */
    XtErrorHandler 	/* handler */
);

extern void XtAppError(
    XtAppContext 	/* app_context */,
    _Xconst _XtString	/* message */
) _X_NORETURN;

extern void XtError( /* obsolete */
    _Xconst _XtString	/* message */
) _X_NORETURN;

extern void XtAppWarning(
    XtAppContext 	/* app_context */,
    _Xconst _XtString	/* message */
);

extern void XtWarning( /* obsolete */
    _Xconst _XtString	/* message */
);

extern XrmDatabase *XtAppGetErrorDatabase(
    XtAppContext 	/* app_context */
);

extern XrmDatabase *XtGetErrorDatabase( /* obsolete */
    void
);

extern void XtAppGetErrorDatabaseText(
    XtAppContext 	/* app_context */,
    _Xconst _XtString	/* name */,
    _Xconst _XtString	/* type */,
    _Xconst _XtString	/* class */,
    _Xconst _XtString 	/* default */,
    _XtString 		/* buffer_return */,
    int 		/* nbytes */,
    XrmDatabase 	/* database */
);

extern void XtGetErrorDatabaseText( /* obsolete */
    _Xconst _XtString	/* name */,
    _Xconst _XtString	/* type */,
    _Xconst _XtString	/* class */,
    _Xconst _XtString 	/* default */,
    _XtString 		/* buffer_return */,
    int 		/* nbytes */
);

/****************************************************************
 *
 * Memory Management
 *
 ****************************************************************/

extern char *XtMalloc(
    Cardinal 		/* size */
);

extern char *XtCalloc(
    Cardinal		/* num */,
    Cardinal 		/* size */
);

extern char *XtRealloc(
    char* 		/* ptr */,
    Cardinal 		/* num */
);

extern void XtFree(
    char*		/* ptr */
);

#ifndef _X_RESTRICT_KYWD
# define _X_RESTRICT_KYWD
#endif
extern Cardinal XtAsprintf(
    _XtString *new_string,
    _Xconst char * _X_RESTRICT_KYWD format,
    ...
) _X_ATTRIBUTE_PRINTF(2,3);

#ifdef XTTRACEMEMORY

extern char *_XtMalloc( /* implementation-private */
    Cardinal	/* size */,
    const char */* file */,
    int	        /* line */
);

extern char *_XtRealloc( /* implementation-private */
    char *	/* ptr */,
    Cardinal    /* size */,
    const char */* file */,
    int		/* line */
);

extern char *_XtCalloc( /* implementation-private */
    Cardinal	/* num */,
    Cardinal 	/* size */,
    const char */* file */,
    int		/* line */
);

extern void _XtFree( /* implementation-private */
    char *	/* ptr */
);

extern Boolean _XtIsValidPointer( /* implementation-private */
    char *	/* ptr */);

extern void _XtPrintMemory( /* implementation-private */
    const char */* filename */);

#define XtMalloc(size) _XtMalloc(size, __FILE__, __LINE__)
#define XtRealloc(ptr,size) _XtRealloc(ptr, size, __FILE__, __LINE__)
#define XtCalloc(num,size) _XtCalloc(num, size, __FILE__, __LINE__)
#define XtFree(ptr) _XtFree(ptr)

#endif /* ifdef XTTRACEMEMORY */

#define XtNew(type) ((type *) XtMalloc((unsigned) sizeof(type)))

#undef XtNewString
extern String XtNewString(String /* str */);
#define XtNewString(str) \
    ((str) != NULL ? (strcpy(XtMalloc((unsigned)strlen(str) + 1), str)) : NULL)

/*************************************************************
 *
 *  Work procs
 *
 **************************************************************/

extern XtWorkProcId XtAddWorkProc( /* obsolete */
    XtWorkProc 		/* proc */,
    XtPointer 		/* closure */
);

extern XtWorkProcId XtAppAddWorkProc(
    XtAppContext 	/* app_context */,
    XtWorkProc 		/* proc */,
    XtPointer 		/* closure */
);

extern void  XtRemoveWorkProc(
    XtWorkProcId 	/* id */
);


/****************************************************************
 *
 * Graphic Context Management
 *****************************************************************/

extern GC XtGetGC(
    Widget 		/* widget */,
    XtGCMask 		/* valueMask */,
    XGCValues* 		/* values */
);

extern GC XtAllocateGC(
    Widget 		/* widget */,
    Cardinal		/* depth */,
    XtGCMask 		/* valueMask */,
    XGCValues* 		/* values */,
    XtGCMask		/* dynamicMask */,
    XtGCMask		/* unusedMask */
);

/* This implementation of XtDestroyGC differs from the formal specification
 * for historic backwards compatibility reasons.  As other implementations
 * may conform to the spec, use of XtReleaseGC is strongly encouraged.
 */
extern void XtDestroyGC( /* obsolete */
    GC 			/* gc */
);

extern void XtReleaseGC(
    Widget 		/* object */,
    GC 			/* gc */
);



extern void XtAppReleaseCacheRefs(
    XtAppContext	/* app_context */,
    XtCacheRef*		/* cache_ref */
);

extern void XtCallbackReleaseCacheRef(
    Widget 		/* widget */,
    XtPointer 		/* closure */,	/* XtCacheRef */
    XtPointer 		/* call_data */
);

extern void XtCallbackReleaseCacheRefList(
    Widget 		/* widget */,
    XtPointer 		/* closure */,	/* XtCacheRef* */
    XtPointer 		/* call_data */
);

extern void XtSetWMColormapWindows(
    Widget 		/* widget */,
    Widget*		/* list */,
    Cardinal		/* count */
);

extern _XtString XtFindFile(
    _Xconst _XtString	/* path */,
    Substitution	/* substitutions */,
    Cardinal 		/* num_substitutions */,
    XtFilePredicate	/* predicate */
);

extern _XtString XtResolvePathname(
    Display*		/* dpy */,
    _Xconst _XtString	/* type */,
    _Xconst _XtString	/* filename */,
    _Xconst _XtString	/* suffix */,
    _Xconst _XtString	/* path */,
    Substitution	/* substitutions */,
    Cardinal		/* num_substitutions */,
    XtFilePredicate 	/* predicate */
);

/****************************************************************
 *
 * Selections
 *
 *****************************************************************/

#define XT_CONVERT_FAIL (Atom)0x80000001

extern void XtDisownSelection(
    Widget 		/* widget */,
    Atom 		/* selection */,
    Time 		/* time */
);

extern void XtGetSelectionValue(
    Widget 		/* widget */,
    Atom 		/* selection */,
    Atom 		/* target */,
    XtSelectionCallbackProc /* callback */,
    XtPointer 		/* closure */,
    Time 		/* time */
);

extern void XtGetSelectionValues(
    Widget 		/* widget */,
    Atom 		/* selection */,
    Atom*		/* targets */,
    int 		/* count */,
    XtSelectionCallbackProc /* callback */,
    XtPointer*		/* closures */,
    Time 		/* time */
);

extern void XtAppSetSelectionTimeout(
    XtAppContext 	/* app_context */,
    unsigned long 	/* timeout */
);

extern void XtSetSelectionTimeout( /* obsolete */
    unsigned long 	/* timeout */
);

extern unsigned long XtAppGetSelectionTimeout(
    XtAppContext 	/* app_context */
);

extern unsigned long XtGetSelectionTimeout( /* obsolete */
    void
);

extern XSelectionRequestEvent *XtGetSelectionRequest(
    Widget 		/* widget */,
    Atom 		/* selection */,
    XtRequestId 	/* request_id */
);

extern void XtGetSelectionValueIncremental(
    Widget 		/* widget */,
    Atom 		/* selection */,
    Atom 		/* target */,
    XtSelectionCallbackProc /* selection_callback */,
    XtPointer 		/* client_data */,
    Time 		/* time */
);

extern void XtGetSelectionValuesIncremental(
    Widget 		/* widget */,
    Atom 		/* selection */,
    Atom*		/* targets */,
    int 		/* count */,
    XtSelectionCallbackProc /* callback */,
    XtPointer*		/* client_data */,
    Time 		/* time */
);

extern void XtSetSelectionParameters(
    Widget		/* requestor */,
    Atom		/* selection */,
    Atom		/* type */,
    XtPointer		/* value */,
    unsigned long	/* length */,
    int			/* format */
);

extern void XtGetSelectionParameters(
    Widget		/* owner */,
    Atom		/* selection */,
    XtRequestId		/* request_id */,
    Atom*		/* type_return */,
    XtPointer*		/* value_return */,
    unsigned long*	/* length_return */,
    int*		/* format_return */
);

extern void XtCreateSelectionRequest(
    Widget		/* requestor */,
    Atom		/* selection */
);

extern void XtSendSelectionRequest(
    Widget		/* requestor */,
    Atom		/* selection */,
    Time		/* time */
);

extern void XtCancelSelectionRequest(
    Widget		/* requestor */,
    Atom		/* selection */
);

extern Atom XtReservePropertyAtom(
    Widget		/* widget */
);

extern void XtReleasePropertyAtom(
    Widget		/* widget */,
    Atom		/* selection */
);

extern void XtGrabKey(
    Widget 		/* widget */,
    _XtKeyCode 		/* keycode */,
    Modifiers	 	/* modifiers */,
    _XtBoolean 		/* owner_events */,
    int 		/* pointer_mode */,
    int 		/* keyboard_mode */
);

extern void XtUngrabKey(
    Widget 		/* widget */,
    _XtKeyCode 		/* keycode */,
    Modifiers	 	/* modifiers */
);

extern int XtGrabKeyboard(
    Widget 		/* widget */,
    _XtBoolean 		/* owner_events */,
    int 		/* pointer_mode */,
    int 		/* keyboard_mode */,
    Time 		/* time */
);

extern void XtUngrabKeyboard(
    Widget 		/* widget */,
    Time 		/* time */
);

extern void XtGrabButton(
    Widget 		/* widget */,
    int 		/* button */,
    Modifiers	 	/* modifiers */,
    _XtBoolean 		/* owner_events */,
    unsigned int	/* event_mask */,
    int 		/* pointer_mode */,
    int 		/* keyboard_mode */,
    Window 		/* confine_to */,
    Cursor 		/* cursor */
);

extern void XtUngrabButton(
    Widget 		/* widget */,
    unsigned int	/* button */,
    Modifiers	 	/* modifiers */
);

extern int XtGrabPointer(
    Widget 		/* widget */,
    _XtBoolean 		/* owner_events */,
    unsigned int	/* event_mask */,
    int 		/* pointer_mode */,
    int 		/* keyboard_mode */,
    Window 		/* confine_to */,
    Cursor 		/* cursor */,
    Time 		/* time */
);

extern void XtUngrabPointer(
    Widget 		/* widget */,
    Time 		/* time */
);

extern void XtGetApplicationNameAndClass(
    Display*		/* dpy */,
    String*		/* name_return */,
    String*		/* class_return */
);

extern void XtRegisterDrawable(
    Display*		/* dpy */,
    Drawable		/* drawable */,
    Widget		/* widget */
);

extern void XtUnregisterDrawable(
    Display*		/* dpy */,
    Drawable		/* drawable */
);

extern Widget XtHooksOfDisplay(
    Display*		/* dpy */
);

typedef struct {
    String type;
    Widget widget;
    ArgList args;
    Cardinal num_args;
} XtCreateHookDataRec, *XtCreateHookData;

typedef struct {
    String type;
    Widget widget;
    XtPointer event_data;
    Cardinal num_event_data;
} XtChangeHookDataRec, *XtChangeHookData;

typedef struct {
    Widget old, req;
    ArgList args;
    Cardinal num_args;
} XtChangeHookSetValuesDataRec, *XtChangeHookSetValuesData;

typedef struct {
    String type;
    Widget widget;
    XtGeometryMask changeMask;
    XWindowChanges changes;
} XtConfigureHookDataRec, *XtConfigureHookData;

typedef struct {
    String type;
    Widget widget;
    XtWidgetGeometry* request;
    XtWidgetGeometry* reply;
    XtGeometryResult result;
} XtGeometryHookDataRec, *XtGeometryHookData;

typedef struct {
    String type;
    Widget widget;
} XtDestroyHookDataRec, *XtDestroyHookData;

extern void XtGetDisplays(
    XtAppContext	/* app_context */,
    Display***		/* dpy_return */,
    Cardinal*		/* num_dpy_return */
);

extern Boolean XtToolkitThreadInitialize(
    void
);

extern void XtAppSetExitFlag(
    XtAppContext	/* app_context */
);

extern Boolean XtAppGetExitFlag(
    XtAppContext	/* app_context */
);

extern void XtAppLock(
    XtAppContext	/* app_context */
);

extern void XtAppUnlock(
    XtAppContext	/* app_context */
);

/*
 *	Predefined Resource Converters
 */


/* String converters */

extern Boolean XtCvtStringToAcceleratorTable(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToAtom(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* Display */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToBool(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToBoolean(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToCommandArgArray(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToCursor(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* Display */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToDimension(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToDirectoryString(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToDisplay(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToFile(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToFloat(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToFont(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* Display */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToFontSet(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* Display, locale */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToFontStruct(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* Display */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToGravity(
    Display*	/* dpy */,
    XrmValuePtr /* args */,
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToInitialState(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToInt(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToPixel(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* Screen, Colormap */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

#define XtCvtStringToPosition XtCvtStringToShort

extern Boolean XtCvtStringToRestartStyle(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToShort(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToTranslationTable(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToUnsignedChar(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtStringToVisual(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* Screen, depth */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

/* int converters */

extern Boolean XtCvtIntToBool(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtIntToBoolean(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtIntToColor(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* Screen, Colormap */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

#define XtCvtIntToDimension XtCvtIntToShort

extern Boolean XtCvtIntToFloat(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtIntToFont(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtIntToPixel(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtIntToPixmap(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

#define XtCvtIntToPosition XtCvtIntToShort

extern Boolean XtCvtIntToShort(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

extern Boolean XtCvtIntToUnsignedChar(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

/* Color converter */

extern Boolean XtCvtColorToPixel(
    Display*	/* dpy */,
    XrmValuePtr /* args */,	/* none */
    Cardinal*   /* num_args */,
    XrmValuePtr	/* fromVal */,
    XrmValuePtr	/* toVal */,
    XtPointer*	/* closure_ret */
);

/* Pixel converter */

#define XtCvtPixelToColor XtCvtIntToColor


_XFUNCPROTOEND

#endif /*_XtIntrinsic_h*/
/* DON'T ADD STUFF AFTER THIS #endif */
