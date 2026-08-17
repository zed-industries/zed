/*
 * Copyright © 2021 Benjamin Otte
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

#ifndef __GTK_SYMBOLIC_PAINTABLE_H__
#define __GTK_SYMBOLIC_PAINTABLE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtktypes.h>

G_BEGIN_DECLS

#define GTK_TYPE_SYMBOLIC_PAINTABLE       (gtk_symbolic_paintable_get_type ())

GDK_AVAILABLE_IN_4_6
G_DECLARE_INTERFACE (GtkSymbolicPaintable, gtk_symbolic_paintable, GTK, SYMBOLIC_PAINTABLE, GdkPaintable)

/**
 * GtkSymbolicPaintableInterface:
 * @snapshot_symbolic: Snapshot the paintable using the given colors. 
 *   See `GtkSymbolicPaintable::snapshot_symbolic()` for details.
 *   If this function is not implemented, [vfunc@Gdk.Paintable.snapshot]
 *   will be called.
 *
 * The list of virtual functions for the `GtkSymbolicPaintable` interface.
 * No function must be implemented, default implementations exist for each one.
 */
struct _GtkSymbolicPaintableInterface
{
  /*< private >*/
  GTypeInterface g_iface;

  /*< public >*/
  void                  (* snapshot_symbolic)                           (GtkSymbolicPaintable   *paintable,
                                                                         GdkSnapshot            *snapshot,
                                                                         double                  width,
                                                                         double                  height,
                                                                         const GdkRGBA          *colors,
                                                                         gsize                   n_colors);
};

GDK_AVAILABLE_IN_4_6
void                    gtk_symbolic_paintable_snapshot_symbolic        (GtkSymbolicPaintable   *paintable,
                                                                         GdkSnapshot            *snapshot,
                                                                         double                  width,
                                                                         double                  height,
                                                                         const GdkRGBA          *colors,
                                                                         gsize                   n_colors);

G_END_DECLS

#endif /* __GTK_SYMBOLIC_PAINTABLE_H__ */
