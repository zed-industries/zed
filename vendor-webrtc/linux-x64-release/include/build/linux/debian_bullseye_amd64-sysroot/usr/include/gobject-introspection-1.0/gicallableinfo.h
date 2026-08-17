/* -*- mode: C; c-file-style: "gnu"; indent-tabs-mode: nil; -*-
 * GObject introspection: Callable
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

#ifndef __GICALLABLEINFO_H__
#define __GICALLABLEINFO_H__

#if !defined (__GIREPOSITORY_H_INSIDE__) && !defined (GI_COMPILATION)
#error "Only <girepository.h> can be included directly."
#endif

#include <gitypes.h>

G_BEGIN_DECLS

/**
 * GI_IS_CALLABLE_INFO
 * @info: an info structure
 *
 * Checks if @info is a #GICallableInfo or derived from it.
 */
#define GI_IS_CALLABLE_INFO(info)					\
    ((g_base_info_get_type((GIBaseInfo*)info) == GI_INFO_TYPE_FUNCTION) || \
     (g_base_info_get_type((GIBaseInfo*)info) == GI_INFO_TYPE_CALLBACK) || \
     (g_base_info_get_type((GIBaseInfo*)info) == GI_INFO_TYPE_SIGNAL) || \
     (g_base_info_get_type((GIBaseInfo*)info) == GI_INFO_TYPE_VFUNC))


GI_AVAILABLE_IN_1_34
gboolean               g_callable_info_is_method (GICallableInfo *info);

GI_AVAILABLE_IN_1_34
gboolean               g_callable_info_can_throw_gerror (GICallableInfo *info);

GI_AVAILABLE_IN_ALL
GITypeInfo *           g_callable_info_get_return_type (GICallableInfo *info);

GI_AVAILABLE_IN_ALL
void                   g_callable_info_load_return_type (GICallableInfo *info,
                                                         GITypeInfo     *type);

GI_AVAILABLE_IN_ALL
const gchar *          g_callable_info_get_return_attribute (GICallableInfo *info,
                                                             const gchar    *name);

GI_AVAILABLE_IN_ALL
gboolean               g_callable_info_iterate_return_attributes (GICallableInfo  *info,
                                                                  GIAttributeIter *iterator,
                                                                  char           **name,
                                                                  char          **value);

GI_AVAILABLE_IN_ALL
GITransfer             g_callable_info_get_caller_owns (GICallableInfo *info);

GI_AVAILABLE_IN_ALL
gboolean               g_callable_info_may_return_null (GICallableInfo *info);

GI_AVAILABLE_IN_ALL
gboolean               g_callable_info_skip_return     (GICallableInfo *info);

GI_AVAILABLE_IN_ALL
gint                   g_callable_info_get_n_args      (GICallableInfo *info);

GI_AVAILABLE_IN_ALL
GIArgInfo *            g_callable_info_get_arg         (GICallableInfo *info,
                                                        gint            n);

GI_AVAILABLE_IN_ALL
void                   g_callable_info_load_arg        (GICallableInfo *info,
                                                        gint            n,
                                                        GIArgInfo      *arg);

GI_AVAILABLE_IN_1_34
gboolean               g_callable_info_invoke          (GICallableInfo   *info,
                                                        gpointer          function,
                                                        const GIArgument *in_args,
                                                        int               n_in_args,
                                                        const GIArgument *out_args,
                                                        int               n_out_args,
                                                        GIArgument       *return_value,
                                                        gboolean          is_method,
                                                        gboolean          throws,
                                                        GError          **error);

GI_AVAILABLE_IN_1_42
GITransfer             g_callable_info_get_instance_ownership_transfer (GICallableInfo *info);

G_END_DECLS


#endif  /* __GICALLABLEINFO_H__ */

