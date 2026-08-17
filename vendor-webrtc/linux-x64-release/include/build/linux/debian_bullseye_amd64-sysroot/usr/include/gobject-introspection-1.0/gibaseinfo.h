/* -*- mode: C; c-file-style: "gnu"; indent-tabs-mode: nil; -*-
 * GObject introspection: GIBaseInfo
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

#ifndef __GIBASEINFO_H__
#define __GIBASEINFO_H__

#if !defined (__GIREPOSITORY_H_INSIDE__) && !defined (GI_COMPILATION)
#error "Only <girepository.h> can be included directly."
#endif

#include <glib-object.h>
#include <gitypelib.h>
#include <gitypes.h>

G_BEGIN_DECLS

/**
 * GIAttributeIter:
 *
 * An opaque structure used to iterate over attributes
 * in a #GIBaseInfo struct.
 */
typedef struct {
  /* <private> */
  gpointer data;
  gpointer data2;
  gpointer data3;
  gpointer data4;
} GIAttributeIter;

#define GI_TYPE_BASE_INFO	(g_base_info_gtype_get_type ())


GI_AVAILABLE_IN_ALL
GType                  g_base_info_gtype_get_type   (void) G_GNUC_CONST;

GI_AVAILABLE_IN_ALL
GIBaseInfo *           g_base_info_ref              (GIBaseInfo   *info);

GI_AVAILABLE_IN_ALL
void                   g_base_info_unref            (GIBaseInfo   *info);

GI_AVAILABLE_IN_ALL
GIInfoType             g_base_info_get_type         (GIBaseInfo   *info);

GI_AVAILABLE_IN_ALL
const gchar *          g_base_info_get_name         (GIBaseInfo   *info);

GI_AVAILABLE_IN_ALL
const gchar *          g_base_info_get_namespace    (GIBaseInfo   *info);

GI_AVAILABLE_IN_ALL
gboolean               g_base_info_is_deprecated    (GIBaseInfo   *info);

GI_AVAILABLE_IN_ALL
const gchar *          g_base_info_get_attribute    (GIBaseInfo   *info,
                                                     const gchar  *name);

GI_AVAILABLE_IN_ALL
gboolean               g_base_info_iterate_attributes (GIBaseInfo      *info,
                                                       GIAttributeIter *iterator,
                                                       char           **name,
                                                       char          **value);

GI_AVAILABLE_IN_ALL
GIBaseInfo *           g_base_info_get_container    (GIBaseInfo   *info);

GI_AVAILABLE_IN_ALL
GITypelib *             g_base_info_get_typelib      (GIBaseInfo   *info);

GI_AVAILABLE_IN_ALL
gboolean               g_base_info_equal            (GIBaseInfo   *info1,
                                                     GIBaseInfo   *info2);

GI_AVAILABLE_IN_ALL
GIBaseInfo *           g_info_new                   (GIInfoType    type,
						     GIBaseInfo   *container,
						     GITypelib     *typelib,
						     guint32       offset);

G_END_DECLS

#endif  /* __GIBASEINFO_H__ */

