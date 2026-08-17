/* -*- Mode: C; tab-width: 8; indent-tabs-mode: t; c-basic-offset: 8 -*-
 *
 * Copyright (C) 2010-2014 Richard Hughes <richard@hughsie.com>
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

#ifndef __CD_COLOR_H__
#define __CD_COLOR_H__

#include <glib-object.h>

G_BEGIN_DECLS

typedef struct {
	guint8	 R;
	guint8	 G;
	guint8	 B;
} CdColorRGB8;

typedef struct {
	gdouble	 L;
	gdouble	 a;
	gdouble	 b;
} CdColorLab;

typedef struct {
	gdouble	 Y;
	gdouble	 x;
	gdouble	 y;
} CdColorYxy;

typedef struct {
	gdouble	 X;
	gdouble	 Y;
	gdouble	 Z;
} CdColorXYZ;

typedef struct {
	gdouble	 R;
	gdouble	 G;
	gdouble	 B;
} CdColorRGB;

typedef struct {
	gdouble	 U;
	gdouble	 V;
	gdouble	 W;
} CdColorUVW;

/**
 * CdColorBlackbodyFlags:
 * @CD_COLOR_BLACKBODY_FLAG_NONE:		No flags set.
 * @CD_COLOR_BLACKBODY_FLAG_USE_PLANCKIAN:	Use Planckian below 5000K
 *
 * Flags used when returning an RGB color from a temperature.
 *
 * Since: 1.3.5
 **/
typedef enum {
	CD_COLOR_BLACKBODY_FLAG_NONE,		/* Since: 1.3.5 */
	CD_COLOR_BLACKBODY_FLAG_USE_PLANCKIAN,	/* Since: 1.3.5 */
	/*< private >*/
	CD_COLOR_BLACKBODY_FLAG_LAST
} CdColorBlackbodyFlags;

typedef struct _CdColorSwatch	CdColorSwatch;

#define	CD_TYPE_COLOR_RGB	(cd_color_rgb_get_type ())
#define	CD_TYPE_COLOR_XYZ	(cd_color_xyz_get_type ())
#define	CD_TYPE_COLOR_LAB	(cd_color_lab_get_type ())
#define	CD_TYPE_COLOR_YXY	(cd_color_yxy_get_type ())
#define	CD_TYPE_COLOR_UVW	(cd_color_uvw_get_type ())
#define	CD_TYPE_COLOR_SWATCH	(cd_color_swatch_get_type ())

/* types */
GType		 cd_color_xyz_get_type			(void);
GType		 cd_color_lab_get_type			(void);
GType		 cd_color_rgb_get_type			(void);
GType		 cd_color_yxy_get_type			(void);
GType		 cd_color_uvw_get_type			(void);
GType		 cd_color_swatch_get_type		(void);

const gchar	*cd_color_swatch_get_name		(const CdColorSwatch	*swatch);
const CdColorLab*cd_color_swatch_get_value		(const CdColorSwatch	*swatch);

CdColorXYZ	*cd_color_xyz_new			(void);
CdColorLab	*cd_color_lab_new			(void);
CdColorRGB	*cd_color_rgb_new			(void);
CdColorYxy	*cd_color_yxy_new			(void);
CdColorUVW	*cd_color_uvw_new			(void);
CdColorSwatch	*cd_color_swatch_new			(void);

void		 cd_color_xyz_free			(CdColorXYZ		*src);
void		 cd_color_rgb_free			(CdColorRGB		*src);
void		 cd_color_lab_free			(CdColorLab		*src);
void		 cd_color_yxy_free			(CdColorYxy		*src);
void		 cd_color_uvw_free			(CdColorUVW		*src);
void		 cd_color_swatch_free			(CdColorSwatch		*src);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(CdColorXYZ, cd_color_xyz_free)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(CdColorRGB, cd_color_rgb_free)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(CdColorLab, cd_color_lab_free)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(CdColorYxy, cd_color_yxy_free)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(CdColorUVW, cd_color_uvw_free)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(CdColorSwatch, cd_color_swatch_free)

CdColorXYZ	*cd_color_xyz_dup			(const CdColorXYZ	*src);
CdColorLab	*cd_color_lab_dup			(const CdColorLab	*src);
CdColorRGB	*cd_color_rgb_dup			(const CdColorRGB	*src);
CdColorYxy	*cd_color_yxy_dup			(const CdColorYxy	*src);
CdColorUVW	*cd_color_uvw_dup			(const CdColorUVW	*src);
CdColorSwatch	*cd_color_swatch_dup			(const CdColorSwatch	*src);

void		 cd_color_xyz_set			(CdColorXYZ		*dest,
							 gdouble		 X,
							 gdouble		 Y,
							 gdouble		 Z);
void		 cd_color_rgb_set			(CdColorRGB		*dest,
							 gdouble		 R,
							 gdouble		 G,
							 gdouble		 B);
void		 cd_color_lab_set			(CdColorLab		*dest,
							 gdouble		 L,
							 gdouble		 a,
							 gdouble		 b);
void		 cd_color_yxy_set			(CdColorYxy		*dest,
							 gdouble		 Y,
							 gdouble		 x,
							 gdouble		 y);
void		 cd_color_uvw_set			(CdColorUVW		*dest,
							 gdouble		 U,
							 gdouble		 V,
							 gdouble		 W);
void		 cd_color_swatch_set_name		(CdColorSwatch		*dest,
							 const gchar		*name);
void		 cd_color_swatch_set_value		(CdColorSwatch		*dest,
							 const CdColorLab	*value);

void		 cd_color_xyz_copy			(const CdColorXYZ	*src,
							 CdColorXYZ		*dest);
void		 cd_color_yxy_copy			(const CdColorYxy	*src,
							 CdColorYxy		*dest);
void		 cd_color_uvw_copy			(const CdColorUVW	*src,
							 CdColorUVW		*dest);
void		 cd_color_lab_copy			(const CdColorLab	*src,
							 CdColorLab		*dest);
gdouble		 cd_color_lab_delta_e76			(const CdColorLab	*p1,
							 const CdColorLab	*p2);
void		 cd_color_xyz_clear			(CdColorXYZ		*dest);
void		 cd_color_rgb_copy			(const CdColorRGB	*src,
							 CdColorRGB		*dest);
void		 cd_color_rgb8_to_rgb			(const CdColorRGB8	*src,
							 CdColorRGB		*dest);
void		 cd_color_rgb_to_rgb8			(const CdColorRGB	*src,
							 CdColorRGB8		*dest);
void		 cd_color_yxy_to_xyz			(const CdColorYxy	*src,
							 CdColorXYZ		*dest);
void		 cd_color_xyz_to_yxy			(const CdColorXYZ	*src,
							 CdColorYxy		*dest);
void		 cd_color_xyz_to_uvw			(const CdColorXYZ	*src,
							 const CdColorXYZ	*whitepoint,
							 CdColorUVW		*dest);
void		 cd_color_yxy_to_uvw			(const CdColorYxy	*src,
							 CdColorUVW		*dest);
void		 cd_color_uvw_set_planckian_locus	(CdColorUVW		*dest,
							 gdouble		 temp);
gdouble		 cd_color_uvw_get_chroma_difference	(const CdColorUVW	*p1,
							 const CdColorUVW	*p2);
gboolean	 cd_color_get_blackbody_rgb		(guint			 temp,
							 CdColorRGB		*result);
gboolean	 cd_color_get_blackbody_rgb_full	(gdouble		 temp,
							 CdColorRGB		*result,
							 CdColorBlackbodyFlags	 flags);
void		 cd_color_rgb_interpolate		(const CdColorRGB	*p1,
							 const CdColorRGB	*p2,
							 gdouble		 index,
							 CdColorRGB		*result);
void		 cd_color_rgb_from_wavelength		(CdColorRGB		*dest,
							 gdouble		 wavelength);
gdouble		 cd_color_xyz_to_cct			(const CdColorXYZ	*src);
void		 cd_color_xyz_normalize			(const CdColorXYZ	*src,
							 gdouble		 max,
							 CdColorXYZ		*dest);

GPtrArray	*cd_color_rgb_array_new			(void);
gboolean	 cd_color_rgb_array_is_monotonic	(const GPtrArray	*array);
GPtrArray	*cd_color_rgb_array_interpolate		(const GPtrArray	*array,
							 guint			 new_length)
							 G_GNUC_WARN_UNUSED_RESULT;

G_END_DECLS

#endif /* __CD_COLOR_H__ */

