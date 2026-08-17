/* -*- Mode: C; tab-width: 8; indent-tabs-mode: t; c-basic-offset: 8 -*-
 *
 * Copyright (C) 2012 Richard Hughes <richard@hughsie.com>
 *
 * Licensed under the GNU Lesser General Public License Version 2.1
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
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301 USA
 */

#ifndef __CD_SESSION_H
#define __CD_SESSION_H

#include <glib.h>

G_BEGIN_DECLS

#define CD_SESSION_DBUS_SERVICE			"org.freedesktop.ColorHelper"
#define CD_SESSION_DBUS_PATH			"/"
#define CD_SESSION_DBUS_INTERFACE		"org.freedesktop.ColorHelper"
#define CD_SESSION_DBUS_INTERFACE_DISPLAY	"org.freedesktop.ColorHelper.Display"

/**
 * CdSessionStatus:
 *
 * The session status.
 **/
typedef enum {
	CD_SESSION_STATUS_IDLE,
	CD_SESSION_STATUS_WAITING_FOR_INTERACTION,
	CD_SESSION_STATUS_RUNNING
} CdSessionStatus;

/**
 * CdSessionInteraction:
 *
 * The interaction required from the user.
 **/
typedef enum {
	CD_SESSION_INTERACTION_ATTACH_TO_SCREEN,
	CD_SESSION_INTERACTION_MOVE_TO_CALIBRATION,
	CD_SESSION_INTERACTION_MOVE_TO_SURFACE,
	CD_SESSION_INTERACTION_SHUT_LAPTOP_LID,
	CD_SESSION_INTERACTION_NONE
} CdSessionInteraction;

/**
 * CdSessionError:
 *
 * Errors returned from the calibration helper.
 */
typedef enum {
	CD_SESSION_ERROR_NONE,
	CD_SESSION_ERROR_INTERNAL,
	CD_SESSION_ERROR_FAILED_TO_FIND_DEVICE,
	CD_SESSION_ERROR_FAILED_TO_FIND_SENSOR,
	CD_SESSION_ERROR_FAILED_TO_FIND_TOOL,
	CD_SESSION_ERROR_FAILED_TO_GENERATE_PROFILE,
	CD_SESSION_ERROR_FAILED_TO_GET_WHITEPOINT,
	CD_SESSION_ERROR_FAILED_TO_OPEN_PROFILE,
	CD_SESSION_ERROR_FAILED_TO_SAVE_PROFILE,
	CD_SESSION_ERROR_INVALID_VALUE,
	CD_SESSION_ERROR_LAST
} CdSessionError;

G_END_DECLS

#endif /* __CD_SESSION_H */

