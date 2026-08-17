/* HSV color selector for GTK+
 *
 * Copyright (C) 1999 The Free Software Foundation
 *
 * Authors: Simon Budig <Simon.Budig@unix-ag.org> (original code)
 *          Federico Mena-Quintero <federico@gimp.org> (cleanup for GTK+)
 *          Jonathan Blandford <jrb@redhat.com> (cleanup for GTK+)
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

#ifndef __GTK_HSV_H__
#define __GTK_HSV_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_HSV            (gtk_hsv_get_type ())
#define GTK_HSV(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_HSV, GtkHSV))
#define GTK_HSV_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_HSV, GtkHSVClass))
#define GTK_IS_HSV(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_HSV))
#define GTK_IS_HSV_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_HSV))
#define GTK_HSV_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_HSV, GtkHSVClass))


typedef struct _GtkHSV              GtkHSV;
typedef struct _GtkHSVPrivate       GtkHSVPrivate;
typedef struct _GtkHSVClass         GtkHSVClass;

struct _GtkHSV
{
  GtkWidget parent_instance;

  /*< private >*/
  GtkHSVPrivate *priv;
};

struct _GtkHSVClass
{
  GtkWidgetClass parent_class;

  /* Notification signals */
  void (* changed) (GtkHSV          *hsv);

  /* Keybindings */
  void (* move)    (GtkHSV          *hsv,
                    GtkDirectionType type);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GDK_DEPRECATED_IN_3_4
GType      gtk_hsv_get_type     (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_4
GtkWidget* gtk_hsv_new          (void);
GDK_DEPRECATED_IN_3_4
void       gtk_hsv_set_color    (GtkHSV    *hsv,
				 double     h,
				 double     s,
				 double     v);
GDK_DEPRECATED_IN_3_4
void       gtk_hsv_get_color    (GtkHSV    *hsv,
				 gdouble   *h,
				 gdouble   *s,
				 gdouble   *v);
GDK_DEPRECATED_IN_3_4
void       gtk_hsv_set_metrics  (GtkHSV    *hsv,
				 gint       size,
				 gint       ring_width);
GDK_DEPRECATED_IN_3_4
void       gtk_hsv_get_metrics  (GtkHSV    *hsv,
				 gint      *size,
				 gint      *ring_width);
GDK_DEPRECATED_IN_3_4
gboolean   gtk_hsv_is_adjusting (GtkHSV    *hsv);

G_END_DECLS

#endif /* __GTK_HSV_H__ */
