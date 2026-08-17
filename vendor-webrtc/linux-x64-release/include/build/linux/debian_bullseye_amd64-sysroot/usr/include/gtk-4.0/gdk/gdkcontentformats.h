/* GTK - The GIMP Toolkit
 * Copyright (C) 2017 Benjamin Otte
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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_CONTENT_FORMATS_H__
#define __GTK_CONTENT_FORMATS_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif


#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>

G_BEGIN_DECLS

#define GDK_TYPE_CONTENT_FORMATS    (gdk_content_formats_get_type ())

GDK_AVAILABLE_IN_ALL
const char *            gdk_intern_mime_type                    (const char                     *string);

GDK_AVAILABLE_IN_ALL
GType                   gdk_content_formats_get_type            (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GdkContentFormats *     gdk_content_formats_new                 (const char                    **mime_types,
                                                                 guint                           n_mime_types);
GDK_AVAILABLE_IN_ALL
GdkContentFormats *     gdk_content_formats_new_for_gtype       (GType                           type);
GDK_AVAILABLE_IN_4_4
GdkContentFormats *     gdk_content_formats_parse               (const char                     *string);
GDK_AVAILABLE_IN_ALL
GdkContentFormats *     gdk_content_formats_ref                 (GdkContentFormats              *formats);
GDK_AVAILABLE_IN_ALL
void                    gdk_content_formats_unref               (GdkContentFormats              *formats);

GDK_AVAILABLE_IN_ALL
void                    gdk_content_formats_print               (GdkContentFormats              *formats,
                                                                 GString                        *string);
GDK_AVAILABLE_IN_ALL
char *                  gdk_content_formats_to_string           (GdkContentFormats              *formats);

GDK_AVAILABLE_IN_ALL
const GType *           gdk_content_formats_get_gtypes          (const GdkContentFormats        *formats,
                                                                 gsize                          *n_gtypes);
GDK_AVAILABLE_IN_ALL
const char * const *    gdk_content_formats_get_mime_types      (const GdkContentFormats        *formats,
                                                                 gsize                          *n_mime_types);

GDK_AVAILABLE_IN_ALL
GdkContentFormats *     gdk_content_formats_union               (GdkContentFormats              *first,
                                                                 const GdkContentFormats        *second) G_GNUC_WARN_UNUSED_RESULT;
GDK_AVAILABLE_IN_ALL
gboolean                gdk_content_formats_match               (const GdkContentFormats        *first,
                                                                 const GdkContentFormats        *second);
GDK_AVAILABLE_IN_ALL
GType                   gdk_content_formats_match_gtype         (const GdkContentFormats        *first,
                                                                 const GdkContentFormats        *second);
GDK_AVAILABLE_IN_ALL
const char *            gdk_content_formats_match_mime_type     (const GdkContentFormats        *first,
                                                                 const GdkContentFormats        *second);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_content_formats_contain_gtype       (const GdkContentFormats        *formats,
                                                                 GType                           type);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_content_formats_contain_mime_type   (const GdkContentFormats        *formats,
                                                                 const char                     *mime_type);

#define GDK_TYPE_CONTENT_FORMATS_BUILDER (gdk_content_formats_builder_get_type ())

typedef struct _GdkContentFormatsBuilder GdkContentFormatsBuilder;

GDK_AVAILABLE_IN_ALL
GType                   gdk_content_formats_builder_get_type    (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GdkContentFormatsBuilder *gdk_content_formats_builder_new        (void);
GDK_AVAILABLE_IN_ALL
GdkContentFormatsBuilder *gdk_content_formats_builder_ref       (GdkContentFormatsBuilder       *builder);
GDK_AVAILABLE_IN_ALL
void                    gdk_content_formats_builder_unref       (GdkContentFormatsBuilder       *builder);
GDK_AVAILABLE_IN_ALL
GdkContentFormats *     gdk_content_formats_builder_free_to_formats (GdkContentFormatsBuilder  *builder) G_GNUC_WARN_UNUSED_RESULT;
GDK_AVAILABLE_IN_ALL
GdkContentFormats *     gdk_content_formats_builder_to_formats  (GdkContentFormatsBuilder  *builder) G_GNUC_WARN_UNUSED_RESULT;
GDK_AVAILABLE_IN_ALL
void                    gdk_content_formats_builder_add_formats (GdkContentFormatsBuilder       *builder,
                                                                 const GdkContentFormats        *formats);
GDK_AVAILABLE_IN_ALL
void                    gdk_content_formats_builder_add_mime_type(GdkContentFormatsBuilder      *builder,
                                                                 const char                     *mime_type);
GDK_AVAILABLE_IN_ALL
void                    gdk_content_formats_builder_add_gtype   (GdkContentFormatsBuilder       *builder,
                                                                 GType                           type);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkContentFormats, gdk_content_formats_unref)

/* dunno where else to put this */
#define GDK_TYPE_FILE_LIST (gdk_file_list_get_type ())
GDK_AVAILABLE_IN_ALL
GType gdk_file_list_get_type (void) G_GNUC_CONST;

/**
 * GdkFileList:
 *
 * An opaque type representing a list of files.
 *
 * Since: 4.6
 */
typedef struct _GdkFileList GdkFileList;

GDK_AVAILABLE_IN_4_6
GSList *        gdk_file_list_get_files (GdkFileList *file_list);
GDK_AVAILABLE_IN_4_8
GdkFileList *   gdk_file_list_new_from_list (GSList *files);
GDK_AVAILABLE_IN_4_8
GdkFileList *   gdk_file_list_new_from_array (GFile **files,
                                              gsize   n_files);

G_END_DECLS

#endif /* __GTK_CONTENT_FORMATS_H__ */
