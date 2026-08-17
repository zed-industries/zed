/* -*- Mode: C; tab-width: 8; indent-tabs-mode: t; c-basic-offset: 8 -*-
 *
 * Copyright (C) 2011-2012 Richard Hughes <richard@hughsie.com>
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

#if !defined (__COLORD_H_INSIDE__) && !defined (CD_COMPILATION)
#error "Only <colord.h> can be included directly."
#endif

#ifndef __CD_CLIENT_SYNC_H
#define __CD_CLIENT_SYNC_H

#include <glib-object.h>

#include "cd-client.h"
#include "cd-device.h"
#include "cd-profile.h"

G_BEGIN_DECLS

gboolean	 cd_client_connect_sync			(CdClient	*client,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_client_delete_profile_sync		(CdClient	*client,
							 CdProfile	*profile,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_client_delete_device_sync		(CdClient	*client,
							 CdDevice	*device,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
CdProfile	*cd_client_find_profile_sync		(CdClient	*client,
							 const gchar	*id,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
CdProfile	*cd_client_find_profile_by_filename_sync (CdClient	*client,
							 const gchar	*filename,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
CdProfile	*cd_client_create_profile_sync		(CdClient	*client,
							 const gchar	*id,
							 CdObjectScope	 scope,
							 GHashTable	*properties,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
CdProfile	*cd_client_create_profile_for_icc_sync	(CdClient	*client,
							 CdIcc		*icc,
							 CdObjectScope	 scope,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
CdProfile	*cd_client_import_profile_sync		(CdClient	*client,
							 GFile		*file,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
CdDevice	*cd_client_create_device_sync		(CdClient	*client,
							 const gchar	*id,
							 CdObjectScope	 scope,
							 GHashTable	*properties,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
GPtrArray	*cd_client_get_devices_sync		(CdClient	*client,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
GPtrArray	*cd_client_get_profiles_sync		(CdClient	*client,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
GPtrArray	*cd_client_get_sensors_sync		(CdClient	*client,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
CdDevice	*cd_client_find_device_sync		(CdClient	*client,
							 const gchar	*id,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
CdDevice	*cd_client_find_device_by_property_sync	(CdClient	*client,
							 const gchar	*key,
							 const gchar	*value,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
CdProfile	*cd_client_get_standard_space_sync	(CdClient	*client,
							 CdStandardSpace standard_space,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
GPtrArray	*cd_client_get_devices_by_kind_sync	(CdClient	*client,
							 CdDeviceKind	 kind,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
CdProfile	*cd_client_find_profile_by_property_sync(CdClient	*client,
							 const gchar	*key,
							 const gchar	*value,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
CdSensor	*cd_client_find_sensor_sync		(CdClient	*client,
							 const gchar	*id,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;

G_END_DECLS

#endif /* __CD_CLIENT_SYNC_H */

