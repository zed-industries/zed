/* GTK - The GIMP Toolkit
 * Copyright (C) 2020 Red Hat, Inc.
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

#ifndef __GTK_EDITABLE_LABEL_H__
#define __GTK_EDITABLE_LABEL_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_EDITABLE_LABEL (gtk_editable_label_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkEditableLabel, gtk_editable_label, GTK, EDITABLE_LABEL, GtkWidget)

GDK_AVAILABLE_IN_ALL
GtkWidget *           gtk_editable_label_new            (const char       *str);

GDK_AVAILABLE_IN_ALL
gboolean              gtk_editable_label_get_editing    (GtkEditableLabel *self);

GDK_AVAILABLE_IN_ALL
void                  gtk_editable_label_start_editing  (GtkEditableLabel *self);

GDK_AVAILABLE_IN_ALL
void                  gtk_editable_label_stop_editing   (GtkEditableLabel *self,
                                                         gboolean          commit);

G_END_DECLS

#endif /* __GTK_EDITABLE_LABEL_H__ */
