/*
 * Copyright (c) 2014 Intel Corporation
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
 * License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 *
 * Author:
 *      Ikey Doherty <michael.i.doherty@intel.com>
 */

#ifndef __GTK_STACK_SIDEBAR_H__
#define __GTK_STACK_SIDEBAR_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkstack.h>

G_BEGIN_DECLS

#define GTK_TYPE_STACK_SIDEBAR           (gtk_stack_sidebar_get_type ())
#define GTK_STACK_SIDEBAR(obj)           (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_STACK_SIDEBAR, GtkStackSidebar))
#define GTK_IS_STACK_SIDEBAR(obj)        (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_STACK_SIDEBAR))

typedef struct _GtkStackSidebar GtkStackSidebar;

GDK_AVAILABLE_IN_ALL
GType       gtk_stack_sidebar_get_type  (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_stack_sidebar_new       (void);
GDK_AVAILABLE_IN_ALL
void        gtk_stack_sidebar_set_stack (GtkStackSidebar *self,
                                         GtkStack        *stack);
GDK_AVAILABLE_IN_ALL
GtkStack *  gtk_stack_sidebar_get_stack (GtkStackSidebar *self);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkStackSidebar, g_object_unref)

G_END_DECLS

#endif /* __GTK_STACK_SIDEBAR_H__ */
