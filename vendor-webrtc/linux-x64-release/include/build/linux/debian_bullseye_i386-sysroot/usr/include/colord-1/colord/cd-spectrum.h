/* -*- Mode: C; tab-width: 8; indent-tabs-mode: t; c-basic-offset: 8 -*-
 *
 * Copyright (C) 2014-2015 Richard Hughes <richard@hughsie.com>
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

#ifndef __CD_SPECTRUM_H__
#define __CD_SPECTRUM_H__

#include <glib-object.h>

G_BEGIN_DECLS

typedef struct _CdSpectrum	CdSpectrum;

#define	CD_TYPE_SPECTRUM	(cd_spectrum_get_type ())

GType		 cd_spectrum_get_type		(void);
CdSpectrum	*cd_spectrum_new		(void);
CdSpectrum	*cd_spectrum_sized_new		(guint			 reserved_size);
CdSpectrum	*cd_spectrum_planckian_new	(gdouble		 temperature);
CdSpectrum	*cd_spectrum_planckian_new_full	(gdouble		 temperature,
						 gdouble		 start,
						 gdouble		 end,
						 gdouble		 resolution);
void		 cd_spectrum_free		(CdSpectrum		*spectrum);
CdSpectrum	*cd_spectrum_dup		(const CdSpectrum	*spectrum);
void		 cd_spectrum_limit_min		(CdSpectrum		*spectrum,
						 gdouble		 value);
void		 cd_spectrum_limit_max		(CdSpectrum		*spectrum,
						 gdouble		 value);
void		 cd_spectrum_normalize		(CdSpectrum		*spectrum,
						 gdouble		 wavelength,
						 gdouble		 value);
void		 cd_spectrum_normalize_max	(CdSpectrum		*spectrum,
						 gdouble		 value);
CdSpectrum	*cd_spectrum_subtract		(CdSpectrum		*s1,
						 CdSpectrum		*s2,
						 gdouble		 resolution);
gchar		*cd_spectrum_to_string		(CdSpectrum		*spectrum,
						 guint			 max_width,
						 guint			 max_height);

const gchar	*cd_spectrum_get_id		(const CdSpectrum	*spectrum);
GArray		*cd_spectrum_get_data		(const CdSpectrum	*spectrum);
gdouble		 cd_spectrum_get_start		(const CdSpectrum	*spectrum);
gdouble		 cd_spectrum_get_end		(const CdSpectrum	*spectrum);
gdouble		 cd_spectrum_get_norm		(const CdSpectrum	*spectrum);
gdouble		 cd_spectrum_get_resolution	(const CdSpectrum	*spectrum);
guint		 cd_spectrum_get_size		(const CdSpectrum	*spectrum);
gdouble		 cd_spectrum_get_value_max	(const CdSpectrum	*spectrum);
gdouble		 cd_spectrum_get_value_min	(const CdSpectrum	*spectrum);
gdouble		 cd_spectrum_get_value		(const CdSpectrum	*spectrum,
						 guint			 idx);
gdouble		 cd_spectrum_get_value_raw	(const CdSpectrum	*spectrum,
						 guint			 idx);
gdouble		 cd_spectrum_get_wavelength	(const CdSpectrum	*spectrum,
						 guint			 idx);
gdouble		 cd_spectrum_get_value_for_nm	(const CdSpectrum	*spectrum,
						 gdouble		 wavelength);

void		 cd_spectrum_set_id		(CdSpectrum		*spectrum,
						 const gchar		*id);
void		 cd_spectrum_set_data		(CdSpectrum		*spectrum,
						 GArray			*value);
void		 cd_spectrum_set_start		(CdSpectrum		*spectrum,
						 gdouble		 start);
void		 cd_spectrum_set_end		(CdSpectrum		*spectrum,
						 gdouble		 end);
void		 cd_spectrum_set_norm		(CdSpectrum		*spectrum,
						 gdouble		 norm);
void		 cd_spectrum_set_value		(CdSpectrum		*spectrum,
						 guint			 idx,
						 gdouble		 data);
void		 cd_spectrum_add_value		(CdSpectrum		*spectrum,
						 gdouble		 data);
void		 cd_spectrum_set_wavelength_cal	(CdSpectrum		*spectrum,
						 gdouble		 c1,
						 gdouble		 c2,
						 gdouble		 c3);
void		 cd_spectrum_get_wavelength_cal	(CdSpectrum		*spectrum,
						 gdouble		 *c1,
						 gdouble		 *c2,
						 gdouble		 *c3);
CdSpectrum	*cd_spectrum_multiply		(CdSpectrum		*s1,
						 CdSpectrum		*s2,
						 gdouble		 resolution);
CdSpectrum	*cd_spectrum_multiply_scalar	(CdSpectrum		*spectrum,
						 gdouble		 value);
CdSpectrum	*cd_spectrum_resample		(CdSpectrum		*spectrum,
						 gdouble		 start,
						 gdouble		 end,
						 gdouble		 resolution);
CdSpectrum	*cd_spectrum_resample_to_size	(CdSpectrum		*spectrum,
						 guint			 size);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(CdSpectrum, cd_spectrum_free)

G_END_DECLS

#endif /* __CD_SPECTRUM_H__ */

