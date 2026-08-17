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

#ifndef _ATSPI_RELATION_H_
#define _ATSPI_RELATION_H_

#include "glib-object.h"

#include "atspi-constants.h"

G_BEGIN_DECLS

#define ATSPI_TYPE_RELATION                    (atspi_relation_get_type ())
#define ATSPI_IS_RELATION(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_RELATION)
#define ATSPI_RELATION(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_RELATION, AtspiRelation)
#define ATSPI_RELATION_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATSPI_TYPE_RELATION, AtspiRelation))

GType atspi_relation_get_type ();

typedef struct _AtspiRelation AtspiRelation;
struct _AtspiRelation
{
  GObject parent;
  AtspiRelationType relation_type;
  GArray *targets;
};

typedef struct _AtspiRelationClass AtspiRelationClass;
struct _AtspiRelationClass
{
  GObjectClass parent_class;
};

AtspiRelationType atspi_relation_get_relation_type (AtspiRelation *obj);

gint atspi_relation_get_n_targets (AtspiRelation *obj);

AtspiAccessible * atspi_relation_get_target (AtspiRelation *obj, gint i);

/* private */
AtspiRelation * _atspi_relation_new_from_iter (DBusMessageIter *iter);

G_END_DECLS

#endif	/* _ATSPI_RELATION_H_ */
