/* -*- mode: C; c-file-style: "gnu"; indent-tabs-mode: nil; -*-
 * GObject introspection: Public typelib API
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

#ifndef __GITYPELIB_H__
#define __GITYPELIB_H__

#if !defined (__GIREPOSITORY_H_INSIDE__) && !defined (GI_COMPILATION)
#error "Only <girepository.h> can be included directly."
#endif

#include <glib.h>

#include <giversionmacros.h>

G_BEGIN_DECLS

/**
 * SECTION:gitypelib
 * @title: GITypelib
 * @short_description: TODO
 *
 * TODO
 */

/**
 * GITypelib:
 *
 * TODO
 */
typedef struct _GITypelib GITypelib;

GI_AVAILABLE_IN_ALL
GITypelib *    g_typelib_new_from_memory       (guint8        *memory,
                                               gsize          len,
					       GError       **error);

GI_AVAILABLE_IN_ALL
GITypelib *    g_typelib_new_from_const_memory (const guint8  *memory,
                                               gsize          len,
					       GError       **error);

GI_AVAILABLE_IN_ALL
GITypelib *    g_typelib_new_from_mapped_file  (GMappedFile   *mfile,
					       GError       **error);

GI_AVAILABLE_IN_ALL
void          g_typelib_free                  (GITypelib     *typelib);

GI_AVAILABLE_IN_ALL
gboolean      g_typelib_symbol                (GITypelib     *typelib,
                                               const gchar  *symbol_name,
                                               gpointer     *symbol);

GI_AVAILABLE_IN_ALL
const gchar * g_typelib_get_namespace         (GITypelib     *typelib);


G_END_DECLS

#endif  /* __GITYPELIB_H__ */

