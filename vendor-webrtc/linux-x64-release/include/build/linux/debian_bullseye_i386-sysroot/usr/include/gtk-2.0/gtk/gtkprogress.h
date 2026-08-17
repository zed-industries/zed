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

#ifndef __GTK_PROGRESS_H__
#define __GTK_PROGRESS_H__


#include <gtk/gtkwidget.h>
#include <gtk/gtkadjustment.h>


G_BEGIN_DECLS

#if !defined (GTK_DISABLE_DEPRECATED) || defined (__GTK_PROGRESS_C__) || defined (__GTK_PROGRESS_BAR_C__)

#define GTK_TYPE_PROGRESS            (gtk_progress_get_type ())
#define GTK_PROGRESS(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_PROGRESS, GtkProgress))
#define GTK_PROGRESS_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_PROGRESS, GtkProgressClass))
#define GTK_IS_PROGRESS(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_PROGRESS))
#define GTK_IS_PROGRESS_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_PROGRESS))
#define GTK_PROGRESS_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_PROGRESS, GtkProgressClass))

#endif /* !GTK_DISABLE_DEPRECATED */

typedef struct _GtkProgress       GtkProgress;
typedef struct _GtkProgressClass  GtkProgressClass;


struct _GtkProgress
{
  GtkWidget widget;

  GtkAdjustment *adjustment;
  GdkPixmap     *offscreen_pixmap;
  gchar         *format;
  gfloat         x_align;
  gfloat         y_align;

  guint          show_text : 1;
  guint          activity_mode : 1;
  guint          use_text_format : 1;
};

struct _GtkProgressClass
{
  GtkWidgetClass parent_class;

  void (* paint)            (GtkProgress *progress);
  void (* update)           (GtkProgress *progress);
  void (* act_mode_enter)   (GtkProgress *progress);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

/* This entire interface is deprecated. Use GtkProgressBar
 * directly.
 */

#if !defined (GTK_DISABLE_DEPRECATED) || defined (__GTK_PROGRESS_C__) || defined (__GTK_PROGRESS_BAR_C__)

GType      gtk_progress_get_type            (void) G_GNUC_CONST;
void       gtk_progress_set_show_text       (GtkProgress   *progress,
					     gboolean       show_text);
void       gtk_progress_set_text_alignment  (GtkProgress   *progress,
					     gfloat         x_align,
					     gfloat         y_align);
void       gtk_progress_set_format_string   (GtkProgress   *progress,
					     const gchar   *format);
void       gtk_progress_set_adjustment      (GtkProgress   *progress,
					     GtkAdjustment *adjustment);
void       gtk_progress_configure           (GtkProgress   *progress,
					     gdouble        value,
					     gdouble        min,
					     gdouble        max);
void       gtk_progress_set_percentage      (GtkProgress   *progress,
					     gdouble        percentage);
void       gtk_progress_set_value           (GtkProgress   *progress,
					     gdouble        value);
gdouble    gtk_progress_get_value           (GtkProgress   *progress);
void       gtk_progress_set_activity_mode   (GtkProgress   *progress,
					     gboolean       activity_mode);
gchar*     gtk_progress_get_current_text    (GtkProgress   *progress);
gchar*     gtk_progress_get_text_from_value (GtkProgress   *progress,
					     gdouble        value);
gdouble    gtk_progress_get_current_percentage (GtkProgress *progress);
gdouble    gtk_progress_get_percentage_from_value (GtkProgress *progress,
						   gdouble      value);

#endif /* !GTK_DISABLE_DEPRECATED */

G_END_DECLS

#endif /* __GTK_PROGRESS_H__ */
