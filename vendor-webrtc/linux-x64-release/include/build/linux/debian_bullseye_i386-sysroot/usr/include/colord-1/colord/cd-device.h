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

#if !defined (__COLORD_H_INSIDE__) && !defined (CD_COMPILATION)
#error "Only <colord.h> can be included directly."
#endif

#ifndef __CD_DEVICE_H
#define __CD_DEVICE_H

#include <glib-object.h>
#include <gio/gio.h>

#include "cd-enum.h"
#include "cd-profile.h"

G_BEGIN_DECLS

#define CD_DEVICE_ERROR		(cd_device_error_quark ())
#define CD_DEVICE_TYPE_ERROR	(cd_device_error_get_type ())

#define CD_TYPE_DEVICE (cd_device_get_type ())
G_DECLARE_DERIVABLE_TYPE (CdDevice, cd_device, CD, DEVICE, GObject)

struct _CdDeviceClass
{
	GObjectClass		 parent_class;
	void			(*changed)		(CdDevice		*device);
	/*< private >*/
	/* Padding for future expansion */
	void (*_cd_device_reserved1) (void);
	void (*_cd_device_reserved2) (void);
	void (*_cd_device_reserved3) (void);
	void (*_cd_device_reserved4) (void);
	void (*_cd_device_reserved5) (void);
	void (*_cd_device_reserved6) (void);
	void (*_cd_device_reserved7) (void);
	void (*_cd_device_reserved8) (void);
};

GQuark		 cd_device_error_quark			(void);
CdDevice	*cd_device_new				(void);
CdDevice	*cd_device_new_with_object_path		(const gchar	*object_path);

/* async */
void		 cd_device_connect			(CdDevice	*device,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_device_connect_finish		(CdDevice	*device,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_device_set_property			(CdDevice	*device,
							 const gchar	*key,
							 const gchar	*value,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_device_set_property_finish		(CdDevice	*device,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_device_add_profile			(CdDevice	*device,
							 CdDeviceRelation relation,
							 CdProfile	*profile,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_device_add_profile_finish		(CdDevice	*device,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_device_remove_profile		(CdDevice	*device,
							 CdProfile	*profile,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_device_remove_profile_finish	(CdDevice	*device,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_device_make_profile_default		(CdDevice	*device,
							 CdProfile	*profile,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_device_make_profile_default_finish	(CdDevice	*device,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_device_profiling_inhibit		(CdDevice	*device,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_device_profiling_inhibit_finish	(CdDevice	*device,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_device_set_enabled			(CdDevice	*device,
							 gboolean	 enabled,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_device_set_enabled_finish		(CdDevice	*device,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_device_profiling_uninhibit		(CdDevice	*device,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_device_profiling_uninhibit_finish	(CdDevice	*device,
							 GAsyncResult	*res,
							 GError		**error);
void		 cd_device_get_profile_for_qualifiers	(CdDevice	*device,
							 const gchar	**qualifiers,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
CdProfile	*cd_device_get_profile_for_qualifiers_finish (CdDevice	*device,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_device_get_profile_relation		(CdDevice	*device,
							 CdProfile	*profile,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
CdDeviceRelation cd_device_get_profile_relation_finish	(CdDevice	*device,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;

/* getters */
const gchar	*cd_device_get_id			(CdDevice	*device);
const gchar	*cd_device_get_model			(CdDevice	*device);
const gchar	*cd_device_get_vendor			(CdDevice	*device);
const gchar	*cd_device_get_serial			(CdDevice	*device);
const gchar	*cd_device_get_seat			(CdDevice	*device);
const gchar	*cd_device_get_format			(CdDevice	*device);
const gchar	**cd_device_get_profiling_inhibitors	(CdDevice	*device);
guint64		 cd_device_get_created			(CdDevice	*device);
guint64		 cd_device_get_modified			(CdDevice	*device);
CdDeviceKind	 cd_device_get_kind			(CdDevice	*device);
CdColorspace	 cd_device_get_colorspace		(CdDevice	*device);
CdDeviceMode	 cd_device_get_mode			(CdDevice	*device);
gboolean	 cd_device_get_enabled			(CdDevice	*device);
gboolean	 cd_device_get_embedded			(CdDevice	*device);
CdObjectScope	 cd_device_get_scope			(CdDevice	*device);
guint		 cd_device_get_owner			(CdDevice	*device);
GPtrArray	*cd_device_get_profiles			(CdDevice	*device);
CdProfile	*cd_device_get_default_profile		(CdDevice	*device);
const gchar	*cd_device_get_object_path		(CdDevice	*device);
GHashTable	*cd_device_get_metadata			(CdDevice	*device);
const gchar	*cd_device_get_metadata_item		(CdDevice	*device,
							 const gchar	*key);

/* helpers */
void		 cd_device_set_object_path		(CdDevice	*device,
							 const gchar	*object_path);
gboolean	 cd_device_get_connected		(CdDevice	*device);
gchar		*cd_device_to_string			(CdDevice	*device);
gboolean	 cd_device_equal			(CdDevice	*device1,
							 CdDevice	*device2);

G_END_DECLS

#endif /* __CD_DEVICE_H */

