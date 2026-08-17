/* -*- Mode: C; tab-width: 8; indent-tabs-mode: t; c-basic-offset: 8 -*-
 *
 * Copyright (C) 2010-2013 Richard Hughes <richard@hughsie.com>
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

#ifndef __CD_BUFFER_H__
#define __CD_BUFFER_H__

#include <glib.h>

G_BEGIN_DECLS

typedef enum {
	CD_BUFFER_KIND_REQUEST,
	CD_BUFFER_KIND_RESPONSE,
	CD_BUFFER_KIND_UNKNOWN
} CdBufferKind;

void		 cd_buffer_debug		(CdBufferKind	 buffer_kind,
						 const guint8	*data,
						 gsize		 length);
guint16		 cd_buffer_read_uint16_be	(const guint8	*buffer);
guint16		 cd_buffer_read_uint16_le	(const guint8	*buffer);
void		 cd_buffer_write_uint16_be	(guint8		*buffer,
						 guint16	 value);
void		 cd_buffer_write_uint16_le	(guint8		*buffer,
						 guint16	 value);
guint32		 cd_buffer_read_uint32_be	(const guint8	*buffer);
guint32		 cd_buffer_read_uint32_le	(const guint8	*buffer);
void		 cd_buffer_write_uint32_be	(guint8		*buffer,
						 guint32	 value);
void		 cd_buffer_write_uint32_le	(guint8		*buffer,
						 guint32	 value);

G_END_DECLS

#endif /* __CD_BUFFER_H__ */

