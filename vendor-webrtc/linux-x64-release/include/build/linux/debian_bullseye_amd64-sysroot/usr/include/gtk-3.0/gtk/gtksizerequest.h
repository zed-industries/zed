/* GTK - The GIMP Toolkit
 * Copyright (C) 2007-2010 Openismus GmbH
 *
 * Authors:
 *      Mathias Hasselmann <mathias@openismus.com>
 *      Tristan Van Berkom <tristan.van.berkom@gmail.com>
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

#ifndef __GTK_SIZE_REQUEST_H__
#define __GTK_SIZE_REQUEST_H__

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

typedef struct _GtkRequestedSize         GtkRequestedSize;

/**
 * GtkRequestedSize:
 * @data: A client pointer
 * @minimum_size: The minimum size needed for allocation in a given orientation
 * @natural_size: The natural size for allocation in a given orientation
 *
 * Represents a request of a screen object in a given orientation. These
 * are primarily used in container implementations when allocating a natural
 * size for children calling. See gtk_distribute_natural_allocation().
 */
struct _GtkRequestedSize
{
  gpointer data;
  gint     minimum_size;
  gint     natural_size;
};


/* General convenience function to aid in allocating natural sizes */
GDK_AVAILABLE_IN_ALL
gint                gtk_distribute_natural_allocation               (gint              extra_space,
                                                                     guint             n_requested_sizes,
                                                                     GtkRequestedSize *sizes);


G_END_DECLS

#endif /* __GTK_SIZE_REQUEST_H__ */
