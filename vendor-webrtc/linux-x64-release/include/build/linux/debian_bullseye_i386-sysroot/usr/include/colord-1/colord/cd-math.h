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

#ifndef __CD_MATH_H__
#define __CD_MATH_H__

#define __CD_MATH_H_INSIDE__

typedef struct {
	gdouble	 m00, m01, m02;
	gdouble	 m10, m11, m12;
	gdouble	 m20, m21, m22;
	/* any addition fields go *after* the data */
} CdMat3x3;

typedef struct {
	double	 v0, v1, v2;
	/* any addition fields go *after* the data */
} CdVec3;

void		 cd_vec3_clear			(CdVec3			*src);
void		 cd_vec3_add			(const CdVec3		*src1,
						 const CdVec3		*src2,
						 CdVec3			*dest);
void		 cd_vec3_subtract		(const CdVec3		*src1,
						 const CdVec3		*src2,
						 CdVec3			*dest);
void		 cd_vec3_scalar_multiply	(const CdVec3		*src,
						 gdouble		 value,
						 CdVec3			*dest);
void		 cd_vec3_copy			(const CdVec3		*src,
						 CdVec3			*dest);
gdouble		 cd_vec3_squared_error		(const CdVec3		*src1,
						 const CdVec3		*src2)
						 G_GNUC_WARN_UNUSED_RESULT;
gchar		*cd_vec3_to_string		(const CdVec3		*src);
gdouble		*cd_vec3_get_data		(const CdVec3		*src);
void		 cd_vec3_init			(CdVec3			*dest,
						 gdouble		 v0,
						 gdouble		 v1,
						 gdouble		 v2);
void		 cd_mat33_init			(CdMat3x3		*dest,
						 gdouble		 m00,
						 gdouble		 m01,
						 gdouble		 m02,
						 gdouble		 m10,
						 gdouble		 m11,
						 gdouble		 m12,
						 gdouble		 m20,
						 gdouble		 m21,
						 gdouble		 m22);
void		 cd_mat33_clear			(const CdMat3x3		*src);
gchar		*cd_mat33_to_string		(const CdMat3x3		*src);
gdouble		*cd_mat33_get_data		(const CdMat3x3		*src);
void		 cd_mat33_set_identity		(CdMat3x3		*src);
void		 cd_mat33_scalar_multiply	(const CdMat3x3		*mat_src,
						 gdouble		 value,
						 CdMat3x3		*mat_dest);
void		 cd_mat33_vector_multiply	(const CdMat3x3		*mat_src,
						 const CdVec3		*vec_src,
						 CdVec3			*vec_dest);
void		 cd_mat33_matrix_multiply	(const CdMat3x3		*mat_src1,
						 const CdMat3x3		*mat_src2,
						 CdMat3x3		*mat_dest);
gboolean	 cd_mat33_reciprocal		(const CdMat3x3		*src,
						 CdMat3x3		*dest);
gdouble		 cd_mat33_determinant		(const CdMat3x3		*src)
						 G_GNUC_WARN_UNUSED_RESULT;
void		 cd_mat33_normalize		(const CdMat3x3		*src,
						 CdMat3x3		*dest);
void		 cd_mat33_copy			(const CdMat3x3		*src,
						 CdMat3x3		*dest);
gboolean	 cd_mat33_is_finite		(const CdMat3x3		*mat,
						 GError			**error);

#undef __CD_MATH_H_INSIDE__

#endif /* __CD_MATH_H__ */

