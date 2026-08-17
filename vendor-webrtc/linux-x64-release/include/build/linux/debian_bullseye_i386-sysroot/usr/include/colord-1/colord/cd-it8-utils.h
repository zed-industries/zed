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

#ifndef __CD_IT8_UTILS_H__
#define __CD_IT8_UTILS_H__

#include <glib-object.h>

#include "cd-it8.h"
#include "cd-spectrum.h"

G_BEGIN_DECLS

gboolean	 cd_it8_utils_calculate_ccmx		(CdIt8		*it8_reference,
							 CdIt8		*it8_measured,
							 CdIt8		*it8_ccmx,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_it8_utils_calculate_xyz_from_cmf	(CdIt8		*cmf,
							 CdSpectrum	*illuminant,
							 CdSpectrum	*spectrum,
							 CdColorXYZ	*value,
							 gdouble	 resolution,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_it8_utils_calculate_cri_from_cmf	(CdIt8		*cmf,
							 CdIt8		*tcs,
							 CdSpectrum	*illuminant,
							 gdouble	*value,
							 gdouble	 resolution,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;
gboolean	 cd_it8_utils_calculate_gamma		(CdIt8		*it8,
							 gdouble	*gamma_y,
							 GError		**error);

G_END_DECLS

#endif /* __CD_IT8_UTILS_H__ */

