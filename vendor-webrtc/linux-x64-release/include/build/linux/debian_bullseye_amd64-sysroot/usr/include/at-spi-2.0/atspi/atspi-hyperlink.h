/*
 * AT-SPI - Assistive Technology Service Provider Interface
 * (Gnome Accessibility Project; http://developer.gnome.org/projects/gap)
 *
 * Copyright 2002 Ximian, Inc.
 *           2002 Sun Microsystems Inc.
 * Copyright 2010, 2011 Novell, Inc.
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

#ifndef _ATSPI_HYPERLINK_H_
#define _ATSPI_HYPERLINK_H_

#include "glib-object.h"

#include "atspi-constants.h"

#include "atspi-text.h"	/* for AtspiRange */
#include "atspi-types.h"

G_BEGIN_DECLS

#define ATSPI_TYPE_HYPERLINK                        (atspi_hyperlink_get_type ())
#define ATSPI_HYPERLINK(obj)                        (G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_HYPERLINK, AtspiHyperlink))
#define ATSPI_HYPERLINK_CLASS(klass)                (G_TYPE_CHECK_CLASS_CAST ((klass), ATSPI_TYPE_HYPERLINK, AtspiHyperlinkClass))
#define ATSPI_IS_HYPERLINK(obj)                     (G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_HYPERLINK))
#define ATSPI_IS_HYPERLINK_CLASS(klass)             (G_TYPE_CHECK_CLASS_TYPE ((klass), ATSPI_TYPE_HYPERLINK))
#define ATSPI_HYPERLINK_GET_CLASS(obj)              (G_TYPE_INSTANCE_GET_CLASS ((obj), ATSPI_TYPE_HYPERLINK, AtspiHyperlinkClass))

struct _AtspiHyperlink
{
  AtspiObject parent;
};

typedef struct _AtspiHyperlinkClass AtspiHyperlinkClass;
struct _AtspiHyperlinkClass
{
  AtspiObjectClass parent_class;
};

GType atspi_hyperlink_get_type (void); 

AtspiHyperlink *
_atspi_hyperlink_new (AtspiApplication *app, const gchar *path);

gint atspi_hyperlink_get_n_anchors (AtspiHyperlink *obj, GError **error);

gchar * atspi_hyperlink_get_uri (AtspiHyperlink *obj, int i, GError **error);

AtspiAccessible* atspi_hyperlink_get_object (AtspiHyperlink *obj, gint i, GError **error);

AtspiRange * atspi_hyperlink_get_index_range (AtspiHyperlink *obj, GError **error);

gint atspi_hyperlink_get_start_index (AtspiHyperlink *obj, GError **error);

gint atspi_hyperlink_get_end_index (AtspiHyperlink *obj, GError **error);

gboolean atspi_hyperlink_is_valid (AtspiHyperlink *obj, GError **error);

G_END_DECLS

#endif	/* _ATSPI_HYPERLINK_H_ */
