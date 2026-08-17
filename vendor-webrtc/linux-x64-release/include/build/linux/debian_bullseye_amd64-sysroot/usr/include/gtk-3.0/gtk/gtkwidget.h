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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_WIDGET_H__
#define __GTK_WIDGET_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkaccelgroup.h>
#include <gtk/gtkborder.h>
#include <gtk/gtktypes.h>
#include <atk/atk.h>

G_BEGIN_DECLS

/* Kinds of widget-specific help */
/**
 * GtkWidgetHelpType:
 * @GTK_WIDGET_HELP_TOOLTIP: Tooltip.
 * @GTK_WIDGET_HELP_WHATS_THIS: What’s this.
 *
 * Kinds of widget-specific help. Used by the ::show-help signal.
 */
typedef enum
{
  GTK_WIDGET_HELP_TOOLTIP,
  GTK_WIDGET_HELP_WHATS_THIS
} GtkWidgetHelpType;

/* Macro for casting a pointer to a GtkWidget or GtkWidgetClass pointer.
 * Macros for testing whether widget or klass are of type GTK_TYPE_WIDGET.
 */
#define GTK_TYPE_WIDGET			  (gtk_widget_get_type ())
#define GTK_WIDGET(widget)		  (G_TYPE_CHECK_INSTANCE_CAST ((widget), GTK_TYPE_WIDGET, GtkWidget))
#define GTK_WIDGET_CLASS(klass)		  (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_WIDGET, GtkWidgetClass))
#define GTK_IS_WIDGET(widget)		  (G_TYPE_CHECK_INSTANCE_TYPE ((widget), GTK_TYPE_WIDGET))
#define GTK_IS_WIDGET_CLASS(klass)	  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_WIDGET))
#define GTK_WIDGET_GET_CLASS(obj)         (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_WIDGET, GtkWidgetClass))

#define GTK_TYPE_REQUISITION              (gtk_requisition_get_type ())

typedef struct _GtkWidgetPrivate       GtkWidgetPrivate;
typedef struct _GtkWidgetClass	       GtkWidgetClass;
typedef struct _GtkWidgetClassPrivate  GtkWidgetClassPrivate;

/**
 * GtkAllocation:
 * @x: the X position of the widget’s area relative to its parents allocation.
 * @y: the Y position of the widget’s area relative to its parents allocation.
 * @width: the width of the widget’s allocated area.
 * @height: the height of the widget’s allocated area.
 *
 * A #GtkAllocation-struct of a widget represents region
 * which has been allocated to the widget by its parent. It is a subregion
 * of its parents allocation. See
 * [GtkWidget’s geometry management section][geometry-management] for
 * more information.
 */
typedef 	GdkRectangle	   GtkAllocation;

/**
 * GtkCallback:
 * @widget: the widget to operate on
 * @data: (closure): user-supplied data
 *
 * The type of the callback functions used for e.g. iterating over
 * the children of a container, see gtk_container_foreach().
 */
typedef void    (*GtkCallback)     (GtkWidget        *widget,
				    gpointer          data);

/**
 * GtkTickCallback:
 * @widget: the widget
 * @frame_clock: the frame clock for the widget (same as calling gtk_widget_get_frame_clock())
 * @user_data: user data passed to gtk_widget_add_tick_callback().
 *
 * Callback type for adding a function to update animations. See gtk_widget_add_tick_callback().
 *
 * Returns: %G_SOURCE_CONTINUE if the tick callback should continue to be called,
 *  %G_SOURCE_REMOVE if the tick callback should be removed.
 *
 * Since: 3.8
 */
typedef gboolean (*GtkTickCallback) (GtkWidget     *widget,
                                     GdkFrameClock *frame_clock,
                                     gpointer       user_data);

/**
 * GtkRequisition:
 * @width: the widget’s desired width
 * @height: the widget’s desired height
 *
 * A #GtkRequisition-struct represents the desired size of a widget. See
 * [GtkWidget’s geometry management section][geometry-management] for
 * more information.
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
  GInitiallyUnowned parent_instance;

  /*< private >*/

  GtkWidgetPrivate *priv;
};

/**
 * GtkWidgetClass:
 * @parent_class: The object class structure needs to be the first
 *   element in the widget class structure in order for the class mechanism
 *   to work correctly. This allows a GtkWidgetClass pointer to be cast to
 *   a GObjectClass pointer.
 * @activate_signal: The signal to emit when a widget of this class is
 *   activated, gtk_widget_activate() handles the emission.
 *   Implementation of this signal is optional.
 * @dispatch_child_properties_changed: Seldomly overidden.
 * @destroy: Signals that all holders of a reference to the widget
 *   should release the reference that they hold.
 * @show: Signal emitted when widget is shown
 * @show_all: Recursively shows a widget, and any child widgets (if the widget is
 * a container).
 * @hide: Signal emitted when widget is hidden.
 * @map: Signal emitted when widget is going to be mapped, that is
 *   when the widget is visible (which is controlled with
 *   gtk_widget_set_visible()) and all its parents up to the toplevel
 *   widget are also visible.
 * @unmap: Signal emitted when widget is going to be unmapped, which
 *   means that either it or any of its parents up to the toplevel
 *   widget have been set as hidden.
 * @realize: Signal emitted when widget is associated with a
 *   #GdkWindow, which means that gtk_widget_realize() has been called or
 *   the widget has been mapped (that is, it is going to be drawn).
 * @unrealize: Signal emitted when the GdkWindow associated with
 *   widget is destroyed, which means that gtk_widget_unrealize() has
 *   been called or the widget has been unmapped (that is, it is going
 *   to be hidden).
 * @size_allocate: Signal emitted to get the widget allocation.
 * @state_changed: Signal emitted when the widget state
 *   changes. Deprecated: 3.0
 * @state_flags_changed: Signal emitted when the widget state changes,
 *   see gtk_widget_get_state_flags().
 * @parent_set: Signal emitted when a new parent has been set on a
 *   widget.
 * @hierarchy_changed: Signal emitted when the anchored state of a
 *   widget changes.
 * @style_set: Signal emitted when a new style has been set on a
 * widget. Deprecated: 3.0
 * @direction_changed: Signal emitted when the text direction of a
 *   widget changes.
 * @grab_notify: Signal emitted when a widget becomes shadowed by a
 *   GTK+ grab (not a pointer or keyboard grab) on another widget, or
 *   when it becomes unshadowed due to a grab being removed.
 * @child_notify: Signal emitted for each child property that has
 *   changed on an object.
 * @draw: Signal emitted when a widget is supposed to render itself.
 * @get_request_mode: This allows a widget to tell its parent container whether
 *   it prefers to be allocated in %GTK_SIZE_REQUEST_HEIGHT_FOR_WIDTH or
 *   %GTK_SIZE_REQUEST_WIDTH_FOR_HEIGHT mode.
 *   %GTK_SIZE_REQUEST_HEIGHT_FOR_WIDTH means the widget prefers to have
 *   #GtkWidgetClass.get_preferred_width() called and then
 *   #GtkWidgetClass.get_preferred_height_for_width().
 *   %GTK_SIZE_REQUEST_CONSTANT_SIZE disables any height-for-width or
 *   width-for-height geometry management for a said widget and is the
 *   default return.
 *   It’s important to note (as described below) that any widget
 *   which trades height-for-width or width-for-height must respond properly 
 *   to both of the virtual methods #GtkWidgetClass.get_preferred_height_for_width()
 *   and #GtkWidgetClass.get_preferred_width_for_height() since it might be 
 *   queried in either #GtkSizeRequestMode by its parent container.
 * @get_preferred_height: This is called by containers to obtain the minimum
 *   and natural height of a widget. A widget that does not actually trade
 *   any height for width or width for height only has to implement these
 *   two virtual methods (#GtkWidgetClass.get_preferred_width() and
 *   #GtkWidgetClass.get_preferred_height()).
 * @get_preferred_width_for_height: This is analogous to
 *   #GtkWidgetClass.get_preferred_height_for_width() except that it
 *   operates in the oposite orientation. It’s rare that a widget actually
 *   does %GTK_SIZE_REQUEST_WIDTH_FOR_HEIGHT requests but this can happen
 *   when, for example, a widget or container gets additional columns to
 *   compensate for a smaller allocated height.
 * @get_preferred_width: This is called by containers to obtain the minimum
 *   and natural width of a widget. A widget will never be allocated a width
 *   less than its minimum and will only ever be allocated a width greater
 *   than the natural width once all of the said widget’s siblings have
 *   received their natural widths.
 *   Furthermore, a widget will only ever be allocated a width greater than
 *   its natural width if it was configured to receive extra expand space
 *   from its parent container.
 * @get_preferred_height_for_width: This is similar to
 *   #GtkWidgetClass.get_preferred_height() except that it is passed a
 *   contextual width to request height for. By implementing this virtual
 *   method it is possible for a #GtkLabel to tell its parent how much height
 *   would be required if the label were to be allocated a said width.
 * @mnemonic_activate: Activates the @widget if @group_cycling is
 *   %FALSE, and just grabs the focus if @group_cycling is %TRUE.
 * @grab_focus: Causes @widget to have the keyboard focus for the
 *   #GtkWindow it’s inside.
 * @focus:
 * @move_focus: Signal emitted when a change of focus is requested
 * @keynav_failed: Signal emitted if keyboard navigation fails.
 * @event: The GTK+ main loop will emit three signals for each GDK
 *   event delivered to a widget: one generic ::event signal, another,
 *   more specific, signal that matches the type of event delivered
 *   (e.g. "key-press-event") and finally a generic "event-after"
 *   signal.
 * @button_press_event: Signal will be emitted when a button
 *   (typically from a mouse) is pressed.
 * @button_release_event: Signal will be emitted when a button
 *   (typically from a mouse) is released.
 * @scroll_event: Signal emitted when a button in the 4 to 7 range is
 *   pressed.
 * @motion_notify_event: Signal emitted when the pointer moves over
 *   the widget’s #GdkWindow.
 * @delete_event: Signal emitted if a user requests that a toplevel
 *   window is closed.
 * @destroy_event: Signal is emitted when a #GdkWindow is destroyed.
 * @key_press_event: Signal emitted when a key is pressed.
 * @key_release_event: Signal is emitted when a key is released.
 * @enter_notify_event: Signal event will be emitted when the pointer
 *   enters the widget’s window.
 * @leave_notify_event: Will be emitted when the pointer leaves the
 *   widget’s window.
 * @configure_event: Signal will be emitted when the size, position or
 *   stacking of the widget’s window has changed.
 * @focus_in_event: Signal emitted when the keyboard focus enters the
 * widget’s window.
 * @focus_out_event: Signal emitted when the keyboard focus leaves the
 * widget’s window.
 * @map_event: Signal emitted when the widget’s window is mapped.
 * @unmap_event: Signal will be emitted when the widget’s window is
 *   unmapped.
 * @property_notify_event: Signal will be emitted when a property on
 *   the widget’s window has been changed or deleted.
 * @selection_clear_event: Signal will be emitted when the the
 *   widget’s window has lost ownership of a selection.
 * @selection_request_event: Signal will be emitted when another
 *   client requests ownership of the selection owned by the widget's
 *   window.
 * @selection_notify_event:
 * @proximity_in_event:
 * @proximity_out_event:
 * @visibility_notify_event: Signal emitted when the widget’s window is
 *   obscured or unobscured.
 * @window_state_event: Signal emitted when the state of the toplevel
 *   window associated to the widget changes.
 * @damage_event: Signal emitted when a redirected window belonging to
 *   widget gets drawn into.
 * @grab_broken_event: Signal emitted when a pointer or keyboard grab
 *   on a window belonging to widget gets broken.
 * @selection_get:
 * @selection_received:
 * @drag_begin: Signal emitted on the drag source when a drag is
 *   started.
 * @drag_end: Signal emitted on the drag source when a drag is
 *   finished.
 * @drag_data_get: Signal emitted on the drag source when the drop
 *   site requests the data which is dragged.
 * @drag_data_delete: Signal emitted on the drag source when a drag
 *   with the action %GDK_ACTION_MOVE is successfully completed.
 * @drag_leave: Signal emitted on the drop site when the cursor leaves
 *   the widget.
 * @drag_motion: signal emitted on the drop site when the user moves
 *   the cursor over the widget during a drag.
 * @drag_drop: Signal emitted on the drop site when the user drops the
 *   data onto the widget.
 * @drag_data_received: Signal emitted on the drop site when the
 *   dragged data has been received.
 * @drag_failed: Signal emitted on the drag source when a drag has
 *   failed.
 * @popup_menu: Signal emitted whenever a widget should pop up a
 *   context menu.
 * @show_help:
 * @get_accessible: Returns the accessible object that describes the
 *   widget to an assistive technology.
 * @screen_changed: Signal emitted when the screen of a widget has
 *   changed.
 * @can_activate_accel: Signal allows applications and derived widgets
 *   to override the default GtkWidget handling for determining whether
 *   an accelerator can be activated.
 * @composited_changed: Signal emitted when the composited status of
 *   widgets screen changes. See gdk_screen_is_composited().
 * @query_tooltip: Signal emitted when “has-tooltip” is %TRUE and the
 *   hover timeout has expired with the cursor hovering “above”
 *   widget; or emitted when widget got focus in keyboard mode.
 * @compute_expand: Computes whether a container should give this
 *   widget extra space when possible.
 * @adjust_size_request: Convert an initial size request from a widget's
 *   #GtkSizeRequestMode virtual method implementations into a size request to
 *   be used by parent containers in laying out the widget.
 *   adjust_size_request adjusts from a child widget's
 *   original request to what a parent container should
 *   use for layout. The @for_size argument will be -1 if the request should
 *   not be for a particular size in the opposing orientation, i.e. if the
 *   request is not height-for-width or width-for-height. If @for_size is
 *   greater than -1, it is the proposed allocation in the opposing
 *   orientation that we need the request for. Implementations of
 *   adjust_size_request should chain up to the default implementation,
 *   which applies #GtkWidget’s margin properties and imposes any values
 *   from gtk_widget_set_size_request(). Chaining up should be last,
 *   after your subclass adjusts the request, so
 *   #GtkWidget can apply constraints and add the margin properly.
 * @adjust_size_allocation: Convert an initial size allocation assigned
 *   by a #GtkContainer using gtk_widget_size_allocate(), into an actual
 *   size allocation to be used by the widget. adjust_size_allocation
 *   adjusts to a child widget’s actual allocation
 *   from what a parent container computed for the
 *   child. The adjusted allocation must be entirely within the original
 *   allocation. In any custom implementation, chain up to the default
 *   #GtkWidget implementation of this method, which applies the margin
 *   and alignment properties of #GtkWidget. Chain up
 *   before performing your own adjustments so your
 *   own adjustments remove more allocation after the #GtkWidget base
 *   class has already removed margin and alignment. The natural size
 *   passed in should be adjusted in the same way as the allocated size,
 *   which allows adjustments to perform alignments or other changes
 *   based on natural size.
 * @style_updated: Signal emitted when the GtkStyleContext of a widget
 *   is changed.
 * @touch_event: Signal emitted when a touch event happens
 * @get_preferred_height_and_baseline_for_width:
 * @adjust_baseline_request:
 * @adjust_baseline_allocation:
 * @queue_draw_region: Invalidates the area of widget defined by
 *   region by calling gdk_window_invalidate_region() on the widget's
 *   window and all its child windows.
 */
struct _GtkWidgetClass
{
  GInitiallyUnownedClass parent_class;

  /*< public >*/

  guint activate_signal;

  /* seldomly overidden */
  void (*dispatch_child_properties_changed) (GtkWidget   *widget,
					     guint        n_pspecs,
					     GParamSpec **pspecs);

  /* basics */
  void (* destroy)             (GtkWidget        *widget);
  void (* show)		       (GtkWidget        *widget);
  void (* show_all)            (GtkWidget        *widget);
  void (* hide)		       (GtkWidget        *widget);
  void (* map)		       (GtkWidget        *widget);
  void (* unmap)	       (GtkWidget        *widget);
  void (* realize)	       (GtkWidget        *widget);
  void (* unrealize)	       (GtkWidget        *widget);
  void (* size_allocate)       (GtkWidget        *widget,
				GtkAllocation    *allocation);
  void (* state_changed)       (GtkWidget        *widget,
				GtkStateType   	  previous_state);
  void (* state_flags_changed) (GtkWidget        *widget,
				GtkStateFlags  	  previous_state_flags);
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
				GParamSpec       *child_property);
  gboolean (* draw)	       (GtkWidget	 *widget,
                                cairo_t          *cr);

  /* size requests */
  GtkSizeRequestMode (* get_request_mode)               (GtkWidget      *widget);

  void               (* get_preferred_height)           (GtkWidget       *widget,
                                                         gint            *minimum_height,
                                                         gint            *natural_height);
  void               (* get_preferred_width_for_height) (GtkWidget       *widget,
                                                         gint             height,
                                                         gint            *minimum_width,
                                                         gint            *natural_width);
  void               (* get_preferred_width)            (GtkWidget       *widget,
                                                         gint            *minimum_width,
                                                         gint            *natural_width);
  void               (* get_preferred_height_for_width) (GtkWidget       *widget,
                                                         gint             width,
                                                         gint            *minimum_height,
                                                         gint            *natural_height);

  /* Mnemonics */
  gboolean (* mnemonic_activate)        (GtkWidget           *widget,
                                         gboolean             group_cycling);

  /* explicit focus */
  void     (* grab_focus)               (GtkWidget           *widget);
  gboolean (* focus)                    (GtkWidget           *widget,
                                         GtkDirectionType     direction);

  /* keyboard navigation */
  void     (* move_focus)               (GtkWidget           *widget,
                                         GtkDirectionType     direction);
  gboolean (* keynav_failed)            (GtkWidget           *widget,
                                         GtkDirectionType     direction);

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
  gboolean (* window_state_event)	(GtkWidget	     *widget,
					 GdkEventWindowState *event);
  gboolean (* damage_event)             (GtkWidget           *widget,
                                         GdkEventExpose      *event);
  gboolean (* grab_broken_event)        (GtkWidget           *widget,
                                         GdkEventGrabBroken  *event);

  /* selection */
  void     (* selection_get)       (GtkWidget          *widget,
				    GtkSelectionData   *selection_data,
				    guint               info,
				    guint               time_);
  void     (* selection_received)  (GtkWidget          *widget,
				    GtkSelectionData   *selection_data,
				    guint               time_);

  /* Source side drag signals */
  void     (* drag_begin)          (GtkWidget         *widget,
				    GdkDragContext     *context);
  void     (* drag_end)	           (GtkWidget	       *widget,
				    GdkDragContext     *context);
  void     (* drag_data_get)       (GtkWidget          *widget,
				    GdkDragContext     *context,
				    GtkSelectionData   *selection_data,
				    guint               info,
				    guint               time_);
  void     (* drag_data_delete)    (GtkWidget          *widget,
				    GdkDragContext     *context);

  /* Target side drag signals */
  void     (* drag_leave)          (GtkWidget          *widget,
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
  void     (* drag_data_received)  (GtkWidget          *widget,
				    GdkDragContext     *context,
				    gint                x,
				    gint                y,
				    GtkSelectionData   *selection_data,
				    guint               info,
				    guint               time_);
  gboolean (* drag_failed)         (GtkWidget          *widget,
                                    GdkDragContext     *context,
                                    GtkDragResult       result);

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
  AtkObject *  (* get_accessible)     (GtkWidget *widget);

  void         (* screen_changed)     (GtkWidget *widget,
                                       GdkScreen *previous_screen);
  gboolean     (* can_activate_accel) (GtkWidget *widget,
                                       guint      signal_id);


  void         (* composited_changed) (GtkWidget *widget);

  gboolean     (* query_tooltip)      (GtkWidget  *widget,
				       gint        x,
				       gint        y,
				       gboolean    keyboard_tooltip,
				       GtkTooltip *tooltip);

  void         (* compute_expand)     (GtkWidget  *widget,
                                       gboolean   *hexpand_p,
                                       gboolean   *vexpand_p);

  void         (* adjust_size_request)    (GtkWidget         *widget,
                                           GtkOrientation     orientation,
                                           gint              *minimum_size,
                                           gint              *natural_size);
  void         (* adjust_size_allocation) (GtkWidget         *widget,
                                           GtkOrientation     orientation,
                                           gint              *minimum_size,
                                           gint              *natural_size,
                                           gint              *allocated_pos,
                                           gint              *allocated_size);

  void         (* style_updated)          (GtkWidget *widget);

  gboolean     (* touch_event)            (GtkWidget     *widget,
                                           GdkEventTouch *event);

  void         (* get_preferred_height_and_baseline_for_width)  (GtkWidget     *widget,
								 gint           width,
								 gint          *minimum_height,
								 gint          *natural_height,
								 gint          *minimum_baseline,
								 gint          *natural_baseline);
  void         (* adjust_baseline_request)(GtkWidget         *widget,
                                           gint              *minimum_baseline,
                                           gint              *natural_baseline);
  void         (* adjust_baseline_allocation) (GtkWidget         *widget,
					       gint              *baseline);
  void         (*queue_draw_region)           (GtkWidget         *widget,
					       const cairo_region_t *region);

  /*< private >*/

  GtkWidgetClassPrivate *priv;

  /* Padding for future expansion */
  void (*_gtk_reserved6) (void);
  void (*_gtk_reserved7) (void);
};


GDK_AVAILABLE_IN_ALL
GType	   gtk_widget_get_type		  (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_widget_new		  (GType		type,
					   const gchar	       *first_property_name,
					   ...);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_destroy		  (GtkWidget	       *widget);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_destroyed		  (GtkWidget	       *widget,
					   GtkWidget	      **widget_pointer);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_unparent		  (GtkWidget	       *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_show                (GtkWidget           *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_hide                (GtkWidget           *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_show_now            (GtkWidget           *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_show_all            (GtkWidget           *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_set_no_show_all     (GtkWidget           *widget,
                                           gboolean             no_show_all);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_get_no_show_all     (GtkWidget           *widget);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_map		  (GtkWidget	       *widget);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_unmap		  (GtkWidget	       *widget);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_realize		  (GtkWidget	       *widget);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_unrealize		  (GtkWidget	       *widget);

GDK_AVAILABLE_IN_ALL
void       gtk_widget_draw                (GtkWidget           *widget,
                                           cairo_t             *cr);
/* Queuing draws */
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_queue_draw	  (GtkWidget	       *widget);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_queue_draw_area	  (GtkWidget	       *widget,
					   gint                 x,
					   gint                 y,
					   gint                 width,
					   gint                 height);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_queue_draw_region   (GtkWidget	       *widget,
                                           const cairo_region_t*region);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_queue_resize	  (GtkWidget	       *widget);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_queue_resize_no_redraw (GtkWidget *widget);
GDK_AVAILABLE_IN_3_20
void       gtk_widget_queue_allocate      (GtkWidget           *widget);
GDK_AVAILABLE_IN_3_8
GdkFrameClock* gtk_widget_get_frame_clock (GtkWidget           *widget);

GDK_DEPRECATED_IN_3_0_FOR(gtk_widget_get_preferred_size)
void       gtk_widget_size_request        (GtkWidget           *widget,
                                           GtkRequisition      *requisition);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_size_allocate	  (GtkWidget	       *widget,
					   GtkAllocation       *allocation);
GDK_AVAILABLE_IN_3_10
void	   gtk_widget_size_allocate_with_baseline	  (GtkWidget	       *widget,
							   GtkAllocation       *allocation,
							   gint                 baseline);

GDK_AVAILABLE_IN_ALL
GtkSizeRequestMode  gtk_widget_get_request_mode               (GtkWidget      *widget);
GDK_AVAILABLE_IN_ALL
void                gtk_widget_get_preferred_width            (GtkWidget      *widget,
                                                               gint           *minimum_width,
                                                               gint           *natural_width);
GDK_AVAILABLE_IN_ALL
void                gtk_widget_get_preferred_height_for_width (GtkWidget      *widget,
                                                               gint            width,
                                                               gint           *minimum_height,
                                                               gint           *natural_height);
GDK_AVAILABLE_IN_ALL
void                gtk_widget_get_preferred_height           (GtkWidget      *widget,
                                                               gint           *minimum_height,
                                                               gint           *natural_height);
GDK_AVAILABLE_IN_ALL
void                gtk_widget_get_preferred_width_for_height (GtkWidget      *widget,
                                                               gint            height,
                                                               gint           *minimum_width,
                                                               gint           *natural_width);
GDK_AVAILABLE_IN_3_10
void   gtk_widget_get_preferred_height_and_baseline_for_width (GtkWidget     *widget,
							       gint           width,
							       gint          *minimum_height,
							       gint          *natural_height,
							       gint          *minimum_baseline,
							       gint          *natural_baseline);
GDK_AVAILABLE_IN_ALL
void                gtk_widget_get_preferred_size             (GtkWidget      *widget,
                                                               GtkRequisition *minimum_size,
                                                               GtkRequisition *natural_size);

GDK_DEPRECATED_IN_3_0_FOR(gtk_widget_get_preferred_size)
void       gtk_widget_get_child_requisition (GtkWidget         *widget,
                                             GtkRequisition    *requisition);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_add_accelerator	  (GtkWidget           *widget,
					   const gchar         *accel_signal,
					   GtkAccelGroup       *accel_group,
					   guint                accel_key,
					   GdkModifierType      accel_mods,
					   GtkAccelFlags        accel_flags);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_remove_accelerator  (GtkWidget           *widget,
					   GtkAccelGroup       *accel_group,
					   guint                accel_key,
					   GdkModifierType      accel_mods);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_set_accel_path      (GtkWidget           *widget,
					   const gchar         *accel_path,
					   GtkAccelGroup       *accel_group);
GDK_AVAILABLE_IN_ALL
GList*     gtk_widget_list_accel_closures (GtkWidget	       *widget);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_can_activate_accel  (GtkWidget           *widget,
                                           guint                signal_id);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_mnemonic_activate   (GtkWidget           *widget,
					   gboolean             group_cycling);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_event		  (GtkWidget	       *widget,
					   GdkEvent	       *event);
GDK_DEPRECATED_IN_3_22
gint       gtk_widget_send_expose         (GtkWidget           *widget,
					   GdkEvent            *event);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_send_focus_change   (GtkWidget           *widget,
                                           GdkEvent            *event);

GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_activate		     (GtkWidget	       *widget);
     
GDK_DEPRECATED_IN_3_14
void	   gtk_widget_reparent		  (GtkWidget	       *widget,
					   GtkWidget	       *new_parent);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_intersect		  (GtkWidget	       *widget,
					   const GdkRectangle  *area,
					   GdkRectangle	       *intersection);
GDK_DEPRECATED_IN_3_14
cairo_region_t *gtk_widget_region_intersect	  (GtkWidget	       *widget,
					   const cairo_region_t     *region);

GDK_AVAILABLE_IN_ALL
void	gtk_widget_freeze_child_notify	  (GtkWidget	       *widget);
GDK_AVAILABLE_IN_ALL
void	gtk_widget_child_notify		  (GtkWidget	       *widget,
					   const gchar	       *child_property);
GDK_AVAILABLE_IN_ALL
void	gtk_widget_thaw_child_notify	  (GtkWidget	       *widget);

GDK_AVAILABLE_IN_ALL
void       gtk_widget_set_can_focus       (GtkWidget           *widget,
                                           gboolean             can_focus);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_get_can_focus       (GtkWidget           *widget);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_has_focus           (GtkWidget           *widget);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_is_focus            (GtkWidget           *widget);
GDK_AVAILABLE_IN_3_2
gboolean   gtk_widget_has_visible_focus   (GtkWidget           *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_grab_focus          (GtkWidget           *widget);
GDK_AVAILABLE_IN_3_20
void       gtk_widget_set_focus_on_click  (GtkWidget           *widget,
                                           gboolean             focus_on_click);
GDK_AVAILABLE_IN_3_20
gboolean   gtk_widget_get_focus_on_click  (GtkWidget           *widget);

GDK_AVAILABLE_IN_ALL
void       gtk_widget_set_can_default     (GtkWidget           *widget,
                                           gboolean             can_default);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_get_can_default     (GtkWidget           *widget);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_has_default         (GtkWidget           *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_grab_default        (GtkWidget           *widget);

GDK_AVAILABLE_IN_ALL
void      gtk_widget_set_receives_default (GtkWidget           *widget,
                                           gboolean             receives_default);
GDK_AVAILABLE_IN_ALL
gboolean  gtk_widget_get_receives_default (GtkWidget           *widget);

GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_has_grab            (GtkWidget           *widget);

GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_device_is_shadowed  (GtkWidget           *widget,
                                           GdkDevice           *device);


GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_name               (GtkWidget    *widget,
							 const gchar  *name);
GDK_AVAILABLE_IN_ALL
const gchar *         gtk_widget_get_name               (GtkWidget    *widget);

GDK_DEPRECATED_IN_3_0_FOR(gtk_widget_set_state_flags)
void                  gtk_widget_set_state              (GtkWidget    *widget,
							 GtkStateType  state);

GDK_DEPRECATED_IN_3_0_FOR(gtk_widget_get_state_flags)
GtkStateType          gtk_widget_get_state              (GtkWidget    *widget);

GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_state_flags        (GtkWidget     *widget,
                                                         GtkStateFlags  flags,
                                                         gboolean       clear);
GDK_AVAILABLE_IN_ALL
void                  gtk_widget_unset_state_flags      (GtkWidget     *widget,
                                                         GtkStateFlags  flags);
GDK_AVAILABLE_IN_ALL
GtkStateFlags         gtk_widget_get_state_flags        (GtkWidget     *widget);

GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_sensitive          (GtkWidget    *widget,
							 gboolean      sensitive);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_widget_get_sensitive          (GtkWidget    *widget);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_widget_is_sensitive           (GtkWidget    *widget);

GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_visible            (GtkWidget    *widget,
                                                         gboolean      visible);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_widget_get_visible            (GtkWidget    *widget);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_widget_is_visible             (GtkWidget    *widget);

GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_has_window         (GtkWidget    *widget,
                                                         gboolean      has_window);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_widget_get_has_window         (GtkWidget    *widget);

GDK_AVAILABLE_IN_ALL
gboolean              gtk_widget_is_toplevel            (GtkWidget    *widget);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_widget_is_drawable            (GtkWidget    *widget);
GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_realized           (GtkWidget    *widget,
                                                         gboolean      realized);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_widget_get_realized           (GtkWidget    *widget);
GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_mapped             (GtkWidget    *widget,
                                                         gboolean      mapped);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_widget_get_mapped             (GtkWidget    *widget);

GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_app_paintable      (GtkWidget    *widget,
							 gboolean      app_paintable);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_widget_get_app_paintable      (GtkWidget    *widget);

GDK_DEPRECATED_IN_3_14
void                  gtk_widget_set_double_buffered    (GtkWidget    *widget,
							 gboolean      double_buffered);
GDK_DEPRECATED_IN_3_14
gboolean              gtk_widget_get_double_buffered    (GtkWidget    *widget);

GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_redraw_on_allocate (GtkWidget    *widget,
							 gboolean      redraw_on_allocate);

GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_parent             (GtkWidget    *widget,
							 GtkWidget    *parent);
GDK_AVAILABLE_IN_ALL
GtkWidget           * gtk_widget_get_parent             (GtkWidget    *widget);

GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_parent_window      (GtkWidget    *widget,
							 GdkWindow    *parent_window);
GDK_AVAILABLE_IN_ALL
GdkWindow           * gtk_widget_get_parent_window      (GtkWidget    *widget);

GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_child_visible      (GtkWidget    *widget,
							 gboolean      is_visible);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_widget_get_child_visible      (GtkWidget    *widget);

GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_window             (GtkWidget    *widget,
                                                         GdkWindow    *window);
GDK_AVAILABLE_IN_ALL
GdkWindow           * gtk_widget_get_window             (GtkWidget    *widget);
GDK_AVAILABLE_IN_3_8
void                  gtk_widget_register_window        (GtkWidget    *widget,
                                                         GdkWindow    *window);
GDK_AVAILABLE_IN_3_8
void                  gtk_widget_unregister_window      (GtkWidget    *widget,
                                                         GdkWindow    *window);

GDK_AVAILABLE_IN_ALL
int                   gtk_widget_get_allocated_width    (GtkWidget     *widget);
GDK_AVAILABLE_IN_ALL
int                   gtk_widget_get_allocated_height   (GtkWidget     *widget);
GDK_AVAILABLE_IN_3_10
int                   gtk_widget_get_allocated_baseline (GtkWidget     *widget);
GDK_AVAILABLE_IN_3_20
void                  gtk_widget_get_allocated_size     (GtkWidget     *widget,
                                                         GtkAllocation *allocation,
                                                         int           *baseline);

GDK_AVAILABLE_IN_ALL
void                  gtk_widget_get_allocation         (GtkWidget     *widget,
                                                         GtkAllocation *allocation);
GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_allocation         (GtkWidget     *widget,
                                                         const GtkAllocation *allocation);
GDK_AVAILABLE_IN_3_14
void                  gtk_widget_set_clip               (GtkWidget     *widget,
                                                         const GtkAllocation *clip);
GDK_AVAILABLE_IN_3_14
void                  gtk_widget_get_clip               (GtkWidget     *widget,
                                                         GtkAllocation *clip);

GDK_DEPRECATED_IN_3_0_FOR(gtk_widget_get_preferred_width & gtk_widget_get_preferred_height)

void                  gtk_widget_get_requisition        (GtkWidget     *widget,
                                                         GtkRequisition *requisition);

GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_child_focus         (GtkWidget           *widget,
                                           GtkDirectionType     direction);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_keynav_failed       (GtkWidget           *widget,
                                           GtkDirectionType     direction);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_error_bell          (GtkWidget           *widget);

GDK_AVAILABLE_IN_ALL
void       gtk_widget_set_size_request    (GtkWidget           *widget,
                                           gint                 width,
                                           gint                 height);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_get_size_request    (GtkWidget           *widget,
                                           gint                *width,
                                           gint                *height);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_set_events	  (GtkWidget	       *widget,
					   gint			events);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_add_events          (GtkWidget           *widget,
					   gint	                events);
GDK_AVAILABLE_IN_ALL
void	   gtk_widget_set_device_events	  (GtkWidget	       *widget,
                                           GdkDevice           *device,
					   GdkEventMask		events);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_add_device_events   (GtkWidget           *widget,
                                           GdkDevice           *device,
					   GdkEventMask         events);
GDK_AVAILABLE_IN_3_8
void	   gtk_widget_set_opacity	  (GtkWidget	       *widget,
					   double		opacity);
GDK_AVAILABLE_IN_3_8
double	   gtk_widget_get_opacity	  (GtkWidget	       *widget);

GDK_AVAILABLE_IN_ALL
void       gtk_widget_set_device_enabled  (GtkWidget    *widget,
                                           GdkDevice    *device,
                                           gboolean      enabled);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_get_device_enabled  (GtkWidget    *widget,
                                           GdkDevice    *device);

GDK_AVAILABLE_IN_ALL
GtkWidget*   gtk_widget_get_toplevel	(GtkWidget	*widget);
GDK_AVAILABLE_IN_ALL
GtkWidget*   gtk_widget_get_ancestor	(GtkWidget	*widget,
					 GType		 widget_type);
GDK_AVAILABLE_IN_ALL
GdkVisual*   gtk_widget_get_visual	(GtkWidget	*widget);
GDK_AVAILABLE_IN_ALL
void         gtk_widget_set_visual	(GtkWidget	*widget,
                                         GdkVisual      *visual);

GDK_AVAILABLE_IN_ALL
GdkScreen *   gtk_widget_get_screen      (GtkWidget *widget);
GDK_AVAILABLE_IN_ALL
gboolean      gtk_widget_has_screen      (GtkWidget *widget);
GDK_AVAILABLE_IN_3_10
gint          gtk_widget_get_scale_factor (GtkWidget *widget);
GDK_AVAILABLE_IN_ALL
GdkDisplay *  gtk_widget_get_display     (GtkWidget *widget);
GDK_DEPRECATED_IN_3_12
GdkWindow *   gtk_widget_get_root_window (GtkWidget *widget);
GDK_AVAILABLE_IN_ALL
GtkSettings*  gtk_widget_get_settings    (GtkWidget *widget);
GDK_AVAILABLE_IN_ALL
GtkClipboard *gtk_widget_get_clipboard   (GtkWidget *widget,
					  GdkAtom    selection);


/* Expand flags and related support */
GDK_AVAILABLE_IN_ALL
gboolean gtk_widget_get_hexpand          (GtkWidget      *widget);
GDK_AVAILABLE_IN_ALL
void     gtk_widget_set_hexpand          (GtkWidget      *widget,
                                          gboolean        expand);
GDK_AVAILABLE_IN_ALL
gboolean gtk_widget_get_hexpand_set      (GtkWidget      *widget);
GDK_AVAILABLE_IN_ALL
void     gtk_widget_set_hexpand_set      (GtkWidget      *widget,
                                          gboolean        set);
GDK_AVAILABLE_IN_ALL
gboolean gtk_widget_get_vexpand          (GtkWidget      *widget);
GDK_AVAILABLE_IN_ALL
void     gtk_widget_set_vexpand          (GtkWidget      *widget,
                                          gboolean        expand);
GDK_AVAILABLE_IN_ALL
gboolean gtk_widget_get_vexpand_set      (GtkWidget      *widget);
GDK_AVAILABLE_IN_ALL
void     gtk_widget_set_vexpand_set      (GtkWidget      *widget,
                                          gboolean        set);
GDK_AVAILABLE_IN_ALL
void     gtk_widget_queue_compute_expand (GtkWidget      *widget);
GDK_AVAILABLE_IN_ALL
gboolean gtk_widget_compute_expand       (GtkWidget      *widget,
                                          GtkOrientation  orientation);


/* Multidevice support */
GDK_AVAILABLE_IN_ALL
gboolean         gtk_widget_get_support_multidevice (GtkWidget      *widget);
GDK_AVAILABLE_IN_ALL
void             gtk_widget_set_support_multidevice (GtkWidget      *widget,
                                                     gboolean        support_multidevice);

/* Accessibility support */
GDK_AVAILABLE_IN_3_2
void             gtk_widget_class_set_accessible_type    (GtkWidgetClass     *widget_class,
                                                          GType               type);
GDK_AVAILABLE_IN_3_2
void             gtk_widget_class_set_accessible_role    (GtkWidgetClass     *widget_class,
                                                          AtkRole             role);
GDK_AVAILABLE_IN_ALL
AtkObject*       gtk_widget_get_accessible               (GtkWidget          *widget);


/* Margin and alignment */
GDK_AVAILABLE_IN_ALL
GtkAlign gtk_widget_get_halign        (GtkWidget *widget);
GDK_AVAILABLE_IN_ALL
void     gtk_widget_set_halign        (GtkWidget *widget,
                                       GtkAlign   align);
GDK_AVAILABLE_IN_ALL
GtkAlign gtk_widget_get_valign        (GtkWidget *widget);
GDK_AVAILABLE_IN_3_10
GtkAlign gtk_widget_get_valign_with_baseline (GtkWidget *widget);
GDK_AVAILABLE_IN_ALL
void     gtk_widget_set_valign        (GtkWidget *widget,
                                       GtkAlign   align);
GDK_DEPRECATED_IN_3_12_FOR(gtk_widget_get_margin_start)
gint     gtk_widget_get_margin_left   (GtkWidget *widget);
GDK_DEPRECATED_IN_3_12_FOR(gtk_widget_set_margin_start)
void     gtk_widget_set_margin_left   (GtkWidget *widget,
                                       gint       margin);
GDK_DEPRECATED_IN_3_12_FOR(gtk_widget_get_margin_end)
gint     gtk_widget_get_margin_right  (GtkWidget *widget);
GDK_DEPRECATED_IN_3_12_FOR(gtk_widget_set_margin_end)
void     gtk_widget_set_margin_right  (GtkWidget *widget,
                                       gint       margin);
GDK_AVAILABLE_IN_3_12
gint     gtk_widget_get_margin_start  (GtkWidget *widget);
GDK_AVAILABLE_IN_3_12
void     gtk_widget_set_margin_start  (GtkWidget *widget,
                                       gint       margin);
GDK_AVAILABLE_IN_3_12
gint     gtk_widget_get_margin_end    (GtkWidget *widget);
GDK_AVAILABLE_IN_3_12
void     gtk_widget_set_margin_end    (GtkWidget *widget,
                                       gint       margin);
GDK_AVAILABLE_IN_ALL
gint     gtk_widget_get_margin_top    (GtkWidget *widget);
GDK_AVAILABLE_IN_ALL
void     gtk_widget_set_margin_top    (GtkWidget *widget,
                                       gint       margin);
GDK_AVAILABLE_IN_ALL
gint     gtk_widget_get_margin_bottom (GtkWidget *widget);
GDK_AVAILABLE_IN_ALL
void     gtk_widget_set_margin_bottom (GtkWidget *widget,
                                       gint       margin);


GDK_AVAILABLE_IN_ALL
gint	     gtk_widget_get_events	(GtkWidget	*widget);
GDK_AVAILABLE_IN_ALL
GdkEventMask gtk_widget_get_device_events (GtkWidget	*widget,
                                           GdkDevice    *device);
GDK_DEPRECATED_IN_3_4_FOR(gdk_window_get_device_position)
void	     gtk_widget_get_pointer	(GtkWidget	*widget,
					 gint		*x,
					 gint		*y);

GDK_AVAILABLE_IN_ALL
gboolean     gtk_widget_is_ancestor	(GtkWidget	*widget,
					 GtkWidget	*ancestor);

GDK_AVAILABLE_IN_ALL
gboolean     gtk_widget_translate_coordinates (GtkWidget  *src_widget,
					       GtkWidget  *dest_widget,
					       gint        src_x,
					       gint        src_y,
					       gint       *dest_x,
					       gint       *dest_y);

/* Hide widget and return TRUE.
 */
GDK_AVAILABLE_IN_ALL
gboolean     gtk_widget_hide_on_delete	(GtkWidget	*widget);

/* Functions to override widget styling */
GDK_DEPRECATED_IN_3_16
void         gtk_widget_override_color            (GtkWidget     *widget,
                                                   GtkStateFlags  state,
                                                   const GdkRGBA *color);
GDK_DEPRECATED_IN_3_16
void         gtk_widget_override_background_color (GtkWidget     *widget,
                                                   GtkStateFlags  state,
                                                   const GdkRGBA *color);

GDK_DEPRECATED_IN_3_16
void         gtk_widget_override_font             (GtkWidget                  *widget,
                                                   const PangoFontDescription *font_desc);

GDK_DEPRECATED_IN_3_16
void         gtk_widget_override_symbolic_color   (GtkWidget     *widget,
                                                   const gchar   *name,
                                                   const GdkRGBA *color);
GDK_DEPRECATED_IN_3_16
void         gtk_widget_override_cursor           (GtkWidget       *widget,
                                                   const GdkRGBA   *cursor,
                                                   const GdkRGBA   *secondary_cursor);

GDK_AVAILABLE_IN_ALL
void       gtk_widget_reset_style       (GtkWidget      *widget);

GDK_AVAILABLE_IN_ALL
PangoContext *gtk_widget_create_pango_context (GtkWidget   *widget);
GDK_AVAILABLE_IN_ALL
PangoContext *gtk_widget_get_pango_context    (GtkWidget   *widget);
GDK_AVAILABLE_IN_3_18
void gtk_widget_set_font_options (GtkWidget                  *widget,
                                  const cairo_font_options_t *options);
GDK_AVAILABLE_IN_3_18
const cairo_font_options_t *gtk_widget_get_font_options (GtkWidget *widget);
GDK_AVAILABLE_IN_ALL
PangoLayout  *gtk_widget_create_pango_layout  (GtkWidget   *widget,
					       const gchar *text);

GDK_DEPRECATED_IN_3_10_FOR(gtk_icon_theme_load_icon)
GdkPixbuf    *gtk_widget_render_icon_pixbuf   (GtkWidget   *widget,
                                               const gchar *stock_id,
                                               GtkIconSize  size);

/* handle composite names for GTK_COMPOSITE_CHILD widgets,
 * the returned name is newly allocated.
 */
GDK_DEPRECATED_IN_3_10_FOR(gtk_widget_class_set_template)
void   gtk_widget_set_composite_name	(GtkWidget	*widget,
					 const gchar   	*name);
GDK_DEPRECATED_IN_3_10_FOR(gtk_widget_class_set_template)
gchar* gtk_widget_get_composite_name	(GtkWidget	*widget);
     
/* Push/pop pairs, to change default values upon a widget's creation.
 * This will override the values that got set by the
 * gtk_widget_set_default_* () functions.
 */
GDK_DEPRECATED_IN_3_10_FOR(gtk_widget_class_set_template)
void	     gtk_widget_push_composite_child (void);
GDK_DEPRECATED_IN_3_10_FOR(gtk_widget_class_set_template)
void	     gtk_widget_pop_composite_child  (void);

/* widget style properties
 */
GDK_AVAILABLE_IN_ALL
void gtk_widget_class_install_style_property        (GtkWidgetClass     *klass,
						     GParamSpec         *pspec);
GDK_AVAILABLE_IN_ALL
void gtk_widget_class_install_style_property_parser (GtkWidgetClass     *klass,
						     GParamSpec         *pspec,
						     GtkRcPropertyParser parser);
GDK_AVAILABLE_IN_ALL
GParamSpec*  gtk_widget_class_find_style_property   (GtkWidgetClass     *klass,
						     const gchar        *property_name);
GDK_AVAILABLE_IN_ALL
GParamSpec** gtk_widget_class_list_style_properties (GtkWidgetClass     *klass,
						     guint              *n_properties);
GDK_AVAILABLE_IN_ALL
void gtk_widget_style_get_property (GtkWidget	     *widget,
				    const gchar    *property_name,
				    GValue	     *value);
GDK_AVAILABLE_IN_ALL
void gtk_widget_style_get_valist   (GtkWidget	     *widget,
				    const gchar    *first_property_name,
				    va_list         var_args);
GDK_AVAILABLE_IN_ALL
void gtk_widget_style_get          (GtkWidget	     *widget,
				    const gchar    *first_property_name,
				    ...) G_GNUC_NULL_TERMINATED;

/* Functions for setting directionality for widgets */

GDK_AVAILABLE_IN_ALL
void             gtk_widget_set_direction         (GtkWidget        *widget,
						   GtkTextDirection  dir);
GDK_AVAILABLE_IN_ALL
GtkTextDirection gtk_widget_get_direction         (GtkWidget        *widget);

GDK_AVAILABLE_IN_ALL
void             gtk_widget_set_default_direction (GtkTextDirection  dir);
GDK_AVAILABLE_IN_ALL
GtkTextDirection gtk_widget_get_default_direction (void);

/* Compositing manager functionality */
GDK_DEPRECATED_IN_3_22_FOR(gdk_screen_is_composited)
gboolean gtk_widget_is_composited (GtkWidget *widget);

/* Counterpart to gdk_window_shape_combine_region.
 */
GDK_AVAILABLE_IN_ALL
void	     gtk_widget_shape_combine_region (GtkWidget *widget,
                                              cairo_region_t *region);
GDK_AVAILABLE_IN_ALL
void	     gtk_widget_input_shape_combine_region (GtkWidget *widget,
                                                    cairo_region_t *region);

GDK_AVAILABLE_IN_ALL
GList* gtk_widget_list_mnemonic_labels  (GtkWidget *widget);
GDK_AVAILABLE_IN_ALL
void   gtk_widget_add_mnemonic_label    (GtkWidget *widget,
					 GtkWidget *label);
GDK_AVAILABLE_IN_ALL
void   gtk_widget_remove_mnemonic_label (GtkWidget *widget,
					 GtkWidget *label);

GDK_AVAILABLE_IN_ALL
void                  gtk_widget_set_tooltip_window    (GtkWidget   *widget,
                                                        GtkWindow   *custom_window);
GDK_AVAILABLE_IN_ALL
GtkWindow *gtk_widget_get_tooltip_window    (GtkWidget   *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_trigger_tooltip_query (GtkWidget   *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_set_tooltip_text      (GtkWidget   *widget,
                                             const gchar *text);
GDK_AVAILABLE_IN_ALL
gchar *    gtk_widget_get_tooltip_text      (GtkWidget   *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_set_tooltip_markup    (GtkWidget   *widget,
                                             const gchar *markup);
GDK_AVAILABLE_IN_ALL
gchar *    gtk_widget_get_tooltip_markup    (GtkWidget   *widget);
GDK_AVAILABLE_IN_ALL
void       gtk_widget_set_has_tooltip       (GtkWidget   *widget,
					     gboolean     has_tooltip);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_widget_get_has_tooltip       (GtkWidget   *widget);

GDK_AVAILABLE_IN_ALL
gboolean   gtk_cairo_should_draw_window     (cairo_t     *cr,
                                             GdkWindow   *window);
GDK_AVAILABLE_IN_ALL
void       gtk_cairo_transform_to_window    (cairo_t     *cr,
                                             GtkWidget   *widget,
                                             GdkWindow   *window);

GDK_AVAILABLE_IN_ALL
GType           gtk_requisition_get_type (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkRequisition *gtk_requisition_new      (void) G_GNUC_MALLOC;
GDK_AVAILABLE_IN_ALL
GtkRequisition *gtk_requisition_copy     (const GtkRequisition *requisition);
GDK_AVAILABLE_IN_ALL
void            gtk_requisition_free     (GtkRequisition       *requisition);

GDK_AVAILABLE_IN_ALL
gboolean     gtk_widget_in_destruction (GtkWidget *widget);

GDK_AVAILABLE_IN_ALL
GtkStyleContext * gtk_widget_get_style_context (GtkWidget *widget);

GDK_AVAILABLE_IN_ALL
GtkWidgetPath *   gtk_widget_get_path (GtkWidget *widget);

GDK_AVAILABLE_IN_3_20
void              gtk_widget_class_set_css_name (GtkWidgetClass *widget_class,
                                                 const char     *name);
GDK_AVAILABLE_IN_3_20
const char *      gtk_widget_class_get_css_name (GtkWidgetClass *widget_class);

GDK_AVAILABLE_IN_3_4
GdkModifierType   gtk_widget_get_modifier_mask (GtkWidget         *widget,
                                                GdkModifierIntent  intent);

GDK_AVAILABLE_IN_3_6
void                    gtk_widget_insert_action_group                  (GtkWidget    *widget,
                                                                         const gchar  *name,
                                                                         GActionGroup *group);



GDK_AVAILABLE_IN_3_8
guint gtk_widget_add_tick_callback (GtkWidget       *widget,
                                    GtkTickCallback  callback,
                                    gpointer         user_data,
                                    GDestroyNotify   notify);

GDK_AVAILABLE_IN_3_8
void gtk_widget_remove_tick_callback (GtkWidget       *widget,
                                      guint            id);

/**
 * gtk_widget_class_bind_template_callback:
 * @widget_class: a #GtkWidgetClass
 * @callback: the callback symbol
 *
 * Binds a callback function defined in a template to the @widget_class.
 *
 * This macro is a convenience wrapper around the
 * gtk_widget_class_bind_template_callback_full() function.
 *
 * Since: 3.10
 */
#define gtk_widget_class_bind_template_callback(widget_class, callback) \
  gtk_widget_class_bind_template_callback_full (GTK_WIDGET_CLASS (widget_class), \
                                                #callback, \
                                                G_CALLBACK (callback))

/**
 * gtk_widget_class_bind_template_child:
 * @widget_class: a #GtkWidgetClass
 * @TypeName: the type name of this widget
 * @member_name: name of the instance member in the instance struct for @data_type
 *
 * Binds a child widget defined in a template to the @widget_class.
 *
 * This macro is a convenience wrapper around the
 * gtk_widget_class_bind_template_child_full() function.
 *
 * This macro will use the offset of the @member_name inside the @TypeName
 * instance structure.
 *
 * Since: 3.10
 */
#define gtk_widget_class_bind_template_child(widget_class, TypeName, member_name) \
  gtk_widget_class_bind_template_child_full (widget_class, \
                                             #member_name, \
                                             FALSE, \
                                             G_STRUCT_OFFSET (TypeName, member_name))

/**
 * gtk_widget_class_bind_template_child_internal:
 * @widget_class: a #GtkWidgetClass
 * @TypeName: the type name, in CamelCase
 * @member_name: name of the instance member in the instance struct for @data_type
 *
 * Binds a child widget defined in a template to the @widget_class, and
 * also makes it available as an internal child in GtkBuilder, under the
 * name @member_name.
 *
 * This macro is a convenience wrapper around the
 * gtk_widget_class_bind_template_child_full() function.
 *
 * This macro will use the offset of the @member_name inside the @TypeName
 * instance structure.
 *
 * Since: 3.10
 */
#define gtk_widget_class_bind_template_child_internal(widget_class, TypeName, member_name) \
  gtk_widget_class_bind_template_child_full (widget_class, \
                                             #member_name, \
                                             TRUE, \
                                             G_STRUCT_OFFSET (TypeName, member_name))

/**
 * gtk_widget_class_bind_template_child_private:
 * @widget_class: a #GtkWidgetClass
 * @TypeName: the type name of this widget
 * @member_name: name of the instance private member in the private struct for @data_type
 *
 * Binds a child widget defined in a template to the @widget_class.
 *
 * This macro is a convenience wrapper around the
 * gtk_widget_class_bind_template_child_full() function.
 *
 * This macro will use the offset of the @member_name inside the @TypeName
 * private data structure (it uses G_PRIVATE_OFFSET(), so the private struct
 * must be added with G_ADD_PRIVATE()).
 *
 * Since: 3.10
 */
#define gtk_widget_class_bind_template_child_private(widget_class, TypeName, member_name) \
  gtk_widget_class_bind_template_child_full (widget_class, \
                                             #member_name, \
                                             FALSE, \
                                             G_PRIVATE_OFFSET (TypeName, member_name))

/**
 * gtk_widget_class_bind_template_child_internal_private:
 * @widget_class: a #GtkWidgetClass
 * @TypeName: the type name, in CamelCase
 * @member_name: name of the instance private member on the private struct for @data_type
 *
 * Binds a child widget defined in a template to the @widget_class, and
 * also makes it available as an internal child in GtkBuilder, under the
 * name @member_name.
 *
 * This macro is a convenience wrapper around the
 * gtk_widget_class_bind_template_child_full() function.
 *
 * This macro will use the offset of the @member_name inside the @TypeName
 * private data structure.
 *
 * Since: 3.10
 */
#define gtk_widget_class_bind_template_child_internal_private(widget_class, TypeName, member_name) \
  gtk_widget_class_bind_template_child_full (widget_class, \
                                             #member_name, \
                                             TRUE, \
                                             G_PRIVATE_OFFSET (TypeName, member_name))

GDK_AVAILABLE_IN_3_10
void    gtk_widget_init_template                        (GtkWidget             *widget);
GDK_AVAILABLE_IN_3_10
GObject *gtk_widget_get_template_child                  (GtkWidget             *widget,
						         GType                  widget_type,
						         const gchar           *name);
GDK_AVAILABLE_IN_3_10
void    gtk_widget_class_set_template                   (GtkWidgetClass        *widget_class,
						         GBytes                *template_bytes);
GDK_AVAILABLE_IN_3_10
void    gtk_widget_class_set_template_from_resource     (GtkWidgetClass        *widget_class,
						         const gchar           *resource_name);
GDK_AVAILABLE_IN_3_10
void    gtk_widget_class_bind_template_callback_full    (GtkWidgetClass        *widget_class,
						         const gchar           *callback_name,
						         GCallback              callback_symbol);
GDK_AVAILABLE_IN_3_10
void    gtk_widget_class_set_connect_func               (GtkWidgetClass        *widget_class,
						         GtkBuilderConnectFunc  connect_func,
						         gpointer               connect_data,
						         GDestroyNotify         connect_data_destroy);
GDK_AVAILABLE_IN_3_10
void    gtk_widget_class_bind_template_child_full       (GtkWidgetClass        *widget_class,
						         const gchar           *name,
						         gboolean               internal_child,
						         gssize                 struct_offset);

GDK_AVAILABLE_IN_3_16
GActionGroup           *gtk_widget_get_action_group     (GtkWidget             *widget,
                                                         const gchar           *prefix);

GDK_AVAILABLE_IN_3_16
const gchar **          gtk_widget_list_action_prefixes (GtkWidget             *widget);

GDK_AVAILABLE_IN_3_18
void                    gtk_widget_set_font_map         (GtkWidget             *widget,
                                                         PangoFontMap          *font_map);
GDK_AVAILABLE_IN_3_18
PangoFontMap *          gtk_widget_get_font_map         (GtkWidget             *widget);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkWidget, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkRequisition, gtk_requisition_free)

G_END_DECLS

#endif /* __GTK_WIDGET_H__ */
