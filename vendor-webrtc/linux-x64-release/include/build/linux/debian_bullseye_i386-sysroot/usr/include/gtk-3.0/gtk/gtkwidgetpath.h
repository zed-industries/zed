/* GTK - The GIMP Toolkit
 * Copyright (C) 2010 Carlos Garnacho <carlosg@gnome.org>
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

#ifndef __GTK_WIDGET_PATH_H__
#define __GTK_WIDGET_PATH_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <glib-object.h>
#include <gdk/gdk.h>
#include <gtk/gtkenums.h>
#include <gtk/gtktypes.h>

G_BEGIN_DECLS

#define GTK_TYPE_WIDGET_PATH (gtk_widget_path_get_type ())

GDK_AVAILABLE_IN_ALL
GType           gtk_widget_path_get_type            (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidgetPath * gtk_widget_path_new                 (void);

GDK_AVAILABLE_IN_ALL
GtkWidgetPath * gtk_widget_path_copy                (const GtkWidgetPath *path);
GDK_AVAILABLE_IN_3_2
GtkWidgetPath * gtk_widget_path_ref                 (GtkWidgetPath       *path);
GDK_AVAILABLE_IN_3_2
void            gtk_widget_path_unref               (GtkWidgetPath       *path);
GDK_AVAILABLE_IN_ALL
void            gtk_widget_path_free                (GtkWidgetPath       *path);

GDK_AVAILABLE_IN_3_2
char *          gtk_widget_path_to_string           (const GtkWidgetPath *path);
GDK_AVAILABLE_IN_ALL
gint            gtk_widget_path_length              (const GtkWidgetPath *path);

GDK_AVAILABLE_IN_ALL
gint            gtk_widget_path_append_type         (GtkWidgetPath       *path,
                                                     GType                type);
GDK_AVAILABLE_IN_ALL
void            gtk_widget_path_prepend_type        (GtkWidgetPath       *path,
                                                     GType                type);
GDK_AVAILABLE_IN_3_2
gint            gtk_widget_path_append_with_siblings(GtkWidgetPath       *path,
                                                     GtkWidgetPath       *siblings,
                                                     guint                sibling_index);
/* gtk_widget_path_append_for_widget() is declared in gtkwidget.c */
GDK_AVAILABLE_IN_3_2
gint            gtk_widget_path_append_for_widget   (GtkWidgetPath       *path,
                                                     GtkWidget           *widget);

GDK_AVAILABLE_IN_ALL
GType               gtk_widget_path_iter_get_object_type  (const GtkWidgetPath *path,
                                                           gint                 pos);
GDK_AVAILABLE_IN_ALL
void                gtk_widget_path_iter_set_object_type  (GtkWidgetPath       *path,
                                                           gint                 pos,
                                                           GType                type);
GDK_AVAILABLE_IN_3_20
const char *        gtk_widget_path_iter_get_object_name  (const GtkWidgetPath *path,
                                                           gint                 pos);
GDK_AVAILABLE_IN_3_20
void                gtk_widget_path_iter_set_object_name  (GtkWidgetPath       *path,
                                                           gint                 pos,
                                                           const char          *name);
GDK_AVAILABLE_IN_ALL
const GtkWidgetPath *
                    gtk_widget_path_iter_get_siblings     (const GtkWidgetPath *path,
                                                           gint                 pos);
GDK_AVAILABLE_IN_ALL
guint               gtk_widget_path_iter_get_sibling_index(const GtkWidgetPath *path,
                                                           gint                 pos);

GDK_AVAILABLE_IN_ALL
const gchar *          gtk_widget_path_iter_get_name  (const GtkWidgetPath *path,
                                                       gint                 pos);
GDK_AVAILABLE_IN_ALL
void                   gtk_widget_path_iter_set_name  (GtkWidgetPath       *path,
                                                       gint                 pos,
                                                       const gchar         *name);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_widget_path_iter_has_name  (const GtkWidgetPath *path,
                                                       gint                 pos,
                                                       const gchar         *name);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_widget_path_iter_has_qname (const GtkWidgetPath *path,
                                                       gint                 pos,
                                                       GQuark               qname);
GDK_AVAILABLE_IN_3_14
GtkStateFlags          gtk_widget_path_iter_get_state (const GtkWidgetPath *path,
                                                       gint                 pos);
GDK_AVAILABLE_IN_3_14
void                   gtk_widget_path_iter_set_state (GtkWidgetPath       *path,
                                                       gint                 pos,
                                                       GtkStateFlags        state);

GDK_AVAILABLE_IN_ALL
void     gtk_widget_path_iter_add_class     (GtkWidgetPath       *path,
                                             gint                 pos,
                                             const gchar         *name);
GDK_AVAILABLE_IN_ALL
void     gtk_widget_path_iter_remove_class  (GtkWidgetPath       *path,
                                             gint                 pos,
                                             const gchar         *name);
GDK_AVAILABLE_IN_ALL
void     gtk_widget_path_iter_clear_classes (GtkWidgetPath       *path,
                                             gint                 pos);
GDK_AVAILABLE_IN_ALL
GSList * gtk_widget_path_iter_list_classes  (const GtkWidgetPath *path,
                                             gint                 pos);
GDK_AVAILABLE_IN_ALL
gboolean gtk_widget_path_iter_has_class     (const GtkWidgetPath *path,
                                             gint                 pos,
                                             const gchar         *name);
GDK_AVAILABLE_IN_ALL
gboolean gtk_widget_path_iter_has_qclass    (const GtkWidgetPath *path,
                                             gint                 pos,
                                             GQuark               qname);

GDK_DEPRECATED_IN_3_14
void     gtk_widget_path_iter_add_region    (GtkWidgetPath      *path,
                                             gint                pos,
                                             const gchar        *name,
                                             GtkRegionFlags     flags);
GDK_DEPRECATED_IN_3_14
void     gtk_widget_path_iter_remove_region (GtkWidgetPath      *path,
                                             gint                pos,
                                             const gchar        *name);
GDK_DEPRECATED_IN_3_14
void     gtk_widget_path_iter_clear_regions (GtkWidgetPath      *path,
                                             gint                pos);

GDK_DEPRECATED_IN_3_14
GSList * gtk_widget_path_iter_list_regions  (const GtkWidgetPath *path,
                                             gint                 pos);

GDK_DEPRECATED_IN_3_14
gboolean gtk_widget_path_iter_has_region    (const GtkWidgetPath *path,
                                             gint                 pos,
                                             const gchar         *name,
                                             GtkRegionFlags      *flags);
GDK_DEPRECATED_IN_3_14
gboolean gtk_widget_path_iter_has_qregion   (const GtkWidgetPath *path,
                                             gint                 pos,
                                             GQuark               qname,
                                             GtkRegionFlags      *flags);

GDK_AVAILABLE_IN_ALL
GType           gtk_widget_path_get_object_type (const GtkWidgetPath *path);

GDK_AVAILABLE_IN_ALL
gboolean        gtk_widget_path_is_type    (const GtkWidgetPath *path,
                                            GType                type);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_widget_path_has_parent (const GtkWidgetPath *path,
                                            GType                type);


G_END_DECLS

#endif /* __GTK_WIDGET_PATH_H__ */
