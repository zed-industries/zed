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

#ifndef __GTK_MAIN_H__
#define __GTK_MAIN_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkwidget.h>
#ifdef G_PLATFORM_WIN32
#include <gtk/gtkbox.h>
#include <gtk/gtkwindow.h>
#endif

G_BEGIN_DECLS

/* Priorities for redrawing and resizing
 */
#define GTK_PRIORITY_RESIZE     (G_PRIORITY_HIGH_IDLE + 10)

#ifndef GTK_DISABLE_DEPRECATED

/* Use GDK_PRIORITY_REDRAW */
#define GTK_PRIORITY_REDRAW     (G_PRIORITY_HIGH_IDLE + 20)

/* Deprecated. Use G_PRIORITY #define's instead
 */
#define GTK_PRIORITY_HIGH       G_PRIORITY_HIGH
#define GTK_PRIORITY_INTERNAL   GTK_PRIORITY_REDRAW
#define GTK_PRIORITY_DEFAULT	G_PRIORITY_DEFAULT_IDLE
#define GTK_PRIORITY_LOW	G_PRIORITY_LOW

#endif /* GTK_DISABLE_DEPRECATED */

typedef gint	(*GtkKeySnoopFunc)	    (GtkWidget	  *grab_widget,
					     GdkEventKey  *event,
					     gpointer	   func_data);

/* Gtk version.
 */
#ifdef G_PLATFORM_WIN32
#ifdef GTK_COMPILATION
#define GTKMAIN_C_VAR extern __declspec(dllexport)
#else
#define GTKMAIN_C_VAR extern __declspec(dllimport)
#endif
#else
#define GTKMAIN_C_VAR extern
#endif

GTKMAIN_C_VAR const guint gtk_major_version;
GTKMAIN_C_VAR const guint gtk_minor_version;
GTKMAIN_C_VAR const guint gtk_micro_version;
GTKMAIN_C_VAR const guint gtk_binary_age;
GTKMAIN_C_VAR const guint gtk_interface_age;
const gchar* gtk_check_version (guint	required_major,
			        guint	required_minor,
			        guint	required_micro);


/* Initialization, exit, mainloop and miscellaneous routines
 */

gboolean gtk_parse_args           (int    *argc,
				   char ***argv);

void     gtk_init                 (int    *argc,
                                   char ***argv);

gboolean gtk_init_check           (int    *argc,
                                   char ***argv);
  
gboolean gtk_init_with_args       (int            *argc,
				   char         ***argv,
				   const char     *parameter_string,
				   GOptionEntry   *entries,
				   const char     *translation_domain,
				   GError        **error);

GOptionGroup *gtk_get_option_group (gboolean open_default_display);
  
#ifdef G_PLATFORM_WIN32

/* Variants that are used to check for correct struct packing
 * when building GTK+-using code.
 */
void	 gtk_init_abi_check       (int	  *argc,
				   char	***argv,
				   int     num_checks,
				   size_t  sizeof_GtkWindow,
				   size_t  sizeof_GtkBox);
gboolean gtk_init_check_abi_check (int	  *argc,
				   char	***argv,
				   int     num_checks,
				   size_t  sizeof_GtkWindow,
				   size_t  sizeof_GtkBox);

#define gtk_init(argc, argv) gtk_init_abi_check (argc, argv, 2, sizeof (GtkWindow), sizeof (GtkBox))
#define gtk_init_check(argc, argv) gtk_init_check_abi_check (argc, argv, 2, sizeof (GtkWindow), sizeof (GtkBox))

#endif

#ifndef GTK_DISABLE_DEPRECATED
void     gtk_exit                 (gint    error_code);
gchar *        gtk_set_locale           (void);
#endif /* GTK_DISABLE_DEPRECATED */

void           gtk_disable_setlocale    (void);
PangoLanguage *gtk_get_default_language (void);
gboolean       gtk_events_pending       (void);

/* The following is the event func GTK+ registers with GDK
 * we expose it mainly to allow filtering of events between
 * GDK and GTK+.
 */
void 	   gtk_main_do_event	   (GdkEvent           *event);

void	   gtk_main		   (void);
guint	   gtk_main_level	   (void);
void	   gtk_main_quit	   (void);
gboolean   gtk_main_iteration	   (void);
/* gtk_main_iteration() calls gtk_main_iteration_do(TRUE) */
gboolean   gtk_main_iteration_do   (gboolean blocking);

gboolean   gtk_true		   (void) G_GNUC_CONST;
gboolean   gtk_false		   (void) G_GNUC_CONST;

void	   gtk_grab_add		   (GtkWidget	       *widget);
GtkWidget* gtk_grab_get_current	   (void);
void	   gtk_grab_remove	   (GtkWidget	       *widget);

#if !defined (GTK_DISABLE_DEPRECATED) || defined (GTK_COMPILATION)
void	   gtk_init_add		   (GtkFunction	       function,
				    gpointer	       data);
void	   gtk_quit_add_destroy	   (guint	       main_level,
				    GtkObject	      *object);
guint	   gtk_quit_add		   (guint	       main_level,
				    GtkFunction	       function,
				    gpointer	       data);
guint	   gtk_quit_add_full	   (guint	       main_level,
				    GtkFunction	       function,
				    GtkCallbackMarshal marshal,
				    gpointer	       data,
				    GDestroyNotify     destroy);
void	   gtk_quit_remove	   (guint	       quit_handler_id);
void	   gtk_quit_remove_by_data (gpointer	       data);
guint	   gtk_timeout_add	   (guint32	       interval,
				    GtkFunction	       function,
				    gpointer	       data);
guint	   gtk_timeout_add_full	   (guint32	       interval,
				    GtkFunction	       function,
				    GtkCallbackMarshal marshal,
				    gpointer	       data,
				    GDestroyNotify     destroy);
void	   gtk_timeout_remove	   (guint	       timeout_handler_id);

guint	   gtk_idle_add		   (GtkFunction	       function,
				    gpointer	       data);
guint	   gtk_idle_add_priority   (gint	       priority,
				    GtkFunction	       function,
				    gpointer	       data);
guint	   gtk_idle_add_full	   (gint	       priority,
				    GtkFunction	       function,
				    GtkCallbackMarshal marshal,
				    gpointer	       data,
				    GDestroyNotify     destroy);
void	   gtk_idle_remove	   (guint	       idle_handler_id);
void	   gtk_idle_remove_by_data (gpointer	       data);
guint	   gtk_input_add_full	   (gint	       source,
				    GdkInputCondition  condition,
				    GdkInputFunction   function,
				    GtkCallbackMarshal marshal,
				    gpointer	       data,
				    GDestroyNotify     destroy);
void	   gtk_input_remove	   (guint	       input_handler_id);
#endif /* GTK_DISABLE_DEPRECATED */

guint	   gtk_key_snooper_install (GtkKeySnoopFunc snooper,
				    gpointer	    func_data);
void	   gtk_key_snooper_remove  (guint	    snooper_handler_id);

GdkEvent*       gtk_get_current_event       (void);
guint32         gtk_get_current_event_time  (void);
gboolean        gtk_get_current_event_state (GdkModifierType *state);

GtkWidget* gtk_get_event_widget	   (GdkEvent	   *event);


/* Private routines internal to GTK+ 
 */
void       gtk_propagate_event     (GtkWidget         *widget,
				    GdkEvent          *event);

gboolean _gtk_boolean_handled_accumulator (GSignalInvocationHint *ihint,
                                   GValue                *return_accu,
                                   const GValue          *handler_return,
                                   gpointer               dummy);

gchar *_gtk_get_lc_ctype (void);

gboolean _gtk_module_has_mixed_deps (GModule *module);


G_END_DECLS

#endif /* __GTK_MAIN_H__ */
