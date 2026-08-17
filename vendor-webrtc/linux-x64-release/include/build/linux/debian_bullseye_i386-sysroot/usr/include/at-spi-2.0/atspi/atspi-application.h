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

#ifndef _ATSPI_APPLICATION_H_
#define _ATSPI_APPLICATION_H_

#include <dbus/dbus.h>

#include "atspi-types.h"
#include "atspi-accessible.h"
#include <sys/time.h>

G_BEGIN_DECLS

#define ATSPI_TYPE_APPLICATION                        (atspi_application_get_type ())
#define ATSPI_APPLICATION(obj)                        (G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_APPLICATION, AtspiApplication))
#define ATSPI_APPLICATION_CLASS(klass)                (G_TYPE_CHECK_CLASS_CAST ((klass), ATSPI_TYPE_APPLICATION, AtspiAccessibleClass))
#define ATSPI_IS_APPLICATION(obj)                     (G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_APPLICATION))
#define ATSPI_IS_APPLICATION_CLASS(klass)             (G_TYPE_CHECK_CLASS_TYPE ((klass), ATSPI_TYPE_APPLICATION))
#define ATSPI_APPLICATION_GET_CLASS(obj)              (G_TYPE_INSTANCE_GET_CLASS ((obj), ATSPI_TYPE_APPLICATION, AtspiAccessibleClass))

typedef struct _AtspiApplication AtspiApplication;
struct _AtspiApplication
{
  GObject parent;
  GHashTable *hash;
  char *bus_name;
  DBusConnection *bus;
  struct _AtspiAccessible *root;
  AtspiCache cache;
  gchar *toolkit_name;
  gchar *toolkit_version;
  gchar *atspi_version;
  struct timeval time_added;
};

typedef struct _AtspiApplicationClass AtspiApplicationClass;
struct _AtspiApplicationClass
{
  GObjectClass parent_class;
};

AtspiApplication *
_atspi_application_new (const char *bus_name);

GType atspi_application_get_type (void);

G_END_DECLS

#endif	/* _ATSPI_APPLICATION_H_ */
