/*
 * Copyright © 2019 Matthias Clasen
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors: Matthias Clasen <mclasen@redhat.com>
 */

#ifndef __GTK_TREE_LIST_ROW_SORTER_H__
#define __GTK_TREE_LIST_ROW_SORTER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkexpression.h>
#include <gtk/gtksorter.h>

G_BEGIN_DECLS

#define GTK_TYPE_TREE_LIST_ROW_SORTER             (gtk_tree_list_row_sorter_get_type ())
GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkTreeListRowSorter, gtk_tree_list_row_sorter, GTK, TREE_LIST_ROW_SORTER, GtkSorter)

GDK_AVAILABLE_IN_ALL
GtkTreeListRowSorter *  gtk_tree_list_row_sorter_new                   (GtkSorter            *sorter);

GDK_AVAILABLE_IN_ALL
GtkSorter *             gtk_tree_list_row_sorter_get_sorter            (GtkTreeListRowSorter *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_tree_list_row_sorter_set_sorter            (GtkTreeListRowSorter *self,
                                                                        GtkSorter            *sorter);

G_END_DECLS

#endif /* __GTK_TREE_LIST_ROW_SORTER_H__ */
