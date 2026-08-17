/* GDK - The GIMP Drawing Kit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
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

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GDK_PROPERTY_H__
#define __GDK_PROPERTY_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>

G_BEGIN_DECLS

typedef enum
{
  GDK_PROP_MODE_REPLACE,
  GDK_PROP_MODE_PREPEND,
  GDK_PROP_MODE_APPEND
} GdkPropMode;

GdkAtom gdk_atom_intern (const gchar *atom_name,
			 gboolean     only_if_exists);
GdkAtom gdk_atom_intern_static_string (const gchar *atom_name);
gchar*  gdk_atom_name   (GdkAtom      atom);

gboolean gdk_property_get    (GdkWindow     *window,
			      GdkAtom        property,
			      GdkAtom        type,
			      gulong         offset,
			      gulong         length,
			      gint           pdelete,
			      GdkAtom       *actual_property_type,
			      gint          *actual_format,
			      gint          *actual_length,
			      guchar       **data);
void     gdk_property_change (GdkWindow     *window,
			      GdkAtom        property,
			      GdkAtom        type,
			      gint           format,
			      GdkPropMode    mode,
			      const guchar  *data,
			      gint           nelements);
void     gdk_property_delete (GdkWindow     *window,
			      GdkAtom        property);
#ifndef GDK_MULTIHEAD_SAFE
#ifndef GDK_DISABLE_DEPRECATED
gint gdk_text_property_to_text_list (GdkAtom        encoding,
				     gint           format,
				     const guchar  *text,
				     gint           length,
				     gchar       ***list);
gboolean gdk_utf8_to_compound_text (const gchar *str,
				    GdkAtom     *encoding,
				    gint        *format,
				    guchar     **ctext,
				    gint        *length);
gint gdk_string_to_compound_text    (const gchar   *str,
				     GdkAtom       *encoding,
				     gint          *format,
				     guchar       **ctext,
				     gint          *length);
gint gdk_text_property_to_utf8_list (GdkAtom        encoding,
				     gint           format,
				     const guchar  *text,
				     gint           length,
				     gchar       ***list);
#endif
#endif

gint gdk_text_property_to_utf8_list_for_display (GdkDisplay     *display,
						 GdkAtom         encoding,
						 gint            format,
						 const guchar   *text,
						 gint            length,
						 gchar        ***list);

gchar   *gdk_utf8_to_string_target   (const gchar *str);
#ifndef GDK_DISABLE_DEPRECATED
gint gdk_text_property_to_text_list_for_display (GdkDisplay     *display,
						 GdkAtom         encoding,
						 gint            format,
						 const guchar   *text,
						 gint            length,
						 gchar        ***list);
gint     gdk_string_to_compound_text_for_display (GdkDisplay   *display,
						  const gchar  *str,
						  GdkAtom      *encoding,
						  gint         *format,
						  guchar      **ctext,
						  gint         *length);
gboolean gdk_utf8_to_compound_text_for_display   (GdkDisplay   *display,
						  const gchar  *str,
						  GdkAtom      *encoding,
						  gint         *format,
						  guchar      **ctext,
						  gint         *length);

void gdk_free_text_list             (gchar        **list);
void gdk_free_compound_text         (guchar        *ctext);
#endif

G_END_DECLS

#endif /* __GDK_PROPERTY_H__ */
