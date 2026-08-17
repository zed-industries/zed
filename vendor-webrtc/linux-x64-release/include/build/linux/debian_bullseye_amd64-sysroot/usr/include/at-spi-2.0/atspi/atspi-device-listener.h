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

#ifndef _ATSPI_DEVICE_LISTENER_H_
#define _ATSPI_DEVICE_LISTENER_H_

#include "glib-object.h"

#include "atspi-types.h"

G_BEGIN_DECLS

GType atspi_device_event_get_type (void);

/**
 * AtspiDeviceListenerCB:
 * @stroke: (transfer full): The #AtspiDeviceEvent for which notification is
 *          being received.
 * @user_data: Data which is passed to the client each time this callback is notified.
 *
 * A callback function prototype via which clients receive device event notifications.
 *
 * Returns: #TRUE if the client wishes to consume/preempt the event, preventing it from being
 * relayed to the currently focussed application, #FALSE if the event delivery should proceed as normal.
 **/
typedef gboolean (*AtspiDeviceListenerCB)    (AtspiDeviceEvent *stroke,
						     void                      *user_data);

/**
 * AtspiDeviceListenerSimpleCB:
 * @stroke: (transfer full): The #AtspiDeviceEvent for which notification is
 *          being received.
 *
 * Similar to #AtspiDeviceListenerCB, but with no user data.
 *
 * Returns: #TRUE if the client wishes to consume/preempt the event, preventing it from being
 * relayed to the currently focussed application, #FALSE if the event delivery should proceed as normal.
 **/
typedef gboolean (*AtspiDeviceListenerSimpleCB)    (const AtspiDeviceEvent *stroke);

#define ATSPI_TYPE_DEVICE_LISTENER                        (atspi_device_listener_get_type ())
#define ATSPI_DEVICE_LISTENER(obj)                        (G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_DEVICE_LISTENER, AtspiDeviceListener))
#define ATSPI_DEVICE_LISTENER_CLASS(klass)                (G_TYPE_CHECK_CLASS_CAST ((klass), ATSPI_TYPE_DEVICE_LISTENER, AtspiDeviceListenerClass))
#define ATSPI_IS_DEVICE_LISTENER(obj)                     (G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_DEVICE_LISTENER))
#define ATSPI_IS_DEVICE_LISTENER_CLASS(klass)             (G_TYPE_CHECK_CLASS_TYPE ((klass), ATSPI_TYPE_DEVICE_LISTENER))
#define ATSPI_DEVICE_LISTENER_GET_CLASS(obj)              (G_TYPE_INSTANCE_GET_CLASS ((obj), ATSPI_TYPE_DEVICE_LISTENER, AtspiDeviceListenerClass))

typedef struct _AtspiDeviceListener AtspiDeviceListener;
struct _AtspiDeviceListener
{
  GObject parent;
  guint id;
  GList *callbacks;
};

typedef struct _AtspiDeviceListenerClass AtspiDeviceListenerClass;
struct _AtspiDeviceListenerClass
{
  GObjectClass parent_class;
  gboolean (*device_event) (AtspiDeviceListener *listener, const AtspiDeviceEvent *event);
};

GType atspi_device_listener_get_type (void);

AtspiDeviceListener *atspi_device_listener_new (AtspiDeviceListenerCB callback, void *user_data, GDestroyNotify callback_destroyed);

AtspiDeviceListener *atspi_device_listener_new_simple (AtspiDeviceListenerSimpleCB callback, GDestroyNotify callback_destroyed);

void atspi_device_listener_add_callback (AtspiDeviceListener *listener, AtspiDeviceListenerCB callback, GDestroyNotify callback_destroyed, void *user_data);

void atspi_device_listener_remove_callback (AtspiDeviceListener  *listener, AtspiDeviceListenerCB callback);

G_END_DECLS

#endif	/* _ATSPI_DEVICE_LISTENER_H_ */
