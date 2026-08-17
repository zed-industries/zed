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

#ifndef _ATSPI_EVENT_LISTENER_H_
#define _ATSPI_EVENT_LISTENER_H_

#include "glib-object.h"

#include "atspi-types.h"

G_BEGIN_DECLS

GType atspi_event_get_type (void);

/**
 * AtspiEventListenerCB:
 * @event: (transfer full): The event for which notification is sent.
 * @user_data: User data which is passed to the callback each time a notification takes place.
 *
 * A function prototype for callbacks via which clients are notified of AT-SPI events.
 * 
 **/
typedef void       (*AtspiEventListenerCB)     (AtspiEvent     *event,
						     void                      *user_data);

/**
 * AtspiEventListenerSimpleCB:
 * @event: (transfer full): The event for which notification is sent.
 *
 * Like #AtspiEventlistenerCB, but with no user_data.
 * 
 **/
typedef void       (*AtspiEventListenerSimpleCB)     (const AtspiEvent     *event);

#define ATSPI_TYPE_EVENT_LISTENER                        (atspi_event_listener_get_type ())
#define ATSPI_EVENT_LISTENER(obj)                        (G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_EVENT_LISTENER, AtspiEventListener))
#define ATSPI_EVENT_LISTENER_CLASS(klass)                (G_TYPE_CHECK_CLASS_CAST ((klass), ATSPI_TYPE_EVENT_LISTENER, AtspiEventListenerClass))
#define ATSPI_IS_EVENT_LISTENER(obj)                     (G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_EVENT_LISTENER))
#define ATSPI_IS_EVENT_LISTENER_CLASS(klass)             (G_TYPE_CHECK_CLASS_TYPE ((klass), ATSPI_TYPE_EVENT_LISTENER))
#define ATSPI_EVENT_LISTENER_GET_CLASS(obj)              (G_TYPE_INSTANCE_GET_CLASS ((obj), ATSPI_TYPE_EVENT_LISTENER, AtspiEventListenerClass))

typedef struct _AtspiEventListener AtspiEventListener;
struct _AtspiEventListener
{
  GObject parent;
  AtspiEventListenerCB callback;
  void *user_data;
  GDestroyNotify cb_destroyed;
};

typedef struct _AtspiEventListenerClass AtspiEventListenerClass;
struct _AtspiEventListenerClass
{
  GObjectClass parent_class;
};

GType atspi_event_listener_get_type (void);

AtspiEventListener *
atspi_event_listener_new (AtspiEventListenerCB callback,
                                 gpointer user_data,
                                 GDestroyNotify callback_destroyed);

AtspiEventListener *
atspi_event_listener_new_simple (AtspiEventListenerSimpleCB callback,
                                 GDestroyNotify callback_destroyed);

gboolean
atspi_event_listener_register (AtspiEventListener *listener,
				 const gchar              *event_type,
				 GError **error);

gboolean
atspi_event_listener_register_full (AtspiEventListener *listener,
				      const gchar              *event_type,
                                      GArray *properties,
				      GError **error);

gboolean
atspi_event_listener_register_from_callback (AtspiEventListenerCB callback,
				             void *user_data,
				             GDestroyNotify callback_destroyed,
				             const gchar              *event_type,
				             GError **error);

gboolean
atspi_event_listener_register_from_callback_full (AtspiEventListenerCB callback,
				                  void *user_data,
				                  GDestroyNotify callback_destroyed,
				                  const gchar              *event_type,
                                                  GArray *properties,
				                  GError **error);

gboolean
atspi_event_listener_register_no_data (AtspiEventListenerSimpleCB callback,
				 GDestroyNotify callback_destroyed,
				 const gchar              *event_type,
				 GError **error);

gboolean
atspi_event_listener_deregister (AtspiEventListener *listener,
				 const gchar              *event_type,
				 GError **error);

gboolean
atspi_event_listener_deregister_from_callback (AtspiEventListenerCB callback,
				               void *user_data,
				               const gchar              *event_type,
				               GError **error);

gboolean
atspi_event_listener_deregister_no_data (AtspiEventListenerSimpleCB callback,
				   const gchar              *event_type,
				   GError **error);

G_END_DECLS

#endif	/* _ATSPI_EVENT_LISTENER_H_ */
