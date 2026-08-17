/* -*- Mode: C; tab-width: 8; indent-tabs-mode: t; c-basic-offset: 8 -*-
 *
 * Copyright (C) 2009-2013 Richard Hughes <richard@hughsie.com>
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

#ifndef __CD_EDID_H
#define __CD_EDID_H

#include <glib-object.h>

#include "cd-color.h"

G_BEGIN_DECLS

#define CD_EDID_ERROR		(cd_edid_error_quark ())

#define CD_TYPE_EDID (cd_edid_get_type ())
G_DECLARE_DERIVABLE_TYPE (CdEdid, cd_edid, CD, EDID, GObject)

struct _CdEdidClass
{
	GObjectClass		 parent_class;
	/*< private >*/
	/* Padding for future expansion */
	void (*_cd_edid_reserved1) (void);
	void (*_cd_edid_reserved2) (void);
	void (*_cd_edid_reserved3) (void);
	void (*_cd_edid_reserved4) (void);
	void (*_cd_edid_reserved5) (void);
	void (*_cd_edid_reserved6) (void);
	void (*_cd_edid_reserved7) (void);
	void (*_cd_edid_reserved8) (void);
};

/**
 * CdEdidError:
 * @CD_EDID_ERROR_FAILED_TO_PARSE:	The EDID file could not be found
 *
 * The CdEdid error code.
 **/
enum {
	CD_EDID_ERROR_FAILED_TO_PARSE,			/* Since: 1.1.2 */
	/*< private >*/
	CD_EDID_ERROR_LAST
};

GQuark		 cd_edid_error_quark			(void);

CdEdid		*cd_edid_new				(void);
void		 cd_edid_reset				(CdEdid		*edid);
gboolean	 cd_edid_parse				(CdEdid		*edid,
							 GBytes		*edid_data,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
const gchar	*cd_edid_get_monitor_name		(CdEdid		*edid);
const gchar	*cd_edid_get_vendor_name		(CdEdid		*edid);
const gchar	*cd_edid_get_serial_number		(CdEdid		*edid);
const gchar	*cd_edid_get_eisa_id			(CdEdid		*edid);
const gchar	*cd_edid_get_checksum			(CdEdid		*edid);
const gchar	*cd_edid_get_pnp_id			(CdEdid		*edid);
guint		 cd_edid_get_width			(CdEdid		*edid);
guint		 cd_edid_get_height			(CdEdid		*edid);
gdouble		 cd_edid_get_gamma			(CdEdid		*edid);
const CdColorYxy *cd_edid_get_red			(CdEdid		*edid);
const CdColorYxy *cd_edid_get_green			(CdEdid		*edid);
const CdColorYxy *cd_edid_get_blue			(CdEdid		*edid);
const CdColorYxy *cd_edid_get_white			(CdEdid		*edid);

G_END_DECLS

#endif /* __CD_EDID_H */

