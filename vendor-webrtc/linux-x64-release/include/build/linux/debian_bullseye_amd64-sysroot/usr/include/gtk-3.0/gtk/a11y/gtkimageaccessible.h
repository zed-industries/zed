/* GTK+ - accessibility implementations
 * Copyright 2001 Sun Microsystems Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_IMAGE_ACCESSIBLE_H__
#define __GTK_IMAGE_ACCESSIBLE_H__

#if !defined (__GTK_A11Y_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk-a11y.h> can be included directly."
#endif

#include <gtk/a11y/gtkwidgetaccessible.h>

G_BEGIN_DECLS

#define GTK_TYPE_IMAGE_ACCESSIBLE                      (gtk_image_accessible_get_type ())
#define GTK_IMAGE_ACCESSIBLE(obj)                      (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_IMAGE_ACCESSIBLE, GtkImageAccessible))
#define GTK_IMAGE_ACCESSIBLE_CLASS(klass)              (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_IMAGE_ACCESSIBLE, GtkImageAccessibleClass))
#define GTK_IS_IMAGE_ACCESSIBLE(obj)                   (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_IMAGE_ACCESSIBLE))
#define GTK_IS_IMAGE_ACCESSIBLE_CLASS(klass)           (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_IMAGE_ACCESSIBLE))
#define GTK_IMAGE_ACCESSIBLE_GET_CLASS(obj)            (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_IMAGE_ACCESSIBLE, GtkImageAccessibleClass))

typedef struct _GtkImageAccessible        GtkImageAccessible;
typedef struct _GtkImageAccessibleClass   GtkImageAccessibleClass;
typedef struct _GtkImageAccessiblePrivate GtkImageAccessiblePrivate;

struct _GtkImageAccessible
{
  GtkWidgetAccessible parent;

  GtkImageAccessiblePrivate *priv;
};

struct _GtkImageAccessibleClass
{
  GtkWidgetAccessibleClass parent_class;
};

GDK_AVAILABLE_IN_ALL
GType gtk_image_accessible_get_type (void);

G_END_DECLS

#endif /* __GTK_IMAGE_ACCESSIBLE_H__ */
