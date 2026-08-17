/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 * GtkStatusbar Copyright (C) 1998 Shawn T. Amundson
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

#ifndef __GTK_STATUSBAR_H__
#define __GTK_STATUSBAR_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkhbox.h>

G_BEGIN_DECLS

#define GTK_TYPE_STATUSBAR            (gtk_statusbar_get_type ())
#define GTK_STATUSBAR(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_STATUSBAR, GtkStatusbar))
#define GTK_STATUSBAR_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_STATUSBAR, GtkStatusbarClass))
#define GTK_IS_STATUSBAR(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_STATUSBAR))
#define GTK_IS_STATUSBAR_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_STATUSBAR))
#define GTK_STATUSBAR_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_STATUSBAR, GtkStatusbarClass))


typedef struct _GtkStatusbar      GtkStatusbar;
typedef struct _GtkStatusbarClass GtkStatusbarClass;

struct _GtkStatusbar
{
  GtkHBox parent_widget;

  GtkWidget *GSEAL (frame);
  GtkWidget *GSEAL (label);

  GSList *GSEAL (messages);
  GSList *GSEAL (keys);

  guint GSEAL (seq_context_id);
  guint GSEAL (seq_message_id);

  GdkWindow *GSEAL (grip_window);

  guint GSEAL (has_resize_grip) : 1;
};

struct _GtkStatusbarClass
{
  GtkHBoxClass parent_class;

  gpointer reserved;

  void	(*text_pushed)	(GtkStatusbar	*statusbar,
			 guint		 context_id,
			 const gchar	*text);
  void	(*text_popped)	(GtkStatusbar	*statusbar,
			 guint		 context_id,
			 const gchar	*text);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GType      gtk_statusbar_get_type     	(void) G_GNUC_CONST;
GtkWidget* gtk_statusbar_new          	(void);
/* If you don't want to use contexts, 0 is a predefined global
 * context_id you can pass to push/pop/remove
 */
guint	   gtk_statusbar_get_context_id	(GtkStatusbar *statusbar,
					 const gchar  *context_description);
/* Returns message_id used for gtk_statusbar_remove */
guint      gtk_statusbar_push          	(GtkStatusbar *statusbar,
					 guint	       context_id,
					 const gchar  *text);
void       gtk_statusbar_pop          	(GtkStatusbar *statusbar,
					 guint	       context_id);
void       gtk_statusbar_remove        	(GtkStatusbar *statusbar,
					 guint	       context_id,
					 guint         message_id);
void       gtk_statusbar_remove_all    	(GtkStatusbar *statusbar,
					 guint	       context_id);
					 

void     gtk_statusbar_set_has_resize_grip (GtkStatusbar *statusbar,
					    gboolean      setting);
gboolean gtk_statusbar_get_has_resize_grip (GtkStatusbar *statusbar);

GtkWidget* gtk_statusbar_get_message_area  (GtkStatusbar *statusbar);

G_END_DECLS

#endif /* __GTK_STATUSBAR_H__ */
