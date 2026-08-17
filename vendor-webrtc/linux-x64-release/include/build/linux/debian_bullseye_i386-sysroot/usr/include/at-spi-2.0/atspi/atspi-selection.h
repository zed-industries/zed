/*
 * AT-SPI - Assistive Technology Service Provider Interface
 * (Gnome Accessibility Project; http://developer.gnome.org/projects/gap)
 *
 * Copyright 2002 Ximian, Inc.
 *           2002 Sun Microsystems Inc.
 * Copyright 2010, 2011 Novell, Inc.
 *           
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef _ATSPI_SELECTION_H_
#define _ATSPI_SELECTION_H_

#include "glib-object.h"

#include "atspi-constants.h"

#include "atspi-types.h"

G_BEGIN_DECLS

#define ATSPI_TYPE_SELECTION                    (atspi_selection_get_type ())
#define ATSPI_IS_SELECTION(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_SELECTION)
#define ATSPI_SELECTION(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_SELECTION, AtspiSelection)
#define ATSPI_SELECTION_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATSPI_TYPE_SELECTION, AtspiSelection))

GType atspi_selection_get_type ();

struct _AtspiSelection
{
  GTypeInterface parent;
};

gint atspi_selection_get_n_selected_children (AtspiSelection *obj, GError **error);

AtspiAccessible * atspi_selection_get_selected_child (AtspiSelection *obj, gint selected_child_index, GError **error);

gboolean atspi_selection_select_child (AtspiSelection *obj, gint child_index, GError **error);

gboolean atspi_selection_deselect_selected_child (AtspiSelection *obj, gint selected_child_index, GError **error);

gboolean atspi_selection_deselect_child (AtspiSelection *obj, gint child_index, GError **error);

gboolean
atspi_selection_is_child_selected (AtspiSelection *obj,
                                   gint child_index, GError **error);

gboolean atspi_selection_select_all (AtspiSelection *obj, GError **error);

gboolean atspi_selection_clear_selection (AtspiSelection *obj, GError **error);

G_END_DECLS

#endif	/* _ATSPI_SELECTION_H_ */
