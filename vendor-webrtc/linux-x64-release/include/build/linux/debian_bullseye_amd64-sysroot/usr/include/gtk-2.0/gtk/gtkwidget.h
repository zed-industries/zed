/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_WIDGET_H__
#define __GTK_WIDGET_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkaccelgroup.h>
#include <gtk/gtkobject.h>
#include <gtk/gtkadjustment.h>
#include <gtk/gtkstyle.h>
#include <gtk/gtksettings.h>
#include <atk/atk.h>

G_BEGIN_DECLS

/**
 * GtkWidgetFlags:
 * @GTK_TOPLEVEL: widgets without a real parent, as there are #GtkWindow<!-- -->s and
 *  #GtkMenu<!-- -->s have this flag set throughout their lifetime.
 *  Toplevel widgets always contain their own #GdkWindow.
 * @GTK_NO_WINDOW: Indicative for a widget that does not provide its own #GdkWindow.
 *  Visible action (e.g. drawing) is performed on the parent's #GdkWindow.
 * @GTK_REALIZED: Set by gtk_widget_realize(), unset by gtk_widget_unrealize().
 *  A realized widget has an associated #GdkWindow.
 * @GTK_MAPPED: Set by gtk_widget_map(), unset by gtk_widget_unmap().
 *  Only realized widgets can be mapped. It means that gdk_window_show()
 *  has been called on the widgets window(s).
 * @GTK_VISIBLE: Set by gtk_widget_show(), unset by gtk_widget_hide(). Implies that a
 *  widget will be mapped as soon as its parent is mapped.
 * @GTK_SENSITIVE: Set and unset by gtk_widget_set_sensitive().
 *  The sensitivity of a widget determines whether it will receive
 *  certain events (e.g. button or key presses). One premise for
 *  the widget's sensitivity is to have this flag set.
 * @GTK_PARENT_SENSITIVE: Set and unset by gtk_widget_set_sensitive() operations on the
 *  parents of the widget.
 *  This is the second premise for the widget's sensitivity. Once
 *  it has %GTK_SENSITIVE and %GTK_PARENT_SENSITIVE set, its state is
 *  effectively sensitive. This is expressed (and can be examined) by
 *  the #GTK_WIDGET_IS_SENSITIVE macro.
 * @GTK_CAN_FOCUS: Determines whether a widget is able to handle focus grabs.
 * @GTK_HAS_FOCUS: Set by gtk_widget_grab_focus() for widgets that also
 *  have %GTK_CAN_FOCUS set. The flag will be unset once another widget
 *  grabs the focus.
 * @GTK_CAN_DEFAULT: The widget is allowed to receive the default action via
 *  gtk_widget_grab_default() and will reserve space to draw the default if possible
 * @GTK_HAS_DEFAULT: The widget currently is receiving the default action and
 *  should be drawn appropriately if possible
 * @GTK_HAS_GRAB: Set by gtk_grab_add(), unset by gtk_grab_remove(). It means that the
 *  widget is in the grab_widgets stack, and will be the preferred one for
 *  receiving events other than ones of cosmetic value.
 * @GTK_RC_STYLE: Indicates that the widget's style has been looked up through the rc
 *  mechanism. It does not imply that the widget actually had a style
 *  defined through the rc mechanism.
 * @GTK_COMPOSITE_CHILD: Indicates that the widget is a composite child of its parent; see
 *  gtk_widget_push_composite_child(), gtk_widget_pop_composite_child().
 * @GTK_NO_REPARENT: Unused since before GTK+ 1.2, will be removed in a future version.
 * @GTK_APP_PAINTABLE: Set and unset by gtk_widget_set_app_paintable().
 *  Must be set on widgets whose window the application directly draws on,
 *  in order to keep GTK+ from overwriting the drawn stuff.  See
 *  <xref linkend="app-paintable-widgets"/> for a detailed
 *  description of this flag.
 * @GTK_RECEIVES_DEFAULT: The widget when focused will receive the default action and have
 *  %GTK_HAS_DEFAULT set even if there is a different widget set as default.
 * @GTK_DOUBLE_BUFFERED: Set and unset by gtk_widget_set_double_buffered().
 *  Indicates that exposes done on the widget should be
 *  double-buffered.  See <xref linkend="double-buffering"/> for a
 *  detailed discussion of how double-buffering works in GTK+ and
 *  why you may want to disable it for special cases.
 * @GTK_NO_SHOW_ALL:
 *
 * Tells about certain properties of the widget.
 */
typedef enum
{
  GTK_TOPLEVEL         = 1 << 4,
  GTK_NO_WINDOW        = 1 << 5,
  GTK_REALIZED         = 1 << 6,
  GTK_MAPPED           = 1 << 7,
  GTK_VISIBLE          = 1 << 8,
  GTK_SENSITIVE        = 1 << 9,
  GTK_PARENT_SENSITIVE = 1 << 10,
  GTK_CAN_FOCUS        = 1 << 11,
  GTK_HAS_FOCUS        = 1 << 12,
  GTK_CAN_DEFAULT      = 1 << 13,
  GTK_HAS_DEFAULT      = 1 << 14,
  GTK_HAS_GRAB	       = 1 << 15,
  GTK_RC_STYLE	       = 1 << 16,
  GTK_COMPOSITE_CHILD  = 1 << 17,
#ifndef GTK_DISABLE_DEPRECATED
  GTK_NO_REPARENT      = 1 << 18,
#endif
  GTK_APP_PAINTABLE    = 1 << 19,
  GTK_RECEIVES_DEFAULT = 1 << 20,
  GTK_DOUBLE_BUFFERED  = 1 << 21,
  GTK_NO_SHOW_ALL      = 1 << 22
} GtkWidgetFlags;

/* Kinds of widget-specific help */
typedef enum
{
  GTK_WIDGET_HELP_TOOLTIP,
  GTK_WIDGET_HELP_WHATS_THIS
} GtkWidgetHelpType;

/* Macro for casting a pointer to a GtkWidget or GtkWidgetClass pointer.
 * Macros for testing whether `widget' or `klass' are of type GTK_TYPE_WIDGET.
 */
#define GTK_TYPE_WIDGET			  (gtk_widget_get_type ())
#define GTK_WIDGET(widget)		  (G_TYPE_CHECK_INSTANCE_CAST ((widget), GTK_TYPE_WIDGET, GtkWidget))
#define GTK_WIDGET_CLASS(klass)		  (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_WIDGET, GtkWidgetClass))
#define GTK_IS_WIDGET(widget)		  (G_TYPE_CHECK_INSTANCE_TYPE ((widget), GTK_TYPE_WIDGET))
#define GTK_IS_WIDGET_CLASS(klass)	  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_WIDGET))
#define GTK_WIDGET_GET_CLASS(obj)         (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_WIDGET, GtkWidgetClass))

/* Macros for extracting various fields from GtkWidget and GtkWidgetClass.
 */
#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_TYPE:
 * @wid: a #GtkWidget.
 *
 * Gets the type of a widget.
 *
 * Deprecated: 2.20: Use G_OBJECT_TYPE() instead.
 */
#define GTK_WIDGET_TYPE(wid)		  (GTK_OBJECT_TYPE (wid))
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_STATE:
 * @wid: a #GtkWidget.
 *
 * Returns the current state of the widget, as a #GtkStateType.
 *
 * Deprecated: 2.20: Use gtk_widget_get_state() instead.
 */
#define GTK_WIDGET_STATE(wid)		  (GTK_WIDGET (wid)->state)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_SAVED_STATE:
 * @wid: a #GtkWidget.
 *
 * Returns the saved state of the widget, as a #GtkStateType.
 *
 * The saved state will be restored when a widget gets sensitive
 * again, after it has been made insensitive with gtk_widget_set_state()
 * or gtk_widget_set_sensitive().
 *
 * Deprecated: 2.20: Do not used it.
 */
#define GTK_WIDGET_SAVED_STATE(wid)	  (GTK_WIDGET (wid)->saved_state)
#endif


/* Macros for extracting the widget flags from GtkWidget.
 */
/**
 * GTK_WIDGET_FLAGS:
 * @wid: a #GtkWidget.
 *
 * Returns the widget flags from @wid.
 *
 * Deprecated: 2.20: Use the proper function to test individual states:
 * gtk_widget_get_app_paintable(), gtk_widget_get_can_default(),
 * gtk_widget_get_can_focus(), gtk_widget_get_double_buffered(),
 * gtk_widget_has_default(), gtk_widget_is_drawable(),
 * gtk_widget_has_focus(), gtk_widget_has_grab(), gtk_widget_get_mapped(),
 * gtk_widget_get_has_window(), gtk_widget_has_rc_style(),
 * gtk_widget_get_realized(), gtk_widget_get_receives_default(),
 * gtk_widget_get_sensitive(), gtk_widget_is_sensitive(),
 * gtk_widget_is_toplevel() or gtk_widget_get_visible().
 */
#define GTK_WIDGET_FLAGS(wid)		  (GTK_OBJECT_FLAGS (wid))
/* FIXME: Deprecating GTK_WIDGET_FLAGS requires fixing GTK internals. */

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_TOPLEVEL:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget is a toplevel widget.
 *
 * Deprecated: 2.20: Use gtk_widget_is_toplevel() instead.
 */
#define GTK_WIDGET_TOPLEVEL(wid)	  ((GTK_WIDGET_FLAGS (wid) & GTK_TOPLEVEL) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_NO_WINDOW:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget doesn't have an own #GdkWindow.
 *
 * Deprecated: 2.20: Use gtk_widget_get_has_window() instead.
 */
#define GTK_WIDGET_NO_WINDOW(wid)	  ((GTK_WIDGET_FLAGS (wid) & GTK_NO_WINDOW) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_REALIZED:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget is realized.
 *
 * Deprecated: 2.20: Use gtk_widget_get_realized() instead.
 */
#define GTK_WIDGET_REALIZED(wid)	  ((GTK_WIDGET_FLAGS (wid) & GTK_REALIZED) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_MAPPED:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget is mapped.
 *
 * Deprecated: 2.20: Use gtk_widget_get_mapped() instead.
 */
#define GTK_WIDGET_MAPPED(wid)		  ((GTK_WIDGET_FLAGS (wid) & GTK_MAPPED) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_VISIBLE:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget is visible.
 *
 * Deprecated: 2.20: Use gtk_widget_get_visible() instead.
 */
#define GTK_WIDGET_VISIBLE(wid)		  ((GTK_WIDGET_FLAGS (wid) & GTK_VISIBLE) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_DRAWABLE:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget is mapped and visible.
 *
 * Deprecated: 2.20: Use gtk_widget_is_drawable() instead.
 */
#define GTK_WIDGET_DRAWABLE(wid)	  (GTK_WIDGET_VISIBLE (wid) && GTK_WIDGET_MAPPED (wid))
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_SENSITIVE:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the #GTK_SENSITIVE flag has be set on the widget.
 *
 * Deprecated: 2.20: Use gtk_widget_get_sensitive() instead.
 */
#define GTK_WIDGET_SENSITIVE(wid)	  ((GTK_WIDGET_FLAGS (wid) & GTK_SENSITIVE) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_PARENT_SENSITIVE:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the #GTK_PARENT_SENSITIVE flag has be set on the widget.
 *
 * Deprecated: 2.20: Use gtk_widget_get_sensitive() on the parent widget instead.
 */
#define GTK_WIDGET_PARENT_SENSITIVE(wid)  ((GTK_WIDGET_FLAGS (wid) & GTK_PARENT_SENSITIVE) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_IS_SENSITIVE:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget is effectively sensitive.
 *
 * Deprecated: 2.20: Use gtk_widget_is_sensitive() instead.
 */
#define GTK_WIDGET_IS_SENSITIVE(wid)	  (GTK_WIDGET_SENSITIVE (wid) && \
					   GTK_WIDGET_PARENT_SENSITIVE (wid))
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_CAN_FOCUS:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget is able to handle focus grabs.
 *
 * Deprecated: 2.20: Use gtk_widget_get_can_focus() instead.
 */
#define GTK_WIDGET_CAN_FOCUS(wid)	  ((GTK_WIDGET_FLAGS (wid) & GTK_CAN_FOCUS) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_HAS_FOCUS:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget has grabbed the focus and no other
 * widget has done so more recently.
 *
 * Deprecated: 2.20: Use gtk_widget_has_focus() instead.
 */
#define GTK_WIDGET_HAS_FOCUS(wid)	  ((GTK_WIDGET_FLAGS (wid) & GTK_HAS_FOCUS) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_CAN_DEFAULT:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget is allowed to receive the default action
 * via gtk_widget_grab_default().
 *
 * Deprecated: 2.20: Use gtk_widget_get_can_default() instead.
 */
#define GTK_WIDGET_CAN_DEFAULT(wid)	  ((GTK_WIDGET_FLAGS (wid) & GTK_CAN_DEFAULT) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_HAS_DEFAULT:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget currently is receiving the default action.
 *
 * Deprecated: 2.20: Use gtk_widget_has_default() instead.
 */
#define GTK_WIDGET_HAS_DEFAULT(wid)	  ((GTK_WIDGET_FLAGS (wid) & GTK_HAS_DEFAULT) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_HAS_GRAB:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget is in the grab_widgets stack, and will be
 * the preferred one for receiving events other than ones of cosmetic value.
 *
 * Deprecated: 2.20: Use gtk_widget_has_grab() instead.
 */
#define GTK_WIDGET_HAS_GRAB(wid)	  ((GTK_WIDGET_FLAGS (wid) & GTK_HAS_GRAB) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_RC_STYLE:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget's style has been looked up through the rc
 * mechanism.
 *
 * Deprecated: 2.20: Use gtk_widget_has_rc_style() instead.
 */
#define GTK_WIDGET_RC_STYLE(wid)	  ((GTK_WIDGET_FLAGS (wid) & GTK_RC_STYLE) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_COMPOSITE_CHILD:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget is a composite child of its parent.
 *
 * Deprecated: 2.20: Use the #GtkWidget:composite-child property instead.
 */
#define GTK_WIDGET_COMPOSITE_CHILD(wid)	  ((GTK_WIDGET_FLAGS (wid) & GTK_COMPOSITE_CHILD) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_APP_PAINTABLE:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the #GTK_APP_PAINTABLE flag has been set on the widget.
 *
 * Deprecated: 2.20: Use gtk_widget_get_app_paintable() instead.
 */
#define GTK_WIDGET_APP_PAINTABLE(wid)	  ((GTK_WIDGET_FLAGS (wid) & GTK_APP_PAINTABLE) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_RECEIVES_DEFAULT:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the widget when focused will receive the default action
 * even if there is a different widget set as default.
 *
 * Deprecated: 2.20: Use gtk_widget_get_receives_default() instead.
 */
#define GTK_WIDGET_RECEIVES_DEFAULT(wid)  ((GTK_WIDGET_FLAGS (wid) & GTK_RECEIVES_DEFAULT) != 0)
#endif

#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_WIDGET_DOUBLE_BUFFERED:
 * @wid: a #GtkWidget.
 *
 * Evaluates to %TRUE if the #GTK_DOUBLE_BUFFERED flag has been set on the widget.
 *
 * Deprecated: 2.20: Use gtk_widget_get_double_buffered() instead.
 */
#define GTK_WIDGET_DOUBLE_BUFFERED(wid)	  ((GTK_WIDGET_FLAGS (wid) & GTK_DOUBLE_BUFFERED) != 0)
#endif


/* Macros for setting and clearing widget flags.
 */
/**
 * GTK_WIDGET_SET_FLAGS:
 * @wid: a #GtkWidget.
 * @flag: the flags to set.
 *
 * Turns on certain widget flags.
 *
 * Deprecated: 2.22: Use the proper function instead: gtk_widget_set_app_paintable(),
 *   gtk_widget_set_can_default(), gtk_widget_set_can_focus(),
 *   gtk_widget_set_double_buffered(), gtk_widget_set_has_window(),
 *   gtk_widget_set_mapped(), gtk_widget_set_no_show_all(),
 *   gtk_widget_set_realized(), gtk_widget_set_receives_default(),
 *   gtk_widget_set_sensitive() or gtk_widget_set_visible().
 *
 */
#define GTK_WIDGET_SET_FLAGS(wid,flag)	  G_STMT_START{ (GTK_WIDGET_FLAGS (wid) |= (flag)); }G_STMT_END
/* FIXME: Deprecating GTK_WIDGET_SET_FLAGS requires fixing GTK internals. */

/**
 * GTK_WIDGET_UNSET_FLAGS:
 * @wid: a #GtkWidget.
 * @flag: the flags to unset.
 *
 * Turns off certain widget flags.
 *
 * Deprecated: 2.22: Use the proper function instead. See GTK_WIDGET_SET_FLAGS().
 */
#define GTK_WIDGET_UNSET_FLAGS(wid,flag)  G_STMT_START{ (GTK_WIDGET_FLAGS (wid) &= ~(flag)); }G_STMT_END
/* FIXME: Deprecating GTK_WIDGET_UNSET_FLAGS requires fixing GTK internals. */

#define GTK_TYPE_REQUISITION              (gtk_requisition_get_type ())

/* forward declaration to avoid excessive includes (and concurrent includes)
 */
typedef struct _GtkRequisition	   GtkRequisition;
typedef struct _GtkSelectionData   GtkSelectionData;
typedef struct _GtkWidgetClass	   GtkWidgetClass;
typedef struct _GtkWidgetAuxInfo   GtkWidgetAuxInfo;
typedef struct _GtkWidgetShapeInfo GtkWidgetShapeInfo;
typedef struct _GtkClipboard	   GtkClipboard;
typedef struct _GtkTooltip         GtkTooltip;
typedef struct _GtkWindow          GtkWindow;

/**
 * GtkAllocation:
 * @x: the X position of the widget's area relative to its parents allocation.
 * @y: the Y position of the widget's area relative to its parents allocation.
 * @width: the width of the widget's allocated area.
 * @height: the height of the widget's allocated area.
 *
 * A <structname>GtkAllocation</structname> of a widget represents region which has been allocated to the
 * widget by its parent. It is a subregion of its parents allocation. See
 * <xref linkend="size-allocation"/> for more information.
 */
typedef 	GdkRectangle	   GtkAllocation;

/**
 * GtkCallback:
 * @widget: the widget to operate on
 * @data: user-supplied data
 *
 * The type of the callback functions used for e.g. iterating over
 * the children of a container, see gtk_container_foreach().
 */
typedef void    (*GtkCallback)     (GtkWidget        *widget,
				    gpointer          data);

/**
 * GtkRequisition:
 * @width: the widget's desired width
 * @height: the widget's desired height
 *
 * A <structname>GtkRequisition</structname> represents the desired size of a widget. See
 * <xref linkend="size-requisition"/> for more information.
 */
struct _GtkRequisition
{
  gint width;
  gint height;
};

/* The widget is the base of the tree for displayable objects.
 *  (A displayable object is one which takes up some amount
 *  of screen real estate). It provides a common base and interface
 *  which actual widgets must adhere to.
 */
struct _GtkWidget
{
  /* The object structure needs to be the first
   *  element in the widget structure in order for
   *  the object mechanism to work correctly. This
   *  allows a GtkWidget pointer to be cast to a
   *  GtkObject pointer.
   */
  GtkObject object;
  
  /* 16 bits of internally used private flags.
   * this will be packed into the same 4 byte alignment frame that
   * state and saved_state go. we therefore don't waste any new
   * space on this.
   */
  guint16 GSEAL (private_flags);
  
  /* The state of the widget. There are actually only
   *  5 widget states (defined in "gtkenums.h").
   */
  guint8 GSEAL (state);
  
  /* The saved state of the widget. When a widget's state
   *  is changed to GTK_STATE_INSENSITIVE via
   *  "gtk_widget_set_state" or "gtk_widget_set_sensitive"
   *  the old state is kept around in this field. The state
   *  will be restored once the widget gets sensitive again.
   */
  guint8 GSEAL (saved_state);
  
  /* The widget's name. If the widget does not have a name
   *  (the name is NULL), then its name (as returned by
   *  "gtk_widget_get_name") is its class's name.
   * Among other things, the widget name is used to determine
   *  the style to use for a widget.
   */
  gchar *GSEAL (name);
  
  /*< public >*/

  /* The style for the widget. The style contains the
   *  colors the widget should be drawn in for each state
   *  along with graphics contexts used to draw with and
   *  the font to use for text.
   */
  GtkStyle *GSEAL (style);
  
  /* The widget's desired size.
   */
  GtkRequisition GSEAL (requisition);
  
  /* The widget's allocated size.
   */
  GtkAllocation GSEAL (allocation);
  
  /* The widget's window or its parent window if it does
   *  not have a window. (Which will be indicated by the
   *  GTK_NO_WINDOW flag being set).
   */
  GdkWindow *GSEAL (window);
  
  /* The widget's parent.
   */
  GtkWidget *GSEAL (parent);
};

/**
 * GtkWidgetClass:
 * @parent_class:
 * @activate_signal:
 * @set_scroll_adjustments_signal:
 *
 * <structfield>activate_signal</structfield>
 * The signal to emit when a widget of this class is activated,
 * gtk_widget_activate() handles the emission. Implementation of this
 * signal is optional.
 *
 *
 * <structfield>set_scroll_adjustment_signal</structfield>
 * This signal is emitted  when a widget of this class is added
 * to a scrolling aware parent, gtk_widget_set_scroll_adjustments()
 * handles the emission.
 * Implementation of this signal is optional.
 */
struct _GtkWidgetClass
{
  /* The object class structure needs to be the first
   *  element in the widget class structure in order for
   *  the class mechanism to work correctly. This allows a
   *  GtkWidgetClass pointer to be cast to a GtkObjectClass
   *  pointer.
   */
  GtkObjectClass parent_class;

  /*< public >*/
  
  guint activate_signal;

  guint set_scroll_adjustments_signal;

  /*< private >*/
  
  /* seldomly overidden */
  void (*dispatch_child_properties_changed) (GtkWidget   *widget,
					     guint        n_pspecs,
					     GParamSpec **pspecs);

  /* basics */
  void (* show)		       (GtkWidget        *widget);
  void (* show_all)            (GtkWidget        *widget);
  void (* hide)		       (GtkWidget        *widget);
  void (* hide_all)            (GtkWidget        *widget);
  void (* map)		       (GtkWidget        *widget);
  void (* unmap)	       (GtkWidget        *widget);
  void (* realize)	       (GtkWidget        *widget);
  void (* unrealize)	       (GtkWidget        *widget);
  void (* size_request)	       (GtkWidget        *widget,
				GtkRequisition   *requisition);
  void (* size_allocate)       (GtkWidget        *widget,
				GtkAllocation    *allocation);
  void (* state_changed)       (GtkWidget        *widget,
				GtkStateType   	  previous_state);
  void (* parent_set)	       (GtkWidget        *widget,
				GtkWidget        *previous_parent);
  void (* hierarchy_changed)   (GtkWidget        *widget,
				GtkWidget        *previous_toplevel);
  void (* style_set)	       (GtkWidget        *widget,
				GtkStyle         *previous_style);
  void (* direction_changed)   (GtkWidget        *widget,
				GtkTextDirection  previous_direction);
  void (* grab_notify)         (GtkWidget        *widget,
				gboolean          was_grabbed);
  void (* child_notify)        (GtkWidget	 *widget,
				GParamSpec       *pspec);
  
  /* Mnemonics */
  gboolean (* mnemonic_activate) (GtkWidget    *widget,
				  gboolean      group_cycling);
  
  /* explicit focus */
  void     (* grab_focus)      (GtkWidget        *widget);
  gboolean (* focus)           (GtkWidget        *widget,
                                GtkDirectionType  direction);
  
  /* events */
  gboolean (* event)			(GtkWidget	     *widget,
					 GdkEvent	     *event);
  gboolean (* button_press_event)	(GtkWidget	     *widget,
					 GdkEventButton      *event);
  gboolean (* button_release_event)	(GtkWidget	     *widget,
					 GdkEventButton      *event);
  gboolean (* scroll_event)		(GtkWidget           *widget,
					 GdkEventScroll      *event);
  gboolean (* motion_notify_event)	(GtkWidget	     *widget,
					 GdkEventMotion      *event);
  gboolean (* delete_event)		(GtkWidget	     *widget,
					 GdkEventAny	     *event);
  gboolean (* destroy_event)		(GtkWidget	     *widget,
					 GdkEventAny	     *event);
  gboolean (* expose_event)		(GtkWidget	     *widget,
					 GdkEventExpose      *event);
  gboolean (* key_press_event)		(GtkWidget	     *widget,
					 GdkEventKey	     *event);
  gboolean (* key_release_event)	(GtkWidget	     *widget,
					 GdkEventKey	     *event);
  gboolean (* enter_notify_event)	(GtkWidget	     *widget,
					 GdkEventCrossing    *event);
  gboolean (* leave_notify_event)	(GtkWidget	     *widget,
					 GdkEventCrossing    *event);
  gboolean (* configure_event)		(GtkWidget	     *widget,
					 GdkEventConfigure   *event);
  gboolean (* focus_in_event)		(GtkWidget	     *widget,
					 GdkEventFocus       *event);
  gboolean (* focus_out_event)		(GtkWidget	     *widget,
					 GdkEventFocus       *event);
  gboolean (* map_event)		(GtkWidget	     *widget,
					 GdkEventAny	     *event);
  gboolean (* unmap_event)		(GtkWidget	     *widget,
					 GdkEventAny	     *event);
  gboolean (* property_notify_event)	(GtkWidget	     *widget,
					 GdkEventProperty    *event);
  gboolean (* selection_clear_event)	(GtkWidget	     *widget,
					 GdkEventSelection   *event);
  gboolean (* selection_request_event)	(GtkWidget	     *widget,
					 GdkEventSelection   *event);
  gboolean (* selection_notify_event)	(GtkWidget	     *widget,
					 GdkEventSelection   *event);
  gboolean (* proximity_in_event)	(GtkWidget	     *widget,
					 GdkEventProximity   *event);
  gboolean (* proximity_out_event)	(GtkWidget	     *widget,
					 GdkEventProximity   *event);
  gboolean (* visibility_notify_event)	(GtkWidget	     *widget,
					 GdkEventVisibility  *event);
  gboolean (* client_event)		(GtkWidget	     *widget,
					 GdkEventClient	     *event);
  gboolean (* no_expose_event)		(GtkWidget	     *widget,
					 GdkEventAny	     *event);
  gboolean (* window_state_event)	(GtkWidget	     *widget,
					 GdkEventWindowState *event);
  
  /* selection */
  void (* selection_get)           (GtkWidget          *widget,
				    GtkSelectionData   *selection_data,
				    guint               info,
				    guint               time_);
  void (* selection_received)      (GtkWidget          *widget,
				    GtkSelectionData   *selection_data,
				    guint               time_);

  /* Source side drag signals */
  void (* drag_begin)	           (GtkWidget	       *widget,
				    GdkDragContext     *context);
  void (* drag_end)	           (GtkWidget	       *widget,
				    GdkDragContext     *context);
  void (* drag_data_get)           (GtkWidget          *widget,
				    GdkDragContext     *context,
				    GtkSelectionData   *selection_data,
				    guint               info,
				    guint               time_);
  void (* drag_data_delete)        (GtkWidget	       *widget,
				    GdkDragContext     *context);

  /* Target side drag signals */
  void (* drag_leave)	           (GtkWidget	       *widget,
				    GdkDragContext     *context,
				    guint               time_);
  gboolean (* drag_motion)         (GtkWidget	       *widget,
				    GdkDragContext     *context,
				    gint                x,
				    gint                y,
				    guint               time_);
  gboolean (* drag_drop)           (GtkWidget	       *widget,
				    GdkDragContext     *context,
				    gint                x,
				    gint                y,
				    guint               time_);
  void (* drag_data_received)      (GtkWidget          *widget,
				    GdkDragContext     *context,
				    gint                x,
				    gint                y,
				    GtkSelectionData   *selection_data,
				    guint               info,
				    guint               time_);

  /* Signals used only for keybindings */
  gboolean (* popup_menu)          (GtkWidget          *widget);

  /* If a widget has multiple tooltips/whatsthis, it should show the
   * one for the current focus location, or if that doesn't make
   * sense, should cycle through them showing each tip alongside
   * whatever piece of the widget it applies to.
   */
  gboolean (* show_help)           (GtkWidget          *widget,
                                    GtkWidgetHelpType   help_type);
  
  /* accessibility support 
   */
  AtkObject*   (*get_accessible)     (GtkWidget *widget);

  void         (*screen_changed)     (GtkWidget *widget,
                                      GdkScreen *previous_screen);
  gboolean     (*can_activate_accel) (GtkWidget *widget,
                                      guint      signal_id);

  /* Sent when a grab is broken. */
  gboolean (*grab_broken_event) (GtkWidget	     *widget,
                                 GdkEventGrabBroken  *event);

  void         (* composited_changed) (GtkWidget *widget);

  gboolean     (* query_tooltip)      (GtkWidget  *widget,
				       gint        x,
				       gint        y,
				       gboolean    keyboard_tooltip,
				       GtkTooltip *tooltip);
  /* Signals without a C default handler class slot:
   * gboolean	(*damage_event)	(GtkWidget      *widget,
   *                             GdkEventExpose *event);
   */

  /* Padding for future expansion */
  void (*_gtk_reserved5) (void);
  void (*_gtk_reserved6) (void);
  void (*_gtk_reserved7) (void);
};

struct _GtkWidgetAuxInfo
{
  gint x;
  gint y;
  gint width;
  gint height;
  guint x_set : 1;
  guint y_set : 1;
};

struct _GtkWidgetShapeInfo
{
  gint16     offset_x;
  gint16     offset_y;
  GdkBitmap *shape_mask;
};

GType	   gtk_widget_get_type		  (void) G_GNUC_CONST;
GtkWidget* gtk_widget_new		  (GType		type,
					   const gchar	       *first_property_name,
					   ...);
void	   gtk_widget_destroy		  (GtkWidget	       *widget);
void	   gtk_widget_destroyed		  (GtkWidget	       *widget,
					   GtkWidget	      **widget_pointer);
#ifndef GTK_DISABLE_DEPRECATED
GtkWidget* gtk_widget_ref		  (GtkWidget	       *widget);
void	   gtk_widget_unref		  (GtkWidget	       *widget);
void	   gtk_widget_set		  (GtkWidget	       *widget,
					   const gchar         *first_property_name,
					   ...) G_GNUC_NULL_TERMINATED;
#endif /* GTK_DISABLE_DEPRECATED */
#if !defined(GTK_DISABLE_DEPRECATED) || defined (GTK_COMPILATION)
void       gtk_widget_hide_all            (GtkWidget           *widget);
#endif
void	   gtk_widget_unparent		  (GtkWidget	       *widget);
void	   gtk_widget_show		  (GtkWidget	       *widget);
void       gtk_widget_show_now            (GtkWidget           *widget);
void	   gtk_widget_hide		  (GtkWidget	       *widget);
void	   gtk_widget_show_all		  (GtkWidget	       *widget);
void       gtk_widget_set_no_show_all     (GtkWidget           *widget,
					   gboolean             no_show_all);
gboolean   gtk_widget_get_no_show_all     (GtkWidget           *widget);
void	   gtk_widget_map		  (GtkWidget	       *widget);
void	   gtk_widget_unmap		  (GtkWidget	       *widget);
void	   gtk_widget_realize		  (GtkWidget	       *widget);
void	   gtk_widget_unrealize		  (GtkWidget	       *widget);

/* Queuing draws */
void	   gtk_widget_queue_draw	  (GtkWidget	       *widget);
void	   gtk_widget_queue_draw_area	  (GtkWidget	       *widget,
					   gint                 x,
					   gint                 y,
					   gint                 width,
					   gint                 height);
#ifndef GTK_DISABLE_DEPRECATED
void	   gtk_widget_queue_clear	  (GtkWidget	       *widget);
void	   gtk_widget_queue_clear_area	  (GtkWidget	       *widget,
					   gint                 x,
					   gint                 y,
					   gint                 width,
					   gint                 height);
#endif /* GTK_DISABLE_DEPRECATED */


void	   gtk_widget_queue_resize	  (GtkWidget	       *widget);
void	   gtk_widget_queue_resize_no_redraw (GtkWidget *widget);
#ifndef GTK_DISABLE_DEPRECATED
void	   gtk_widget_draw		  (GtkWidget	       *widget,
					   const GdkRectangle  *area);
#endif /* GTK_DISABLE_DEPRECATED */
void	   gtk_widget_size_request	  (GtkWidget	       *widget,
					   GtkRequisition      *requisition);
void	   gtk_widget_size_allocate	  (GtkWidget	       *widget,
					   GtkAllocation       *allocation);
void       gtk_widget_get_child_requisition (GtkWidget	       *widget,
					     GtkRequisition    *requisition);
void	   gtk_widget_add_accelerator	  (GtkWidget           *widget,
					   const gchar         *accel_signal,
					   GtkAccelGroup       *accel_group,
					   guint                accel_key,
					   GdkModifierType      accel_mods,
					   GtkAccelFlags        accel_flags);
gboolean   gtk_widget_remove_accelerator  (GtkWidget           *widget,
					   GtkAccelGroup       *accel_group,
					   guint                accel_key,
					   GdkModifierType      accel_mods);
void       gtk_widget_set_accel_path      (GtkWidget           *widget,
					   const gchar         *accel_path,
					   GtkAccelGroup       *accel_group);
const gchar* _gtk_widget_get_accel_path   (GtkWidget           *widget,
					   gboolean	       *locked);
GList*     gtk_widget_list_accel_closures (GtkWidget	       *widget);
gboolean   gtk_widget_can_activate_accel  (GtkWidget           *widget,
                                           guint                signal_id);
gboolean   gtk_widget_mnemonic_activate   (GtkWidget           *widget,
					   gboolean             group_cycling);
gboolean   gtk_widget_event		  (GtkWidget	       *widget,
					   GdkEvent	       *event);
gint       gtk_widget_send_expose         (GtkWidget           *widget,
					   GdkEvent            *event);
gboolean   gtk_widget_send_focus_change   (GtkWidget           *widget,
                                           GdkEvent            *event);

gboolean   gtk_widget_activate		     (GtkWidget	       *widget);
gboolean   gtk_widget_set_scroll_adjustments (GtkWidget        *widget,
					      GtkAdjustment    *hadjustment,
					      GtkAdjustment    *vadjustment);
     
void	   gtk_widget_reparent		  (GtkWidget	       *widget,
					   GtkWidget	       *new_parent);
gboolean   gtk_widget_intersect		  (GtkWidget	       *widget,
					   const GdkRectangle  *area,
					   GdkRectangle	       *intersection);
GdkRegion *gtk_widget_region_intersect	  (GtkWidget	       *widget,
					   const GdkRegion     *region);

void	gtk_widget_freeze_child_notify	  (GtkWidget	       *widget);
void	gtk_widget_child_notify		  (GtkWidget	       *widget,
					   const gchar	       *child_property);
void	gtk_widget_thaw_child_notify	  (GtkWidget	       *widget);

void       gtk_widget_set_can_focus       (GtkWidget           *widget,
                                           gboolean             can_focus);
gboolean   gtk_widget_get_can_focus       (GtkWidget           *widget);
gboolean   gtk_widget_has_focus           (GtkWidget           *widget);
gboolean   gtk_widget_is_focus            (GtkWidget           *widget);
void       gtk_widget_grab_focus          (GtkWidget           *widget);

void       gtk_widget_set_can_default     (GtkWidget           *widget,
                                           gboolean             can_default);
gboolean   gtk_widget_get_can_default     (GtkWidget           *widget);
gboolean   gtk_widget_has_default         (GtkWidget           *widget);
void       gtk_widget_grab_default        (GtkWidget           *widget);

void      gtk_widget_set_receives_default (GtkWidget           *widget,
                                           gboolean             receives_default);
gboolean  gtk_widget_get_receives_default (GtkWidget           *widget);

gboolean   gtk_widget_has_grab            (GtkWidget           *widget);

void                  gtk_widget_set_name               (GtkWidget    *widget,
							 const gchar  *name);
const gchar*          gtk_widget_get_name               (GtkWidget    *widget);

void                  gtk_widget_set_state              (GtkWidget    *widget,
							 GtkStateType  state);
GtkStateType          gtk_widget_get_state              (GtkWidget    *widget);

void                  gtk_widget_set_sensitive          (GtkWidget    *widget,
							 gboolean      sensitive);
gboolean              gtk_widget_get_sensitive          (GtkWidget    *widget);
gboolean              gtk_widget_is_sensitive           (GtkWidget    *widget);

void                  gtk_widget_set_visible            (GtkWidget    *widget,
                                                         gboolean      visible);
gboolean              gtk_widget_get_visible            (GtkWidget    *widget);

void                  gtk_widget_set_has_window         (GtkWidget    *widget,
                                                         gboolean      has_window);
gboolean              gtk_widget_get_has_window         (GtkWidget    *widget);

gboolean              gtk_widget_is_toplevel            (GtkWidget    *widget);
gboolean              gtk_widget_is_drawable            (GtkWidget    *widget);
void                  gtk_widget_set_realized           (GtkWidget    *widget,
                                                         gboolean      realized);
gboolean              gtk_widget_get_realized           (GtkWidget    *widget);
void                  gtk_widget_set_mapped             (GtkWidget    *widget,
                                                         gboolean      mapped);
gboolean              gtk_widget_get_mapped             (GtkWidget    *widget);

void                  gtk_widget_set_app_paintable      (GtkWidget    *widget,
							 gboolean      app_paintable);
gboolean              gtk_widget_get_app_paintable      (GtkWidget    *widget);

void                  gtk_widget_set_double_buffered    (GtkWidget    *widget,
							 gboolean      double_buffered);
gboolean              gtk_widget_get_double_buffered    (GtkWidget    *widget);

void                  gtk_widget_set_redraw_on_allocate (GtkWidget    *widget,
							 gboolean      redraw_on_allocate);

void                  gtk_widget_set_parent             (GtkWidget    *widget,
							 GtkWidget    *parent);
GtkWidget           * gtk_widget_get_parent             (GtkWidget    *widget);

void                  gtk_widget_set_parent_window      (GtkWidget    *widget,
							 GdkWindow    *parent_window);
GdkWindow           * gtk_widget_get_parent_window      (GtkWidget    *widget);

void                  gtk_widget_set_child_visible      (GtkWidget    *widget,
							 gboolean      is_visible);
gboolean              gtk_widget_get_child_visible      (GtkWidget    *widget);

void                  gtk_widget_set_window             (GtkWidget    *widget,
                                                         GdkWindow    *window);
GdkWindow           * gtk_widget_get_window             (GtkWidget    *widget);

void                  gtk_widget_get_allocation         (GtkWidget     *widget,
                                                         GtkAllocation *allocation);
void                  gtk_widget_set_allocation         (GtkWidget     *widget,
                                                         const GtkAllocation *allocation);

void                  gtk_widget_get_requisition        (GtkWidget     *widget,
                                                         GtkRequisition *requisition);

gboolean   gtk_widget_child_focus         (GtkWidget           *widget,
                                           GtkDirectionType     direction);
gboolean   gtk_widget_keynav_failed       (GtkWidget           *widget,
                                           GtkDirectionType     direction);
void       gtk_widget_error_bell          (GtkWidget           *widget);

void       gtk_widget_set_size_request    (GtkWidget           *widget,
                                           gint                 width,
                                           gint                 height);
void       gtk_widget_get_size_request    (GtkWidget           *widget,
                                           gint                *width,
                                           gint                *height);
#ifndef GTK_DISABLE_DEPRECATED
void	   gtk_widget_set_uposition	  (GtkWidget	       *widget,
					   gint			x,
					   gint			y);
void	   gtk_widget_set_usize		  (GtkWidget	       *widget,
					   gint			width,
					   gint			height);
#endif

void	   gtk_widget_set_events	  (GtkWidget	       *widget,
					   gint			events);
void       gtk_widget_add_events          (GtkWidget           *widget,
					   gint	                events);
void	   gtk_widget_set_extension_events (GtkWidget		*widget,
					    GdkExtensionMode	mode);

GdkExtensionMode gtk_widget_get_extension_events (GtkWidget	*widget);
GtkWidget*   gtk_widget_get_toplevel	(GtkWidget	*widget);
GtkWidget*   gtk_widget_get_ancestor	(GtkWidget	*widget,
					 GType		 widget_type);
GdkColormap* gtk_widget_get_colormap	(GtkWidget	*widget);
GdkVisual*   gtk_widget_get_visual	(GtkWidget	*widget);

GdkScreen *   gtk_widget_get_screen      (GtkWidget *widget);
gboolean      gtk_widget_has_screen      (GtkWidget *widget);
GdkDisplay *  gtk_widget_get_display     (GtkWidget *widget);
GdkWindow *   gtk_widget_get_root_window (GtkWidget *widget);
GtkSettings*  gtk_widget_get_settings    (GtkWidget *widget);
GtkClipboard *gtk_widget_get_clipboard   (GtkWidget *widget,
					  GdkAtom    selection);
GdkPixmap *   gtk_widget_get_snapshot    (GtkWidget    *widget,
                                          GdkRectangle *clip_rect);

#ifndef GTK_DISABLE_DEPRECATED

/**
 * gtk_widget_set_visual:
 * @widget: a #GtkWidget
 * @visual: a visual
 *
 * This function is deprecated; it does nothing.
 */
#define gtk_widget_set_visual(widget,visual)  ((void) 0)

/**
 * gtk_widget_push_visual:
 * @visual: a visual
 *
 * This function is deprecated; it does nothing.
 */
#define gtk_widget_push_visual(visual)        ((void) 0)

/**
 * gtk_widget_pop_visual:
 *
 * This function is deprecated; it does nothing.
 */
#define gtk_widget_pop_visual()               ((void) 0)

/**
 * gtk_widget_set_default_visual:
 * @visual: a visual
 *
 * This function is deprecated; it does nothing.
 */
#define gtk_widget_set_default_visual(visual) ((void) 0)

#endif /* GTK_DISABLE_DEPRECATED */

/* Accessibility support */
AtkObject*       gtk_widget_get_accessible               (GtkWidget          *widget);

/* The following functions must not be called on an already
 * realized widget. Because it is possible that somebody
 * can call get_colormap() or get_visual() and save the
 * result, these functions are probably only safe to
 * call in a widget's init() function.
 */
void         gtk_widget_set_colormap    (GtkWidget      *widget,
					 GdkColormap    *colormap);

gint	     gtk_widget_get_events	(GtkWidget	*widget);
void	     gtk_widget_get_pointer	(GtkWidget	*widget,
					 gint		*x,
					 gint		*y);

gboolean     gtk_widget_is_ancestor	(GtkWidget	*widget,
					 GtkWidget	*ancestor);

gboolean     gtk_widget_translate_coordinates (GtkWidget  *src_widget,
					       GtkWidget  *dest_widget,
					       gint        src_x,
					       gint        src_y,
					       gint       *dest_x,
					       gint       *dest_y);

/* Hide widget and return TRUE.
 */
gboolean     gtk_widget_hide_on_delete	(GtkWidget	*widget);

/* Widget styles.
 */
void        gtk_widget_style_attach       (GtkWidget            *style);

gboolean    gtk_widget_has_rc_style       (GtkWidget            *widget);
void	    gtk_widget_set_style          (GtkWidget            *widget,
                                           GtkStyle             *style);
void        gtk_widget_ensure_style       (GtkWidget            *widget);
GtkStyle *  gtk_widget_get_style          (GtkWidget            *widget);

void        gtk_widget_modify_style       (GtkWidget            *widget,
					   GtkRcStyle           *style);
GtkRcStyle *gtk_widget_get_modifier_style (GtkWidget            *widget);
void        gtk_widget_modify_fg          (GtkWidget            *widget,
					   GtkStateType          state,
					   const GdkColor       *color);
void        gtk_widget_modify_bg          (GtkWidget            *widget,
					   GtkStateType          state,
					   const GdkColor       *color);
void        gtk_widget_modify_text        (GtkWidget            *widget,
					   GtkStateType          state,
					   const GdkColor       *color);
void        gtk_widget_modify_base        (GtkWidget            *widget,
					   GtkStateType          state,
					   const GdkColor       *color);
void        gtk_widget_modify_cursor      (GtkWidget            *widget,
					   const GdkColor       *primary,
					   const GdkColor       *secondary);
void        gtk_widget_modify_font        (GtkWidget            *widget,
					   PangoFontDescription *font_desc);

#ifndef GTK_DISABLE_DEPRECATED

/**
 * gtk_widget_set_rc_style:
 * @widget: a #GtkWidget.
 *
 * Equivalent to <literal>gtk_widget_set_style (widget, NULL)</literal>.
 *
 * Deprecated: 2.0: Use gtk_widget_set_style() with a %NULL @style argument instead.
 */
#define gtk_widget_set_rc_style(widget)          (gtk_widget_set_style (widget, NULL))

/**
 * gtk_widget_restore_default_style:
 * @widget: a #GtkWidget.
 *
 * Equivalent to <literal>gtk_widget_set_style (widget, NULL)</literal>.
 *
 * Deprecated: 2.0: Use gtk_widget_set_style() with a %NULL @style argument instead.
 */
#define gtk_widget_restore_default_style(widget) (gtk_widget_set_style (widget, NULL))
#endif

PangoContext *gtk_widget_create_pango_context (GtkWidget   *widget);
PangoContext *gtk_widget_get_pango_context    (GtkWidget   *widget);
PangoLayout  *gtk_widget_create_pango_layout  (GtkWidget   *widget,
					       const gchar *text);

GdkPixbuf    *gtk_widget_render_icon          (GtkWidget   *widget,
                                               const gchar *stock_id,
                                               GtkIconSize  size,
                                               const gchar *detail);

/* handle composite names for GTK_COMPOSITE_CHILD widgets,
 * the returned name is newly allocated.
 */
void   gtk_widget_set_composite_name	(GtkWidget	*widget,
					 const gchar   	*name);
gchar* gtk_widget_get_composite_name	(GtkWidget	*widget);
     
/* Descend recursively and set rc-style on all widgets without user styles */
void       gtk_widget_reset_rc_styles   (GtkWidget      *widget);

/* Push/pop pairs, to change default values upon a widget's creation.
 * This will override the values that got set by the
 * gtk_widget_set_default_* () functions.
 */
void	     gtk_widget_push_colormap	     (GdkColormap *cmap);
void	     gtk_widget_push_composite_child (void);
void	     gtk_widget_pop_composite_child  (void);
void	     gtk_widget_pop_colormap	     (void);

/* widget style properties
 */
void gtk_widget_class_install_style_property        (GtkWidgetClass     *klass,
						     GParamSpec         *pspec);
void gtk_widget_class_install_style_property_parser (GtkWidgetClass     *klass,
						     GParamSpec         *pspec,
						     GtkRcPropertyParser parser);
GParamSpec*  gtk_widget_class_find_style_property   (GtkWidgetClass     *klass,
						     const gchar        *property_name);
GParamSpec** gtk_widget_class_list_style_properties (GtkWidgetClass     *klass,
						     guint              *n_properties);
void gtk_widget_style_get_property (GtkWidget	     *widget,
				    const gchar    *property_name,
				    GValue	     *value);
void gtk_widget_style_get_valist   (GtkWidget	     *widget,
				    const gchar    *first_property_name,
				    va_list         var_args);
void gtk_widget_style_get          (GtkWidget	     *widget,
				    const gchar    *first_property_name,
				    ...) G_GNUC_NULL_TERMINATED;


/* Set certain default values to be used at widget creation time.
 */
void	     gtk_widget_set_default_colormap (GdkColormap *colormap);
GtkStyle*    gtk_widget_get_default_style    (void);
#ifndef GDK_MULTIHEAD_SAFE
GdkColormap* gtk_widget_get_default_colormap (void);
GdkVisual*   gtk_widget_get_default_visual   (void);
#endif

/* Functions for setting directionality for widgets
 */

void             gtk_widget_set_direction         (GtkWidget        *widget,
						   GtkTextDirection  dir);
GtkTextDirection gtk_widget_get_direction         (GtkWidget        *widget);

void             gtk_widget_set_default_direction (GtkTextDirection  dir);
GtkTextDirection gtk_widget_get_default_direction (void);

/* Compositing manager functionality */
gboolean gtk_widget_is_composited (GtkWidget *widget);

/* Counterpart to gdk_window_shape_combine_mask.
 */
void	     gtk_widget_shape_combine_mask (GtkWidget *widget,
					    GdkBitmap *shape_mask,
					    gint       offset_x,
					    gint       offset_y);
void	     gtk_widget_input_shape_combine_mask (GtkWidget *widget,
						  GdkBitmap *shape_mask,
						  gint       offset_x,
						  gint       offset_y);

#if !defined(GTK_DISABLE_DEPRECATED) || defined (GTK_COMPILATION)
/* internal function */
void	     gtk_widget_reset_shapes	   (GtkWidget *widget);
#endif

/* Compute a widget's path in the form "GtkWindow.MyLabel", and
 * return newly alocated strings.
 */
void	     gtk_widget_path		   (GtkWidget *widget,
					    guint     *path_length,
					    gchar    **path,
					    gchar    **path_reversed);
void	     gtk_widget_class_path	   (GtkWidget *widget,
					    guint     *path_length,
					    gchar    **path,
					    gchar    **path_reversed);

GList* gtk_widget_list_mnemonic_labels  (GtkWidget *widget);
void   gtk_widget_add_mnemonic_label    (GtkWidget *widget,
					 GtkWidget *label);
void   gtk_widget_remove_mnemonic_label (GtkWidget *widget,
					 GtkWidget *label);

void                  gtk_widget_set_tooltip_window    (GtkWidget   *widget,
                                                        GtkWindow   *custom_window);
GtkWindow *gtk_widget_get_tooltip_window    (GtkWidget   *widget);
void       gtk_widget_trigger_tooltip_query (GtkWidget   *widget);
void       gtk_widget_set_tooltip_text      (GtkWidget   *widget,
                                             const gchar *text);
gchar *    gtk_widget_get_tooltip_text      (GtkWidget   *widget);
void       gtk_widget_set_tooltip_markup    (GtkWidget   *widget,
                                             const gchar *markup);
gchar *    gtk_widget_get_tooltip_markup    (GtkWidget   *widget);
void       gtk_widget_set_has_tooltip       (GtkWidget   *widget,
					     gboolean     has_tooltip);
gboolean   gtk_widget_get_has_tooltip       (GtkWidget   *widget);

GType           gtk_requisition_get_type (void) G_GNUC_CONST;
GtkRequisition *gtk_requisition_copy     (const GtkRequisition *requisition);
void            gtk_requisition_free     (GtkRequisition       *requisition);

#if	defined (GTK_TRACE_OBJECTS) && defined (__GNUC__)
#  define gtk_widget_ref g_object_ref
#  define gtk_widget_unref g_object_unref
#endif	/* GTK_TRACE_OBJECTS && __GNUC__ */

void              _gtk_widget_set_has_default             (GtkWidget    *widget,
                                                           gboolean      has_default);
void              _gtk_widget_set_has_grab                (GtkWidget    *widget,
                                                           gboolean      has_grab);
void              _gtk_widget_set_is_toplevel             (GtkWidget    *widget,
                                                           gboolean      is_toplevel);

void              _gtk_widget_grab_notify                 (GtkWidget    *widget,
						           gboolean	was_grabbed);

GtkWidgetAuxInfo *_gtk_widget_get_aux_info                (GtkWidget    *widget,
							   gboolean      create);
void              _gtk_widget_propagate_hierarchy_changed (GtkWidget    *widget,
							   GtkWidget    *previous_toplevel);
void              _gtk_widget_propagate_screen_changed    (GtkWidget    *widget,
							   GdkScreen    *previous_screen);
void		  _gtk_widget_propagate_composited_changed (GtkWidget    *widget);

void	   _gtk_widget_set_pointer_window  (GtkWidget      *widget,
					    GdkWindow      *pointer_window);
GdkWindow *_gtk_widget_get_pointer_window  (GtkWidget      *widget);
gboolean   _gtk_widget_is_pointer_widget   (GtkWidget      *widget);
void       _gtk_widget_synthesize_crossing (GtkWidget      *from,
					    GtkWidget      *to,
					    GdkCrossingMode mode);

GdkColormap* _gtk_widget_peek_colormap (void);

void         _gtk_widget_buildable_finish_accelerator (GtkWidget *widget,
						       GtkWidget *toplevel,
						       gpointer   user_data);

G_END_DECLS

#endif /* __GTK_WIDGET_H__ */
