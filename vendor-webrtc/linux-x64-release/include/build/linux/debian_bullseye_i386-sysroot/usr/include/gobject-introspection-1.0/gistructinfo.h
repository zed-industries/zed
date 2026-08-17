/* -*- mode: C; c-file-style: "gnu"; indent-tabs-mode: nil; -*-
 * GObject introspection: Struct
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

#ifndef __GISTRUCTINFO_H__
#define __GISTRUCTINFO_H__

#if !defined (__GIREPOSITORY_H_INSIDE__) && !defined (GI_COMPILATION)
#error "Only <girepository.h> can be included directly."
#endif

#include <gitypes.h>

G_BEGIN_DECLS

/**
 * GI_IS_STRUCT_INFO
 * @info: an info structure
 *
 * Checks if @info is a #GIStructInfo.
 */
#define GI_IS_STRUCT_INFO(info) \
    (g_base_info_get_type((GIBaseInfo*)info) ==  GI_INFO_TYPE_STRUCT)


GI_AVAILABLE_IN_ALL
gint             g_struct_info_get_n_fields    (GIStructInfo *info);

GI_AVAILABLE_IN_ALL
GIFieldInfo *    g_struct_info_get_field       (GIStructInfo *info,
						gint          n);

GI_AVAILABLE_IN_1_46
GIFieldInfo *    g_struct_info_find_field      (GIStructInfo *info,
						const gchar  *name);

GI_AVAILABLE_IN_ALL
gint             g_struct_info_get_n_methods   (GIStructInfo *info);

GI_AVAILABLE_IN_ALL
GIFunctionInfo * g_struct_info_get_method      (GIStructInfo *info,
						gint          n);

GI_AVAILABLE_IN_ALL
GIFunctionInfo * g_struct_info_find_method     (GIStructInfo *info,
						const gchar  *name);

GI_AVAILABLE_IN_ALL
gsize            g_struct_info_get_size        (GIStructInfo *info);

GI_AVAILABLE_IN_ALL
gsize            g_struct_info_get_alignment   (GIStructInfo *info);

GI_AVAILABLE_IN_ALL
gboolean         g_struct_info_is_gtype_struct (GIStructInfo *info);

GI_AVAILABLE_IN_ALL
gboolean         g_struct_info_is_foreign      (GIStructInfo *info);

G_END_DECLS


#endif  /* __GISTRUCTINFO_H__ */
