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

#ifndef _ATSPI_MISC_H_
#define _ATSPI_MISC_H_

G_BEGIN_DECLS

int atspi_init (void);
gboolean atspi_is_initialized (void);

void atspi_event_main ();

void atspi_event_quit ();

int atspi_exit ();

DBusConnection *
atspi_get_a11y_bus ();

void
atspi_set_timeout (gint val, gint startup_time);

void
atspi_set_main_context (GMainContext *cnx);

gchar * atspi_role_get_name (AtspiRole role);
G_END_DECLS

#endif	/* _ATSPI_MISC_H_ */
