/* gtklayoutmanager.h: Layout manager base class
 * Copyright 2019  The GNOME Foundation
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
 * Author: Emmanuele Bassi
 */
#pragma once

#include <gsk/gsk.h>
#include <gtk/gtktypes.h>
#include <gtk/gtkwidget.h>
#include <gtk/gtklayoutchild.h>

G_BEGIN_DECLS

#define GTK_TYPE_LAYOUT_MANAGER (gtk_layout_manager_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_DERIVABLE_TYPE (GtkLayoutManager, gtk_layout_manager, GTK, LAYOUT_MANAGER, GObject)

/**
 * GtkLayoutManagerClass:
 * @get_request_mode: a virtual function, used to return the preferred
 *   request mode for the layout manager; for instance, "width for height"
 *   or "height for width"; see `GtkSizeRequestMode`
 * @measure: a virtual function, used to measure the minimum and preferred
 *   sizes of the widget using the layout manager for a given orientation
 * @allocate: a virtual function, used to allocate the size of the widget
 *   using the layout manager
 * @layout_child_type: the type of `GtkLayoutChild` used by this layout manager
 * @create_layout_child: a virtual function, used to create a `GtkLayoutChild`
 *   meta object for the layout properties
 * @root: a virtual function, called when the widget using the layout
 *   manager is attached to a `GtkRoot`
 * @unroot: a virtual function, called when the widget using the layout
 *   manager is detached from a `GtkRoot`
 *
 * The `GtkLayoutManagerClass` structure contains only private data, and
 * should only be accessed through the provided API, or when subclassing
 * `GtkLayoutManager`.
 */
struct _GtkLayoutManagerClass
{
  /*< private >*/
  GObjectClass parent_class;

  /*< public >*/
  GtkSizeRequestMode (* get_request_mode)    (GtkLayoutManager *manager,
                                              GtkWidget        *widget);

  void               (* measure)             (GtkLayoutManager *manager,
                                              GtkWidget        *widget,
                                              GtkOrientation    orientation,
                                              int               for_size,
                                              int              *minimum,
                                              int              *natural,
                                              int              *minimum_baseline,
                                              int              *natural_baseline);

  void               (* allocate)            (GtkLayoutManager *manager,
                                              GtkWidget        *widget,
                                              int               width,
                                              int               height,
                                              int               baseline);

  GType              layout_child_type;

  /**
   * GtkLayoutManagerClass::create_layout_child:
   * @manager: the `GtkLayoutManager`
   * @widget: the widget using the @manager
   * @for_child: the child of @widget
   *
   * Create a `GtkLayoutChild` instance for the given @for_child widget.
   *
   * Returns: (transfer full): a `GtkLayoutChild`
   */
  GtkLayoutChild *   (* create_layout_child) (GtkLayoutManager *manager,
                                              GtkWidget        *widget,
                                              GtkWidget        *for_child);

  void               (* root)                (GtkLayoutManager *manager);
  void               (* unroot)              (GtkLayoutManager *manager);

  /*< private >*/
  gpointer _padding[16];
};

GDK_AVAILABLE_IN_ALL
void                    gtk_layout_manager_measure              (GtkLayoutManager *manager,
                                                                 GtkWidget        *widget,
                                                                 GtkOrientation    orientation,
                                                                 int               for_size,
                                                                 int              *minimum,
                                                                 int              *natural,
                                                                 int              *minimum_baseline,
                                                                 int              *natural_baseline);
GDK_AVAILABLE_IN_ALL
void                    gtk_layout_manager_allocate             (GtkLayoutManager *manager,
                                                                 GtkWidget        *widget,
                                                                 int               width,
                                                                 int               height,
                                                                 int               baseline);
GDK_AVAILABLE_IN_ALL
GtkSizeRequestMode      gtk_layout_manager_get_request_mode     (GtkLayoutManager *manager);

GDK_AVAILABLE_IN_ALL
GtkWidget *             gtk_layout_manager_get_widget           (GtkLayoutManager *manager);

GDK_AVAILABLE_IN_ALL
void                    gtk_layout_manager_layout_changed       (GtkLayoutManager *manager);

GDK_AVAILABLE_IN_ALL
GtkLayoutChild *        gtk_layout_manager_get_layout_child     (GtkLayoutManager *manager,
                                                                 GtkWidget        *child);

G_END_DECLS
