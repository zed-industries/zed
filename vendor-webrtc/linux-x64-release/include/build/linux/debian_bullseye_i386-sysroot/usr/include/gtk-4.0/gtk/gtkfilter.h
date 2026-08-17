/*
 * Copyright © 2019 Benjamin Otte
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
 * Authors: Benjamin Otte <otte@gnome.org>
 */

#ifndef __GTK_FILTER_H__
#define __GTK_FILTER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>

G_BEGIN_DECLS

/**
 * GtkFilterMatch:
 * @GTK_FILTER_MATCH_SOME: The filter matches some items,
 *   gtk_filter_match() may return %TRUE or %FALSE
 * @GTK_FILTER_MATCH_NONE: The filter does not match any item,
 *   gtk_filter_match() will always return %FALSE.
 * @GTK_FILTER_MATCH_ALL: The filter matches all items,
 *   gtk_filter_match() will alays return %TRUE.
 *
 * Describes the known strictness of a filter.
 *
 * Note that for filters where the strictness is not known,
 * %GTK_FILTER_MATCH_SOME is always an acceptable value,
 * even if a filter does match all or no items.
 */
typedef enum {
  GTK_FILTER_MATCH_SOME = 0,
  GTK_FILTER_MATCH_NONE,
  GTK_FILTER_MATCH_ALL
} GtkFilterMatch;

/**
 * GtkFilterChange:
 * @GTK_FILTER_CHANGE_DIFFERENT: The filter change cannot be
 *   described with any of the other enumeration values.
 * @GTK_FILTER_CHANGE_LESS_STRICT: The filter is less strict than
 *   it was before: All items that it used to return %TRUE for
 *   still return %TRUE, others now may, too.
 * @GTK_FILTER_CHANGE_MORE_STRICT: The filter is more strict than
 *   it was before: All items that it used to return %FALSE for
 *   still return %FALSE, others now may, too.
 *
 * Describes changes in a filter in more detail and allows objects
 * using the filter to optimize refiltering items.
 *
 * If you are writing an implementation and are not sure which
 * value to pass, %GTK_FILTER_CHANGE_DIFFERENT is always a correct
 * choice.
 */
typedef enum {
  GTK_FILTER_CHANGE_DIFFERENT = 0,
  GTK_FILTER_CHANGE_LESS_STRICT,
  GTK_FILTER_CHANGE_MORE_STRICT,
} GtkFilterChange;

#define GTK_TYPE_FILTER             (gtk_filter_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_DERIVABLE_TYPE (GtkFilter, gtk_filter, GTK, FILTER, GObject)

struct _GtkFilterClass
{
  GObjectClass parent_class;

  gboolean              (* match)                               (GtkFilter              *self,
                                                                 gpointer                item);

  /* optional */
  GtkFilterMatch        (* get_strictness)                      (GtkFilter              *self);

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
gboolean                gtk_filter_match                        (GtkFilter              *self,
                                                                 gpointer                item);
GDK_AVAILABLE_IN_ALL
GtkFilterMatch          gtk_filter_get_strictness               (GtkFilter              *self);

/* for filter implementations */
GDK_AVAILABLE_IN_ALL
void                    gtk_filter_changed                      (GtkFilter              *self,
                                                                 GtkFilterChange         change);


G_END_DECLS

#endif /* __GTK_FILTER_H__ */
