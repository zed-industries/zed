/* GTK - The GIMP Toolkit
 * Copyright © 2012 Red Hat, Inc.
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
 *
 * Author: Cosimo Cecchi <cosimoc@gnome.org>
 *
 */

#ifndef __GTK_LEVEL_BAR_H__
#define __GTK_LEVEL_BAR_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_LEVEL_BAR            (gtk_level_bar_get_type ())
#define GTK_LEVEL_BAR(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_LEVEL_BAR, GtkLevelBar))
#define GTK_LEVEL_BAR_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_LEVEL_BAR, GtkLevelBarClass))
#define GTK_IS_LEVEL_BAR(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_LEVEL_BAR))
#define GTK_IS_LEVEL_BAR_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_LEVEL_BAR))
#define GTK_LEVEL_BAR_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_LEVEL_BAR, GtkLevelBarClass))

/**
 * GTK_LEVEL_BAR_OFFSET_LOW:
 *
 * The name used for the stock low offset included by #GtkLevelBar.
 *
 * Since: 3.6
 */
#define GTK_LEVEL_BAR_OFFSET_LOW  "low"

/**
 * GTK_LEVEL_BAR_OFFSET_HIGH:
 *
 * The name used for the stock high offset included by #GtkLevelBar.
 *
 * Since: 3.6
 */
#define GTK_LEVEL_BAR_OFFSET_HIGH "high"

/**
 * GTK_LEVEL_BAR_OFFSET_FULL:
 *
 * The name used for the stock full offset included by #GtkLevelBar.
 *
 * Since: 3.20
 */
#define GTK_LEVEL_BAR_OFFSET_FULL "full"

typedef struct _GtkLevelBarClass   GtkLevelBarClass;
typedef struct _GtkLevelBar        GtkLevelBar;
typedef struct _GtkLevelBarPrivate GtkLevelBarPrivate;

struct _GtkLevelBar {
  /*< private >*/
  GtkWidget parent;

  GtkLevelBarPrivate *priv;
};

struct _GtkLevelBarClass {
  /*< private >*/
  GtkWidgetClass parent_class;

  void (* offset_changed) (GtkLevelBar *self,
                           const gchar *name);

  /* padding for future class expansion */
  gpointer padding[16];
};

GDK_AVAILABLE_IN_3_6
GType      gtk_level_bar_get_type           (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_6
GtkWidget *gtk_level_bar_new                (void);

GDK_AVAILABLE_IN_3_6
GtkWidget *gtk_level_bar_new_for_interval   (gdouble      min_value,
                                             gdouble      max_value);

GDK_AVAILABLE_IN_3_6
void       gtk_level_bar_set_mode           (GtkLevelBar *self,
                                             GtkLevelBarMode mode);
GDK_AVAILABLE_IN_3_6
GtkLevelBarMode gtk_level_bar_get_mode      (GtkLevelBar *self);

GDK_AVAILABLE_IN_3_6
void       gtk_level_bar_set_value          (GtkLevelBar *self,
                                             gdouble      value);
GDK_AVAILABLE_IN_3_6
gdouble    gtk_level_bar_get_value          (GtkLevelBar *self);

GDK_AVAILABLE_IN_3_6
void       gtk_level_bar_set_min_value      (GtkLevelBar *self,
                                             gdouble      value);
GDK_AVAILABLE_IN_3_6
gdouble    gtk_level_bar_get_min_value      (GtkLevelBar *self);

GDK_AVAILABLE_IN_3_6
void       gtk_level_bar_set_max_value      (GtkLevelBar *self,
                                             gdouble      value);
GDK_AVAILABLE_IN_3_6
gdouble    gtk_level_bar_get_max_value      (GtkLevelBar *self);

GDK_AVAILABLE_IN_3_8
void       gtk_level_bar_set_inverted       (GtkLevelBar *self,
                                             gboolean     inverted);

GDK_AVAILABLE_IN_3_8
gboolean   gtk_level_bar_get_inverted       (GtkLevelBar *self);

GDK_AVAILABLE_IN_3_6
void       gtk_level_bar_add_offset_value   (GtkLevelBar *self,
                                             const gchar *name,
                                             gdouble      value);
GDK_AVAILABLE_IN_3_6
void       gtk_level_bar_remove_offset_value (GtkLevelBar *self,
                                              const gchar *name);
GDK_AVAILABLE_IN_3_6
gboolean   gtk_level_bar_get_offset_value   (GtkLevelBar *self,
                                             const gchar *name,
                                             gdouble     *value);

G_END_DECLS

#endif /* __GTK_LEVEL_BAR_H__ */
