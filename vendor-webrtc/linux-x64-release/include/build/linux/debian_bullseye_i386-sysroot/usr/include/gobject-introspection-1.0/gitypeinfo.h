/* -*- mode: C; c-file-style: "gnu"; indent-tabs-mode: nil; -*-
 * GObject introspection: Type
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

#ifndef __GITYPEINFO_H__
#define __GITYPEINFO_H__

#if !defined (__GIREPOSITORY_H_INSIDE__) && !defined (GI_COMPILATION)
#error "Only <girepository.h> can be included directly."
#endif

#include <gitypes.h>

G_BEGIN_DECLS

/**
 * GI_IS_TYPE_INFO
 * @info: an info structure
 *
 * Checks if @info is a #GITypeInfo.
 */
#define GI_IS_TYPE_INFO(info) \
    (g_base_info_get_type((GIBaseInfo*)info) ==  GI_INFO_TYPE_TYPE)

/**
 * G_TYPE_TAG_IS_BASIC
 * @tag: a type tag
 *
 * Checks if @tag is a basic type.
 */
#define G_TYPE_TAG_IS_BASIC(tag) (tag < GI_TYPE_TAG_ARRAY || tag == GI_TYPE_TAG_UNICHAR)

GI_AVAILABLE_IN_ALL
const gchar*           g_type_tag_to_string            (GITypeTag   type);

GI_AVAILABLE_IN_ALL
const gchar*           g_info_type_to_string           (GIInfoType  type);


GI_AVAILABLE_IN_ALL
gboolean               g_type_info_is_pointer          (GITypeInfo *info);

GI_AVAILABLE_IN_ALL
GITypeTag              g_type_info_get_tag             (GITypeInfo *info);

GI_AVAILABLE_IN_ALL
GITypeInfo *           g_type_info_get_param_type      (GITypeInfo *info,
						        gint       n);

GI_AVAILABLE_IN_ALL
GIBaseInfo *           g_type_info_get_interface       (GITypeInfo *info);

GI_AVAILABLE_IN_ALL
gint                   g_type_info_get_array_length    (GITypeInfo *info);

GI_AVAILABLE_IN_ALL
gint                   g_type_info_get_array_fixed_size(GITypeInfo *info);

GI_AVAILABLE_IN_ALL
gboolean               g_type_info_is_zero_terminated  (GITypeInfo *info);

GI_AVAILABLE_IN_ALL
GIArrayType            g_type_info_get_array_type      (GITypeInfo *info);

GI_AVAILABLE_IN_1_66
GITypeTag              g_type_info_get_storage_type    (GITypeInfo *info);

GI_AVAILABLE_IN_1_66
void                   g_type_info_argument_from_hash_pointer (GITypeInfo *info,
                                                               gpointer    hash_pointer,
                                                               GIArgument *arg);

GI_AVAILABLE_IN_1_66
gpointer               g_type_info_hash_pointer_from_argument (GITypeInfo *info,
                                                               GIArgument *arg);

G_END_DECLS


#endif  /* __GITYPEINFO_H__ */

