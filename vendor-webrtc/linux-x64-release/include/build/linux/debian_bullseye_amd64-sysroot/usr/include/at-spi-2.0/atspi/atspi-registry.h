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

#ifndef _ATSPI_REGISTRY_H_
#define _ATSPI_REGISTRY_H_

#include "atspi-accessible.h"
#include "atspi-types.h"
#include "atspi-device-listener.h"

G_BEGIN_DECLS

GType atspi_key_definition_get_type ();

gint atspi_get_desktop_count ();

AtspiAccessible* atspi_get_desktop (gint i);

GArray *atspi_get_desktop_list ();

gboolean
atspi_register_keystroke_listener (AtspiDeviceListener  *listener,
					 GArray *key_set,
					 AtspiKeyMaskType         modmask,
					 AtspiKeyEventMask        event_types,
					 AtspiKeyListenerSyncType sync_type,
                                         GError **error);

gboolean
atspi_deregister_keystroke_listener (AtspiDeviceListener *listener,
                            GArray              *key_set,
					        AtspiKeyMaskType         modmask,
					        AtspiKeyEventMask        event_types,
					        GError **error);

gboolean
atspi_register_device_event_listener (AtspiDeviceListener  *listener,
				 AtspiDeviceEventMask  event_types,
				 void                      *filter, GError **error);

gboolean
atspi_deregister_device_event_listener (AtspiDeviceListener *listener,
				   void                     *filter, GError **error);

gboolean
atspi_generate_keyboard_event (glong keyval,
			   const gchar *keystring,
			   AtspiKeySynthType synth_type, GError **error);

gboolean
atspi_generate_mouse_event (glong x, glong y, const gchar *name, GError **error);

void
atspi_set_reference_window (AtspiAccessible *accessible);
G_END_DECLS

#endif	/* _ATSPI_REGISTRY_H_ */
