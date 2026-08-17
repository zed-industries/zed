/* -*- Mode: C; tab-width: 8; indent-tabs-mode: t; c-basic-offset: 8 -*-
 *
 * Copyright (C) 2010-2012 Richard Hughes <richard@hughsie.com>
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

#ifndef __CD_PROFILE_H
#define __CD_PROFILE_H

#include <glib-object.h>
#include <gio/gio.h>

#include "cd-enum.h"
#include "cd-icc.h"

G_BEGIN_DECLS

#define CD_PROFILE_ERROR	(cd_profile_error_quark ())
#define CD_PROFILE_TYPE_ERROR	(cd_profile_error_get_type ())

#define CD_TYPE_PROFILE (cd_profile_get_type ())
G_DECLARE_DERIVABLE_TYPE (CdProfile, cd_profile, CD, PROFILE, GObject)

struct _CdProfileClass
{
	GObjectClass		 parent_class;
	void			(*changed)		(CdProfile		*profile);
	/*< private >*/
	/* Padding for future expansion */
	void (*_cd_profile_reserved1) (void);
	void (*_cd_profile_reserved2) (void);
	void (*_cd_profile_reserved3) (void);
	void (*_cd_profile_reserved4) (void);
	void (*_cd_profile_reserved5) (void);
	void (*_cd_profile_reserved6) (void);
	void (*_cd_profile_reserved7) (void);
	void (*_cd_profile_reserved8) (void);
};

GQuark		 cd_profile_error_quark			(void);
CdProfile	*cd_profile_new				(void);
CdProfile	*cd_profile_new_with_object_path	(const gchar	*object_path);

/* async */
void		 cd_profile_connect			(CdProfile	*profile,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_profile_connect_finish		(CdProfile	*profile,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_profile_set_property		(CdProfile	*profile,
							 const gchar	*key,
							 const gchar	*value,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_profile_set_property_finish		(CdProfile	*profile,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_profile_install_system_wide		(CdProfile	*profile,
							 GCancellable	*cancellable,
							 GAsyncReadyCallback callback,
							 gpointer	 user_data);
gboolean	 cd_profile_install_system_wide_finish	(CdProfile	*profile,
							 GAsyncResult	*res,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;

/* getters */
const gchar	*cd_profile_get_id			(CdProfile	*profile);
const gchar	*cd_profile_get_filename		(CdProfile	*profile);
const gchar	*cd_profile_get_qualifier		(CdProfile	*profile);
const gchar	*cd_profile_get_format			(CdProfile	*profile);
const gchar	*cd_profile_get_title			(CdProfile	*profile);
const gchar	*cd_profile_get_object_path		(CdProfile	*profile);
CdProfileKind	 cd_profile_get_kind			(CdProfile	*profile);
CdColorspace	 cd_profile_get_colorspace		(CdProfile	*profile);
CdObjectScope	 cd_profile_get_scope			(CdProfile	*profile);
guint		 cd_profile_get_owner			(CdProfile	*profile);
gchar		**cd_profile_get_warnings		(CdProfile	*profile);
gint64		 cd_profile_get_created			(CdProfile	*profile);
gint64		 cd_profile_get_age			(CdProfile	*profile);
gboolean	 cd_profile_get_has_vcgt		(CdProfile	*profile);
gboolean	 cd_profile_get_is_system_wide		(CdProfile	*profile);
GHashTable	*cd_profile_get_metadata		(CdProfile	*profile)
							 G_GNUC_WARN_UNUSED_RESULT;
const gchar	*cd_profile_get_metadata_item		(CdProfile	*profile,
							 const gchar	*key);

/* helpers */
void		 cd_profile_set_object_path		(CdProfile	*profile,
							 const gchar	*object_path);
gboolean	 cd_profile_get_connected		(CdProfile	*profile);
gchar		*cd_profile_to_string			(CdProfile	*profile);
gboolean	 cd_profile_equal			(CdProfile	*profile1,
							 CdProfile	*profile2);
gboolean	 cd_profile_has_access			(CdProfile	*profile);
CdIcc		*cd_profile_load_icc			(CdProfile	*profile,
							 CdIccLoadFlags	 flags,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;

G_END_DECLS

#endif /* __CD_PROFILE_H */

