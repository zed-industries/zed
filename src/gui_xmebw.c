/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */
/*
 *
 * (C) 2002,2005 by Marcin Dalecki <martin@dalecki.de>
 *
 * MARCIN DALECKI ASSUMES NO RESPONSIBILITY FOR THE USE OR INABILITY TO USE ANY
 * OF THIS SOFTWARE . THIS SOFTWARE IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY
 * KIND, AND MARCIN DALECKI EXPRESSLY DISCLAIMS ALL IMPLIED WARRANTIES,
 * INCLUDING BUT NOT LIMITED TO THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
 * FITNESS FOR A PARTICULAR PURPOSE.
 */

/*
 * Enhanced Motif PushButton widget with move over behavior.
 */

#include "vim.h"

#ifdef FEAT_TOOLBAR

#include <Xm/XmP.h>
#include <Xm/DrawP.h>
#if defined(HAVE_XM_TRAITP_H) && defined(HAVE_XM_MANAGER_H) \
	&& defined(HAVE_XM_UNHIGHLIGHTT_H) && defined(HAVE_XM_XPMP_H)
# include <Xm/TraitP.h>
# include <Xm/Manager.h>
# include <Xm/UnhighlightT.h>
# include <Xm/XpmP.h>
# define UNHIGHLIGHTT
#else
# ifdef HAVE_X11_XPM_H
#  ifdef VMS
#   include <xpm.h>
#  else
#   include <X11/xpm.h>
#  endif
# endif
#endif
#include <Xm/ManagerP.h>
#include <Xm/Display.h>
#include <Xm/DisplayP.h>

#include <X11/Shell.h>
#include <X11/ShellP.h>

#include "gui_xmebwp.h"

// Provide some missing wrappers, which are missed from the LessTif
// implementation.  Also missing in Motif 1.2 and earlier.
//
// We neither use XmeGetPixmapData or _XmGetPixmapData, since with LessTif the
// pixmap will not appear in its caches properly. We cache the interesting
// values in XmEnhancedButtonPart instead ourself.
#if defined(LESSTIF_VERSION) || (XmVersion <= 1002)
# ifndef Lab_IsMenupane
#  define Lab_IsMenupane(w) (Lab_MenuType(w) == (int)XmMENU_POPUP || \
		    Lab_MenuType(w) == (int)XmMENU_PULLDOWN)
# endif
# define XmeClearBorder	    _XmClearBorder
# define XmeDrawShadows	    _XmDrawShadows
# define XmeDrawHighlight(a, b, c, d, e, f, g, h) \
    _XmDrawHighlight(a, b, c, d, e, f, g, h, LineSolid)
#endif

// Older VMS systems do not have xos_r.h and cannot haldle XtProcessLocking
#if defined(VMS)
# if defined(HAVE_XOS_R_H)
#  define XTPROCESS_LOCK XtProcessLock()
#  define XTPROCESS_UNLOCK XtProcessUnlock()
# else
#  define XTPROCESS_LOCK
#  define XTPROCESS_UNLOCK
# endif
#else
# define XTPROCESS_LOCK XtProcessLock()
# define XTPROCESS_UNLOCK XtProcessUnlock()
#endif

/*
 * Motif internals we have to cheat around with.
 */

// Hopefully this will never change...
#ifndef XmFOCUS_IGNORE
# define XmFOCUS_IGNORE       1<<1
#endif

extern Boolean _XmGetInDragMode(Widget widget);
extern void _XmPrimitiveEnter(Widget wid,
			      XEvent * event,
			      String * params, Cardinal * num_params);
extern void _XmPrimitiveLeave(Widget wid,
			      XEvent * event,
			      String * params, Cardinal * num_params);
extern void _XmSetFocusFlag(Widget w, unsigned int mask, Boolean value);
extern void _XmCalcLabelDimensions(Widget wid);

/*
 * Declaration of class methods.
 */
static void Destroy(Widget w);
static void Initialize(Widget rq, Widget eb, ArgList args, Cardinal *n);
static Boolean SetValues(Widget current, Widget request, Widget new, ArgList args, Cardinal *n);
static void Redisplay(Widget, XEvent *, Region);

/*
 * Declaration of action methods.
 */
static void Enter(Widget, XEvent *, String *, Cardinal *);
static void Leave(Widget, XEvent *, String *, Cardinal *);
static void BorderHighlight(Widget);
static void BorderUnhighlight(Widget);

/*
 * 4 x 4 stipple for desensitized widgets
 */
#define stipple_width  4
#define stipple_height 4
static char stipple_bits[] = { 0x0a, 0x05, 0x0a, 0x05 };
#define STIPPLE_BITMAP	xmEnhancedButtonClassRec.enhancedbutton_class.stipple_bitmap

/*
 * Override actions.
 */
static XtActionsRec actionsList[] =
{
    {"Enter", Enter},
    {"Leave", Leave},
};

static XtResource resources[] =
{
    {
	XmNpixmapData, XmCPixmap, XmRString, sizeof(String),
	XtOffsetOf(XmEnhancedButtonRec, enhancedbutton.pixmap_data),
	XmRImmediate, (XtPointer) NULL
    }, {
	XmNpixmapFile, XmCPixmap, XmRString, sizeof(String),
	XtOffsetOf(XmEnhancedButtonRec, enhancedbutton.pixmap_file),
	XmRImmediate, (XtPointer) NULL
    }, {
	XmNspacing, XmCSpacing, XmRHorizontalDimension, sizeof(Dimension),
	XtOffsetOf(XmEnhancedButtonRec, enhancedbutton.spacing),
	XmRImmediate, (XtPointer) 2
    },
    {
	XmNlabelLocation, XmCLocation, XmRInt, sizeof(int),
	XtOffsetOf(XmEnhancedButtonRec, enhancedbutton.label_location),
	XtRImmediate, (XtPointer) XmRIGHT
    }
};

// This is needed to work around a bug in Lesstif 2, leaving the extension
// NULL somehow results in getting it set to an invalid pointer.
XmPrimitiveClassExtRec xmEnhancedButtonPrimClassExtRec =
{
    /* next_extension      */ NULL,
    /* record_type	   */ NULLQUARK,
    /* version		   */ XmPrimitiveClassExtVersion,
    /* record_size	   */ sizeof(XmPrimitiveClassExtRec),
    /* widget_baseline	   */ XmInheritBaselineProc,
    /* widget_display_rect */ XmInheritDisplayRectProc,
    /* widget_margins      */ NULL
};

XmEnhancedButtonClassRec xmEnhancedButtonClassRec =
{
    {
	// core_class fields
	/* superclass		 */ (WidgetClass) & xmPushButtonClassRec,
	/* class_name		 */ "XmEnhancedButton",
	/* widget_size		 */ sizeof(XmEnhancedButtonRec),
	/* class_initialize	 */ NULL,
	/* class_part_initialize */ NULL,
	/* class_inited		 */ False,
	/* initialize		 */ Initialize,
	/* initialize_hook	 */ NULL,
	/* realize		 */ XtInheritRealize,
	/* actions		 */ actionsList,
	/* num_actions		 */ XtNumber(actionsList),
	/* resources		 */ resources,
	/* num_resources	 */ XtNumber(resources),
	/* xrm_class		 */ NULLQUARK,
	/* compress_motion	 */ True,
	/* compress_exposure	 */ XtExposeCompressMaximal,
	/* compress_enterleave	 */ True,
	/* visible_interest	 */ False,
	/* destroy		 */ Destroy,
	/* resize		 */ XtInheritResize,
	/* expose		 */ Redisplay,
	/* set_values		 */ SetValues,
	/* set_values_hook	 */ NULL,
	/* set_values_almost	 */ XtInheritSetValuesAlmost,
	/* get_values_hook	 */ NULL,
	/* accept_focus		 */ XtInheritAcceptFocus,
	/* version		 */ XtVersion,
	/* callback_private	 */ NULL,
	/* tm_table		 */ NULL,
	/* query_geometry	 */ NULL,
	/* display_accelerator	 */ XtInheritDisplayAccelerator,
	/* extension		 */ NULL
    },

    // primitive_class fields
    {
	/* border highlight	 */ BorderHighlight,
	/* border_unhighlight	 */ BorderUnhighlight,
	/* translations		 */ XtInheritTranslations,
	/* arm and activate	 */ XmInheritArmAndActivate,
	/* synthetic resources	 */ NULL,
	/* number of syn res	 */ 0,
	/* extension		 */ (XtPointer)&xmEnhancedButtonPrimClassExtRec,
    },

    // label_class fields
    {
	/* setOverrideCallback	 */ XmInheritSetOverrideCallback,
	/* menuProcs		 */ XmInheritMenuProc,
	/* translations		 */ XtInheritTranslations,
	/* extension		 */ NULL,
    },

    // pushbutton_class record
    {
	/* extension		 */ (XtPointer) NULL,
    },

    // enhancedbutton_class fields
    {
	/* stipple_bitmap	 */ None
    }
};


WidgetClass xmEnhancedButtonWidgetClass =
				       (WidgetClass)&xmEnhancedButtonClassRec;


/*
 * Create a slightly fainter pixmap to be shown on button entry.
 */
    static unsigned short
bump_color(unsigned short value)
{
    int tmp = 2 * (((int) value - 65535) / 3) + 65535;

    return tmp;
}

    static int
alloc_color(Display	*display,
	Colormap	colormap,
	char		*colorname,
	XColor		*xcolor,
	void		*closure UNUSED)
{
    int status;

    if (colorname)
	if (!XParseColor(display, colormap, colorname, xcolor))
	    return -1;

    xcolor->red = bump_color(xcolor->red);
    xcolor->green = bump_color(xcolor->green);
    xcolor->blue = bump_color(xcolor->blue);

    status = XAllocColor(display, colormap, xcolor);
    return status != 0 ? 1 : 0;
}

// XPM
static char * blank_xpm[] =
{
// width height ncolors cpp [x_hot y_hot]
"12 12 4 1 0 0",
// colors
"#	s iconColor1	m black	c #000000",
".	s none	m none	c none",
"X	s topShadowColor	m none	c #DCDEE5",
"o	s bottomShadowColor	m black	c #5D6069",
// pixels
"##########..",
"#XXXXXXXX#..",
"#X.......#o.",
"#X.......#o.",
"#X.......#o.",
"#X.......#o.",
"#X.......#o.",
"#X.......#o.",
"#X.......#o.",
"##########o.",
"..ooooooooo.",
"............"};

/*
 * Set the pixmap.
 */
    static void
set_pixmap(XmEnhancedButtonWidget eb)
{
    // Configure defines XPMATTRIBUTES_TYPE as XpmAttributes or as
    // XpmAttributes_21, depending on what is in Xm/XpmP.h.
    XPMATTRIBUTES_TYPE   attr;
    Pixmap	    sen_pix;
    Window	    root;
    static XpmColorSymbol color[8] = {
	{"none", "none", 0},
	{"None", "none", 0},
	{"background", NULL, 0},
	{"foreground", NULL, 0},
	{"bottomShadowColor", NULL, 0},
	{"topShadowColor", NULL, 0},
	{"highlightColor", NULL, 0},
	{"armColor", NULL, 0}
    };
    int		    scr;
    Display	    *dpy = XtDisplay(eb);
    int		    x;
    int		    y;
    unsigned int    height, width, border, depth;
    int		    status = 0;
    Pixmap	    mask;
    Pixmap	    pix = None;
    Pixmap	    arm_pix = None;
    Pixmap	    ins_pix = None;
    Pixmap	    high_pix = None;
    char	    **data = (char **) eb->enhancedbutton.pixmap_data;
    char	    *fname = (char *) eb->enhancedbutton.pixmap_file;
    int		    shift;
    GC		    gc;

    // Make sure there is a default value for the pixmap.
    if (!data)
	return;

    gc = XtGetGC((Widget)eb, (XtGCMask)0, NULL);

    scr = DefaultScreen(dpy);
    root = RootWindow(dpy, scr);

    eb->label.pixmap = None;

    eb->enhancedbutton.pixmap_depth = 0;
    eb->enhancedbutton.pixmap_width = 0;
    eb->enhancedbutton.pixmap_height = 0;
    eb->enhancedbutton.normal_pixmap = None;
    eb->enhancedbutton.armed_pixmap = None;
    eb->enhancedbutton.highlight_pixmap = None;
    eb->enhancedbutton.insensitive_pixmap = None;

    // We use dynamic colors, get them now.
    motif_get_toolbar_colors(
	    &eb->core.background_pixel,
	    &eb->primitive.foreground,
	    &eb->primitive.bottom_shadow_color,
	    &eb->primitive.top_shadow_color,
	    &eb->primitive.highlight_color);

    // Setup color substitution table.
    color[0].pixel = eb->core.background_pixel;
    color[1].pixel = eb->core.background_pixel;
    color[2].pixel = eb->core.background_pixel;
    color[3].pixel = eb->primitive.foreground;
    color[4].pixel = eb->core.background_pixel;
    color[5].pixel = eb->primitive.top_shadow_color;
    color[6].pixel = eb->primitive.highlight_color;
    color[7].pixel = eb->pushbutton.arm_color;

    // Create the "sensitive" pixmap.
    attr.valuemask = XpmColorSymbols | XpmCloseness;
    attr.closeness = 65535;	// accuracy isn't crucial
    attr.colorsymbols = color;
    attr.numsymbols = XtNumber(color);

    if (fname)
	status = XpmReadFileToPixmap(dpy, root, fname, &pix, &mask, &attr);
    if (!fname || status != XpmSuccess)
	status = XpmCreatePixmapFromData(dpy, root, data, &pix, &mask, &attr);

    // If something failed, we will fill in the default pixmap.
    if (status != XpmSuccess)
	status = XpmCreatePixmapFromData(dpy, root, blank_xpm, &pix,
								&mask, &attr);

    XpmFreeAttributes(&attr);

    XGetGeometry(dpy, pix, &root, &x, &y, &width, &height, &border, &depth);

    // TODO: does the shift depend on label_location somehow?
    shift = eb->primitive.shadow_thickness / 2;

    if (shift < 1)
	shift = 1;

    sen_pix = XCreatePixmap(dpy, root, width + shift, height + shift, depth);

    XSetForeground(dpy, gc, eb->core.background_pixel);
    XFillRectangle(dpy, sen_pix, gc, 0, 0, width + shift, height + shift);
    XSetClipMask(dpy, gc, mask);
    XSetClipOrigin(dpy, gc, shift, shift);
    XCopyArea(dpy, pix, sen_pix, gc, 0, 0, width, height, shift, shift);

    // Create the "highlight" pixmap.
    color[4].pixel = eb->primitive.bottom_shadow_color;
#ifdef XpmAllocColor // SGI doesn't have it
    attr.valuemask = XpmColorSymbols | XpmCloseness | XpmAllocColor;
    attr.alloc_color = alloc_color;
#else
    attr.valuemask = XpmColorSymbols | XpmCloseness;
#endif
    attr.closeness = 65535;	// accuracy isn't crucial
    attr.colorsymbols = color;
    attr.numsymbols = XtNumber(color);

    status = XpmCreatePixmapFromData(dpy, root, data, &pix, NULL, &attr);
    XpmFreeAttributes(&attr);

    high_pix = XCreatePixmap(dpy, root, width + shift, height + shift, depth);

#if 1
    XSetForeground(dpy, gc, eb->core.background_pixel);
#else
    XSetForeground(dpy, gc, eb->primitive.top_shadow_color);
#endif
    XSetClipMask(dpy, gc, None);
    XFillRectangle(dpy, high_pix, gc, 0, 0, width + shift, height + shift);
    XSetClipMask(dpy, gc, mask);
    XSetClipOrigin(dpy, gc, 0, 0);
    XCopyArea(dpy, pix, high_pix, gc, 0, 0, width, height, 0, 0);

    arm_pix = XCreatePixmap(dpy, pix, width + shift, height + shift, depth);

    if (eb->pushbutton.fill_on_arm)
	XSetForeground(dpy, gc, eb->pushbutton.arm_color);
    else
	XSetForeground(dpy, gc, eb->core.background_pixel);
    XSetClipOrigin(dpy, gc, shift, shift);
    XSetClipMask(dpy, gc, None);
    XFillRectangle(dpy, arm_pix, gc, 0, 0, width + shift, height + shift);
    XSetClipMask(dpy, gc, mask);
    XSetClipOrigin(dpy, gc, 2 * shift, 2 * shift);
    XCopyArea(dpy, pix, arm_pix, gc, 0, 0, width, height, 2 * shift, 2 * shift);

    XFreePixmap(dpy, pix);
    XFreePixmap(dpy, mask);

    // Create the "insensitive" pixmap.
    attr.valuemask = XpmColorSymbols | XpmCloseness | XpmColorKey;
    attr.closeness = 65535;	// accuracy isn't crucial
    attr.colorsymbols = color;
    attr.numsymbols = ARRAY_LENGTH(color);
    attr.color_key = XPM_MONO;
    status = XpmCreatePixmapFromData(dpy, root, data, &pix, &mask, &attr);

    // Need to create new Pixmaps with the mask applied.

    ins_pix = XCreatePixmap(dpy, root, width + shift, height + shift, depth);

    XSetForeground(dpy, gc, eb->core.background_pixel);
    XSetClipOrigin(dpy, gc, 0, 0);
    XSetClipMask(dpy, gc, None);
    XFillRectangle(dpy, ins_pix, gc, 0, 0, width + shift, height + shift);
    XSetClipMask(dpy, gc, mask);
    XSetForeground(dpy, gc, eb->primitive.top_shadow_color);
    XSetClipOrigin(dpy, gc, 2 * shift, 2 * shift);
    XFillRectangle(dpy, ins_pix, gc, 2 * shift, 2 * shift, width, height);
    XSetForeground(dpy, gc, eb->primitive.bottom_shadow_color);
    XSetClipOrigin(dpy, gc, shift, shift);
    XFillRectangle(dpy, ins_pix, gc, 0, 0, width + shift, height + shift);
    XtReleaseGC((Widget) eb, gc);

    XpmFreeAttributes(&attr);

    eb->enhancedbutton.pixmap_depth = depth;
    eb->enhancedbutton.pixmap_width = width;
    eb->enhancedbutton.pixmap_height = height;
    eb->enhancedbutton.normal_pixmap = sen_pix;
    eb->enhancedbutton.highlight_pixmap = high_pix;
    eb->enhancedbutton.insensitive_pixmap = ins_pix;
    eb->enhancedbutton.armed_pixmap = arm_pix;

    eb->enhancedbutton.doing_setvalues = True;
    eb->enhancedbutton.doing_setvalues = False;

    XFreePixmap(dpy, pix);
    XFreePixmap(dpy, mask);
}

#define	BUTTON_MASK ( \
	Button1Mask | Button2Mask | Button3Mask | Button4Mask | Button5Mask \
)

    static void
draw_shadows(XmEnhancedButtonWidget eb)
{
    GC		top_gc;
    GC		bottom_gc;
    Boolean	etched_in;

    if (!eb->primitive.shadow_thickness)
       return;

    if ((eb->core.width <= 2 * eb->primitive.highlight_thickness)
	    || (eb->core.height <= 2 * eb->primitive.highlight_thickness))
	return;

#if !defined(LESSTIF_VERSION) && (XmVersion > 1002)
    {
	XmDisplay	dpy;

	dpy = (XmDisplay) XmGetXmDisplay(XtDisplay(eb));
	etched_in = dpy->display.enable_etched_in_menu;
    }
#else
    etched_in = False;
#endif
    if (!etched_in ^ eb->pushbutton.armed)
    {
	top_gc = eb->primitive.top_shadow_GC;
	bottom_gc = eb->primitive.bottom_shadow_GC;
    }
    else
    {
	top_gc = eb->primitive.bottom_shadow_GC;
	bottom_gc = eb->primitive.top_shadow_GC;
    }

    XmeDrawShadows(XtDisplay(eb), XtWindow(eb),
	    top_gc,
	    bottom_gc,
	    eb->primitive.highlight_thickness,
	    eb->primitive.highlight_thickness,
	    eb->core.width - 2 * eb->primitive.highlight_thickness,
	    eb->core.height - 2 * eb->primitive.highlight_thickness,
	    eb->primitive.shadow_thickness,
	    (unsigned)(etched_in ? XmSHADOW_IN : XmSHADOW_OUT));
}

    static void
draw_highlight(XmEnhancedButtonWidget eb)
{
    eb->primitive.highlighted = True;
    eb->primitive.highlight_drawn = True;

    if (!XtWidth(eb) || !XtHeight(eb) || !eb->primitive.highlight_thickness)
	return;

    XmeDrawHighlight(XtDisplay(eb), XtWindow(eb),
	    eb->primitive.highlight_GC, 0, 0,
	    XtWidth(eb), XtHeight(eb),
	    eb->primitive.highlight_thickness);
}

    static void
draw_unhighlight(XmEnhancedButtonWidget eb)
{
    GC manager_background_GC;

    eb->primitive.highlighted = False;
    eb->primitive.highlight_drawn = False;

    if (!XtWidth(eb) || !XtHeight(eb) || !eb->primitive.highlight_thickness)
	return;

    if (XmIsManager(eb->core.parent))
    {
#ifdef UNHIGHLIGHTT
	XmSpecifyUnhighlightTrait UnhighlightT;

	if (((UnhighlightT = (XmSpecifyUnhighlightTrait) XmeTraitGet((XtPointer)
			    XtClass(eb->core.parent), XmQTspecifyUnhighlight))
		    != NULL) && (UnhighlightT->getUnhighlightGC != NULL))
	{
	    // if unhighlight trait in parent use specified GC...
	    manager_background_GC =
		 UnhighlightT->getUnhighlightGC(eb->core.parent, (Widget) eb);
	}
	else
	{
	    // ...otherwise, use parent's background GC
	    manager_background_GC = ((XmManagerWidget)
				    (eb->core.parent))->manager.background_GC;
	}
#else
	manager_background_GC = ((XmManagerWidget)
				    (eb->core.parent))->manager.background_GC;
#endif
	XmeDrawHighlight(XtDisplay(eb), XtWindow(eb),
			 manager_background_GC,
			 0, 0, XtWidth(eb), XtHeight(eb),
			 eb->primitive.highlight_thickness);
	if (!eb->pushbutton.armed && eb->primitive.shadow_thickness)
	    XmeClearBorder(XtDisplay(eb), XtWindow(eb),
		    eb->primitive.highlight_thickness,
		    eb->primitive.highlight_thickness,
		    eb->core.width - 2 * eb->primitive.highlight_thickness,
		    eb->core.height - 2 * eb->primitive.highlight_thickness,
		    eb->primitive.shadow_thickness);
    }
    else
	XmeClearBorder(XtDisplay(eb), XtWindow(eb), 0, 0, XtWidth(eb),
		       XtHeight(eb), eb->primitive.highlight_thickness);
}

    static void
draw_pixmap(XmEnhancedButtonWidget eb,
	    XEvent *event UNUSED,
	    Region region UNUSED)
{
    Pixmap	pix;
    GC		gc = eb->label.normal_GC;
    int		depth;
    Cardinal	width;
    Cardinal	height;
    Cardinal	w;
    Cardinal	h;
    int		x;
    int		y;

    if (!XtIsSensitive((Widget) eb))
	pix = eb->enhancedbutton.insensitive_pixmap;
    else
    {
	if (eb->primitive.highlighted && !eb->pushbutton.armed)
	    pix = eb->enhancedbutton.highlight_pixmap;
	else if (eb->pushbutton.armed)
	    pix = eb->enhancedbutton.armed_pixmap;
	else
	    pix = eb->enhancedbutton.normal_pixmap;
    }

    if (pix == None || !eb->enhancedbutton.pixmap_data)
	return;

    depth = eb->enhancedbutton.pixmap_depth;
    w = eb->enhancedbutton.pixmap_width;
    h = eb->enhancedbutton.pixmap_height;

    gc = eb->label.normal_GC;
    x = eb->primitive.highlight_thickness
	+ eb->primitive.shadow_thickness
	+ eb->label.margin_width;
    y = eb->primitive.highlight_thickness
	+ eb->primitive.shadow_thickness
	+ eb->label.margin_height;
    width = eb->core.width - 2 * x;
    if (w < width)
	width = w;
    height = eb->core.height - 2 * y;
    if (h < height)
	height = h;
    if (depth == (int)eb->core.depth)
	XCopyArea(XtDisplay(eb), pix, XtWindow(eb), gc, 0, 0,
		width, height, x, y);
    else if (depth == 1)
	XCopyPlane(XtDisplay(eb), pix, XtWindow(eb), gc, 0, 0,
		width, height, x, y, (unsigned long)1);
}

/*
 * Draw the label contained in the pushbutton.
 */
    static void
draw_label(XmEnhancedButtonWidget eb, XEvent *event, Region region)
{
    GC		tmp_gc = NULL;
    Boolean	replaceGC = False;
    Boolean	deadjusted = False;
#if !defined(LESSTIF_VERSION) && (XmVersion > 1002)
    XmDisplay	dpy = (XmDisplay)XmGetXmDisplay(XtDisplay(eb));
    Boolean	etched_in = dpy->display.enable_etched_in_menu;
#else
    Boolean	etched_in = False;
#endif

    if (eb->pushbutton.armed
	    && ((!Lab_IsMenupane(eb) && eb->pushbutton.fill_on_arm)
		|| (Lab_IsMenupane(eb) && etched_in)))
    {
	if (eb->label.label_type == (int)XmSTRING
		&& eb->pushbutton.arm_color == eb->primitive.foreground)
	{
	    tmp_gc = eb->label.normal_GC;
	    eb->label.normal_GC = eb->pushbutton.background_gc;
	    replaceGC = True;
	}
    }

    /*
     * If the button contains a labeled pixmap, we will take it instead of our
     * own pixmap.
     */

    if (eb->label.label_type == (int)XmPIXMAP)
    {
	if (eb->pushbutton.armed)
	{
	    if (eb->pushbutton.arm_pixmap != XmUNSPECIFIED_PIXMAP)
		eb->label.pixmap = eb->pushbutton.arm_pixmap;
	    else
		eb->label.pixmap = eb->pushbutton.unarm_pixmap;
	}
	else
	    // pushbutton is not armed
	    eb->label.pixmap = eb->pushbutton.unarm_pixmap;
    }

    /*
     *	Temporarily remove the Xm3D_ENHANCE_PIXEL hack ("adjustment") from the
     *	margin values, so we don't confuse Label.
     */
    if (eb->pushbutton.default_button_shadow_thickness > 0)
    {
	deadjusted = True;
	Lab_MarginLeft(eb) -= Xm3D_ENHANCE_PIXEL;
	Lab_MarginRight(eb) -= Xm3D_ENHANCE_PIXEL;
	Lab_MarginTop(eb) -= Xm3D_ENHANCE_PIXEL;
	Lab_MarginBottom(eb) -= Xm3D_ENHANCE_PIXEL;
    }

    {
	XtExposeProc expose;

	XTPROCESS_LOCK;
	expose = xmLabelClassRec.core_class.expose;
	XTPROCESS_UNLOCK;
	(*expose)((Widget) eb, event, region);
    }

    if (deadjusted)
    {
	Lab_MarginLeft(eb) += Xm3D_ENHANCE_PIXEL;
	Lab_MarginRight(eb) += Xm3D_ENHANCE_PIXEL;
	Lab_MarginTop(eb) += Xm3D_ENHANCE_PIXEL;
	Lab_MarginBottom(eb) += Xm3D_ENHANCE_PIXEL;
    }

    if (replaceGC)
	eb->label.normal_GC = tmp_gc;
}

    static void
Enter(Widget wid,
      XEvent *event,
      String *params UNUSED,
      Cardinal *num_params UNUSED)
{
    XmEnhancedButtonWidget eb = (XmEnhancedButtonWidget) wid;
    XmPushButtonCallbackStruct call_value;

    if (Lab_IsMenupane(eb))
    {
	if ((((ShellWidget) XtParent(XtParent(eb)))->shell.popped_up)
		&& _XmGetInDragMode((Widget) eb))
	{
#if !defined(LESSTIF_VERSION) && (XmVersion > 1002)
	    XmDisplay dpy = (XmDisplay) XmGetXmDisplay(XtDisplay(wid));
	    Boolean etched_in = dpy->display.enable_etched_in_menu;
#else
	    Boolean etched_in = False;
#endif

	    if (eb->pushbutton.armed)
		return;

	    // ...so KHelp event is delivered correctly.
	    _XmSetFocusFlag(XtParent(XtParent(eb)), XmFOCUS_IGNORE, TRUE);
	    XtSetKeyboardFocus(XtParent(XtParent(eb)), (Widget) eb);
	    _XmSetFocusFlag(XtParent(XtParent(eb)), XmFOCUS_IGNORE, FALSE);

	    eb->pushbutton.armed = TRUE;

	    ((XmManagerWidget) XtParent(wid))->manager.active_child = wid;

	    // etched in menu button
	    if (etched_in && !XmIsTearOffButton(eb))
	    {
		XFillRectangle(XtDisplay(eb), XtWindow(eb),
			       eb->pushbutton.fill_gc,
			       0, 0, eb->core.width, eb->core.height);
		draw_label(eb, event, NULL);
		draw_pixmap(eb, event, NULL);
	    }

	    if ((eb->core.width > 2 * eb->primitive.highlight_thickness)
		    && (eb->core.height >
				       2 * eb->primitive.highlight_thickness))
	    {
		XmeDrawShadows(XtDisplay(eb), XtWindow(eb),
			eb->primitive.top_shadow_GC,
			eb->primitive.bottom_shadow_GC,
			eb->primitive.highlight_thickness,
			eb->primitive.highlight_thickness,
			eb->core.width - 2 * eb->primitive.highlight_thickness,
			eb->core.height - 2 * eb->primitive.highlight_thickness,
			eb->primitive.shadow_thickness,
			(unsigned)(etched_in ? XmSHADOW_IN : XmSHADOW_OUT));
	    }

	    if (eb->pushbutton.arm_callback)
	    {
		XFlush(XtDisplay(eb));

		call_value.reason = (int)XmCR_ARM;
		call_value.event = event;
		XtCallCallbackList((Widget) eb,
				   eb->pushbutton.arm_callback,
				   &call_value);
	    }
	}
    }
    else
    {
	XtExposeProc expose;

	_XmPrimitiveEnter((Widget) eb, event, NULL, NULL);
	if (eb->pushbutton.armed == TRUE)
	{
	    XTPROCESS_LOCK;
	    expose = XtClass(eb)->core_class.expose;
	    XTPROCESS_UNLOCK;
	    (*expose) (wid, event, (Region) NULL);
	}

	draw_highlight(eb);
	draw_shadows(eb);
	draw_pixmap(eb, event, NULL);
    }
}

    static void
Leave(Widget wid,
      XEvent *event,
      String *params UNUSED,
      Cardinal *num_params UNUSED)
{
    XmEnhancedButtonWidget eb = (XmEnhancedButtonWidget)wid;
    XmPushButtonCallbackStruct call_value;

    if (Lab_IsMenupane(eb))
    {
#if !defined(LESSTIF_VERSION) && (XmVersion > 1002)
	XmDisplay dpy = (XmDisplay) XmGetXmDisplay(XtDisplay(wid));
	Boolean etched_in = dpy->display.enable_etched_in_menu;
#else
	Boolean etched_in = False;
#endif

	if (_XmGetInDragMode((Widget)eb)
		&& eb->pushbutton.armed
		&& ( // !ActiveTearOff ||
				       event->xcrossing.mode == NotifyNormal))
	{
	    eb->pushbutton.armed = FALSE;

	    ((XmManagerWidget) XtParent(wid))->manager.active_child = NULL;

	    if (etched_in && !XmIsTearOffButton(eb))
	    {
		XFillRectangle(XtDisplay(eb), XtWindow(eb),
			       eb->pushbutton.background_gc,
			       0, 0, eb->core.width, eb->core.height);
		draw_label(eb, event, NULL);
		draw_pixmap(eb, event, NULL);
	    }
	    else
		XmeClearBorder
		    (XtDisplay(eb), XtWindow(eb),
		     eb->primitive.highlight_thickness,
		     eb->primitive.highlight_thickness,
		     eb->core.width -
		     2 * eb->primitive.highlight_thickness,
		     eb->core.height -
		     2 * eb->primitive.highlight_thickness,
		     eb->primitive.shadow_thickness);

	    if (eb->pushbutton.disarm_callback)
	    {
		XFlush(XtDisplay(eb));

		call_value.reason = (int)XmCR_DISARM;
		call_value.event = event;
		XtCallCallbackList((Widget) eb,
				   eb->pushbutton.disarm_callback,
				   &call_value);
	    }
	}
    }
    else
    {
	_XmPrimitiveLeave((Widget) eb, event, NULL, NULL);

	if (eb->pushbutton.armed == TRUE)
	{
	    XtExposeProc expose;
	    eb->pushbutton.armed = FALSE;
	    XTPROCESS_LOCK;
	    expose = XtClass(eb)->core_class.expose;
	    XTPROCESS_UNLOCK;
	    (*expose) (wid, event, (Region)NULL);
	    draw_unhighlight(eb);
	    draw_pixmap(eb, event, NULL);
	    eb->pushbutton.armed = TRUE;
	}
	else
	{
	    draw_unhighlight(eb);
	    draw_pixmap(eb, event, NULL);
	}
    }
}

#define IsNull(p)   ((p) == XmUNSPECIFIED_PIXMAP)

    static void
set_size(XmEnhancedButtonWidget newtb)
{
    unsigned int w = 0;
    unsigned int h = 0;

    _XmCalcLabelDimensions((Widget) newtb);

    // Find out how big the pixmap is
    if (newtb->enhancedbutton.pixmap_data
	    && !IsNull(newtb->label.pixmap)
	    && !IsNull(newtb->enhancedbutton.normal_pixmap))
    {
	w = newtb->enhancedbutton.pixmap_width;
	h = newtb->enhancedbutton.pixmap_height;
    }

    /*
     * Please note that we manipulate the width only in case of push buttons
     * not used in the context of a menu pane.
     */
    if (Lab_IsMenupane(newtb))
    {
	newtb->label.margin_left = w + 2 * (newtb->primitive.shadow_thickness
		+ newtb->primitive.highlight_thickness)
	    + newtb->label.margin_width;
    }
    else
    {
	newtb->label.margin_left = w;
	newtb->core.width = w + 2 * (newtb->primitive.shadow_thickness
		+ newtb->primitive.highlight_thickness
		+ newtb->label.margin_width)
	    + newtb->label.TextRect.width;

	if (newtb->label.TextRect.width > 0)
	{
	    newtb->label.margin_left += newtb->label.margin_width
					  + newtb->primitive.shadow_thickness;
	    newtb->core.width += newtb->label.margin_width
					  + newtb->primitive.shadow_thickness;
	}
    }
    if (newtb->label.TextRect.height < h)
    {
	newtb->core.height = h  + 2 * (newtb->primitive.shadow_thickness
		+ newtb->primitive.highlight_thickness
		+ newtb->label.margin_height);
    }
    else
    {
	// FIXME: We should calculate a drawing offset for the pixmap here to
	// adjust it.
    }

#if 0
    printf("%d %d %d %d %d %d - %d %d\n", newtb->enhancedbutton.normal_pixmap,
	    h, newtb->core.height,
	    newtb->primitive.shadow_thickness,
	    newtb->primitive.highlight_thickness,
	    newtb->label.margin_height,
	    newtb->core.width,
	    newtb->core.height);
#endif

    // Invoke Label's Resize procedure.
    {
	XtWidgetProc resize;
	XTPROCESS_LOCK;
	resize = xmLabelClassRec.core_class.resize;
	XTPROCESS_UNLOCK;

	(* resize) ((Widget) newtb);
    }
}

    static void
Initialize(Widget rq, Widget ebw, ArgList args UNUSED, Cardinal *n UNUSED)
{
    XmEnhancedButtonWidget  request = (XmEnhancedButtonWidget)rq;
    XmEnhancedButtonWidget  eb = (XmEnhancedButtonWidget)ebw;
    XtWidgetProc	    resize;

    XTPROCESS_LOCK;
    resize = xmLabelClassRec.core_class.resize;
    XTPROCESS_UNLOCK;

    // Create a bitmap for stippling (Drawable resources are cheap).
    if (STIPPLE_BITMAP == None)
    {
	Display *dpy = XtDisplay((Widget) request);
	Window	rootW = DefaultRootWindow(dpy);

	STIPPLE_BITMAP = XCreateBitmapFromData(dpy, rootW, stipple_bits,
		stipple_width, stipple_height);
    }
    eb->enhancedbutton.doing_setvalues = False;

    // First see what type of extended label this is.
    if (eb->enhancedbutton.pixmap_data)
    {
	XmString str;
	set_pixmap(eb);

	// FIXME: this is not the perfect way to deal with menus, which do not
	// have any string set right now.
	str = XmStringCreateLocalized("");
	XtVaSetValues((Widget) eb, XmNlabelString, str, NULL);
	XmStringFree(str);
    }
    eb->label.pixmap = eb->enhancedbutton.normal_pixmap;

    if (request->core.width == 0)
	eb->core.width = 0;
    if (request->core.height == 0)
	eb->core.height = 0;
    set_size(eb);

    (* resize)((Widget)eb);
}

    static void
free_pixmaps(XmEnhancedButtonWidget eb)
{
    /*
     * Clear the old pixmaps.
     */
    Pixmap norm_pix = eb->enhancedbutton.normal_pixmap;
    Pixmap arm_pix = eb->enhancedbutton.armed_pixmap;
    Pixmap insen_pix = eb->enhancedbutton.insensitive_pixmap;
    Pixmap high_pix = eb->enhancedbutton.highlight_pixmap;

    if (norm_pix != None && norm_pix != XmUNSPECIFIED_PIXMAP)
	XFreePixmap(XtDisplay(eb), norm_pix);

    if (arm_pix != None && arm_pix != XmUNSPECIFIED_PIXMAP)
	XFreePixmap(XtDisplay(eb), arm_pix);

    if (insen_pix != None && insen_pix != XmUNSPECIFIED_PIXMAP)
	XFreePixmap(XtDisplay(eb), insen_pix);

    if (high_pix != None && high_pix != XmUNSPECIFIED_PIXMAP)
	XFreePixmap(XtDisplay(eb), high_pix);
}

    static void
Destroy(Widget w)
{
    if (!XmIsEnhancedButton(w))
	return;

    free_pixmaps((XmEnhancedButtonWidget)w);
}

    static Boolean
SetValues(Widget current,
	  Widget request UNUSED,
	  Widget new,
	  ArgList args UNUSED,
	  Cardinal *n UNUSED)
{
    XmEnhancedButtonWidget  cur = (XmEnhancedButtonWidget) current;
    XmEnhancedButtonWidget  eb = (XmEnhancedButtonWidget) new;
    Boolean		    redraw = False;
    Boolean		    change = True;
    Display		    *dpy = XtDisplay(current);

#define NOT_EQUAL(field)       (cur->field != eb->field)

    /*
     * Make sure that lost sensitivity is causing the border to vanish as well.
     */
    if (NOT_EQUAL(core.sensitive) && !Lab_IsMenupane(current))
    {
	if (cur->core.sensitive == True)
	{
	    draw_unhighlight(eb);
	}
	else
	{
	    int		    r_x;
	    int		    r_y;
	    unsigned int    r_height;
	    unsigned int    r_width;
	    unsigned int    r_border;
	    unsigned int    r_depth;
	    int		    root_x;
	    int		    root_y;
	    int		    win_x;
	    int		    win_y;
	    Window	    root;
	    Window	    root_q;
	    Window	    child;
	    unsigned int    mask;

	    /*
	     * Artificially let the highlight appear if the mouse is over us.
	     */
	    // Best way to get the root window of object:
	    XGetGeometry(dpy, XtWindow(cur), &root, &r_x, &r_y, &r_width,
			 &r_height, &r_border, &r_depth);
	    XQueryPointer(XtDisplay(cur), XtWindow(cur), &root_q, &child,
			  &root_x, &root_y, &win_x, &win_y, &mask);

	    if (root == root_q)
	    {
		if ((win_x < 0) || (win_y < 0))
		    return False;

		if ((win_x > (int)r_width) || (win_y > (int)r_height))
		    return False;
		draw_highlight(eb);
		draw_shadows(eb);
	    }
	}

	return True;
    }

    /*
     * Check for changed ExtLabelString.
     */
    if (NOT_EQUAL(primitive.shadow_thickness))
    {
	redraw = True;
	// Don't change the pixmaps
	change = False;
    }

    if (NOT_EQUAL(primitive.foreground))
	redraw = True;
    if (NOT_EQUAL(core.background_pixel))
	redraw = True;
    if (NOT_EQUAL(pushbutton.fill_on_arm))
	redraw = True;
    if (NOT_EQUAL(enhancedbutton.spacing))
	redraw = True;
    if (NOT_EQUAL(enhancedbutton.label_location))
    {
	redraw = True;
	change = False;
    }
    if (NOT_EQUAL(label._label))
    {
	redraw = True;
	set_size(eb);
    }

    if (redraw == True)
    {
	if (change)
	    set_pixmap(eb);
	if (eb->primitive.highlighted)
	    eb->label.pixmap = eb->enhancedbutton.highlight_pixmap;
	else
	    eb->label.pixmap = eb->enhancedbutton.normal_pixmap;
	if (change)
	    set_size(eb);
	redraw = False;
    }

    return redraw;
}

    static void
Redisplay(Widget w, XEvent *event, Region region)
{
    XmEnhancedButtonWidget  eb = (XmEnhancedButtonWidget) w;
#if !defined(LESSTIF_VERSION) && (XmVersion > 1002)
    XmDisplay		    dpy;
    XtEnum		    default_button_emphasis;
#endif
    XRectangle		    box;
    int			    dx;
    int			    adjust;
    short		    fill = 0;

    if (!XtIsRealized((Widget)eb))
	return;

#if !defined(LESSTIF_VERSION) && (XmVersion > 1002)
    dpy = (XmDisplay)XmGetXmDisplay(XtDisplay(eb));
    default_button_emphasis = dpy->display.default_button_emphasis;
#endif

    /*
     * Compute the area allocated to the label of the pushbutton; fill in the
     * dimensions in the box.
     */

    if ((eb->pushbutton.arm_color == eb->primitive.top_shadow_color)
	    || (eb->pushbutton.arm_color == eb->primitive.bottom_shadow_color))
	fill = 1;

    if (eb->pushbutton.compatible)
	adjust = eb->pushbutton.show_as_default;
    else
	adjust = eb->pushbutton.default_button_shadow_thickness;

    if (adjust > 0)
    {
	adjust = adjust + eb->primitive.shadow_thickness;
	adjust = (adjust << 1);
	dx = eb->primitive.highlight_thickness + adjust + fill;
    }
    else
	dx = (eb->primitive.highlight_thickness
		+ eb->primitive.shadow_thickness + fill);

    box.x = dx;
    box.y = dx;
    adjust = (dx << 1);
    box.width  = eb->core.width - adjust;
    box.height = eb->core.height - adjust;

    /*
     * Redraw the background.
     */
    if (!Lab_IsMenupane(eb))
    {
	GC  gc;

	// Don't shade if the button contains a label with a pixmap, since
	// there is no variant of the label available with the needed
	// background.
	if (eb->pushbutton.armed && eb->pushbutton.fill_on_arm)
	{
		if (eb->label.label_type == (int)XmPIXMAP)
		{
		    if (eb->pushbutton.arm_pixmap != XmUNSPECIFIED_PIXMAP)
			gc = eb->pushbutton.fill_gc;
		    else
			gc = eb->pushbutton.background_gc;
		}
		else
		    gc = eb->pushbutton.fill_gc;
	}
	else
	    gc = eb->pushbutton.background_gc;
	// really need to fill with background if not armed ?
	if (gc)
	    XFillRectangle(XtDisplay(eb), XtWindow(eb), gc,
		    box.x, box.y, box.width, box.height);
    }

    draw_label(eb, event, region);

    if (Lab_IsMenupane(eb))
    {
	if (eb->pushbutton.armed)
	    (*(((XmPushButtonWidgetClass)XtClass(eb))
	       ->primitive_class.border_highlight))(w);
	draw_pixmap(eb, event, region);
    }
    else
    {
	adjust = 0;

#if !defined(LESSTIF_VERSION) && (XmVersion > 1002)
	/*
	 *  NOTE: PushButton has two types of shadows: primitive-shadow and
	 *  default-button-shadow.  If pushbutton is in a menu only primitive
	 *  shadows are drawn.
	 */
	switch (default_button_emphasis)
	{
	    case XmEXTERNAL_HIGHLIGHT:
		adjust = (eb->primitive.highlight_thickness -
			 (eb->pushbutton.default_button_shadow_thickness
			  ?  Xm3D_ENHANCE_PIXEL : 0));
		break;

	    case XmINTERNAL_HIGHLIGHT:
		break;

	    default:
		assert(FALSE);
		return;
	}
#endif

	/*
	 * Clear the area not occupied by label with parents background color.
	 * Label will invoke BorderUnhighlight() on the highlight_thickness
	 * area, which is redundant when XmEXTERNAL_HIGHLIGHT default button
	 * shadow emphasis is used.
	 */
	if (box.x > adjust)
	{
	    int borderwidth =box.x - adjust;
	    int rectwidth = eb->core.width - 2 * adjust;
	    int rectheight = eb->core.height - 2 * adjust;

	    if (XmIsManager(XtParent(eb)))
	    {
		XmeDrawHighlight(XtDisplay(eb), XtWindow(eb),
						     XmParentBackgroundGC(eb),
			adjust, adjust, rectwidth, rectheight, borderwidth);
	    }
	    else
	    {
		XmeClearBorder(XtDisplay(eb), XtWindow(eb),
			  adjust, adjust, rectwidth, rectheight, borderwidth);
	    }

#if !defined(LESSTIF_VERSION) && (XmVersion > 1002)
	    switch (default_button_emphasis)
	    {
		case XmINTERNAL_HIGHLIGHT:
		    // The call above erases the border highlighting.
		    if (eb->primitive.highlight_drawn)
			(*(((XmPushButtonWidgetClass) XtClass (eb))
			   ->primitive_class.border_highlight)) ((Widget) eb) ;
		    break;

		default:
		    break;
	    }
#endif
	}

	if (eb->pushbutton.default_button_shadow_thickness)
	{
	    if (eb->pushbutton.show_as_default)
	    {
		/*
		 *  - get the topShadowColor and bottomShadowColor from the
		 *  parent; use those colors to construct top and bottom gc;
		 *  use these GCs to draw the shadows of the button.
		 *
		 *  - Should not be called if pushbutton is in a row column or
		 *  in a menu.
		 *
		 *  - Should be called only if a defaultbuttonshadow is to be
		 *  drawn.
		 */
		GC	    top_gc;
		GC	    bottom_gc;
		int	    default_button_shadow_thickness;
		int	    x, y, width, height, delta;
		Widget	    parent;

		if (eb->pushbutton.compatible
				     && (eb->pushbutton.show_as_default == 0))
		    return;

		if (!eb->pushbutton.compatible
			&& (eb->pushbutton.default_button_shadow_thickness
									== 0))
		    return;

		delta = eb->primitive.highlight_thickness;

		/*
		 * May need more complex computation for getting the GCs.
		 */
		parent = XtParent(eb);
		if (XmIsManager(parent))
		{
		    // Use the parent's GC so monochrome works.
		    bottom_gc = XmParentTopShadowGC(eb);
		    top_gc = XmParentBottomShadowGC(eb);
		}
		else
		{
		    // Use your own pixel for drawing.
		    bottom_gc = eb->primitive.top_shadow_GC;
		    top_gc = eb->primitive.bottom_shadow_GC;
		}

		if ((bottom_gc == None) || (top_gc == None))
		    return;


		if (eb->pushbutton.compatible)
		    default_button_shadow_thickness =
					       eb->pushbutton.show_as_default;
		else
		    default_button_shadow_thickness =
			       eb->pushbutton.default_button_shadow_thickness;

#if !defined(LESSTIF_VERSION) && (XmVersion > 1002)
		/*
		 * Compute location of bounding box to contain the
		 * defaultButtonShadow.
		 */
		switch (default_button_emphasis)
		{
		    case XmEXTERNAL_HIGHLIGHT:
			delta = eb->primitive.highlight_thickness;
			break;

		    case XmINTERNAL_HIGHLIGHT:
			delta = Xm3D_ENHANCE_PIXEL;
			break;

		    default:
			assert(FALSE);
			return;
		}
#endif

		x = y = delta;
		width = eb->core.width - 2 * delta;
		height = eb->core.height - 2 * delta;

		if ((width > 0) && (height > 0))
		    XmeDrawShadows(XtDisplay(eb), XtWindow(eb),
			    top_gc, bottom_gc, x, y, width, height,
			    default_button_shadow_thickness,
			    (unsigned)XmSHADOW_OUT);
	    }
	}

	if (eb->primitive.highlight_drawn)
	    draw_shadows(eb);
	draw_pixmap(eb, event, region);
    }
}

    static void
BorderHighlight(Widget w)
{
    XmEnhancedButtonWidget eb = (XmEnhancedButtonWidget)w;

    (*(xmPushButtonClassRec.primitive_class.border_highlight))(w);
    draw_pixmap(eb, NULL, NULL);
}

    static void
BorderUnhighlight(Widget w)
{
    XmEnhancedButtonWidget eb = (XmEnhancedButtonWidget)w;

    (*(xmPushButtonClassRec.primitive_class.border_unhighlight))(w);
    draw_pixmap(eb, NULL, NULL);
}

#endif // FEAT_TOOLBAR
