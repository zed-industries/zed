/*
 * AT-SPI - Assistive Technology Service Provider Interface
 * (Gnome Accessibility Project; http://developer.gnome.org/projects/gap)
 *
 * Copyright 2002 Ximian, Inc.
 *           2002 Sun Microsystems Inc.
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

#ifndef _ATSPI_ACTION_H_
#define _ATSPI_ACTION_H_

#include "glib-object.h"

#include "atspi-constants.h"

#include "atspi-types.h"

G_BEGIN_DECLS

#define ATSPI_TYPE_ACTION                    (atspi_action_get_type ())
#define ATSPI_IS_ACTION(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_ACTION)
#define ATSPI_ACTION(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_ACTION, AtspiAction)
#define ATSPI_ACTION_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATSPI_TYPE_ACTION, AtspiAction))

GType atspi_action_get_type ();

struct _AtspiAction
{
  GTypeInterface parent;
};

gchar * atspi_action_get_action_description (AtspiAction *obj, int i, GError **error);

gchar * atspi_action_get_action_name (AtspiAction *obj, gint i, GError **error);

gint atspi_action_get_n_actions (AtspiAction *obj, GError **error);

gchar * atspi_action_get_key_binding (AtspiAction *obj, gint i, GError **error);


gchar * atspi_action_get_localized_name (AtspiAction *obj, gint i, GError **error);

gboolean atspi_action_do_action (AtspiAction *obj, gint i, GError **error);

#ifndef ATSPI_DISABLE_DEPRECATED
gchar * atspi_action_get_description (AtspiAction *obj, gint i, GError **error);
gchar * atspi_action_get_name (AtspiAction *obj, gint i, GError **error);
#endif
G_END_DECLS

#endif	/* _ATSPI_ACTION_H_ */
