/*
 * AT-SPI - Assistive Technology Service Provider Interface
 * (Gnome Accessibility Project; http://developer.gnome.org/projects/gap)
 *
 * Copyright 2020 SUSE LLC.
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

#ifndef _ATSPI_DEVICE_LEGACY_H_
#define _ATSPI_DEVICE_LEGACY_H_

#include "glib-object.h"

#include "atspi-types.h"
#include "atspi-device.h"

G_BEGIN_DECLS

#define ATSPI_TYPE_DEVICE_LEGACY                        (atspi_device_legacy_get_type ())
#define ATSPI_DEVICE_LEGACY(obj)                        (G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_DEVICE_LEGACY, AtspiDeviceLegacy))
#define ATSPI_DEVICE_LEGACY_CLASS(klass)                (G_TYPE_CHECK_CLASS_CAST ((klass), ATSPI_TYPE_DEVICE_LEGACY, AtspiDeviceLegacyClass))
#define ATSPI_IS_DEVICE_LEGACY(obj)                     (G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_DEVICE_LEGACY))
#define ATSPI_IS_DEVICE_LEGACY_CLASS(klass)             (G_TYPE_CHECK_CLASS_TYPE ((klass), ATSPI_TYPE_DEVICE_LEGACY))
#define ATSPI_DEVICE_LEGACY_GET_CLASS(obj)              (G_TYPE_INSTANCE_GET_CLASS ((obj), ATSPI_TYPE_DEVICE_LEGACY, AtspiDeviceLegacyClass))

typedef struct _AtspiDeviceLegacy AtspiDeviceLegacy;
struct _AtspiDeviceLegacy
{
  AtspiDevice parent;
};

typedef struct _AtspiDeviceLegacyClass AtspiDeviceLegacyClass;
struct _AtspiDeviceLegacyClass
{
  AtspiDeviceClass parent_class;
};

GType atspi_device_legacy_get_type (void);

AtspiDeviceLegacy *atspi_device_legacy_new ();

G_END_DECLS

#endif	/* _ATSPI_DEVICE_LEGACY_H_ */
