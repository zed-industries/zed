/* gtkcellrenderer.h
 * Copyright (C) 2000  Red Hat, Inc.,  Jonathan Blandford <jrb@redhat.com>
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

#ifndef __GTK_CELL_RENDERER_H__
#define __GTK_CELL_RENDERER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcelleditable.h>

G_BEGIN_DECLS


/**
 * GtkCellRendererState:
 * @GTK_CELL_RENDERER_SELECTED: The cell is currently selected, and
 *  probably has a selection colored background to render to.
 * @GTK_CELL_RENDERER_PRELIT: The mouse is hovering over the cell.
 * @GTK_CELL_RENDERER_INSENSITIVE: The cell is drawn in an insensitive manner
 * @GTK_CELL_RENDERER_SORTED: The cell is in a sorted row
 * @GTK_CELL_RENDERER_FOCUSED: The cell is in the focus row.
 * @GTK_CELL_RENDERER_EXPANDABLE: The cell is in a row that can be expanded. Since 3.4
 * @GTK_CELL_RENDERER_EXPANDED: The cell is in a row that is expanded. Since 3.4
 *
 * Tells how a cell is to be rendered.
 */
typedef enum
{
  GTK_CELL_RENDERER_SELECTED    = 1 << 0,
  GTK_CELL_RENDERER_PRELIT      = 1 << 1,
  GTK_CELL_RENDERER_INSENSITIVE = 1 << 2,
  /* this flag means the cell is in the sort column/row */
  GTK_CELL_RENDERER_SORTED      = 1 << 3,
  GTK_CELL_RENDERER_FOCUSED     = 1 << 4,
  GTK_CELL_RENDERER_EXPANDABLE  = 1 << 5,
  GTK_CELL_RENDERER_EXPANDED    = 1 << 6
} GtkCellRendererState;

/**
 * GtkCellRendererMode:
 * @GTK_CELL_RENDERER_MODE_INERT: The cell is just for display
 *  and cannot be interacted with.  Note that this doesn’t mean that eg. the
 *  row being drawn can’t be selected -- just that a particular element of
 *  it cannot be individually modified.
 * @GTK_CELL_RENDERER_MODE_ACTIVATABLE: The cell can be clicked.
 * @GTK_CELL_RENDERER_MODE_EDITABLE: The cell can be edited or otherwise modified.
 *
 * Identifies how the user can interact with a particular cell.
 */
typedef enum
{
  GTK_CELL_RENDERER_MODE_INERT,
  GTK_CELL_RENDERER_MODE_ACTIVATABLE,
  GTK_CELL_RENDERER_MODE_EDITABLE
} GtkCellRendererMode;

#define GTK_TYPE_CELL_RENDERER		  (gtk_cell_renderer_get_type ())
#define GTK_CELL_RENDERER(obj)		  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CELL_RENDERER, GtkCellRenderer))
#define GTK_CELL_RENDERER_CLASS(klass)	  (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_CELL_RENDERER, GtkCellRendererClass))
#define GTK_IS_CELL_RENDERER(obj)	  (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CELL_RENDERER))
#define GTK_IS_CELL_RENDERER_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_CELL_RENDERER))
#define GTK_CELL_RENDERER_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_CELL_RENDERER, GtkCellRendererClass))

typedef struct _GtkCellRenderer              GtkCellRenderer;
typedef struct _GtkCellRendererPrivate       GtkCellRendererPrivate;
typedef struct _GtkCellRendererClass         GtkCellRendererClass;
typedef struct _GtkCellRendererClassPrivate  GtkCellRendererClassPrivate;

struct _GtkCellRenderer
{
  GInitiallyUnowned parent_instance;

  /*< private >*/
  GtkCellRendererPrivate *priv;
};

/**
 * GtkCellRendererClass:
 * @get_request_mode: Called to gets whether the cell renderer prefers
 *    a height-for-width layout or a width-for-height layout.
 * @get_preferred_width: Called to get a renderer’s natural width.
 * @get_preferred_height_for_width: Called to get a renderer’s natural height for width.
 * @get_preferred_height: Called to get a renderer’s natural height.
 * @get_preferred_width_for_height: Called to get a renderer’s natural width for height.
 * @get_aligned_area: Called to get the aligned area used by @cell inside @cell_area.
 * @get_size: Called to get the width and height needed to render the cell. Deprecated: 3.0.
 * @render: Called to render the content of the #GtkCellRenderer.
 * @activate: Called to activate the content of the #GtkCellRenderer.
 * @start_editing: Called to initiate editing the content of the #GtkCellRenderer.
 * @editing_canceled: Signal gets emitted when the user cancels the process of editing a cell.
 * @editing_started: Signal gets emitted when a cell starts to be edited.
 */
struct _GtkCellRendererClass
{
  /*< private >*/
  GInitiallyUnownedClass parent_class;

  /*< public >*/

  /* vtable - not signals */
  GtkSizeRequestMode (* get_request_mode)                (GtkCellRenderer      *cell);
  void               (* get_preferred_width)             (GtkCellRenderer      *cell,
                                                          GtkWidget            *widget,
                                                          gint                 *minimum_size,
                                                          gint                 *natural_size);
  void               (* get_preferred_height_for_width)  (GtkCellRenderer      *cell,
                                                          GtkWidget            *widget,
                                                          gint                  width,
                                                          gint                 *minimum_height,
                                                          gint                 *natural_height);
  void               (* get_preferred_height)            (GtkCellRenderer      *cell,
                                                          GtkWidget            *widget,
                                                          gint                 *minimum_size,
                                                          gint                 *natural_size);
  void               (* get_preferred_width_for_height)  (GtkCellRenderer      *cell,
                                                          GtkWidget            *widget,
                                                          gint                  height,
                                                          gint                 *minimum_width,
                                                          gint                 *natural_width);
  void               (* get_aligned_area)                (GtkCellRenderer      *cell,
                                                          GtkWidget            *widget,
							  GtkCellRendererState  flags,
                                                          const GdkRectangle   *cell_area,
                                                          GdkRectangle         *aligned_area);
  void               (* get_size)                        (GtkCellRenderer      *cell,
                                                          GtkWidget            *widget,
                                                          const GdkRectangle   *cell_area,
                                                          gint                 *x_offset,
                                                          gint                 *y_offset,
                                                          gint                 *width,
                                                          gint                 *height);
  void               (* render)                          (GtkCellRenderer      *cell,
                                                          cairo_t              *cr,
                                                          GtkWidget            *widget,
                                                          const GdkRectangle   *background_area,
                                                          const GdkRectangle   *cell_area,
                                                          GtkCellRendererState  flags);
  gboolean           (* activate)                        (GtkCellRenderer      *cell,
                                                          GdkEvent             *event,
                                                          GtkWidget            *widget,
                                                          const gchar          *path,
                                                          const GdkRectangle   *background_area,
                                                          const GdkRectangle   *cell_area,
                                                          GtkCellRendererState  flags);
  GtkCellEditable *  (* start_editing)                   (GtkCellRenderer      *cell,
                                                          GdkEvent             *event,
                                                          GtkWidget            *widget,
                                                          const gchar          *path,
                                                          const GdkRectangle   *background_area,
                                                          const GdkRectangle   *cell_area,
                                                          GtkCellRendererState  flags);

  /* Signals */
  void (* editing_canceled) (GtkCellRenderer *cell);
  void (* editing_started)  (GtkCellRenderer *cell,
			     GtkCellEditable *editable,
			     const gchar     *path);

  /*< private >*/

  GtkCellRendererClassPrivate *priv;

  /* Padding for future expansion */
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType              gtk_cell_renderer_get_type       (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkSizeRequestMode gtk_cell_renderer_get_request_mode               (GtkCellRenderer    *cell);
GDK_AVAILABLE_IN_ALL
void               gtk_cell_renderer_get_preferred_width            (GtkCellRenderer    *cell,
                                                                     GtkWidget          *widget,
                                                                     gint               *minimum_size,
                                                                     gint               *natural_size);
GDK_AVAILABLE_IN_ALL
void               gtk_cell_renderer_get_preferred_height_for_width (GtkCellRenderer    *cell,
                                                                     GtkWidget          *widget,
                                                                     gint                width,
                                                                     gint               *minimum_height,
                                                                     gint               *natural_height);
GDK_AVAILABLE_IN_ALL
void               gtk_cell_renderer_get_preferred_height           (GtkCellRenderer    *cell,
                                                                     GtkWidget          *widget,
                                                                     gint               *minimum_size,
                                                                     gint               *natural_size);
GDK_AVAILABLE_IN_ALL
void               gtk_cell_renderer_get_preferred_width_for_height (GtkCellRenderer    *cell,
                                                                     GtkWidget          *widget,
                                                                     gint                height,
                                                                     gint               *minimum_width,
                                                                     gint               *natural_width);
GDK_AVAILABLE_IN_ALL
void               gtk_cell_renderer_get_preferred_size             (GtkCellRenderer    *cell,
                                                                     GtkWidget          *widget,
                                                                     GtkRequisition     *minimum_size,
                                                                     GtkRequisition     *natural_size);
GDK_AVAILABLE_IN_ALL
void               gtk_cell_renderer_get_aligned_area               (GtkCellRenderer    *cell,
								     GtkWidget          *widget,
								     GtkCellRendererState flags,
								     const GdkRectangle *cell_area,
								     GdkRectangle       *aligned_area);

GDK_DEPRECATED_IN_3_0_FOR(gtk_cell_renderer_get_preferred_size)
void             gtk_cell_renderer_get_size       (GtkCellRenderer      *cell,
                                                   GtkWidget            *widget,
                                                   const GdkRectangle   *cell_area,
                                                   gint                 *x_offset,
                                                   gint                 *y_offset,
                                                   gint                 *width,
                                                   gint                 *height);
GDK_AVAILABLE_IN_ALL
void             gtk_cell_renderer_render         (GtkCellRenderer      *cell,
                                                   cairo_t              *cr,
						   GtkWidget            *widget,
						   const GdkRectangle   *background_area,
						   const GdkRectangle   *cell_area,
						   GtkCellRendererState  flags);
GDK_AVAILABLE_IN_ALL
gboolean         gtk_cell_renderer_activate       (GtkCellRenderer      *cell,
						   GdkEvent             *event,
						   GtkWidget            *widget,
						   const gchar          *path,
						   const GdkRectangle   *background_area,
						   const GdkRectangle   *cell_area,
						   GtkCellRendererState  flags);
GDK_AVAILABLE_IN_ALL
GtkCellEditable *gtk_cell_renderer_start_editing  (GtkCellRenderer      *cell,
						   GdkEvent             *event,
						   GtkWidget            *widget,
						   const gchar          *path,
						   const GdkRectangle   *background_area,
						   const GdkRectangle   *cell_area,
						   GtkCellRendererState  flags);

GDK_AVAILABLE_IN_ALL
void             gtk_cell_renderer_set_fixed_size (GtkCellRenderer      *cell,
						   gint                  width,
						   gint                  height);
GDK_AVAILABLE_IN_ALL
void             gtk_cell_renderer_get_fixed_size (GtkCellRenderer      *cell,
						   gint                 *width,
						   gint                 *height);

GDK_AVAILABLE_IN_ALL
void             gtk_cell_renderer_set_alignment  (GtkCellRenderer      *cell,
                                                   gfloat                xalign,
                                                   gfloat                yalign);
GDK_AVAILABLE_IN_ALL
void             gtk_cell_renderer_get_alignment  (GtkCellRenderer      *cell,
                                                   gfloat               *xalign,
                                                   gfloat               *yalign);

GDK_AVAILABLE_IN_ALL
void             gtk_cell_renderer_set_padding    (GtkCellRenderer      *cell,
                                                   gint                  xpad,
                                                   gint                  ypad);
GDK_AVAILABLE_IN_ALL
void             gtk_cell_renderer_get_padding    (GtkCellRenderer      *cell,
                                                   gint                 *xpad,
                                                   gint                 *ypad);

GDK_AVAILABLE_IN_ALL
void             gtk_cell_renderer_set_visible    (GtkCellRenderer      *cell,
                                                   gboolean              visible);
GDK_AVAILABLE_IN_ALL
gboolean         gtk_cell_renderer_get_visible    (GtkCellRenderer      *cell);

GDK_AVAILABLE_IN_ALL
void             gtk_cell_renderer_set_sensitive  (GtkCellRenderer      *cell,
                                                   gboolean              sensitive);
GDK_AVAILABLE_IN_ALL
gboolean         gtk_cell_renderer_get_sensitive  (GtkCellRenderer      *cell);

GDK_AVAILABLE_IN_ALL
gboolean         gtk_cell_renderer_is_activatable (GtkCellRenderer      *cell);

/* For use by cell renderer implementations only */
GDK_AVAILABLE_IN_ALL
void             gtk_cell_renderer_stop_editing   (GtkCellRenderer      *cell,
                                                   gboolean              canceled);


void            _gtk_cell_renderer_calc_offset    (GtkCellRenderer      *cell,
                                                   const GdkRectangle   *cell_area,
                                                   GtkTextDirection      direction,
                                                   gint                  width,
                                                   gint                  height,
                                                   gint                 *x_offset,
                                                   gint                 *y_offset);

GDK_AVAILABLE_IN_ALL
GtkStateFlags   gtk_cell_renderer_get_state       (GtkCellRenderer      *cell,
                                                   GtkWidget            *widget,
                                                   GtkCellRendererState  cell_state);

GDK_AVAILABLE_IN_ALL
void            gtk_cell_renderer_class_set_accessible_type 
                                                  (GtkCellRendererClass *renderer_class,
                                                   GType                 type);
GType           _gtk_cell_renderer_get_accessible_type
                                                  (GtkCellRenderer *     renderer);

G_END_DECLS

#endif /* __GTK_CELL_RENDERER_H__ */
