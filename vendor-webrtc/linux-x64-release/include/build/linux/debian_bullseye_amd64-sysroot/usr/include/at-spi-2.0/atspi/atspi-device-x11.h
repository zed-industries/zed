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

#ifndef _ATSPI_DEVICE_X11_H_
#define _ATSPI_DEVICE_X11_H_

#include "glib-object.h"

#include "atspi-types.h"
#include "atspi-device.h"

G_BEGIN_DECLS

#define ATSPI_TYPE_DEVICE_X11                        (atspi_device_x11_get_type ())
#define ATSPI_DEVICE_X11(obj)                        (G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_DEVICE_X11, AtspiDeviceX11))
#define ATSPI_DEVICE_X11_CLASS(klass)                (G_TYPE_CHECK_CLASS_CAST ((klass), ATSPI_TYPE_DEVICE_X11, AtspiDeviceX11Class))
#define ATSPI_IS_DEVICE_X11(obj)                     (G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_DEVICE_X11))
#define ATSPI_IS_DEVICE_X11_CLASS(klass)             (G_TYPE_CHECK_CLASS_TYPE ((klass), ATSPI_TYPE_DEVICE_X11))
#define ATSPI_DEVICE_X11_GET_CLASS(obj)              (G_TYPE_INSTANCE_GET_CLASS ((obj), ATSPI_TYPE_DEVICE_X11, AtspiDeviceX11Class))

typedef struct _AtspiDeviceX11 AtspiDeviceX11;
struct _AtspiDeviceX11
{
  AtspiDevice parent;
};

typedef struct _AtspiDeviceX11Class AtspiDeviceX11Class;
struct _AtspiDeviceX11Class
{
  AtspiDeviceClass parent_class;
};

GType atspi_device_x11_get_type (void);

AtspiDeviceX11 *atspi_device_x11_new ();

G_END_DECLS

#endif	/* _ATSPI_DEVICE_X11_H_ */
