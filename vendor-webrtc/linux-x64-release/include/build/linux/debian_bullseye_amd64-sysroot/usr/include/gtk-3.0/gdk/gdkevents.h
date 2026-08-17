/* GDK - The GIMP Drawing Kit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
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

#ifndef __GDK_EVENTS_H__
#define __GDK_EVENTS_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>
#include <gdk/gdkdnd.h>
#include <gdk/gdkdevice.h>
#include <gdk/gdkdevicetool.h>

G_BEGIN_DECLS


/**
 * SECTION:event_structs
 * @Short_description: Data structures specific to each type of event
 * @Title: Event Structures
 *
 * The event structures contain data specific to each type of event in GDK.
 *
 * > A common mistake is to forget to set the event mask of a widget so that
 * > the required events are received. See gtk_widget_set_events().
 */


#define GDK_TYPE_EVENT          (gdk_event_get_type ())
#define GDK_TYPE_EVENT_SEQUENCE (gdk_event_sequence_get_type ())

/**
 * GDK_PRIORITY_EVENTS:
 *
 * This is the priority that events from the X server are given in the
 * [GLib Main Loop][glib-The-Main-Event-Loop].
 */
#define GDK_PRIORITY_EVENTS	(G_PRIORITY_DEFAULT)

/**
 * GDK_PRIORITY_REDRAW: (value 120)
 *
 * This is the priority that the idle handler processing window updates
 * is given in the
 * [GLib Main Loop][glib-The-Main-Event-Loop].
 */
#define GDK_PRIORITY_REDRAW     (G_PRIORITY_HIGH_IDLE + 20)

/**
 * GDK_EVENT_PROPAGATE:
 *
 * Use this macro as the return value for continuing the propagation of
 * an event handler.
 *
 * Since: 3.4
 */
#define GDK_EVENT_PROPAGATE     (FALSE)

/**
 * GDK_EVENT_STOP:
 *
 * Use this macro as the return value for stopping the propagation of
 * an event handler.
 *
 * Since: 3.4
 */
#define GDK_EVENT_STOP          (TRUE)

/**
 * GDK_BUTTON_PRIMARY:
 *
 * The primary button. This is typically the left mouse button, or the
 * right button in a left-handed setup.
 *
 * Since: 3.4
 */
#define GDK_BUTTON_PRIMARY      (1)

/**
 * GDK_BUTTON_MIDDLE:
 *
 * The middle button.
 *
 * Since: 3.4
 */
#define GDK_BUTTON_MIDDLE       (2)

/**
 * GDK_BUTTON_SECONDARY:
 *
 * The secondary button. This is typically the right mouse button, or the
 * left button in a left-handed setup.
 *
 * Since: 3.4
 */
#define GDK_BUTTON_SECONDARY    (3)



typedef struct _GdkEventAny	    GdkEventAny;
typedef struct _GdkEventExpose	    GdkEventExpose;
typedef struct _GdkEventVisibility  GdkEventVisibility;
typedef struct _GdkEventMotion	    GdkEventMotion;
typedef struct _GdkEventButton	    GdkEventButton;
typedef struct _GdkEventTouch       GdkEventTouch;
typedef struct _GdkEventScroll      GdkEventScroll;  
typedef struct _GdkEventKey	    GdkEventKey;
typedef struct _GdkEventFocus	    GdkEventFocus;
typedef struct _GdkEventCrossing    GdkEventCrossing;
typedef struct _GdkEventConfigure   GdkEventConfigure;
typedef struct _GdkEventProperty    GdkEventProperty;
typedef struct _GdkEventSelection   GdkEventSelection;
typedef struct _GdkEventOwnerChange GdkEventOwnerChange;
typedef struct _GdkEventProximity   GdkEventProximity;
typedef struct _GdkEventDND         GdkEventDND;
typedef struct _GdkEventWindowState GdkEventWindowState;
typedef struct _GdkEventSetting     GdkEventSetting;
typedef struct _GdkEventGrabBroken  GdkEventGrabBroken;
typedef struct _GdkEventTouchpadSwipe GdkEventTouchpadSwipe;
typedef struct _GdkEventTouchpadPinch GdkEventTouchpadPinch;
typedef struct _GdkEventPadButton   GdkEventPadButton;
typedef struct _GdkEventPadAxis     GdkEventPadAxis;
typedef struct _GdkEventPadGroupMode GdkEventPadGroupMode;

typedef struct _GdkEventSequence    GdkEventSequence;

typedef union  _GdkEvent	    GdkEvent;

/**
 * GdkEventFunc:
 * @event: the #GdkEvent to process.
 * @data: (closure): user data set when the event handler was installed with
 *   gdk_event_handler_set().
 *
 * Specifies the type of function passed to gdk_event_handler_set() to
 * handle all GDK events.
 */
typedef void (*GdkEventFunc) (GdkEvent *event,
			      gpointer	data);

/* Event filtering */

/**
 * GdkXEvent:
 *
 * Used to represent native events (XEvents for the X11
 * backend, MSGs for Win32).
 */
typedef void GdkXEvent;	  /* Can be cast to window system specific
			   * even type, XEvent on X11, MSG on Win32.
			   */

/**
 * GdkFilterReturn:
 * @GDK_FILTER_CONTINUE: event not handled, continue processing.
 * @GDK_FILTER_TRANSLATE: native event translated into a GDK event and stored
 *  in the `event` structure that was passed in.
 * @GDK_FILTER_REMOVE: event handled, terminate processing.
 *
 * Specifies the result of applying a #GdkFilterFunc to a native event.
 */
typedef enum {
  GDK_FILTER_CONTINUE,	  /* Event not handled, continue processesing */
  GDK_FILTER_TRANSLATE,	  /* Native event translated into a GDK event and
                             stored in the "event" structure that was
                             passed in */
  GDK_FILTER_REMOVE	  /* Terminate processing, removing event */
} GdkFilterReturn;

/**
 * GdkFilterFunc:
 * @xevent: the native event to filter.
 * @event: the GDK event to which the X event will be translated.
 * @data: (closure): user data set when the filter was installed.
 *
 * Specifies the type of function used to filter native events before they are
 * converted to GDK events.
 *
 * When a filter is called, @event is unpopulated, except for
 * `event->window`. The filter may translate the native
 * event to a GDK event and store the result in @event, or handle it without
 * translation. If the filter translates the event and processing should
 * continue, it should return %GDK_FILTER_TRANSLATE.
 *
 * Returns: a #GdkFilterReturn value.
 */
typedef GdkFilterReturn (*GdkFilterFunc) (GdkXEvent *xevent,
					  GdkEvent *event,
					  gpointer  data);


/**
 * GdkEventType:
 * @GDK_NOTHING: a special code to indicate a null event.
 * @GDK_DELETE: the window manager has requested that the toplevel window be
 *   hidden or destroyed, usually when the user clicks on a special icon in the
 *   title bar.
 * @GDK_DESTROY: the window has been destroyed.
 * @GDK_EXPOSE: all or part of the window has become visible and needs to be
 *   redrawn.
 * @GDK_MOTION_NOTIFY: the pointer (usually a mouse) has moved.
 * @GDK_BUTTON_PRESS: a mouse button has been pressed.
 * @GDK_2BUTTON_PRESS: a mouse button has been double-clicked (clicked twice
 *   within a short period of time). Note that each click also generates a
 *   %GDK_BUTTON_PRESS event.
 * @GDK_DOUBLE_BUTTON_PRESS: alias for %GDK_2BUTTON_PRESS, added in 3.6.
 * @GDK_3BUTTON_PRESS: a mouse button has been clicked 3 times in a short period
 *   of time. Note that each click also generates a %GDK_BUTTON_PRESS event.
 * @GDK_TRIPLE_BUTTON_PRESS: alias for %GDK_3BUTTON_PRESS, added in 3.6.
 * @GDK_BUTTON_RELEASE: a mouse button has been released.
 * @GDK_KEY_PRESS: a key has been pressed.
 * @GDK_KEY_RELEASE: a key has been released.
 * @GDK_ENTER_NOTIFY: the pointer has entered the window.
 * @GDK_LEAVE_NOTIFY: the pointer has left the window.
 * @GDK_FOCUS_CHANGE: the keyboard focus has entered or left the window.
 * @GDK_CONFIGURE: the size, position or stacking order of the window has changed.
 *   Note that GTK+ discards these events for %GDK_WINDOW_CHILD windows.
 * @GDK_MAP: the window has been mapped.
 * @GDK_UNMAP: the window has been unmapped.
 * @GDK_PROPERTY_NOTIFY: a property on the window has been changed or deleted.
 * @GDK_SELECTION_CLEAR: the application has lost ownership of a selection.
 * @GDK_SELECTION_REQUEST: another application has requested a selection.
 * @GDK_SELECTION_NOTIFY: a selection has been received.
 * @GDK_PROXIMITY_IN: an input device has moved into contact with a sensing
 *   surface (e.g. a touchscreen or graphics tablet).
 * @GDK_PROXIMITY_OUT: an input device has moved out of contact with a sensing
 *   surface.
 * @GDK_DRAG_ENTER: the mouse has entered the window while a drag is in progress.
 * @GDK_DRAG_LEAVE: the mouse has left the window while a drag is in progress.
 * @GDK_DRAG_MOTION: the mouse has moved in the window while a drag is in
 *   progress.
 * @GDK_DRAG_STATUS: the status of the drag operation initiated by the window
 *   has changed.
 * @GDK_DROP_START: a drop operation onto the window has started.
 * @GDK_DROP_FINISHED: the drop operation initiated by the window has completed.
 * @GDK_CLIENT_EVENT: a message has been received from another application.
 * @GDK_VISIBILITY_NOTIFY: the window visibility status has changed.
 * @GDK_SCROLL: the scroll wheel was turned
 * @GDK_WINDOW_STATE: the state of a window has changed. See #GdkWindowState
 *   for the possible window states
 * @GDK_SETTING: a setting has been modified.
 * @GDK_OWNER_CHANGE: the owner of a selection has changed. This event type
 *   was added in 2.6
 * @GDK_GRAB_BROKEN: a pointer or keyboard grab was broken. This event type
 *   was added in 2.8.
 * @GDK_DAMAGE: the content of the window has been changed. This event type
 *   was added in 2.14.
 * @GDK_TOUCH_BEGIN: A new touch event sequence has just started. This event
 *   type was added in 3.4.
 * @GDK_TOUCH_UPDATE: A touch event sequence has been updated. This event type
 *   was added in 3.4.
 * @GDK_TOUCH_END: A touch event sequence has finished. This event type
 *   was added in 3.4.
 * @GDK_TOUCH_CANCEL: A touch event sequence has been canceled. This event type
 *   was added in 3.4.
 * @GDK_TOUCHPAD_SWIPE: A touchpad swipe gesture event, the current state
 *   is determined by its phase field. This event type was added in 3.18.
 * @GDK_TOUCHPAD_PINCH: A touchpad pinch gesture event, the current state
 *   is determined by its phase field. This event type was added in 3.18.
 * @GDK_PAD_BUTTON_PRESS: A tablet pad button press event. This event type
 *   was added in 3.22.
 * @GDK_PAD_BUTTON_RELEASE: A tablet pad button release event. This event type
 *   was added in 3.22.
 * @GDK_PAD_RING: A tablet pad axis event from a "ring". This event type was
 *   added in 3.22.
 * @GDK_PAD_STRIP: A tablet pad axis event from a "strip". This event type was
 *   added in 3.22.
 * @GDK_PAD_GROUP_MODE: A tablet pad group mode change. This event type was
 *   added in 3.22.
 * @GDK_EVENT_LAST: marks the end of the GdkEventType enumeration. Added in 2.18
 *
 * Specifies the type of the event.
 *
 * Do not confuse these events with the signals that GTK+ widgets emit.
 * Although many of these events result in corresponding signals being emitted,
 * the events are often transformed or filtered along the way.
 *
 * In some language bindings, the values %GDK_2BUTTON_PRESS and
 * %GDK_3BUTTON_PRESS would translate into something syntactically
 * invalid (eg `Gdk.EventType.2ButtonPress`, where a
 * symbol is not allowed to start with a number). In that case, the
 * aliases %GDK_DOUBLE_BUTTON_PRESS and %GDK_TRIPLE_BUTTON_PRESS can
 * be used instead.
 */
typedef enum
{
  GDK_NOTHING		= -1,
  GDK_DELETE		= 0,
  GDK_DESTROY		= 1,
  GDK_EXPOSE		= 2,
  GDK_MOTION_NOTIFY	= 3,
  GDK_BUTTON_PRESS	= 4,
  GDK_2BUTTON_PRESS	= 5,
  GDK_DOUBLE_BUTTON_PRESS = GDK_2BUTTON_PRESS,
  GDK_3BUTTON_PRESS	= 6,
  GDK_TRIPLE_BUTTON_PRESS = GDK_3BUTTON_PRESS,
  GDK_BUTTON_RELEASE	= 7,
  GDK_KEY_PRESS		= 8,
  GDK_KEY_RELEASE	= 9,
  GDK_ENTER_NOTIFY	= 10,
  GDK_LEAVE_NOTIFY	= 11,
  GDK_FOCUS_CHANGE	= 12,
  GDK_CONFIGURE		= 13,
  GDK_MAP		= 14,
  GDK_UNMAP		= 15,
  GDK_PROPERTY_NOTIFY	= 16,
  GDK_SELECTION_CLEAR	= 17,
  GDK_SELECTION_REQUEST = 18,
  GDK_SELECTION_NOTIFY	= 19,
  GDK_PROXIMITY_IN	= 20,
  GDK_PROXIMITY_OUT	= 21,
  GDK_DRAG_ENTER        = 22,
  GDK_DRAG_LEAVE        = 23,
  GDK_DRAG_MOTION       = 24,
  GDK_DRAG_STATUS       = 25,
  GDK_DROP_START        = 26,
  GDK_DROP_FINISHED     = 27,
  GDK_CLIENT_EVENT	= 28,
  GDK_VISIBILITY_NOTIFY = 29,
  GDK_SCROLL            = 31,
  GDK_WINDOW_STATE      = 32,
  GDK_SETTING           = 33,
  GDK_OWNER_CHANGE      = 34,
  GDK_GRAB_BROKEN       = 35,
  GDK_DAMAGE            = 36,
  GDK_TOUCH_BEGIN       = 37,
  GDK_TOUCH_UPDATE      = 38,
  GDK_TOUCH_END         = 39,
  GDK_TOUCH_CANCEL      = 40,
  GDK_TOUCHPAD_SWIPE    = 41,
  GDK_TOUCHPAD_PINCH    = 42,
  GDK_PAD_BUTTON_PRESS  = 43,
  GDK_PAD_BUTTON_RELEASE = 44,
  GDK_PAD_RING          = 45,
  GDK_PAD_STRIP         = 46,
  GDK_PAD_GROUP_MODE    = 47,
  GDK_EVENT_LAST        /* helper variable for decls */
} GdkEventType;

/**
 * GdkVisibilityState:
 * @GDK_VISIBILITY_UNOBSCURED: the window is completely visible.
 * @GDK_VISIBILITY_PARTIAL: the window is partially visible.
 * @GDK_VISIBILITY_FULLY_OBSCURED: the window is not visible at all.
 *
 * Specifies the visiblity status of a window for a #GdkEventVisibility.
 */
typedef enum
{
  GDK_VISIBILITY_UNOBSCURED,
  GDK_VISIBILITY_PARTIAL,
  GDK_VISIBILITY_FULLY_OBSCURED
} GdkVisibilityState;

/**
 * GdkTouchpadGesturePhase:
 * @GDK_TOUCHPAD_GESTURE_PHASE_BEGIN: The gesture has begun.
 * @GDK_TOUCHPAD_GESTURE_PHASE_UPDATE: The gesture has been updated.
 * @GDK_TOUCHPAD_GESTURE_PHASE_END: The gesture was finished, changes
 *   should be permanently applied.
 * @GDK_TOUCHPAD_GESTURE_PHASE_CANCEL: The gesture was cancelled, all
 *   changes should be undone.
 *
 * Specifies the current state of a touchpad gesture. All gestures are
 * guaranteed to begin with an event with phase %GDK_TOUCHPAD_GESTURE_PHASE_BEGIN,
 * followed by 0 or several events with phase %GDK_TOUCHPAD_GESTURE_PHASE_UPDATE.
 *
 * A finished gesture may have 2 possible outcomes, an event with phase
 * %GDK_TOUCHPAD_GESTURE_PHASE_END will be emitted when the gesture is
 * considered successful, this should be used as the hint to perform any
 * permanent changes.

 * Cancelled gestures may be so for a variety of reasons, due to hardware
 * or the compositor, or due to the gesture recognition layers hinting the
 * gesture did not finish resolutely (eg. a 3rd finger being added during
 * a pinch gesture). In these cases, the last event will report the phase
 * %GDK_TOUCHPAD_GESTURE_PHASE_CANCEL, this should be used as a hint
 * to undo any visible/permanent changes that were done throughout the
 * progress of the gesture.
 *
 * See also #GdkEventTouchpadSwipe and #GdkEventTouchpadPinch.
 *
 */
typedef enum
{
  GDK_TOUCHPAD_GESTURE_PHASE_BEGIN,
  GDK_TOUCHPAD_GESTURE_PHASE_UPDATE,
  GDK_TOUCHPAD_GESTURE_PHASE_END,
  GDK_TOUCHPAD_GESTURE_PHASE_CANCEL
} GdkTouchpadGesturePhase;

/**
 * GdkScrollDirection:
 * @GDK_SCROLL_UP: the window is scrolled up.
 * @GDK_SCROLL_DOWN: the window is scrolled down.
 * @GDK_SCROLL_LEFT: the window is scrolled to the left.
 * @GDK_SCROLL_RIGHT: the window is scrolled to the right.
 * @GDK_SCROLL_SMOOTH: the scrolling is determined by the delta values
 *   in #GdkEventScroll. See gdk_event_get_scroll_deltas(). Since: 3.4
 *
 * Specifies the direction for #GdkEventScroll.
 */
typedef enum
{
  GDK_SCROLL_UP,
  GDK_SCROLL_DOWN,
  GDK_SCROLL_LEFT,
  GDK_SCROLL_RIGHT,
  GDK_SCROLL_SMOOTH
} GdkScrollDirection;

/**
 * GdkNotifyType:
 * @GDK_NOTIFY_ANCESTOR: the window is entered from an ancestor or
 *   left towards an ancestor.
 * @GDK_NOTIFY_VIRTUAL: the pointer moves between an ancestor and an
 *   inferior of the window.
 * @GDK_NOTIFY_INFERIOR: the window is entered from an inferior or
 *   left towards an inferior.
 * @GDK_NOTIFY_NONLINEAR: the window is entered from or left towards
 *   a window which is neither an ancestor nor an inferior.
 * @GDK_NOTIFY_NONLINEAR_VIRTUAL: the pointer moves between two windows
 *   which are not ancestors of each other and the window is part of
 *   the ancestor chain between one of these windows and their least
 *   common ancestor.
 * @GDK_NOTIFY_UNKNOWN: an unknown type of enter/leave event occurred.
 *
 * Specifies the kind of crossing for #GdkEventCrossing.
 *
 * See the X11 protocol specification of LeaveNotify for
 * full details of crossing event generation.
 */
typedef enum
{
  GDK_NOTIFY_ANCESTOR		= 0,
  GDK_NOTIFY_VIRTUAL		= 1,
  GDK_NOTIFY_INFERIOR		= 2,
  GDK_NOTIFY_NONLINEAR		= 3,
  GDK_NOTIFY_NONLINEAR_VIRTUAL	= 4,
  GDK_NOTIFY_UNKNOWN		= 5
} GdkNotifyType;

/**
 * GdkCrossingMode:
 * @GDK_CROSSING_NORMAL: crossing because of pointer motion.
 * @GDK_CROSSING_GRAB: crossing because a grab is activated.
 * @GDK_CROSSING_UNGRAB: crossing because a grab is deactivated.
 * @GDK_CROSSING_GTK_GRAB: crossing because a GTK+ grab is activated.
 * @GDK_CROSSING_GTK_UNGRAB: crossing because a GTK+ grab is deactivated.
 * @GDK_CROSSING_STATE_CHANGED: crossing because a GTK+ widget changed
 *   state (e.g. sensitivity).
 * @GDK_CROSSING_TOUCH_BEGIN: crossing because a touch sequence has begun,
 *   this event is synthetic as the pointer might have not left the window.
 * @GDK_CROSSING_TOUCH_END: crossing because a touch sequence has ended,
 *   this event is synthetic as the pointer might have not left the window.
 * @GDK_CROSSING_DEVICE_SWITCH: crossing because of a device switch (i.e.
 *   a mouse taking control of the pointer after a touch device), this event
 *   is synthetic as the pointer didn’t leave the window.
 *
 * Specifies the crossing mode for #GdkEventCrossing.
 */
typedef enum
{
  GDK_CROSSING_NORMAL,
  GDK_CROSSING_GRAB,
  GDK_CROSSING_UNGRAB,
  GDK_CROSSING_GTK_GRAB,
  GDK_CROSSING_GTK_UNGRAB,
  GDK_CROSSING_STATE_CHANGED,
  GDK_CROSSING_TOUCH_BEGIN,
  GDK_CROSSING_TOUCH_END,
  GDK_CROSSING_DEVICE_SWITCH
} GdkCrossingMode;

/**
 * GdkPropertyState:
 * @GDK_PROPERTY_NEW_VALUE: the property value was changed.
 * @GDK_PROPERTY_DELETE: the property was deleted.
 *
 * Specifies the type of a property change for a #GdkEventProperty.
 */
typedef enum
{
  GDK_PROPERTY_NEW_VALUE,
  GDK_PROPERTY_DELETE
} GdkPropertyState;

/**
 * GdkWindowState:
 * @GDK_WINDOW_STATE_WITHDRAWN: the window is not shown.
 * @GDK_WINDOW_STATE_ICONIFIED: the window is minimized.
 * @GDK_WINDOW_STATE_MAXIMIZED: the window is maximized.
 * @GDK_WINDOW_STATE_STICKY: the window is sticky.
 * @GDK_WINDOW_STATE_FULLSCREEN: the window is maximized without
 *   decorations.
 * @GDK_WINDOW_STATE_ABOVE: the window is kept above other windows.
 * @GDK_WINDOW_STATE_BELOW: the window is kept below other windows.
 * @GDK_WINDOW_STATE_FOCUSED: the window is presented as focused (with active decorations).
 * @GDK_WINDOW_STATE_TILED: the window is in a tiled state, Since 3.10. Since 3.22.23, this
 *                          is deprecated in favor of per-edge information.
 * @GDK_WINDOW_STATE_TOP_TILED: whether the top edge is tiled, Since 3.22.23
 * @GDK_WINDOW_STATE_TOP_RESIZABLE: whether the top edge is resizable, Since 3.22.23
 * @GDK_WINDOW_STATE_RIGHT_TILED: whether the right edge is tiled, Since 3.22.23
 * @GDK_WINDOW_STATE_RIGHT_RESIZABLE: whether the right edge is resizable, Since 3.22.23
 * @GDK_WINDOW_STATE_BOTTOM_TILED: whether the bottom edge is tiled, Since 3.22.23
 * @GDK_WINDOW_STATE_BOTTOM_RESIZABLE: whether the bottom edge is resizable, Since 3.22.23
 * @GDK_WINDOW_STATE_LEFT_TILED: whether the left edge is tiled, Since 3.22.23
 * @GDK_WINDOW_STATE_LEFT_RESIZABLE: whether the left edge is resizable, Since 3.22.23
 *
 * Specifies the state of a toplevel window.
 */
typedef enum
{
  GDK_WINDOW_STATE_WITHDRAWN        = 1 << 0,
  GDK_WINDOW_STATE_ICONIFIED        = 1 << 1,
  GDK_WINDOW_STATE_MAXIMIZED        = 1 << 2,
  GDK_WINDOW_STATE_STICKY           = 1 << 3,
  GDK_WINDOW_STATE_FULLSCREEN       = 1 << 4,
  GDK_WINDOW_STATE_ABOVE            = 1 << 5,
  GDK_WINDOW_STATE_BELOW            = 1 << 6,
  GDK_WINDOW_STATE_FOCUSED          = 1 << 7,
  GDK_WINDOW_STATE_TILED            = 1 << 8,
  GDK_WINDOW_STATE_TOP_TILED        = 1 << 9,
  GDK_WINDOW_STATE_TOP_RESIZABLE    = 1 << 10,
  GDK_WINDOW_STATE_RIGHT_TILED      = 1 << 11,
  GDK_WINDOW_STATE_RIGHT_RESIZABLE  = 1 << 12,
  GDK_WINDOW_STATE_BOTTOM_TILED     = 1 << 13,
  GDK_WINDOW_STATE_BOTTOM_RESIZABLE = 1 << 14,
  GDK_WINDOW_STATE_LEFT_TILED       = 1 << 15,
  GDK_WINDOW_STATE_LEFT_RESIZABLE   = 1 << 16
} GdkWindowState;

/**
 * GdkSettingAction:
 * @GDK_SETTING_ACTION_NEW: a setting was added.
 * @GDK_SETTING_ACTION_CHANGED: a setting was changed.
 * @GDK_SETTING_ACTION_DELETED: a setting was deleted.
 *
 * Specifies the kind of modification applied to a setting in a
 * #GdkEventSetting.
 */
typedef enum
{
  GDK_SETTING_ACTION_NEW,
  GDK_SETTING_ACTION_CHANGED,
  GDK_SETTING_ACTION_DELETED
} GdkSettingAction;

/**
 * GdkOwnerChange:
 * @GDK_OWNER_CHANGE_NEW_OWNER: some other app claimed the ownership
 * @GDK_OWNER_CHANGE_DESTROY: the window was destroyed
 * @GDK_OWNER_CHANGE_CLOSE: the client was closed
 *
 * Specifies why a selection ownership was changed.
 */
typedef enum
{
  GDK_OWNER_CHANGE_NEW_OWNER,
  GDK_OWNER_CHANGE_DESTROY,
  GDK_OWNER_CHANGE_CLOSE
} GdkOwnerChange;

/**
 * GdkEventAny:
 * @type: the type of the event.
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 *
 * Contains the fields which are common to all event structs.
 * Any event pointer can safely be cast to a pointer to a #GdkEventAny to
 * access these fields.
 */
struct _GdkEventAny
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
};

/**
 * GdkEventExpose:
 * @type: the type of the event (%GDK_EXPOSE or %GDK_DAMAGE).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @area: bounding box of @region.
 * @region: the region that needs to be redrawn.
 * @count: the number of contiguous %GDK_EXPOSE events following this one.
 *   The only use for this is “exposure compression”, i.e. handling all
 *   contiguous %GDK_EXPOSE events in one go, though GDK performs some
 *   exposure compression so this is not normally needed.
 *
 * Generated when all or part of a window becomes visible and needs to be
 * redrawn.
 */
struct _GdkEventExpose
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  GdkRectangle area;
  cairo_region_t *region;
  gint count; /* If non-zero, how many more events follow. */
};

/**
 * GdkEventVisibility:
 * @type: the type of the event (%GDK_VISIBILITY_NOTIFY).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @state: the new visibility state (%GDK_VISIBILITY_FULLY_OBSCURED,
 *   %GDK_VISIBILITY_PARTIAL or %GDK_VISIBILITY_UNOBSCURED).
 *
 * Generated when the window visibility status has changed.
 *
 * Deprecated: 3.12: Modern composited windowing systems with pervasive
 *     transparency make it impossible to track the visibility of a window
 *     reliably, so this event can not be guaranteed to provide useful
 *     information.
 */
struct _GdkEventVisibility
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  GdkVisibilityState state;
};

/**
 * GdkEventMotion:
 * @type: the type of the event.
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @time: the time of the event in milliseconds.
 * @x: the x coordinate of the pointer relative to the window.
 * @y: the y coordinate of the pointer relative to the window.
 * @axes: @x, @y translated to the axes of @device, or %NULL if @device is
 *   the mouse.
 * @state: (type GdkModifierType): a bit-mask representing the state of
 *   the modifier keys (e.g. Control, Shift and Alt) and the pointer
 *   buttons. See #GdkModifierType.
 * @is_hint: set to 1 if this event is just a hint, see the
 *   %GDK_POINTER_MOTION_HINT_MASK value of #GdkEventMask.
 * @device: the master device that the event originated from. Use
 * gdk_event_get_source_device() to get the slave device.
 * @x_root: the x coordinate of the pointer relative to the root of the
 *   screen.
 * @y_root: the y coordinate of the pointer relative to the root of the
 *   screen.
 *
 * Generated when the pointer moves.
 */
struct _GdkEventMotion
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  guint32 time;
  gdouble x;
  gdouble y;
  gdouble *axes;
  guint state;
  gint16 is_hint;
  GdkDevice *device;
  gdouble x_root, y_root;
};

/**
 * GdkEventButton:
 * @type: the type of the event (%GDK_BUTTON_PRESS, %GDK_2BUTTON_PRESS,
 *   %GDK_3BUTTON_PRESS or %GDK_BUTTON_RELEASE).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @time: the time of the event in milliseconds.
 * @x: the x coordinate of the pointer relative to the window.
 * @y: the y coordinate of the pointer relative to the window.
 * @axes: @x, @y translated to the axes of @device, or %NULL if @device is
 *   the mouse.
 * @state: (type GdkModifierType): a bit-mask representing the state of
 *   the modifier keys (e.g. Control, Shift and Alt) and the pointer
 *   buttons. See #GdkModifierType.
 * @button: the button which was pressed or released, numbered from 1 to 5.
 *   Normally button 1 is the left mouse button, 2 is the middle button,
 *   and 3 is the right button. On 2-button mice, the middle button can
 *   often be simulated by pressing both mouse buttons together.
 * @device: the master device that the event originated from. Use
 * gdk_event_get_source_device() to get the slave device.
 * @x_root: the x coordinate of the pointer relative to the root of the
 *   screen.
 * @y_root: the y coordinate of the pointer relative to the root of the
 *   screen.
 *
 * Used for button press and button release events. The
 * @type field will be one of %GDK_BUTTON_PRESS,
 * %GDK_2BUTTON_PRESS, %GDK_3BUTTON_PRESS or %GDK_BUTTON_RELEASE,
 *
 * Double and triple-clicks result in a sequence of events being received.
 * For double-clicks the order of events will be:
 *
 * - %GDK_BUTTON_PRESS
 * - %GDK_BUTTON_RELEASE
 * - %GDK_BUTTON_PRESS
 * - %GDK_2BUTTON_PRESS
 * - %GDK_BUTTON_RELEASE
 *
 * Note that the first click is received just like a normal
 * button press, while the second click results in a %GDK_2BUTTON_PRESS
 * being received just after the %GDK_BUTTON_PRESS.
 *
 * Triple-clicks are very similar to double-clicks, except that
 * %GDK_3BUTTON_PRESS is inserted after the third click. The order of the
 * events is:
 *
 * - %GDK_BUTTON_PRESS
 * - %GDK_BUTTON_RELEASE
 * - %GDK_BUTTON_PRESS
 * - %GDK_2BUTTON_PRESS
 * - %GDK_BUTTON_RELEASE
 * - %GDK_BUTTON_PRESS
 * - %GDK_3BUTTON_PRESS
 * - %GDK_BUTTON_RELEASE
 *
 * For a double click to occur, the second button press must occur within
 * 1/4 of a second of the first. For a triple click to occur, the third
 * button press must also occur within 1/2 second of the first button press.
 */
struct _GdkEventButton
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  guint32 time;
  gdouble x;
  gdouble y;
  gdouble *axes;
  guint state;
  guint button;
  GdkDevice *device;
  gdouble x_root, y_root;
};

/**
 * GdkEventTouch:
 * @type: the type of the event (%GDK_TOUCH_BEGIN, %GDK_TOUCH_UPDATE,
 *   %GDK_TOUCH_END, %GDK_TOUCH_CANCEL)
 * @window: the window which received the event
 * @send_event: %TRUE if the event was sent explicitly.
 * @time: the time of the event in milliseconds.
 * @x: the x coordinate of the pointer relative to the window
 * @y: the y coordinate of the pointer relative to the window
 * @axes: @x, @y translated to the axes of @device, or %NULL if @device is
 *   the mouse
 * @state: (type GdkModifierType): a bit-mask representing the state of
 *   the modifier keys (e.g. Control, Shift and Alt) and the pointer
 *   buttons. See #GdkModifierType
 * @sequence: the event sequence that the event belongs to
 * @emulating_pointer: whether the event should be used for emulating
 *   pointer event
 * @device: the master device that the event originated from. Use
 * gdk_event_get_source_device() to get the slave device.
 * @x_root: the x coordinate of the pointer relative to the root of the
 *   screen
 * @y_root: the y coordinate of the pointer relative to the root of the
 *   screen
 *
 * Used for touch events.
 * @type field will be one of %GDK_TOUCH_BEGIN, %GDK_TOUCH_UPDATE,
 * %GDK_TOUCH_END or %GDK_TOUCH_CANCEL.
 *
 * Touch events are grouped into sequences by means of the @sequence
 * field, which can also be obtained with gdk_event_get_event_sequence().
 * Each sequence begins with a %GDK_TOUCH_BEGIN event, followed by
 * any number of %GDK_TOUCH_UPDATE events, and ends with a %GDK_TOUCH_END
 * (or %GDK_TOUCH_CANCEL) event. With multitouch devices, there may be
 * several active sequences at the same time.
 */
struct _GdkEventTouch
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  guint32 time;
  gdouble x;
  gdouble y;
  gdouble *axes;
  guint state;
  GdkEventSequence *sequence;
  gboolean emulating_pointer;
  GdkDevice *device;
  gdouble x_root, y_root;
};

/**
 * GdkEventScroll:
 * @type: the type of the event (%GDK_SCROLL).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @time: the time of the event in milliseconds.
 * @x: the x coordinate of the pointer relative to the window.
 * @y: the y coordinate of the pointer relative to the window.
 * @state: (type GdkModifierType): a bit-mask representing the state of
 *   the modifier keys (e.g. Control, Shift and Alt) and the pointer
 *   buttons. See #GdkModifierType.
 * @direction: the direction to scroll to (one of %GDK_SCROLL_UP,
 *   %GDK_SCROLL_DOWN, %GDK_SCROLL_LEFT, %GDK_SCROLL_RIGHT or
 *   %GDK_SCROLL_SMOOTH).
 * @device: the master device that the event originated from. Use
 * gdk_event_get_source_device() to get the slave device.
 * @x_root: the x coordinate of the pointer relative to the root of the
 *   screen.
 * @y_root: the y coordinate of the pointer relative to the root of the
 *   screen.
 * @delta_x: the x coordinate of the scroll delta
 * @delta_y: the y coordinate of the scroll delta
 *
 * Generated from button presses for the buttons 4 to 7. Wheel mice are
 * usually configured to generate button press events for buttons 4 and 5
 * when the wheel is turned.
 *
 * Some GDK backends can also generate “smooth” scroll events, which
 * can be recognized by the %GDK_SCROLL_SMOOTH scroll direction. For
 * these, the scroll deltas can be obtained with
 * gdk_event_get_scroll_deltas().
 */
struct _GdkEventScroll
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  guint32 time;
  gdouble x;
  gdouble y;
  guint state;
  GdkScrollDirection direction;
  GdkDevice *device;
  gdouble x_root, y_root;
  gdouble delta_x;
  gdouble delta_y;
  guint is_stop : 1;
};

/**
 * GdkEventKey:
 * @type: the type of the event (%GDK_KEY_PRESS or %GDK_KEY_RELEASE).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @time: the time of the event in milliseconds.
 * @state: (type GdkModifierType): a bit-mask representing the state of
 *   the modifier keys (e.g. Control, Shift and Alt) and the pointer
 *   buttons. See #GdkModifierType.
 * @keyval: the key that was pressed or released. See the
 *   `gdk/gdkkeysyms.h` header file for a
 *   complete list of GDK key codes.
 * @length: the length of @string.
 * @string: a string containing an approximation of the text that
 *   would result from this keypress. The only correct way to handle text
 *   input of text is using input methods (see #GtkIMContext), so this
 *   field is deprecated and should never be used.
 *   (gdk_unicode_to_keyval() provides a non-deprecated way of getting
 *   an approximate translation for a key.) The string is encoded in the
 *   encoding of the current locale (Note: this for backwards compatibility:
 *   strings in GTK+ and GDK are typically in UTF-8.) and NUL-terminated.
 *   In some cases, the translation of the key code will be a single
 *   NUL byte, in which case looking at @length is necessary to distinguish
 *   it from the an empty translation.
 * @hardware_keycode: the raw code of the key that was pressed or released.
 * @group: the keyboard group.
 * @is_modifier: a flag that indicates if @hardware_keycode is mapped to a
 *   modifier. Since 2.10
 *
 * Describes a key press or key release event.
 */
struct _GdkEventKey
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  guint32 time;
  guint state;
  guint keyval;
  gint length;
  gchar *string;
  guint16 hardware_keycode;
  guint8 group;
  guint is_modifier : 1;
};

/**
 * GdkEventCrossing:
 * @type: the type of the event (%GDK_ENTER_NOTIFY or %GDK_LEAVE_NOTIFY).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @subwindow: the window that was entered or left.
 * @time: the time of the event in milliseconds.
 * @x: the x coordinate of the pointer relative to the window.
 * @y: the y coordinate of the pointer relative to the window.
 * @x_root: the x coordinate of the pointer relative to the root of the screen.
 * @y_root: the y coordinate of the pointer relative to the root of the screen.
 * @mode: the crossing mode (%GDK_CROSSING_NORMAL, %GDK_CROSSING_GRAB,
 *  %GDK_CROSSING_UNGRAB, %GDK_CROSSING_GTK_GRAB, %GDK_CROSSING_GTK_UNGRAB or
 *  %GDK_CROSSING_STATE_CHANGED).  %GDK_CROSSING_GTK_GRAB, %GDK_CROSSING_GTK_UNGRAB,
 *  and %GDK_CROSSING_STATE_CHANGED were added in 2.14 and are always synthesized,
 *  never native.
 * @detail: the kind of crossing that happened (%GDK_NOTIFY_INFERIOR,
 *  %GDK_NOTIFY_ANCESTOR, %GDK_NOTIFY_VIRTUAL, %GDK_NOTIFY_NONLINEAR or
 *  %GDK_NOTIFY_NONLINEAR_VIRTUAL).
 * @focus: %TRUE if @window is the focus window or an inferior.
 * @state: (type GdkModifierType): a bit-mask representing the state of
 *   the modifier keys (e.g. Control, Shift and Alt) and the pointer
 *   buttons. See #GdkModifierType.
 *
 * Generated when the pointer enters or leaves a window.
 */
struct _GdkEventCrossing
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  GdkWindow *subwindow;
  guint32 time;
  gdouble x;
  gdouble y;
  gdouble x_root;
  gdouble y_root;
  GdkCrossingMode mode;
  GdkNotifyType detail;
  gboolean focus;
  guint state;
};

/**
 * GdkEventFocus:
 * @type: the type of the event (%GDK_FOCUS_CHANGE).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @in: %TRUE if the window has gained the keyboard focus, %FALSE if
 *   it has lost the focus.
 *
 * Describes a change of keyboard focus.
 */
struct _GdkEventFocus
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  gint16 in;
};

/**
 * GdkEventConfigure:
 * @type: the type of the event (%GDK_CONFIGURE).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @x: the new x coordinate of the window, relative to its parent.
 * @y: the new y coordinate of the window, relative to its parent.
 * @width: the new width of the window.
 * @height: the new height of the window.
 *
 * Generated when a window size or position has changed.
 */
struct _GdkEventConfigure
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  gint x, y;
  gint width;
  gint height;
};

/**
 * GdkEventProperty:
 * @type: the type of the event (%GDK_PROPERTY_NOTIFY).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @atom: the property that was changed.
 * @time: the time of the event in milliseconds.
 * @state: (type GdkPropertyState): whether the property was changed
 *   (%GDK_PROPERTY_NEW_VALUE) or deleted (%GDK_PROPERTY_DELETE).
 *
 * Describes a property change on a window.
 */
struct _GdkEventProperty
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  GdkAtom atom;
  guint32 time;
  guint state;
};

/**
 * GdkEventSelection:
 * @type: the type of the event (%GDK_SELECTION_CLEAR,
 *   %GDK_SELECTION_NOTIFY or %GDK_SELECTION_REQUEST).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @selection: the selection.
 * @target: the target to which the selection should be converted.
 * @property: the property in which to place the result of the conversion.
 * @time: the time of the event in milliseconds.
 * @requestor: the window on which to place @property or %NULL if none.
 *
 * Generated when a selection is requested or ownership of a selection
 * is taken over by another client application.
 */
struct _GdkEventSelection
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  GdkAtom selection;
  GdkAtom target;
  GdkAtom property;
  guint32 time;
  GdkWindow *requestor;
};

/**
 * GdkEventOwnerChange:
 * @type: the type of the event (%GDK_OWNER_CHANGE).
 * @window: the window which received the event
 * @send_event: %TRUE if the event was sent explicitly.
 * @owner: the new owner of the selection, or %NULL if there is none
 * @reason: the reason for the ownership change as a #GdkOwnerChange value
 * @selection: the atom identifying the selection
 * @time: the timestamp of the event
 * @selection_time: the time at which the selection ownership was taken
 *   over
 *
 * Generated when the owner of a selection changes. On X11, this
 * information is only available if the X server supports the XFIXES
 * extension.
 *
 * Since: 2.6
 */
struct _GdkEventOwnerChange
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  GdkWindow *owner;
  GdkOwnerChange reason;
  GdkAtom selection;
  guint32 time;
  guint32 selection_time;
};

/**
 * GdkEventProximity:
 * @type: the type of the event (%GDK_PROXIMITY_IN or %GDK_PROXIMITY_OUT).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @time: the time of the event in milliseconds.
 * @device: the master device that the event originated from. Use
 * gdk_event_get_source_device() to get the slave device.
 *
 * Proximity events are generated when using GDK’s wrapper for the
 * XInput extension. The XInput extension is an add-on for standard X
 * that allows you to use nonstandard devices such as graphics tablets.
 * A proximity event indicates that the stylus has moved in or out of
 * contact with the tablet, or perhaps that the user’s finger has moved
 * in or out of contact with a touch screen.
 *
 * This event type will be used pretty rarely. It only is important for
 * XInput aware programs that are drawing their own cursor.
 */
struct _GdkEventProximity
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  guint32 time;
  GdkDevice *device;
};

/**
 * GdkEventSetting:
 * @type: the type of the event (%GDK_SETTING).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @action: what happened to the setting (%GDK_SETTING_ACTION_NEW,
 *   %GDK_SETTING_ACTION_CHANGED or %GDK_SETTING_ACTION_DELETED).
 * @name: the name of the setting.
 *
 * Generated when a setting is modified.
 */
struct _GdkEventSetting
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  GdkSettingAction action;
  char *name;
};

/**
 * GdkEventWindowState:
 * @type: the type of the event (%GDK_WINDOW_STATE).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @changed_mask: mask specifying what flags have changed.
 * @new_window_state: the new window state, a combination of
 *   #GdkWindowState bits.
 *
 * Generated when the state of a toplevel window changes.
 */
struct _GdkEventWindowState
{
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  GdkWindowState changed_mask;
  GdkWindowState new_window_state;
};

/**
 * GdkEventGrabBroken:
 * @type: the type of the event (%GDK_GRAB_BROKEN)
 * @window: the window which received the event, i.e. the window
 *   that previously owned the grab
 * @send_event: %TRUE if the event was sent explicitly.
 * @keyboard: %TRUE if a keyboard grab was broken, %FALSE if a pointer
 *   grab was broken
 * @implicit: %TRUE if the broken grab was implicit
 * @grab_window: If this event is caused by another grab in the same
 *   application, @grab_window contains the new grab window. Otherwise
 *   @grab_window is %NULL.
 *
 * Generated when a pointer or keyboard grab is broken. On X11, this happens
 * when the grab window becomes unviewable (i.e. it or one of its ancestors
 * is unmapped), or if the same application grabs the pointer or keyboard
 * again. Note that implicit grabs (which are initiated by button presses)
 * can also cause #GdkEventGrabBroken events.
 *
 * Since: 2.8
 */
struct _GdkEventGrabBroken {
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  gboolean keyboard;
  gboolean implicit;
  GdkWindow *grab_window;
};

/**
 * GdkEventDND:
 * @type: the type of the event (%GDK_DRAG_ENTER, %GDK_DRAG_LEAVE,
 *   %GDK_DRAG_MOTION, %GDK_DRAG_STATUS, %GDK_DROP_START or
 *   %GDK_DROP_FINISHED).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @context: the #GdkDragContext for the current DND operation.
 * @time: the time of the event in milliseconds.
 * @x_root: the x coordinate of the pointer relative to the root of the
 *   screen, only set for %GDK_DRAG_MOTION and %GDK_DROP_START.
 * @y_root: the y coordinate of the pointer relative to the root of the
 *   screen, only set for %GDK_DRAG_MOTION and %GDK_DROP_START.
 *
 * Generated during DND operations.
 */
struct _GdkEventDND {
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  GdkDragContext *context;

  guint32 time;
  gshort x_root, y_root;
};

/**
 * GdkEventTouchpadSwipe:
 * @type: the type of the event (%GDK_TOUCHPAD_SWIPE)
 * @window: the window which received the event
 * @send_event: %TRUE if the event was sent explicitly
 * @phase: the current phase of the gesture
 * @n_fingers: The number of fingers triggering the swipe
 * @time: the time of the event in milliseconds
 * @x: The X coordinate of the pointer
 * @y: The Y coordinate of the pointer
 * @dx: Movement delta in the X axis of the swipe focal point
 * @dy: Movement delta in the Y axis of the swipe focal point
 * @x_root: The X coordinate of the pointer, relative to the
 *   root of the screen.
 * @y_root: The Y coordinate of the pointer, relative to the
 *   root of the screen.
 * @state: (type GdkModifierType): a bit-mask representing the state of
 *   the modifier keys (e.g. Control, Shift and Alt) and the pointer
 *   buttons. See #GdkModifierType.
 *
 * Generated during touchpad swipe gestures.
 */
struct _GdkEventTouchpadSwipe {
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  gint8 phase;
  gint8 n_fingers;
  guint32 time;
  gdouble x;
  gdouble y;
  gdouble dx;
  gdouble dy;
  gdouble x_root, y_root;
  guint state;
};

/**
 * GdkEventTouchpadPinch:
 * @type: the type of the event (%GDK_TOUCHPAD_PINCH)
 * @window: the window which received the event
 * @send_event: %TRUE if the event was sent explicitly
 * @phase: the current phase of the gesture
 * @n_fingers: The number of fingers triggering the pinch
 * @time: the time of the event in milliseconds
 * @x: The X coordinate of the pointer
 * @y: The Y coordinate of the pointer
 * @dx: Movement delta in the X axis of the swipe focal point
 * @dy: Movement delta in the Y axis of the swipe focal point
 * @angle_delta: The angle change in radians, negative angles
 *   denote counter-clockwise movements
 * @scale: The current scale, relative to that at the time of
 *   the corresponding %GDK_TOUCHPAD_GESTURE_PHASE_BEGIN event
 * @x_root: The X coordinate of the pointer, relative to the
 *   root of the screen.
 * @y_root: The Y coordinate of the pointer, relative to the
 *   root of the screen.
 * @state: (type GdkModifierType): a bit-mask representing the state of
 *   the modifier keys (e.g. Control, Shift and Alt) and the pointer
 *   buttons. See #GdkModifierType.
 *
 * Generated during touchpad swipe gestures.
 */
struct _GdkEventTouchpadPinch {
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  gint8 phase;
  gint8 n_fingers;
  guint32 time;
  gdouble x;
  gdouble y;
  gdouble dx;
  gdouble dy;
  gdouble angle_delta;
  gdouble scale;
  gdouble x_root, y_root;
  guint state;
};

/**
 * GdkEventPadButton:
 * @type: the type of the event (%GDK_PAD_BUTTON_PRESS or %GDK_PAD_BUTTON_RELEASE).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @time: the time of the event in milliseconds.
 * @group: the pad group the button belongs to. A %GDK_SOURCE_TABLET_PAD device
 *   may have one or more groups containing a set of buttons/rings/strips each.
 * @button: The pad button that was pressed.
 * @mode: The current mode of @group. Different groups in a %GDK_SOURCE_TABLET_PAD
 *   device may have different current modes.
 *
 * Generated during %GDK_SOURCE_TABLET_PAD button presses and releases.
 *
 * Since: 3.22
 */
struct _GdkEventPadButton {
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  guint32 time;
  guint group;
  guint button;
  guint mode;
};

/**
 * GdkEventPadAxis:
 * @type: the type of the event (%GDK_PAD_RING or %GDK_PAD_STRIP).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @time: the time of the event in milliseconds.
 * @group: the pad group the ring/strip belongs to. A %GDK_SOURCE_TABLET_PAD
 *   device may have one or more groups containing a set of buttons/rings/strips
 *   each.
 * @index: number of strip/ring that was interacted. This number is 0-indexed.
 * @mode: The current mode of @group. Different groups in a %GDK_SOURCE_TABLET_PAD
 *   device may have different current modes.
 * @value: The current value for the given axis.
 *
 * Generated during %GDK_SOURCE_TABLET_PAD interaction with tactile sensors.
 *
 * Since: 3.22
 */
struct _GdkEventPadAxis {
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  guint32 time;
  guint group;
  guint index;
  guint mode;
  gdouble value;
};

/**
 * GdkEventPadGroupMode:
 * @type: the type of the event (%GDK_PAD_GROUP_MODE).
 * @window: the window which received the event.
 * @send_event: %TRUE if the event was sent explicitly.
 * @time: the time of the event in milliseconds.
 * @group: the pad group that is switching mode. A %GDK_SOURCE_TABLET_PAD
 *   device may have one or more groups containing a set of buttons/rings/strips
 *   each.
 * @mode: The new mode of @group. Different groups in a %GDK_SOURCE_TABLET_PAD
 *   device may have different current modes.
 *
 * Generated during %GDK_SOURCE_TABLET_PAD mode switches in a group.
 *
 * Since: 3.22
 */
struct _GdkEventPadGroupMode {
  GdkEventType type;
  GdkWindow *window;
  gint8 send_event;
  guint32 time;
  guint group;
  guint mode;
};

/**
 * GdkEvent:
 * @type: the #GdkEventType
 * @any: a #GdkEventAny
 * @expose: a #GdkEventExpose
 * @visibility: a #GdkEventVisibility
 * @motion: a #GdkEventMotion
 * @button: a #GdkEventButton
 * @touch: a #GdkEventTouch
 * @scroll: a #GdkEventScroll
 * @key: a #GdkEventKey
 * @crossing: a #GdkEventCrossing
 * @focus_change: a #GdkEventFocus
 * @configure: a #GdkEventConfigure
 * @property: a #GdkEventProperty
 * @selection: a #GdkEventSelection
 * @owner_change: a #GdkEventOwnerChange
 * @proximity: a #GdkEventProximity
 * @dnd: a #GdkEventDND
 * @window_state: a #GdkEventWindowState
 * @setting: a #GdkEventSetting
 * @grab_broken: a #GdkEventGrabBroken
 * @touchpad_swipe: a #GdkEventTouchpadSwipe
 * @touchpad_pinch: a #GdkEventTouchpadPinch
 * @pad_button: a #GdkEventPadButton
 * @pad_axis: a #GdkEventPadAxis
 * @pad_group_mode: a #GdkEventPadGroupMode
 *
 * A #GdkEvent contains a union of all of the event types,
 * and allows access to the data fields in a number of ways.
 *
 * The event type is always the first field in all of the event types, and
 * can always be accessed with the following code, no matter what type of
 * event it is:
 * |[<!-- language="C" -->
 *   GdkEvent *event;
 *   GdkEventType type;
 *
 *   type = event->type;
 * ]|
 *
 * To access other fields of the event, the pointer to the event
 * can be cast to the appropriate event type, or the union member
 * name can be used. For example if the event type is %GDK_BUTTON_PRESS
 * then the x coordinate of the button press can be accessed with:
 * |[<!-- language="C" -->
 *   GdkEvent *event;
 *   gdouble x;
 *
 *   x = ((GdkEventButton*)event)->x;
 * ]|
 * or:
 * |[<!-- language="C" -->
 *   GdkEvent *event;
 *   gdouble x;
 *
 *   x = event->button.x;
 * ]|
 */
union _GdkEvent
{
  GdkEventType		    type;
  GdkEventAny		    any;
  GdkEventExpose	    expose;
  GdkEventVisibility	    visibility;
  GdkEventMotion	    motion;
  GdkEventButton	    button;
  GdkEventTouch             touch;
  GdkEventScroll            scroll;
  GdkEventKey		    key;
  GdkEventCrossing	    crossing;
  GdkEventFocus		    focus_change;
  GdkEventConfigure	    configure;
  GdkEventProperty	    property;
  GdkEventSelection	    selection;
  GdkEventOwnerChange  	    owner_change;
  GdkEventProximity	    proximity;
  GdkEventDND               dnd;
  GdkEventWindowState       window_state;
  GdkEventSetting           setting;
  GdkEventGrabBroken        grab_broken;
  GdkEventTouchpadSwipe     touchpad_swipe;
  GdkEventTouchpadPinch     touchpad_pinch;
  GdkEventPadButton         pad_button;
  GdkEventPadAxis           pad_axis;
  GdkEventPadGroupMode      pad_group_mode;
};

GDK_AVAILABLE_IN_ALL
GType     gdk_event_get_type            (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_14
GType     gdk_event_sequence_get_type   (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
gboolean  gdk_events_pending	 	(void);
GDK_AVAILABLE_IN_ALL
GdkEvent* gdk_event_get			(void);

GDK_AVAILABLE_IN_ALL
GdkEvent* gdk_event_peek                (void);
GDK_AVAILABLE_IN_ALL
void      gdk_event_put	 		(const GdkEvent *event);

GDK_AVAILABLE_IN_ALL
GdkEvent* gdk_event_new                 (GdkEventType    type);
GDK_AVAILABLE_IN_ALL
GdkEvent* gdk_event_copy     		(const GdkEvent *event);
GDK_AVAILABLE_IN_ALL
void	  gdk_event_free     		(GdkEvent 	*event);

GDK_AVAILABLE_IN_3_10
GdkWindow *gdk_event_get_window         (const GdkEvent *event);

GDK_AVAILABLE_IN_ALL
guint32   gdk_event_get_time            (const GdkEvent  *event);
GDK_AVAILABLE_IN_ALL
gboolean  gdk_event_get_state           (const GdkEvent  *event,
                                         GdkModifierType *state);
GDK_AVAILABLE_IN_ALL
gboolean  gdk_event_get_coords		(const GdkEvent  *event,
					 gdouble	 *x_win,
					 gdouble	 *y_win);
GDK_AVAILABLE_IN_ALL
gboolean  gdk_event_get_root_coords	(const GdkEvent *event,
					 gdouble	*x_root,
					 gdouble	*y_root);
GDK_AVAILABLE_IN_3_2
gboolean  gdk_event_get_button          (const GdkEvent *event,
                                         guint          *button);
GDK_AVAILABLE_IN_3_2
gboolean  gdk_event_get_click_count     (const GdkEvent *event,
                                         guint          *click_count);
GDK_AVAILABLE_IN_3_2
gboolean  gdk_event_get_keyval          (const GdkEvent *event,
                                         guint          *keyval);
GDK_AVAILABLE_IN_3_2
gboolean  gdk_event_get_keycode         (const GdkEvent *event,
                                         guint16        *keycode);
GDK_AVAILABLE_IN_3_2
gboolean gdk_event_get_scroll_direction (const GdkEvent *event,
                                         GdkScrollDirection *direction);
GDK_AVAILABLE_IN_3_4
gboolean  gdk_event_get_scroll_deltas   (const GdkEvent *event,
                                         gdouble         *delta_x,
                                         gdouble         *delta_y);

GDK_AVAILABLE_IN_3_20
gboolean  gdk_event_is_scroll_stop_event (const GdkEvent *event);

GDK_AVAILABLE_IN_ALL
gboolean  gdk_event_get_axis            (const GdkEvent  *event,
                                         GdkAxisUse       axis_use,
                                         gdouble         *value);
GDK_AVAILABLE_IN_ALL
void       gdk_event_set_device         (GdkEvent        *event,
                                         GdkDevice       *device);
GDK_AVAILABLE_IN_ALL
GdkDevice* gdk_event_get_device         (const GdkEvent  *event);
GDK_AVAILABLE_IN_ALL
void       gdk_event_set_source_device  (GdkEvent        *event,
                                         GdkDevice       *device);
GDK_AVAILABLE_IN_ALL
GdkDevice* gdk_event_get_source_device  (const GdkEvent  *event);
GDK_AVAILABLE_IN_ALL
void       gdk_event_request_motions    (const GdkEventMotion *event);
GDK_AVAILABLE_IN_3_4
gboolean   gdk_event_triggers_context_menu (const GdkEvent *event);

GDK_AVAILABLE_IN_ALL
gboolean  gdk_events_get_distance       (GdkEvent        *event1,
                                         GdkEvent        *event2,
                                         gdouble         *distance);
GDK_AVAILABLE_IN_ALL
gboolean  gdk_events_get_angle          (GdkEvent        *event1,
                                         GdkEvent        *event2,
                                         gdouble         *angle);
GDK_AVAILABLE_IN_ALL
gboolean  gdk_events_get_center         (GdkEvent        *event1,
                                         GdkEvent        *event2,
                                         gdouble         *x,
                                         gdouble         *y);

GDK_AVAILABLE_IN_ALL
void	  gdk_event_handler_set 	(GdkEventFunc    func,
					 gpointer        data,
					 GDestroyNotify  notify);

GDK_AVAILABLE_IN_ALL
void       gdk_event_set_screen         (GdkEvent        *event,
                                         GdkScreen       *screen);
GDK_AVAILABLE_IN_ALL
GdkScreen *gdk_event_get_screen         (const GdkEvent  *event);

GDK_AVAILABLE_IN_3_4
GdkEventSequence *gdk_event_get_event_sequence (const GdkEvent *event);

GDK_AVAILABLE_IN_3_10
GdkEventType gdk_event_get_event_type   (const GdkEvent *event);

GDK_AVAILABLE_IN_3_20
GdkSeat  *gdk_event_get_seat            (const GdkEvent *event);

GDK_AVAILABLE_IN_ALL
void	  gdk_set_show_events		(gboolean	 show_events);
GDK_AVAILABLE_IN_ALL
gboolean  gdk_get_show_events		(void);

GDK_AVAILABLE_IN_ALL
gboolean gdk_setting_get                (const gchar    *name,
                                         GValue         *value);

GDK_AVAILABLE_IN_3_22
GdkDeviceTool *gdk_event_get_device_tool (const GdkEvent *event);

GDK_AVAILABLE_IN_3_22
void           gdk_event_set_device_tool (GdkEvent       *event,
                                          GdkDeviceTool  *tool);

GDK_AVAILABLE_IN_3_22
int            gdk_event_get_scancode    (GdkEvent *event);

GDK_AVAILABLE_IN_3_22
gboolean       gdk_event_get_pointer_emulated (GdkEvent *event);

G_END_DECLS

#endif /* __GDK_EVENTS_H__ */
