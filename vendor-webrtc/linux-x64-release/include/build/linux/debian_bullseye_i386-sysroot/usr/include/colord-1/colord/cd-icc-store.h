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

#ifndef __CD_ICC_STORE_H
#define __CD_ICC_STORE_H

#include <glib-object.h>

#include "cd-icc.h"

G_BEGIN_DECLS

#define CD_TYPE_ICC_STORE (cd_icc_store_get_type ())
G_DECLARE_DERIVABLE_TYPE (CdIccStore, cd_icc_store, CD, ICC_STORE, GObject)

struct _CdIccStoreClass
{
	GObjectClass	parent_class;
	void		(* added)		(CdIcc		*icc);
	void		(* removed)		(CdIcc		*icc);
};

/**
 * CdIccStoreSearchFlags:
 * @CD_ICC_STORE_SEARCH_FLAGS_NONE:			No flags set.
 * @CD_ICC_STORE_SEARCH_FLAGS_CREATE_LOCATION:		Create the location if it does not exist
 *
 * Flags used when adding scan locations.
 *
 * Since: 1.1.1
 **/
typedef enum {
	CD_ICC_STORE_SEARCH_FLAGS_NONE			= 0,	/* Since: 1.0.2 */
	CD_ICC_STORE_SEARCH_FLAGS_CREATE_LOCATION	= 1,	/* Since: 1.0.2 */
	/*< private >*/
	CD_ICC_STORE_SEARCH_FLAGS_LAST
} CdIccStoreSearchFlags;

/**
 * CdIccStoreSearchKind:
 * @CD_ICC_STORE_SEARCH_KIND_SYSTEM:		Per-system locations
 * @CD_ICC_STORE_SEARCH_KIND_MACHINE:		Per-machine locations
 * @CD_ICC_STORE_SEARCH_KIND_USER:		Per-user locations
 *
 * The kind of profiles locations to search.
 *
 * Since: 1.1.1
 **/
typedef enum {
	CD_ICC_STORE_SEARCH_KIND_SYSTEM,		/* Since: 1.0.2 */
	CD_ICC_STORE_SEARCH_KIND_MACHINE,		/* Since: 1.0.2 */
	CD_ICC_STORE_SEARCH_KIND_USER,			/* Since: 1.0.2 */
	/*< private >*/
	CD_ICC_STORE_SEARCH_KIND_LAST
} CdIccStoreSearchKind;

CdIccStore	*cd_icc_store_new		(void);
gboolean	 cd_icc_store_search_location	(CdIccStore	*store,
						 const gchar	*location,
						 CdIccStoreSearchFlags search_flags,
						 GCancellable	*cancellable,
						 GError		**error);
gboolean	 cd_icc_store_search_kind	(CdIccStore	*store,
						 CdIccStoreSearchKind search_kind,
						 CdIccStoreSearchFlags search_flags,
						 GCancellable	*cancellable,
						 GError		**error);
void		 cd_icc_store_set_load_flags	(CdIccStore	*store,
						 CdIccLoadFlags	 load_flags);
CdIccLoadFlags	 cd_icc_store_get_load_flags	(CdIccStore	*store);
void		 cd_icc_store_set_cache		(CdIccStore	*store,
						 GResource	*cache);
GPtrArray	*cd_icc_store_get_all		(CdIccStore	*store);
CdIcc		*cd_icc_store_find_by_filename	(CdIccStore	*store,
						 const gchar	*filename);
CdIcc		*cd_icc_store_find_by_checksum	(CdIccStore	*store,
						 const gchar	*checksum);

G_END_DECLS

#endif /* __CD_ICC_STORE_H */
