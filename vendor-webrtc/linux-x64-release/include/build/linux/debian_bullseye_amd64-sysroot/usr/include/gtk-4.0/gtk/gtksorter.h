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

#ifndef __GTK_SORTER_H__
#define __GTK_SORTER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkenums.h>

G_BEGIN_DECLS

/**
 * GtkSorterOrder:
 * @GTK_SORTER_ORDER_PARTIAL: A partial order. Any `GtkOrdering` is possible.
 * @GTK_SORTER_ORDER_NONE: No order, all elements are considered equal.
 *   gtk_sorter_compare() will only return %GTK_ORDERING_EQUAL.
 * @GTK_SORTER_ORDER_TOTAL: A total order. gtk_sorter_compare() will only
 *   return %GTK_ORDERING_EQUAL if an item is compared with itself. Two
 *   different items will never cause this value to be returned.
 *
 * Describes the type of order that a `GtkSorter` may produce.
 */
typedef enum {
  GTK_SORTER_ORDER_PARTIAL,
  GTK_SORTER_ORDER_NONE,
  GTK_SORTER_ORDER_TOTAL
} GtkSorterOrder;

/**
 * GtkSorterChange:
 * @GTK_SORTER_CHANGE_DIFFERENT: The sorter change cannot be described
 *   by any of the other enumeration values
 * @GTK_SORTER_CHANGE_INVERTED: The sort order was inverted. Comparisons
 *   that returned %GTK_ORDERING_SMALLER now return %GTK_ORDERING_LARGER
 *   and vice versa. Other comparisons return the same values as before.
 * @GTK_SORTER_CHANGE_LESS_STRICT: The sorter is less strict: Comparisons
 *   may now return %GTK_ORDERING_EQUAL that did not do so before.
 * @GTK_SORTER_CHANGE_MORE_STRICT: The sorter is more strict: Comparisons
 *   that did return %GTK_ORDERING_EQUAL may not do so anymore.
 *
 * Describes changes in a sorter in more detail and allows users
 * to optimize resorting.
 */
typedef enum {
  GTK_SORTER_CHANGE_DIFFERENT,
  GTK_SORTER_CHANGE_INVERTED,
  GTK_SORTER_CHANGE_LESS_STRICT,
  GTK_SORTER_CHANGE_MORE_STRICT
} GtkSorterChange;

#define GTK_TYPE_SORTER             (gtk_sorter_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_DERIVABLE_TYPE (GtkSorter, gtk_sorter, GTK, SORTER, GObject)

/**
 * GtkSorterClass
 * @compare: Compare two items. See gtk_sorter_compare() for details.
 * @get_order: Get the `GtkSorderOrder` that applies to the current sorter.
 *   If unimplemented, it returns %GTK_SORTER_ORDER_PARTIAL.
 *
 * The virtual table for `GtkSorter`.
 */
struct _GtkSorterClass
{
  GObjectClass parent_class;

  GtkOrdering           (* compare)                             (GtkSorter              *self,
                                                                 gpointer                item1,
                                                                 gpointer                item2);

  /* optional */
  GtkSorterOrder        (* get_order)                           (GtkSorter              *self);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
  void (*_gtk_reserved5) (void);
  void (*_gtk_reserved6) (void);
  void (*_gtk_reserved7) (void);
  void (*_gtk_reserved8) (void);
};

GDK_AVAILABLE_IN_ALL
GtkOrdering             gtk_sorter_compare                      (GtkSorter              *self,
                                                                 gpointer                item1,
                                                                 gpointer                item2);
GDK_AVAILABLE_IN_ALL
GtkSorterOrder          gtk_sorter_get_order                    (GtkSorter              *self);

/* for sorter implementations */
GDK_AVAILABLE_IN_ALL
void                    gtk_sorter_changed                      (GtkSorter              *self,
                                                                 GtkSorterChange         change);


G_END_DECLS

#endif /* __GTK_SORTER_H__ */

