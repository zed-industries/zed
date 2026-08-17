/* -*- mode: C; c-file-style: "gnu" -*- */
/* atspi-gmain.h atspi dbus gmain prototypes
 *
 * Licensed under the Academic Free License version 2.1
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2.1 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 *
 * This file is based on dbus-gmain.c from dbus-glib with functions renamed
 * and unnecessary code removed.
 */

#ifndef _ATSPI_GMAIN_H
#define _ATSPI_GMAIN_H

G_BEGIN_DECLS

void
atspi_dbus_connection_setup_with_g_main (DBusConnection *connection,
				   GMainContext   *context);

void
atspi_dbus_server_setup_with_g_main (DBusServer   *server,
                               GMainContext *context);

G_END_DECLS

#endif
