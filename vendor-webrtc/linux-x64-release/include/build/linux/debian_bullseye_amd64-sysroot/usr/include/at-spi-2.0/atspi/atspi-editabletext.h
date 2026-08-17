/*
 * AT-SPI - Assistive Technology Service Provider Interface
 * (Gnome Accessibility Project; http://developer.gnome.org/projects/gap)
 *
 * Copyright 2002 Ximian, Inc.
 *           2002 Sun Microsystems Inc.
 *           
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef _ATSPI_EDITABLE_TEXT_H_
#define _ATSPI_EDITABLE_TEXT_H_

#include "glib-object.h"

#include "atspi-constants.h"

#include "atspi-types.h"

G_BEGIN_DECLS

#define ATSPI_TYPE_EDITABLE_TEXT                    (atspi_editable_text_get_type ())
#define ATSPI_IS_EDITABLE_TEXT(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_EDITABLE_TEXT)
#define ATSPI_EDITABLE_TEXT(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_EDITABLE_TEXT, AtspiEditableText)
#define ATSPI_EDITABLE_TEXT_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATSPI_TYPE_EDITABLE_TEXT, AtspiEditableText))

GType atspi_editable_text_get_type ();

struct _AtspiEditableText
{
  GTypeInterface parent;
};

#if 0
gboolean atspi_editable_text_set_attributes (AtspiEditableText *obj, const char *attributes, gint start_pos, gint end_pos, GError **error)
#endif

gboolean atspi_editable_text_set_text_contents (AtspiEditableText *obj, const gchar *new_contents, GError **error);

gboolean atspi_editable_text_insert_text (AtspiEditableText *obj, gint position, const gchar *text, gint length, GError **error);

gboolean atspi_editable_text_copy_text (AtspiEditableText *obj, gint start_pos, gint end_pos, GError **error);

gboolean atspi_editable_text_cut_text (AtspiEditableText *obj, gint start_pos, gint end_pos, GError **error);

gboolean atspi_editable_text_delete_text (AtspiEditableText *obj, gint start_pos, gint end_pos, GError **error);

gboolean atspi_editable_text_paste_text (AtspiEditableText *obj, gint position, GError **error);

G_END_DECLS

#endif	/* _ATSPI_EDITABLE_TEXT_H_ */
