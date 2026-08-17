/*
 * gdkdisplay.h
 * 
 * Copyright 2001 Sun Microsystems Inc. 
 *
 * Erwann Chenede <erwann.chenede@sun.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GDK_DISPLAY_H__
#define __GDK_DISPLAY_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>
#include <gdk/gdkevents.h>

G_BEGIN_DECLS

typedef struct _GdkDisplayClass GdkDisplayClass;
typedef struct _GdkDisplayPointerHooks GdkDisplayPointerHooks;

#define GDK_TYPE_DISPLAY              (gdk_display_get_type ())
#define GDK_DISPLAY_OBJECT(object)    (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_DISPLAY, GdkDisplay))
#define GDK_DISPLAY_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_DISPLAY, GdkDisplayClass))
#define GDK_IS_DISPLAY(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_DISPLAY))
#define GDK_IS_DISPLAY_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_DISPLAY))
#define GDK_DISPLAY_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_DISPLAY, GdkDisplayClass))

/* Tracks information about the keyboard grab on this display */
typedef struct
{
  GdkWindow *window;
  GdkWindow *native_window;
  gulong serial;
  gboolean owner_events;
  guint32 time;
} GdkKeyboardGrabInfo;

/* Tracks information about which window and position the pointer last was in.
 * This is useful when we need to synthesize events later.
 * Note that we track toplevel_under_pointer using enter/leave events,
 * so in the case of a grab, either with owner_events==FALSE or with the
 * pointer in no clients window the x/y coordinates may actually be outside
 * the window.
 */
typedef struct
{
  GdkWindow *toplevel_under_pointer; /* The toplevel window with mouse inside, tracked via native events */
  GdkWindow *window_under_pointer; /* The window that last got sent a normal enter event */
  gdouble toplevel_x, toplevel_y; 
  guint32 state;
  guint32 button;
  gulong motion_hint_serial; /* 0 == didn't deliver hinted motion event */
} GdkPointerWindowInfo;

struct _GdkDisplay
{
  GObject parent_instance;

  /*< private >*/
  GList *GSEAL (queued_events);
  GList *GSEAL (queued_tail);

  /* Information for determining if the latest button click
   * is part of a double-click or triple-click
   */
  guint32 GSEAL (button_click_time[2]);	/* The last 2 button click times. */
  GdkWindow *GSEAL (button_window[2]);  /* The last 2 windows to receive button presses. */
  gint GSEAL (button_number[2]);        /* The last 2 buttons to be pressed. */

  guint GSEAL (double_click_time);	/* Maximum time between clicks in msecs */
  GdkDevice *GSEAL (core_pointer);	/* Core pointer device */

  const GdkDisplayPointerHooks *GSEAL (pointer_hooks); /* Current hooks for querying pointer */
  
  guint GSEAL (closed) : 1;		/* Whether this display has been closed */
  guint GSEAL (ignore_core_events) : 1; /* Don't send core motion and button event */

  guint GSEAL (double_click_distance);	/* Maximum distance between clicks in pixels */
  gint GSEAL (button_x[2]);             /* The last 2 button click positions. */
  gint GSEAL (button_y[2]);

  GList *GSEAL (pointer_grabs);
  GdkKeyboardGrabInfo GSEAL (keyboard_grab);
  GdkPointerWindowInfo GSEAL (pointer_info);

  /* Last reported event time from server */
  guint32 GSEAL (last_event_time);
};

struct _GdkDisplayClass
{
  GObjectClass parent_class;
  
  const gchar *              (*get_display_name)   (GdkDisplay *display);
  gint			     (*get_n_screens)      (GdkDisplay *display);
  GdkScreen *		     (*get_screen)         (GdkDisplay *display,
						    gint        screen_num);
  GdkScreen *		     (*get_default_screen) (GdkDisplay *display);

  
  /* Signals */
  void (*closed) (GdkDisplay *display,
		  gboolean    is_error);
};

struct _GdkDisplayPointerHooks
{
  void (*get_pointer)              (GdkDisplay      *display,
				    GdkScreen      **screen,
				    gint            *x,
				    gint            *y,
				    GdkModifierType *mask);
  GdkWindow* (*window_get_pointer) (GdkDisplay      *display,
				    GdkWindow       *window,
				    gint            *x,
				    gint            *y,
				    GdkModifierType *mask);
  GdkWindow* (*window_at_pointer)  (GdkDisplay      *display,
				    gint            *win_x,
				    gint            *win_y);
};

GType       gdk_display_get_type (void) G_GNUC_CONST;
GdkDisplay *gdk_display_open                (const gchar *display_name);

const gchar * gdk_display_get_name         (GdkDisplay *display);

gint        gdk_display_get_n_screens      (GdkDisplay  *display);
GdkScreen * gdk_display_get_screen         (GdkDisplay  *display,
					    gint         screen_num);
GdkScreen * gdk_display_get_default_screen (GdkDisplay  *display);
void        gdk_display_pointer_ungrab     (GdkDisplay  *display,
					    guint32      time_);
void        gdk_display_keyboard_ungrab    (GdkDisplay  *display,
					    guint32      time_);
gboolean    gdk_display_pointer_is_grabbed (GdkDisplay  *display);
void        gdk_display_beep               (GdkDisplay  *display);
void        gdk_display_sync               (GdkDisplay  *display);
void        gdk_display_flush              (GdkDisplay  *display);

void	    gdk_display_close		       (GdkDisplay  *display);
gboolean    gdk_display_is_closed          (GdkDisplay  *display);

GList *     gdk_display_list_devices       (GdkDisplay  *display);

GdkEvent* gdk_display_get_event  (GdkDisplay     *display);
GdkEvent* gdk_display_peek_event (GdkDisplay     *display);
void      gdk_display_put_event  (GdkDisplay     *display,
				  const GdkEvent *event);

void gdk_display_add_client_message_filter (GdkDisplay   *display,
					    GdkAtom       message_type,
					    GdkFilterFunc func,
					    gpointer      data);

void gdk_display_set_double_click_time     (GdkDisplay   *display,
					    guint         msec);
void gdk_display_set_double_click_distance (GdkDisplay   *display,
					    guint         distance);

GdkDisplay *gdk_display_get_default (void);

GdkDevice  *gdk_display_get_core_pointer (GdkDisplay *display);

void             gdk_display_get_pointer           (GdkDisplay             *display,
						    GdkScreen             **screen,
						    gint                   *x,
						    gint                   *y,
						    GdkModifierType        *mask);
GdkWindow *      gdk_display_get_window_at_pointer (GdkDisplay             *display,
						    gint                   *win_x,
						    gint                   *win_y);
void             gdk_display_warp_pointer          (GdkDisplay             *display,
						    GdkScreen              *screen,
						    gint                   x,
						    gint                   y);

#ifndef GDK_DISABLE_DEPRECATED
GdkDisplayPointerHooks *gdk_display_set_pointer_hooks (GdkDisplay                   *display,
						       const GdkDisplayPointerHooks *new_hooks);
#endif

GdkDisplay *gdk_display_open_default_libgtk_only (void);

gboolean gdk_display_supports_cursor_alpha     (GdkDisplay    *display);
gboolean gdk_display_supports_cursor_color     (GdkDisplay    *display);
guint    gdk_display_get_default_cursor_size   (GdkDisplay    *display);
void     gdk_display_get_maximal_cursor_size   (GdkDisplay    *display,
						guint         *width,
						guint         *height);

GdkWindow *gdk_display_get_default_group       (GdkDisplay *display); 

gboolean gdk_display_supports_selection_notification (GdkDisplay *display);
gboolean gdk_display_request_selection_notification  (GdkDisplay *display,
						      GdkAtom     selection);

gboolean gdk_display_supports_clipboard_persistence (GdkDisplay    *display);
void     gdk_display_store_clipboard                (GdkDisplay    *display,
						     GdkWindow     *clipboard_window,
						     guint32        time_,
						     const GdkAtom *targets,
						     gint           n_targets);

gboolean gdk_display_supports_shapes           (GdkDisplay    *display);
gboolean gdk_display_supports_input_shapes     (GdkDisplay    *display);
gboolean gdk_display_supports_composite        (GdkDisplay    *display);

G_END_DECLS

#endif	/* __GDK_DISPLAY_H__ */
