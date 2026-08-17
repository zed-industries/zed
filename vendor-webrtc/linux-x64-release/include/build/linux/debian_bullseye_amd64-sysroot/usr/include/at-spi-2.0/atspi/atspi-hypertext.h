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

#ifndef _ATSPI_HYPERTEXT_H_
#define _ATSPI_HYPERTEXT_H_

#include "glib-object.h"

#include "atspi-constants.h"

#include "atspi-types.h"

G_BEGIN_DECLS

#define ATSPI_TYPE_HYPERTEXT                    (atspi_hypertext_get_type ())
#define ATSPI_IS_HYPERTEXT(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_HYPERTEXT)
#define ATSPI_HYPERTEXT(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_HYPERTEXT, AtspiHypertext)
#define ATSPI_HYPERTEXT_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATSPI_TYPE_HYPERTEXT, AtspiHypertext))

GType atspi_hypertext_get_type ();

struct _AtspiHypertext
{
  GTypeInterface parent;
};

gint atspi_hypertext_get_n_links (AtspiHypertext *obj, GError **error);

AtspiHyperlink * atspi_hypertext_get_link (AtspiHypertext *obj, gint link_index, GError **error);

gint atspi_hypertext_get_link_index (AtspiHypertext *obj, gint             character_offset, GError **error);

G_END_DECLS

#endif	/* _ATSPI_HYPERTEXT_H_ */
