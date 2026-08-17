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

#ifndef __CD_TRANSFORM_H
#define __CD_TRANSFORM_H

#include <glib-object.h>
#include <gio/gio.h>

#include "cd-enum.h"
#include "cd-icc.h"

G_BEGIN_DECLS

#define CD_TRANSFORM_ERROR		(cd_transform_error_quark ())
#define CD_TRANSFORM_TYPE_ERROR		(cd_transform_error_get_type ())

#define CD_TYPE_TRANSFORM (cd_transform_get_type ())
G_DECLARE_DERIVABLE_TYPE (CdTransform, cd_transform, CD, TRANSFORM, GObject)

/**
 * CdTransformError:
 * @CD_TRANSFORM_ERROR_FAILED_TO_SETUP_TRANSFORM:	Failed to setup transform
 * @CD_TRANSFORM_ERROR_INVALID_COLORSPACE:		Invalid colorspace
 *
 * The transform error code.
 *
 * Since: 0.1.34
 **/
typedef enum {
	CD_TRANSFORM_ERROR_FAILED_TO_SETUP_TRANSFORM,
	CD_TRANSFORM_ERROR_INVALID_COLORSPACE,
	CD_TRANSFORM_ERROR_LAST
} CdTransformError;

struct _CdTransformClass
{
	GObjectClass		 parent_class;
	/*< private >*/
	/* Padding for future expansion */
	void (*_cd_transform_reserved1) (void);
	void (*_cd_transform_reserved2) (void);
	void (*_cd_transform_reserved3) (void);
	void (*_cd_transform_reserved4) (void);
	void (*_cd_transform_reserved5) (void);
	void (*_cd_transform_reserved6) (void);
	void (*_cd_transform_reserved7) (void);
	void (*_cd_transform_reserved8) (void);
};

GQuark		 cd_transform_error_quark		(void);
CdTransform	*cd_transform_new			(void);

void		 cd_transform_set_input_icc		(CdTransform	*transform,
							 CdIcc		*icc);
void		 cd_transform_set_input_pixel_format	(CdTransform	*transform,
							 CdPixelFormat	 pixel_format);
CdIcc		*cd_transform_get_input_icc		(CdTransform	*transform);
CdPixelFormat	 cd_transform_get_input_pixel_format	(CdTransform	*transform);

void		 cd_transform_set_output_icc		(CdTransform	*transform,
							 CdIcc		*icc);
void		 cd_transform_set_output_pixel_format	(CdTransform	*transform,
							 CdPixelFormat	 pixel_format);
CdIcc		*cd_transform_get_output_icc		(CdTransform	*transform);
CdPixelFormat	 cd_transform_get_output_pixel_format	(CdTransform	*transform);

void		 cd_transform_set_abstract_icc		(CdTransform	*transform,
							 CdIcc		*icc);
CdIcc		*cd_transform_get_abstract_icc		(CdTransform	*transform);
void		 cd_transform_set_rendering_intent	(CdTransform	*transform,
							 CdRenderingIntent rendering_intent);
CdRenderingIntent cd_transform_get_rendering_intent	(CdTransform	*transform);
void		 cd_transform_set_bpc			(CdTransform	*transform,
							 gboolean	 bpc);
gboolean	 cd_transform_get_bpc			(CdTransform	*transform);
void		 cd_transform_set_max_threads		(CdTransform	*transform,
							 guint		 max_threads);
guint		 cd_transform_get_max_threads		(CdTransform	*transform);
gboolean	 cd_transform_process			(CdTransform	*transform,
							 gpointer	 data_in,
							 gpointer	 data_out,
							 guint		 width,
							 guint		 height,
							 guint		 rowstride,
							 GCancellable	*cancellable,
							 GError		**error)
							 G_GNUC_WARN_UNUSED_RESULT;

G_END_DECLS

#endif /* __CD_TRANSFORM_H */

