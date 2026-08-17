/* -*- Mode: C; tab-width: 8; indent-tabs-mode: t; c-basic-offset: 8 -*-
 *
 * Copyright (C) 2013 Richard Hughes <richard@hughsie.com>
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

#ifndef __CD_ICC_H
#define __CD_ICC_H

#include <glib-object.h>
#include <gio/gio.h>

#include "cd-color.h"
#include "cd-enum.h"
#include "cd-edid.h"

G_BEGIN_DECLS

#define CD_ICC_ERROR		(cd_icc_error_quark ())
#define CD_ICC_TYPE_ERROR	(cd_icc_error_get_type ())

#define CD_TYPE_ICC (cd_icc_get_type ())
G_DECLARE_DERIVABLE_TYPE (CdIcc, cd_icc, CD, ICC, GObject)

/**
 * CdIccError:
 * @CD_ICC_ERROR_FAILED_TO_OPEN:	Failed to open file
 * @CD_ICC_ERROR_FAILED_TO_PARSE:	Failed to parse data
 * @CD_ICC_ERROR_INVALID_LOCALE:	Locale was invalid
 * @CD_ICC_ERROR_NO_DATA:		No data to read
 * @CD_ICC_ERROR_FAILED_TO_SAVE:	Failed to save file
 * @CD_ICC_ERROR_FAILED_TO_CREATE:	Failed to create file
 * @CD_ICC_ERROR_INVALID_COLORSPACE:	Invalid colorspace
 * @CD_ICC_ERROR_CORRUPTION_DETECTED:	Corruption has been detected
 * @CD_ICC_ERROR_INTERNAL:		Something inside LCMS broke
 *
 * The ICC error code.
 **/
typedef enum {
	CD_ICC_ERROR_FAILED_TO_OPEN,			/* Since: 0.1.32 */
	CD_ICC_ERROR_FAILED_TO_PARSE,			/* Since: 0.1.32 */
	CD_ICC_ERROR_INVALID_LOCALE,			/* Since: 0.1.32 */
	CD_ICC_ERROR_NO_DATA,				/* Since: 0.1.32 */
	CD_ICC_ERROR_FAILED_TO_SAVE,			/* Since: 0.1.32 */
	CD_ICC_ERROR_FAILED_TO_CREATE,			/* Since: 0.1.32 */
	CD_ICC_ERROR_INVALID_COLORSPACE,		/* Since: 0.1.34 */
	CD_ICC_ERROR_CORRUPTION_DETECTED,		/* Since: 1.1.1 */
	CD_ICC_ERROR_INTERNAL,				/* Since: 1.1.1 */
	/*< private >*/
	CD_ICC_ERROR_LAST
} CdIccError;

struct _CdIccClass
{
	GObjectClass		 parent_class;
	/*< private >*/
	/* Padding for future expansion */
	void (*_cd_icc_reserved1) (void);
	void (*_cd_icc_reserved2) (void);
	void (*_cd_icc_reserved3) (void);
	void (*_cd_icc_reserved4) (void);
	void (*_cd_icc_reserved5) (void);
	void (*_cd_icc_reserved6) (void);
	void (*_cd_icc_reserved7) (void);
	void (*_cd_icc_reserved8) (void);
};

/**
 * CdIccLoadFlags:
 * @CD_ICC_LOAD_FLAGS_NONE:		No flags set.
 * @CD_ICC_LOAD_FLAGS_NAMED_COLORS:	Parse any named colors in the profile.
 * @CD_ICC_LOAD_FLAGS_TRANSLATIONS:	Parse all translations in the profile.
 * @CD_ICC_LOAD_FLAGS_METADATA:		Parse the metadata in the profile.
 * @CD_ICC_LOAD_FLAGS_FALLBACK_MD5:	Calculate the profile MD5 if a profile
 * 					ID was not supplied in the profile.
 * @CD_ICC_LOAD_FLAGS_PRIMARIES:	Parse the primaries in the profile.
 * @CD_ICC_LOAD_FLAGS_CHARACTERIZATION:	Load the characterization data from the profile
 *
 * Flags used when loading an ICC profile.
 *
 * Since: 0.1.32
 **/
typedef enum {
	CD_ICC_LOAD_FLAGS_NONE		= 0,		/* Since: 0.1.32 */
	CD_ICC_LOAD_FLAGS_NAMED_COLORS	= (1 << 0),	/* Since: 0.1.32 */
	CD_ICC_LOAD_FLAGS_TRANSLATIONS	= (1 << 1),	/* Since: 0.1.32 */
	CD_ICC_LOAD_FLAGS_METADATA	= (1 << 2),	/* Since: 0.1.32 */
	CD_ICC_LOAD_FLAGS_FALLBACK_MD5	= (1 << 3),	/* Since: 0.1.32 */
	CD_ICC_LOAD_FLAGS_PRIMARIES	= (1 << 4),	/* Since: 0.1.32 */
	CD_ICC_LOAD_FLAGS_CHARACTERIZATION = (1 << 5),	/* Since: 1.1.1 */
	/* new entries go here: */
	CD_ICC_LOAD_FLAGS_ALL		= 0xff,		/* Since: 0.1.32 */
	/*< private >*/
	CD_ICC_LOAD_FLAGS_LAST
} CdIccLoadFlags;

/**
 * CdIccSaveFlags:
 * @CD_ICC_SAVE_FLAGS_NONE:		No flags set.
 *
 * Flags used when saving an ICC profile.
 *
 * Since: 0.1.32
 **/
typedef enum {
	CD_ICC_SAVE_FLAGS_NONE		= 0,		/* Since: 0.1.32 */
	/*< private >*/
	CD_ICC_SAVE_FLAGS_LAST
} CdIccSaveFlags;

GQuark		 cd_icc_error_quark			(void);
CdIcc		*cd_icc_new				(void);

gboolean	 cd_icc_load_data			(CdIcc		*icc,
							 const guint8	*data,
							 gsize		 data_len,
							 CdIccLoadFlags	 flags,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_icc_load_file			(CdIcc		*icc,
							 GFile		*file,
							 CdIccLoadFlags	 flags,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_icc_load_fd				(CdIcc		*icc,
							 gint		 fd,
							 CdIccLoadFlags	 flags,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_icc_load_handle			(CdIcc		*icc,
							 gpointer	 handle,
							 CdIccLoadFlags	 flags,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
GBytes		*cd_icc_save_data			(CdIcc		*icc,
							 CdIccSaveFlags	 flags,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_icc_save_file			(CdIcc		*icc,
							 GFile		*file,
							 CdIccSaveFlags	 flags,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_icc_save_default			(CdIcc		*icc,
							 CdIccSaveFlags	 flags,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gchar		*cd_icc_to_string			(CdIcc		*icc)
							 G_GNUC_WARN_UNUSED_RESULT;
gpointer	 cd_icc_get_handle			(CdIcc		*icc);
gpointer	 cd_icc_get_context			(CdIcc		*icc);
guint32		 cd_icc_get_size			(CdIcc		*icc);
const gchar	*cd_icc_get_filename			(CdIcc		*icc);
void		 cd_icc_set_filename			(CdIcc		*icc,
							 const gchar	*filename);
gdouble		 cd_icc_get_version			(CdIcc		*icc);
void		 cd_icc_set_version			(CdIcc		*icc,
							 gdouble	 version);
CdProfileKind	 cd_icc_get_kind			(CdIcc		*icc);
void		 cd_icc_set_kind			(CdIcc		*icc,
							 CdProfileKind	 kind);
CdColorspace	 cd_icc_get_colorspace			(CdIcc		*icc);
void		 cd_icc_set_colorspace			(CdIcc		*icc,
							 CdColorspace	 colorspace);
GHashTable	*cd_icc_get_metadata			(CdIcc		*icc);
const gchar	*cd_icc_get_metadata_item		(CdIcc		*icc,
							 const gchar	*key);
void		 cd_icc_add_metadata			(CdIcc		*icc,
							 const gchar	*key,
							 const gchar	*value);
void		 cd_icc_remove_metadata			(CdIcc		*icc,
							 const gchar	*key);
GPtrArray	*cd_icc_get_named_colors		(CdIcc		*icc);
gboolean	 cd_icc_get_can_delete			(CdIcc		*icc);
GDateTime	*cd_icc_get_created			(CdIcc		*icc);
void		 cd_icc_set_created			(CdIcc		*icc,
							 GDateTime	*creation_time);
const gchar	*cd_icc_get_checksum			(CdIcc		*icc);
const gchar	*cd_icc_get_description			(CdIcc		*icc,
							 const gchar	*locale,
							 GError		**error);
const gchar	*cd_icc_get_characterization_data	(CdIcc		*icc);
void		 cd_icc_set_characterization_data	(CdIcc		*icc,
							 const gchar	*data);
const gchar	*cd_icc_get_copyright			(CdIcc		*icc,
							 const gchar	*locale,
							 GError		**error);
const gchar	*cd_icc_get_manufacturer		(CdIcc		*icc,
							 const gchar	*locale,
							 GError		**error);
const gchar	*cd_icc_get_model			(CdIcc		*icc,
							 const gchar	*locale,
							 GError		**error);
void		 cd_icc_set_description			(CdIcc		*icc,
							 const gchar	*locale,
							 const gchar	*value);
void		 cd_icc_set_description_items		(CdIcc		*icc,
							 GHashTable	*values);
void		 cd_icc_set_copyright			(CdIcc		*icc,
							 const gchar	*locale,
							 const gchar	*value);
void		 cd_icc_set_copyright_items		(CdIcc		*icc,
							 GHashTable	*values);
void		 cd_icc_set_manufacturer		(CdIcc		*icc,
							 const gchar	*locale,
							 const gchar	*value);
void		 cd_icc_set_manufacturer_items		(CdIcc		*icc,
							 GHashTable	*values);
void		 cd_icc_set_model			(CdIcc		*icc,
							 const gchar	*locale,
							 const gchar	*value);
void		 cd_icc_set_model_items			(CdIcc		*icc,
							 GHashTable	*values);
const CdColorXYZ *cd_icc_get_red			(CdIcc		*icc);
const CdColorXYZ *cd_icc_get_green			(CdIcc		*icc);
const CdColorXYZ *cd_icc_get_blue			(CdIcc		*icc);
const CdColorXYZ *cd_icc_get_white			(CdIcc		*icc);
guint		 cd_icc_get_temperature			(CdIcc		*icc);
GArray		*cd_icc_get_warnings			(CdIcc		*icc)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_icc_create_from_edid		(CdIcc		*icc,
							 gdouble	 gamma_value,
							 const CdColorYxy *red,
							 const CdColorYxy *green,
							 const CdColorYxy *blue,
							 const CdColorYxy *white,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_icc_create_from_edid_data		(CdIcc		*icc,
							 CdEdid		*edid,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_icc_create_default			(CdIcc		*icc,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_icc_create_default_full		(CdIcc		*icc,
							 CdIccLoadFlags	 flags,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
GPtrArray	*cd_icc_get_vcgt			(CdIcc		*icc,
							 guint		 size,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_icc_set_vcgt			(CdIcc		*icc,
							 GPtrArray	*vcgt,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
GPtrArray	*cd_icc_get_response			(CdIcc		*icc,
							 guint		 size,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gchar		**cd_icc_get_tags			(CdIcc		*icc,
							 GError		**error);
GBytes		*cd_icc_get_tag_data			(CdIcc		*icc,
							 const gchar	*tag,
							 GError		**error);
gboolean	 cd_icc_set_tag_data			(CdIcc		*icc,
							 const gchar	*tag,
							 GBytes		*data,
							 GError		**error);

G_END_DECLS

#endif /* __CD_ICC_H */

