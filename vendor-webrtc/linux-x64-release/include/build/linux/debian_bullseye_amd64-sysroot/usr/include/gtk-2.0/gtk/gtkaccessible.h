/* GTK - The GIMP Toolkit
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_ACCESSIBLE_H__
#define __GTK_ACCESSIBLE_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <atk/atk.h>
#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_ACCESSIBLE                  (gtk_accessible_get_type ())
#define GTK_ACCESSIBLE(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ACCESSIBLE, GtkAccessible))
#define GTK_ACCESSIBLE_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_ACCESSIBLE, GtkAccessibleClass))
#define GTK_IS_ACCESSIBLE(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_ACCESSIBLE))
#define GTK_IS_ACCESSIBLE_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_ACCESSIBLE))
#define GTK_ACCESSIBLE_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_ACCESSIBLE, GtkAccessibleClass))

typedef struct _GtkAccessible                GtkAccessible;
typedef struct _GtkAccessibleClass           GtkAccessibleClass;

  /*
   * This object is a thin wrapper, in the GTK+ namespace, for AtkObject
   */
struct _GtkAccessible
{
  AtkObject parent;

  /*
   * The GtkWidget whose properties and features are exported via this 
   * accessible instance.
   */
  GtkWidget *GSEAL (widget);
};

struct _GtkAccessibleClass
{
  AtkObjectClass parent_class;

  void (*connect_widget_destroyed)              (GtkAccessible     *accessible);
  
  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GType gtk_accessible_get_type (void) G_GNUC_CONST;

void        gtk_accessible_set_widget                  (GtkAccessible     *accessible,
                                                        GtkWidget         *widget);
GtkWidget*  gtk_accessible_get_widget                  (GtkAccessible     *accessible);
void        gtk_accessible_connect_widget_destroyed    (GtkAccessible     *accessible);

G_END_DECLS

#endif /* __GTK_ACCESSIBLE_H__ */


