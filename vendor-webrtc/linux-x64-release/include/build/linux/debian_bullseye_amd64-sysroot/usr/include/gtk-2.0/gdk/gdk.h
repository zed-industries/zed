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

#ifndef __GDK_H__
#define __GDK_H__

#define __GDK_H_INSIDE__

#include <gdk/gdkapplaunchcontext.h>
#include <gdk/gdkcairo.h>
#include <gdk/gdkcolor.h>
#include <gdk/gdkcursor.h>
#include <gdk/gdkdisplay.h>
#include <gdk/gdkdisplaymanager.h>
#include <gdk/gdkdnd.h>
#include <gdk/gdkdrawable.h>
#include <gdk/gdkenumtypes.h>
#include <gdk/gdkevents.h>
#include <gdk/gdkfont.h>
#include <gdk/gdkgc.h>
#include <gdk/gdkimage.h>
#include <gdk/gdkinput.h>
#include <gdk/gdkkeys.h>
#include <gdk/gdkpango.h>
#include <gdk/gdkpixbuf.h>
#include <gdk/gdkpixmap.h>
#include <gdk/gdkproperty.h>
#include <gdk/gdkregion.h>
#include <gdk/gdkrgb.h>
#include <gdk/gdkscreen.h>
#include <gdk/gdkselection.h>
#include <gdk/gdkspawn.h>
#include <gdk/gdktestutils.h>
#include <gdk/gdktypes.h>
#include <gdk/gdkvisual.h>
#include <gdk/gdkwindow.h>

#undef __GDK_H_INSIDE__

G_BEGIN_DECLS


/* Initialization, exit and events
 */
#define	  GDK_PRIORITY_EVENTS		(G_PRIORITY_DEFAULT)
void 	  gdk_parse_args	   	(gint	   	*argc,
					 gchar        ***argv);
void 	  gdk_init		   	(gint	   	*argc,
					 gchar        ***argv);
gboolean  gdk_init_check   	        (gint	   	*argc,
					 gchar        ***argv);
void gdk_add_option_entries_libgtk_only (GOptionGroup *group);
void gdk_pre_parse_libgtk_only          (void);

#ifndef GDK_DISABLE_DEPRECATED
void  	  gdk_exit		   	(gint	    	 error_code);
gchar*	  gdk_set_locale	   	(void);
#endif /* GDK_DISABLE_DEPRECATED */

const char *         gdk_get_program_class (void);
void                 gdk_set_program_class (const char *program_class);

/* Push and pop error handlers for X errors
 */
void      gdk_error_trap_push           (void);
gint      gdk_error_trap_pop            (void);

#ifndef GDK_DISABLE_DEPRECATED
void	  gdk_set_use_xshm		(gboolean	 use_xshm);
gboolean  gdk_get_use_xshm		(void);
#endif /* GDK_DISABLE_DEPRECATED */

gchar*	                  gdk_get_display		(void);
const gchar*	          gdk_get_display_arg_name	(void);

#ifndef GDK_DISABLE_DEPRECATED
gint gdk_input_add_full	  (gint		     source,
			   GdkInputCondition condition,
			   GdkInputFunction  function,
			   gpointer	     data,
			   GDestroyNotify    destroy);
gint gdk_input_add	  (gint		     source,
			   GdkInputCondition condition,
			   GdkInputFunction  function,
			   gpointer	     data);
void gdk_input_remove	  (gint		     tag);
#endif /* GDK_DISABLE_DEPRECATED */

GdkGrabStatus gdk_pointer_grab       (GdkWindow    *window,
				      gboolean      owner_events,
				      GdkEventMask  event_mask,
				      GdkWindow    *confine_to,
				      GdkCursor    *cursor,
				      guint32       time_);
GdkGrabStatus gdk_keyboard_grab      (GdkWindow    *window,
				      gboolean      owner_events,
				      guint32       time_);

gboolean gdk_pointer_grab_info_libgtk_only (GdkDisplay *display,
					    GdkWindow **grab_window,
					    gboolean   *owner_events);
gboolean gdk_keyboard_grab_info_libgtk_only (GdkDisplay *display,
					     GdkWindow **grab_window,
					     gboolean   *owner_events);

#ifndef GDK_MULTIHEAD_SAFE
void          gdk_pointer_ungrab     (guint32       time_);
void          gdk_keyboard_ungrab    (guint32       time_);
gboolean      gdk_pointer_is_grabbed (void);

gint gdk_screen_width  (void) G_GNUC_CONST;
gint gdk_screen_height (void) G_GNUC_CONST;

gint gdk_screen_width_mm  (void) G_GNUC_CONST;
gint gdk_screen_height_mm (void) G_GNUC_CONST;

void gdk_beep (void);
#endif /* GDK_MULTIHEAD_SAFE */

void gdk_flush (void);

#ifndef GDK_MULTIHEAD_SAFE
void gdk_set_double_click_time             (guint       msec);
#endif

/* Rectangle utilities
 */
gboolean gdk_rectangle_intersect (const GdkRectangle *src1,
				  const GdkRectangle *src2,
				  GdkRectangle       *dest);
void     gdk_rectangle_union     (const GdkRectangle *src1,
				  const GdkRectangle *src2,
				  GdkRectangle       *dest);

GType gdk_rectangle_get_type (void) G_GNUC_CONST;

#define GDK_TYPE_RECTANGLE (gdk_rectangle_get_type ())

/* Conversion functions between wide char and multibyte strings. 
 */
#ifndef GDK_DISABLE_DEPRECATED
gchar     *gdk_wcstombs          (const GdkWChar   *src);
gint       gdk_mbstowcs          (GdkWChar         *dest,
				  const gchar      *src,
				  gint              dest_max);
#endif

/* Miscellaneous */
#ifndef GDK_MULTIHEAD_SAFE
gboolean gdk_event_send_client_message      (GdkEvent       *event,
					     GdkNativeWindow winid);
void     gdk_event_send_clientmessage_toall (GdkEvent  *event);
#endif
gboolean gdk_event_send_client_message_for_display (GdkDisplay *display,
						    GdkEvent       *event,
						    GdkNativeWindow winid);

void gdk_notify_startup_complete (void);

void gdk_notify_startup_complete_with_id (const gchar* startup_id);

/* Threading
 */

#if !defined (GDK_DISABLE_DEPRECATED) || defined (GDK_COMPILATION)
GDKVAR GMutex *gdk_threads_mutex; /* private */
#endif

GDKVAR GCallback gdk_threads_lock;
GDKVAR GCallback gdk_threads_unlock;

void     gdk_threads_enter                    (void);
void     gdk_threads_leave                    (void);
void     gdk_threads_init                     (void);
void     gdk_threads_set_lock_functions       (GCallback enter_fn,
					       GCallback leave_fn);

guint    gdk_threads_add_idle_full            (gint           priority,
		                               GSourceFunc    function,
		                               gpointer       data,
		                               GDestroyNotify notify);
guint    gdk_threads_add_idle                 (GSourceFunc    function,
		                               gpointer       data);
guint    gdk_threads_add_timeout_full         (gint           priority,
                                               guint          interval,
                                               GSourceFunc    function,
                                               gpointer       data,
                                               GDestroyNotify notify);
guint    gdk_threads_add_timeout              (guint          interval,
                                               GSourceFunc    function,
                                               gpointer       data);
guint    gdk_threads_add_timeout_seconds_full (gint           priority,
                                               guint          interval,
                                               GSourceFunc    function,
                                               gpointer       data,
                                               GDestroyNotify notify);
guint    gdk_threads_add_timeout_seconds      (guint          interval,
                                               GSourceFunc    function,
                                               gpointer       data);

#ifdef	G_THREADS_ENABLED
#  define GDK_THREADS_ENTER()	G_STMT_START {	\
      if (gdk_threads_lock)                 	\
        (*gdk_threads_lock) ();			\
   } G_STMT_END
#  define GDK_THREADS_LEAVE()	G_STMT_START { 	\
      if (gdk_threads_unlock)                 	\
        (*gdk_threads_unlock) ();		\
   } G_STMT_END
#else	/* !G_THREADS_ENABLED */
#  define GDK_THREADS_ENTER()
#  define GDK_THREADS_LEAVE()
#endif	/* !G_THREADS_ENABLED */

G_END_DECLS


#endif /* __GDK_H__ */
