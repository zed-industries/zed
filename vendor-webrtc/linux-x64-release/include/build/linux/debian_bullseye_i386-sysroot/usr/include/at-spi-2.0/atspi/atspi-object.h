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

#ifndef _ATSPI_OBJECT_H_
#define _ATSPI_OBJECT_H_

#include "glib-object.h"

#include "atspi-application.h"
#include "atspi-types.h"

G_BEGIN_DECLS

#define ATSPI_TYPE_OBJECT                        (atspi_object_get_type ())
#define ATSPI_OBJECT(obj)                        (G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_OBJECT, AtspiObject))
#define ATSPI_OBJECT_CLASS(klass)                (G_TYPE_CHECK_CLASS_CAST ((klass), ATSPI_TYPE_OBJECT, AtspiObjectClass))
#define ATSPI_IS_OBJECT(obj)                     (G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_OBJECT))
#define ATSPI_IS_OBJECT_CLASS(klass)             (G_TYPE_CHECK_CLASS_TYPE ((klass), ATSPI_TYPE_OBJECT))
#define ATSPI_OBJECT_GET_CLASS(obj)              (G_TYPE_INSTANCE_GET_CLASS ((obj), ATSPI_TYPE_OBJECT, AtspiObjectClass))

typedef struct _AtspiObject AtspiObject;
struct _AtspiObject
{
  GObject parent;
  AtspiApplication *app;
  char *path;
};

typedef struct _AtspiObjectClass AtspiObjectClass;
struct _AtspiObjectClass
{
  GObjectClass parent_class;
};

GType atspi_object_get_type (void); 

G_END_DECLS

#endif	/* _ATSPI_OBJECT_H_ */
