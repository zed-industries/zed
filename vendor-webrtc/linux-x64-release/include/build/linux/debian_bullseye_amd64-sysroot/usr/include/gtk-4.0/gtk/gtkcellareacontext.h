/* gtkcellareacontext.h
 *
 * Copyright (C) 2010 Openismus GmbH
 *
 * Authors:
 *      Tristan Van Berkom <tristanvb@openismus.com>
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
 */

#ifndef __GTK_CELL_AREA_CONTEXT_H__
#define __GTK_CELL_AREA_CONTEXT_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcellarea.h>

G_BEGIN_DECLS

#define GTK_TYPE_CELL_AREA_CONTEXT            (gtk_cell_area_context_get_type ())
#define GTK_CELL_AREA_CONTEXT(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CELL_AREA_CONTEXT, GtkCellAreaContext))
#define GTK_CELL_AREA_CONTEXT_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_CELL_AREA_CONTEXT, GtkCellAreaContextClass))
#define GTK_IS_CELL_AREA_CONTEXT(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CELL_AREA_CONTEXT))
#define GTK_IS_CELL_AREA_CONTEXT_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_CELL_AREA_CONTEXT))
#define GTK_CELL_AREA_CONTEXT_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_CELL_AREA_CONTEXT, GtkCellAreaContextClass))

typedef struct _GtkCellAreaContextPrivate       GtkCellAreaContextPrivate;
typedef struct _GtkCellAreaContextClass         GtkCellAreaContextClass;

struct _GtkCellAreaContext
{
  /*< private >*/
  GObject parent_instance;
};

/**
 * GtkCellAreaContextClass:
 * @allocate: This tells the context that an allocation width or height
 *   (or both) have been decided for a group of rows. The context should
 *   store any allocations for internally aligned cells at this point so
 *   that they dont need to be recalculated at gtk_cell_area_render() time.
 * @reset: Clear any previously stored information about requested and
 *   allocated sizes for the context.
 * @get_preferred_height_for_width: Returns the aligned height for the given
 *   width that context must store while collecting sizes for it’s rows.
 * @get_preferred_width_for_height: Returns the aligned width for the given
 *   height that context must store while collecting sizes for it’s rows.
 */
struct _GtkCellAreaContextClass
{
  /*< private >*/
  GObjectClass parent_class;

  /*< public >*/
  void    (* allocate)                       (GtkCellAreaContext *context,
                                              int                 width,
                                              int                 height);
  void    (* reset)                          (GtkCellAreaContext *context);
  void    (* get_preferred_height_for_width) (GtkCellAreaContext *context,
                                              int                 width,
                                              int                *minimum_height,
                                              int                *natural_height);
  void    (* get_preferred_width_for_height) (GtkCellAreaContext *context,
                                              int                 height,
                                              int                *minimum_width,
                                              int                *natural_width);

  /*< private >*/

  gpointer padding[8];
};

GDK_AVAILABLE_IN_ALL
GType        gtk_cell_area_context_get_type              (void) G_GNUC_CONST;

/* Main apis */
GDK_AVAILABLE_IN_ALL
GtkCellArea *gtk_cell_area_context_get_area                        (GtkCellAreaContext *context);
GDK_AVAILABLE_IN_ALL
void         gtk_cell_area_context_allocate                        (GtkCellAreaContext *context,
                                                                    int                 width,
                                                                    int                 height);
GDK_AVAILABLE_IN_ALL
void         gtk_cell_area_context_reset                           (GtkCellAreaContext *context);

/* Apis for GtkCellArea clients to consult cached values
 * for a series of GtkTreeModel rows
 */
GDK_AVAILABLE_IN_ALL
void         gtk_cell_area_context_get_preferred_width            (GtkCellAreaContext *context,
                                                                   int                *minimum_width,
                                                                   int                *natural_width);
GDK_AVAILABLE_IN_ALL
void         gtk_cell_area_context_get_preferred_height           (GtkCellAreaContext *context,
                                                                   int                *minimum_height,
                                                                   int                *natural_height);
GDK_AVAILABLE_IN_ALL
void         gtk_cell_area_context_get_preferred_height_for_width (GtkCellAreaContext *context,
                                                                   int                 width,
                                                                   int                *minimum_height,
                                                                   int                *natural_height);
GDK_AVAILABLE_IN_ALL
void         gtk_cell_area_context_get_preferred_width_for_height (GtkCellAreaContext *context,
                                                                   int                 height,
                                                                   int                *minimum_width,
                                                                   int                *natural_width);
GDK_AVAILABLE_IN_ALL
void         gtk_cell_area_context_get_allocation                 (GtkCellAreaContext *context,
                                                                   int                *width,
                                                                   int                *height);

/* Apis for GtkCellArea implementations to update cached values
 * for multiple GtkTreeModel rows
 */
GDK_AVAILABLE_IN_ALL
void         gtk_cell_area_context_push_preferred_width  (GtkCellAreaContext *context,
                                                          int                 minimum_width,
                                                          int                 natural_width);
GDK_AVAILABLE_IN_ALL
void         gtk_cell_area_context_push_preferred_height (GtkCellAreaContext *context,
                                                          int                 minimum_height,
                                                          int                 natural_height);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkCellAreaContext, g_object_unref)

G_END_DECLS

#endif /* __GTK_CELL_AREA_CONTEXT_H__ */
