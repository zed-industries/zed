/* GTK - The GIMP Toolkit
 *
 * Copyright (C) 2003 Sun Microsystems, Inc.
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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors:
 *      Mark McLoughlin <mark@skynet.ie>
 */

#ifndef __GTK_EXPANDER_H__
#define __GTK_EXPANDER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_EXPANDER            (gtk_expander_get_type ())
#define GTK_EXPANDER(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_EXPANDER, GtkExpander))
#define GTK_IS_EXPANDER(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_EXPANDER))

typedef struct _GtkExpander        GtkExpander;

GDK_AVAILABLE_IN_ALL
GType                 gtk_expander_get_type            (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget            *gtk_expander_new                 (const char *label);
GDK_AVAILABLE_IN_ALL
GtkWidget            *gtk_expander_new_with_mnemonic   (const char *label);

GDK_AVAILABLE_IN_ALL
void                  gtk_expander_set_expanded        (GtkExpander *expander,
                                                        gboolean     expanded);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_expander_get_expanded        (GtkExpander *expander);

GDK_AVAILABLE_IN_ALL
void                  gtk_expander_set_label           (GtkExpander *expander,
                                                        const char *label);
GDK_AVAILABLE_IN_ALL
const char *         gtk_expander_get_label           (GtkExpander *expander);

GDK_AVAILABLE_IN_ALL
void                  gtk_expander_set_use_underline   (GtkExpander *expander,
                                                        gboolean     use_underline);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_expander_get_use_underline   (GtkExpander *expander);

GDK_AVAILABLE_IN_ALL
void                  gtk_expander_set_use_markup      (GtkExpander *expander,
                                                        gboolean    use_markup);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_expander_get_use_markup      (GtkExpander *expander);

GDK_AVAILABLE_IN_ALL
void                  gtk_expander_set_label_widget    (GtkExpander *expander,
                                                        GtkWidget   *label_widget);
GDK_AVAILABLE_IN_ALL
GtkWidget            *gtk_expander_get_label_widget    (GtkExpander *expander);
GDK_AVAILABLE_IN_ALL
void                  gtk_expander_set_resize_toplevel (GtkExpander *expander,
                                                        gboolean     resize_toplevel);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_expander_get_resize_toplevel (GtkExpander *expander);

GDK_AVAILABLE_IN_ALL
void                  gtk_expander_set_child           (GtkExpander *expander,
                                                        GtkWidget      *child);
GDK_AVAILABLE_IN_ALL
GtkWidget *           gtk_expander_get_child           (GtkExpander *expander);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkExpander, g_object_unref)

G_END_DECLS

#endif /* __GTK_EXPANDER_H__ */
