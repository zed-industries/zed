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

#ifndef GTK_DISABLE_DEPRECATED

#ifndef __GTK_TOOLTIPS_H__
#define __GTK_TOOLTIPS_H__

#include <gtk/gtkwidget.h>
#include <gtk/gtkwindow.h>


G_BEGIN_DECLS

#define GTK_TYPE_TOOLTIPS                  (gtk_tooltips_get_type ())
#define GTK_TOOLTIPS(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TOOLTIPS, GtkTooltips))
#define GTK_TOOLTIPS_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TOOLTIPS, GtkTooltipsClass))
#define GTK_IS_TOOLTIPS(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TOOLTIPS))
#define GTK_IS_TOOLTIPS_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TOOLTIPS))
#define GTK_TOOLTIPS_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TOOLTIPS, GtkTooltipsClass))


typedef struct _GtkTooltips	 GtkTooltips;
typedef struct _GtkTooltipsClass GtkTooltipsClass;
typedef struct _GtkTooltipsData	 GtkTooltipsData;

struct _GtkTooltipsData
{
  GtkTooltips *tooltips;
  GtkWidget *widget;
  gchar *tip_text;
  gchar *tip_private;
};

struct _GtkTooltips
{
  GtkObject parent_instance;

  /*< private >*/
  GtkWidget *tip_window;
  GtkWidget *tip_label;
  GtkTooltipsData *active_tips_data;
  GList *tips_data_list; /* unused */

  guint   delay : 30;
  guint	  enabled : 1;
  guint   have_grab : 1;
  guint   use_sticky_delay : 1;
  gint	  timer_tag;
  GTimeVal last_popdown;
};

struct _GtkTooltipsClass
{
  GtkObjectClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GType		 gtk_tooltips_get_type	   (void) G_GNUC_CONST;
GtkTooltips*	 gtk_tooltips_new	   (void);

void		 gtk_tooltips_enable	   (GtkTooltips   *tooltips);
void		 gtk_tooltips_disable	   (GtkTooltips   *tooltips);
void		 gtk_tooltips_set_delay	   (GtkTooltips   *tooltips,
					    guint	   delay);
void		 gtk_tooltips_set_tip	   (GtkTooltips   *tooltips,
					    GtkWidget	  *widget,
					    const gchar   *tip_text,
					    const gchar   *tip_private);
GtkTooltipsData* gtk_tooltips_data_get	   (GtkWidget	  *widget);
void             gtk_tooltips_force_window (GtkTooltips   *tooltips);

gboolean         gtk_tooltips_get_info_from_tip_window (GtkWindow    *tip_window,
                                                        GtkTooltips **tooltips,
                                                        GtkWidget   **current_widget);

G_END_DECLS

#endif /* __GTK_TOOLTIPS_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
