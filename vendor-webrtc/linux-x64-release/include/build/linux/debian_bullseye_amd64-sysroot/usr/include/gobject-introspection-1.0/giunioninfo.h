/* -*- mode: C; c-file-style: "gnu"; indent-tabs-mode: nil; -*-
 * GObject introspection: Union
 *
 * Copyright (C) 2005 Matthias Clasen
 * Copyright (C) 2008,2009 Red Hat, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GIUNIONINFO_H__
#define __GIUNIONINFO_H__

#if !defined (__GIREPOSITORY_H_INSIDE__) && !defined (GI_COMPILATION)
#error "Only <girepository.h> can be included directly."
#endif

#include <gitypes.h>

G_BEGIN_DECLS

/**
 * GI_IS_UNION_INFO
 * @info: an info structure
 *
 * Checks if @info is a #GIUnionInfo.
 */
#define GI_IS_UNION_INFO(info) \
    (g_base_info_get_type((GIBaseInfo*)info) ==  GI_INFO_TYPE_UNION)

GI_AVAILABLE_IN_ALL
gint             g_union_info_get_n_fields             (GIUnionInfo *info);

GI_AVAILABLE_IN_ALL
GIFieldInfo *    g_union_info_get_field                (GIUnionInfo *info,
							gint         n);

GI_AVAILABLE_IN_ALL
gint             g_union_info_get_n_methods            (GIUnionInfo *info);

GI_AVAILABLE_IN_ALL
GIFunctionInfo * g_union_info_get_method               (GIUnionInfo *info,
							gint         n);

GI_AVAILABLE_IN_ALL
gboolean         g_union_info_is_discriminated         (GIUnionInfo *info);

GI_AVAILABLE_IN_ALL
gint             g_union_info_get_discriminator_offset (GIUnionInfo *info);

GI_AVAILABLE_IN_ALL
GITypeInfo *     g_union_info_get_discriminator_type   (GIUnionInfo *info);

GI_AVAILABLE_IN_ALL
GIConstantInfo * g_union_info_get_discriminator        (GIUnionInfo *info,
							gint         n);

GI_AVAILABLE_IN_ALL
GIFunctionInfo * g_union_info_find_method              (GIUnionInfo *info,
							const gchar *name);

GI_AVAILABLE_IN_ALL
gsize            g_union_info_get_size                 (GIUnionInfo *info);

GI_AVAILABLE_IN_ALL
gsize            g_union_info_get_alignment            (GIUnionInfo *info);

G_END_DECLS


#endif  /* __GIUNIONINFO_H__ */

