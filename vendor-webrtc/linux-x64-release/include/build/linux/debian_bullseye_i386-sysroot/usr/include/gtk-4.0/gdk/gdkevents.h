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

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkdevice.h>
#include <gdk/gdkdevicetool.h>
#include <gdk/gdkdrag.h>
#include <gdk/gdkenums.h>
#include <gdk/gdktypes.h>
#include <gdk/gdkversionmacros.h>

G_BEGIN_DECLS

#define GDK_TYPE_EVENT          (gdk_event_get_type ())
#define GDK_TYPE_EVENT_SEQUENCE (gdk_event_sequence_get_type ())

#define GDK_IS_EVENT(obj)       (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GDK_TYPE_EVENT))
#define GDK_EVENT(obj)          (G_TYPE_CHECK_INSTANCE_CAST ((obj), GDK_TYPE_EVENT, GdkEvent))

#define GDK_IS_EVENT_TYPE(event, type)  (gdk_event_get_event_type ((event)) == (type))

/**
 * GDK_PRIORITY_EVENTS: (value 0)
 *
 * This is the priority that events from the X server are given in the main loop.
 */
#define GDK_PRIORITY_EVENTS	(G_PRIORITY_DEFAULT)

/**
 * GDK_PRIORITY_REDRAW: (value 120)
 *
 * This is the priority that the idle handler processing surface updates
 * is given in the main loop.
 */
#define GDK_PRIORITY_REDRAW     (G_PRIORITY_HIGH_IDLE + 20)

/**
 * GDK_EVENT_PROPAGATE:
 *
 * Use this macro as the return value for continuing the propagation of
 * an event handler.
 */
#define GDK_EVENT_PROPAGATE     (FALSE)

/**
 * GDK_EVENT_STOP:
 *
 * Use this macro as the return value for stopping the propagation of
 * an event handler.
 */
#define GDK_EVENT_STOP          (TRUE)

/**
 * GDK_BUTTON_PRIMARY:
 *
 * The primary button. This is typically the left mouse button, or the
 * right button in a left-handed setup.
 */
#define GDK_BUTTON_PRIMARY      (1)

/**
 * GDK_BUTTON_MIDDLE:
 *
 * The middle button.
 */
#define GDK_BUTTON_MIDDLE       (2)

/**
 * GDK_BUTTON_SECONDARY:
 *
 * The secondary button. This is typically the right mouse button, or the
 * left button in a left-handed setup.
 */
#define GDK_BUTTON_SECONDARY    (3)

typedef struct _GdkEventSequence        GdkEventSequence;
typedef struct _GdkEvent                GdkEvent;

#define GDK_TYPE_BUTTON_EVENT (gdk_button_event_get_type())
#define GDK_TYPE_CROSSING_EVENT (gdk_crossing_event_get_type())
#define GDK_TYPE_DELETE_EVENT (gdk_delete_event_get_type())
#define GDK_TYPE_DND_EVENT (gdk_dnd_event_get_type())
#define GDK_TYPE_FOCUS_EVENT (gdk_focus_event_get_type())
#define GDK_TYPE_GRAB_BROKEN_EVENT (gdk_grab_broken_event_get_type())
#define GDK_TYPE_KEY_EVENT (gdk_key_event_get_type())
#define GDK_TYPE_MOTION_EVENT (gdk_motion_event_get_type())
#define GDK_TYPE_PAD_EVENT (gdk_pad_event_get_type())
#define GDK_TYPE_PROXIMITY_EVENT (gdk_proximity_event_get_type())
#define GDK_TYPE_SCROLL_EVENT (gdk_scroll_event_get_type())
#define GDK_TYPE_TOUCH_EVENT (gdk_touch_event_get_type())
#define GDK_TYPE_TOUCHPAD_EVENT (gdk_touchpad_event_get_type())

typedef struct _GdkButtonEvent          GdkButtonEvent;
typedef struct _GdkCrossingEvent        GdkCrossingEvent;
typedef struct _GdkDeleteEvent          GdkDeleteEvent;
typedef struct _GdkDNDEvent             GdkDNDEvent;
typedef struct _GdkFocusEvent           GdkFocusEvent;
typedef struct _GdkGrabBrokenEvent      GdkGrabBrokenEvent;
typedef struct _GdkKeyEvent             GdkKeyEvent;
typedef struct _GdkMotionEvent          GdkMotionEvent;
typedef struct _GdkPadEvent             GdkPadEvent;
typedef struct _GdkProximityEvent       GdkProximityEvent;
typedef struct _GdkScrollEvent          GdkScrollEvent;
typedef struct _GdkTouchEvent           GdkTouchEvent;
typedef struct _GdkTouchpadEvent        GdkTouchpadEvent;

/**
 * GdkEventType:
 * @GDK_DELETE: the window manager has requested that the toplevel surface be
 *   hidden or destroyed, usually when the user clicks on a special icon in the
 *   title bar.
 * @GDK_MOTION_NOTIFY: the pointer (usually a mouse) has moved.
 * @GDK_BUTTON_PRESS: a mouse button has been pressed.
 * @GDK_BUTTON_RELEASE: a mouse button has been released.
 * @GDK_KEY_PRESS: a key has been pressed.
 * @GDK_KEY_RELEASE: a key has been released.
 * @GDK_ENTER_NOTIFY: the pointer has entered the surface.
 * @GDK_LEAVE_NOTIFY: the pointer has left the surface.
 * @GDK_FOCUS_CHANGE: the keyboard focus has entered or left the surface.
 * @GDK_PROXIMITY_IN: an input device has moved into contact with a sensing
 *   surface (e.g. a touchscreen or graphics tablet).
 * @GDK_PROXIMITY_OUT: an input device has moved out of contact with a sensing
 *   surface.
 * @GDK_DRAG_ENTER: the mouse has entered the surface while a drag is in progress.
 * @GDK_DRAG_LEAVE: the mouse has left the surface while a drag is in progress.
 * @GDK_DRAG_MOTION: the mouse has moved in the surface while a drag is in
 *   progress.
 * @GDK_DROP_START: a drop operation onto the surface has started.
 * @GDK_SCROLL: the scroll wheel was turned
 * @GDK_GRAB_BROKEN: a pointer or keyboard grab was broken.
 * @GDK_TOUCH_BEGIN: A new touch event sequence has just started.
 * @GDK_TOUCH_UPDATE: A touch event sequence has been updated.
 * @GDK_TOUCH_END: A touch event sequence has finished.
 * @GDK_TOUCH_CANCEL: A touch event sequence has been canceled.
 * @GDK_TOUCHPAD_SWIPE: A touchpad swipe gesture event, the current state
 *   is determined by its phase field.
 * @GDK_TOUCHPAD_PINCH: A touchpad pinch gesture event, the current state
 *   is determined by its phase field.
 * @GDK_PAD_BUTTON_PRESS: A tablet pad button press event.
 * @GDK_PAD_BUTTON_RELEASE: A tablet pad button release event.
 * @GDK_PAD_RING: A tablet pad axis event from a "ring".
 * @GDK_PAD_STRIP: A tablet pad axis event from a "strip".
 * @GDK_PAD_GROUP_MODE: A tablet pad group mode change.
 * @GDK_TOUCHPAD_HOLD: A touchpad hold gesture event, the current state
 *   is determined by its phase field. Since: 4.6
 * @GDK_EVENT_LAST: marks the end of the GdkEventType enumeration.
 *
 * Specifies the type of the event.
 */
typedef enum
{
  GDK_DELETE,
  GDK_MOTION_NOTIFY,
  GDK_BUTTON_PRESS,
  GDK_BUTTON_RELEASE,
  GDK_KEY_PRESS,
  GDK_KEY_RELEASE,
  GDK_ENTER_NOTIFY,
  GDK_LEAVE_NOTIFY,
  GDK_FOCUS_CHANGE,
  GDK_PROXIMITY_IN,
  GDK_PROXIMITY_OUT,
  GDK_DRAG_ENTER,
  GDK_DRAG_LEAVE,
  GDK_DRAG_MOTION,
  GDK_DROP_START,
  GDK_SCROLL,
  GDK_GRAB_BROKEN,
  GDK_TOUCH_BEGIN,
  GDK_TOUCH_UPDATE,
  GDK_TOUCH_END,
  GDK_TOUCH_CANCEL,
  GDK_TOUCHPAD_SWIPE,
  GDK_TOUCHPAD_PINCH,
  GDK_PAD_BUTTON_PRESS,
  GDK_PAD_BUTTON_RELEASE,
  GDK_PAD_RING,
  GDK_PAD_STRIP,
  GDK_PAD_GROUP_MODE,
  GDK_TOUCHPAD_HOLD,
  GDK_EVENT_LAST        /* helper variable for decls */
} GdkEventType;

/**
 * GdkTouchpadGesturePhase:
 * @GDK_TOUCHPAD_GESTURE_PHASE_BEGIN: The gesture has begun.
 * @GDK_TOUCHPAD_GESTURE_PHASE_UPDATE: The gesture has been updated.
 * @GDK_TOUCHPAD_GESTURE_PHASE_END: The gesture was finished, changes
 *   should be permanently applied.
 * @GDK_TOUCHPAD_GESTURE_PHASE_CANCEL: The gesture was cancelled, all
 *   changes should be undone.
 *
 * Specifies the current state of a touchpad gesture.
 *
 * All gestures are guaranteed to begin with an event with phase
 * %GDK_TOUCHPAD_GESTURE_PHASE_BEGIN, followed by 0 or several events
 * with phase %GDK_TOUCHPAD_GESTURE_PHASE_UPDATE.
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
 * @GDK_SCROLL_UP: the surface is scrolled up.
 * @GDK_SCROLL_DOWN: the surface is scrolled down.
 * @GDK_SCROLL_LEFT: the surface is scrolled to the left.
 * @GDK_SCROLL_RIGHT: the surface is scrolled to the right.
 * @GDK_SCROLL_SMOOTH: the scrolling is determined by the delta values
 *   in scroll events. See gdk_scroll_event_get_deltas()
 *
 * Specifies the direction for scroll events.
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
 * GdkScrollUnit:
 * @GDK_SCROLL_UNIT_WHEEL: The delta is in number of wheel clicks.
 * @GDK_SCROLL_UNIT_SURFACE: The delta is in surface pixels to scroll directly
 *   on screen.
 *
 * Specifies the unit of scroll deltas.
 *
 * When you get %GDK_SCROLL_UNIT_WHEEL, a delta of 1.0 means 1 wheel detent
 * click in the south direction, 2.0 means 2 wheel detent clicks in the south
 * direction... This is the same logic for negative values but in the north
 * direction.
 *
 * If you get %GDK_SCROLL_UNIT_SURFACE, are managing a scrollable view and get a
 * value of 123, you have to scroll 123 surface logical pixels right if it's
 * @delta_x or down if it's @delta_y. This is the same logic for negative values
 * but you have to scroll left instead of right if it's @delta_x and up instead
 * of down if it's @delta_y.
 *
 * 1 surface logical pixel is equal to 1 real screen pixel multiplied by the
 * final scale factor of your graphical interface (the product of the desktop
 * scale factor and eventually a custom scale factor in your app).
 *
 * Since: 4.8
 */
typedef enum
{
  GDK_SCROLL_UNIT_WHEEL,
  GDK_SCROLL_UNIT_SURFACE
} GdkScrollUnit;

/**
 * GdkNotifyType:
 * @GDK_NOTIFY_ANCESTOR: the surface is entered from an ancestor or
 *   left towards an ancestor.
 * @GDK_NOTIFY_VIRTUAL: the pointer moves between an ancestor and an
 *   inferior of the surface.
 * @GDK_NOTIFY_INFERIOR: the surface is entered from an inferior or
 *   left towards an inferior.
 * @GDK_NOTIFY_NONLINEAR: the surface is entered from or left towards
 *   a surface which is neither an ancestor nor an inferior.
 * @GDK_NOTIFY_NONLINEAR_VIRTUAL: the pointer moves between two surfaces
 *   which are not ancestors of each other and the surface is part of
 *   the ancestor chain between one of these surfaces and their least
 *   common ancestor.
 * @GDK_NOTIFY_UNKNOWN: an unknown type of enter/leave event occurred.
 *
 * Specifies the kind of crossing for enter and leave events.
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
 * @GDK_CROSSING_GTK_GRAB: crossing because a GTK grab is activated.
 * @GDK_CROSSING_GTK_UNGRAB: crossing because a GTK grab is deactivated.
 * @GDK_CROSSING_STATE_CHANGED: crossing because a GTK widget changed
 *   state (e.g. sensitivity).
 * @GDK_CROSSING_TOUCH_BEGIN: crossing because a touch sequence has begun,
 *   this event is synthetic as the pointer might have not left the surface.
 * @GDK_CROSSING_TOUCH_END: crossing because a touch sequence has ended,
 *   this event is synthetic as the pointer might have not left the surface.
 * @GDK_CROSSING_DEVICE_SWITCH: crossing because of a device switch (i.e.
 *   a mouse taking control of the pointer after a touch device), this event
 *   is synthetic as the pointer didn’t leave the surface.
 *
 * Specifies the crossing mode for enter and leave events.
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

GDK_AVAILABLE_IN_ALL
GType                   gdk_event_get_type              (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GType                   gdk_event_sequence_get_type     (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GdkEvent *              gdk_event_ref                   (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
void                    gdk_event_unref                 (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
GdkEventType            gdk_event_get_event_type        (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
GdkSurface *            gdk_event_get_surface           (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
GdkSeat *               gdk_event_get_seat              (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
GdkDevice *             gdk_event_get_device            (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
GdkDeviceTool *         gdk_event_get_device_tool       (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
guint32                 gdk_event_get_time              (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
GdkDisplay *            gdk_event_get_display           (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
GdkEventSequence *      gdk_event_get_event_sequence    (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
GdkModifierType         gdk_event_get_modifier_state    (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
gboolean                gdk_event_get_position          (GdkEvent  *event,
					                 double	   *x,
					                 double	   *y);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_event_get_axes              (GdkEvent  *event,
                                                         double   **axes,
                                                         guint     *n_axes);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_event_get_axis              (GdkEvent   *event,
                                                         GdkAxisUse  axis_use,
                                                         double     *value);
GDK_AVAILABLE_IN_ALL
GdkTimeCoord *          gdk_event_get_history           (GdkEvent *event,
                                                         guint    *out_n_coords);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_event_get_pointer_emulated (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
GType                   gdk_button_event_get_type       (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
guint                   gdk_button_event_get_button     (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
GType                   gdk_scroll_event_get_type       (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GdkScrollDirection      gdk_scroll_event_get_direction  (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
void                    gdk_scroll_event_get_deltas     (GdkEvent *event,
                                                         double   *delta_x,
                                                         double   *delta_y);
GDK_AVAILABLE_IN_4_8
GdkScrollUnit           gdk_scroll_event_get_unit       (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
gboolean                gdk_scroll_event_is_stop        (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
GType                   gdk_key_event_get_type          (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
guint                   gdk_key_event_get_keyval        (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
guint                   gdk_key_event_get_keycode       (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
GdkModifierType         gdk_key_event_get_consumed_modifiers (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
guint                   gdk_key_event_get_layout        (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
guint                   gdk_key_event_get_level         (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_key_event_is_modifier       (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
GType                   gdk_focus_event_get_type        (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
gboolean                gdk_focus_event_get_in          (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
GType                   gdk_touch_event_get_type        (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
gboolean                gdk_touch_event_get_emulating_pointer (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
GType                   gdk_crossing_event_get_type     (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GdkCrossingMode         gdk_crossing_event_get_mode     (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
GdkNotifyType           gdk_crossing_event_get_detail   (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_crossing_event_get_focus    (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
GType                   gdk_touchpad_event_get_type     (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GdkTouchpadGesturePhase
                        gdk_touchpad_event_get_gesture_phase (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
guint                   gdk_touchpad_event_get_n_fingers     (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
void                    gdk_touchpad_event_get_deltas        (GdkEvent *event,
                                                              double   *dx,
                                                              double   *dy);
GDK_AVAILABLE_IN_ALL
double                  gdk_touchpad_event_get_pinch_angle_delta (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
double                  gdk_touchpad_event_get_pinch_scale       (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
GType                   gdk_pad_event_get_type          (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
guint                   gdk_pad_event_get_button        (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
void                    gdk_pad_event_get_axis_value    (GdkEvent *event,
                                                         guint    *index,
                                                         double   *value);
GDK_AVAILABLE_IN_ALL
void                    gdk_pad_event_get_group_mode    (GdkEvent *event,
                                                         guint    *group,
                                                         guint    *mode);
GDK_AVAILABLE_IN_ALL
GType                   gdk_dnd_event_get_type          (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GdkDrop *               gdk_dnd_event_get_drop          (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
GType                   gdk_grab_broken_event_get_type  (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GdkSurface *            gdk_grab_broken_event_get_grab_surface (GdkEvent *event);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_grab_broken_event_get_implicit     (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
GType                   gdk_motion_event_get_type       (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GType                   gdk_delete_event_get_type       (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GType                   gdk_proximity_event_get_type    (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
gboolean                gdk_event_triggers_context_menu (GdkEvent *event);

GDK_AVAILABLE_IN_ALL
gboolean                gdk_events_get_distance         (GdkEvent *event1,
                                                         GdkEvent *event2,
                                                         double   *distance);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_events_get_angle            (GdkEvent *event1,
                                                         GdkEvent *event2,
                                                         double   *angle);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_events_get_center           (GdkEvent *event1,
                                                         GdkEvent *event2,
                                                         double   *x,
                                                         double   *y);

/**
 * GdkKeyMatch:
 * @GDK_KEY_MATCH_NONE: The key event does not match
 * @GDK_KEY_MATCH_PARTIAL: The key event matches if keyboard state
 *   (specifically, the currently active group) is ignored
 * @GDK_KEY_MATCH_EXACT: The key event matches
 *
 * Describes how well an event matches a given keyval and modifiers.
 *
 * `GdkKeyMatch` values are returned by [method@Gdk.KeyEvent.matches].
 */
typedef enum {
  GDK_KEY_MATCH_NONE,
  GDK_KEY_MATCH_PARTIAL,
  GDK_KEY_MATCH_EXACT
} GdkKeyMatch;

GDK_AVAILABLE_IN_ALL
GdkKeyMatch            gdk_key_event_matches (GdkEvent        *event,
                                              guint            keyval,
                                              GdkModifierType  modifiers);

GDK_AVAILABLE_IN_ALL
gboolean               gdk_key_event_get_match (GdkEvent        *event,
                                                guint           *keyval,
                                                GdkModifierType *modifiers);


G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkEvent, gdk_event_unref)

G_END_DECLS

#endif /* __GDK_EVENTS_H__ */
