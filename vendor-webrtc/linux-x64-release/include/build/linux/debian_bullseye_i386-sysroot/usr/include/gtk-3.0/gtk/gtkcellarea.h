/* gtkcellarea.h
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

#ifndef __GTK_CELL_AREA_H__
#define __GTK_CELL_AREA_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcellrenderer.h>
#include <gtk/gtkwidget.h>
#include <gtk/gtktreemodel.h>

G_BEGIN_DECLS

#define GTK_TYPE_CELL_AREA                (gtk_cell_area_get_type ())
#define GTK_CELL_AREA(obj)                (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CELL_AREA, GtkCellArea))
#define GTK_CELL_AREA_CLASS(klass)        (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_CELL_AREA, GtkCellAreaClass))
#define GTK_IS_CELL_AREA(obj)     (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CELL_AREA))
#define GTK_IS_CELL_AREA_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_CELL_AREA))
#define GTK_CELL_AREA_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_CELL_AREA, GtkCellAreaClass))

typedef struct _GtkCellArea              GtkCellArea;
typedef struct _GtkCellAreaClass         GtkCellAreaClass;
typedef struct _GtkCellAreaPrivate       GtkCellAreaPrivate;
typedef struct _GtkCellAreaContext       GtkCellAreaContext;

/**
 * GTK_CELL_AREA_WARN_INVALID_CELL_PROPERTY_ID:
 * @object: the #GObject on which set_cell_property() or get_cell_property()
 *     was called
 * @property_id: the numeric id of the property
 * @pspec: the #GParamSpec of the property
 *
 * This macro should be used to emit a standard warning about unexpected
 * properties in set_cell_property() and get_cell_property() implementations.
 */
#define GTK_CELL_AREA_WARN_INVALID_CELL_PROPERTY_ID(object, property_id, pspec) \
  G_OBJECT_WARN_INVALID_PSPEC ((object), "cell property id", (property_id), (pspec))

/**
 * GtkCellCallback:
 * @renderer: the cell renderer to operate on
 * @data: (closure): user-supplied data
 *
 * The type of the callback functions used for iterating over
 * the cell renderers of a #GtkCellArea, see gtk_cell_area_foreach().
 *
 * Returns: %TRUE to stop iterating over cells.
 */
typedef gboolean    (*GtkCellCallback) (GtkCellRenderer  *renderer,
                                        gpointer          data);

/**
 * GtkCellAllocCallback:
 * @renderer: the cell renderer to operate on
 * @cell_area: the area allocated to @renderer inside the rectangle
 *     provided to gtk_cell_area_foreach_alloc().
 * @cell_background: the background area for @renderer inside the
 *     background area provided to gtk_cell_area_foreach_alloc().
 * @data: (closure): user-supplied data
 *
 * The type of the callback functions used for iterating over the
 * cell renderers and their allocated areas inside a #GtkCellArea,
 * see gtk_cell_area_foreach_alloc().
 *
 * Returns: %TRUE to stop iterating over cells.
 */
typedef gboolean    (*GtkCellAllocCallback) (GtkCellRenderer    *renderer,
                                             const GdkRectangle *cell_area,
                                             const GdkRectangle *cell_background,
                                             gpointer            data);


struct _GtkCellArea
{
  /*< private >*/
  GInitiallyUnowned parent_instance;

  GtkCellAreaPrivate *priv;
};


/**
 * GtkCellAreaClass:
 * @add: adds a #GtkCellRenderer to the area.
 * @remove: removes a #GtkCellRenderer from the area.
 * @foreach: calls the #GtkCellCallback function on every #GtkCellRenderer in
 *     the area with the provided user data until the callback returns %TRUE.
 * @foreach_alloc: Calls the #GtkCellAllocCallback function on every
 *     #GtkCellRenderer in the area with the allocated area for the cell
 *     and the provided user data until the callback returns %TRUE.
 * @event: Handle an event in the area, this is generally used to activate
 *     a cell at the event location for button events but can also be used
 *     to generically pass events to #GtkWidgets drawn onto the area.
 * @render: Actually render the area’s cells to the specified rectangle,
 *     @background_area should be correctly distributed to the cells
 *     corresponding background areas.
 * @apply_attributes: Apply the cell attributes to the cells. This is
 *     implemented as a signal and generally #GtkCellArea subclasses don't
 *     need to implement it since it is handled by the base class.
 * @create_context: Creates and returns a class specific #GtkCellAreaContext
 *     to store cell alignment and allocation details for a said #GtkCellArea
 *     class.
 * @copy_context: Creates a new #GtkCellAreaContext in the same state as
 *     the passed @context with any cell alignment data and allocations intact.
 * @get_request_mode: This allows an area to tell its layouting widget whether
 *     it prefers to be allocated in %GTK_SIZE_REQUEST_HEIGHT_FOR_WIDTH or
 *     %GTK_SIZE_REQUEST_WIDTH_FOR_HEIGHT mode.
 * @get_preferred_width: Calculates the minimum and natural width of the
 *     areas cells with the current attributes applied while considering
 *     the particular layouting details of the said #GtkCellArea. While
 *     requests are performed over a series of rows, alignments and overall
 *     minimum and natural sizes should be stored in the corresponding
 *     #GtkCellAreaContext.
 * @get_preferred_height_for_width: Calculates the minimum and natural height
 *     for the area if the passed @context would be allocated the given width.
 *     When implementing this virtual method it is safe to assume that @context
 *     has already stored the aligned cell widths for every #GtkTreeModel row
 *     that @context will be allocated for since this information was stored
 *     at #GtkCellAreaClass.get_preferred_width() time. This virtual method
 *     should also store any necessary alignments of cell heights for the
 *     case that the context is allocated a height.
 * @get_preferred_height: Calculates the minimum and natural height of the
 *     areas cells with the current attributes applied. Essentially this is
 *     the same as #GtkCellAreaClass.get_preferred_width() only for areas
 *     that are being requested as %GTK_SIZE_REQUEST_WIDTH_FOR_HEIGHT.
 * @get_preferred_width_for_height: Calculates the minimum and natural width
 *     for the area if the passed @context would be allocated the given
 *     height. The same as #GtkCellAreaClass.get_preferred_height_for_width()
 *     only for handling requests in the %GTK_SIZE_REQUEST_WIDTH_FOR_HEIGHT
 *     mode.
 * @set_cell_property: This should be implemented to handle changes in child
 *     cell properties for a given #GtkCellRenderer that were previously
 *     installed on the #GtkCellAreaClass with gtk_cell_area_class_install_cell_property().
 * @get_cell_property: This should be implemented to report the values of
 *     child cell properties for a given child #GtkCellRenderer.
 * @focus: This virtual method should be implemented to navigate focus from
 *     cell to cell inside the #GtkCellArea. The #GtkCellArea should move
 *     focus from cell to cell inside the area and return %FALSE if focus
 *     logically leaves the area with the following exceptions: When the
 *     area contains no activatable cells, the entire area recieves focus.
 *     Focus should not be given to cells that are actually “focus siblings”
 *     of other sibling cells (see gtk_cell_area_get_focus_from_sibling()).
 *     Focus is set by calling gtk_cell_area_set_focus_cell().
 * @is_activatable: Returns whether the #GtkCellArea can respond to
 *     #GtkCellAreaClass.activate(), usually this does not need to be
 *     implemented since the base class takes care of this however it can
 *     be enhanced if the #GtkCellArea subclass can handle activation in
 *     other ways than activating its #GtkCellRenderers.
 * @activate: This is called when the layouting widget rendering the
 *     #GtkCellArea activates the focus cell (see gtk_cell_area_get_focus_cell()).
 */
struct _GtkCellAreaClass
{
  /*< private >*/
  GInitiallyUnownedClass parent_class;

  /*< public >*/

  /* Basic methods */
  void               (* add)                             (GtkCellArea             *area,
                                                          GtkCellRenderer         *renderer);
  void               (* remove)                          (GtkCellArea             *area,
                                                          GtkCellRenderer         *renderer);
  void               (* foreach)                         (GtkCellArea             *area,
                                                          GtkCellCallback          callback,
                                                          gpointer                 callback_data);
  void               (* foreach_alloc)                   (GtkCellArea             *area,
                                                          GtkCellAreaContext      *context,
                                                          GtkWidget               *widget,
                                                          const GdkRectangle      *cell_area,
                                                          const GdkRectangle      *background_area,
                                                          GtkCellAllocCallback     callback,
                                                          gpointer                 callback_data);
  gint               (* event)                           (GtkCellArea             *area,
                                                          GtkCellAreaContext      *context,
                                                          GtkWidget               *widget,
                                                          GdkEvent                *event,
                                                          const GdkRectangle      *cell_area,
                                                          GtkCellRendererState     flags);
  void               (* render)                          (GtkCellArea             *area,
                                                          GtkCellAreaContext      *context,
                                                          GtkWidget               *widget,
                                                          cairo_t                 *cr,
                                                          const GdkRectangle      *background_area,
                                                          const GdkRectangle      *cell_area,
                                                          GtkCellRendererState     flags,
                                                          gboolean                 paint_focus);
  void               (* apply_attributes)                (GtkCellArea             *area,
                                                          GtkTreeModel            *tree_model,
                                                          GtkTreeIter             *iter,
                                                          gboolean                 is_expander,
                                                          gboolean                 is_expanded);

  /* Geometry */
  GtkCellAreaContext *(* create_context)                 (GtkCellArea             *area);
  GtkCellAreaContext *(* copy_context)                   (GtkCellArea             *area,
                                                          GtkCellAreaContext      *context);
  GtkSizeRequestMode (* get_request_mode)                (GtkCellArea             *area);
  void               (* get_preferred_width)             (GtkCellArea             *area,
                                                          GtkCellAreaContext      *context,
                                                          GtkWidget               *widget,
                                                          gint                    *minimum_width,
                                                          gint                    *natural_width);
  void               (* get_preferred_height_for_width)  (GtkCellArea             *area,
                                                          GtkCellAreaContext      *context,
                                                          GtkWidget               *widget,
                                                          gint                     width,
                                                          gint                    *minimum_height,
                                                          gint                    *natural_height);
  void               (* get_preferred_height)            (GtkCellArea             *area,
                                                          GtkCellAreaContext      *context,
                                                          GtkWidget               *widget,
                                                          gint                    *minimum_height,
                                                          gint                    *natural_height);
  void               (* get_preferred_width_for_height)  (GtkCellArea             *area,
                                                          GtkCellAreaContext      *context,
                                                          GtkWidget               *widget,
                                                          gint                     height,
                                                          gint                    *minimum_width,
                                                          gint                    *natural_width);

  /* Cell Properties */
  void               (* set_cell_property)               (GtkCellArea             *area,
                                                          GtkCellRenderer         *renderer,
                                                          guint                    property_id,
                                                          const GValue            *value,
                                                          GParamSpec              *pspec);
  void               (* get_cell_property)               (GtkCellArea             *area,
                                                          GtkCellRenderer         *renderer,
                                                          guint                    property_id,
                                                          GValue                  *value,
                                                          GParamSpec              *pspec);

  /* Focus */
  gboolean           (* focus)                           (GtkCellArea             *area,
                                                          GtkDirectionType         direction);
  gboolean           (* is_activatable)                  (GtkCellArea             *area);
  gboolean           (* activate)                        (GtkCellArea             *area,
                                                          GtkCellAreaContext      *context,
                                                          GtkWidget               *widget,
                                                          const GdkRectangle      *cell_area,
                                                          GtkCellRendererState     flags,
                                                          gboolean                 edit_only);

  /*< private >*/

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
GType                 gtk_cell_area_get_type                       (void) G_GNUC_CONST;

/* Basic methods */
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_add                            (GtkCellArea          *area,
                                                                    GtkCellRenderer      *renderer);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_remove                         (GtkCellArea          *area,
                                                                    GtkCellRenderer      *renderer);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_cell_area_has_renderer                   (GtkCellArea          *area,
                                                                    GtkCellRenderer      *renderer);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_foreach                        (GtkCellArea          *area,
                                                                    GtkCellCallback       callback,
                                                                    gpointer              callback_data);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_foreach_alloc                  (GtkCellArea          *area,
                                                                    GtkCellAreaContext   *context,
                                                                    GtkWidget            *widget,
                                                                    const GdkRectangle   *cell_area,
                                                                    const GdkRectangle   *background_area,
                                                                    GtkCellAllocCallback  callback,
                                                                    gpointer              callback_data);
GDK_AVAILABLE_IN_ALL
gint                  gtk_cell_area_event                          (GtkCellArea          *area,
                                                                    GtkCellAreaContext   *context,
                                                                    GtkWidget            *widget,
                                                                    GdkEvent             *event,
                                                                    const GdkRectangle   *cell_area,
                                                                    GtkCellRendererState  flags);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_render                         (GtkCellArea          *area,
                                                                    GtkCellAreaContext   *context,
                                                                    GtkWidget            *widget,
                                                                    cairo_t              *cr,
                                                                    const GdkRectangle   *background_area,
                                                                    const GdkRectangle   *cell_area,
                                                                    GtkCellRendererState  flags,
                                                                    gboolean              paint_focus);

GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_get_cell_allocation            (GtkCellArea          *area,
                                                                    GtkCellAreaContext   *context,
                                                                    GtkWidget            *widget,
                                                                    GtkCellRenderer      *renderer,
                                                                    const GdkRectangle   *cell_area,
                                                                    GdkRectangle         *allocation);
GDK_AVAILABLE_IN_ALL
GtkCellRenderer      *gtk_cell_area_get_cell_at_position           (GtkCellArea          *area,
                                                                    GtkCellAreaContext   *context,
                                                                    GtkWidget            *widget,
                                                                    const GdkRectangle   *cell_area,
                                                                    gint                  x,
                                                                    gint                  y,
                                                                    GdkRectangle         *alloc_area);

/* Geometry */
GDK_AVAILABLE_IN_ALL
GtkCellAreaContext   *gtk_cell_area_create_context                 (GtkCellArea        *area);
GDK_AVAILABLE_IN_ALL
GtkCellAreaContext   *gtk_cell_area_copy_context                   (GtkCellArea        *area,
                                                                    GtkCellAreaContext *context);
GDK_AVAILABLE_IN_ALL
GtkSizeRequestMode    gtk_cell_area_get_request_mode               (GtkCellArea        *area);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_get_preferred_width            (GtkCellArea        *area,
                                                                    GtkCellAreaContext *context,
                                                                    GtkWidget          *widget,
                                                                    gint               *minimum_width,
                                                                    gint               *natural_width);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_get_preferred_height_for_width (GtkCellArea        *area,
                                                                    GtkCellAreaContext *context,
                                                                    GtkWidget          *widget,
                                                                    gint                width,
                                                                    gint               *minimum_height,
                                                                    gint               *natural_height);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_get_preferred_height           (GtkCellArea        *area,
                                                                    GtkCellAreaContext *context,
                                                                    GtkWidget          *widget,
                                                                    gint               *minimum_height,
                                                                    gint               *natural_height);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_get_preferred_width_for_height (GtkCellArea        *area,
                                                                    GtkCellAreaContext *context,
                                                                    GtkWidget          *widget,
                                                                    gint                height,
                                                                    gint               *minimum_width,
                                                                    gint               *natural_width);
GDK_AVAILABLE_IN_ALL
const gchar *         gtk_cell_area_get_current_path_string        (GtkCellArea        *area);


/* Attributes */
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_apply_attributes               (GtkCellArea        *area,
                                                                    GtkTreeModel       *tree_model,
                                                                    GtkTreeIter        *iter,
                                                                    gboolean            is_expander,
                                                                    gboolean            is_expanded);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_attribute_connect              (GtkCellArea        *area,
                                                                    GtkCellRenderer    *renderer,
                                                                    const gchar        *attribute,
                                                                    gint                column);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_attribute_disconnect           (GtkCellArea        *area,
                                                                    GtkCellRenderer    *renderer,
                                                                    const gchar        *attribute);
GDK_AVAILABLE_IN_3_14
gint                  gtk_cell_area_attribute_get_column           (GtkCellArea        *area,
                                                                    GtkCellRenderer    *renderer,
                                                                    const gchar        *attribute);


/* Cell Properties */
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_class_install_cell_property    (GtkCellAreaClass   *aclass,
                                                                    guint               property_id,
                                                                    GParamSpec         *pspec);
GDK_AVAILABLE_IN_ALL
GParamSpec*           gtk_cell_area_class_find_cell_property       (GtkCellAreaClass   *aclass,
                                                                    const gchar        *property_name);
GDK_AVAILABLE_IN_ALL
GParamSpec**          gtk_cell_area_class_list_cell_properties     (GtkCellAreaClass   *aclass,
                                                                    guint                   *n_properties);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_add_with_properties            (GtkCellArea        *area,
                                                                    GtkCellRenderer    *renderer,
                                                                    const gchar     *first_prop_name,
                                                                    ...) G_GNUC_NULL_TERMINATED;
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_cell_set                       (GtkCellArea        *area,
                                                                    GtkCellRenderer    *renderer,
                                                                    const gchar        *first_prop_name,
                                                                    ...) G_GNUC_NULL_TERMINATED;
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_cell_get                       (GtkCellArea        *area,
                                                                    GtkCellRenderer    *renderer,
                                                                    const gchar        *first_prop_name,
                                                                    ...) G_GNUC_NULL_TERMINATED;
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_cell_set_valist                (GtkCellArea        *area,
                                                                    GtkCellRenderer    *renderer,
                                                                    const gchar        *first_property_name,
                                                                    va_list             var_args);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_cell_get_valist                (GtkCellArea        *area,
                                                                    GtkCellRenderer    *renderer,
                                                                    const gchar        *first_property_name,
                                                                    va_list             var_args);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_cell_set_property              (GtkCellArea        *area,
                                                                    GtkCellRenderer    *renderer,
                                                                    const gchar        *property_name,
                                                                    const GValue       *value);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_cell_get_property              (GtkCellArea        *area,
                                                                    GtkCellRenderer    *renderer,
                                                                    const gchar        *property_name,
                                                                    GValue             *value);

/* Focus */
GDK_AVAILABLE_IN_ALL
gboolean              gtk_cell_area_is_activatable                 (GtkCellArea         *area);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_cell_area_activate                       (GtkCellArea         *area,
                                                                    GtkCellAreaContext  *context,
                                                                    GtkWidget           *widget,
                                                                    const GdkRectangle  *cell_area,
                                                                    GtkCellRendererState flags,
                                                                    gboolean             edit_only);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_cell_area_focus                          (GtkCellArea         *area,
                                                                    GtkDirectionType     direction);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_set_focus_cell                 (GtkCellArea          *area,
                                                                    GtkCellRenderer      *renderer);
GDK_AVAILABLE_IN_ALL
GtkCellRenderer      *gtk_cell_area_get_focus_cell                 (GtkCellArea          *area);


/* Focus siblings */
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_add_focus_sibling              (GtkCellArea          *area,
                                                                    GtkCellRenderer      *renderer,
                                                                    GtkCellRenderer      *sibling);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_remove_focus_sibling           (GtkCellArea          *area,
                                                                    GtkCellRenderer      *renderer,
                                                                    GtkCellRenderer      *sibling);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_cell_area_is_focus_sibling               (GtkCellArea          *area,
                                                                    GtkCellRenderer      *renderer,
                                                                    GtkCellRenderer      *sibling);
GDK_AVAILABLE_IN_ALL
const GList *         gtk_cell_area_get_focus_siblings             (GtkCellArea          *area,
                                                                    GtkCellRenderer      *renderer);
GDK_AVAILABLE_IN_ALL
GtkCellRenderer      *gtk_cell_area_get_focus_from_sibling         (GtkCellArea          *area,
                                                                    GtkCellRenderer      *renderer);

/* Cell Activation/Editing */
GDK_AVAILABLE_IN_ALL
GtkCellRenderer      *gtk_cell_area_get_edited_cell                (GtkCellArea          *area);
GDK_AVAILABLE_IN_ALL
GtkCellEditable      *gtk_cell_area_get_edit_widget                (GtkCellArea          *area);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_cell_area_activate_cell                  (GtkCellArea          *area,
                                                                    GtkWidget            *widget,
                                                                    GtkCellRenderer      *renderer,
                                                                    GdkEvent             *event,
                                                                    const GdkRectangle   *cell_area,
                                                                    GtkCellRendererState  flags);
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_stop_editing                   (GtkCellArea          *area,
                                                                    gboolean              canceled);

/* Functions for area implementations */

/* Distinguish the inner cell area from the whole requested area including margins */
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_inner_cell_area                (GtkCellArea        *area,
                                                                    GtkWidget          *widget,
                                                                    const GdkRectangle *cell_area,
                                                                    GdkRectangle       *inner_area);

/* Request the size of a cell while respecting the cell margins (requests are margin inclusive) */
GDK_AVAILABLE_IN_ALL
void                  gtk_cell_area_request_renderer               (GtkCellArea        *area,
                                                                    GtkCellRenderer    *renderer,
                                                                    GtkOrientation      orientation,
                                                                    GtkWidget          *widget,
                                                                    gint                for_size,
                                                                    gint               *minimum_size,
                                                                    gint               *natural_size);

/* For api stability, this is called from gtkcelllayout.c in order to ensure the correct
 * object is passed to the user function in gtk_cell_layout_set_cell_data_func.
 *
 * This private api takes gpointer & GFunc arguments to circumvent circular header file
 * dependancies.
 */
void                 _gtk_cell_area_set_cell_data_func_with_proxy  (GtkCellArea           *area,
								    GtkCellRenderer       *cell,
								    GFunc                  func,
								    gpointer               func_data,
								    GDestroyNotify         destroy,
								    gpointer               proxy);

G_END_DECLS

#endif /* __GTK_CELL_AREA_H__ */
