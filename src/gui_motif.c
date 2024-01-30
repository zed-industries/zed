/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved		by Bram Moolenaar
 *				GUI/Motif support by Robert Webb
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

#include "vim.h"

#include <Xm/Form.h>
#include <Xm/RowColumn.h>
#include <Xm/PushB.h>
#include <Xm/Text.h>
#include <Xm/TextF.h>
#include <Xm/Separator.h>
#include <Xm/Label.h>
#include <Xm/CascadeB.h>
#include <Xm/ScrollBar.h>
#include <Xm/MenuShell.h>
#include <Xm/DrawingA.h>
#if (XmVersion >= 1002)
# include <Xm/RepType.h>
#endif
#include <Xm/Frame.h>
#include <Xm/LabelG.h>
#include <Xm/ToggleBG.h>
#include <Xm/SeparatoG.h>
#include <Xm/XmP.h>

#include <X11/keysym.h>
#include <X11/Xatom.h>
#include <X11/StringDefs.h>
#include <X11/Intrinsic.h>
#ifdef HAVE_X11_XPM_H
# if defined(VMS)
#  include <xpm.h>
# else
#  include <X11/xpm.h>
# endif
#else
# ifdef HAVE_XM_XPMP_H
#  include <Xm/XpmP.h>
# endif
#endif
#ifdef HAVE_XM_NOTEBOOK_H
# include <Xm/Notebook.h>
#endif

#include "gui_xmebw.h"	// for our Enhanced Button Widget

#if defined(FEAT_GUI_DIALOG) && defined(HAVE_XPM)
# include "../pixmaps/alert.xpm"
# include "../pixmaps/error.xpm"
# include "../pixmaps/generic.xpm"
# include "../pixmaps/info.xpm"
# include "../pixmaps/quest.xpm"
#endif

#define MOTIF_POPUP

extern Widget vimShell;

static Widget vimForm;
static Widget textAreaForm;
Widget textArea;
#ifdef FEAT_TOOLBAR
static Widget toolBarFrame;
static Widget toolBar;
#endif
#ifdef FEAT_GUI_TABLINE
static Widget	tabLine;
static Widget	tabLine_menu = 0;
static int	showing_tabline = 0;
#endif
#ifdef FEAT_MENU
# if (XmVersion >= 1002)
// remember the last set value for the tearoff item
static int tearoff_val = (int)XmTEAR_OFF_ENABLED;
# endif
static Widget menuBar;
#endif

#ifdef FEAT_TOOLBAR
static void reset_focus(void);
#endif

static void gui_motif_menu_colors(Widget id);
static void gui_motif_scroll_colors(Widget id);

#if (XmVersion >= 1002)
# define STRING_TAG  XmFONTLIST_DEFAULT_TAG
#else
# define STRING_TAG  XmSTRING_DEFAULT_CHARSET
#endif

/*
 * Call-back routines.
 */

    static void
scroll_cb(Widget w UNUSED, XtPointer client_data, XtPointer call_data)
{
    scrollbar_T *sb;
    long	value;
    int		dragging;

    sb = gui_find_scrollbar((long)client_data);

    value = ((XmScrollBarCallbackStruct *)call_data)->value;
    dragging = (((XmScrollBarCallbackStruct *)call_data)->reason ==
							      (int)XmCR_DRAG);
    gui_drag_scrollbar(sb, value, dragging);
}

#ifdef FEAT_GUI_TABLINE
    static void
tabline_cb(
    Widget	w UNUSED,
    XtPointer	client_data UNUSED,
    XtPointer	call_data)
{
    XmNotebookCallbackStruct *nptr;

    nptr = (XmNotebookCallbackStruct *)call_data;
    if (nptr->reason != (int)XmCR_NONE)
	send_tabline_event(nptr->page_number);
}

    static void
tabline_button_cb(
    Widget	w,
    XtPointer	client_data UNUSED,
    XtPointer	call_data UNUSED)
{
    XtPointer	cmd, tab_idx;

    XtVaGetValues(w, XmNuserData, &cmd, NULL);
    XtVaGetValues(tabLine_menu, XmNuserData, &tab_idx, NULL);

    send_tabline_menu_event((int)(long)tab_idx, (int)(long)cmd);
}

/*
 * Tabline single mouse click timeout handler
 */
    static void
motif_tabline_timer_cb (
    XtPointer		timed_out,
    XtIntervalId	*interval_id UNUSED)
{
    *((int *)timed_out) = TRUE;
}

/*
 * check if the tabline tab scroller is clicked
 */
    static int
tabline_scroller_clicked(
    char		*scroller_name,
    XButtonPressedEvent *event)
{
    Widget	tab_scroll_w;
    Position	pos_x, pos_y;
    Dimension	width, height;

    tab_scroll_w = XtNameToWidget(tabLine, scroller_name);
    if (tab_scroll_w != (Widget)0)
    {
	XtVaGetValues(tab_scroll_w, XmNx, &pos_x, XmNy, &pos_y, XmNwidth,
		      &width, XmNheight, &height, NULL);
	if (pos_x >= 0)
	{
	    // Tab scroller (next) is visible
	    if (event->x >= pos_x && event->x <= pos_x + width
		    && event->y >= pos_y && event->y <= pos_y + height)
		// Clicked on the scroller
		return TRUE;
	}
    }
    return FALSE;
}

    static void
tabline_menu_cb(
    Widget	w,
    XtPointer	closure UNUSED,
    XEvent	*e,
    Boolean	*continue_dispatch UNUSED)
{
    Widget			tab_w;
    XButtonPressedEvent		*event;
    int				tab_idx = 0;
    WidgetList			children;
    Cardinal			numChildren;
    static XtIntervalId		timer = (XtIntervalId)0;
    static int			timed_out = TRUE;

    event = (XButtonPressedEvent *)e;

    if (event->button == Button1)
    {
	if (tabline_scroller_clicked("MajorTabScrollerNext", event)
	    || tabline_scroller_clicked("MajorTabScrollerPrevious", event))
	    return;

	if (!timed_out)
	{
	    XtRemoveTimeOut(timer);
	    timed_out = TRUE;

	    /*
	     * Double click on the tabline gutter, add a new tab
	     */
	    send_tabline_menu_event(0, TABLINE_MENU_NEW);
	}
	else
	{
	    /*
	     * Single click on the tabline gutter, start a timer to check
	     * for double clicks
	     */
	    timer = XtAppAddTimeOut(app_context, (long_u)p_mouset,
				    motif_tabline_timer_cb, &timed_out);
	    timed_out = FALSE;
	}
	return;
    }

    if (event->button == Button2)
    {
	// Middle mouse click on tabpage label closes that tab.
	XtVaGetValues(tabLine_menu, XmNuserData, &tab_idx, NULL);
	send_tabline_menu_event(tab_idx, (int)TABLINE_MENU_CLOSE);
	return;
    }

    if (event->button != Button3)
	return;

    // When ignoring events don't show the menu.
    if (hold_gui_events || cmdwin_type != 0)
	return;

    if (event->subwindow != None)
    {
	tab_w = XtWindowToWidget(XtDisplay(w), event->subwindow);
	// LINTED: avoid warning: dubious operation on enum
	if (tab_w != (Widget)0 && XmIsPushButton(tab_w))
	    XtVaGetValues(tab_w, XmNpageNumber, &tab_idx, NULL);
    }

    XtVaSetValues(tabLine_menu, XmNuserData, (XtPointer)(long)tab_idx, NULL);
    XtVaGetValues(tabLine_menu, XmNchildren, &children, XmNnumChildren,
		  &numChildren, NULL);
    XtManageChildren(children, numChildren);
    XmMenuPosition(tabLine_menu, (XButtonPressedEvent *)e) ;
    XtManageChild(tabLine_menu);
}

    static void
tabline_balloon_cb(BalloonEval *beval, int state UNUSED)
{
    int		nr;
    tabpage_T	*tp;

    if (beval->target == (Widget)0)
	return;

    XtVaGetValues(beval->target, XmNpageNumber, &nr, NULL);
    tp = find_tabpage(nr);
    if (tp == NULL)
	return;

    get_tabline_label(tp, TRUE);
    gui_mch_post_balloon(beval, NameBuff);
}

#endif

/*
 * End of call-back routines
 */

/*
 * Implement three dimensional shading of insensitive labels.
 * By Marcin Dalecki.
 */

#include <Xm/XmP.h>
#include <Xm/LabelP.h>

static XtExposeProc old_label_expose = NULL;

    static void
label_expose(Widget _w, XEvent *_event, Region _region)
{
    GC		    insensitiveGC;
    XmLabelWidget   lw = (XmLabelWidget)_w;
    unsigned char   label_type = (int)XmSTRING;

    XtVaGetValues(_w, XmNlabelType, &label_type, (XtPointer)0);

    if (XtIsSensitive(_w) || label_type != (int)XmSTRING)
	(*old_label_expose)(_w, _event, _region);
    else
    {
	XGCValues   values;
	XtGCMask    mask;
	XtGCMask    dynamic;
	XFontStruct *fs;

	_XmFontListGetDefaultFont(lw->label.font, &fs);

	// FIXME: we should be doing the whole drawing ourself here.
	insensitiveGC = lw->label.insensitive_GC;

	mask = GCForeground | GCBackground | GCGraphicsExposures;
	dynamic = GCClipMask | GCClipXOrigin | GCClipYOrigin;
	values.graphics_exposures = False;

	if (fs != 0)
	{
	    mask |= GCFont;
	    values.font = fs->fid;
	}

	if (lw->primitive.top_shadow_pixmap != None
		&& lw->primitive.top_shadow_pixmap != XmUNSPECIFIED_PIXMAP)
	{
	    mask |= GCFillStyle | GCTile;
	    values.fill_style = FillTiled;
	    values.tile = lw->primitive.top_shadow_pixmap;
	}

	lw->label.TextRect.x += 1;
	lw->label.TextRect.y += 1;
	if (lw->label._acc_text != 0)
	{
	    lw->label.acc_TextRect.x += 1;
	    lw->label.acc_TextRect.y += 1;
	}

	values.foreground = lw->primitive.top_shadow_color;
	values.background = lw->core.background_pixel;

	lw->label.insensitive_GC = XtAllocateGC((Widget)lw, 0, mask,
					       &values, dynamic, (XtGCMask)0);
	(*old_label_expose)(_w, _event, _region);
	XtReleaseGC(_w, lw->label.insensitive_GC);

	lw->label.TextRect.x -= 1;
	lw->label.TextRect.y -= 1;
	if (lw->label._acc_text != 0)
	{
	    lw->label.acc_TextRect.x -= 1;
	    lw->label.acc_TextRect.y -= 1;
	}

	values.foreground = lw->primitive.bottom_shadow_color;
	values.background = lw->core.background_pixel;

	lw->label.insensitive_GC = XtAllocateGC((Widget) lw, 0, mask,
					       &values, dynamic, (XtGCMask)0);
	(*old_label_expose)(_w, _event, _region);
	XtReleaseGC(_w, lw->label.insensitive_GC);

	lw->label.insensitive_GC = insensitiveGC;
    }
}

/*
 * Create all the motif widgets necessary.
 */
    void
gui_x11_create_widgets(void)
{
#ifdef FEAT_GUI_TABLINE
    Widget	button, scroller;
    Arg		args[10];
    int		n;
    XmString	xms;
#endif

    /*
     * Install the 3D shade effect drawing routines.
     */
    if (old_label_expose == NULL)
    {
	old_label_expose = xmLabelWidgetClass->core_class.expose;
	xmLabelWidgetClass->core_class.expose = label_expose;
    }

    /*
     * Start out by adding the configured border width into the border offset
     */
    gui.border_offset = gui.border_width;

    /*
     * Install the tearOffModel resource converter.
     */
#if (XmVersion >= 1002)
    XmRepTypeInstallTearOffModelConverter();
#endif

    // Make sure the "Quit" menu entry of the window manager is ignored
    XtVaSetValues(vimShell, XmNdeleteResponse, XmDO_NOTHING, NULL);

    vimForm = XtVaCreateManagedWidget("vimForm",
	xmFormWidgetClass, vimShell,
	XmNborderWidth, 0,
	XmNhighlightThickness, 0,
	XmNshadowThickness, 0,
	XmNmarginWidth, 0,
	XmNmarginHeight, 0,
	XmNresizePolicy, XmRESIZE_ANY,
	NULL);
    gui_motif_menu_colors(vimForm);

#ifdef FEAT_MENU
    {
	Arg al[7]; // Make sure there is enough room for arguments!
	int ac = 0;

# if (XmVersion >= 1002)
	XtSetArg(al[ac], XmNtearOffModel, tearoff_val); ac++;
# endif
	XtSetArg(al[ac], XmNleftAttachment,  XmATTACH_FORM); ac++;
	XtSetArg(al[ac], XmNtopAttachment,   XmATTACH_FORM); ac++;
	XtSetArg(al[ac], XmNrightAttachment, XmATTACH_FORM); ac++;
# ifndef FEAT_TOOLBAR
	// Always stick to right hand side.
	XtSetArg(al[ac], XmNrightOffset, 0); ac++;
# endif
	XtSetArg(al[ac], XmNmarginHeight, 0); ac++;
	menuBar = XmCreateMenuBar(vimForm, "menuBar", al, ac);
	XtManageChild(menuBar);

	gui_motif_menu_colors(menuBar);
    }
#endif

#ifdef FEAT_TOOLBAR
    /*
     * Create an empty ToolBar. We should get buttons defined from menu.vim.
     */
    toolBarFrame = XtVaCreateWidget("toolBarFrame",
	xmFrameWidgetClass, vimForm,
	XmNshadowThickness, 0,
	XmNmarginHeight, 0,
	XmNmarginWidth, 0,
	XmNleftAttachment, XmATTACH_FORM,
	XmNrightAttachment, XmATTACH_FORM,
	NULL);
    gui_motif_menu_colors(toolBarFrame);

    toolBar = XtVaCreateManagedWidget("toolBar",
	xmRowColumnWidgetClass, toolBarFrame,
	XmNchildType, XmFRAME_WORKAREA_CHILD,
	XmNrowColumnType, XmWORK_AREA,
	XmNorientation, XmHORIZONTAL,
	XmNtraversalOn, False,
	XmNisHomogeneous, False,
	XmNpacking, XmPACK_TIGHT,
	XmNspacing, 0,
	XmNshadowThickness, 0,
	XmNhighlightThickness, 0,
	XmNmarginHeight, 0,
	XmNmarginWidth, 0,
	XmNadjustLast, True,
	NULL);
    gui_motif_menu_colors(toolBar);

#endif

#ifdef FEAT_GUI_TABLINE
    // Create the Vim GUI tabline
    n = 0;
    XtSetArg(args[n], XmNbindingType, XmNONE); n++;
    XtSetArg(args[n], XmNorientation, XmVERTICAL); n++;
    XtSetArg(args[n], XmNbackPageSize, XmNONE); n++;
    XtSetArg(args[n], XmNbackPageNumber, 0); n++;
    XtSetArg(args[n], XmNbackPagePlacement, XmTOP_RIGHT); n++;
    XtSetArg(args[n], XmNmajorTabSpacing, 0); n++;
    XtSetArg(args[n], XmNshadowThickness, 0); n++;
    XtSetArg(args[n], XmNleftAttachment, XmATTACH_FORM); n++;
    XtSetArg(args[n], XmNrightAttachment, XmATTACH_FORM); n++;
    tabLine = XmCreateNotebook(vimForm, "Vim tabline", args, n);

    XtAddCallback(tabLine, XmNpageChangedCallback, (XtCallbackProc)tabline_cb,
			NULL);
    XtAddEventHandler(tabLine, ButtonPressMask, False,
			(XtEventHandler)tabline_menu_cb, NULL);

    gui.tabline_height = TABLINE_HEIGHT;

    /*
     * Set the size of the minor next/prev scrollers to zero, so
     * that they are not displayed. Due to a bug in OpenMotif 2.3,
     * even if these children widget are unmanaged, they are again
     * managed by the Notebook widget and the notebook widget geometry
     * is adjusted to account for the minor scroller widgets.
     */
    scroller = XtNameToWidget(tabLine, "MinorTabScrollerNext");
    XtVaSetValues(scroller, XmNwidth, 0, XmNresizable, False,
		  XmNtraversalOn, False, NULL);
    scroller = XtNameToWidget(tabLine, "MinorTabScrollerPrevious");
    XtVaSetValues(scroller, XmNwidth, 0, XmNresizable, False,
		  XmNtraversalOn, False, NULL);

    // Create the tabline popup menu
    tabLine_menu = XmCreatePopupMenu(tabLine, "tabline popup", NULL, 0);

    // Add the buttons to the tabline popup menu
    n = 0;
    XtSetArg(args[n], XmNuserData, (XtPointer)TABLINE_MENU_CLOSE); n++;
    xms = XmStringCreate((char *)"Close tab", STRING_TAG);
    XtSetArg(args[n], XmNlabelString, xms); n++;
    button = XmCreatePushButton(tabLine_menu, "Close", args, n);
    XtAddCallback(button, XmNactivateCallback,
		  (XtCallbackProc)tabline_button_cb, NULL);
    XmStringFree(xms);

    n = 0;
    XtSetArg(args[n], XmNuserData, (XtPointer)TABLINE_MENU_NEW); n++;
    xms = XmStringCreate((char *)"New Tab", STRING_TAG);
    XtSetArg(args[n], XmNlabelString, xms); n++;
    button = XmCreatePushButton(tabLine_menu, "New Tab", args, n);
    XtAddCallback(button, XmNactivateCallback,
		  (XtCallbackProc)tabline_button_cb, NULL);
    XmStringFree(xms);

    n = 0;
    XtSetArg(args[n], XmNuserData, (XtPointer)TABLINE_MENU_OPEN); n++;
    xms = XmStringCreate((char *)"Open tab...", STRING_TAG);
    XtSetArg(args[n], XmNlabelString, xms); n++;
    button = XmCreatePushButton(tabLine_menu, "Open tab...", args, n);
    XtAddCallback(button, XmNactivateCallback,
		  (XtCallbackProc)tabline_button_cb, NULL);
    XmStringFree(xms);
#endif

    textAreaForm = XtVaCreateManagedWidget("textAreaForm",
	xmFormWidgetClass, vimForm,
	XmNleftAttachment, XmATTACH_FORM,
	XmNrightAttachment, XmATTACH_FORM,
	XmNbottomAttachment, XmATTACH_FORM,
	XmNtopAttachment, XmATTACH_FORM,
	XmNmarginWidth, 0,
	XmNmarginHeight, 0,
	XmNresizePolicy, XmRESIZE_ANY,
	NULL);
    gui_motif_scroll_colors(textAreaForm);

    textArea = XtVaCreateManagedWidget("textArea",
	xmDrawingAreaWidgetClass, textAreaForm,
	XmNforeground, gui.norm_pixel,
	XmNbackground, gui.back_pixel,
	XmNleftAttachment, XmATTACH_FORM,
	XmNtopAttachment, XmATTACH_FORM,
	XmNrightAttachment, XmATTACH_FORM,
	XmNbottomAttachment, XmATTACH_FORM,

	/*
	 * These take some control away from the user, but avoids making them
	 * add resources to get a decent looking setup.
	 */
	XmNborderWidth, 0,
	XmNhighlightThickness, 0,
	XmNshadowThickness, 0,
	NULL);

    /*
     * Install the callbacks.
     */
    gui_x11_callbacks(textArea, vimForm);

    // Pretend we don't have input focus, we will get an event if we do.
    gui.in_focus = FALSE;
}

/*
 * Called when the GUI is not going to start after all.
 */
    void
gui_x11_destroy_widgets(void)
{
    textArea = NULL;
#ifdef FEAT_MENU
    menuBar = NULL;
#endif
}

    void
gui_mch_set_text_area_pos(
    int	    x UNUSED,
    int	    y UNUSED,
    int	    w UNUSED,
    int	    h UNUSED)
{
#ifdef FEAT_TOOLBAR
    // Give keyboard focus to the textArea instead of the toolbar.
    reset_focus();
#endif
}

    void
gui_x11_set_back_color(void)
{
    if (textArea != NULL)
#if (XmVersion >= 1002)
	XmChangeColor(textArea, gui.back_pixel);
#else
	XtVaSetValues(textArea,
		  XmNbackground, gui.back_pixel,
		  NULL);
#endif
}

/*
 * Manage dialog centered on pointer.
 */
    void
manage_centered(Widget dialog_child)
{
    Widget shell = XtParent(dialog_child);
    Window root, child;
    unsigned int mask;
    unsigned int width, height, border_width, depth;
    int x, y, win_x, win_y, maxX, maxY;
    Boolean mappedWhenManaged;

    // Temporarily set value of XmNmappedWhenManaged
    // to stop the dialog from popping up right away
    XtVaGetValues(shell, XmNmappedWhenManaged, &mappedWhenManaged, NULL);
    XtVaSetValues(shell, XmNmappedWhenManaged, False, NULL);

    XtManageChild(dialog_child);

    // Get the pointer position (x, y)
    XQueryPointer(XtDisplay(shell), XtWindow(shell), &root, &child,
		  &x, &y, &win_x, &win_y, &mask);

    // Translate the pointer position (x, y) into a position for the new
    // window that will place the pointer at its center
    XGetGeometry(XtDisplay(shell), XtWindow(shell), &root, &win_x, &win_y,
		 &width, &height, &border_width, &depth);
    width += 2 * border_width;
    height += 2 * border_width;
    x -= width / 2;
    y -= height / 2;

    // Ensure that the dialog remains on screen
    maxX = XtScreen(shell)->width - width;
    maxY = XtScreen(shell)->height - height;
    if (x < 0)
	x = 0;
    if (x > maxX)
	x = maxX;
    if (y < 0)
	y = 0;
    if (y > maxY)
	y = maxY;

    // Set desired window position in the DialogShell
    XtVaSetValues(shell, XmNx, x, XmNy, y, NULL);

    // Map the widget
    XtMapWidget(shell);

    // Restore the value of XmNmappedWhenManaged
    XtVaSetValues(shell, XmNmappedWhenManaged, mappedWhenManaged, NULL);
}

#if defined(FEAT_MENU) || defined(FEAT_GUI_DIALOG) || defined(PROTO)

/*
 * Encapsulate the way an XmFontList is created.
 */
    XmFontList
gui_motif_create_fontlist(XFontStruct *font)
{
    XmFontList font_list;

# if (XmVersion <= 1001)
    // Motif 1.1 method
    font_list = XmFontListCreate(font, STRING_TAG);
# else
    // Motif 1.2 method
    XmFontListEntry font_list_entry;

    font_list_entry = XmFontListEntryCreate(STRING_TAG, XmFONT_IS_FONT,
					    (XtPointer)font);
    font_list = XmFontListAppendEntry(NULL, font_list_entry);
    XmFontListEntryFree(&font_list_entry);
# endif
    return font_list;
}

# if ((XmVersion > 1001) && defined(FEAT_XFONTSET)) || defined(PROTO)
    XmFontList
gui_motif_fontset2fontlist(XFontSet *fontset)
{
    XmFontList font_list;

    // Motif 1.2 method
    XmFontListEntry font_list_entry;

    font_list_entry = XmFontListEntryCreate(STRING_TAG,
					    XmFONT_IS_FONTSET,
					    (XtPointer)*fontset);
    font_list = XmFontListAppendEntry(NULL, font_list_entry);
    XmFontListEntryFree(&font_list_entry);
    return font_list;
}
# endif

#endif

#if defined(FEAT_MENU) || defined(PROTO)
/*
 * Menu stuff.
 */

static void gui_motif_add_actext(vimmenu_T *menu);
#if (XmVersion >= 1002)
static void toggle_tearoff(Widget wid);
static void gui_mch_recurse_tearoffs(vimmenu_T *menu);
#endif
static void submenu_change(vimmenu_T *mp, int colors);

static void do_set_mnemonics(int enable);
static int menu_enabled = TRUE;

    void
gui_mch_enable_menu(int flag)
{
    if (flag)
    {
	XtManageChild(menuBar);
#ifdef FEAT_TOOLBAR
	if (XtIsManaged(XtParent(toolBar)))
	{
	    // toolBar is attached to top form
	    XtVaSetValues(XtParent(toolBar),
		XmNtopAttachment, XmATTACH_WIDGET,
		XmNtopWidget, menuBar,
		NULL);
#ifdef FEAT_GUI_TABLINE
	    if (showing_tabline)
	    {
		XtVaSetValues(tabLine,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, XtParent(toolBar),
			      NULL);
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, tabLine,
			      NULL);
	    }
	    else
#endif
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, XtParent(toolBar),
			      NULL);
	}
	else
#endif
	{
#ifdef FEAT_GUI_TABLINE
	    if (showing_tabline)
	    {
		XtVaSetValues(tabLine,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, menuBar,
			      NULL);
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, tabLine,
			      NULL);
	    }
	    else
#endif
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, menuBar,
			      NULL);
	}
    }
    else
    {
	XtUnmanageChild(menuBar);
#ifdef FEAT_TOOLBAR
	if (XtIsManaged(XtParent(toolBar)))
	{
	    XtVaSetValues(XtParent(toolBar),
		XmNtopAttachment, XmATTACH_FORM,
		NULL);
#ifdef FEAT_GUI_TABLINE
	    if (showing_tabline)
	    {
		XtVaSetValues(tabLine,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, XtParent(toolBar),
			      NULL);
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, tabLine,
			      NULL);
	    }
	    else
#endif
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, XtParent(toolBar),
			      NULL);
	}
	else
#endif
	{
#ifdef FEAT_GUI_TABLINE
	    if (showing_tabline)
	    {
		XtVaSetValues(tabLine,
			      XmNtopAttachment, XmATTACH_FORM,
			      NULL);
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, tabLine,
			      NULL);
	    }
	    else
#endif
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_FORM,
			      NULL);
	}
    }

}

/*
 * Enable or disable mnemonics for the toplevel menus.
 */
    void
gui_motif_set_mnemonics(int enable)
{
    /*
     * Don't enable menu mnemonics when the menu bar is disabled, LessTif
     * crashes when using a mnemonic then.
     */
    if (!menu_enabled)
	enable = FALSE;
    do_set_mnemonics(enable);
}

    static void
do_set_mnemonics(int enable)
{
    vimmenu_T	*menu;

    FOR_ALL_MENUS(menu)
	if (menu->id != (Widget)0)
	    XtVaSetValues(menu->id,
		    XmNmnemonic, enable ? menu->mnemonic : NUL,
		    NULL);
}

    void
gui_mch_add_menu(vimmenu_T *menu, int idx)
{
    XmString	label;
    Widget	shell;
    vimmenu_T	*parent = menu->parent;

#ifdef MOTIF_POPUP
    if (menu_is_popup(menu->name))
    {
	Arg arg[2];
	int n = 0;

	// Only create the popup menu when it's actually used, otherwise there
	// is a delay when using the right mouse button.
# if (XmVersion <= 1002)
	if (mouse_model_popup())
# endif
	{
	    if (gui.menu_bg_pixel != INVALCOLOR)
	    {
		XtSetArg(arg[0], XmNbackground, gui.menu_bg_pixel); n++;
	    }
	    if (gui.menu_fg_pixel != INVALCOLOR)
	    {
		XtSetArg(arg[1], XmNforeground, gui.menu_fg_pixel); n++;
	    }
	    menu->submenu_id = XmCreatePopupMenu(textArea, "contextMenu",
								      arg, n);
	    menu->id = (Widget)0;
	}
	return;
    }
#endif

    if (!menu_is_menubar(menu->name)
	    || (parent != NULL && parent->submenu_id == (Widget)0))
	return;

    label = XmStringCreate((char *)menu->dname, STRING_TAG);
    if (label == NULL)
	return;
    menu->id = XtVaCreateWidget("subMenu",
	    xmCascadeButtonWidgetClass,
	    (parent == NULL) ? menuBar : parent->submenu_id,
	    XmNlabelString, label,
	    XmNmnemonic, p_wak[0] == 'n' ? NUL : menu->mnemonic,
#if (XmVersion >= 1002)
	    // submenu: count the tearoff item (needed for LessTif)
	    XmNpositionIndex, idx + (parent != NULL
			   && tearoff_val == (int)XmTEAR_OFF_ENABLED ? 1 : 0),
#endif
	    NULL);
    XmStringFree(label);

    if (menu->id == (Widget)0)		// failed
	return;

    // The "Help" menu is a special case, and should be placed at the far
    // right hand side of the menu-bar.  It's recognized by its high priority.
    if (parent == NULL && menu->priority >= 9999)
	XtVaSetValues(menuBar,
		XmNmenuHelpWidget, menu->id,
		NULL);

    gui_motif_menu_colors(menu->id);
    gui_motif_menu_fontlist(menu->id);

    // add accelerator text
    gui_motif_add_actext(menu);

    shell = XtVaCreateWidget("subMenuShell",
	xmMenuShellWidgetClass, menu->id,
	XmNwidth, 1,
	XmNheight, 1,
	NULL);
    gui_motif_menu_colors(shell);
    menu->submenu_id = XtVaCreateWidget("rowColumnMenu",
	xmRowColumnWidgetClass, shell,
	XmNrowColumnType, XmMENU_PULLDOWN,
	NULL);
    gui_motif_menu_colors(menu->submenu_id);

    if (menu->submenu_id == (Widget)0)		// failed
	return;

#if (XmVersion >= 1002)
    // Set the colors for the tear off widget
    toggle_tearoff(menu->submenu_id);
#endif

    XtVaSetValues(menu->id,
	XmNsubMenuId, menu->submenu_id,
	NULL);

    // When we add a top-level item to the menu bar, we can figure out how
    // high the menu bar should be.
    if (parent == NULL)
	gui_mch_compute_menu_height(menu->id);
}


/*
 * Add mnemonic and accelerator text to a menu button.
 */
    static void
gui_motif_add_actext(vimmenu_T *menu)
{
    XmString	label;

    // Add accelerator text, if there is one
    if (menu->actext == NULL || menu->id == (Widget)0)
	return;

    label = XmStringCreate((char *)menu->actext, STRING_TAG);
    if (label == NULL)
	return;
    XtVaSetValues(menu->id, XmNacceleratorText, label, NULL);
    XmStringFree(label);
}

    void
gui_mch_toggle_tearoffs(int enable)
{
#if (XmVersion >= 1002)
    if (enable)
	tearoff_val = (int)XmTEAR_OFF_ENABLED;
    else
	tearoff_val = (int)XmTEAR_OFF_DISABLED;
    toggle_tearoff(menuBar);
    gui_mch_recurse_tearoffs(root_menu);
#endif
}

#if (XmVersion >= 1002)
/*
 * Set the tearoff for one menu widget on or off, and set the color of the
 * tearoff widget.
 */
    static void
toggle_tearoff(Widget wid)
{
    Widget	w;

    XtVaSetValues(wid, XmNtearOffModel, tearoff_val, NULL);
    if (tearoff_val == (int)XmTEAR_OFF_ENABLED
	    && (w = XmGetTearOffControl(wid)) != (Widget)0)
	gui_motif_menu_colors(w);
}

    static void
gui_mch_recurse_tearoffs(vimmenu_T *menu)
{
    while (menu != NULL)
    {
	if (!menu_is_popup(menu->name))
	{
	    if (menu->submenu_id != (Widget)0)
		toggle_tearoff(menu->submenu_id);
	    gui_mch_recurse_tearoffs(menu->children);
	}
	menu = menu->next;
    }
}
#endif

    int
gui_mch_text_area_extra_height(void)
{
    Dimension	shadowHeight;

    XtVaGetValues(textAreaForm, XmNshadowThickness, &shadowHeight, NULL);
    return shadowHeight;
}

/*
 * Compute the height of the menu bar.
 * We need to check all the items for their position and height, for the case
 * there are several rows, and/or some characters extend higher or lower.
 */
    void
gui_mch_compute_menu_height(
    Widget	id)		    // can be NULL when deleting menu
{
    Dimension	y, maxy;
    Dimension	margin, shadow;
    vimmenu_T	*mp;
    static Dimension	height = 21;	// normal height of a menu item

    /*
     * Get the height of the new item, before managing it, because it will
     * still reflect the font size.  After managing it depends on the menu
     * height, which is what we just wanted to get!.
     */
    if (id != (Widget)0)
	XtVaGetValues(id, XmNheight, &height, NULL);

    // Find any menu Widget, to be able to call XtManageChild()
    else
	FOR_ALL_MENUS(mp)
	    if (mp->id != (Widget)0 && menu_is_menubar(mp->name))
	    {
		id = mp->id;
		break;
	    }

    /*
     * Now manage the menu item, to make them all be positioned (makes an
     * extra row when needed, removes it when not needed).
     */
    if (id != (Widget)0)
	XtManageChild(id);

    /*
     * Now find the menu item that is the furthest down, and get its position.
     */
    maxy = 0;
    FOR_ALL_MENUS(mp)
    {
	if (mp->id != (Widget)0 && menu_is_menubar(mp->name))
	{
	    XtVaGetValues(mp->id, XmNy, &y, NULL);
	    if (y > maxy)
		maxy = y;
	}
    }

    XtVaGetValues(menuBar,
	XmNmarginHeight, &margin,
	XmNshadowThickness, &shadow,
	NULL);

    /*
     * This computation is the result of trial-and-error:
     * maxy =	The maximum position of an item; required for when there are
     *		two or more rows
     * height = height of an item, before managing it;	Hopefully this will
     *		change with the font height.  Includes shadow-border.
     * shadow =	shadow-border; must be subtracted from the height.
     * margin = margin around the menu buttons;  Must be added.
     * Add 4 for the underlining of shortcut keys.
     */
    gui.menu_height = maxy + height - 2 * shadow + 2 * margin + 4;

    // Somehow the menu bar doesn't resize automatically.  Set it here,
    // even though this is a catch 22.  Don't do this when starting up,
    // somehow the menu gets very high then.
    if (gui.shell_created)
	XtVaSetValues(menuBar, XmNheight, gui.menu_height, NULL);
}

#ifdef FEAT_TOOLBAR

/*
 * Icons used by the toolbar code.
 */
#include "gui_x11_pm.h"

static char **get_toolbar_pixmap(vimmenu_T *menu, char **fname);

/*
 * Read an Xpm file.  Return OK or FAIL.
 */
    static int
check_xpm(char_u *path)
{
    XpmAttributes attrs;
    int		status;
    Pixmap	mask;
    Pixmap	map;

    attrs.valuemask = 0;

    // Create the "sensitive" pixmap
    status = XpmReadFileToPixmap(gui.dpy,
	    RootWindow(gui.dpy, DefaultScreen(gui.dpy)),
	    (char *)path, &map, &mask, &attrs);
    XpmFreeAttributes(&attrs);

    if (status == XpmSuccess)
	return OK;
    return FAIL;
}


/*
 * Allocated a pixmap for toolbar menu "menu".
 * When it's to be read from a file, "fname" is set to the file name
 * (in allocated memory).
 * Return a blank pixmap if it fails.
 */
    static char **
get_toolbar_pixmap(vimmenu_T *menu, char **fname)
{
    char_u	buf[MAXPATHL];		// buffer storing expanded pathname
    char	**xpm = NULL;		// xpm array
    int		res;

    *fname = NULL;
    buf[0] = NUL;			// start with NULL path

    if (menu->iconfile != NULL)
    {
	// Use the "icon="  argument.
	gui_find_iconfile(menu->iconfile, buf, "xpm");
	res = check_xpm(buf);

	// If it failed, try using the menu name.
	if (res == FAIL && gui_find_bitmap(menu->name, buf, "xpm") == OK)
	    res = check_xpm(buf);
	if (res == OK)
	{
	    *fname = (char *)vim_strsave(buf);
	    return tb_blank_xpm;
	}
    }

    if (menu->icon_builtin || gui_find_bitmap(menu->name, buf, "xpm") == FAIL)
    {
	if (menu->iconidx >= 0 && menu->iconidx
	       < (int)(sizeof(built_in_pixmaps) / sizeof(built_in_pixmaps[0])))
	    xpm = built_in_pixmaps[menu->iconidx];
	else
	    xpm = tb_blank_xpm;
    }

    return xpm;
}

/*
 * Add arguments for the toolbar pixmap to a menu item.
 */
    static int
add_pixmap_args(vimmenu_T *menu, Arg *args, int n)
{
    vim_free(menu->xpm_fname);
    menu->xpm = get_toolbar_pixmap(menu, &menu->xpm_fname);
    if (menu->xpm == NULL)
    {
	XtSetArg(args[n], XmNlabelType, XmSTRING); n++;
    }
    else
    {
	if (menu->xpm_fname != NULL)
	{
	    XtSetArg(args[n], XmNpixmapFile, menu->xpm_fname); n++;
	}
	XtSetArg(args[n], XmNpixmapData, menu->xpm); n++;
	XtSetArg(args[n], XmNlabelLocation, XmBOTTOM); n++;
    }
    return n;
}
#endif // FEAT_TOOLBAR

    void
gui_mch_add_menu_item(vimmenu_T *menu, int idx)
{
    XmString	label;
    vimmenu_T	*parent = menu->parent;

# if (XmVersion <= 1002)
    // Don't add Popup menu items when the popup menu isn't used.
    if (menu_is_child_of_popup(menu) && !mouse_model_popup())
	return;
# endif

# ifdef FEAT_TOOLBAR
    if (menu_is_toolbar(parent->name))
    {
	WidgetClass	type;
	XmString	xms = NULL;    // fallback label if pixmap not found
	int		n;
	Arg		args[18];

	n = 0;
	if (menu_is_separator(menu->name))
	{
	    char	*cp;
	    Dimension	wid;

	    /*
	     * A separator has the format "-sep%d[:%d]-". The optional :%d is
	     * a width specifier. If no width is specified then we choose one.
	     */
	    cp = (char *)vim_strchr(menu->name, ':');
	    if (cp != NULL)
		wid = (Dimension)atoi(++cp);
	    else
		wid = 4;

	    type = xmSeparatorWidgetClass;
	    XtSetArg(args[n], XmNwidth, wid); n++;
	    XtSetArg(args[n], XmNminWidth, wid); n++;
	    XtSetArg(args[n], XmNorientation, XmVERTICAL); n++;
	    XtSetArg(args[n], XmNseparatorType, XmSHADOW_ETCHED_IN); n++;
	}
	else
	{
	    // Without shadows one can't sense whatever the button has been
	    // pressed or not! However we want to save a bit of space...
	    // Need the highlightThickness to see the focus.
	    XtSetArg(args[n], XmNhighlightThickness, 1); n++;
	    XtSetArg(args[n], XmNhighlightOnEnter, True); n++;
	    XtSetArg(args[n], XmNmarginWidth, 0); n++;
	    XtSetArg(args[n], XmNmarginHeight, 0); n++;
	    XtSetArg(args[n], XmNtraversalOn, False); n++;
	    // Set the label here, so that we can switch between icons/text
	    // by changing the XmNlabelType resource.
	    xms = XmStringCreate((char *)menu->dname, STRING_TAG);
	    XtSetArg(args[n], XmNlabelString, xms); n++;

	    n = add_pixmap_args(menu, args, n);

	    type = xmEnhancedButtonWidgetClass;
	}

	XtSetArg(args[n], XmNpositionIndex, idx); n++;
	if (menu->id == NULL)
	{
	    menu->id = XtCreateManagedWidget((char *)menu->dname,
			type, toolBar, args, n);
	    if (menu->id != NULL && type == xmEnhancedButtonWidgetClass)
	    {
		XtAddCallback(menu->id,
			XmNactivateCallback, gui_x11_menu_cb, menu);
	    }
	}
	else
	    XtSetValues(menu->id, args, n);
	if (xms != NULL)
	    XmStringFree(xms);

# ifdef FEAT_BEVAL_GUI
	gui_mch_menu_set_tip(menu);
# endif

	menu->parent = parent;
	menu->submenu_id = NULL;
	// When adding first item to toolbar it might have to be enabled .
	if (!XtIsManaged(XtParent(toolBar))
		    && vim_strchr(p_go, GO_TOOLBAR) != NULL)
	    gui_mch_show_toolbar(TRUE);
	gui.toolbar_height = gui_mch_compute_toolbar_height();
	return;
    } // toolbar menu item
# endif

    // No parent, must be a non-menubar menu
    if (parent->submenu_id == (Widget)0)
	return;

    menu->submenu_id = (Widget)0;

    // Add menu separator
    if (menu_is_separator(menu->name))
    {
	menu->id = XtVaCreateWidget("subMenu",
		xmSeparatorGadgetClass, parent->submenu_id,
#if (XmVersion >= 1002)
		// count the tearoff item (needed for LessTif)
		XmNpositionIndex, idx + (tearoff_val == (int)XmTEAR_OFF_ENABLED
								     ? 1 : 0),
#endif
		NULL);
	gui_motif_menu_colors(menu->id);
	return;
    }

    label = XmStringCreate((char *)menu->dname, STRING_TAG);
    if (label == NULL)
	return;
    menu->id = XtVaCreateWidget("subMenu",
	xmPushButtonWidgetClass, parent->submenu_id,
	XmNlabelString, label,
	XmNmnemonic, menu->mnemonic,
#if (XmVersion >= 1002)
	// count the tearoff item (needed for LessTif)
	XmNpositionIndex, idx + (tearoff_val == (int)XmTEAR_OFF_ENABLED
								     ? 1 : 0),
#endif
	NULL);
    gui_motif_menu_colors(menu->id);
    gui_motif_menu_fontlist(menu->id);
    XmStringFree(label);

    if (menu->id != (Widget)0)
    {
	XtAddCallback(menu->id, XmNactivateCallback, gui_x11_menu_cb,
		(XtPointer)menu);
	// add accelerator text
	gui_motif_add_actext(menu);
    }
}

#if (XmVersion <= 1002) || defined(PROTO)
/*
 * This function will destroy/create the popup menus dynamically,
 * according to the value of 'mousemodel'.
 * This will fix the "right mouse button freeze" that occurs when
 * there exists a popup menu but it isn't managed.
 */
    void
gui_motif_update_mousemodel(vimmenu_T *menu)
{
    int		idx = 0;

    // When GUI hasn't started the menus have not been created.
    if (!gui.in_use)
      return;

    while (menu)
    {
      if (menu->children != NULL)
      {
	  if (menu_is_popup(menu->name))
	  {
	      if (mouse_model_popup())
	      {
		  // Popup menu will be used.  Create the popup menus.
		  gui_mch_add_menu(menu, idx);
		  gui_motif_update_mousemodel(menu->children);
	      }
	      else
	      {
		  // Popup menu will not be used.  Destroy the popup menus.
		  gui_motif_update_mousemodel(menu->children);
		  gui_mch_destroy_menu(menu);
	      }
	  }
      }
      else if (menu_is_child_of_popup(menu))
      {
	  if (mouse_model_popup())
	      gui_mch_add_menu_item(menu, idx);
	  else
	      gui_mch_destroy_menu(menu);
      }
      menu = menu->next;
      ++idx;
    }
}
#endif

    void
gui_mch_new_menu_colors(void)
{
    if (menuBar == (Widget)0)
	return;
    gui_motif_menu_colors(menuBar);
#ifdef FEAT_TOOLBAR
    gui_motif_menu_colors(toolBarFrame);
    gui_motif_menu_colors(toolBar);
#endif

    submenu_change(root_menu, TRUE);
}

    void
gui_mch_new_menu_font(void)
{
    if (menuBar == (Widget)0)
	return;
    submenu_change(root_menu, FALSE);
    {
	Dimension   height;
	Position w, h;

	XtVaGetValues(menuBar, XmNheight, &height, NULL);
	gui.menu_height = height;

	XtVaGetValues(vimShell, XtNwidth, &w, XtNheight, &h, NULL);
	gui_resize_shell(w, h
#ifdef FEAT_XIM
		- xim_get_status_area_height()
#endif
		     );
    }
    gui_set_shellsize(FALSE, TRUE, RESIZE_VERT);
    ui_new_shellsize();
}

#if defined(FEAT_BEVAL_GUI) || defined(PROTO)
    void
gui_mch_new_tooltip_font(void)
{
# ifdef FEAT_TOOLBAR
    vimmenu_T   *menu;

    if (toolBar == (Widget)0)
	return;

    menu = gui_find_menu((char_u *)"ToolBar");
    if (menu != NULL)
	submenu_change(menu, FALSE);
# endif
}

    void
gui_mch_new_tooltip_colors(void)
{
# ifdef FEAT_TOOLBAR
    vimmenu_T   *toolbar;

    if (toolBar == (Widget)0)
	return;

    toolbar = gui_find_menu((char_u *)"ToolBar");
    if (toolbar != NULL)
	submenu_change(toolbar, TRUE);
# endif
}
#endif

    static void
submenu_change(
    vimmenu_T	*menu,
    int		colors)		// TRUE for colors, FALSE for font
{
    vimmenu_T	*mp;

    for (mp = menu; mp != NULL; mp = mp->next)
    {
	if (mp->id != (Widget)0)
	{
	    if (colors)
	    {
		gui_motif_menu_colors(mp->id);
#ifdef FEAT_TOOLBAR
		// For a toolbar item: Free the pixmap and allocate a new one,
		// so that the background color is right.
		if (mp->xpm != NULL)
		{
		    int		n = 0;
		    Arg		args[18];

		    n = add_pixmap_args(mp, args, n);
		    XtSetValues(mp->id, args, n);
		}
# ifdef FEAT_BEVAL_GUI
		// If we have a tooltip, then we need to change its font
		if (mp->tip != NULL)
		{
		    Arg args[2];

		    args[0].name = XmNbackground;
		    args[0].value = gui.tooltip_bg_pixel;
		    args[1].name = XmNforeground;
		    args[1].value = gui.tooltip_fg_pixel;
		    XtSetValues(mp->tip->balloonLabel, &args[0], XtNumber(args));
		}
# endif
#endif
	    }
	    else
	    {
		gui_motif_menu_fontlist(mp->id);
#ifdef FEAT_BEVAL_GUI
		// If we have a tooltip, then we need to change its font
		if (mp->tip != NULL)
		{
		    Arg args[1];

		    args[0].name = XmNfontList;
		    args[0].value = (XtArgVal)gui_motif_fontset2fontlist(
						    &gui.tooltip_fontset);
		    XtSetValues(mp->tip->balloonLabel, &args[0], XtNumber(args));
		}
#endif
	    }
	}

	if (mp->children != NULL)
	{
#if (XmVersion >= 1002)
	    // Set the colors/font for the tear off widget
	    if (mp->submenu_id != (Widget)0)
	    {
		if (colors)
		    gui_motif_menu_colors(mp->submenu_id);
		else
		    gui_motif_menu_fontlist(mp->submenu_id);
		toggle_tearoff(mp->submenu_id);
	    }
#endif
	    // Set the colors for the children
	    submenu_change(mp->children, colors);
	}
    }
}

/*
 * Destroy the machine specific menu widget.
 */
    void
gui_mch_destroy_menu(vimmenu_T *menu)
{
    // Please be sure to destroy the parent widget first (i.e. menu->id).
    // On the other hand, problems have been reported that the submenu must be
    // deleted first...
    if (menu->submenu_id != (Widget)0)
    {
	XtDestroyWidget(menu->submenu_id);
	menu->submenu_id = (Widget)0;
    }

    if (menu->id == (Widget)0)
	return;

    Widget	    parent;

    parent = XtParent(menu->id);
#if defined(FEAT_TOOLBAR) && defined(FEAT_BEVAL_GUI)
    if (parent == toolBar && menu->tip != NULL)
    {
	// We try to destroy this before the actual menu, because there are
	// callbacks, etc. that will be unregistered during the tooltip
	// destruction.
	//
	// If you call "gui_mch_destroy_beval_area()" after destroying
	// menu->id, then the tooltip's window will have already been
	// deallocated by Xt, and unknown behaviour will ensue (probably
	// a core dump).
	gui_mch_destroy_beval_area(menu->tip);
	menu->tip = NULL;
    }
#endif
    XtDestroyWidget(menu->id);
    menu->id = (Widget)0;
    if (parent == menuBar)
	gui_mch_compute_menu_height((Widget)0);
#ifdef FEAT_TOOLBAR
    else if (parent == toolBar)
    {
	Cardinal    num_children;

	// When removing last toolbar item, don't display the toolbar.
	XtVaGetValues(toolBar, XmNnumChildren, &num_children, NULL);
	if (num_children == 0)
	    gui_mch_show_toolbar(FALSE);
	else
	    gui.toolbar_height = gui_mch_compute_toolbar_height();
    }
#endif
}

    void
gui_mch_show_popupmenu(vimmenu_T *menu UNUSED)
{
#ifdef MOTIF_POPUP
    XmMenuPosition(menu->submenu_id, gui_x11_get_last_mouse_event());
    XtManageChild(menu->submenu_id);
#endif
}

#endif // FEAT_MENU

/*
 * Set the menu and scrollbar colors to their default values.
 */
    void
gui_mch_def_colors(void)
{
    if (!gui.in_use)
	return;

    gui.menu_fg_pixel = gui_get_color((char_u *)gui.rsrc_menu_fg_name);
    gui.menu_bg_pixel = gui_get_color((char_u *)gui.rsrc_menu_bg_name);
    gui.scroll_fg_pixel = gui_get_color((char_u *)gui.rsrc_scroll_fg_name);
    gui.scroll_bg_pixel = gui_get_color((char_u *)gui.rsrc_scroll_bg_name);
#ifdef FEAT_BEVAL_GUI
    gui.tooltip_fg_pixel =
	gui_get_color((char_u *)gui.rsrc_tooltip_fg_name);
    gui.tooltip_bg_pixel =
	gui_get_color((char_u *)gui.rsrc_tooltip_bg_name);
#endif
}


/*
 * Scrollbar stuff.
 */

    void
gui_mch_set_scrollbar_thumb(
    scrollbar_T *sb,
    long	val,
    long	size,
    long	max)
{
    if (sb->id != (Widget)0)
	XtVaSetValues(sb->id,
		  XmNvalue, val,
		  XmNsliderSize, size,
		  XmNpageIncrement, (size > 2 ? size - 2 : 1),
		  XmNmaximum, max + 1,	    // Motif has max one past the end
		  NULL);
}

    void
gui_mch_set_scrollbar_pos(
    scrollbar_T *sb,
    int		x,
    int		y,
    int		w,
    int		h)
{
    if (sb->id == (Widget)0)
	return;

    if (sb->type == SBAR_LEFT || sb->type == SBAR_RIGHT)
    {
	if (y == 0)
	    h -= gui.border_offset;
	else
	    y -= gui.border_offset;
	XtVaSetValues(sb->id,
		XmNtopOffset, y,
		XmNbottomOffset, -y - h,
		XmNwidth, w,
		NULL);
    }
    else
	XtVaSetValues(sb->id,
		XmNtopOffset, y,
		XmNleftOffset, x,
		XmNrightOffset, gui.which_scrollbars[SBAR_RIGHT]
						     ? gui.scrollbar_width : 0,
		XmNheight, h,
		NULL);
    XtManageChild(sb->id);
}

    int
gui_mch_get_scrollbar_xpadding(void)
{
    int xpad;
    Dimension tw, ww;
    Position  tx;

    XtVaGetValues(textArea, XtNwidth, &tw, XtNx, &tx, NULL);
    XtVaGetValues(vimShell, XtNwidth, &ww, NULL);
    xpad = ww - tw - tx - gui.scrollbar_width;
    return (xpad < 0) ? 0 : xpad;
}

    int
gui_mch_get_scrollbar_ypadding(void)
{
    int ypad;
    Dimension th, wh;
    Position  ty;

    XtVaGetValues(textArea, XtNheight, &th, XtNy, &ty, NULL);
    XtVaGetValues(vimShell, XtNheight, &wh, NULL);
    ypad = wh - th - ty - gui.scrollbar_height;
    return (ypad < 0) ? 0 : ypad;
}

    void
gui_mch_enable_scrollbar(scrollbar_T *sb, int flag)
{
    Arg		args[16];
    int		n;

    if (sb->id == (Widget)0)
	return;

    n = 0;
    if (flag)
    {
	switch (sb->type)
	{
	    case SBAR_LEFT:
		XtSetArg(args[n], XmNleftOffset, gui.scrollbar_width); n++;
		break;

	    case SBAR_RIGHT:
		XtSetArg(args[n], XmNrightOffset, gui.scrollbar_width); n++;
		break;

	    case SBAR_BOTTOM:
		XtSetArg(args[n], XmNbottomOffset, gui.scrollbar_height);n++;
		break;
	}
	XtSetValues(textArea, args, n);
	XtManageChild(sb->id);
    }
    else
    {
	if (!gui.which_scrollbars[sb->type])
	{
	    // The scrollbars of this type are all disabled, adjust the
	    // textArea attachment offset.
	    switch (sb->type)
	    {
		case SBAR_LEFT:
		    XtSetArg(args[n], XmNleftOffset, 0); n++;
		    break;

		case SBAR_RIGHT:
		    XtSetArg(args[n], XmNrightOffset, 0); n++;
		    break;

		case SBAR_BOTTOM:
		    XtSetArg(args[n], XmNbottomOffset, 0);n++;
		    break;
	    }
	    XtSetValues(textArea, args, n);
	}
	XtUnmanageChild(sb->id);
    }
}

    void
gui_mch_create_scrollbar(
    scrollbar_T *sb,
    int		orient)	// SBAR_VERT or SBAR_HORIZ
{
    Arg		args[16];
    int		n = 0;

    XtSetArg(args[n], XmNminimum, 0); n++;
    XtSetArg(args[n], XmNorientation,
	    (orient == SBAR_VERT) ? XmVERTICAL : XmHORIZONTAL); n++;

    switch (sb->type)
    {
	case SBAR_LEFT:
	    XtSetArg(args[n], XmNtopAttachment, XmATTACH_FORM); n++;
	    XtSetArg(args[n], XmNbottomAttachment, XmATTACH_OPPOSITE_FORM); n++;
	    XtSetArg(args[n], XmNleftAttachment, XmATTACH_FORM); n++;
	    XtSetArg(args[n], XmNwidth, gui.scrollbar_width); n++;
	    break;

	case SBAR_RIGHT:
	    XtSetArg(args[n], XmNtopAttachment, XmATTACH_FORM); n++;
	    XtSetArg(args[n], XmNbottomAttachment, XmATTACH_OPPOSITE_FORM); n++;
	    XtSetArg(args[n], XmNrightAttachment, XmATTACH_FORM); n++;
	    XtSetArg(args[n], XmNwidth, gui.scrollbar_width); n++;
	    break;

	case SBAR_BOTTOM:
	    XtSetArg(args[n], XmNleftAttachment, XmATTACH_FORM); n++;
	    XtSetArg(args[n], XmNrightAttachment, XmATTACH_FORM); n++;
	    XtSetArg(args[n], XmNbottomAttachment, XmATTACH_FORM); n++;
	    XtSetArg(args[n], XmNheight, gui.scrollbar_height); n++;
	    break;
    }

    sb->id = XtCreateWidget("scrollBar",
	    xmScrollBarWidgetClass, textAreaForm, args, n);
    if (sb->id == (Widget)0)
	return;

    gui_mch_set_scrollbar_colors(sb);
    XtAddCallback(sb->id, XmNvalueChangedCallback,
	    scroll_cb, (XtPointer)sb->ident);
    XtAddCallback(sb->id, XmNdragCallback,
	    scroll_cb, (XtPointer)sb->ident);
    XtAddEventHandler(sb->id, KeyPressMask, FALSE, gui_x11_key_hit_cb,
	    (XtPointer)0);
}

    void
gui_mch_destroy_scrollbar(scrollbar_T *sb)
{
    if (sb->id != (Widget)0)
	XtDestroyWidget(sb->id);
}

    void
gui_mch_set_scrollbar_colors(scrollbar_T *sb)
{
    if (sb->id != (Widget)0)
    {
	if (gui.scroll_bg_pixel != INVALCOLOR)
	{
#if (XmVersion>=1002)
	    // This should not only set the through color but also adjust
	    // related colors, such as shadows.
	    XmChangeColor(sb->id, gui.scroll_bg_pixel);
#endif

	    // Set the through color directly, in case XmChangeColor() decided
	    // to change it.
	    XtVaSetValues(sb->id,
		    XmNtroughColor, gui.scroll_bg_pixel,
		    NULL);
	}

	if (gui.scroll_fg_pixel != INVALCOLOR)
	    XtVaSetValues(sb->id,
		    XmNforeground, gui.scroll_fg_pixel,
		    XmNbackground, gui.scroll_fg_pixel,
		    NULL);
    }

    // This is needed for the rectangle below the vertical scrollbars.
    if (sb == &gui.bottom_sbar && textAreaForm != (Widget)0)
	gui_motif_scroll_colors(textAreaForm);
}

/*
 * Miscellaneous stuff:
 */

    Window
gui_x11_get_wid(void)
{
    return(XtWindow(textArea));
}

/*
 * Look for a widget in the widget tree w, with a mnemonic matching keycode.
 * When one is found, simulate a button press on that widget and give it the
 * keyboard focus.  If the mnemonic is on a label, look in the userData field
 * of the label to see if it points to another widget, and give that the focus.
 */
    static void
do_mnemonic(Widget w, unsigned int keycode)
{
    WidgetList	    children;
    int		    numChildren, i;
    Boolean	    isMenu;
    KeySym	    mnemonic = '\0';
    char	    mneString[2];
    Widget	    userData;
    unsigned char   rowColType;

    if (XtIsComposite(w))
    {
	if (XtClass(w) == xmRowColumnWidgetClass)
	{
	    XtVaGetValues(w, XmNrowColumnType, &rowColType, NULL);
	    isMenu = (rowColType != (unsigned char)XmWORK_AREA);
	}
	else
	    isMenu = False;
	if (!isMenu)
	{
	    XtVaGetValues(w, XmNchildren, &children, XmNnumChildren,
			  &numChildren, NULL);
	    for (i = 0; i < numChildren; i++)
		do_mnemonic(children[i], keycode);
	}
    }
    else
    {
	XtVaGetValues(w, XmNmnemonic, &mnemonic, NULL);
	if (mnemonic != '\0')
	{
	    mneString[0] = mnemonic;
	    mneString[1] = '\0';
	    if (XKeysymToKeycode(XtDisplay(XtParent(w)),
				       XStringToKeysym(mneString)) == keycode)
	    {
		if (XtClass(w) == xmLabelWidgetClass
			|| XtClass(w) == xmLabelGadgetClass)
		{
		    XtVaGetValues(w, XmNuserData, &userData, NULL);
		    if (userData != NULL && XtIsWidget(userData))
			XmProcessTraversal(userData, XmTRAVERSE_CURRENT);
		}
		else
		{
		    XKeyPressedEvent keyEvent;

		    XmProcessTraversal(w, XmTRAVERSE_CURRENT);

		    CLEAR_FIELD(keyEvent);
		    keyEvent.type = KeyPress;
		    keyEvent.serial = 1;
		    keyEvent.send_event = True;
		    keyEvent.display = XtDisplay(w);
		    keyEvent.window = XtWindow(w);
		    XtCallActionProc(w, "Activate", (XEvent *) & keyEvent,
								     NULL, 0);
		}
	    }
	}
    }
}

/*
 * Callback routine for dialog mnemonic processing.
 */
    static void
mnemonic_event(
	Widget	    w,
	XtPointer   call_data UNUSED,
	XKeyEvent   *event,
	Boolean	    *b UNUSED)
{
    do_mnemonic(w, event->keycode);
}


/*
 * Search the widget tree under w for widgets with mnemonics.  When found, add
 * a passive grab to the dialog widget for the mnemonic character, thus
 * directing mnemonic events to the dialog widget.
 */
    static void
add_mnemonic_grabs(Widget dialog, Widget w)
{
    char	    mneString[2];
    WidgetList	    children;
    int		    numChildren, i;
    Boolean	    isMenu;
    KeySym	    mnemonic = '\0';
    unsigned char   rowColType;

    if (XtIsComposite(w))
    {
	if (XtClass(w) == xmRowColumnWidgetClass)
	{
	    XtVaGetValues(w, XmNrowColumnType, &rowColType, NULL);
	    isMenu = (rowColType != (unsigned char)XmWORK_AREA);
	}
	else
	    isMenu = False;
	if (!isMenu)
	{
	    XtVaGetValues(w, XmNchildren, &children, XmNnumChildren,
							  &numChildren, NULL);
	    for (i = 0; i < numChildren; i++)
		add_mnemonic_grabs(dialog, children[i]);
	}
    }
    else
    {
	XtVaGetValues(w, XmNmnemonic, &mnemonic, NULL);
	if (mnemonic != '\0')
	{
	    mneString[0] = mnemonic;
	    mneString[1] = '\0';
	    XtGrabKey(dialog, XKeysymToKeycode(XtDisplay(dialog),
						  XStringToKeysym(mneString)),
		    Mod1Mask, True, GrabModeAsync, GrabModeAsync);
	}
    }
}

/*
 * Add a handler for mnemonics in a dialog.  Motif itself only handles
 * mnemonics in menus. Mnemonics added or changed after this call will be
 * ignored.
 *
 * To add a mnemonic to a text field or list, set the XmNmnemonic resource on
 * the appropriate label and set the XmNuserData resource of the label to the
 * widget to get the focus when the mnemonic is typed.
 */
    static void
activate_dialog_mnemonics(Widget dialog)
{
    if (!dialog)
	return;

    XtAddEventHandler(dialog, KeyPressMask, False,
			   (XtEventHandler) mnemonic_event, (XtPointer) NULL);
    add_mnemonic_grabs(dialog, dialog);
}

/*
 * Removes the event handler and key-grabs for dialog mnemonic handling.
 */
    static void
suppress_dialog_mnemonics(Widget dialog)
{
    if (!dialog)
	return;

    XtUngrabKey(dialog, AnyKey, Mod1Mask);
    XtRemoveEventHandler(dialog, KeyPressMask, False,
			   (XtEventHandler) mnemonic_event, (XtPointer) NULL);
}

#if defined(FEAT_BROWSE) || defined(FEAT_GUI_DIALOG)
/*
 * Use the 'guifont' or 'guifontset' as a fontlist for a dialog widget.
 */
    static void
set_fontlist(Widget id)
{
    XmFontList fl;

#ifdef FONTSET_ALWAYS
    if (gui.fontset != NOFONTSET)
    {
	fl = gui_motif_fontset2fontlist((XFontSet *)&gui.fontset);
	if (fl != NULL)
	{
	    if (XtIsManaged(id))
	    {
		XtUnmanageChild(id);
		XtVaSetValues(id, XmNfontList, fl, NULL);
		// We should force the widget to recalculate its
		// geometry now.
		XtManageChild(id);
	    }
	    else
		XtVaSetValues(id, XmNfontList, fl, NULL);
	    XmFontListFree(fl);
	}
    }
#else
    if (gui.norm_font != NOFONT)
    {
	fl = gui_motif_create_fontlist((XFontStruct *)gui.norm_font);
	if (fl != NULL)
	{
	    if (XtIsManaged(id))
	    {
		XtUnmanageChild(id);
		XtVaSetValues(id, XmNfontList, fl, NULL);
		// We should force the widget to recalculate its
		// geometry now.
		XtManageChild(id);
	    }
	    else
		XtVaSetValues(id, XmNfontList, fl, NULL);
	    XmFontListFree(fl);
	}
    }
#endif
}
#endif

#if defined(FEAT_BROWSE) || defined(PROTO)

/*
 * file selector related stuff
 */

#include <Xm/FileSB.h>
#include <Xm/XmStrDefs.h>

typedef struct dialog_callback_arg
{
    char *  args;   // not used right now
    int	    id;
} dcbarg_T;

static Widget dialog_wgt;
static char *browse_fname = NULL;
static XmStringCharSet charset = (XmStringCharSet) XmSTRING_DEFAULT_CHARSET;
				// used to set up XmStrings

static void DialogCancelCB(Widget, XtPointer, XtPointer);
static void DialogAcceptCB(Widget, XtPointer, XtPointer);

/*
 * This function is used to translate the predefined label text of the
 * precomposed dialogs. We do this explicitly to allow:
 *
 * - usage of gettext for translation, as in all the other places.
 *
 * - equalize the messages between different GUI implementations as far as
 * possible.
 */
    static void
set_predefined_label(Widget parent, String name, char *new_label)
{
    XmString	str;
    Widget	w;
    char_u	*p, *next;
    KeySym	mnemonic = NUL;

    w = XtNameToWidget(parent, name);

    if (!w)
	return;

    p = vim_strsave((char_u *)new_label);
    if (p == NULL)
	return;
    for (next = p; *next; ++next)
    {
	if (*next == DLG_HOTKEY_CHAR)
	{
	    int len = STRLEN(next);

	    if (len > 0)
	    {
		mch_memmove(next, next + 1, len);
		mnemonic = next[0];
	    }
	}
    }

    str = XmStringCreate((char *)p, STRING_TAG);
    vim_free(p);

    if (str != NULL)
    {
	XtVaSetValues(w,
		XmNlabelString, str,
		XmNmnemonic, mnemonic,
		NULL);
	XmStringFree(str);
    }
    gui_motif_menu_fontlist(w);
}

    static void
set_predefined_fontlist(Widget parent, String name)
{
    Widget w;
    w = XtNameToWidget(parent, name);

    if (!w)
	return;

    set_fontlist(w);
}

/*
 * Put up a file requester.
 * Returns the selected name in allocated memory, or NULL for Cancel.
 */
    char_u *
gui_mch_browse(
    int		saving UNUSED,	// select file to write
    char_u	*title,		// title for the window
    char_u	*dflt,		// default name
    char_u	*ext UNUSED,	// not used (extension added)
    char_u	*initdir,	// initial directory, NULL for current dir
    char_u	*filter)	// file name filter
{
    char_u	dirbuf[MAXPATHL];
    char_u	dfltbuf[MAXPATHL];
    char_u	*pattern;
    char_u	*tofree = NULL;

    // There a difference between the resource name and value, Therefore, we
    // avoid to (ab-)use the (maybe internationalized!) dialog title as a
    // dialog name.

    dialog_wgt = XmCreateFileSelectionDialog(vimShell, "browseDialog", NULL, 0);

    if (initdir == NULL || *initdir == NUL)
    {
	mch_dirname(dirbuf, MAXPATHL);
	initdir = dirbuf;
    }

    if (dflt == NULL)
	dflt = (char_u *)"";
    else if (STRLEN(initdir) + STRLEN(dflt) + 2 < MAXPATHL)
    {
	// The default selection should be the full path, "dflt" is only the
	// file name.
	STRCPY(dfltbuf, initdir);
	add_pathsep(dfltbuf);
	STRCAT(dfltbuf, dflt);
	dflt = dfltbuf;
    }

    // Can only use one pattern for a file name.  Get the first pattern out of
    // the filter.  An empty pattern means everything matches.
    if (filter == NULL)
	pattern = (char_u *)"";
    else
    {
	char_u	*s, *p;

	s = filter;
	for (p = filter; *p != NUL; ++p)
	{
	    if (*p == '\t')	// end of description, start of pattern
		s = p + 1;
	    if (*p == ';' || *p == '\n')	// end of (first) pattern
		break;
	}
	pattern = vim_strnsave(s, p - s);
	tofree = pattern;
	if (pattern == NULL)
	    pattern = (char_u *)"";
    }

    XtVaSetValues(dialog_wgt,
	XtVaTypedArg,
	    XmNdirectory, XmRString, (char *)initdir, STRLEN(initdir) + 1,
	XtVaTypedArg,
	    XmNdirSpec,	XmRString, (char *)dflt, STRLEN(dflt) + 1,
	XtVaTypedArg,
	    XmNpattern,	XmRString, (char *)pattern, STRLEN(pattern) + 1,
	XtVaTypedArg,
	    XmNdialogTitle, XmRString, (char *)title, STRLEN(title) + 1,
	NULL);

    set_predefined_label(dialog_wgt, "Apply", _("&Filter"));
    set_predefined_label(dialog_wgt, "Cancel", _("&Cancel"));
    set_predefined_label(dialog_wgt, "Dir", _("Directories"));
    set_predefined_label(dialog_wgt, "FilterLabel", _("Filter"));
    set_predefined_label(dialog_wgt, "Help", _("&Help"));
    set_predefined_label(dialog_wgt, "Items", _("Files"));
    set_predefined_label(dialog_wgt, "OK", _("&OK"));
    set_predefined_label(dialog_wgt, "Selection", _("Selection"));

    // This is to save us from silly external settings using not fixed with
    // fonts for file selection.
    set_predefined_fontlist(dialog_wgt, "DirListSW.DirList");
    set_predefined_fontlist(dialog_wgt, "ItemsListSW.ItemsList");

    gui_motif_menu_colors(dialog_wgt);
    if (gui.scroll_bg_pixel != INVALCOLOR)
	XtVaSetValues(dialog_wgt, XmNtroughColor, gui.scroll_bg_pixel, NULL);

    XtAddCallback(dialog_wgt, XmNokCallback, DialogAcceptCB, (XtPointer)0);
    XtAddCallback(dialog_wgt, XmNcancelCallback, DialogCancelCB, (XtPointer)0);
    // We have no help in this window, so hide help button
    XtUnmanageChild(XmFileSelectionBoxGetChild(dialog_wgt,
					(unsigned char)XmDIALOG_HELP_BUTTON));

    manage_centered(dialog_wgt);
    activate_dialog_mnemonics(dialog_wgt);

    // sit in a loop until the dialog box has gone away
    do
    {
	XtAppProcessEvent(XtWidgetToApplicationContext(dialog_wgt),
	    (XtInputMask)XtIMAll);
    } while (XtIsManaged(dialog_wgt));

    suppress_dialog_mnemonics(dialog_wgt);
    XtDestroyWidget(dialog_wgt);
    vim_free(tofree);

    if (browse_fname == NULL)
	return NULL;
    return vim_strsave((char_u *)browse_fname);
}

/*
 * The code below was originally taken from
 *	/usr/examples/motif/xmsamplers/xmeditor.c
 * on Digital Unix 4.0d, but heavily modified.
 */

/*
 * Process callback from Dialog cancel actions.
 */
    static void
DialogCancelCB(
    Widget	w UNUSED,		//  widget id
    XtPointer	client_data UNUSED,	//  data from application
    XtPointer	call_data UNUSED)	//  data from widget class
{
    if (browse_fname != NULL)
    {
	XtFree(browse_fname);
	browse_fname = NULL;
    }
    XtUnmanageChild(dialog_wgt);
}

/*
 * Process callback from Dialog actions.
 */
    static void
DialogAcceptCB(
    Widget	w UNUSED,		//  widget id
    XtPointer	client_data UNUSED,	//  data from application
    XtPointer	call_data)		//  data from widget class
{
    XmFileSelectionBoxCallbackStruct *fcb;

    if (browse_fname != NULL)
    {
	XtFree(browse_fname);
	browse_fname = NULL;
    }
    fcb = (XmFileSelectionBoxCallbackStruct *)call_data;

    // get the filename from the file selection box
    XmStringGetLtoR(fcb->value, charset, &browse_fname);

    // popdown the file selection box
    XtUnmanageChild(dialog_wgt);
}

#endif // FEAT_BROWSE

#if defined(FEAT_GUI_DIALOG) || defined(PROTO)

static int	dialogStatus;

/*
 * Callback function for the textfield.  When CR is hit this works like
 * hitting the "OK" button, ESC like "Cancel".
 */
    static void
keyhit_callback(
    Widget		w,
    XtPointer		client_data UNUSED,
    XEvent		*event,
    Boolean		*cont UNUSED)
{
    char	buf[2];
    KeySym	key_sym;

    if (XLookupString(&(event->xkey), buf, 2, &key_sym, NULL) == 1)
    {
	if (*buf == CAR)
	    dialogStatus = 1;
	else if (*buf == ESC)
	    dialogStatus = 2;
    }
    if ((key_sym == XK_Left || key_sym == XK_Right)
	    && !(event->xkey.state & ShiftMask))
	XmTextFieldClearSelection(w, XtLastTimestampProcessed(gui.dpy));
}

    static void
butproc(
    Widget	w UNUSED,
    XtPointer	client_data,
    XtPointer	call_data UNUSED)
{
    dialogStatus = (int)(long)client_data + 1;
}

#ifdef HAVE_XPM

    static Widget
create_pixmap_label(
    Widget	parent,
    String	name,
    char	**data,
    ArgList	args,
    Cardinal	arg)
{
    Widget		label;
    Display		*dsp;
    Screen		*scr;
    int			depth;
    Pixmap		pixmap = 0;
    XpmAttributes	attr;
    Boolean		rs;
    XpmColorSymbol	color[5] =
    {
	{"none", NULL, 0},
	{"iconColor1", NULL, 0},
	{"bottomShadowColor", NULL, 0},
	{"topShadowColor", NULL, 0},
	{"selectColor", NULL, 0}
    };

    label = XmCreateLabelGadget(parent, name, args, arg);

    /*
     * We need to be careful here, since in case of gadgets, there is
     * no way to get the background color directly from the widget itself.
     * In such cases we get it from The Core part of his parent instead.
     */
    dsp = XtDisplayOfObject(label);
    scr = XtScreenOfObject(label);
    XtVaGetValues(XtIsSubclass(label, coreWidgetClass)
	    ?  label : XtParent(label),
		  XmNdepth, &depth,
		  XmNbackground, &color[0].pixel,
		  XmNforeground, &color[1].pixel,
		  XmNbottomShadowColor, &color[2].pixel,
		  XmNtopShadowColor, &color[3].pixel,
		  XmNhighlight, &color[4].pixel,
		  NULL);

    attr.valuemask = XpmColorSymbols | XpmCloseness | XpmDepth;
    attr.colorsymbols = color;
    attr.numsymbols = 5;
    attr.closeness = 65535;
    attr.depth = depth;
    XpmCreatePixmapFromData(dsp, RootWindowOfScreen(scr),
		    data, &pixmap, NULL, &attr);

    XtVaGetValues(label, XmNrecomputeSize, &rs, NULL);
    XtVaSetValues(label, XmNrecomputeSize, True, NULL);
    XtVaSetValues(label,
	    XmNlabelType, XmPIXMAP,
	    XmNlabelPixmap, pixmap,
	    NULL);
    XtVaSetValues(label, XmNrecomputeSize, rs, NULL);

    return label;
}
#endif

    int
gui_mch_dialog(
    int		type UNUSED,
    char_u	*title,
    char_u	*message,
    char_u	*button_names,
    int		dfltbutton,
    char_u	*textfield,		// buffer of size IOSIZE
    int		ex_cmd UNUSED)
{
    char_u		*buts;
    char_u		*p, *next;
    XtAppContext	app;
    XmString		label;
    int			butcount;
    Widget		w;
    Widget		dialogform = NULL;
    Widget		form = NULL;
    Widget		dialogtextfield = NULL;
    Widget		*buttons;
    Widget		sep_form = NULL;
    Boolean		vertical;
    Widget		separator = NULL;
    int			n;
    Arg			args[6];
#ifdef HAVE_XPM
    char		**icon_data = NULL;
    Widget		dialogpixmap = NULL;
#endif

    if (title == NULL)
	title = (char_u *)_("Vim dialog");

    // if our pointer is currently hidden, then we should show it.
    gui_mch_mousehide(FALSE);

    dialogform = XmCreateFormDialog(vimShell, (char *)"dialog", NULL, 0);

    // Check 'v' flag in 'guioptions': vertical button placement.
    vertical = (vim_strchr(p_go, GO_VERTICAL) != NULL);

    // Set the title of the Dialog window
    label = XmStringCreateSimple((char *)title);
    if (label == NULL)
	return -1;
    XtVaSetValues(dialogform,
	    XmNdialogTitle, label,
	    XmNhorizontalSpacing, 4,
	    XmNverticalSpacing, vertical ? 0 : 4,
	    NULL);
    XmStringFree(label);

    // make a copy, so that we can insert NULs
    buts = vim_strsave(button_names);
    if (buts == NULL)
	return -1;

    // Count the number of buttons and allocate buttons[].
    butcount = 1;
    for (p = buts; *p; ++p)
	if (*p == DLG_BUTTON_SEP)
	    ++butcount;
    buttons = ALLOC_MULT(Widget, butcount);
    if (buttons == NULL)
    {
	vim_free(buts);
	return -1;
    }

    /*
     * Create the buttons.
     */
    sep_form = (Widget) 0;
    p = buts;
    for (butcount = 0; *p; ++butcount)
    {
	KeySym mnemonic = NUL;

	for (next = p; *next; ++next)
	{
	    if (*next == DLG_HOTKEY_CHAR)
	    {
		int len = STRLEN(next);

		if (len > 0)
		{
		    mch_memmove(next, next + 1, len);
		    mnemonic = next[0];
		}
	    }
	    if (*next == DLG_BUTTON_SEP)
	    {
		*next++ = NUL;
		break;
	    }
	}
	label = XmStringCreate(_((char *)p), STRING_TAG);
	if (label == NULL)
	    break;

	buttons[butcount] = XtVaCreateManagedWidget("button",
		xmPushButtonWidgetClass, dialogform,
		XmNlabelString, label,
		XmNmnemonic, mnemonic,
		XmNbottomAttachment, XmATTACH_FORM,
		XmNbottomOffset, 4,
		XmNshowAsDefault, butcount == dfltbutton - 1,
		XmNdefaultButtonShadowThickness, 1,
		NULL);
	XmStringFree(label);
	gui_motif_menu_fontlist(buttons[butcount]);

	// Layout properly.

	if (butcount > 0)
	{
	    if (vertical)
		XtVaSetValues(buttons[butcount],
			XmNtopWidget, buttons[butcount - 1],
			NULL);
	    else
	    {
		if (*next == NUL)
		{
		    XtVaSetValues(buttons[butcount],
			    XmNrightAttachment, XmATTACH_FORM,
			    XmNrightOffset, 4,
			    NULL);

		    // fill in a form as invisible separator
		    sep_form = XtVaCreateWidget("separatorForm",
			    xmFormWidgetClass,	dialogform,
			    XmNleftAttachment, XmATTACH_WIDGET,
			    XmNleftWidget, buttons[butcount - 1],
			    XmNrightAttachment, XmATTACH_WIDGET,
			    XmNrightWidget, buttons[butcount],
			    XmNbottomAttachment, XmATTACH_FORM,
			    XmNbottomOffset, 4,
			    NULL);
		    XtManageChild(sep_form);
		}
		else
		{
		    XtVaSetValues(buttons[butcount],
			    XmNleftAttachment, XmATTACH_WIDGET,
			    XmNleftWidget, buttons[butcount - 1],
			    NULL);
		}
	    }
	}
	else if (!vertical)
	{
	    if (*next == NUL)
	    {
		XtVaSetValues(buttons[0],
			XmNrightAttachment, XmATTACH_FORM,
			XmNrightOffset, 4,
			NULL);

		// fill in a form as invisible separator
		sep_form = XtVaCreateWidget("separatorForm",
			xmFormWidgetClass, dialogform,
			XmNleftAttachment, XmATTACH_FORM,
			XmNleftOffset, 4,
			XmNrightAttachment, XmATTACH_WIDGET,
			XmNrightWidget, buttons[0],
			XmNbottomAttachment, XmATTACH_FORM,
			XmNbottomOffset, 4,
			NULL);
		XtManageChild(sep_form);
	    }
	    else
		XtVaSetValues(buttons[0],
			XmNleftAttachment, XmATTACH_FORM,
			XmNleftOffset, 4,
			NULL);
	}

	XtAddCallback(buttons[butcount], XmNactivateCallback,
			  (XtCallbackProc)butproc, (XtPointer)(long)butcount);
	p = next;
    }
    vim_free(buts);

    separator = (Widget) 0;
    if (butcount > 0)
    {
	// Create the separator for beauty.
	n = 0;
	XtSetArg(args[n], XmNorientation, XmHORIZONTAL); n++;
	XtSetArg(args[n], XmNbottomAttachment, XmATTACH_WIDGET); n++;
	XtSetArg(args[n], XmNbottomWidget, buttons[0]); n++;
	XtSetArg(args[n], XmNbottomOffset, 4); n++;
	XtSetArg(args[n], XmNleftAttachment, XmATTACH_FORM); n++;
	XtSetArg(args[n], XmNrightAttachment, XmATTACH_FORM); n++;
	separator = XmCreateSeparatorGadget(dialogform, "separator", args, n);
	XtManageChild(separator);
    }

    if (textfield != NULL)
    {
	dialogtextfield = XtVaCreateWidget("textField",
		xmTextFieldWidgetClass, dialogform,
		XmNleftAttachment, XmATTACH_FORM,
		XmNrightAttachment, XmATTACH_FORM,
		NULL);
	if (butcount > 0)
	    XtVaSetValues(dialogtextfield,
		    XmNbottomAttachment, XmATTACH_WIDGET,
		    XmNbottomWidget, separator,
		    NULL);
	else
	    XtVaSetValues(dialogtextfield,
		    XmNbottomAttachment, XmATTACH_FORM,
		    NULL);

	set_fontlist(dialogtextfield);
	XmTextFieldSetString(dialogtextfield, (char *)textfield);
	XtManageChild(dialogtextfield);
	XtAddEventHandler(dialogtextfield, KeyPressMask, False,
			    (XtEventHandler)keyhit_callback, (XtPointer)NULL);
    }

    // Form holding both message and pixmap labels
    form = XtVaCreateWidget("separatorForm",
	    xmFormWidgetClass, dialogform,
	    XmNleftAttachment, XmATTACH_FORM,
	    XmNrightAttachment, XmATTACH_FORM,
	    XmNtopAttachment, XmATTACH_FORM,
	    NULL);
    XtManageChild(form);

#ifdef HAVE_XPM
    // Add a pixmap, left of the message.
    switch (type)
    {
	case VIM_GENERIC:
	    icon_data = generic_xpm;
	    break;
	case VIM_ERROR:
	    icon_data = error_xpm;
	    break;
	case VIM_WARNING:
	    icon_data = alert_xpm;
	    break;
	case VIM_INFO:
	    icon_data = info_xpm;
	    break;
	case VIM_QUESTION:
	    icon_data = quest_xpm;
	    break;
	default:
	    icon_data = generic_xpm;
    }

    n = 0;
    XtSetArg(args[n], XmNtopAttachment, XmATTACH_FORM); n++;
    XtSetArg(args[n], XmNtopOffset, 8); n++;
    XtSetArg(args[n], XmNbottomAttachment, XmATTACH_FORM); n++;
    XtSetArg(args[n], XmNbottomOffset, 8); n++;
    XtSetArg(args[n], XmNleftAttachment, XmATTACH_FORM); n++;
    XtSetArg(args[n], XmNleftOffset, 8); n++;

    dialogpixmap = create_pixmap_label(form, "dialogPixmap",
	    icon_data, args, n);
    XtManageChild(dialogpixmap);
#endif

    // Create the dialog message.
    // Since LessTif is apparently having problems with the creation of
    // properly localized string, we use LtoR here. The symptom is that the
    // string is not shown properly in multiple lines as it does in native
    // Motif.
    label = XmStringCreateLtoR((char *)message, STRING_TAG);
    if (label == NULL)
	return -1;
    w = XtVaCreateManagedWidget("dialogMessage",
				xmLabelGadgetClass, form,
				XmNlabelString, label,
				XmNalignment, XmALIGNMENT_BEGINNING,
				XmNtopAttachment, XmATTACH_FORM,
				XmNtopOffset, 8,
#ifdef HAVE_XPM
				XmNleftAttachment, XmATTACH_WIDGET,
				XmNleftWidget, dialogpixmap,
#else
				XmNleftAttachment, XmATTACH_FORM,
#endif
				XmNleftOffset, 8,
				XmNrightAttachment, XmATTACH_FORM,
				XmNrightOffset, 8,
				XmNbottomAttachment, XmATTACH_FORM,
				XmNbottomOffset, 8,
				NULL);
    XmStringFree(label);
    set_fontlist(w);

    if (textfield != NULL)
    {
	XtVaSetValues(form,
		XmNbottomAttachment, XmATTACH_WIDGET,
		XmNbottomWidget, dialogtextfield,
		NULL);
    }
    else
    {
	if (butcount > 0)
	    XtVaSetValues(form,
		    XmNbottomAttachment, XmATTACH_WIDGET,
		    XmNbottomWidget, separator,
		    NULL);
	else
	    XtVaSetValues(form,
		    XmNbottomAttachment, XmATTACH_FORM,
		    NULL);
    }

    if (dfltbutton < 1)
	dfltbutton = 1;
    if (dfltbutton > butcount)
	dfltbutton = butcount;
    XtVaSetValues(dialogform,
	    XmNdefaultButton, buttons[dfltbutton - 1], NULL);
    if (textfield != NULL)
	XtVaSetValues(dialogform, XmNinitialFocus, dialogtextfield, NULL);
    else
	XtVaSetValues(dialogform, XmNinitialFocus, buttons[dfltbutton - 1],
									NULL);

    manage_centered(dialogform);
    activate_dialog_mnemonics(dialogform);

    if (textfield != NULL && *textfield != NUL)
    {
	// This only works after the textfield has been realised.
	XmTextFieldSetSelection(dialogtextfield,
			 (XmTextPosition)0, (XmTextPosition)STRLEN(textfield),
					   XtLastTimestampProcessed(gui.dpy));
	XmTextFieldSetCursorPosition(dialogtextfield,
					   (XmTextPosition)STRLEN(textfield));
    }

    app = XtWidgetToApplicationContext(dialogform);

    // Loop until a button is pressed or the dialog is killed somehow.
    dialogStatus = -1;
    for (;;)
    {
	XtAppProcessEvent(app, (XtInputMask)XtIMAll);
	if (dialogStatus >= 0 || !XtIsManaged(dialogform))
	    break;
    }

    vim_free(buttons);

    if (textfield != NULL)
    {
	p = (char_u *)XmTextGetString(dialogtextfield);
	if (p == NULL || dialogStatus < 0)
	    *textfield = NUL;
	else
	    vim_strncpy(textfield, p, IOSIZE - 1);
	XtFree((char *)p);
    }

    suppress_dialog_mnemonics(dialogform);
    XtDestroyWidget(dialogform);

    return dialogStatus;
}
#endif // FEAT_GUI_DIALOG

#if defined(FEAT_TOOLBAR) || defined(PROTO)
    void
gui_mch_show_toolbar(int showit)
{
    Cardinal	numChildren;	    // how many children toolBar has

    if (toolBar == (Widget)0)
	return;
    XtVaGetValues(toolBar, XmNnumChildren, &numChildren, NULL);
    if (showit && numChildren > 0)
    {
	// Assume that we want to show the toolbar if p_toolbar contains
	// valid option settings, therefore p_toolbar must not be NULL.
	WidgetList  children;

	XtVaGetValues(toolBar, XmNchildren, &children, NULL);
	{
	    void    (*action)(BalloonEval *);
	    int	    text = 0;

	    if (strstr((const char *)p_toolbar, "tooltips"))
		action = &gui_mch_enable_beval_area;
	    else
		action = &gui_mch_disable_beval_area;
	    if (strstr((const char *)p_toolbar, "text"))
		text = 1;
	    else if (strstr((const char *)p_toolbar, "icons"))
		text = -1;
	    if (text != 0)
	    {
		vimmenu_T   *toolbar;
		vimmenu_T   *cur;

		FOR_ALL_MENUS(toolbar)
		    if (menu_is_toolbar(toolbar->dname))
			break;
		// Assumption: toolbar is NULL if there is no toolbar,
		//	       otherwise it contains the toolbar menu structure.
		//
		// Assumption: "numChildren" == the number of items in the list
		//	       of items beginning with toolbar->children.
		if (toolbar)
		{
		    for (cur = toolbar->children; cur; cur = cur->next)
		    {
			Arg	    args[1];
			int	    n = 0;

			// Enable/Disable tooltip (OK to enable while
			// currently enabled).
			if (cur->tip != NULL)
			    (*action)(cur->tip);
			if (!menu_is_separator(cur->name))
			{
			    if (text == 1 || cur->xpm == NULL)
			    {
				XtSetArg(args[n], XmNlabelType, XmSTRING);
				++n;
			    }
			    if (cur->id != NULL)
			    {
				XtUnmanageChild(cur->id);
				XtSetValues(cur->id, args, n);
				XtManageChild(cur->id);
			    }
			}
		    }
		}
	    }
	}
	gui.toolbar_height = gui_mch_compute_toolbar_height();
	XtManageChild(XtParent(toolBar));
#ifdef FEAT_GUI_TABLINE
	if (showing_tabline)
	{
	    XtVaSetValues(tabLine,
			  XmNtopAttachment, XmATTACH_WIDGET,
			  XmNtopWidget, XtParent(toolBar),
			  NULL);
	    XtVaSetValues(textAreaForm,
			  XmNtopAttachment, XmATTACH_WIDGET,
			  XmNtopWidget, tabLine,
			  NULL);
	}
	else
#endif
	    XtVaSetValues(textAreaForm,
			  XmNtopAttachment, XmATTACH_WIDGET,
			  XmNtopWidget, XtParent(toolBar),
			  NULL);
	if (XtIsManaged(menuBar))
	    XtVaSetValues(XtParent(toolBar),
		    XmNtopAttachment, XmATTACH_WIDGET,
		    XmNtopWidget, menuBar,
		    NULL);
	else
	    XtVaSetValues(XtParent(toolBar),
		    XmNtopAttachment, XmATTACH_FORM,
		    NULL);
    }
    else
    {
	gui.toolbar_height = 0;
	if (XtIsManaged(menuBar))
	{
#ifdef FEAT_GUI_TABLINE
	    if (showing_tabline)
	    {
		XtVaSetValues(tabLine,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, menuBar,
			      NULL);
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, tabLine,
			      NULL);
	    }
	    else
#endif
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, menuBar,
			      NULL);
	}
	else
	{
#ifdef FEAT_GUI_TABLINE
	    if (showing_tabline)
	    {
		XtVaSetValues(tabLine,
			      XmNtopAttachment, XmATTACH_FORM,
			      NULL);
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, tabLine,
			      NULL);
	    }
	    else
#endif
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_FORM,
			      NULL);
	}

	XtUnmanageChild(XtParent(toolBar));
    }
    gui_set_shellsize(FALSE, FALSE, RESIZE_VERT);
}

/*
 * A toolbar button has been pushed; now reset the input focus
 * such that the user can type page up/down etc. and have the
 * input go to the editor window, not the button
 */
    static void
reset_focus(void)
{
    if (textArea != NULL)
	XmProcessTraversal(textArea, XmTRAVERSE_CURRENT);
}

    int
gui_mch_compute_toolbar_height(void)
{
    Dimension	borders;
    Dimension	height;		    // total Toolbar height
    Dimension	whgt;		    // height of each widget
    WidgetList	children;	    // list of toolBar's children
    Cardinal	numChildren;	    // how many children toolBar has
    int		i;

    borders = 0;
    height = 0;
    if (toolBar != (Widget)0 && toolBarFrame != (Widget)0)
    {				    // get height of XmFrame parent
	Dimension	fst;
	Dimension	fmh;
	Dimension	tst;
	Dimension	tmh;

	XtVaGetValues(toolBarFrame,
		XmNshadowThickness, &fst,
		XmNmarginHeight, &fmh,
		NULL);
	borders += fst + fmh;
	XtVaGetValues(toolBar,
		XmNshadowThickness, &tst,
		XmNmarginHeight, &tmh,
		XmNchildren, &children,
		XmNnumChildren, &numChildren, NULL);
	borders += tst + tmh;
	for (i = 0; i < (int)numChildren; i++)
	{
	    whgt = 0;
	    XtVaGetValues(children[i], XmNheight, &whgt, NULL);
	    if (height < whgt)
		height = whgt;
	}
    }
#ifdef LESSTIF_VERSION
    // Hack: When starting up we get wrong dimensions.
    if (height < 10)
	height = 24;
#endif

    return (int)(height + (borders << 1));
}

    void
motif_get_toolbar_colors(
    Pixel       *bgp,
    Pixel       *fgp,
    Pixel       *bsp,
    Pixel       *tsp,
    Pixel       *hsp)
{
    XtVaGetValues(toolBar,
	    XmNbackground, bgp,
	    XmNforeground, fgp,
	    XmNbottomShadowColor, bsp,
	    XmNtopShadowColor, tsp,
	    XmNhighlightColor, hsp,
	    NULL);
}
#endif

#if defined(FEAT_GUI_TABLINE) || defined(PROTO)
/*
 * Show or hide the tabline.
 */
    void
gui_mch_show_tabline(int showit)
{
    if (tabLine == (Widget)0)
	return;

    if (!showit != !showing_tabline)
    {
	if (showit)
	{
	    XtManageChild(tabLine);
	    XtUnmanageChild(XtNameToWidget(tabLine, "PageScroller"));
	    XtUnmanageChild(XtNameToWidget(tabLine, "MinorTabScrollerNext"));
	    XtUnmanageChild(XtNameToWidget(tabLine,
					   "MinorTabScrollerPrevious"));
#ifdef FEAT_MENU
# ifdef FEAT_TOOLBAR
	    if (XtIsManaged(XtParent(toolBar)))
		XtVaSetValues(tabLine,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, XtParent(toolBar), NULL);
	    else
# endif
		if (XtIsManaged(menuBar))
		XtVaSetValues(tabLine,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, menuBar, NULL);
	    else
#endif
		XtVaSetValues(tabLine,
			      XmNtopAttachment, XmATTACH_FORM, NULL);
	    XtVaSetValues(textAreaForm,
			  XmNtopAttachment, XmATTACH_WIDGET,
			  XmNtopWidget, tabLine,
			  NULL);
	}
	else
	{
	    XtUnmanageChild(tabLine);
#ifdef FEAT_MENU
# ifdef FEAT_TOOLBAR
	    if (XtIsManaged(XtParent(toolBar)))
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, XtParent(toolBar), NULL);
	    else
# endif
		if (XtIsManaged(menuBar))
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_WIDGET,
			      XmNtopWidget, menuBar, NULL);
	    else
#endif
		XtVaSetValues(textAreaForm,
			      XmNtopAttachment, XmATTACH_FORM, NULL);
	}
	showing_tabline = showit;
    }
}

/*
 * Return TRUE when tabline is displayed.
 */
    int
gui_mch_showing_tabline(void)
{
    return tabLine != (Widget)0 && showing_tabline;
}

/*
 * Update the labels of the tabline.
 */
    void
gui_mch_update_tabline(void)
{
    tabpage_T		*tp;
    int			nr = 1, n;
    Arg			args[10];
    int			curtabidx = 0, currentpage;
    Widget		tab;
    XmNotebookPageInfo	page_info;
    XmNotebookPageStatus page_status;
    int			last_page, tab_count;
    XmString		label_str;
    char		*label_cstr;
    BalloonEval		*beval;

    if (tabLine == (Widget)0)
	return;

    // Add a label for each tab page.  They all contain the same text area.
    for (tp = first_tabpage; tp != NULL; tp = tp->tp_next, ++nr)
    {
	if (tp == curtab)
	    curtabidx = nr;

	page_status = XmNotebookGetPageInfo(tabLine, nr, &page_info);
	if (page_status == XmPAGE_INVALID
		|| page_info.major_tab_widget == (Widget)0)
	{
	    // Add the tab
	    n = 0;
	    XtSetArg(args[n], XmNnotebookChildType, XmMAJOR_TAB); n++;
	    XtSetArg(args[n], XmNtraversalOn, False); n++;
	    XtSetArg(args[n], XmNalignment, XmALIGNMENT_BEGINNING); n++;
	    XtSetArg(args[n], XmNhighlightThickness, 1); n++;
	    XtSetArg(args[n], XmNshadowThickness , 1); n++;
	    tab = XmCreatePushButton(tabLine, "-Empty-", args, n);
	    XtManageChild(tab);
	    beval = gui_mch_create_beval_area(tab, NULL, tabline_balloon_cb,
									NULL);
	    XtVaSetValues(tab, XmNuserData, beval, NULL);
	}
	else
	    tab = page_info.major_tab_widget;

	XtVaSetValues(tab, XmNpageNumber, nr, NULL);

	/*
	 * Change the label text only if it is different
	 */
	XtVaGetValues(tab, XmNlabelString, &label_str, NULL);
	if (XmStringGetLtoR(label_str, XmSTRING_DEFAULT_CHARSET, &label_cstr))
	{
	    get_tabline_label(tp, FALSE);
	    if (STRCMP(label_cstr, NameBuff) != 0)
	    {
		XtVaSetValues(tab, XtVaTypedArg, XmNlabelString, XmRString,
			      NameBuff, STRLEN(NameBuff) + 1, NULL);
		/*
		 * Force a resize of the tab label button
		 */
		XtUnmanageChild(tab);
		XtManageChild(tab);
	    }
	    XtFree(label_cstr);
	}
    }

    tab_count = nr - 1;

    XtVaGetValues(tabLine, XmNlastPageNumber, &last_page, NULL);

    // Remove any old labels.
    while (nr <= last_page)
    {
	if (XmNotebookGetPageInfo(tabLine, nr, &page_info) != XmPAGE_INVALID
	    && page_info.page_number == nr
	    && page_info.major_tab_widget != (Widget)0)
	{
	    XtVaGetValues(page_info.major_tab_widget, XmNuserData, &beval, NULL);
	    if (beval != NULL)
		gui_mch_destroy_beval_area(beval);
	    XtUnmanageChild(page_info.major_tab_widget);
	    XtDestroyWidget(page_info.major_tab_widget);
	}
	nr++;
    }

    XtVaSetValues(tabLine, XmNlastPageNumber, tab_count, NULL);

    XtVaGetValues(tabLine, XmNcurrentPageNumber, &currentpage, NULL);
    if (currentpage != curtabidx)
	XtVaSetValues(tabLine, XmNcurrentPageNumber, curtabidx, NULL);
}

/*
 * Set the current tab to "nr".  First tab is 1.
 */
    void
gui_mch_set_curtab(int nr)
{
    int		currentpage;

    if (tabLine == (Widget)0)
	return;

    XtVaGetValues(tabLine, XmNcurrentPageNumber, &currentpage, NULL);
    if (currentpage != nr)
	XtVaSetValues(tabLine, XmNcurrentPageNumber, nr, NULL);
}
#endif

/*
 * Set the colors of Widget "id" to the menu colors.
 */
    static void
gui_motif_menu_colors(Widget id)
{
    if (gui.menu_bg_pixel != INVALCOLOR)
#if (XmVersion >= 1002)
	XmChangeColor(id, gui.menu_bg_pixel);
#else
	XtVaSetValues(id, XmNbackground, gui.menu_bg_pixel, NULL);
#endif
    if (gui.menu_fg_pixel != INVALCOLOR)
	XtVaSetValues(id, XmNforeground, gui.menu_fg_pixel, NULL);
}

/*
 * Set the colors of Widget "id" to the scrollbar colors.
 */
    static void
gui_motif_scroll_colors(Widget id)
{
    if (gui.scroll_bg_pixel != INVALCOLOR)
#if (XmVersion >= 1002)
	XmChangeColor(id, gui.scroll_bg_pixel);
#else
	XtVaSetValues(id, XmNbackground, gui.scroll_bg_pixel, NULL);
#endif
    if (gui.scroll_fg_pixel != INVALCOLOR)
	XtVaSetValues(id, XmNforeground, gui.scroll_fg_pixel, NULL);
}

/*
 * Set the fontlist for Widget "id" to use gui.menu_fontset or gui.menu_font.
 */
    void
gui_motif_menu_fontlist(Widget id UNUSED)
{
#ifdef FEAT_MENU
#ifdef FONTSET_ALWAYS
    if (gui.menu_fontset != NOFONTSET)
    {
	XmFontList fl;

	fl = gui_motif_fontset2fontlist((XFontSet *)&gui.menu_fontset);
	if (fl != NULL)
	{
	    if (XtIsManaged(id))
	    {
		XtUnmanageChild(id);
		XtVaSetValues(id, XmNfontList, fl, NULL);
		// We should force the widget to recalculate its
		// geometry now.
		XtManageChild(id);
	    }
	    else
		XtVaSetValues(id, XmNfontList, fl, NULL);
	    XmFontListFree(fl);
	}
    }
#else
    if (gui.menu_font != NOFONT)
    {
	XmFontList fl;

	fl = gui_motif_create_fontlist((XFontStruct *)gui.menu_font);
	if (fl != NULL)
	{
	    if (XtIsManaged(id))
	    {
		XtUnmanageChild(id);
		XtVaSetValues(id, XmNfontList, fl, NULL);
		// We should force the widget to recalculate its
		// geometry now.
		XtManageChild(id);
	    }
	    else
		XtVaSetValues(id, XmNfontList, fl, NULL);
	    XmFontListFree(fl);
	}
    }
#endif
#endif
}


/*
 * We don't create it twice for the sake of speed.
 */

typedef struct _SharedFindReplace
{
    Widget dialog;	// the main dialog widget
    Widget wword;	// 'Exact match' check button
    Widget mcase;	// 'match case' check button
    Widget up;		// search direction 'Up' radio button
    Widget down;	// search direction 'Down' radio button
    Widget what;	// 'Find what' entry text widget
    Widget with;	// 'Replace with' entry text widget
    Widget find;	// 'Find Next' action button
    Widget replace;	// 'Replace With' action button
    Widget all;		// 'Replace All' action button
    Widget undo;	// 'Undo' action button

    Widget cancel;
} SharedFindReplace;

static SharedFindReplace find_widgets = {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL};
static SharedFindReplace repl_widgets = {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL};

    static void
find_replace_destroy_callback(
    Widget	w UNUSED,
    XtPointer	client_data,
    XtPointer	call_data UNUSED)
{
    SharedFindReplace *cd = (SharedFindReplace *)client_data;

    if (cd != NULL)
       // suppress_dialog_mnemonics(cd->dialog);
	cd->dialog = (Widget)0;
}

    static void
find_replace_dismiss_callback(
    Widget	w UNUSED,
    XtPointer	client_data,
    XtPointer	call_data UNUSED)
{
    SharedFindReplace *cd = (SharedFindReplace *)client_data;

    if (cd != NULL)
	XtUnmanageChild(cd->dialog);
}

    static void
entry_activate_callback(
    Widget	w UNUSED,
    XtPointer	client_data,
    XtPointer	call_data UNUSED)
{
    XmProcessTraversal((Widget)client_data, XmTRAVERSE_CURRENT);
}

    static void
find_replace_callback(
    Widget	w UNUSED,
    XtPointer	client_data,
    XtPointer	call_data UNUSED)
{
    long_u	flags = (long_u)client_data;
    char	*find_text, *repl_text;
    Boolean	direction_down = TRUE;
    Boolean	wword;
    Boolean	mcase;
    SharedFindReplace *sfr;

    if (flags == FRD_UNDO)
    {
	char_u	*save_cpo = p_cpo;

	// No need to be Vi compatible here.
	p_cpo = empty_option;
	u_undo(1);
	p_cpo = save_cpo;
	gui_update_screen();
	return;
    }

    // Get the search/replace strings from the dialog
    if (flags == FRD_FINDNEXT)
    {
	repl_text = NULL;
	sfr = &find_widgets;
    }
    else
    {
	repl_text = XmTextFieldGetString(repl_widgets.with);
	sfr = &repl_widgets;
    }
    find_text = XmTextFieldGetString(sfr->what);
    XtVaGetValues(sfr->down, XmNset, &direction_down, NULL);
    XtVaGetValues(sfr->wword, XmNset, &wword, NULL);
    XtVaGetValues(sfr->mcase, XmNset, &mcase, NULL);
    if (wword)
	flags |= FRD_WHOLE_WORD;
    if (mcase)
	flags |= FRD_MATCH_CASE;

    (void)gui_do_findrepl((int)flags, (char_u *)find_text, (char_u *)repl_text,
							      direction_down);

    if (find_text != NULL)
	XtFree(find_text);
    if (repl_text != NULL)
	XtFree(repl_text);
}

    static void
find_replace_keypress(
    Widget		w UNUSED,
    SharedFindReplace	*frdp,
    XKeyEvent		*event,
    Boolean		*b UNUSED)
{
    KeySym keysym;

    if (frdp == NULL)
	return;

    keysym = XLookupKeysym(event, 0);

    // the scape key pops the whole dialog down
    if (keysym == XK_Escape)
	XtUnmanageChild(frdp->dialog);
}

    static void
set_label(Widget w, char *label)
{
    XmString	str;
    char_u	*p, *next;
    KeySym	mnemonic = NUL;

    if (!w)
	return;

    p = vim_strsave((char_u *)label);
    if (p == NULL)
	return;
    for (next = p; *next; ++next)
    {
	if (*next == DLG_HOTKEY_CHAR)
	{
	    int len = STRLEN(next);

	    if (len > 0)
	    {
		mch_memmove(next, next + 1, len);
		mnemonic = next[0];
	    }
	}
    }

    str = XmStringCreateSimple((char *)p);
    vim_free(p);
    if (str)
    {
	XtVaSetValues(w,
		XmNlabelString, str,
		XmNmnemonic, mnemonic,
		NULL);
	XmStringFree(str);
    }
    gui_motif_menu_fontlist(w);
}

    static void
find_replace_dialog_create(char_u *arg, int do_replace)
{
    SharedFindReplace	*frdp;
    Widget		separator;
    Widget		input_form;
    Widget		button_form;
    Widget		toggle_form;
    Widget		frame;
    XmString		str;
    int			n;
    Arg			args[6];
    int			wword = FALSE;
    int			mcase = !p_ic;
    Dimension		width;
    Dimension		widest;
    char_u		*entry_text;

    frdp = do_replace ? &repl_widgets : &find_widgets;

    // Get the search string to use.
    entry_text = get_find_dialog_text(arg, &wword, &mcase);

    // If the dialog already exists, just raise it.
    if (frdp->dialog)
    {
	gui_motif_synch_fonts();

	// If the window is already up, just pop it to the top
	if (XtIsManaged(frdp->dialog))
	    XMapRaised(XtDisplay(frdp->dialog),
					    XtWindow(XtParent(frdp->dialog)));
	else
	    XtManageChild(frdp->dialog);
	XtPopup(XtParent(frdp->dialog), XtGrabNone);
	XmProcessTraversal(frdp->what, XmTRAVERSE_CURRENT);

	if (entry_text != NULL)
	    XmTextFieldSetString(frdp->what, (char *)entry_text);
	vim_free(entry_text);

	XtVaSetValues(frdp->wword, XmNset, wword, NULL);
	return;
    }

    // Create a fresh new dialog window
    if (do_replace)
	 str = XmStringCreateSimple(_("VIM - Search and Replace..."));
    else
	 str = XmStringCreateSimple(_("VIM - Search..."));

    n = 0;
    XtSetArg(args[n], XmNautoUnmanage, False); n++;
    XtSetArg(args[n], XmNnoResize, True); n++;
    XtSetArg(args[n], XmNdialogTitle, str); n++;

    frdp->dialog = XmCreateFormDialog(vimShell, "findReplaceDialog", args, n);
    XmStringFree(str);
    XtAddCallback(frdp->dialog, XmNdestroyCallback,
	    find_replace_destroy_callback, frdp);

    button_form = XtVaCreateWidget("buttonForm",
	    xmFormWidgetClass,	frdp->dialog,
	    XmNrightAttachment, XmATTACH_FORM,
	    XmNrightOffset, 4,
	    XmNtopAttachment, XmATTACH_FORM,
	    XmNtopOffset, 4,
	    XmNbottomAttachment, XmATTACH_FORM,
	    XmNbottomOffset, 4,
	    NULL);

    frdp->find = XtVaCreateManagedWidget("findButton",
	    xmPushButtonWidgetClass, button_form,
	    XmNsensitive, True,
	    XmNtopAttachment, XmATTACH_FORM,
	    XmNleftAttachment, XmATTACH_FORM,
	    XmNrightAttachment, XmATTACH_FORM,
	    NULL);
    set_label(frdp->find, _("Find &Next"));

    XtAddCallback(frdp->find, XmNactivateCallback,
	    find_replace_callback,
	    (do_replace ? (XtPointer)FRD_R_FINDNEXT : (XtPointer)FRD_FINDNEXT));

    if (do_replace)
    {
	frdp->replace = XtVaCreateManagedWidget("replaceButton",
		xmPushButtonWidgetClass, button_form,
		XmNtopAttachment, XmATTACH_WIDGET,
		XmNtopWidget, frdp->find,
		XmNleftAttachment, XmATTACH_FORM,
		XmNrightAttachment, XmATTACH_FORM,
		NULL);
	set_label(frdp->replace, _("&Replace"));
	XtAddCallback(frdp->replace, XmNactivateCallback,
		find_replace_callback, (XtPointer)FRD_REPLACE);

	frdp->all = XtVaCreateManagedWidget("replaceAllButton",
		xmPushButtonWidgetClass, button_form,
		XmNtopAttachment, XmATTACH_WIDGET,
		XmNtopWidget, frdp->replace,
		XmNleftAttachment, XmATTACH_FORM,
		XmNrightAttachment, XmATTACH_FORM,
		NULL);
	set_label(frdp->all, _("Replace &All"));
	XtAddCallback(frdp->all, XmNactivateCallback,
		find_replace_callback, (XtPointer)FRD_REPLACEALL);

	frdp->undo = XtVaCreateManagedWidget("undoButton",
		xmPushButtonWidgetClass, button_form,
		XmNtopAttachment, XmATTACH_WIDGET,
		XmNtopWidget, frdp->all,
		XmNleftAttachment, XmATTACH_FORM,
		XmNrightAttachment, XmATTACH_FORM,
		NULL);
	set_label(frdp->undo, _("&Undo"));
	XtAddCallback(frdp->undo, XmNactivateCallback,
		find_replace_callback, (XtPointer)FRD_UNDO);
    }

    frdp->cancel = XtVaCreateManagedWidget("closeButton",
	    xmPushButtonWidgetClass, button_form,
	    XmNleftAttachment, XmATTACH_FORM,
	    XmNrightAttachment, XmATTACH_FORM,
	    XmNbottomAttachment, XmATTACH_FORM,
	    NULL);
    set_label(frdp->cancel, _("&Cancel"));
    XtAddCallback(frdp->cancel, XmNactivateCallback,
	    find_replace_dismiss_callback, frdp);
    gui_motif_menu_fontlist(frdp->cancel);

    XtManageChild(button_form);

    n = 0;
    XtSetArg(args[n], XmNorientation, XmVERTICAL); n++;
    XtSetArg(args[n], XmNrightAttachment, XmATTACH_WIDGET); n++;
    XtSetArg(args[n], XmNrightWidget, button_form); n++;
    XtSetArg(args[n], XmNrightOffset, 4); n++;
    XtSetArg(args[n], XmNtopAttachment, XmATTACH_FORM); n++;
    XtSetArg(args[n], XmNbottomAttachment, XmATTACH_FORM); n++;
    separator = XmCreateSeparatorGadget(frdp->dialog, "separator", args, n);
    XtManageChild(separator);

    input_form = XtVaCreateWidget("inputForm",
	    xmFormWidgetClass,	frdp->dialog,
	    XmNleftAttachment, XmATTACH_FORM,
	    XmNleftOffset, 4,
	    XmNrightAttachment, XmATTACH_WIDGET,
	    XmNrightWidget, separator,
	    XmNrightOffset, 4,
	    XmNtopAttachment, XmATTACH_FORM,
	    XmNtopOffset, 4,
	    NULL);

    {
	Widget label_what;
	Widget label_with = (Widget)0;

	str = XmStringCreateSimple(_("Find what:"));
	label_what = XtVaCreateManagedWidget("whatLabel",
		xmLabelGadgetClass, input_form,
		XmNlabelString, str,
		XmNleftAttachment,	XmATTACH_FORM,
		XmNtopAttachment, XmATTACH_FORM,
		XmNtopOffset, 4,
		NULL);
	XmStringFree(str);
	gui_motif_menu_fontlist(label_what);

	frdp->what = XtVaCreateManagedWidget("whatText",
		xmTextFieldWidgetClass, input_form,
		XmNtopAttachment, XmATTACH_FORM,
		XmNrightAttachment, XmATTACH_FORM,
		XmNleftAttachment, XmATTACH_FORM,
		NULL);

	if (do_replace)
	{
	    frdp->with = XtVaCreateManagedWidget("withText",
		    xmTextFieldWidgetClass,	input_form,
		    XmNtopAttachment, XmATTACH_WIDGET,
		    XmNtopWidget, frdp->what,
		    XmNtopOffset, 4,
		    XmNleftAttachment, XmATTACH_FORM,
		    XmNrightAttachment, XmATTACH_FORM,
		    XmNbottomAttachment, XmATTACH_FORM,
		    NULL);

	    XtAddCallback(frdp->with, XmNactivateCallback,
		    find_replace_callback, (XtPointer) FRD_R_FINDNEXT);

	    str = XmStringCreateSimple(_("Replace with:"));
	    label_with = XtVaCreateManagedWidget("withLabel",
		    xmLabelGadgetClass, input_form,
		    XmNlabelString, str,
		    XmNleftAttachment, XmATTACH_FORM,
		    XmNtopAttachment, XmATTACH_WIDGET,
		    XmNtopWidget, frdp->what,
		    XmNtopOffset, 4,
		    XmNbottomAttachment, XmATTACH_FORM,
		    NULL);
	    XmStringFree(str);
	    gui_motif_menu_fontlist(label_with);

	    /*
	     * Make the entry activation only change the input focus onto the
	     * with item.
	     */
	    XtAddCallback(frdp->what, XmNactivateCallback,
		    entry_activate_callback, frdp->with);
	    XtAddEventHandler(frdp->with, KeyPressMask, False,
			    (XtEventHandler)find_replace_keypress,
			    (XtPointer) frdp);

	}
	else
	{
	    /*
	     * Make the entry activation do the search.
	     */
	    XtAddCallback(frdp->what, XmNactivateCallback,
		    find_replace_callback, (XtPointer)FRD_FINDNEXT);
	}
	XtAddEventHandler(frdp->what, KeyPressMask, False,
			    (XtEventHandler)find_replace_keypress,
			    (XtPointer)frdp);

	// Get the maximum width between the label widgets and line them up.
	n = 0;
	XtSetArg(args[n], XmNwidth, &width); n++;
	XtGetValues(label_what, args, n);
	widest = width;
	if (do_replace)
	{
	    XtGetValues(label_with, args, n);
	    if (width > widest)
		widest = width;
	}

	XtVaSetValues(frdp->what, XmNleftOffset, widest, NULL);
	if (do_replace)
	    XtVaSetValues(frdp->with, XmNleftOffset, widest, NULL);

    }

    XtManageChild(input_form);

    {
	Widget radio_box;
	Widget w;

	frame = XtVaCreateWidget("directionFrame",
		xmFrameWidgetClass, frdp->dialog,
		XmNtopAttachment, XmATTACH_WIDGET,
		XmNtopWidget, input_form,
		XmNtopOffset, 4,
		XmNbottomAttachment, XmATTACH_FORM,
		XmNbottomOffset, 4,
		XmNrightAttachment, XmATTACH_OPPOSITE_WIDGET,
		XmNrightWidget, input_form,
		NULL);

	str = XmStringCreateSimple(_("Direction"));
	w = XtVaCreateManagedWidget("directionFrameLabel",
		xmLabelGadgetClass, frame,
		XmNlabelString, str,
		XmNchildHorizontalAlignment, XmALIGNMENT_BEGINNING,
		XmNchildType, XmFRAME_TITLE_CHILD,
		NULL);
	XmStringFree(str);
	gui_motif_menu_fontlist(w);

	radio_box = XmCreateRadioBox(frame, "radioBox",
		(ArgList)NULL, 0);

	str = XmStringCreateSimple( _("Up"));
	frdp->up = XtVaCreateManagedWidget("upRadioButton",
		xmToggleButtonGadgetClass, radio_box,
		XmNlabelString, str,
		XmNset, False,
		NULL);
	XmStringFree(str);
	gui_motif_menu_fontlist(frdp->up);

	str = XmStringCreateSimple(_("Down"));
	frdp->down = XtVaCreateManagedWidget("downRadioButton",
		xmToggleButtonGadgetClass, radio_box,
		XmNlabelString, str,
		XmNset, True,
		NULL);
	XmStringFree(str);
	gui_motif_menu_fontlist(frdp->down);

	XtManageChild(radio_box);
	XtManageChild(frame);
    }

    toggle_form = XtVaCreateWidget("toggleForm",
	    xmFormWidgetClass,	frdp->dialog,
	    XmNleftAttachment, XmATTACH_FORM,
	    XmNleftOffset, 4,
	    XmNrightAttachment, XmATTACH_WIDGET,
	    XmNrightWidget, frame,
	    XmNrightOffset, 4,
	    XmNtopAttachment, XmATTACH_WIDGET,
	    XmNtopWidget, input_form,
	    XmNtopOffset, 4,
	    XmNbottomAttachment, XmATTACH_FORM,
	    XmNbottomOffset, 4,
	    NULL);

    str = XmStringCreateSimple(_("Match whole word only"));
    frdp->wword = XtVaCreateManagedWidget("wordToggle",
	    xmToggleButtonGadgetClass, toggle_form,
	    XmNlabelString, str,
	    XmNtopAttachment, XmATTACH_FORM,
	    XmNtopOffset, 4,
	    XmNleftAttachment, XmATTACH_FORM,
	    XmNleftOffset, 4,
	    XmNset, wword,
	    NULL);
    XmStringFree(str);

    str = XmStringCreateSimple(_("Match case"));
    frdp->mcase = XtVaCreateManagedWidget("caseToggle",
	    xmToggleButtonGadgetClass, toggle_form,
	    XmNlabelString, str,
	    XmNleftAttachment, XmATTACH_FORM,
	    XmNleftOffset, 4,
	    XmNtopAttachment, XmATTACH_WIDGET,
	    XmNtopWidget, frdp->wword,
	    XmNtopOffset, 4,
	    XmNset, mcase,
	    NULL);
    XmStringFree(str);
    gui_motif_menu_fontlist(frdp->wword);
    gui_motif_menu_fontlist(frdp->mcase);

    XtManageChild(toggle_form);

    if (entry_text != NULL)
	XmTextFieldSetString(frdp->what, (char *)entry_text);
    vim_free(entry_text);

    gui_motif_synch_fonts();

    manage_centered(frdp->dialog);
    activate_dialog_mnemonics(frdp->dialog);
    XmProcessTraversal(frdp->what, XmTRAVERSE_CURRENT);
}

   void
gui_mch_find_dialog(exarg_T *eap)
{
    if (!gui.in_use)
	return;

    find_replace_dialog_create(eap->arg, FALSE);
}


    void
gui_mch_replace_dialog(exarg_T *eap)
{
    if (!gui.in_use)
	return;

    find_replace_dialog_create(eap->arg, TRUE);
}

/*
 * Synchronize all gui elements, which are dependent upon the
 * main text font used. Those are in esp. the find/replace dialogs.
 * If you don't understand why this should be needed, please try to
 * search for "pi\xea\xb6\xe6" in iso8859-2.
 */
    void
gui_motif_synch_fonts(void)
{
    SharedFindReplace *frdp;
    int		    do_replace;
    XFontStruct	    *font;
    XmFontList	    font_list;

    // FIXME: Unless we find out how to create a XmFontList from a XFontSet,
    // we just give up here on font synchronization.
    font = (XFontStruct *)gui.norm_font;
    if (font == NULL)
	return;

    font_list = gui_motif_create_fontlist(font);

    // OK this loop is a bit tricky...
    for (do_replace = 0; do_replace <= 1; ++do_replace)
    {
	frdp = (do_replace) ? (&repl_widgets) : (&find_widgets);
	if (frdp->dialog)
	{
	    XtVaSetValues(frdp->what, XmNfontList, font_list, NULL);
	    if (do_replace)
		XtVaSetValues(frdp->with, XmNfontList, font_list, NULL);
	}
    }

    XmFontListFree(font_list);
}
