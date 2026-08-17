/* GTK - The GIMP Toolkit
 *
 * Copyright (C) 2010  Intel Corporation
 * Copyright (C) 2010  RedHat, Inc.
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
 *
 * Author:
 *      Emmanuele Bassi <ebassi@linux.intel.com>
 *      Matthias Clasen <mclasen@redhat.com>
 *
 * Based on similar code from Mx.
 */

#ifndef __GTK_SWITCH_H__
#define __GTK_SWITCH_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_SWITCH                 (gtk_switch_get_type ())
#define GTK_SWITCH(obj)                 (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SWITCH, GtkSwitch))
#define GTK_IS_SWITCH(obj)              (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SWITCH))

typedef struct _GtkSwitch               GtkSwitch;


GDK_AVAILABLE_IN_ALL
GType gtk_switch_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget *     gtk_switch_new          (void);

GDK_AVAILABLE_IN_ALL
void            gtk_switch_set_active   (GtkSwitch *self,
                                         gboolean   is_active);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_switch_get_active   (GtkSwitch *self);

GDK_AVAILABLE_IN_ALL
void            gtk_switch_set_state   (GtkSwitch *self,
                                        gboolean   state);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_switch_get_state   (GtkSwitch *self);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkSwitch, g_object_unref)

G_END_DECLS

#endif /* __GTK_SWITCH_H__ */
