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

#ifndef __CD_DOM_H
#define __CD_DOM_H

#include <glib-object.h>

#include "cd-color.h"

G_BEGIN_DECLS

#define CD_DOM_ERROR		(cd_dom_error_quark ())
#define CD_DOM_TYPE_ERROR	(cd_dom_error_get_type ())

#define CD_TYPE_DOM (cd_dom_get_type ())
G_DECLARE_DERIVABLE_TYPE (CdDom, cd_dom, CD, DOM, GObject)

struct _CdDomClass
{
	GObjectClass		 parent_class;
	/*< private >*/
	/* Padding for future expansion */
	void (*_cd_dom_reserved1) (void);
	void (*_cd_dom_reserved2) (void);
	void (*_cd_dom_reserved3) (void);
	void (*_cd_dom_reserved4) (void);
	void (*_cd_dom_reserved5) (void);
	void (*_cd_dom_reserved6) (void);
	void (*_cd_dom_reserved7) (void);
	void (*_cd_dom_reserved8) (void);
};

GQuark		 cd_dom_error_quark			(void);
CdDom		*cd_dom_new				(void);
gchar		*cd_dom_to_string			(CdDom		*dom);
gboolean	 cd_dom_parse_xml_data			(CdDom		*dom,
							 const gchar	*data,
							 gssize		 data_len,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
const GNode	*cd_dom_get_node			(CdDom		*dom,
							 const GNode	*root,
							 const gchar	*path)
							 G_GNUC_WARN_UNUSED_RESULT;
const gchar	*cd_dom_get_node_name			(const GNode	*node);
const gchar	*cd_dom_get_node_data			(const GNode	*node);
gint		 cd_dom_get_node_data_as_int		(const GNode	*node);
gdouble		 cd_dom_get_node_data_as_double		(const GNode	*node);
const gchar	*cd_dom_get_node_attribute		(const GNode	*node,
							 const gchar	*key);
gboolean	 cd_dom_get_node_rgb			(const GNode	*node,
							 CdColorRGB	*rgb);
gboolean	 cd_dom_get_node_yxy			(const GNode	*node,
							 CdColorYxy	*yxy);
gboolean	 cd_dom_get_node_lab			(const GNode	*node,
							 CdColorLab	*lab);
GHashTable	*cd_dom_get_node_localized		(const GNode	*node,
							 const gchar	*key);

G_END_DECLS

#endif /* __CD_DOM_H */

