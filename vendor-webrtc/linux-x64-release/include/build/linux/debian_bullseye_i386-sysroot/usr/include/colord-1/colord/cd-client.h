/* -*- Mode: C; tab-width: 8; indent-tabs-mode: t; c-basic-offset: 8 -*-
 *
 * Copyright (C) 2010-2013 Richard Hughes <richard@hughsie.com>
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

#ifndef __CD_CLIENT_H
#define __CD_CLIENT_H

#include <glib-object.h>
#include <gio/gio.h>

#include "cd-device.h"
#include "cd-profile.h"
#include "cd-sensor.h"

G_BEGIN_DECLS

#define CD_CLIENT_ERROR		(cd_client_error_quark ())
#define CD_CLIENT_TYPE_ERROR	(cd_client_error_get_type ())

#define CD_TYPE_CLIENT (cd_client_get_type ())
G_DECLARE_DERIVABLE_TYPE (CdClient, cd_client, CD, CLIENT, GObject)

struct _CdClientClass
{
	GObjectClass		 parent_class;
	void			(*device_added)		(CdClient		*client,
							 CdDevice		*device);
	void			(*device_removed)	(CdClient		*client,
							 CdDevice		*device);
	void			(*device_changed)	(CdClient		*client,
							 CdDevice		*device);
	void			(*profile_added)	(CdClient		*client,
							 CdProfile		*profile);
	void			(*profile_removed)	(CdClient		*client,
							 CdProfile		*profile);
	void			(*profile_changed)	(CdClient		*client,
							 CdProfile		*profile);
	void			(*sensor_added)		(CdClient		*client,
							 CdSensor		*sensor);
	void			(*sensor_removed)	(CdClient		*client,
							 CdSensor		*sensor);
	void			(*sensor_changed)	(CdClient		*client,
							 CdSensor		*sensor);
	void			(*changed)              (CdClient		*client);
	/*< private >*/
	/* Padding for future expansion */
	void (*_cd_client_reserved1) (void);
	void (*_cd_client_reserved2) (void);
	void (*_cd_client_reserved3) (void);
	void (*_cd_client_reserved4) (void);
	void (*_cd_client_reserved5) (void);
	void (*_cd_client_reserved6) (void);
	void (*_cd_client_reserved7) (void);
	void (*_cd_client_reserved8) (void);
};

GQuark		 cd_client_error_quark			(void);
CdClient	*cd_client_new				(void);

/* async */
void		 cd_client_connect			(CdClient	*client,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_client_connect_finish		(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		cd_client_create_device			(CdClient	*client,
							 const gchar	*id,
							 CdObjectScope	 scope,
							 GHashTable	*properties,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
CdDevice	*cd_client_create_device_finish		(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		cd_client_create_profile		(CdClient	*client,
							 const gchar	*id,
							 CdObjectScope	 scope,
							 GHashTable	*properties,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
CdProfile	*cd_client_create_profile_finish	(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_client_create_profile_for_icc	(CdClient	*client,
							 CdIcc		*icc,
							 CdObjectScope	 scope,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
CdProfile	*cd_client_create_profile_for_icc_finish (CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		cd_client_import_profile		(CdClient	*client,
							 GFile		*file,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
CdProfile	*cd_client_import_profile_finish	(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_client_delete_device		(CdClient	*client,
							 CdDevice	*device,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_client_delete_device_finish		(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_client_delete_profile		(CdClient	*client,
							 CdProfile	*profile,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_client_delete_profile_finish	(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		cd_client_find_device			(CdClient	*client,
							 const gchar	*id,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
CdDevice	*cd_client_find_device_finish 		(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		cd_client_find_device_by_property	(CdClient	*client,
							 const gchar	*key,
							 const gchar	*value,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
CdDevice	*cd_client_find_device_by_property_finish (CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		cd_client_find_profile			(CdClient	*client,
							 const gchar	*id,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
CdProfile	*cd_client_find_profile_finish 		(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;

void		cd_client_find_profile_by_filename	(CdClient	*client,
							 const gchar	*filename,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
CdProfile	*cd_client_find_profile_by_filename_finish (CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		cd_client_get_standard_space		(CdClient	*client,
							 CdStandardSpace standard_space,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
CdProfile	*cd_client_get_standard_space_finish	(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_client_get_devices			(CdClient	*client,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
GPtrArray	*cd_client_get_devices_finish		(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_client_get_devices_by_kind		(CdClient	*client,
							 CdDeviceKind	 kind,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
GPtrArray	*cd_client_get_devices_by_kind_finish	(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_client_get_profiles			(CdClient	*client,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
GPtrArray	*cd_client_get_profiles_finish		(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_client_get_sensors			(CdClient	*client,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
GPtrArray	*cd_client_get_sensors_finish		(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		cd_client_find_profile_by_property	(CdClient	*client,
							 const gchar	*key,
							 const gchar	*value,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
CdProfile	*cd_client_find_profile_by_property_finish (CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		cd_client_find_sensor			(CdClient	*client,
							 const gchar	*id,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
CdSensor	*cd_client_find_sensor_finish 		(CdClient	*client,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;

/* getters */
gboolean	 cd_client_get_connected		(CdClient	*client);
gboolean	 cd_client_get_has_server		(CdClient	*client);
const gchar	*cd_client_get_daemon_version		(CdClient	*client);
const gchar	*cd_client_get_system_vendor		(CdClient	*client);
const gchar	*cd_client_get_system_model		(CdClient	*client);

G_END_DECLS

#endif /* __CD_CLIENT_H */

